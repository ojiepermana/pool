//! The authority model of the POOL control plane.
//!
//! This crate holds the rules that classify a start ([`classify_startup`]),
//! decide when a bootstrap may happen ([`plan_bootstrap`]) and decide when
//! writes to a cluster are admitted ([`writer_admission`]), as plain functions
//! over values. It reads no file, no clock and no socket: the caller reads the
//! journal, the clock and the probes and passes what it found. Its one
//! dependency is `pool-config`, and it never depends on `pool-core`.
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
mod observation;

pub use admission::{Closed, ClosedReason, EligiblePrimary, WriterAdmission, writer_admission};
pub use authority::{
    ApprovedTopology, Authority, AuthorityState, Hold, Quarantine, StoreProbe, StoredAuthority,
    classify_startup,
};
pub use bootstrap::{BootstrapPlan, BootstrapRefusal, BootstrapRefused, plan_bootstrap};
pub use observation::{MonotonicTime, NodeFacts, Observation, ObservationOutcome, TimelineId};
