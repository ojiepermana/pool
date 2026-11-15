//! Golden packets and messages of spec 0004, "The wire subset", whole and
//! split in two pieces at every byte position (`0004 AC-28`), and the bytes
//! of every error that POOL generates.

use pool_protocol::auth::{self, Authentication, TransactionStatus};
use pool_protocol::message::{FrameError, Message, MessageReader};
use pool_protocol::sqlstate::{PoolError, error_sqlstate};
use pool_protocol::startup::{
    CANCEL_REQUEST_CODE, FirstPacket, FirstPacketReader, GSSENC_REQUEST_CODE, SSL_REQUEST_CODE,
    encode_ssl_request, encode_startup, encode_startup_version,
};
use pool_protocol::{message, scram};

const STARTUP_MAX: u32 = 8192;

/// Feeds `pieces` in order and collects every result; checks that the reader took every byte.
fn first_packets(pieces: &[&[u8]]) -> Vec<Result<FirstPacket, PoolError>> {
    let mut reader = FirstPacketReader::new(STARTUP_MAX);
    let mut out = Vec::new();
    for piece in pieces {
        let mut rest: &[u8] = piece;
        while !rest.is_empty() {
            let (used, result) = reader.feed(rest);
            rest = &rest[used..];
            if let Some(result) = result {
                let stop = result.is_err();
                out.push(result);
                if stop {
                    return out;
                }
            } else {
                assert!(
                    rest.is_empty(),
                    "a reader without a result takes all it is given"
                );
            }
        }
    }
    out
}

fn messages(limit: u32, pieces: &[&[u8]]) -> Vec<Result<Message, FrameError>> {
    let mut reader = MessageReader::new(limit);
    let mut out = Vec::new();
    for piece in pieces {
        let mut rest: &[u8] = piece;
        while !rest.is_empty() {
            let (used, result) = reader.feed(rest);
            rest = &rest[used..];
            if let Some(result) = result {
                let stop = result.is_err();
                out.push(result);
                if stop {
                    return out;
                }
            }
        }
    }
    out
}

fn header(len: u32, code: u32) -> Vec<u8> {
    let mut out = len.to_be_bytes().to_vec();
    out.extend_from_slice(&code.to_be_bytes());
    out
}

/// The golden first packets: each as bytes, with the one result it must give.
fn golden_first_packets() -> Vec<(&'static str, Vec<u8>)> {
    let mut cancel = header(16, CANCEL_REQUEST_CODE);
    cancel.extend_from_slice(&[0, 0, 0x30, 0x39, 0xde, 0xad, 0xbe, 0xef]);
    let mut huge = header(u32::MAX, 0x0003_0000);
    huge.extend_from_slice(b"user\0");
    vec![
        ("ssl", encode_ssl_request().to_vec()),
        ("gssenc", header(8, GSSENC_REQUEST_CODE)),
        ("cancel", cancel),
        (
            "psql",
            encode_startup(&[
                ("user", b"alice"),
                ("database", b"app"),
                ("application_name", b"psql"),
                ("client_encoding", b"UTF8"),
            ]),
        ),
        ("user only", encode_startup(&[("user", b"alice")])),
        (
            "protocol 3.2",
            encode_startup_version(3, 2, &[("user", b"alice"), ("_pq_.test_option", b"1")]),
        ),
        (
            "replication",
            encode_startup(&[("user", b"alice"), ("replication", b"database")]),
        ),
        (
            "options",
            encode_startup(&[("user", b"alice"), ("options", b"-c search_path=x")]),
        ),
        (
            "major 2",
            encode_startup_version(2, 0, &[("user", b"alice")]),
        ),
        (
            "major 4",
            encode_startup_version(4, 0, &[("user", b"alice")]),
        ),
        ("no user", encode_startup(&[("database", b"app")])),
        ("twice", encode_startup(&[("user", b"a"), ("user", b"b")])),
        ("short", header(7, SSL_REQUEST_CODE)),
        ("too large", huge),
        (
            "direct TLS",
            vec![0x16, 0x03, 0x01, 0x02, 0x00, 0x01, 0x00, 0x01, 0xfc, 0x03],
        ),
    ]
}

