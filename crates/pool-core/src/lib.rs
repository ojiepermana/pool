//! The data plane of POOL: secrets, TLS, SCRAM, the backend connection and
//! the session.
//!
//! This crate holds the secret types and the reading of protected files
//! ([`secret`]), the TLS setup as server and as client ([`tls`]), SCRAM on
//! both sides ([`scram`]) and the backend connection with its one row query
//! ([`backend`]), and the reading of what the configuration names ([`load`]). The route view ([`route`]), the session records
//! ([`records`]), the log events of the data plane ([`report`]) and the
//! session with its limits, relay, backend loss, conflict ending and
//! shutdown ([`session`]) are Build plan step 9. It depends on `pool-config`
//! and `pool-protocol`, and never on `pool-control`: the daemon maps the
//! control snapshot into the route view of this crate.
//!
//! Secrets are held in types whose `Debug` prints `[REDACTED]`, and no error
//! of this crate carries text from a secret, a file, a name or a peer
//! (`0004 AC-24`, `SEC-11`).
//!
//! The design is spec 0004 (`docs/specs/0004-one-real-primary-path/index.md`).

pub mod backend;
pub mod load;
pub mod records;
pub mod report;
pub mod route;
pub mod scram;
pub mod secret;
pub mod session;
pub mod tls;

mod wire;

/// The text that `Debug` prints for every secret.
pub(crate) const REDACTED: &str = "[REDACTED]";
