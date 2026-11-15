//! The first packets of a connection, which carry no tag (spec 0004, "Startup").
//!
//! [`FirstPacketReader`] reads the first packets of one connection: it
//! decodes the eight header bytes (a length and a code) with
//! [`decode_first_header`], asks the [`Handshake`] of the connection what to
//! do with that packet, and only then reads a body, whose length the header
//! has already checked. The two functions can also be called on their own.

use crate::sqlstate::PoolError;
use crate::{be_u32, take_cstr};

/// The code of `SSLRequest`.
pub const SSL_REQUEST_CODE: u32 = 80_877_103;
/// The code of `GSSENCRequest`.
pub const GSSENC_REQUEST_CODE: u32 = 80_877_104;
/// The code of `CancelRequest`.
pub const CANCEL_REQUEST_CODE: u32 = 80_877_102;
/// The major version of the protocol that POOL speaks.
pub const PROTOCOL_MAJOR: u16 = 3;
/// The minor version of the protocol that POOL speaks.
pub const PROTOCOL_MINOR: u16 = 0;
/// The size of the length and code of a first packet.
pub const FIRST_HEADER_LEN: usize = 8;

/// The kind of a first packet, read from its code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstKind {
    /// `SSLRequest`.
    SslRequest,
    /// `GSSENCRequest`.
    GssEncRequest,
    /// `CancelRequest`.
    CancelRequest,
    /// `StartupMessage` for protocol 3 with this minor version.
    Startup {
        /// The minor version the client asked for.
        minor: u16,
    },
}

/// A checked first packet header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FirstHeader {
    /// What the packet is.
    pub kind: FirstKind,
    /// The bytes of the body after the eight header bytes.
    pub body_len: u32,
}

/// Decodes the eight header bytes of a first packet.
///
/// `max_len` is `limits.startup_packet_max_bytes`, compared with the length
/// field. A length below 8 is `08P01` and a length above the limit is
/// `54000`, before anything else is looked at. `SSLRequest` and
/// `GSSENCRequest` must have length 8. A major version other than 3 is
/// `0A000`, and an unknown code of the reserved major version 1234 is
/// `08P01`.
pub fn decode_first_header(
    bytes: [u8; FIRST_HEADER_LEN],
    max_len: u32,
) -> Result<FirstHeader, PoolError> {
    let len = be_u32(&bytes[..4]).ok_or(PoolError::ProtocolViolation)?;
    let code = be_u32(&bytes[4..]).ok_or(PoolError::ProtocolViolation)?;
    if len < 8 {
        return Err(PoolError::ProtocolViolation);
    }
    if len > max_len {
        return Err(PoolError::MessageTooLarge);
    }
    let body_len = len - 8;
    let major = (code >> 16) as u16;
    let minor = (code & 0xffff) as u16;
    let kind = match code {
        SSL_REQUEST_CODE | GSSENC_REQUEST_CODE if body_len != 0 => {
            return Err(PoolError::ProtocolViolation);
        }
        SSL_REQUEST_CODE => FirstKind::SslRequest,
        GSSENC_REQUEST_CODE => FirstKind::GssEncRequest,
        CANCEL_REQUEST_CODE => FirstKind::CancelRequest,
        _ if major == 1234 => return Err(PoolError::ProtocolViolation),
        _ if major == PROTOCOL_MAJOR => FirstKind::Startup { minor },
        _ => return Err(PoolError::FeatureNotSupported),
    };
    Ok(FirstHeader { kind, body_len })
}

/// What the caller does with a first packet that [`Handshake::next`] accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeStep {
    /// Answer `S`, run the TLS handshake, then read a new first packet.
    AnswerSsl,
    /// Answer `N`, then read a new first packet.
    AnswerGssEnc,
    /// Read the rest of the packet, do nothing and close, with no answer.
    DropCancel,
    /// Read the body and parse it with [`parse_startup`].
    ReadStartup,
}

/// The order of first packets on one connection.
///
/// At most an optional `GSSENCRequest`, then `SSLRequest`, then, over TLS, a
/// `StartupMessage` or a `CancelRequest`. A `CancelRequest` may also be the
/// very first packet. A `StartupMessage` before TLS is `28000`. Every other
/// order is `08P01`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Handshake {
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum State {
    #[default]
    First,
    AfterGssEnc,
    Tls,
    Done,
}

