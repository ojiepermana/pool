//! Observations: what one probe sample saw on one node, and the freshness
//! rule that decides whether a sample counts as evidence.

use std::collections::HashMap;
use std::time::Duration;

use pool_config::{NodeId, SystemIdentifier};

/// A PostgreSQL timeline: a whole number from 1 to 4294967295.
///
/// It compares only with another `TimelineId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimelineId(u32);

impl TimelineId {
    /// Builds a timeline from the number a node reported. Returns `None` for
    /// 0, which PostgreSQL never uses.
    pub fn from_observed(value: u32) -> Option<TimelineId> {
        (value >= 1).then_some(TimelineId(value))
    }

    /// The timeline number.
    pub fn get(self) -> u32 {
        self.0
    }
}

/// A point on the monotonic clock, as a time since a start point the caller
/// chose.
///
/// This crate never reads a clock. The daemon reads its monotonic clock and
/// passes the value; tests pass fixed values. Two values compare only when
/// they were measured from the same start point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonotonicTime(Duration);

impl MonotonicTime {
    /// The time that lies `since_origin` after the caller's start point.
    pub fn from_origin(since_origin: Duration) -> MonotonicTime {
        MonotonicTime(since_origin)
    }

    /// The time since the caller's start point.
    pub fn since_origin(self) -> Duration {
        self.0
    }
}

/// What a probe that reached a node read from it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodeFacts {
    /// The system identifier the node reported.
    pub system_identifier: SystemIdentifier,
    /// The timeline the node writes on or replays on.
    pub timeline: TimelineId,
    /// The result of `pg_is_in_recovery()`.
    pub in_recovery: bool,
}

/// The outcome of one probe sample.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationOutcome {
    /// The probe reached the node and read these facts.
    Reached(NodeFacts),
    /// The probe did not reach the node or could not read its facts. This is
    /// no evidence of anything.
    Failed,
}

/// One probe sample of one node.
///
/// An observation is evidence only. It never sets or changes the approved
/// writer; it can only close admission.
///
/// Restart rule: none survives a restart. It lives in memory only, and
/// admission stays closed until fresh observations exist again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    node: NodeId,
    sampled_at: MonotonicTime,
    outcome: ObservationOutcome,
}

impl Observation {
    /// Records one sample of `node`, taken at `sampled_at`.
    pub fn new(
        node: NodeId,
        sampled_at: MonotonicTime,
        outcome: ObservationOutcome,
    ) -> Observation {
        Observation {
            node,
            sampled_at,
            outcome,
        }
    }

    /// The node that was sampled.
    pub fn node(&self) -> &NodeId {
        &self.node
    }

    /// When the sample was taken, on the monotonic clock.
    pub fn sampled_at(&self) -> MonotonicTime {
        self.sampled_at
    }

    /// What the sample saw.
    pub fn outcome(&self) -> ObservationOutcome {
        self.outcome
    }
}

/// The samples that admission reads: at most one per node (`0003 AC-18`).
///
/// Recording a sample of a node keeps the one with the later sample time,
/// and the one already held when the times are equal. So the list it
/// returns never holds two samples of one node, and the rule "Fresh" of spec
/// 0003 never meets a tie. A failed sample is recorded like any other: it
/// replaces the last good sample of its node, so a node that stops answering
/// loses its evidence at once.
///
/// Restart rule: memory only. A start begins with an empty board, and
/// admission stays closed until fresh samples exist again.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObservationBoard {
    observations: Vec<Observation>,
    index: HashMap<NodeId, usize>,
}

impl ObservationBoard {
    /// An empty board.
    pub fn new() -> ObservationBoard {
        ObservationBoard::default()
    }

    /// Records one sample. It replaces the sample of the same node only when
    /// it was taken later.
    pub fn record(&mut self, observation: Observation) {
        match self.index.get(&observation.node) {
            Some(&position) => {
                if let Some(held) = self.observations.get_mut(position)
                    && observation.sampled_at > held.sampled_at
                {
                    *held = observation;
                }
            }
            None => {
                self.index
                    .insert(observation.node.clone(), self.observations.len());
                self.observations.push(observation);
            }
        }
    }

    /// The samples, one per node, in the order their nodes were first
    /// recorded.
    pub fn observations(&self) -> &[Observation] {
        &self.observations
    }

    /// The sample of `node`, if the board holds one.
    pub(crate) fn get(&self, node: &NodeId) -> Option<&Observation> {
        self.index
            .get(node)
            .and_then(|&position| self.observations.get(position))
    }

    /// The latest sample time on the board, or `None` when it is empty.
    pub(crate) fn latest(&self) -> Option<MonotonicTime> {
        self.observations.iter().map(|o| o.sampled_at).max()
    }

    /// The later of `now` and the latest sample time: the now that
    /// [`crate::build_snapshot`] and [`crate::JournalRead::plan_bootstrap`]
    /// use. A later now only makes samples older, so it can close and never
    /// open.
    pub(crate) fn clamp(&self, now: MonotonicTime) -> MonotonicTime {
        self.latest().map_or(now, |latest| latest.max(now))
    }
}

/// The evidence the functions of this crate read about nodes: for each node,
/// the facts of its latest sample when that sample is fresh and reached it.
pub(crate) struct Evidence<'a> {
    observations: &'a [Observation],
    now: MonotonicTime,
    expiry: Duration,
}

impl<'a> Evidence<'a> {
    pub(crate) fn new(
        observations: &'a [Observation],
        now: MonotonicTime,
        expiry: Duration,
    ) -> Evidence<'a> {
        Evidence {
            observations,
            now,
            expiry,
        }
    }

    /// The facts of `node`, or `None` when the node has no evidence.
    ///
    /// The latest sample of the node is used. Two samples that share the
    /// latest time give no evidence. The latest sample is fresh when it is not
    /// later than `now` and its age is less than the expiry. A sample that is
    /// not fresh, or that failed, gives no evidence.
    pub(crate) fn facts(&self, node: &NodeId) -> Option<NodeFacts> {
        let mut latest: Option<&Observation> = None;
        let mut tied = false;
        for observation in self.observations.iter().filter(|o| &o.node == node) {
            match latest {
                Some(current) if observation.sampled_at < current.sampled_at => {}
                Some(current) if observation.sampled_at == current.sampled_at => tied = true,
                _ => {
                    latest = Some(observation);
                    tied = false;
                }
            }
        }
        let latest = latest?;
        if tied {
            return None;
        }
        let age = self.now.0.checked_sub(latest.sampled_at.0)?;
        if age >= self.expiry {
            return None;
        }
        match latest.outcome {
            ObservationOutcome::Reached(facts) => Some(facts),
            ObservationOutcome::Failed => None,
        }
    }
}