#[test]
fn first_packets_give_their_rows() {
    let results: Vec<_> = golden_first_packets()
        .into_iter()
        .map(|(name, bytes)| (name, first_packets(&[&bytes])))
        .collect();
    let one = |name: &str| {
        let (_, r) = results
            .iter()
            .find(|(n, _)| *n == name)
            .expect("golden name");
        assert_eq!(r.len(), 1, "{name}: {r:?}");
        r[0].clone()
    };
    assert_eq!(one("ssl"), Ok(FirstPacket::SslRequest));
    assert_eq!(one("gssenc"), Ok(FirstPacket::GssEncRequest));
    assert_eq!(one("cancel"), Ok(FirstPacket::CancelRequest));
    let Ok(FirstPacket::Startup(psql)) = one("psql") else {
        panic!("psql is a startup packet");
    };
    assert_eq!(
        (psql.user.as_slice(), psql.database.as_slice()),
        (&b"alice"[..], &b"app"[..])
    );
    assert_eq!(psql.forwarded.len(), 2);
    assert_eq!(psql.negotiation(), None);
    let Ok(FirstPacket::Startup(newer)) = one("protocol 3.2") else {
        panic!("protocol 3.2 is a startup packet");
    };
    let npv = newer.negotiation().expect("negotiation");
    let parsed = auth::parse_negotiate_protocol_version(&npv[5..]).expect("valid");
    assert_eq!(
        (parsed.minor, parsed.unsupported),
        (0, vec![b"_pq_.test_option".to_vec()])
    );
    assert_eq!(one("replication"), Err(PoolError::FeatureNotSupported));
    assert_eq!(one("options"), Err(PoolError::FeatureNotSupported));
    assert_eq!(one("major 2"), Err(PoolError::FeatureNotSupported));
    assert_eq!(one("major 4"), Err(PoolError::FeatureNotSupported));
    assert_eq!(one("no user"), Err(PoolError::ProtocolViolation));
    assert_eq!(one("twice"), Err(PoolError::ProtocolViolation));
    assert_eq!(one("short"), Err(PoolError::ProtocolViolation));
    assert_eq!(one("too large"), Err(PoolError::MessageTooLarge));
    assert_eq!(one("direct TLS"), Err(PoolError::MessageTooLarge));
}

#[test]
fn first_packets_split_at_every_byte_give_the_same_result() {
    for (name, bytes) in golden_first_packets() {
        let whole = first_packets(&[&bytes]);
        for at in 0..=bytes.len() {
            let (a, b) = bytes.split_at(at);
            assert_eq!(first_packets(&[a, b]), whole, "{name} split at {at}");
        }
        // One byte at a time, the hardest split of all.
        let singles: Vec<&[u8]> = bytes.chunks(1).collect();
        assert_eq!(first_packets(&singles), whole, "{name} byte by byte");
    }
}

#[test]
fn a_first_packet_reader_never_takes_a_byte_after_its_packet() {
    for (name, bytes) in golden_first_packets() {
        let mut reader = FirstPacketReader::new(STARTUP_MAX);
        let mut input = bytes.clone();
        input.extend_from_slice(b"\x16\x03\x01NEXT");
        let mut used = 0;
        loop {
            assert!(reader.wanted() > 0, "{name}");
            let (n, result) = reader.feed(&input[used..]);
            used += n;
            if result.is_some() {
                break;
            }
        }
        assert!(
            used <= bytes.len(),
            "{name}: took {used} of {}",
            bytes.len()
        );
    }
}

fn backend_error(fields: &[(u8, &str)]) -> Vec<u8> {
    let mut body = Vec::new();
    for (field, value) in fields {
        body.push(*field);
        body.extend_from_slice(value.as_bytes());
        body.push(0);
    }
    body.push(0);
    message::encode_message(b'E', &body)
}

/// The golden messages after the first packets, from both sides.
fn golden_messages() -> Vec<(&'static str, Vec<u8>)> {
    let mut out = vec![
        ("query", message::encode_query("insert into t values (1)")),
        ("empty query", message::encode_query("")),
        ("terminate", message::encode_terminate()),
        ("auth ok", auth::encode_authentication_ok()),
        (
            "auth sasl",
            auth::encode_authentication_sasl(&[auth::SCRAM_SHA_256]),
        ),
        (
            "auth sasl plus",
            auth::encode_authentication_sasl(&[auth::SCRAM_SHA_256_PLUS, auth::SCRAM_SHA_256]),
        ),
        (
            "auth continue",
            auth::encode_authentication_sasl_continue(
                scram::encode_server_first("abcdefghijklmnop", b"0123456789abcdef", 4096)
                    .as_bytes(),
            ),
        ),
        (
            "auth final",
            auth::encode_authentication_sasl_final(scram::encode_server_final(&[5; 32]).as_bytes()),
        ),
        (
            "auth md5",
            auth::encode_authentication_other(5, &[1, 2, 3, 4]),
        ),
        (
            "sasl initial",
            auth::encode_sasl_initial_response(
                auth::SCRAM_SHA_256,
                scram::encode_client_first("abcdefgh").as_bytes(),
            ),
        ),
        (
            "sasl response",
            auth::encode_sasl_response(
                scram::encode_client_final(
                    &scram::client_final_without_proof("abcdefghijklmnop"),
                    &[9; 32],
                )
                .as_bytes(),
            ),
        ),
        (
            "negotiate",
            auth::encode_negotiate_protocol_version(0, &[b"_pq_.a".to_vec()]),
        ),
        (
            "ready idle",
            auth::encode_ready_for_query(TransactionStatus::Idle),
        ),
        (
            "ready in transaction",
            auth::encode_ready_for_query(TransactionStatus::InTransaction),
        ),
        (
            "parameter",
            auth::encode_parameter_status("in_hot_standby", "off"),
        ),
        ("key", auth::encode_backend_key_data(4242, &[1, 2, 3, 4])),
        (
            "backend error",
            backend_error(&[
                (b'S', "ERROR"),
                (b'V', "ERROR"),
                (b'C', "23505"),
                (b'M', "duplicate key"),
            ]),
        ),
    ];
    for error in PoolError::ALL {
        out.push(("pool error", error.encode()));
    }
    out
}

