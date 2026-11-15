//! The backend connection against a scripted node over an in memory stream.
//!
//! The node is written with the encoders of `pool-protocol` and the SCRAM
//! server of this crate, and speaks TLS with the test certificates under
//! `tests/fixtures/`. No container, no PostgreSQL.

use std::time::Duration;

use pool_protocol::auth::{self, TransactionStatus};
use pool_protocol::message::encode_message;
use pool_protocol::scram as text;
use pool_protocol::startup::{FirstPacket, FirstPacketReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio::sync::oneshot;
use tokio_rustls::TlsAcceptor;

use super::*;
use crate::scram::{ServerCredential, ServerExchange, verifier_text};
use crate::secret::{CertificateChain, PrivateKey, TrustRoots, Verifier};
use crate::tls::server_config;

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}/{name}")).expect("fixture")
}

fn node_tls(ca: &str, name: &str) -> NodeTls {
    NodeTls::new(&TrustRoots::parse(&fixture(ca)).expect("ca"), name).expect("name")
}

fn password(text: &str) -> Password {
    Password::parse(text.as_bytes()).expect("password")
}

fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(10)
}

#[derive(Clone)]
enum Auth {
    Scram(Tamper),
    Method(i32),
    OkAtOnce,
    Mechanisms(Vec<&'static str>),
}

#[derive(Clone, Copy, PartialEq)]
enum Tamper {
    None,
    ForeignNonce,
    Count(u32),
    ErrorFinal,
    WrongSignature,
}

#[derive(Clone)]
struct Script {
    answer: u8,
    mechanisms: Vec<&'static str>,
    auth: Auth,
    password: &'static str,
    params: Vec<(&'static str, &'static str)>,
    status: TransactionStatus,
    error_after_auth: bool,
    rows: Vec<Vec<Option<&'static str>>>,
}

impl Script {
    fn primary() -> Self {
        Self {
            answer: b'S',
            mechanisms: vec![auth::SCRAM_SHA_256],
            auth: Auth::Scram(Tamper::None),
            password: "backend-secret",
            params: vec![
                ("server_version", "18.7"),
                ("in_hot_standby", "off"),
                ("client_encoding", "UTF8"),
            ],
            status: TransactionStatus::Idle,
            error_after_auth: false,
            rows: vec![vec![Some("7613"), Some("primary"), None]],
        }
    }
}

/// What the node saw: the startup parameters and the client-first message.
#[derive(Debug, Default)]
struct Seen {
    startup: Vec<(String, String)>,
    mechanism: String,
    client_first: String,
}

async fn node(stream: DuplexStream, script: Script, seen: oneshot::Sender<Seen>) {
    let _ = run_node(stream, script, seen).await;
}

async fn run_node(
    mut stream: DuplexStream,
    script: Script,
    seen: oneshot::Sender<Seen>,
) -> Option<()> {
    let mut first = [0u8; 8];
    stream.read_exact(&mut first).await.ok()?;
    assert_eq!(
        first,
        pool_protocol::startup::encode_ssl_request(),
        "SSLRequest first"
    );
    stream.write_all(&[script.answer]).await.ok()?;
    if script.answer != b'S' {
        return None;
    }
    let chain = CertificateChain::parse(&fixture("node.pem")).expect("chain");
    let key = PrivateKey::parse(&fixture("node.key")).expect("key");
    let acceptor = TlsAcceptor::from(server_config(&chain, &key).expect("server config"));
    let mut tls = acceptor.accept(stream).await.ok()?;
    let mut reader = FirstPacketReader::new(65536);
    // One reader per connection: it has seen the SSLRequest read above.
    let _ = reader.feed(&first);
    let mut buf = [0u8; 1];
    let startup = loop {
        tls.read_exact(&mut buf).await.ok()?;
        if let (_, Some(result)) = reader.feed(&buf) {
            break result.expect("a valid startup packet");
        }
    };
    let FirstPacket::Startup(request) = startup else {
        panic!("a startup packet");
    };
    let mut record = Seen {
        startup: vec![
            (
                "user".into(),
                String::from_utf8(request.user.clone()).expect("utf-8"),
            ),
            (
                "database".into(),
                String::from_utf8(request.database.clone()).expect("utf-8"),
            ),
        ],
        ..Seen::default()
    };
    for f in &request.forwarded {
        record.startup.push((
            f.name.into(),
            String::from_utf8(f.value.clone()).expect("utf-8"),
        ));
    }
    match script.auth.clone() {
        Auth::Method(code) => {
            tls.write_all(&auth::encode_authentication_other(code, &[1, 2, 3, 4]))
                .await
                .ok()?;
            let _ = seen.send(record);
            return None;
        }
        Auth::OkAtOnce => {
            tls.write_all(&auth::encode_authentication_ok())
                .await
                .ok()?;
            let _ = seen.send(record);
            return None;
        }
        Auth::Mechanisms(list) => {
            tls.write_all(&auth::encode_authentication_sasl(&list))
                .await
                .ok()?;
            let _ = seen.send(record);
            return None;
        }
        Auth::Scram(tamper) => {
            tls.write_all(&auth::encode_authentication_sasl(&script.mechanisms))
                .await
                .ok()?;
            let message = read_message(&mut tls, 1024).await.ok()?;
            assert_eq!(message.tag, b'p');
            let initial =
                auth::parse_sasl_initial_response(&message.body).expect("initial response");
            let data = initial.data.expect("data");
            record.mechanism = initial.mechanism.clone();
            record.client_first = String::from_utf8(data.clone()).expect("utf-8");
            let _ = seen.send(record);
            let verifier = Verifier::parse(
                verifier_text(&password(script.password), b"node salt", 4096).as_bytes(),
            )
            .expect("verifier");
            let (exchange, mut server_first) =
                ServerExchange::start(ServerCredential::Known(&verifier), &data)
                    .expect("client first");
            match tamper {
                Tamper::ForeignNonce => server_first = format!("r=other{}", &server_first[2..]),
                Tamper::Count(count) => {
                    server_first = server_first.replace(",i=4096", &format!(",i={count}"));
                }
                _ => {}
            }
            tls.write_all(&auth::encode_authentication_sasl_continue(
                server_first.as_bytes(),
            ))
            .await
            .ok()?;
            let message = read_message(&mut tls, 1024).await.ok()?;
            let server_final = match exchange.finish(&message.body) {
                Ok(server_final) => server_final,
                Err(_) => {
                    // PostgreSQL answers a wrong password with an ErrorResponse.
                    tls.write_all(&backend_error("28P01")).await.ok()?;
                    return None;
                }
            };
            let server_final = match tamper {
                Tamper::ErrorFinal => "e=invalid-proof".to_string(),
                Tamper::WrongSignature => text::encode_server_final(&[0; 32]),
                _ => server_final,
            };
            tls.write_all(&auth::encode_authentication_sasl_final(
                server_final.as_bytes(),
            ))
            .await
            .ok()?;
            tls.write_all(&auth::encode_authentication_ok())
                .await
                .ok()?;
        }
    }
    if script.error_after_auth {
        tls.write_all(&backend_error("3D000")).await.ok()?;
        return None;
    }
    for (name, value) in &script.params {
        tls.write_all(&auth::encode_parameter_status(name, value))
            .await
            .ok()?;
    }
    tls.write_all(&auth::encode_backend_key_data(4242, &[1, 2, 3, 4]))
        .await
        .ok()?;
    tls.write_all(&encode_message(b'N', b"SNOTICE\0Mhello\0\0"))
        .await
        .ok()?;
    tls.write_all(&auth::encode_ready_for_query(script.status))
        .await
        .ok()?;
    loop {
        let message = read_message(&mut tls, 65536).await.ok()?;
        match message.tag {
            b'Q' => {
                tls.write_all(&encode_message(b'T', &[0, 0])).await.ok()?;
                for row in &script.rows {
                    tls.write_all(&data_row_message(row)).await.ok()?;
                }
                tls.write_all(&encode_message(b'C', b"SELECT 1\0"))
                    .await
                    .ok()?;
                tls.write_all(&auth::encode_ready_for_query(TransactionStatus::Idle))
                    .await
                    .ok()?;
            }
            _ => return Some(()),
        }
    }
}

fn backend_error(code: &str) -> Vec<u8> {
    let body = format!(
        "SFATAL\0VFATAL\0C{code}\0Mpassword authentication failed for user \"secret-login\"\0\0"
    );
    encode_message(b'E', body.as_bytes())
}

fn data_row_message(row: &[Option<&str>]) -> Vec<u8> {
    let mut body = u16::try_from(row.len())
        .expect("small")
        .to_be_bytes()
        .to_vec();
    for column in row {
        match column {
            None => body.extend_from_slice(&(-1i32).to_be_bytes()),
            Some(value) => {
                body.extend_from_slice(&i32::try_from(value.len()).expect("small").to_be_bytes());
                body.extend_from_slice(value.as_bytes());
            }
        }
    }
    encode_message(b'D', &body)
}

struct Run {
    result: Result<BackendConnection<DuplexStream>, BackendError>,
    seen: Seen,
}

async fn run(script: Script, tls: NodeTls, login: &Login, purpose: Purpose) -> Run {
    let (client, server) = tokio::io::duplex(65536);
    let (tx, rx) = oneshot::channel();
    let task = tokio::spawn(node(server, script, tx));
    let result = establish(client, &tls, login, purpose, deadline()).await;
    let seen = rx.await.unwrap_or_default();
    if result.is_err() {
        let _ = task.await;
    }
    Run { result, seen }
}

fn login(password: &str) -> Login {
    Login::new(
        "secret-login",
        "appdb",
        Credential::new(self::password(password)),
    )
}

fn good_tls() -> NodeTls {
    node_tls("ca.pem", "node.test")
}

#[tokio::test]
async fn a_session_opens_with_scram_and_keeps_parameters_and_key() {
    let login = login("backend-secret").forward("application_name", b"psql");
    let run = run(Script::primary(), good_tls(), &login, Purpose::Session).await;
    let mut conn = run.result.expect("open");
    assert_eq!(
        run.seen.startup,
        [
            ("user".to_string(), "secret-login".to_string()),
            ("database".to_string(), "appdb".to_string()),
            ("application_name".to_string(), "psql".to_string())
        ]
    );
    assert_eq!(run.seen.mechanism, auth::SCRAM_SHA_256);
    // The header n, an empty name and an 18 byte nonce in base64.
    let nonce = run
        .seen
        .client_first
        .strip_prefix("n,,n=,r=")
        .expect("client first shape");
    assert_eq!(
        pool_protocol::base64::decode(nonce.as_bytes()).map(|n| n.len()),
        Ok(18)
    );
    assert_eq!(conn.parameters().len(), 3);
    assert_eq!(conn.backend_key().map(|k| k.process_id), Some(4242));
    assert_eq!(conn.status(), TransactionStatus::Idle);
    let row = conn.query_one_row("select 1").await.expect("query");
    assert_eq!(
        row,
        Some(vec![
            Some("7613".to_string()),
            Some("primary".to_string()),
            None
        ])
    );
    let debug = format!("{conn:?} {login:?}");
    for secret in ["backend-secret", "secret-login", "appdb", "4242", "psql"] {
        assert!(!debug.contains(secret), "{secret} in {debug}");
    }
}

#[tokio::test]
async fn scram_sha_256_is_chosen_beside_plus() {
    let mut script = Script::primary();
    script.mechanisms = vec![auth::SCRAM_SHA_256_PLUS, auth::SCRAM_SHA_256];
    let run = run(
        script,
        good_tls(),
        &login("backend-secret"),
        Purpose::Session,
    )
    .await;
    assert!(run.result.is_ok());
    assert_eq!(run.seen.mechanism, auth::SCRAM_SHA_256);
    assert!(run.seen.client_first.starts_with("n,,"));
}

#[tokio::test]
async fn the_keys_are_derived_once_per_credential_salt_and_count() {
    let login = login("backend-secret");
    for _ in 0..3 {
        assert!(
            run(Script::primary(), good_tls(), &login, Purpose::Probe)
                .await
                .result
                .is_ok()
        );
    }
    assert_eq!(login.credential.derivations(), 1);
}

#[tokio::test]
async fn no_startup_packet_without_s_and_a_checked_certificate() {
    let mut script = Script::primary();
    script.answer = b'N';
    let run_n = run(
        script,
        good_tls(),
        &login("backend-secret"),
        Purpose::Session,
    )
    .await;
    assert_eq!(run_n.result.err(), Some(BackendError::TlsRefused));
    assert!(run_n.seen.startup.is_empty());
    for tls in [
        node_tls("other-ca.pem", "node.test"),
        node_tls("ca.pem", "other.test"),
    ] {
        let run = run(
            Script::primary(),
            tls,
            &login("backend-secret"),
            Purpose::Session,
        )
        .await;
        assert_eq!(run.result.err(), Some(BackendError::Tls));
        assert!(
            run.seen.startup.is_empty(),
            "no startup packet after a failed check"
        );
    }
}

#[tokio::test]
async fn methods_other_than_scram_end_the_attempt() {
    for auth in [
        Auth::Method(3),
        Auth::Method(5),
        Auth::Method(7),
        Auth::OkAtOnce,
        Auth::Mechanisms(vec![auth::SCRAM_SHA_256_PLUS]),
    ] {
        let mut script = Script::primary();
        script.auth = auth;
        let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        assert_eq!(run.result.err(), Some(BackendError::AuthenticationMethod));
    }
}

#[tokio::test]
async fn a_wrong_password_and_a_bad_server_end_the_attempt() {
    let run_wrong = run(
        Script::primary(),
        good_tls(),
        &login("not-the-password"),
        Purpose::Probe,
    )
    .await;
    assert_eq!(
        run_wrong.result.err(),
        Some(BackendError::Server {
            sqlstate: Some(*b"28P01")
        })
    );
    for (tamper, expected) in [
        (Tamper::ForeignNonce, BackendError::Protocol),
        (Tamper::Count(600_001), BackendError::Protocol),
        (Tamper::Count(0), BackendError::Protocol),
        (Tamper::ErrorFinal, BackendError::Authentication),
        (Tamper::WrongSignature, BackendError::Authentication),
    ] {
        let mut script = Script::primary();
        script.auth = Auth::Scram(tamper);
        let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        assert_eq!(run.result.err(), Some(expected));
    }
}

#[tokio::test]
async fn a_session_needs_in_hot_standby_off_and_an_idle_status() {
    for params in [
        vec![("in_hot_standby", "on")],
        vec![("server_version", "18.7")],
    ] {
        let mut script = Script::primary();
        script.params = params;
        let session = run(
            script.clone(),
            good_tls(),
            &login("backend-secret"),
            Purpose::Session,
        )
        .await;
        assert_eq!(session.result.err(), Some(BackendError::NotPrimary));
        let probe = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        assert!(probe.result.is_ok(), "the probe reads the role itself");
    }
    let mut script = Script::primary();
    script.status = TransactionStatus::InTransaction;
    let run_t = run(
        script,
        good_tls(),
        &login("backend-secret"),
        Purpose::Session,
    )
    .await;
    assert_eq!(run_t.result.err(), Some(BackendError::NotIdle));
    let mut script = Script::primary();
    script.error_after_auth = true;
    let run_e = run(
        script,
        good_tls(),
        &login("backend-secret"),
        Purpose::Session,
    )
    .await;
    assert_eq!(
        run_e.result.err(),
        Some(BackendError::Server {
            sqlstate: Some(*b"3D000")
        })
    );
}

#[tokio::test]
async fn too_many_parameters_end_the_attempt() {
    let names: Vec<&'static str> = (0..=PARAMETER_STATUS_MAX_COUNT)
        .map(|i| &*Box::leak(format!("p{i}").into_boxed_str()))
        .collect();
    let mut script = Script::primary();
    script.params = names.iter().map(|n| (*n, "x")).collect();
    let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
    assert_eq!(run.result.err(), Some(BackendError::Protocol));
}

#[tokio::test]
async fn the_one_row_query_returns_nothing_for_zero_or_two_rows() {
    for rows in [vec![], vec![vec![Some("a")], vec![Some("b")]]] {
        let mut script = Script::primary();
        script.rows = rows;
        let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        let mut conn = run.result.expect("open");
        assert_eq!(conn.query_one_row("select 1").await, Ok(None));
    }
}

#[tokio::test]
async fn a_silent_node_runs_into_the_deadline() {
    let (client, _server) = tokio::io::duplex(1024);
    let soon = Instant::now() + Duration::from_millis(100);
    let result = establish(client, &good_tls(), &login("x"), Purpose::Probe, soon).await;
    assert_eq!(result.err(), Some(BackendError::Timeout));
}

#[tokio::test]
async fn connect_over_tcp() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let port = listener.local_addr().expect("addr").port();
    let accept = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept");
        let (tx, _rx) = oneshot::channel();
        // Bridge the TCP stream to the scripted node through a duplex pair.
        let (a, b) = tokio::io::duplex(65536);
        let bridge = tokio::spawn(async move {
            let (mut r, mut w) = tokio::io::split(stream);
            let (mut ar, mut aw) = tokio::io::split(a);
            let _ = tokio::join!(
                tokio::io::copy(&mut r, &mut aw),
                tokio::io::copy(&mut ar, &mut w)
            );
        });
        node(b, Script::primary(), tx).await;
        bridge.abort();
    });
    let node = Node::new("127.0.0.1", port, good_tls());
    let mut conn = connect(
        &node,
        &login("backend-secret"),
        Purpose::Session,
        deadline(),
    )
    .await
    .expect("open over TCP");
    assert!(
        conn.query_one_row("select 1")
            .await
            .expect("query")
            .is_some()
    );
    drop(conn);
    accept.abort();
    // A closed port: no connection.
    let closed = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let port = closed.local_addr().expect("addr").port();
    drop(closed);
    let result = connect(
        &Node::new("127.0.0.1", port, good_tls()),
        &login("x"),
        Purpose::Probe,
        deadline(),
    )
    .await;
    assert_eq!(result.err(), Some(BackendError::Connect));
    assert!(!format!("{node:?}").contains("127.0.0.1"));
}

#[tokio::test]
async fn a_probe_answer_over_4096_bytes_is_not_read_on() {
    for (len, ok) in [(3500usize, true), (5000, false)] {
        let mut script = Script::primary();
        let value: &'static str = Box::leak("x".repeat(len).into_boxed_str());
        script.rows = vec![vec![Some(value)]];
        let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        let mut conn = run.result.expect("open");
        let answer = conn.query_one_row("select 1").await;
        if ok {
            assert!(matches!(answer, Ok(Some(_))), "{len}");
        } else {
            assert_eq!(answer, Err(BackendError::Protocol), "{len}");
        }
    }
}

#[tokio::test]
async fn a_startup_message_of_the_node_over_16384_bytes_ends_the_attempt() {
    for (len, ok) in [(16000usize, true), (16500, false)] {
        let mut script = Script::primary();
        let value: &'static str = Box::leak("v".repeat(len).into_boxed_str());
        script.params = vec![("in_hot_standby", "off"), ("big", value)];
        let run = run(script, good_tls(), &login("backend-secret"), Purpose::Probe).await;
        assert_eq!(run.result.is_ok(), ok, "{len}");
        if !ok {
            assert_eq!(run.result.err(), Some(BackendError::Protocol));
        }
    }
}
