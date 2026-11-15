//! Durable authority as the journal reader found it, and the classification
//! of a start.

use std::collections::BTreeSet;

use pool_config::{
    AuthorityId, AuthorityRevision, ClusterId, NodeId, StartSequence, SystemIdentifier,
    TopologyEpoch, ValidatedConfig,
};

use crate::observation::TimelineId;

/// What the journal reader found in the state directory.
///
/// In the product only the journal reader of scope 004 builds one. Tests
/// build one by hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreProbe {
    /// The state directory holds no journal file.
    Absent,
    /// A file is there but cannot be opened, is not a POOL journal, has a
    /// newer schema, or fails its integrity check.
    Unreadable,
    /// A journal was read.
    Present(StoredAuthority),
}

/// The journal identity (the `store` row) with everything the journal holds
/// about authority.
///
/// Restart rule: it survives a restart, in the journal. At every start it is
/// read first, without writing, and classified by [`classify_startup`]; only
/// a `Ready` start advances the start sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredAuthority {
    /// The identity of the journal, made by bootstrap.
    pub authority_id: AuthorityId,
    /// The revision of durable authority.
    pub authority_revision: AuthorityRevision,
    /// The number of starts that found this journal `Ready`.
    pub start_sequence: StartSequence,
    /// Whether the journal waits for a reconcile after a restore.
    pub hold: Hold,
    /// The approved topology of each cluster the journal holds.
    pub clusters: Vec<ApprovedTopology>,
    /// The quarantined nodes.
    pub quarantine: Vec<Quarantine>,
}

/// The hold of a journal.
///
/// Restart rule: it survives a restart, in the journal, and is cleared only
/// by the reconcile operation of scope 009.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hold {
    /// No hold: the journal is the live one.
    None,
    /// The journal came out of a supported backup and waits for a reconcile.
    RestorePending,
}

/// The approved topology of one cluster: its approved writer, its members and
/// its conflict latch.
///
/// Restart rule: it survives a restart, in the journal. It is loaded and then
/// compared with the configuration and with fresh observations before any
/// write is admitted. An observation never changes it, except that the daemon
/// may set the conflict latch, which only restricts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovedTopology {
    /// The cluster.
    pub cluster: ClusterId,
    /// The system identifier every node of the cluster must report.
    pub system_identifier: SystemIdentifier,
    /// The one node that may take writes.
    pub approved_writer: NodeId,
    /// The timeline the writer reported when it was approved.
    pub writer_timeline: TimelineId,
    /// The topology epoch of the cluster.
    pub topology_epoch: TopologyEpoch,
    /// The member nodes, the approved writer among them.
    pub members: Vec<NodeId>,
    /// Set when a second primary or a later timeline was seen. Cleared only by
    /// an operation.
    pub conflict_latched: bool,
}

/// One quarantined node.
///
/// Restart rule: it survives a restart, in the journal, and is still in force
/// after it. A healthy probe never clears it; only an operation with rejoin
/// evidence does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quarantine {
    /// The cluster of the node.
    pub cluster: ClusterId,
    /// The quarantined node.
    pub node: NodeId,
}

/// The class of a start.
///
/// Only `Ready` carries an [`Authority`], so no other state can lead to an
/// open admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityState {
    /// No journal and no witness: a machine that was never bootstrapped.
    Uninitialized,
    /// A journal exists, but the configuration carries no witness for it.
    Unconfirmed,
    /// The configuration carries a witness, but there is no journal.
    Missing,
    /// The journal cannot be read or is not consistent.
    Unreadable,
    /// The witness names another journal.
    Mismatch,
    /// The journal waits for a reconcile after a restore.
    Held,
    /// The witness confirms the journal and nothing holds it.
    Ready(Authority),
}

/// Durable authority that the configuration confirmed.
///
/// Only [`classify_startup`] makes one. Code outside this crate cannot build
/// one:
///
/// ```compile_fail,E0451
/// fn build(stored: pool_control::StoredAuthority) -> pool_control::Authority {
///     pool_control::Authority { stored }
/// }
/// ```
///
/// Restart rule: it lives in memory for one run and is made again by
/// classifying the journal at the next start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authority {
    stored: StoredAuthority,
}

impl Authority {
    /// The identity of the confirmed journal.
    pub fn authority_id(&self) -> AuthorityId {
        self.stored.authority_id
    }

    /// The revision of durable authority as it was read.
    pub fn authority_revision(&self) -> AuthorityRevision {
        self.stored.authority_revision
    }

    /// The start sequence as it was read, before this start advances it.
    pub fn start_sequence(&self) -> StartSequence {
        self.stored.start_sequence
    }

    pub(crate) fn stored(&self) -> &StoredAuthority {
        &self.stored
    }
}

/// Classifies a start from what the journal reader found and from the
/// witness `control.authority_id` of the configuration.
///
/// A journal that names a cluster twice, names a member of a cluster twice,
/// has an approved writer outside its members, or has a quarantine row for a
/// cluster it does not hold, is `Unreadable`.
pub fn classify_startup(config: &ValidatedConfig, probe: StoreProbe) -> AuthorityState {
    let witness = config.control().and_then(|control| control.authority_id());
    match (probe, witness) {
        (StoreProbe::Absent, None) => AuthorityState::Uninitialized,
        (StoreProbe::Absent, Some(_)) => AuthorityState::Missing,
        (StoreProbe::Unreadable, _) => AuthorityState::Unreadable,
        (StoreProbe::Present(stored), witness) => {
            if !is_consistent(&stored) {
                return AuthorityState::Unreadable;
            }
            match witness {
                None => AuthorityState::Unconfirmed,
                Some(witness) if witness != stored.authority_id => AuthorityState::Mismatch,
                Some(_) => match stored.hold {
                    Hold::RestorePending => AuthorityState::Held,
                    Hold::None => AuthorityState::Ready(Authority { stored }),
                },
            }
        }
    }
}

fn is_consistent(stored: &StoredAuthority) -> bool {
    let mut clusters = BTreeSet::new();
    for topology in &stored.clusters {
        if !clusters.insert(&topology.cluster) {
            return false;
        }
        let mut members = BTreeSet::new();
        if !topology.members.iter().all(|member| members.insert(member)) {
            return false;
        }
        if !members.contains(&topology.approved_writer) {
            return false;
        }
    }
    stored
        .quarantine
        .iter()
        .all(|row| clusters.contains(&row.cluster))
}
