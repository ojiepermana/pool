//! The errors that POOL itself sends, spec 0004 "SQLSTATEs that POOL generates".
//!
//! Every error of this slice is `FATAL` and is followed by closing the
//! connection. It has the fields `S` and `V` (both `FATAL`), `C` (the
//! SQLSTATE) and `M` (a constant message), and no other field.

use crate::message::encode_message;

/// One row of the table of errors that POOL generates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PoolError {
    /// A startup packet without TLS.
    TlsRequired,
    /// A protocol version, parameter, message or COPY that this build does not support.
    FeatureNotSupported,
    /// A packet or message that breaks the protocol.
    ProtocolViolation,
    /// A startup packet or a message above its configured size.
    MessageTooLarge,
    /// A wrong password, or a user name that is no principal.
    AuthenticationFailed,
    /// An authenticated principal with no identity entry for the database.
    NoIdentityEntry,
    /// No session permit, no principal permit, no room among the waiters, or the wait for a backend ran out.
    TooManyConnections,
    /// The writer of the cluster is not admitted.
    NotAccepting,
    /// The backend could not be opened.
    BackendNotEstablished,
    /// The backend was lost, or POOL ended the session, while a query was in flight.
    OutcomeUnknown,
    /// The backend was lost while no query was in flight.
    BackendLost,
    /// Shutdown, or the cluster closed with a positive conflict, while no query was in flight.
    EndedByProxy,
}

impl PoolError {
    /// Every row, in the order of the table of the spec.
    pub const ALL: [Self; 12] = [
        Self::TlsRequired,
        Self::FeatureNotSupported,
        Self::ProtocolViolation,
        Self::MessageTooLarge,
        Self::AuthenticationFailed,
        Self::NoIdentityEntry,
        Self::TooManyConnections,
        Self::NotAccepting,
        Self::BackendNotEstablished,
        Self::OutcomeUnknown,
        Self::BackendLost,
        Self::EndedByProxy,
    ];

    /// The five character SQLSTATE.
    pub fn sqlstate(self) -> &'static str {
        match self {
            Self::TlsRequired | Self::NoIdentityEntry => "28000",
            Self::FeatureNotSupported => "0A000",
            Self::ProtocolViolation => "08P01",
            Self::MessageTooLarge => "54000",
            Self::AuthenticationFailed => "28P01",
            Self::TooManyConnections => "53300",
            Self::NotAccepting => "57P03",
            Self::BackendNotEstablished | Self::BackendLost => "08006",
            Self::OutcomeUnknown => "08007",
            Self::EndedByProxy => "57P01",
        }
    }

    /// The constant message text. It never holds a name, an address or a varying number.
    pub fn message(self) -> &'static str {
        match self {
            Self::TlsRequired => "TLS is required",
            Self::FeatureNotSupported => "this protocol feature is not supported",
            Self::ProtocolViolation => "protocol violation",
            Self::MessageTooLarge => "a message is larger than the configured limit",
            Self::AuthenticationFailed => "authentication failed",
            Self::NoIdentityEntry => "no identity entry for this user and database",
            Self::TooManyConnections => "too many connections",
            Self::NotAccepting => "the database is not accepting connections through this proxy",
            Self::BackendNotEstablished => "the backend connection could not be established",
            Self::OutcomeUnknown => {
                "the connection was ended; the outcome of the statement in flight is unknown"
            }
            Self::BackendLost => "the backend connection was lost",
            Self::EndedByProxy => "the session was ended by the proxy",
        }
    }

    /// The whole `ErrorResponse` message, tag and length included.
    pub fn encode(self) -> Vec<u8> {
        let mut body = Vec::with_capacity(64);
        for (field, value) in [
            (b'S', "FATAL"),
            (b'V', "FATAL"),
            (b'C', self.sqlstate()),
            (b'M', self.message()),
        ] {
            body.push(field);
            body.extend_from_slice(value.as_bytes());
            body.push(0);
        }
        body.push(0);
        encode_message(b'E', &body)
    }
}

/// Returns the SQLSTATE field (`C`) of the body of an `ErrorResponse` or `NoticeResponse`.
///
/// Used for the log event of a failed backend attempt. Returns `None` when
/// the body is malformed or the field is missing or not five ASCII
/// alphanumeric characters, so no other text of the backend leaves this call.
pub fn error_sqlstate(body: &[u8]) -> Option<[u8; 5]> {
    let mut rest = body;
    loop {
        let (&field, after) = rest.split_first()?;
        if field == 0 {
            return None;
        }
        let (value, next) = crate::take_cstr(after)?;
        if field == b'C' {
            let code: [u8; 5] = value.try_into().ok()?;
            return code.iter().all(u8::is_ascii_alphanumeric).then_some(code);
        }
        rest = next;
    }
}
