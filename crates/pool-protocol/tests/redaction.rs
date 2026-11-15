//! `Debug` of every type that holds content prints no content (`0004 AC-24`,
//! spec 0004 "Log events": a type that holds a SCRAM message, a nonce, a
//! proof, a name or parameter value from a client, SQL or error text from
//! PostgreSQL writes `Debug` by hand).

use pool_protocol::auth::{self, Authentication};
use pool_protocol::message::{Message, MessageReader, encode_message, encode_query};
use pool_protocol::scram::{self, Gs2Header, ServerFinal};
use pool_protocol::startup::{FirstPacket, FirstPacketReader, encode_ssl_request, encode_startup};

const CANARY: &str = "CANARY-9f2c7b1e";

fn assert_hidden(what: &str, debug: String) {
    assert!(!debug.contains(CANARY), "{what} shows its content: {debug}");
    assert!(
        !debug.contains("Q0FOQVJZ"),
        "{what} shows its content in base64: {debug}"
    );
}

#[test]
fn messages_and_readers_print_no_body() {
    let sql = format!("select '{CANARY}'");
    let mut reader = MessageReader::new(1 << 16);
    let message = reader
        .feed(&encode_query(&sql))
        .1
        .expect("complete")
        .expect("valid");
    assert_hidden("Message", format!("{message:?}"));
    assert_eq!(
        format!("{message:?}"),
        format!("Message {{ tag: 'Q', body_len: {} }}", sql.len() + 1)
    );
    // A reader that holds part of a body.
    let mut partial = MessageReader::new(1 << 16);
    let bytes = encode_query(&sql);
    partial.feed(&bytes[..bytes.len() - 2]);
    assert_hidden("MessageReader", format!("{partial:?}"));
    let direct = Message {
        tag: b'E',
        body: CANARY.as_bytes().to_vec(),
    };
    assert_hidden("Message", format!("{direct:?}"));
}

#[test]
fn authentication_and_sasl_messages_print_no_scram_text() {
    for value in [
        Authentication::SaslContinue(format!("r={CANARY},s=c2FsdA==,i=4096").into_bytes()),
        Authentication::SaslFinal(format!("v={CANARY}").into_bytes()),
        Authentication::Sasl(vec![CANARY.to_string()]),
    ] {
        assert_hidden("Authentication", format!("{value:?}"));
    }
    let initial = auth::parse_sasl_initial_response(
        &auth::encode_sasl_initial_response(CANARY, format!("n,,n={CANARY},r={CANARY}").as_bytes())
            [5..],
    )
    .expect("valid");
    assert_hidden("SaslInitialResponse", format!("{initial:?}"));
    let negotiation = auth::parse_negotiate_protocol_version(
        &auth::encode_negotiate_protocol_version(0, &[format!("_pq_.{CANARY}").into_bytes()])[5..],
    )
    .expect("valid");
    assert_hidden("NegotiateProtocolVersion", format!("{negotiation:?}"));
    let parameter =
        auth::parse_parameter_status(&auth::encode_parameter_status(CANARY, CANARY)[5..])
            .expect("valid");
    assert_hidden("ParameterStatus", format!("{parameter:?}"));
    let raw = encode_message(b'S', format!("{CANARY}\0{CANARY}\0").as_bytes());
    assert_hidden(
        "ParameterStatus",
        format!("{:?}", auth::parse_parameter_status(&raw[5..])),
    );
}

#[test]
fn scram_messages_print_no_nonce_salt_proof_or_signature() {
    let client_nonce = format!("{CANARY}client");
    let first = scram::parse_client_first(format!("n,,n={CANARY},r={client_nonce}").as_bytes())
        .expect("valid");
    assert_hidden("ClientFirst", format!("{first:?}"));
    let nonce = format!("{client_nonce}{CANARY}server");
    let server_first = scram::encode_server_first(&nonce, CANARY.as_bytes(), 4096);
    let parsed = scram::parse_server_first(server_first.as_bytes(), &client_nonce).expect("valid");
    assert_hidden("ServerFirst", format!("{parsed:?}"));
    assert!(format!("{parsed:?}").contains("4096"), "the count may show");
    let fin = scram::encode_client_final(&scram::client_final_without_proof(&nonce), &[7; 32]);
    let parsed =
        scram::parse_client_final(fin.as_bytes(), Gs2Header::NoBinding, &nonce).expect("valid");
    assert_hidden("ClientFinal", format!("{parsed:?}"));
    let signature = ServerFinal::Verifier([7; 32]);
    let text = format!("{signature:?}");
    assert!(!text.contains('7'), "{text}");
}

#[test]
fn startup_values_and_a_reader_in_the_middle_of_a_body_print_no_value() {
    let startup = encode_startup(&[
        ("user", CANARY.as_bytes()),
        ("database", CANARY.as_bytes()),
        ("application_name", CANARY.as_bytes()),
    ]);
    let mut reader = FirstPacketReader::new(8192);
    reader.feed(&encode_ssl_request());
    let (_, packet) = reader.feed(&startup);
    let Some(Ok(FirstPacket::Startup(request))) = packet else {
        panic!("a startup packet");
    };
    assert_hidden(
        "FirstPacket",
        format!("{:?}", FirstPacket::Startup(request.clone())),
    );
    assert_hidden("Forwarded", format!("{:?}", request.forwarded));
    let mut partial = FirstPacketReader::new(8192);
    partial.feed(&encode_ssl_request());
    partial.feed(&startup[..startup.len() - 3]);
    assert_hidden("FirstPacketReader", format!("{partial:?}"));
}
