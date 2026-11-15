//! The PostgreSQL wire format of POOL, as functions over bytes.
//!
//! This crate will hold the subset of the frontend and backend protocol that
//! scope 004 speaks: the first packets, startup parameters, message headers,
//! the error message, the authentication messages, `NegotiateProtocolVersion`,
//! the SCRAM message text and base64. It opens no socket, starts no runtime
//! and depends on no other crate, so it can be fuzzed on its own.
//!
//! The subset is specified in spec 0004, "The wire subset"
//! (`docs/specs/0004-one-real-primary-path/index.md`). Its code arrives in
//! Build plan step 4.