impl Handshake {
    /// The state of a connection that has sent nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reports whether TLS has been agreed on this connection.
    pub fn is_tls(&self) -> bool {
        self.state == State::Tls
    }

    /// Takes the next first packet. After an error or a final step no packet is accepted.
    pub fn next(&mut self, kind: FirstKind) -> Result<HandshakeStep, PoolError> {
        let (state, step) = match (self.state, kind) {
            (State::First, FirstKind::GssEncRequest) => {
                (State::AfterGssEnc, HandshakeStep::AnswerGssEnc)
            }
            (State::First | State::AfterGssEnc, FirstKind::SslRequest) => {
                (State::Tls, HandshakeStep::AnswerSsl)
            }
            (State::First | State::Tls, FirstKind::CancelRequest) => {
                (State::Done, HandshakeStep::DropCancel)
            }
            (State::First | State::AfterGssEnc, FirstKind::Startup { .. }) => {
                self.state = State::Done;
                return Err(PoolError::TlsRequired);
            }
            (State::Tls, FirstKind::Startup { .. }) => (State::Done, HandshakeStep::ReadStartup),
            _ => {
                self.state = State::Done;
                return Err(PoolError::ProtocolViolation);
            }
        };
        self.state = state;
        Ok(step)
    }
}

/// A parameter that POOL forwards to the backend unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forwarded {
    /// `application_name` or `client_encoding`.
    pub name: &'static str,
    /// The value, as the client sent it.
    pub value: Vec<u8>,
}

/// A startup packet that POOL accepts.
#[derive(Clone, PartialEq, Eq)]
pub struct StartupRequest {
    /// The minor version the client asked for.
    pub minor: u16,
    /// The claimed principal, byte for byte.
    pub user: Vec<u8>,
    /// The requested database; the value of `user` when the client sent none.
    pub database: Vec<u8>,
    /// `application_name` and `client_encoding`, in the order the client sent them.
    pub forwarded: Vec<Forwarded>,
    /// The names that start with `_pq_.`, in the order the client sent them.
    pub protocol_options: Vec<Vec<u8>>,
}

impl std::fmt::Debug for StartupRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Names of people and databases stay out of default telemetry.
        f.debug_struct("StartupRequest")
            .field("minor", &self.minor)
            .field("forwarded", &self.forwarded.len())
            .field("protocol_options", &self.protocol_options.len())
            .finish_non_exhaustive()
    }
}

impl StartupRequest {
    /// The `NegotiateProtocolVersion` to send right after the startup packet, if one is needed.
    ///
    /// It is needed for a minor version above 0 or a `_pq_.` parameter.
    pub fn negotiation(&self) -> Option<Vec<u8>> {
        (self.minor > PROTOCOL_MINOR || !self.protocol_options.is_empty()).then(|| {
            crate::auth::encode_negotiate_protocol_version(PROTOCOL_MINOR, &self.protocol_options)
        })
    }
}

/// Parses the body of a `StartupMessage` (after the eight header bytes).
///
/// Pairs of text each ended by a zero byte, and one more zero byte at the
/// end. A name that appears twice, an empty name, bytes after the end or a
/// missing `user` are `08P01`. `replication`, `options` and every name that
/// is not listed in the spec are `0A000`. The structure is checked first,
/// then `user`, then the names.
pub fn parse_startup(minor: u16, body: &[u8]) -> Result<StartupRequest, PoolError> {
    let violation = PoolError::ProtocolViolation;
    let mut pairs: Vec<(&[u8], &[u8])> = Vec::new();
    let mut rest = body;
    loop {
        let (name, after) = take_cstr(rest).ok_or(violation)?;
        if name.is_empty() {
            if !after.is_empty() {
                return Err(violation);
            }
            break;
        }
        let (value, after) = take_cstr(after).ok_or(violation)?;
        if pairs.iter().any(|(seen, _)| *seen == name) {
            return Err(violation);
        }
        pairs.push((name, value));
        rest = after;
    }
    let user = pairs
        .iter()
        .find(|(name, _)| *name == b"user")
        .map(|(_, value)| value.to_vec())
        .ok_or(violation)?;
    let mut request = StartupRequest {
        minor,
        database: user.clone(),
        user,
        forwarded: Vec::new(),
        protocol_options: Vec::new(),
    };
    for (name, value) in pairs {
        match name {
            b"user" => {}
            b"database" => request.database = value.to_vec(),
            b"application_name" | b"client_encoding" => request.forwarded.push(Forwarded {
                name: if name == b"application_name" {
                    "application_name"
                } else {
                    "client_encoding"
                },
                value: value.to_vec(),
            }),
            _ if name.starts_with(b"_pq_.") => request.protocol_options.push(name.to_vec()),
            _ => return Err(PoolError::FeatureNotSupported),
        }
    }
    Ok(request)
}

