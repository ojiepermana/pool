//! Authentication and backend startup messages (spec 0004, "Frontend
//! authentication" and "Backend connection").
//!
//! Each parser takes the body of a whole message, as [`crate::message::MessageReader`]
//! returns it, and each encoder returns the whole message with tag and length.

use crate::message::encode_message;
use crate::{Malformed, be_i32, be_u32, take_cstr};

/// The one SASL mechanism of this slice.
pub const SCRAM_SHA_256: &str = "SCRAM-SHA-256";
/// The mechanism with channel binding, which POOL never offers or chooses.
pub const SCRAM_SHA_256_PLUS: &str = "SCRAM-SHA-256-PLUS";
/// The largest authentication message that POOL reads from a client, body included.
pub const AUTH_MESSAGE_MAX_BYTES: u32 = 1024;

/// An `Authentication` message (tag `R`) from a node.
#[derive(Clone, PartialEq, Eq)]
pub enum Authentication {
    /// `AuthenticationOk`.
    Ok,
    /// `AuthenticationSASL` with the offered mechanisms.
    Sasl(Vec<String>),
    /// `AuthenticationSASLContinue` with the server-first message.
    SaslContinue(Vec<u8>),
    /// `AuthenticationSASLFinal` with the server-final message.
    SaslFinal(Vec<u8>),
    /// Any other method, by its code: clear text (3), MD5 (5), GSS, SSPI and so on.
    Other(i32),
}

impl std::fmt::Debug for Authentication {
    /// The kind, the mechanism names a node offered, and the length of a SCRAM message; never the message.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Authentication::Ok"),
            Self::Sasl(mechanisms) => f
                .debug_tuple("Authentication::Sasl")
                .field(&mechanisms.len())
                .finish(),
            Self::SaslContinue(data) => f
                .debug_struct("Authentication::SaslContinue")
                .field("len", &data.len())
                .finish(),
            Self::SaslFinal(data) => f
                .debug_struct("Authentication::SaslFinal")
                .field("len", &data.len())
                .finish(),
            Self::Other(code) => f.debug_tuple("Authentication::Other").field(code).finish(),
        }
    }
}

const OK: i32 = 0;
const SASL: i32 = 10;
const SASL_CONTINUE: i32 = 11;
const SASL_FINAL: i32 = 12;

/// Parses the body of an `Authentication` message.
pub fn parse_authentication(body: &[u8]) -> Result<Authentication, Malformed> {
    let code = be_i32(body).ok_or(Malformed)?;
    let rest = &body[4..];
    match code {
        OK if rest.is_empty() => Ok(Authentication::Ok),
        OK => Err(Malformed),
        SASL => {
            let mut mechanisms = Vec::new();
            let mut rest = rest;
            loop {
                let (name, after) = take_cstr(rest).ok_or(Malformed)?;
                if name.is_empty() {
                    if !after.is_empty() || mechanisms.is_empty() {
                        return Err(Malformed);
                    }
                    return Ok(Authentication::Sasl(mechanisms));
                }
                let name = std::str::from_utf8(name).map_err(|_| Malformed)?;
                mechanisms.push(name.to_string());
                rest = after;
            }
        }
        SASL_CONTINUE => Ok(Authentication::SaslContinue(rest.to_vec())),
        SASL_FINAL => Ok(Authentication::SaslFinal(rest.to_vec())),
        other => Ok(Authentication::Other(other)),
    }
}

fn authentication(code: i32, data: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + data.len());
    body.extend_from_slice(&code.to_be_bytes());
    body.extend_from_slice(data);
    encode_message(b'R', &body)
}

/// `AuthenticationOk`.
pub fn encode_authentication_ok() -> Vec<u8> {
    authentication(OK, &[])
}

/// `AuthenticationSASL` with these mechanisms, in order. POOL offers only [`SCRAM_SHA_256`].
pub fn encode_authentication_sasl(mechanisms: &[&str]) -> Vec<u8> {
    let mut data = Vec::new();
    for name in mechanisms {
        data.extend_from_slice(name.as_bytes());
        data.push(0);
    }
    data.push(0);
    authentication(SASL, &data)
}

/// `AuthenticationSASLContinue` with the server-first message.
pub fn encode_authentication_sasl_continue(data: &[u8]) -> Vec<u8> {
    authentication(SASL_CONTINUE, data)
}

/// `AuthenticationSASLFinal` with the server-final message.
pub fn encode_authentication_sasl_final(data: &[u8]) -> Vec<u8> {
    authentication(SASL_FINAL, data)
}

