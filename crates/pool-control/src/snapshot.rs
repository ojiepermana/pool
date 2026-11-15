//! The topology snapshot and the controller of the control loop.
//!
//! [`build_snapshot`] is the one caller of [`writer_admission`] in the
//! product. It passes the samples of an [`ObservationBoard`], which holds one
//! sample per node, and it never passes a now earlier than the latest sample.
//! [`Controller`] runs one round in memory: it builds a snapshot, sets the
//! conflict latch of each cluster that asks for it, and builds the snapshot
//! again, so a second primary closes its cluster before any write to the
//! journal. The contract is spec 0004, section "Control loop and topology
//! snapshot".

use std::fmt;

use pool_config::{ClusterId, ValidatedConfig};

use crate::admission::{WriterAdmission, writer_admission};
use crate::authority::AuthorityState;
use crate::journal::LatchReceipt;
use crate::observation::{MonotonicTime, Observation, ObservationBoard};

/// The largest value a snapshot generation holds, as for the generation
/// types of `pool-config`.
const MAX_GENERATION: u64 = 9_223_372_036_854_775_807;

/// The generation of one topology snapshot. It starts at 1 and advances by
/// one with every round of the control loop. It compares only with another
/// `SnapshotGeneration`.
///
/// Restart rule: memory only. Each start begins again at 1, and a generation
/// is comparable only inside one start sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SnapshotGeneration(u64);

impl SnapshotGeneration {
    /// The generation of the first snapshot of a run.
    pub const FIRST: SnapshotGeneration = SnapshotGeneration(1);

    /// Builds the generation from a number. Returns `None` for 0 or for a
    /// value above 9223372036854775807.
    pub fn from_stored(value: u64) -> Option<SnapshotGeneration> {
        (1..=MAX_GENERATION)
            .contains(&value)
            .then_some(SnapshotGeneration(value))
    }

    /// The number.
    pub fn get(self) -> u64 {
        self.0
    }

    /// The next generation, larger by one. Returns `None` at
    /// 9223372036854775807; it never wraps.
    pub fn checked_next(self) -> Option<SnapshotGeneration> {
        (self.0 < MAX_GENERATION).then(|| SnapshotGeneration(self.0 + 1))
    }
}

impl fmt::Display for SnapshotGeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The answer of admission for one cluster in a snapshot.
///
/// Restart rule: memory only, gone at a restart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClusterAdmission {
    cluster: ClusterId,
    admission: WriterAdmission,
    fresh_until: Option<MonotonicTime>,
}

impl ClusterAdmission {
    /// The cluster.
    pub fn cluster(&self) -> &ClusterId {
        &self.cluster
    }

    /// The answer of [`writer_admission`] for the cluster.
    pub fn admission(&self) -> &WriterAdmission {
        &self.admission
    }

    /// For an open answer, the time at which the sample of the approved
    /// writer stops being fresh: its sample time plus the expiry. `None` for
    /// a closed answer. The data plane refuses new sessions after this time,
    /// so a control loop that stops publishing closes every route by itself.
    pub fn fresh_until(&self) -> Option<MonotonicTime> {
        self.fresh_until
    }
}

/// One answer of admission per configured cluster, at one generation.
///
/// Restart rule: memory only, gone at a restart. The daemon publishes a new
/// one after every round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySnapshot {
    generation: SnapshotGeneration,
    clusters: Vec<ClusterAdmission>,
}

impl TopologySnapshot {
    /// The generation of the snapshot.
    pub fn generation(&self) -> SnapshotGeneration {
        self.generation
    }

    /// The answer for each cluster of the configuration, in its order.
    pub fn clusters(&self) -> &[ClusterAdmission] {
        &self.clusters
    }
}

/// Builds the snapshot: [`writer_admission`] once for every cluster of
/// `config`, with the samples of `board` and the expiry of `config.probe()`.
///
/// The now it uses is the later of `now` and the latest sample time on the
/// board, so no sample is ever later than now (`0003 AC-18`). A later now
/// only makes samples older, so this can close and never open. For an open
/// answer, `fresh_until` is the sample time of the approved writer plus the
/// expiry.
pub fn build_snapshot(
    config: &ValidatedConfig,
    state: &AuthorityState,
    board: &ObservationBoard,
    now: MonotonicTime,
    generation: SnapshotGeneration,
) -> TopologySnapshot {
    let now = board.clamp(now);
    let expiry = config.probe().observation_expiry();
    let clusters = config
        .clusters()
        .iter()
        .map(|cluster| {
            let admission = writer_admission(
                config,
                state,
                cluster.id(),
                board.observations(),
                now,
                expiry,
            );
            let fresh_until = match &admission {
                WriterAdmission::Open(primary) => board
                    .get(primary.node())
                    .map(|sample| fresh_until(sample, expiry)),
                WriterAdmission::Closed(_) => None,
            };
            ClusterAdmission {
                cluster: cluster.id().clone(),
                admission,
                fresh_until,
            }
        })
        .collect();
    TopologySnapshot {
        generation,
        clusters,
    }
}

/// The sample time plus the expiry. At the end of the clock it is the sample
/// time itself, which only shortens the route.
fn fresh_until(sample: &Observation, expiry: std::time::Duration) -> MonotonicTime {
    let since = sample.sampled_at().since_origin();
    MonotonicTime::from_origin(since.checked_add(expiry).unwrap_or(since))
}

