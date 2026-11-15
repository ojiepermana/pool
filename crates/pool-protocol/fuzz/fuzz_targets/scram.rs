#![no_main]
//! The four SCRAM message parsers and the base64 decoder (spec 0004, "Fuzzing").
//!
//! Nothing may panic. What base64 decodes, it encodes back to the same text,
//! and what it encodes, it decodes back to the same bytes.

use libfuzzer_sys::fuzz_target;
use pool_protocol::base64;
use pool_protocol::scram::{
    Gs2Header, parse_client_final, parse_client_first, parse_server_final, parse_server_first,
};

fuzz_target!(|data: &[u8]| {
    if let Ok(first) = parse_client_first(data) {
        assert!(!first.nonce.is_empty());
        // A client-final message whose nonce is that of the client.
        let _ = parse_client_final(data, first.header, &first.nonce);
    }
    let nonce = String::from_utf8_lossy(data.get(..8).unwrap_or(data)).into_owned();
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
