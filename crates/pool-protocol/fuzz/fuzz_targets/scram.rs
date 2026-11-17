#![no_main]
//! The four SCRAM message parsers and the base64 decoder (spec 0004, "Fuzzing").
//!
//! Nothing may panic. What base64 decodes, it encodes back to the same text,
//! and what it encodes, it decodes back to the same bytes.
//!
//! The first byte gives the length of a nonce, 0 to 63, which the next bytes
//! hold; the rest is the message (decision 94: the nonce comes from a part of
//! the input of its own, so the parser of the final client message can get
//! past its nonce check and decode the proof).

use libfuzzer_sys::fuzz_target;
use pool_protocol::base64;
use pool_protocol::scram::{
    Gs2Header, parse_client_final, parse_client_first, parse_server_final, parse_server_first,
};

fuzz_target!(|input: &[u8]| {
    let (nonce, data) = match input.split_first() {
        Some((&len, rest)) => rest.split_at(usize::from(len % 64).min(rest.len())),
        None => (&[][..], input),
    };
    let nonce = String::from_utf8_lossy(nonce).into_owned();
    if let Ok(first) = parse_client_first(data) {
        assert!(!first.nonce.is_empty());
        // A client-final message whose nonce is that of the client.
        let _ = parse_client_final(data, first.header, &first.nonce);
    }
    if let Ok(server) = parse_server_first(data, &nonce) {
        assert!((1..=pool_protocol::scram::MAX_ITERATIONS).contains(&server.iterations));
        assert!(server.nonce.starts_with(&nonce));
    }
    for header in [Gs2Header::NoBinding, Gs2Header::BindingNotOffered] {
        let _ = parse_client_final(data, header, &nonce);
    }
    let _ = parse_server_final(data);
    if let Ok(decoded) = base64::decode(data) {
        assert_eq!(base64::encode(&decoded).as_bytes(), data);
    }
    assert_eq!(
        base64::decode(base64::encode(data).as_bytes()).as_deref(),
        Ok(data)
    );
});