/// An `Authentication` message with any code, for scripted nodes in tests.
pub fn encode_authentication_other(code: i32, data: &[u8]) -> Vec<u8> {
    authentication(code, data)
}

/// The body of a `SASLInitialResponse` (tag `p`) from a client.
#[derive(Clone, PartialEq, Eq)]
pub struct SaslInitialResponse {
    /// The chosen mechanism.
    pub mechanism: String,
    /// The client-first message, or `None` when the length field is -1.
    pub data: Option<Vec<u8>>,
}

impl std::fmt::Debug for SaslInitialResponse {
    /// The length of the client-first message; never the message, which holds a nonce and a name.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaslInitialResponse")
            .field("mechanism", &"[REDACTED]")
            .field("data_len", &self.data.as_ref().map(Vec::len))
            .finish()
    }
}

/// Parses the body of a `SASLInitialResponse`.
///
/// The length field of the data must be -1 or exactly the number of bytes
/// that follow it. Nothing is reserved from it.
pub fn parse_sasl_initial_response(body: &[u8]) -> Result<SaslInitialResponse, Malformed> {
    let (mechanism, rest) = take_cstr(body).ok_or(Malformed)?;
    let mechanism = std::str::from_utf8(mechanism)
        .map_err(|_| Malformed)?
        .to_string();
    let len = be_i32(rest).ok_or(Malformed)?;
    let data = &rest[4..];
    let data = match len {
        -1 if data.is_empty() => None,
        n if usize::try_from(n).ok() == Some(data.len()) => Some(data.to_vec()),
        _ => return Err(Malformed),
    };
    Ok(SaslInitialResponse { mechanism, data })
}

/// `SASLInitialResponse` with this mechanism and client-first message.
///
/// # Panics
///
/// When `data` is longer than `i32::MAX` bytes.
pub fn encode_sasl_initial_response(mechanism: &str, data: &[u8]) -> Vec<u8> {
    let len = i32::try_from(data.len()).expect("SASL data below 2 GiB");
    let mut body = Vec::with_capacity(mechanism.len() + 5 + data.len());
    body.extend_from_slice(mechanism.as_bytes());
    body.push(0);
    body.extend_from_slice(&len.to_be_bytes());
    body.extend_from_slice(data);
    encode_message(b'p', &body)
}

/// `SASLResponse` with the client-final message. Its body is the message itself.
pub fn encode_sasl_response(data: &[u8]) -> Vec<u8> {
    encode_message(b'p', data)
}

/// `NegotiateProtocolVersion`: the newest minor version POOL speaks and the options it does not support.
///
/// # Panics
///
/// When more than `u32::MAX` names are given.
pub fn encode_negotiate_protocol_version(minor: u16, unsupported: &[Vec<u8>]) -> Vec<u8> {
    let count = u32::try_from(unsupported.len()).expect("fewer than 4 G names");
    let mut body = Vec::new();
    body.extend_from_slice(&u32::from(minor).to_be_bytes());
    body.extend_from_slice(&count.to_be_bytes());
    for name in unsupported {
        body.extend_from_slice(name);
        body.push(0);
    }
    encode_message(b'v', &body)
}

/// The body of a `NegotiateProtocolVersion`, for scripted clients in tests.
#[derive(Clone, PartialEq, Eq)]
pub struct NegotiateProtocolVersion {
    /// The newest minor version the server speaks.
    pub minor: u32,
    /// The options it does not support.
    pub unsupported: Vec<Vec<u8>>,
}

impl std::fmt::Debug for NegotiateProtocolVersion {
    /// The minor version and the count of option names; never the names, which a client wrote.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NegotiateProtocolVersion")
            .field("minor", &self.minor)
            .field("unsupported", &self.unsupported.len())
            .finish()
    }
}

/// Parses the body of a `NegotiateProtocolVersion`. The count is checked against the names, never reserved from.
pub fn parse_negotiate_protocol_version(
    body: &[u8],
) -> Result<NegotiateProtocolVersion, Malformed> {
    let minor = be_u32(body).ok_or(Malformed)?;
    let count = be_u32(body.get(4..).ok_or(Malformed)?).ok_or(Malformed)?;
    let mut rest = &body[8..];
    let mut unsupported = Vec::new();
    while !rest.is_empty() {
        let (name, after) = take_cstr(rest).ok_or(Malformed)?;
        unsupported.push(name.to_vec());
        rest = after;
    }
    if usize::try_from(count).ok() != Some(unsupported.len()) {
        return Err(Malformed);
    }
    Ok(NegotiateProtocolVersion { minor, unsupported })
}