#[test]
fn messages_split_at_every_byte_give_the_same_result() {
    for (name, bytes) in golden_messages() {
        let whole = messages(4096, &[&bytes]);
        assert_eq!(whole.len(), 1, "{name}");
        assert_eq!(whole[0].as_ref().map(|m| m.tag), Ok(bytes[0]), "{name}");
        for at in 0..=bytes.len() {
            let (a, b) = bytes.split_at(at);
            assert_eq!(messages(4096, &[a, b]), whole, "{name} split at {at}");
        }
        let singles: Vec<&[u8]> = bytes.chunks(1).collect();
        assert_eq!(messages(4096, &singles), whole, "{name} byte by byte");
    }
}

#[test]
fn a_stream_of_all_golden_messages_splits_anywhere() {
    let stream: Vec<u8> = golden_messages().into_iter().flat_map(|(_, b)| b).collect();
    let whole = messages(4096, &[&stream]);
    assert_eq!(whole.len(), golden_messages().len());
    for at in 0..=stream.len() {
        let (a, b) = stream.split_at(at);
        assert_eq!(messages(4096, &[a, b]), whole, "split at {at}");
    }
}

#[test]
fn headers_split_at_every_byte_classify_the_same() {
    for (name, bytes) in golden_messages() {
        let first: [u8; 5] = bytes[..5].try_into().expect("five bytes");
        let whole = (
            message::classify_frontend(first, 1 << 24),
            message::classify_backend(first),
        );
        for at in 0..=5 {
            let mut reader = message::HeaderReader::new();
            let (n, none) = reader.feed(&bytes[..at]);
            assert_eq!((n, none.is_none() || at == 5), (at, true), "{name}");
            let (_, header) = if at == 5 {
                (0, none)
            } else {
                reader.feed(&bytes[at..])
            };
            let header = header.expect("complete").expect("valid length");
            let mut raw = [header.tag, 0, 0, 0, 0];
            raw[1..].copy_from_slice(&header.len.to_be_bytes());
            assert_eq!(
                (
                    message::classify_frontend(raw, 1 << 24),
                    message::classify_backend(raw)
                ),
                whole,
                "{name} at {at}"
            );
        }
    }
}

#[test]
fn golden_messages_parse_to_their_values() {
    let all = golden_messages();
    let body = |name: &str| {
        let (_, bytes) = all.iter().find(|(n, _)| *n == name).expect("golden name");
        bytes[5..].to_vec()
    };
    assert_eq!(
        auth::parse_authentication(&body("auth ok")),
        Ok(Authentication::Ok)
    );
    assert_eq!(
        auth::parse_authentication(&body("auth sasl")),
        Ok(Authentication::Sasl(vec!["SCRAM-SHA-256".into()]))
    );
    assert_eq!(
        auth::parse_authentication(&body("auth md5")),
        Ok(Authentication::Other(5))
    );
    let initial = auth::parse_sasl_initial_response(&body("sasl initial")).expect("valid");
    let first = scram::parse_client_first(&initial.data.expect("data")).expect("client first");
    assert_eq!(first.nonce, "abcdefgh");
    let Ok(Authentication::SaslContinue(server_first)) =
        auth::parse_authentication(&body("auth continue"))
    else {
        panic!("continue");
    };
    let server = scram::parse_server_first(&server_first, "abcdefgh").expect("server first");
    assert_eq!(
        (server.iterations, server.salt.as_slice()),
        (4096, &b"0123456789abcdef"[..])
    );
    let fin = scram::parse_client_final(&body("sasl response"), first.header, "abcdefghijklmnop")
        .expect("client final");
    assert_eq!(fin.proof, [9; 32]);
    assert_eq!(
        auth::parse_ready_for_query(&body("ready idle")),
        Ok(TransactionStatus::Idle)
    );
    assert_eq!(error_sqlstate(&body("backend error")), Some(*b"23505"));
    assert_eq!(
        auth::parse_backend_key_data(&body("key")).map(|k| k.process_id),
        Ok(4242)
    );
}