/// What [`FirstPacketReader`] gives for one first packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FirstPacket {
    /// `SSLRequest`: answer `S`, run the TLS handshake, then feed the same reader the bytes that come over TLS.
    SslRequest,
    /// `GSSENCRequest`: answer `N`, then feed the same reader the next packet.
    GssEncRequest,
    /// `CancelRequest`, whose body was read and dropped: close with no answer.
    CancelRequest,
    /// A `StartupMessage` over TLS that [`parse_startup`] accepted.
    Startup(StartupRequest),
}

/// Reads the first packets of one connection, before and after its TLS handshake, from pieces of any size.
///
/// The reader keeps the [`Handshake`] of the connection and asks it as soon
/// as the eight header bytes are there. A refusal or a packet without a body
/// (`SSLRequest`, `GSSENCRequest`) is returned at that moment, so a
/// `StartupMessage` that came without TLS gets `28000` and its parameters
/// are never read. A body is read only for a `StartupMessage` over TLS,
/// which is reserved after the header checked its length against the limit,
/// and for a `CancelRequest`, which is counted and dropped, never stored.
/// The reader never takes a byte after the packet it is reading. After an
/// error or a final packet it accepts nothing more.
#[derive(Debug, Clone)]
pub struct FirstPacketReader {
    max_len: u32,
    handshake: Handshake,
    header: [u8; FIRST_HEADER_LEN],
    filled: usize,
    body: Option<(FirstKind, Vec<u8>, usize)>,
}

impl FirstPacketReader {
    /// A reader for a new connection, for packets of at most `max_len` bytes (`limits.startup_packet_max_bytes`).
    pub fn new(max_len: u32) -> Self {
        Self {
            max_len,
            handshake: Handshake::new(),
            header: [0; FIRST_HEADER_LEN],
            filled: 0,
            body: None,
        }
    }

    /// Reports whether the connection has agreed on TLS.
    pub fn is_tls(&self) -> bool {
        self.handshake.is_tls()
    }

    /// How many more bytes the current packet needs, as far as known now.
    pub fn wanted(&self) -> usize {
        match &self.body {
            Some((FirstKind::CancelRequest, _, left)) => *left,
            Some((_, body, len)) => len - body.len(),
            None => FIRST_HEADER_LEN - self.filled,
        }
    }

    /// Takes bytes from the start of `input`; returns how many and, once a packet is decided, the result.
    pub fn feed(&mut self, input: &[u8]) -> (usize, Option<Result<FirstPacket, PoolError>>) {
        let mut used = 0;
        if self.body.is_none() {
            let take = (FIRST_HEADER_LEN - self.filled).min(input.len());
            self.header[self.filled..self.filled + take].copy_from_slice(&input[..take]);
            self.filled += take;
            used += take;
            if self.filled < FIRST_HEADER_LEN {
                return (used, None);
            }
            self.filled = 0;
            let decided = decode_first_header(self.header, self.max_len)
                .and_then(|header| self.handshake.next(header.kind).map(|step| (header, step)));
            let (header, step) = match decided {
                Ok(decided) => decided,
                Err(error) => return (used, Some(Err(error))),
            };
            let len = header.body_len as usize;
            match step {
                HandshakeStep::AnswerSsl => return (used, Some(Ok(FirstPacket::SslRequest))),
                HandshakeStep::AnswerGssEnc => return (used, Some(Ok(FirstPacket::GssEncRequest))),
                HandshakeStep::DropCancel => self.body = Some((header.kind, Vec::new(), len)),
                HandshakeStep::ReadStartup => {
                    // The length was checked against the limit above.
                    self.body = Some((header.kind, Vec::with_capacity(len), len));
                }
            }
        }
        let rest = &input[used..];
        let Some((kind, body, len)) = self.body.as_mut() else {
            return (used, None);
        };
        if *kind == FirstKind::CancelRequest {
            let take = (*len).min(rest.len());
            *len -= take;
            used += take;
            if *len > 0 {
                return (used, None);
            }
        } else {
            let take = (*len - body.len()).min(rest.len());
            body.extend_from_slice(&rest[..take]);
            used += take;
            if body.len() < *len {
                return (used, None);
            }
        }
        let packet = match *kind {
            FirstKind::Startup { minor } => parse_startup(minor, body).map(FirstPacket::Startup),
            _ => Ok(FirstPacket::CancelRequest),
        };
        self.body = None;
        (used, Some(packet))
    }
}