/// The status byte of `ReadyForQuery`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionStatus {
    /// `I`: not in a transaction.
    Idle,
    /// `T`: in a transaction.
    InTransaction,
    /// `E`: in a failed transaction.
    Failed,
}

impl TransactionStatus {
    /// The status byte.
    pub fn byte(self) -> u8 {
        match self {
            Self::Idle => b'I',
            Self::InTransaction => b'T',
            Self::Failed => b'E',
        }
    }
}

/// Parses the body of `ReadyForQuery`: exactly one status byte.
pub fn parse_ready_for_query(body: &[u8]) -> Result<TransactionStatus, Malformed> {
    match body {
        [b'I'] => Ok(TransactionStatus::Idle),
        [b'T'] => Ok(TransactionStatus::InTransaction),
        [b'E'] => Ok(TransactionStatus::Failed),
        _ => Err(Malformed),
    }
}

/// `ReadyForQuery` with this status.
pub fn encode_ready_for_query(status: TransactionStatus) -> Vec<u8> {
    encode_message(b'Z', &[status.byte()])
}

/// One `ParameterStatus` (tag `S`): a name and a value.
#[derive(Clone, PartialEq, Eq)]
pub struct ParameterStatus {
    /// The parameter name.
    pub name: Vec<u8>,
    /// Its value.
    pub value: Vec<u8>,
}

impl std::fmt::Debug for ParameterStatus {
    /// The lengths of name and value; never their text.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParameterStatus")
            .field("name_len", &self.name.len())
            .field("value_len", &self.value.len())
            .finish()
    }
}

/// Parses the body of a `ParameterStatus`: two texts each ended by a zero byte, and nothing after.
pub fn parse_parameter_status(body: &[u8]) -> Result<ParameterStatus, Malformed> {
    let (name, rest) = take_cstr(body).ok_or(Malformed)?;
    let (value, rest) = take_cstr(rest).ok_or(Malformed)?;
    if name.is_empty() || !rest.is_empty() {
        return Err(Malformed);
    }
    Ok(ParameterStatus {
        name: name.to_vec(),
        value: value.to_vec(),
    })
}

/// `ParameterStatus`, for scripted nodes in tests.
pub fn encode_parameter_status(name: &str, value: &str) -> Vec<u8> {
    let mut body = Vec::with_capacity(name.len() + value.len() + 2);
    body.extend_from_slice(name.as_bytes());
    body.push(0);
    body.extend_from_slice(value.as_bytes());
    body.push(0);
    encode_message(b'S', &body)
}

/// The real cancellation key of a backend (tag `K`). It is kept private and never sent to a client.
#[derive(Clone, PartialEq, Eq)]
pub struct BackendKeyData {
    /// The process ID of the backend.
    pub process_id: i32,
    /// The secret key: four bytes in protocol 3.0, up to 256 in 3.2.
    pub secret_key: Vec<u8>,
}

impl std::fmt::Debug for BackendKeyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BackendKeyData([REDACTED])")
    }
}

/// Parses the body of `BackendKeyData`: a process ID and a key of 4 to 256 bytes.
pub fn parse_backend_key_data(body: &[u8]) -> Result<BackendKeyData, Malformed> {
    let process_id = be_i32(body).ok_or(Malformed)?;
    let key = &body[4..];
    if !(4..=256).contains(&key.len()) {
        return Err(Malformed);
    }
    Ok(BackendKeyData {
        process_id,
        secret_key: key.to_vec(),
    })
}

