//! The PostgreSQL wire format of POOL, as functions over bytes.
//!
//! This crate holds the subset of the frontend and backend protocol that
//! scope 004 speaks: the first packets and their order ([`startup`]), the
//! message header and whole message framing ([`message`]), the
//! authentication and status messages ([`auth`]), the error messages that
//! POOL itself sends ([`sqlstate`]), the SCRAM message text ([`scram`]) and
//! base64 ([`base64`]). It opens no socket, starts no runtime, reads no file
//! and depends on no other crate, so it can be fuzzed on its own.
//!
//! Every reader takes its input in pieces of any size and gives the same
//! result as when it is fed the whole input at once (`0004 AC-28`). No
//! reader reserves memory from a length field before it has checked that
//! field against its limit.
//!
//! The subset is specified in spec 0004, "The wire subset"
//! (`docs/specs/0004-one-real-primary-path/index.md`).

pub mod auth;
pub mod base64;
pub mod message;
pub mod scram;
pub mod sqlstate;
pub mod startup;

/// The input broke the wire format, and nothing more is known about it.
///
/// It carries no text from the input, so it is safe to log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Malformed;

impl std::fmt::Display for Malformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("malformed protocol data")
    }
}

impl std::error::Error for Malformed {}

/// Reads a big endian `u32` at the start of `bytes`.
pub(crate) fn be_u32(bytes: &[u8]) -> Option<u32> {
    let array: [u8; 4] = bytes.get(..4)?.try_into().ok()?;
    Some(u32::from_be_bytes(array))
}

/// Reads a big endian `i32` at the start of `bytes`.
pub(crate) fn be_i32(bytes: &[u8]) -> Option<i32> {
    be_u32(bytes).map(|value| i32::from_be_bytes(value.to_be_bytes()))
}

/// Splits `bytes` at the first zero byte: the text before it, and the rest after it.
pub(crate) fn take_cstr(bytes: &[u8]) -> Option<(&[u8], &[u8])> {
    let end = bytes.iter().position(|&b| b == 0)?;
    Some((&bytes[..end], &bytes[end + 1..]))
}
