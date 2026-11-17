//! Durable authority as the journal reader found it, and the classification
//! of a start.

use std::collections::BTreeSet;

use pool_config::{
    AuthorityId, AuthorityRevision, ClusterId, NodeId, StartSequence, SystemIdentifier,
    TopologyEpoch, ValidatedConfig,
};

use crate::admission::Closed;
use crate::journal::LatchReceipt;
use crate::observation::TimelineId;

/// What the journal reader found in the state directory.
///
/// In the product only the journal reader, [`crate::Journal::read`], builds
/// one, and it keeps it inside a [`crate::JournalRead`]. A source guard keeps
/// this name out of every other crate (`0003 AC-19`). Tests of this crate
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
/// The fields are public so that the journal reader can fill it and the
/// tests of this crate can build one by hand. No other crate names this
/// type: a source guard refuses it outside `pool-control` (`0003 AC-19`).
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

impl AuthorityState {
    /// The stable name of the state: `uninitialized`, `unconfirmed`,
    /// `missing`, `unreadable`, `mismatch`, `restore_pending` or `ready`.
    /// The commands build their output and their findings from it.
    pub fn name(&self) -> &'static str {
        match self {
            AuthorityState::Uninitialized => "uninitialized",
            AuthorityState::Unconfirmed => "unconfirmed",
            AuthorityState::Missing => "missing",
            AuthorityState::Unreadable => "unreadable",
            AuthorityState::Mismatch => "mismatch",
            AuthorityState::Held => "restore_pending",
            AuthorityState::Ready(_) => "ready",
        }
    }

    /// Sets the conflict latch of `cluster` in memory when `closed` asks for
    /// it, without any write to the journal (`0003 AC-17`).
    ///
    /// On a `Ready` state that holds `cluster`, an answer whose
    /// [`Closed::latches_conflict`] is true sets the latch. From then on
    /// [`crate::writer_admission`] closes `cluster` with
    /// `authority.conflict_latched`, also when the second primary is no
    /// longer seen. Every other cluster keeps its answer. An answer that does
    /// not ask for the latch, a state that is not `Ready`, or a cluster the
    /// state does not hold change nothing.
    ///
    /// Returns true when the latch of `cluster` is set in the state after the
    /// call, whether this call set it or it was set before. The write to the
    /// journal follows: the daemon calls [`crate::Journal::set_conflict_latch`]
    /// and hands the receipt to [`AuthorityState::latch_recorded`].
    pub fn latch_conflict(&mut self, cluster: &ClusterId, closed: &Closed) -> bool {
        let AuthorityState::Ready(authority) = self else {
            return false;
        };
        let Some(topology) = authority
            .stored
            .clusters
            .iter_mut()
            .find(|topology| &topology.cluster == cluster)
        else {
            return false;
        };
        if closed.latches_conflict() {
            topology.conflict_latched = true;
        }
        topology.conflict_latched
    }

    /// Takes the authority revision of `receipt`, the proof that the journal
    /// holds the latch of its cluster (`0003 AC-17`).
    ///
    /// It changes the state only when it is `Ready`, the latch of the
    /// receipt's cluster is set in it, and the revision of the receipt is
    /// higher than the one the state holds. From then on an open answer for
    /// another cluster carries that revision.
    pub fn latch_recorded(&mut self, receipt: &LatchReceipt) {
        let AuthorityState::Ready(authority) = self else {
            return;
        };
        let latched =
            authority.stored.clusters.iter().any(|topology| {
                &topology.cluster == receipt.cluster() && topology.conflict_latched
            });
        // Receipts can be handed over in another order than they were
        // written; the state keeps the highest revision it has seen.
        if latched && receipt.authority_revision() > authority.stored.authority_revision {
            authority.stored.authority_revision = receipt.authority_revision();
        }
    }

    /// Whether the conflict latch of `cluster` is set in a `Ready` state.
    pub(crate) fn is_latched(&self, cluster: &ClusterId) -> bool {
        match self {
            AuthorityState::Ready(authority) => authority
                .stored
                .clusters
                .iter()
                .any(|topology| &topology.cluster == cluster && topology.conflict_latched),
            _ => false,
        }
    }
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
/// The same paths through the public functions compile, so the snippet
/// above can fail only because the field is private (the code after the
/// comma is not checked by the stable compiler):
///
/// ```
/// fn read(
///     stored: pool_control::StoredAuthority,
///     authority: &pool_control::Authority,
/// ) -> pool_config::AuthorityRevision {
///     drop(stored);
///     authority.authority_revision()
/// }
/// ```
///
/// Restart rule: it lives in memory for one run and is made again by
/// classifying the journal at the next start. During the run it can only
/// become stricter: [`AuthorityState::latch_conflict`] sets a conflict latch,
/// and [`AuthorityState::latch_recorded`] takes the revision of that write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authority {
    stored: StoredAuthority,
}

impl Authority {
    /// The identity of the confirmed journal.
    pub fn authority_id(&self) -> AuthorityId {
        self.stored.authority_id
    }

    /// The revision of durable authority: as it was read, or the revision of
    /// the last latch receipt the state took.
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
