//! The data plane of POOL: secrets, TLS, SCRAM, the backend connection and
//! the session.
//!
//! This crate will hold the secret types and the reading of protected files,
//! the TLS setup as server and as client, SCRAM on both sides, the backend
//! connection, the session with its lease, pool key and limits, the relay
//! and the listener loop. It depends on `pool-config` and `pool-protocol`,
//! and never on `pool-control`: the daemon maps the control snapshot into
//! the route view of this crate.
//!
//! The design is spec 0004 (`docs/specs/0004-one-real-primary-path/index.md`).
//! Its code arrives in Build plan steps 7 and 9.
