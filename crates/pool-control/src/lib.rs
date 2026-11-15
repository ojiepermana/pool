//! The authority model of the POOL control plane.
//!
//! This crate will hold the rules that classify a start, decide when a
//! bootstrap may happen and decide when writes to a cluster are admitted, as
//! plain functions over values. It reads no file, no clock and no socket: the
//! caller reads the journal, the clock and the probes and passes what it
//! found. Its one dependency is `pool-config`, and it never depends on
//! `pool-core`.
//!
//! The rules are specified in spec 0003, "The authority model in
//! pool-control" (`docs/specs/0003-control-and-session-data-model/index.md`).
// provisional: F3 (Build plan step 5 of spec 0003 adds the authority model)
