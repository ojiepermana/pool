//! The control loop of the daemon (spec 0004, "Control loop and topology
//! snapshot").
//!
//! One task owns the `Controller` of `pool-control`. It receives samples
//! from the probe tasks over a bounded channel, runs a round after each
//! sample and at least once per `probe.period_ms`, maps the snapshot into
//! the route view of `pool-core` and publishes it at once. Only then does it
//! write each new conflict latch to the journal, off the workers of the
//! runtime. `pool-core` never sees a type of `pool-control`: the mapping
//! lives here.

use std::collections::HashMap;
use std::sync::Arc;

use pool_config::{ClusterId, ValidatedConfig};
use pool_control::{
    ClosedReason, Controller, Journal, LatchReceipt, MonotonicTime, Observation, TopologySnapshot,
    WriterAdmission,
};
use pool_core::route::{Route, RouteState, RouteView};
use tokio::sync::{mpsc, watch};
use tokio::time::Instant;

/// The reasons that end the running sessions of a cluster (spec 0004, "When
/// a running session is ended"). Silence alone does not.
fn is_conflict(reason: ClosedReason) -> bool {
    matches!(
        reason,
        ClosedReason::WriterIdentity
            | ClosedReason::WriterInRecovery
            | ClosedReason::WriterTimeline
            | ClosedReason::SecondPrimary
            | ClosedReason::TimelineAhead
            | ClosedReason::ConflictLatched
    )
}

/// The route view of a snapshot: an open answer with its freshness becomes
/// an open route, every other answer a closed one, with the conflict flag.
pub fn route_view(snapshot: &TopologySnapshot) -> RouteView {
    let routes = snapshot
        .clusters()
        .iter()
        .map(|cluster| {
            let state = match (cluster.admission(), cluster.fresh_until()) {
                (WriterAdmission::Open(primary), Some(fresh_until)) => RouteState::Open(Route {
                    node: primary.node().clone(),
                    topology_epoch: primary.topology_epoch(),
                    authority_revision: primary.authority_revision(),
                    fresh_until: fresh_until.since_origin(),
                }),
                (WriterAdmission::Open(_), None) => RouteState::Closed { conflict: false },
                (WriterAdmission::Closed(closed), _) => RouteState::Closed {
                    conflict: closed.reasons().iter().any(|reason| is_conflict(*reason)),
                },
            };
            (cluster.cluster().clone(), state)
        })
        .collect();
    RouteView::new(routes)
}

/// What a round said about one cluster, to tell when it changed.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Seen {
    Open(String),
    Closed(String),
}

/// The control loop: its controller, the journal for latch writes, and the
/// sender of the route view.
pub struct ControlLoop {
    config: Arc<ValidatedConfig>,
    controller: Controller,
    journal: Option<Journal>,
    routes: watch::Sender<RouteView>,
    origin: Instant,
    seen: HashMap<ClusterId, Seen>,
    unrecorded: Vec<ClusterId>,
}

impl ControlLoop {
    /// A loop for the state classified at start, with an empty board.
    pub fn new(
        config: Arc<ValidatedConfig>,
        controller: Controller,
        journal: Journal,
        routes: watch::Sender<RouteView>,
        origin: Instant,
    ) -> ControlLoop {
        ControlLoop {
            config,
            controller,
            journal: Some(journal),
            routes,
            origin,
            seen: HashMap::new(),
            unrecorded: Vec::new(),
        }
    }

