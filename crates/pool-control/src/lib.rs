//! The authority model of the POOL control plane.
//!
//! This crate holds the rules that classify a start ([`classify_startup`]),
//! decide when a bootstrap may happen ([`plan_bootstrap`]) and decide when
//! writes to a cluster are admitted ([`writer_admission`]), as plain functions
//! over values. Around them it holds what scope 004 connects: the journal on
//! disk ([`Journal`], the one file that names its SQLite binding), the board
//! of probe samples ([`ObservationBoard`]), the topology snapshot
//! ([`build_snapshot`]), the controller of the control loop ([`Controller`])
//! and the probe query ([`PROBE_QUERY`]).
//!
//! It reads no clock, no environment and no socket, and it starts nothing
//! that runs on its own: the caller reads the clock and the probes and passes
//! what it found. Its dependencies are `pool-config` and the SQLite binding,
//! and it never depends on `pool-core`.
//!
//! Other crates never hold a journal value. They get a [`JournalRead`] from
//! [`Journal::read`], an [`AuthorityState`] from [`JournalRead::classify`]
//! and a [`BootstrapPlan`] from [`JournalRead::bootstrap_plan`]. A source
//! guard keeps the names of the journal value and of the three rule
//! functions out of every other crate (spec 0003, `AC-18` and `AC-19`).
//!
//! Durable authority (which node may take writes) and observed database state
//! (what a probe just saw) are separate. An observation never sets or changes
//! the approved writer. Writes are admitted only when the configuration, the
//! journal and a fresh observation all agree; every other state ends closed.
//!
//! The rules are specified in spec 0003, "The authority model in
//! pool-control" (`docs/specs/0003-control-and-session-data-model/index.md`).

mod admission;
mod authority;
mod bootstrap;
mod journal;
mod observation;
mod probe;
mod snapshot;

pub use admission::{Closed, ClosedReason, EligiblePrimary, WriterAdmission, writer_admission};
pub use authority::{
    ApprovedTopology, Authority, AuthorityState, Hold, Quarantine, StoreProbe, StoredAuthority,
    classify_startup,
};
pub use bootstrap::{BootstrapPlan, BootstrapRefusal, BootstrapRefused, plan_bootstrap};
pub use journal::{Journal, JournalError, JournalRead, LatchReceipt};
pub use observation::{
    MonotonicTime, NodeFacts, Observation, ObservationBoard, ObservationOutcome, TimelineId,
};
pub use probe::{PROBE_QUERY, observation_from_row};
pub use snapshot::{
    ClusterAdmission, Controller, SnapshotGeneration, TopologySnapshot, build_snapshot,
};