/// `SSLRequest`, as POOL sends it to a node.
pub fn encode_ssl_request() -> [u8; 8] {
    let mut out = [0; 8];
    out[..4].copy_from_slice(&8u32.to_be_bytes());
    out[4..].copy_from_slice(&SSL_REQUEST_CODE.to_be_bytes());
    out
}

/// A `StartupMessage` for protocol 3.0 with the given parameters, in order.
///
/// # Panics
///
/// When the packet is longer than `u32::MAX` bytes; every caller builds small packets.
pub fn encode_startup(params: &[(&str, &[u8])]) -> Vec<u8> {
    encode_startup_version(PROTOCOL_MAJOR, PROTOCOL_MINOR, params)
}

/// A `StartupMessage` for any version, for tests and scripted clients.
///
/// # Panics
///
/// When the packet is longer than `u32::MAX` bytes.
pub fn encode_startup_version(major: u16, minor: u16, params: &[(&str, &[u8])]) -> Vec<u8> {
    let mut body = Vec::new();
    for (name, value) in params {
        body.extend_from_slice(name.as_bytes());
        body.push(0);
        body.extend_from_slice(value);
        body.push(0);
    }
    body.push(0);
    let len = u32::try_from(body.len() + 8).expect("startup packet below 4 GiB");
    let mut out = Vec::with_capacity(body.len() + 8);
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(&major.to_be_bytes());
    out.extend_from_slice(&minor.to_be_bytes());
    out.extend_from_slice(&body);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(len: u32, code: u32) -> [u8; 8] {
        let mut out = [0; 8];
        out[..4].copy_from_slice(&len.to_be_bytes());
        out[4..].copy_from_slice(&code.to_be_bytes());
        out
    }

    #[test]
    fn header_rows() {
        let ok = |len, code| decode_first_header(header(len, code), 8192);
        assert_eq!(
            ok(8, SSL_REQUEST_CODE).map(|h| h.kind),
            Ok(FirstKind::SslRequest)
        );
        assert_eq!(
            ok(8, GSSENC_REQUEST_CODE).map(|h| h.kind),
            Ok(FirstKind::GssEncRequest)
        );
        assert_eq!(
            ok(16, CANCEL_REQUEST_CODE).map(|h| (h.kind, h.body_len)),
            Ok((FirstKind::CancelRequest, 8))
        );
        assert_eq!(ok(9, SSL_REQUEST_CODE), Err(PoolError::ProtocolViolation));
        assert_eq!(
            ok(12, GSSENC_REQUEST_CODE),
            Err(PoolError::ProtocolViolation)
        );
        assert_eq!(
            ok(20, 0x0003_0000).map(|h| h.kind),
            Ok(FirstKind::Startup { minor: 0 })
        );
        assert_eq!(
            ok(20, 0x0003_0002).map(|h| h.kind),
            Ok(FirstKind::Startup { minor: 2 })
        );
        assert_eq!(ok(20, 0x0002_0000), Err(PoolError::FeatureNotSupported));
        assert_eq!(ok(20, 0x0004_0000), Err(PoolError::FeatureNotSupported));
        assert_eq!(ok(20, 0x04d2_1631), Err(PoolError::ProtocolViolation));
        for len in [0, 4, 7] {
            assert_eq!(ok(len, SSL_REQUEST_CODE), Err(PoolError::ProtocolViolation));
        }
        assert_eq!(ok(8193, 0x0003_0000), Err(PoolError::MessageTooLarge));
        assert_eq!(ok(u32::MAX, 0x0003_0000), Err(PoolError::MessageTooLarge));
        assert!(ok(8192, 0x0003_0000).is_ok());
        // Direct TLS: a handshake record as the first bytes fails the length rule.
        assert_eq!(
            decode_first_header([0x16, 0x03, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01], 8192),
            Err(PoolError::MessageTooLarge)
        );
    }

    #[test]
    fn handshake_order() {
        use FirstKind::*;
        use HandshakeStep::*;
        let run = |kinds: &[FirstKind]| {
            let mut h = Handshake::new();
            kinds.iter().map(|k| h.next(*k)).collect::<Vec<_>>()
        };
        let startup = Startup { minor: 0 };
        assert_eq!(
            run(&[SslRequest, startup]),
            [Ok(AnswerSsl), Ok(ReadStartup)]
        );
        assert_eq!(
            run(&[GssEncRequest, SslRequest, startup]),
            [Ok(AnswerGssEnc), Ok(AnswerSsl), Ok(ReadStartup)]
        );
        assert_eq!(run(&[CancelRequest]), [Ok(DropCancel)]);
        assert_eq!(
            run(&[SslRequest, CancelRequest]),
            [Ok(AnswerSsl), Ok(DropCancel)]
        );
        assert_eq!(run(&[startup]), [Err(PoolError::TlsRequired)]);
        assert_eq!(
            run(&[GssEncRequest, startup]),
            [Ok(AnswerGssEnc), Err(PoolError::TlsRequired)]
        );
        let violation = Err(PoolError::ProtocolViolation);
        assert_eq!(
            run(&[GssEncRequest, GssEncRequest]),
            [Ok(AnswerGssEnc), violation]
        );
        assert_eq!(
            run(&[GssEncRequest, CancelRequest]),
            [Ok(AnswerGssEnc), violation]
        );
        assert_eq!(run(&[SslRequest, SslRequest]), [Ok(AnswerSsl), violation]);
        assert_eq!(
            run(&[SslRequest, GssEncRequest]),
            [Ok(AnswerSsl), violation]
        );
        assert_eq!(
            run(&[SslRequest, startup, startup]),
            [Ok(AnswerSsl), Ok(ReadStartup), violation]
        );
        assert_eq!(
            run(&[CancelRequest, SslRequest]),
            [Ok(DropCancel), violation]
        );
        assert_eq!(
            run(&[startup, SslRequest]),
            [Err(PoolError::TlsRequired), violation]
        );
    }

    fn body(pairs: &[(&str, &str)]) -> Vec<u8> {
        let p: Vec<(&str, &[u8])> = pairs.iter().map(|(n, v)| (*n, v.as_bytes())).collect();
        encode_startup(&p)[8..].to_vec()
    }

    #[test]
    fn parameter_rows() {
        let r = parse_startup(0, &body(&[("user", "alice")])).expect("valid");
        assert_eq!(
            (r.user.as_slice(), r.database.as_slice()),
            (&b"alice"[..], &b"alice"[..])
        );
        let r = parse_startup(
            0,
            &body(&[
                ("application_name", "psql"),
                ("database", "app"),
                ("user", "alice"),
                ("client_encoding", "UTF8"),
                ("_pq_.x", "1"),
            ]),
        )
        .expect("valid");
        assert_eq!(r.database, b"app");
        assert_eq!(
            r.forwarded,
            [
                Forwarded {
                    name: "application_name",
                    value: b"psql".to_vec()
                },
                Forwarded {
                    name: "client_encoding",
                    value: b"UTF8".to_vec()
                }
            ]
        );
        assert_eq!(r.protocol_options, [b"_pq_.x".to_vec()]);
        for refused in [
            ("replication", "true"),
            ("options", "-c x=1"),
            ("search_path", "a"),
            ("User", "x"),
        ] {
            assert_eq!(
                parse_startup(0, &body(&[("user", "alice"), refused])),
                Err(PoolError::FeatureNotSupported),
                "{refused:?}"
            );
        }
        let violation = Err(PoolError::ProtocolViolation);
        assert_eq!(parse_startup(0, &body(&[("database", "app")])), violation);
        assert_eq!(
            parse_startup(0, &body(&[("user", "a"), ("user", "b")])),
            violation
        );
        assert_eq!(parse_startup(0, b"\0"), violation);
        assert_eq!(parse_startup(0, b""), violation);
        assert_eq!(parse_startup(0, b"user\0alice\0"), violation);
        assert_eq!(parse_startup(0, b"user\0alice"), violation);
        assert_eq!(parse_startup(0, b"user\0alice\0\0x"), violation);
        assert_eq!(parse_startup(0, b"\0x\0user\0alice\0\0"), violation);
        // The structure is checked before the names: a refused name does not hide a later violation.
        assert_eq!(
            parse_startup(0, b"options\0x\0user\0a\0user\0b\0\0"),
            violation
        );
        // Values are bytes; the user name is kept byte for byte.
        let r = parse_startup(0, b"user\0\xff\xfe\0\0").expect("valid");
        assert_eq!(r.user, b"\xff\xfe");
        assert!(!format!("{r:?}").contains("\\xff"));
    }

    #[test]
    fn negotiation_only_when_needed() {
        let plain = parse_startup(0, &body(&[("user", "alice")])).expect("valid");
        assert_eq!(plain.negotiation(), None);
        let newer = parse_startup(2, &body(&[("user", "alice")])).expect("valid");
        assert_eq!(
            newer.negotiation(),
            Some(b"v\0\0\0\x0c\0\0\0\0\0\0\0\0".to_vec())
        );
        let option = parse_startup(0, &body(&[("user", "alice"), ("_pq_.a", "1")])).expect("valid");
        assert_eq!(
            option.negotiation(),
            Some(b"v\0\0\0\x13\0\0\0\0\0\0\0\x01_pq_.a\0".to_vec())
        );
    }

    #[test]
    fn a_cancel_body_is_dropped_and_nothing_after_the_packet_is_taken() {
        let mut packet = header(16, CANCEL_REQUEST_CODE).to_vec();
        packet.extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 0xAA]);
        let mut reader = FirstPacketReader::new(8192);
        assert_eq!(
            reader.feed(&packet),
            (16, Some(Ok(FirstPacket::CancelRequest)))
        );
        let mut reader = FirstPacketReader::new(8192);
        let mut ssl = encode_ssl_request().to_vec();
        ssl.push(0x16);
        assert_eq!(reader.feed(&ssl), (8, Some(Ok(FirstPacket::SslRequest))));
    }

    #[test]
    fn one_reader_follows_the_connection_and_never_reads_a_body_before_tls() {
        let startup = encode_startup(&[("user", b"alice")]);
        // Without TLS: 28000 after the eight header bytes, the parameters unread.
        let mut reader = FirstPacketReader::new(8192);
        assert_eq!(
            reader.feed(&startup),
            (8, Some(Err(PoolError::TlsRequired)))
        );
        // GSSENCRequest, SSLRequest, then the startup packet over TLS, on one reader.
        let mut reader = FirstPacketReader::new(8192);
        let gss = header(8, GSSENC_REQUEST_CODE);
        assert_eq!(reader.feed(&gss), (8, Some(Ok(FirstPacket::GssEncRequest))));
        assert!(!reader.is_tls());
        assert_eq!(
            reader.feed(&encode_ssl_request()),
            (8, Some(Ok(FirstPacket::SslRequest)))
        );
        assert!(reader.is_tls());
        let (used, packet) = reader.feed(&startup);
        assert_eq!(used, startup.len());
        assert!(matches!(packet, Some(Ok(FirstPacket::Startup(_)))));
        // Nothing after a final packet.
        assert_eq!(
            reader.feed(&startup).1,
            Some(Err(PoolError::ProtocolViolation))
        );
        // A second SSLRequest and a GSSENCRequest after TLS are refused at their header.
        for second in [encode_ssl_request(), header(8, GSSENC_REQUEST_CODE)] {
            let mut reader = FirstPacketReader::new(8192);
            reader.feed(&encode_ssl_request());
            assert_eq!(
                reader.feed(&second),
                (8, Some(Err(PoolError::ProtocolViolation)))
            );
        }
        // A CancelRequest after TLS is read and dropped; after GSSENCRequest it is refused.
        let mut cancel = header(16, CANCEL_REQUEST_CODE).to_vec();
        cancel.extend_from_slice(&[0; 8]);
        let mut reader = FirstPacketReader::new(8192);
        reader.feed(&encode_ssl_request());
        assert_eq!(
            reader.feed(&cancel),
            (16, Some(Ok(FirstPacket::CancelRequest)))
        );
        let mut reader = FirstPacketReader::new(8192);
        reader.feed(&gss);
        assert_eq!(
            reader.feed(&cancel),
            (8, Some(Err(PoolError::ProtocolViolation)))
        );
    }
}