    /// Runs rounds until `stop` changes: after each sample, and at least
    /// once per `probe.period_ms`. Returns the journal, to be closed.
    pub async fn run(
        mut self,
        mut samples: mpsc::Receiver<Observation>,
        mut stop: watch::Receiver<bool>,
    ) -> Option<Journal> {
        let mut tick = tokio::time::interval(self.config.probe().period());
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                biased;
                _ = stop.changed() => break,
                sample = samples.recv() => match sample {
                    Some(observation) => self.controller.record(observation),
                    None => break,
                },
                _ = tick.tick() => {}
            }
            self.round().await;
        }
        self.journal
    }

    /// One round: in memory first, published at once, then the disk.
    async fn round(&mut self) {
        let now = MonotonicTime::from_origin(self.origin.elapsed());
        let snapshot = self.controller.round(&self.config, now);
        self.routes.send_replace(route_view(&snapshot));
        self.log_changes(&snapshot);
        for cluster in self.controller.unrecorded_latches() {
            if !self.unrecorded.contains(cluster) {
                tracing::warn!(
                    event = "authority.conflict_latched",
                    cluster_id = cluster.as_str(),
                    recorded = false
                );
                self.unrecorded.push(cluster.clone());
            }
        }
        for cluster in self.controller.unrecorded_latches().to_vec() {
            match self.write_latch(cluster).await {
                Ok(receipt) => {
                    tracing::warn!(
                        event = "authority.conflict_latched",
                        cluster_id = receipt.cluster().as_str(),
                        recorded = true
                    );
                    self.unrecorded.retain(|held| held != receipt.cluster());
                    self.controller.latch_recorded(&receipt);
                }
                Err(class) => {
                    tracing::error!(event = "journal.write_failed", class);
                }
            }
        }
    }

    /// Writes one latch on a blocking thread, so no worker waits for the disk.
    async fn write_latch(&mut self, cluster: ClusterId) -> Result<LatchReceipt, &'static str> {
        let Some(mut journal) = self.journal.take() else {
            return Err("journal.open");
        };
        let changed = crate::commands::unix_ms();
        let task = tokio::task::spawn_blocking(move || {
            let written = journal.set_conflict_latch(&cluster, changed);
            (journal, written)
        });
        match task.await {
            Ok((journal, written)) => {
                self.journal = Some(journal);
                written.map_err(|error| error.as_str())
            }
            // The blocking task did not come back: the journal is gone for
            // this run, and every later write fails the same way.
            Err(_) => Err("journal.write"),
        }
    }

    /// Writes `admission.changed` for each cluster whose state or reasons
    /// differ from the last round.
    fn log_changes(&mut self, snapshot: &TopologySnapshot) {
        let generation = snapshot.generation().get();
        for cluster in snapshot.clusters() {
            let seen = match cluster.admission() {
                WriterAdmission::Open(primary) => Seen::Open(primary.node().as_str().to_owned()),
                WriterAdmission::Closed(closed) => Seen::Closed(
                    closed
                        .reasons()
                        .iter()
                        .map(|reason| reason.as_str())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            };
            if self.seen.get(cluster.cluster()) == Some(&seen) {
                continue;
            }
            match &seen {
                Seen::Open(node) => tracing::info!(
                    event = "admission.changed",
                    cluster_id = cluster.cluster().as_str(),
                    state = "open",
                    node_id = node.as_str(),
                    snapshot_generation = generation
                ),
                Seen::Closed(reasons) => tracing::info!(
                    event = "admission.changed",
                    cluster_id = cluster.cluster().as_str(),
                    state = "closed",
                    reasons = reasons.as_str(),
                    snapshot_generation = generation
                ),
            }
            self.seen.insert(cluster.cluster().clone(), seen);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_conflict_reasons_are_the_six_of_the_spec() {
        let conflict: Vec<&str> = [
            ClosedReason::Uninitialized,
            ClosedReason::Unconfirmed,
            ClosedReason::Missing,
            ClosedReason::Unreadable,
            ClosedReason::Mismatch,
            ClosedReason::RestorePending,
            ClosedReason::ClusterUnknown,
            ClosedReason::ConfigConflict,
            ClosedReason::ConflictLatched,
            ClosedReason::WriterQuarantined,
            ClosedReason::WriterMissing,
            ClosedReason::WriterIdentity,
            ClosedReason::WriterInRecovery,
            ClosedReason::WriterTimeline,
            ClosedReason::SecondPrimary,
            ClosedReason::TimelineAhead,
        ]
        .into_iter()
        .filter(|reason| is_conflict(*reason))
        .map(ClosedReason::as_str)
        .collect();
        assert_eq!(
            conflict,
            [
                "authority.conflict_latched",
                "evidence.writer_identity",
                "evidence.writer_in_recovery",
                "evidence.writer_timeline",
                "evidence.second_primary",
                "evidence.timeline_ahead",
            ]
        );
    }
}
