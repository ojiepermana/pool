#![no_main]
//! The parsers of `auth.rs`, with `parse_sasl_initial_response` first, and
//! `error_sqlstate` (spec 0004, "Fuzzing", decision 94).
//!
//! `parse_sasl_initial_response` reads bytes of a client before any proof.
//! Every parser gets the whole input. Nothing may panic, and what a parser
//! accepts, its encoder writes back to the same bytes.

use libfuzzer_sys::fuzz_target;
use pool_protocol::auth::{self, Authentication};
use pool_protocol::sqlstate::error_sqlstate;

fuzz_target!(|data: &[u8]| {
    if let Ok(initial) = auth::parse_sasl_initial_response(data) {
        if let Some(client_first) = &initial.data {
            let encoded = auth::encode_sasl_initial_response(&initial.mechanism, client_first);
            assert_eq!(&encoded[5..], data);
        }
    }
    match auth::parse_authentication(data) {
        Ok(Authentication::Ok) => assert_eq!(data, [0, 0, 0, 0]),
        Ok(Authentication::Sasl(mechanisms)) => {
            let names: Vec<&str> = mechanisms.iter().map(String::as_str).collect();
            assert_eq!(&auth::encode_authentication_sasl(&names)[5..], data);
        }
        Ok(Authentication::SaslContinue(rest)) => {
            assert_eq!(&auth::encode_authentication_sasl_continue(&rest)[5..], data);
        }
        Ok(Authentication::SaslFinal(rest)) => {
            assert_eq!(&auth::encode_authentication_sasl_final(&rest)[5..], data);
        }
        Ok(Authentication::Other(_)) | Err(_) => {}
    }
    if let Ok(negotiation) = auth::parse_negotiate_protocol_version(data) {
        let encoded =
            auth::encode_negotiate_protocol_version(negotiation.minor, &negotiation.unsupported);
        assert_eq!(&encoded[5..], data);
    }
    if let Ok(status) = auth::parse_ready_for_query(data) {
        assert_eq!(&auth::encode_ready_for_query(status)[5..], data);
    }
    if let Ok(parameter) = auth::parse_parameter_status(data) {
        if let (Ok(name), Ok(value)) = (
            std::str::from_utf8(&parameter.name),
            std::str::from_utf8(&parameter.value),
        ) {
            assert_eq!(&auth::encode_parameter_status(name, value)[5..], data);
        }
    }
    if let Ok(key) = auth::parse_backend_key_data(data) {
        assert_eq!(
            &auth::encode_backend_key_data(key.process_id, &key.secret_key)[5..],
            data
        );
    }
    if let Some(code) = error_sqlstate(data) {
        assert!(code.iter().all(u8::is_ascii));
    }
});