/// The state of the control loop: the authority state, the samples, the next
/// snapshot generation and the conflict latches that the journal does not
/// hold yet.
///
/// It works in memory only and never waits for the disk. The daemon calls
/// [`Controller::round`], publishes the snapshot at once, and only then
/// writes each latch of [`Controller::unrecorded_latches`] to the journal and
/// hands each receipt to [`Controller::latch_recorded`]. A write that fails
/// leaves its cluster on the list, and the daemon tries again after the next
/// round. The latch holds in memory either way.
///
/// Restart rule: memory only. At the next start the journal is read again,
/// and a latch that was written is part of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Controller {
    state: AuthorityState,
    board: ObservationBoard,
    generation: Option<SnapshotGeneration>,
    last_generation: SnapshotGeneration,
    unrecorded: Vec<ClusterId>,
}

impl Controller {
    /// A controller for a state that was classified at the start, with an
    /// empty board.
    pub fn new(state: AuthorityState) -> Controller {
        Controller {
            state,
            board: ObservationBoard::new(),
            generation: Some(SnapshotGeneration::FIRST),
            last_generation: SnapshotGeneration::FIRST,
            unrecorded: Vec::new(),
        }
    }

    /// Records one probe sample on the board.
    pub fn record(&mut self, observation: Observation) {
        self.board.record(observation);
    }

    /// Runs one round: builds a snapshot, sets in the state the conflict
    /// latch of each cluster whose answer asks for it, and builds the
    /// snapshot again, so the one it returns already shows
    /// `authority.conflict_latched`. A cluster whose latch this round set is
    /// put on the list of latches the journal does not hold yet.
    ///
    /// Each round advances the snapshot generation by one. When the
    /// generation cannot advance, the round returns every cluster closed.
    pub fn round(&mut self, config: &ValidatedConfig, now: MonotonicTime) -> TopologySnapshot {
        let Some(generation) = self.generation else {
            // The generation is at its largest value. No state but `Ready`
            // can open, so the snapshot of an unreadable state closes every
            // cluster with a reason that already exists.
            return build_snapshot(
                config,
                &AuthorityState::Unreadable,
                &self.board,
                now,
                self.last_generation,
            );
        };
        let first = build_snapshot(config, &self.state, &self.board, now, generation);
        let mut changed = false;
        for cluster in first.clusters() {
            let WriterAdmission::Closed(closed) = cluster.admission() else {
                continue;
            };
            let before = self.state.is_latched(cluster.cluster());
            let after = self.state.latch_conflict(cluster.cluster(), closed);
            if after && !before {
                changed = true;
                if !self.unrecorded.contains(cluster.cluster()) {
                    self.unrecorded.push(cluster.cluster().clone());
                }
            }
        }
        let snapshot = if changed {
            build_snapshot(config, &self.state, &self.board, now, generation)
        } else {
            first
        };
        self.last_generation = generation;
        self.generation = generation.checked_next();
        snapshot
    }

    /// The clusters whose latch is set in memory and not yet in the journal,
    /// in the order their latches were set.
    pub fn unrecorded_latches(&self) -> &[ClusterId] {
        &self.unrecorded
    }

    /// Takes the receipt of a journal write: the cluster leaves the list of
    /// latches the journal does not hold yet, and the state takes the
    /// authority revision of the receipt.
    pub fn latch_recorded(&mut self, receipt: &LatchReceipt) {
        self.unrecorded
            .retain(|cluster| cluster != receipt.cluster());
        self.state.latch_recorded(receipt);
    }

    /// The authority state, with every latch this run set.
    pub fn state(&self) -> &AuthorityState {
        &self.state
    }

    /// The samples the next round reads.
    pub fn board(&self) -> &ObservationBoard {
        &self.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> ValidatedConfig {
        let text = "config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n\n\
                    [[cluster]]\nid = \"main\"\nsystem_identifier = \"7412345678901234567\"\nexpected_writer = \"pg1\"\n\n\
                    [[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5432\n";
        pool_config::validate(text.as_bytes()).expect("valid")
    }

    #[test]
    fn snapshot_generation_starts_at_one_and_stops_at_the_top() {
        assert_eq!(SnapshotGeneration::FIRST.get(), 1);
        assert_eq!(SnapshotGeneration::from_stored(0), None);
        let top = SnapshotGeneration::from_stored(MAX_GENERATION).expect("top");
        assert_eq!(top.checked_next(), None);
        assert_eq!(SnapshotGeneration::from_stored(MAX_GENERATION + 1), None);
        assert_eq!(
            SnapshotGeneration::FIRST
                .checked_next()
                .map(SnapshotGeneration::get),
            Some(2)
        );
        assert_eq!(top.to_string(), "9223372036854775807");
    }

    #[test]
    fn a_round_whose_generation_cannot_advance_closes_every_cluster() {
        let config = config();
        let mut controller = Controller::new(AuthorityState::Uninitialized);
        let top = SnapshotGeneration::from_stored(MAX_GENERATION).expect("top");
        controller.generation = Some(top);
        let last = controller.round(&config, MonotonicTime::from_origin(Default::default()));
        assert_eq!(last.generation(), top);
        assert_eq!(controller.generation, None);
        for _ in 0..2 {
            let snapshot =
                controller.round(&config, MonotonicTime::from_origin(Default::default()));
            assert_eq!(snapshot.generation(), top);
            for cluster in snapshot.clusters() {
                let WriterAdmission::Closed(closed) = cluster.admission() else {
                    panic!("open after the last generation");
                };
                let names: Vec<&str> = closed.reasons().iter().map(|r| r.as_str()).collect();
                assert_eq!(names, ["authority.unreadable"]);
                assert_eq!(cluster.fresh_until(), None);
            }
        }
    }
}
