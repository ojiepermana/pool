//! Writer admission: may a write go to a node of one cluster, and to which.

use std::time::Duration;

use pool_config::{AuthorityRevision, ClusterId, NodeId, TopologyEpoch, ValidatedConfig};

use crate::authority::{AuthorityState, StoredAuthority};
use crate::observation::{Evidence, MonotonicTime, Observation};

/// One reason why admission is closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosedReason {
    /// `authority.uninitialized`: no journal and no witness.
    Uninitialized,
    /// `authority.unconfirmed`: a journal without a witness.
    Unconfirmed,
    /// `authority.missing`: a witness without a journal.
    Missing,
    /// `authority.unreadable`: the journal cannot be read or is not consistent.
    Unreadable,
    /// `authority.mismatch`: the witness names another journal.
    Mismatch,
    /// `authority.restore_pending`: the journal waits for a reconcile.
    RestorePending,
    /// `authority.cluster_unknown`: the journal or the configuration has no
    /// such cluster.
    ClusterUnknown,
    /// `authority.config_conflict`: the configuration and the journal disagree.
    ConfigConflict,
    /// `authority.conflict_latched`: the conflict latch of the cluster is set.
    ConflictLatched,
    /// `authority.writer_quarantined`: the approved writer is quarantined.
    WriterQuarantined,
    /// `evidence.writer_missing`: no fresh observation reached the approved
    /// writer.
    WriterMissing,
    /// `evidence.writer_identity`: the approved writer reports another system
    /// identifier.
    WriterIdentity,
    /// `evidence.writer_in_recovery`: the approved writer is in recovery.
    WriterInRecovery,
    /// `evidence.writer_timeline`: the approved writer reports another
    /// timeline than the journal recorded.
    WriterTimeline,
    /// `evidence.second_primary`: another node reports that it is a primary.
    SecondPrimary,
    /// `evidence.timeline_ahead`: another node is on a later timeline.
    TimelineAhead,
}

impl ClosedReason {
    /// The reason name.
    pub fn as_str(self) -> &'static str {
        match self {
            ClosedReason::Uninitialized => "authority.uninitialized",
            ClosedReason::Unconfirmed => "authority.unconfirmed",
            ClosedReason::Missing => "authority.missing",
            ClosedReason::Unreadable => "authority.unreadable",
            ClosedReason::Mismatch => "authority.mismatch",
            ClosedReason::RestorePending => "authority.restore_pending",
            ClosedReason::ClusterUnknown => "authority.cluster_unknown",
            ClosedReason::ConfigConflict => "authority.config_conflict",
            ClosedReason::ConflictLatched => "authority.conflict_latched",
            ClosedReason::WriterQuarantined => "authority.writer_quarantined",
            ClosedReason::WriterMissing => "evidence.writer_missing",
            ClosedReason::WriterIdentity => "evidence.writer_identity",
            ClosedReason::WriterInRecovery => "evidence.writer_in_recovery",
            ClosedReason::WriterTimeline => "evidence.writer_timeline",
            ClosedReason::SecondPrimary => "evidence.second_primary",
            ClosedReason::TimelineAhead => "evidence.timeline_ahead",
        }
    }
}

/// The answer of [`writer_admission`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriterAdmission {
    /// Writes may go to this node.
    Open(EligiblePrimary),
    /// No write may go to any node of the cluster.
    Closed(Closed),
}

/// The node that may take writes, at the generations it was admitted under.
///
/// Only [`writer_admission`] makes one. Its node is always the approved
/// writer of the journal. Code outside this crate cannot build one:
///
/// ```compile_fail,E0451
/// fn build(
///     cluster: pool_config::ClusterId,
///     node: pool_config::NodeId,
///     topology_epoch: pool_config::TopologyEpoch,
///     authority_revision: pool_config::AuthorityRevision,
/// ) -> pool_control::EligiblePrimary {
///     pool_control::EligiblePrimary { cluster, node, topology_epoch, authority_revision }
/// }
/// ```
///
/// Restart rule: gone at a restart. It is made again by a fresh admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EligiblePrimary {
    cluster: ClusterId,
    node: NodeId,
    topology_epoch: TopologyEpoch,
    authority_revision: AuthorityRevision,
}

impl EligiblePrimary {
    /// The cluster.
    pub fn cluster(&self) -> &ClusterId {
        &self.cluster
    }

    /// The approved writer.
    pub fn node(&self) -> &NodeId {
        &self.node
    }

    /// The topology epoch of the cluster in the journal.
    pub fn topology_epoch(&self) -> TopologyEpoch {
        self.topology_epoch
    }

    /// The authority revision of the journal.
    pub fn authority_revision(&self) -> AuthorityRevision {
        self.authority_revision
    }
}

/// A closed admission and its reasons, never empty, each at most once, in
/// the order of the rules.
///
/// Only [`writer_admission`] makes one. Code outside this crate cannot build
/// one:
///
/// ```compile_fail,E0451
/// fn build(reasons: Vec<pool_control::ClosedReason>) -> pool_control::Closed {
///     pool_control::Closed { reasons }
/// }
/// ```
///
/// Restart rule: gone at a restart. A latched conflict survives in the
/// journal, not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closed {
    reasons: Vec<ClosedReason>,
}