/// `BackendKeyData`, for scripted nodes in tests.
pub fn encode_backend_key_data(process_id: i32, secret_key: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(4 + secret_key.len());
    body.extend_from_slice(&process_id.to_be_bytes());
    body.extend_from_slice(secret_key);
    encode_message(b'K', &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body(message: &[u8]) -> &[u8] {
        &message[5..]
    }

    #[test]
    fn authentication_round_trips() {
        assert_eq!(
            parse_authentication(body(&encode_authentication_ok())),
            Ok(Authentication::Ok)
        );
        assert_eq!(
            parse_authentication(body(&encode_authentication_sasl(&[
                SCRAM_SHA_256_PLUS,
                SCRAM_SHA_256
            ]))),
            Ok(Authentication::Sasl(vec![
                SCRAM_SHA_256_PLUS.into(),
                SCRAM_SHA_256.into()
            ]))
        );
        assert_eq!(
            parse_authentication(body(&encode_authentication_sasl_continue(b"r=x"))),
            Ok(Authentication::SaslContinue(b"r=x".to_vec()))
        );
        assert_eq!(
            parse_authentication(body(&encode_authentication_sasl_final(b"v=x"))),
            Ok(Authentication::SaslFinal(b"v=x".to_vec()))
        );
        assert_eq!(
            parse_authentication(&[0, 0, 0, 3]),
            Ok(Authentication::Other(3))
        );
        assert_eq!(
            parse_authentication(&[0, 0, 0, 5, 1, 2, 3, 4]),
            Ok(Authentication::Other(5))
        );
        for bad in [
            &b""[..],
            b"\0\0\0",
            b"\0\0\0\0x",
            b"\0\0\0\x0a",
            b"\0\0\0\x0a\0",
            b"\0\0\0\x0aA\0",
            b"\0\0\0\x0aA\0\0x",
        ] {
            assert_eq!(parse_authentication(bad), Err(Malformed), "{bad:?}");
        }
    }

    #[test]
    fn sasl_initial_response_length_must_match() {
        let msg = encode_sasl_initial_response(SCRAM_SHA_256, b"n,,n=,r=abc");
        assert_eq!(
            parse_sasl_initial_response(body(&msg)),
            Ok(SaslInitialResponse {
                mechanism: SCRAM_SHA_256.into(),
                data: Some(b"n,,n=,r=abc".to_vec())
            })
        );
        assert_eq!(
            parse_sasl_initial_response(b"SCRAM-SHA-256\0\xff\xff\xff\xff"),
            Ok(SaslInitialResponse {
                mechanism: SCRAM_SHA_256.into(),
                data: None
            })
        );
        for bad in [
            &b"SCRAM-SHA-256\0\x7f\xff\xff\xffabc"[..],
            b"SCRAM-SHA-256\0\0\0\0\x02abc",
            b"SCRAM-SHA-256\0\0\0\0\x04abc",
            b"SCRAM-SHA-256\0\xff\xff\xff\xfex",
            b"SCRAM-SHA-256\0\xff\xff\xff\xffx",
            b"SCRAM-SHA-256\0\0\0",
            b"SCRAM-SHA-256",
        ] {
            assert_eq!(parse_sasl_initial_response(bad), Err(Malformed), "{bad:?}");
        }
    }

    #[test]
    fn status_and_key_messages() {
        for status in [
            TransactionStatus::Idle,
            TransactionStatus::InTransaction,
            TransactionStatus::Failed,
        ] {
            assert_eq!(
                parse_ready_for_query(body(&encode_ready_for_query(status))),
                Ok(status)
            );
        }
        assert_eq!(parse_ready_for_query(b"X"), Err(Malformed));
        assert_eq!(parse_ready_for_query(b"II"), Err(Malformed));
        assert_eq!(
            parse_parameter_status(body(&encode_parameter_status("in_hot_standby", "off"))),
            Ok(ParameterStatus {
                name: b"in_hot_standby".to_vec(),
                value: b"off".to_vec()
            })
        );
        for bad in [&b"a\0b"[..], b"a\0b\0c", b"\0b\0", b"a"] {
            assert_eq!(parse_parameter_status(bad), Err(Malformed));
        }
        let key = parse_backend_key_data(body(&encode_backend_key_data(42, &[9, 8, 7, 6])))
            .expect("valid");
        assert_eq!(
            (key.process_id, key.secret_key.as_slice()),
            (42, &[9, 8, 7, 6][..])
        );
        assert_eq!(format!("{key:?}"), "BackendKeyData([REDACTED])");
        assert_eq!(
            parse_backend_key_data(&[0, 0, 0, 1, 2, 3, 4]),
            Err(Malformed)
        );
        assert_eq!(parse_backend_key_data(&[0; 4 + 257]), Err(Malformed));
        assert!(parse_backend_key_data(&[0; 4 + 256]).is_ok());
    }

    #[test]
    fn negotiation_round_trips_and_checks_its_count() {
        let names = vec![b"_pq_.a".to_vec(), b"_pq_.b".to_vec()];
        let msg = encode_negotiate_protocol_version(0, &names);
        assert_eq!(
            parse_negotiate_protocol_version(body(&msg)),
            Ok(NegotiateProtocolVersion {
                minor: 0,
                unsupported: names
            })
        );
        assert_eq!(
            parse_negotiate_protocol_version(b"\0\0\0\0\xff\xff\xff\xff"),
            Err(Malformed)
        );
        assert_eq!(
            parse_negotiate_protocol_version(b"\0\0\0\0\0\0\0\x01a"),
            Err(Malformed)
        );
    }
}