#[test]
fn every_pool_error_has_its_exact_bytes() {
    // The first row, byte for byte.
    assert_eq!(
        PoolError::TlsRequired.encode(),
        b"E\0\0\0\x2bSFATAL\0VFATAL\0C28000\0MTLS is required\0\0".to_vec()
    );
    // Every row of the table of spec 0004, "SQLSTATEs that POOL generates".
    let table: [(PoolError, &str, &str); 12] = [
        (PoolError::TlsRequired, "28000", "TLS is required"),
        (
            PoolError::FeatureNotSupported,
            "0A000",
            "this protocol feature is not supported",
        ),
        (PoolError::ProtocolViolation, "08P01", "protocol violation"),
        (
            PoolError::MessageTooLarge,
            "54000",
            "a message is larger than the configured limit",
        ),
        (
            PoolError::AuthenticationFailed,
            "28P01",
            "authentication failed",
        ),
        (
            PoolError::NoIdentityEntry,
            "28000",
            "no identity entry for this user and database",
        ),
        (
            PoolError::TooManyConnections,
            "53300",
            "too many connections",
        ),
        (
            PoolError::NotAccepting,
            "57P03",
            "the database is not accepting connections through this proxy",
        ),
        (
            PoolError::BackendNotEstablished,
            "08006",
            "the backend connection could not be established",
        ),
        (
            PoolError::OutcomeUnknown,
            "08007",
            "the connection was ended; the outcome of the statement in flight is unknown",
        ),
        (
            PoolError::BackendLost,
            "08006",
            "the backend connection was lost",
        ),
        (
            PoolError::EndedByProxy,
            "57P01",
            "the session was ended by the proxy",
        ),
    ];
    assert_eq!(table.map(|(e, _, _)| e), PoolError::ALL);
    for (error, code, text) in table {
        let body = format!("SFATAL\0VFATAL\0C{code}\0M{text}\0\0");
        let mut expected = vec![b'E'];
        expected.extend_from_slice(&u32::try_from(body.len() + 4).expect("small").to_be_bytes());
        expected.extend_from_slice(body.as_bytes());
        assert_eq!(error.encode(), expected, "{error:?}");
        assert_eq!(
            error_sqlstate(&expected[5..]),
            Some(code.as_bytes().try_into().expect("five"))
        );
    }
}

#[test]
fn a_length_field_is_checked_before_anything_is_reserved() {
    // A first packet that claims 4 GiB is refused after its eight header bytes.
    let mut reader = FirstPacketReader::new(STARTUP_MAX);
    assert_eq!(
        reader.feed(&header(u32::MAX, 0x0003_0000)),
        (8, Some(Err(PoolError::MessageTooLarge)))
    );
    // A cancel request at the limit is counted, never stored.
    let mut reader = FirstPacketReader::new(STARTUP_MAX);
    let (used, result) = reader.feed(&header(STARTUP_MAX, CANCEL_REQUEST_CODE));
    assert_eq!((used, result), (8, None));
    assert_eq!(reader.wanted(), STARTUP_MAX as usize - 8);
    // A message that claims 4 GiB is refused after its five header bytes.
    for tag in *b"pRES" {
        let mut bytes = vec![tag];
        bytes.extend_from_slice(&u32::MAX.to_be_bytes());
        assert_eq!(
            messages(auth::AUTH_MESSAGE_MAX_BYTES, &[&bytes]),
            [Err(FrameError::TooLarge)]
        );
    }
    // The authentication limit is 1024 bytes of body.
    let at_limit = message::encode_message(b'p', &[b'a'; 1024]);
    assert!(messages(auth::AUTH_MESSAGE_MAX_BYTES, &[&at_limit])[0].is_ok());
    let over = message::encode_message(b'p', &[b'a'; 1025]);
    assert_eq!(
        messages(auth::AUTH_MESSAGE_MAX_BYTES, &[&over]),
        [Err(FrameError::TooLarge)]
    );
    // Counts inside a body are compared, never reserved from.
    assert!(auth::parse_negotiate_protocol_version(b"\0\0\0\0\x7f\xff\xff\xff").is_err());
    assert!(auth::parse_sasl_initial_response(b"SCRAM-SHA-256\0\x7f\xff\xff\xff").is_err());
}