impl Closed {
    fn new(reasons: Vec<ClosedReason>) -> Closed {
        debug_assert!(!reasons.is_empty());
        Closed { reasons }
    }

    /// The reasons, in the order of the rules.
    pub fn reasons(&self) -> &[ClosedReason] {
        &self.reasons
    }

    /// True when the reasons report a second primary or a later timeline. The
    /// daemon then sets the conflict latch of the cluster in the journal.
    pub fn latches_conflict(&self) -> bool {
        self.reasons
            .iter()
            .any(|r| matches!(r, ClosedReason::SecondPrimary | ClosedReason::TimelineAhead))
    }
}

/// Decides whether a write may go to a node of `cluster`, and to which one.
///
/// In any state but `Ready` the answer is closed with the one reason of that
/// state, and the observations are not looked at. In `Ready` every rule of
/// spec 0003, "Writer admission", runs and every reason that applies is
/// reported. Admission is open only when the configuration, the journal and a
/// fresh observation agree on the writer and no conflict is seen.
pub fn writer_admission(
    config: &ValidatedConfig,
    state: &AuthorityState,
    cluster: &ClusterId,
    observations: &[Observation],
    now: MonotonicTime,
    expiry: Duration,
) -> WriterAdmission {
    let authority = match state {
        AuthorityState::Ready(authority) => authority,
        other => return closed(vec![state_reason(other)]),
    };
    admit_ready(
        config,
        authority.stored(),
        cluster,
        &Evidence::new(observations, now, expiry),
    )
}

fn state_reason(state: &AuthorityState) -> ClosedReason {
    match state {
        AuthorityState::Uninitialized => ClosedReason::Uninitialized,
        AuthorityState::Unconfirmed => ClosedReason::Unconfirmed,
        AuthorityState::Missing => ClosedReason::Missing,
        AuthorityState::Mismatch => ClosedReason::Mismatch,
        AuthorityState::Held => ClosedReason::RestorePending,
        // `Ready` never reaches here; treat it as unreadable to stay closed.
        AuthorityState::Unreadable | AuthorityState::Ready(_) => ClosedReason::Unreadable,
    }
}

fn closed(reasons: Vec<ClosedReason>) -> WriterAdmission {
    WriterAdmission::Closed(Closed::new(reasons))
}

fn admit_ready(
    config: &ValidatedConfig,
    stored: &StoredAuthority,
    cluster: &ClusterId,
    evidence: &Evidence<'_>,
) -> WriterAdmission {
    let approved = stored.clusters.iter().find(|t| &t.cluster == cluster);
    let configured = config.clusters().iter().find(|c| c.id() == cluster);
    let (Some(approved), Some(configured)) = (approved, configured) else {
        return closed(vec![ClosedReason::ClusterUnknown]);
    };

    let configured_nodes: Vec<&NodeId> = config
        .nodes()
        .iter()
        .filter(|node| node.cluster() == cluster)
        .map(|node| node.id())
        .collect();
    let is_quarantined = |node: &NodeId| {
        stored
            .quarantine
            .iter()
            .any(|row| &row.cluster == cluster && &row.node == node)
    };
    let writer = &approved.approved_writer;

    let mut reasons = Vec::new();

    // 2. The configuration and the journal must agree.
    if configured.expected_writer() != writer
        || configured.system_identifier() != approved.system_identifier
        || approved
            .members
            .iter()
            .any(|member| !configured_nodes.contains(&member))
    {
        reasons.push(ClosedReason::ConfigConflict);
    }
    // 3. A latched conflict stays until an operation clears it.
    if approved.conflict_latched {
        reasons.push(ClosedReason::ConflictLatched);
    }
    // 4. A quarantined writer never takes writes.
    if is_quarantined(writer) {
        reasons.push(ClosedReason::WriterQuarantined);
    }
    // 5 to 8. Fresh evidence about the approved writer.
    match evidence.facts(writer) {
        None => reasons.push(ClosedReason::WriterMissing),
        Some(facts) if facts.system_identifier != approved.system_identifier => {
            reasons.push(ClosedReason::WriterIdentity);
        }
        Some(facts) => {
            if facts.in_recovery {
                reasons.push(ClosedReason::WriterInRecovery);
            }
            if facts.timeline != approved.writer_timeline {
                reasons.push(ClosedReason::WriterTimeline);
            }
        }
    }
    // 9 and 10. Positive conflict from another configured node of the cluster.
    let others: Vec<_> = configured_nodes
        .iter()
        .filter(|node| **node != writer && !is_quarantined(node))
        .filter_map(|node| evidence.facts(node))
        .filter(|facts| facts.system_identifier == approved.system_identifier)
        .collect();
    if others.iter().any(|facts| !facts.in_recovery) {
        reasons.push(ClosedReason::SecondPrimary);
    }
    if others
        .iter()
        .any(|facts| facts.timeline > approved.writer_timeline)
    {
        reasons.push(ClosedReason::TimelineAhead);
    }

    if reasons.is_empty() {
        WriterAdmission::Open(EligiblePrimary {
            cluster: approved.cluster.clone(),
            node: writer.clone(),
            topology_epoch: approved.topology_epoch,
            authority_revision: stored.authority_revision,
        })
    } else {
        closed(reasons)
    }
}
