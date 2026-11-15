//! The whole session over in memory streams (`tokio::io::duplex`): a scripted
//! client written with the encoders of `pool-protocol` and the SCRAM client
//! of this crate, and a scripted node behind the connector. No socket, no
//! container, no PostgreSQL. The hermetic halves of `0004 AC-14` to `AC-17`,
//! `AC-19`, `AC-20`, `AC-22`, `AC-23` and `AC-25`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use pool_config::{AuthorityRevision, ClusterId, NodeId, TopologyEpoch, ValidatedConfig};
use pool_core::backend::{
    BackendConnection, BackendError, Connector, Credential, Login, Node, Purpose, establish,
};
use pool_core::report::Refusal;
use pool_core::route::{Route, RouteState, RouteView};
use pool_core::scram::{
    ClientExchange, MockKey, ServerCredential, ServerExchange, derive_keys, verifier_text,
};
use pool_core::secret::{CertificateChain, Password, PrivateKey, TrustRoots, Verifier};
use pool_core::session::{Server, ServerParts};
use pool_core::tls::{NodeTls, server_config};
use pool_protocol::auth::{self, Authentication, TransactionStatus};
use pool_protocol::message::{
    Message, MessageReader, encode_message, encode_query, encode_terminate,
};
use pool_protocol::sqlstate::error_sqlstate;
use pool_protocol::startup::{
    FirstPacket, FirstPacketReader, GSSENC_REQUEST_CODE, encode_ssl_request, encode_startup,
    encode_startup_version,
};
use rustls::pki_types::ServerName;
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio::sync::watch;
use tokio::time::Instant;
use tokio_rustls::client::TlsStream;
use tokio_rustls::{TlsAcceptor, TlsConnector};

const FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
const PRINCIPAL: &str = "orders_app";
const PRINCIPAL_PASSWORD: &str = "client-secret";
const BACKEND_LOGIN: &str = "orders_rw";
const BACKEND_DATABASE: &str = "orders_db";
const BACKEND_PASSWORD: &str = "backend-secret";

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}/{name}")).expect("fixture")
}

fn password(text: &str) -> Password {
    Password::parse(text.as_bytes()).expect("password")
}

fn config(limits: &str) -> ValidatedConfig {
    let text = format!(
        r#"config_version = 1

[control]
state_directory = "/var/lib/pool"
authority_id = "0123456789abcdef0123456789abcdef"

[listener]
host = "127.0.0.1"
port = 6432
tls_certificate_file = "/etc/pool/server.crt"
tls_private_key_file = "/etc/pool/server.key"

[limits]
{limits}

[[cluster]]
id = "main"
system_identifier = "7412345678901234567"
expected_writer = "pg1"
monitor_login = "pool_monitor"
monitor_credential_file = "/etc/pool/monitor"

[[node]]
id = "pg1"
cluster = "main"
host = "node.test"
port = 5432
tls_ca_file = "/etc/pool/ca.crt"

[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "main"
backend_database = "{BACKEND_DATABASE}"
backend_login = "{BACKEND_LOGIN}"
credential_file = "/etc/pool/orders_rw"

[[principal]]
id = "orders-app"
name = "{PRINCIPAL}"
verifier_file = "/etc/pool/orders_app.scram"
"#
    );
    pool_config::validate(text.as_bytes()).expect("a valid test configuration")
}

// ---- the scripted node ----

/// What the node does when it gets a query, chosen by the SQL text.
fn answer(sql: &str) -> Answer {
    match sql {
        "select 1" => Answer::Rows(vec!["1".into()], TransactionStatus::Idle),
        "begin" | "insert" => Answer::Command(TransactionStatus::InTransaction),
        "commit" => Answer::Command(TransactionStatus::Idle),
        "commit, then the connection is cut" => Answer::Cut,
        "copy" => Answer::Copy,
        "a big row" => Answer::Rows(vec!["x".repeat(200_000)], TransactionStatus::Idle),
        "a broken message" => Answer::Broken,
        _ => Answer::Hang,
    }
}

enum Answer {
    Rows(Vec<String>, TransactionStatus),
    Command(TransactionStatus),
    Cut,
    Copy,
    Broken,
    Hang,
}

#[derive(Default)]
struct NodeLog {
    startups: Vec<(String, String)>,
    queries: Vec<String>,
    terminated: usize,
    closed: usize,
}

struct NodeState {
    hot_standby: Mutex<&'static str>,
    opened: AtomicUsize,
    log: Mutex<NodeLog>,
    after_open: Mutex<Option<RouteView>>,
    routes: watch::Sender<RouteView>,
    tls: Mutex<Option<NodeTls>>,
}

#[derive(Clone)]
struct TestConnector(Arc<NodeState>);

impl Connector for TestConnector {
    type Stream = DuplexStream;

    fn open(
        &self,
        node: &Node,
        login: &Login,
        purpose: Purpose,
        deadline: Instant,
    ) -> impl std::future::Future<Output = Result<BackendConnection<DuplexStream>, BackendError>> + Send
    {
        let state = Arc::clone(&self.0);
        let tls = state
            .tls
            .lock()
            .expect("lock")
            .clone()
            .unwrap_or_else(|| node.tls().clone());
        let login = login.clone();
        async move {
            state.opened.fetch_add(1, Ordering::SeqCst);
            let (client, server) = tokio::io::duplex(1 << 20);
            tokio::spawn(run_node(server, Arc::clone(&state)));
            let result = establish(client, &tls, &login, purpose, deadline).await;
            if let Some(view) = state.after_open.lock().expect("lock").take() {
                state.routes.send_replace(view);
            }
            result
        }
    }
}

async fn read_msg<S: tokio::io::AsyncRead + Unpin>(stream: &mut S, limit: u32) -> Option<Message> {
    let mut reader = MessageReader::new(limit);
    let mut buf = vec![0u8; 65536];
    loop {
        let want = reader.wanted().clamp(1, buf.len());
        stream.read_exact(&mut buf[..want]).await.ok()?;
        if let (_, Some(result)) = reader.feed(&buf[..want]) {
            return result.ok();
        }
    }
}

async fn run_node(mut stream: DuplexStream, state: Arc<NodeState>) {
    let mut first = [0u8; 8];
    if stream.read_exact(&mut first).await.is_err() {
        return;
    }
    let _ = stream.write_all(b"S").await;
    let chain = CertificateChain::parse(&fixture("node.pem")).expect("chain");
    let key = PrivateKey::parse(&fixture("node.key")).expect("key");
    let Ok(mut tls) = TlsAcceptor::from(server_config(&chain, &key).expect("config"))
        .accept(stream)
        .await
    else {
        return;
    };
    let mut reader = FirstPacketReader::new(65536);
    let mut byte = [0u8; 1];
    let request = loop {
        if tls.read_exact(&mut byte).await.is_err() {
            return;
        }
        if let (_, Some(result)) = reader.feed(&byte) {
            match result {
                Ok(FirstPacket::Startup(request)) => break request,
                _ => return,
            }
        }
    };
    let text = |b: &[u8]| String::from_utf8(b.to_vec()).expect("utf-8");
    state
        .log
        .lock()
        .expect("lock")
        .startups
        .push((text(&request.user), text(&request.database)));
    // SCRAM as PostgreSQL does it, against the backend password.
    let verifier =
        Verifier::parse(verifier_text(&password(BACKEND_PASSWORD), b"node salt", 4096).as_bytes())
            .expect("verifier");
    let _ = tls
        .write_all(&auth::encode_authentication_sasl(&[auth::SCRAM_SHA_256]))
        .await;
    let Some(initial) = read_msg(&mut tls, 1024).await else {
        return;
    };
    let initial = auth::parse_sasl_initial_response(&initial.body).expect("initial");
    let (exchange, server_first) = ServerExchange::start(
        ServerCredential::Known(&verifier),
        &initial.data.expect("data"),
    )
    .expect("first");
    let _ = tls
        .write_all(&auth::encode_authentication_sasl_continue(
            server_first.as_bytes(),
        ))
        .await;
    let Some(fin) = read_msg(&mut tls, 1024).await else {
        return;
    };
    let Ok(server_final) = exchange.finish(&fin.body) else {
        return;
    };
    let _ = tls
        .write_all(&auth::encode_authentication_sasl_final(
            server_final.as_bytes(),
        ))
        .await;
    let _ = tls.write_all(&auth::encode_authentication_ok()).await;
    let hot = *state.hot_standby.lock().expect("lock");
    for (name, value) in [
        ("server_version", "18.7"),
        ("in_hot_standby", hot),
        ("client_encoding", "UTF8"),
    ] {
        let _ = tls
            .write_all(&auth::encode_parameter_status(name, value))
            .await;
    }
    let _ = tls
        .write_all(&auth::encode_backend_key_data(
            4242,
            &[0xde, 0xad, 0xbe, 0xef],
        ))
        .await;
    let _ = tls
        .write_all(&auth::encode_ready_for_query(TransactionStatus::Idle))
        .await;
    let _ = tls.flush().await;
    loop {
        let Some(message) = read_msg(&mut tls, 1 << 24).await else {
            state.log.lock().expect("lock").closed += 1;
            return;
        };
        match message.tag {
            b'X' => {
                state.log.lock().expect("lock").terminated += 1;
                continue;
            }
            b'Q' => {}
            _ => return,
        }
        let sql = text(&message.body[..message.body.len() - 1]);
        state.log.lock().expect("lock").queries.push(sql.clone());
        match answer(&sql) {
            Answer::Rows(columns, status) => {
                let _ = tls.write_all(&encode_message(b'T', &[0, 1])).await;
                let mut row = u16::try_from(columns.len())
                    .expect("small")
                    .to_be_bytes()
                    .to_vec();
                for c in &columns {
                    row.extend_from_slice(&u32::try_from(c.len()).expect("small").to_be_bytes());
                    row.extend_from_slice(c.as_bytes());
                }
                let _ = tls.write_all(&encode_message(b'D', &row)).await;
                let _ = tls.write_all(&encode_message(b'C', b"SELECT 1\0")).await;
                let _ = tls.write_all(&auth::encode_ready_for_query(status)).await;
            }
            Answer::Command(status) => {
                let _ = tls.write_all(&encode_message(b'C', b"OK\0")).await;
                let _ = tls.write_all(&auth::encode_ready_for_query(status)).await;
            }
            Answer::Cut => return,
            Answer::Copy => {
                let _ = tls.write_all(&encode_message(b'G', &[0, 0, 0])).await;
            }
            Answer::Broken => {
                let _ = tls.write_all(&encode_message(b'K', &[0; 8])).await;
            }
            Answer::Hang => {}
        }
        let _ = tls.flush().await;
    }
}

// ---- the harness ----

struct Harness {
    server: Arc<Server<TestConnector>>,
    node: Arc<NodeState>,
    cluster: ClusterId,
    node_id: NodeId,
    origin: Instant,
}

impl Harness {
    fn new(limits: &str) -> Self {
        let config = Arc::new(config(limits));
        let cluster = config.clusters()[0].id().clone();
        let node_id = config.nodes()[0].id().clone();
        let origin = Instant::now();
        let (routes, receiver) = watch::channel(RouteView::closed(std::slice::from_ref(&cluster)));
        let node = Arc::new(NodeState {
            hot_standby: Mutex::new("off"),
            opened: AtomicUsize::new(0),
            log: Mutex::new(NodeLog::default()),
            after_open: Mutex::new(None),
            routes,
            tls: Mutex::new(None),
        });
        let chain = CertificateChain::parse(&fixture("node.pem")).expect("chain");
        let key = PrivateKey::parse(&fixture("node.key")).expect("key");
        let verifier = Verifier::parse(
            verifier_text(&password(PRINCIPAL_PASSWORD), b"principal salt", 4096).as_bytes(),
        )
        .expect("verifier");
        let roots = TrustRoots::parse(&fixture("ca.pem")).expect("roots");
        let parts = ServerParts {
            tls: server_config(&chain, &key).expect("server config"),
            verifiers: vec![(config.principals()[0].id().clone(), verifier)],
            credentials: vec![(
                config.identities()[0].id().clone(),
                Credential::new(password(BACKEND_PASSWORD)),
            )],
            nodes: vec![(
                node_id.clone(),
                Node::new(
                    "node.test",
                    5432,
                    NodeTls::new(&roots, "node.test").expect("tls"),
                ),
            )],
            mock_key: MockKey::generate().expect("random"),
            connector: TestConnector(Arc::clone(&node)),
            routes: receiver,
            origin,
            start_sequence: 3,
            config,
        };
        let harness = Self {
            server: Server::new(parts).expect("server"),
            node,
            cluster,
            node_id,
            origin,
        };
        harness.open();
        harness
    }

    fn route(&self, epoch: u64, fresh_for: Duration) -> RouteView {
        RouteView::new(vec![(
            self.cluster.clone(),
            RouteState::Open(Route {
                node: self.node_id.clone(),
                topology_epoch: TopologyEpoch::from_stored(epoch).expect("epoch"),
                authority_revision: AuthorityRevision::FIRST,
                fresh_until: self.origin.elapsed() + fresh_for,
            }),
        )])
    }

    fn open(&self) {
        self.node
            .routes
            .send_replace(self.route(1, Duration::from_secs(3600)));
    }

    fn close(&self, conflict: bool) {
        self.node.routes.send_replace(RouteView::new(vec![(
            self.cluster.clone(),
            RouteState::Closed { conflict },
        )]));
    }

    /// A new raw connection to the server.
    fn raw(&self) -> DuplexStream {
        let (client, server) = tokio::io::duplex(1 << 20);
        self.server
            .accept_stream(server)
            .expect("a handshake permit");
        client
    }

    fn opened(&self) -> usize {
        self.node.opened.load(Ordering::SeqCst)
    }

    fn log<T>(&self, f: impl FnOnce(&NodeLog) -> T) -> T {
        f(&self.node.log.lock().expect("lock"))
    }
}

async fn upgrade(mut stream: DuplexStream) -> TlsStream<DuplexStream> {
    stream
        .write_all(&encode_ssl_request())
        .await
        .expect("write");
    let mut answer = [0u8; 1];
    stream.read_exact(&mut answer).await.expect("answer");
    assert_eq!(answer, *b"S");
    let mut store = rustls::RootCertStore::empty();
    for cert in rustls::pki_types::CertificateDer::pem_slice_iter(&fixture("ca.pem")) {
        store.add(cert.expect("cert")).expect("add");
    }
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::aws_lc_rs::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("versions")
    .with_root_certificates(store)
    .with_no_client_auth();
    TlsConnector::from(Arc::new(config))
        .connect(ServerName::try_from("node.test").expect("name"), stream)
        .await
        .expect("tls to the listener")
}

use rustls::pki_types::pem::PemObject;

/// How a login ended: a ready client with the parameters it got, or the tags it saw and the SQLSTATE.
enum Login2 {
    Ready(Box<TlsStream<DuplexStream>>, Vec<Message>),
    Refused {
        tags: Vec<u8>,
        sqlstate: Option<[u8; 5]>,
    },
}

impl Login2 {
    fn ready(self) -> (TlsStream<DuplexStream>, Vec<Message>) {
        match self {
            Self::Ready(tls, messages) => (*tls, messages),
            Self::Refused { sqlstate, tags } => panic!(
                "refused with {:?} after {:?}",
                sqlstate.map(|s| String::from_utf8_lossy(&s).into_owned()),
                String::from_utf8_lossy(&tags)
            ),
        }
    }

    fn refused(self) -> (Vec<u8>, String) {
        match self {
            Self::Ready(..) => panic!("the login was not refused"),
            Self::Refused { tags, sqlstate } => (
                tags,
                sqlstate
                    .map(|s| String::from_utf8_lossy(&s).into_owned())
                    .unwrap_or_default(),
            ),
        }
    }
}

/// Startup over TLS and SCRAM as `user` with `password`, asking for `database`.
async fn login_with(
    mut tls: TlsStream<DuplexStream>,
    user: &str,
    secret: &str,
    database: &str,
) -> Login2 {
    let startup = encode_startup(&[
        ("user", user.as_bytes()),
        ("database", database.as_bytes()),
        ("application_name", b"psql"),
    ]);
    tls.write_all(&startup).await.expect("startup");
    scram_and_ready(tls, secret).await
}

async fn scram_and_ready(mut tls: TlsStream<DuplexStream>, secret: &str) -> Login2 {
    let mut tags = Vec::new();
    let refused = |tags: Vec<u8>, m: &Message| Login2::Refused {
        tags,
        sqlstate: error_sqlstate(&m.body),
    };
    let Some(m) = read_msg(&mut tls, 1 << 20).await else {
        return Login2::Refused {
            tags,
            sqlstate: None,
        };
    };
    tags.push(m.tag);
    let m = if m.tag == b'v' {
        let Some(next) = read_msg(&mut tls, 1 << 20).await else {
            return Login2::Refused {
                tags,
                sqlstate: None,
            };
        };
        tags.push(next.tag);
        next
    } else {
        m
    };
    if m.tag == b'E' {
        return refused(tags, &m);
    }
    assert_eq!(
        auth::parse_authentication(&m.body),
        Ok(Authentication::Sasl(vec![auth::SCRAM_SHA_256.into()])),
        "the frontend offers SCRAM-SHA-256 and nothing else"
    );
    let (client, first) = ClientExchange::start().expect("random");
    tls.write_all(&auth::encode_sasl_initial_response(
        auth::SCRAM_SHA_256,
        first.as_bytes(),
    ))
    .await
    .expect("write");
    let Some(m) = read_msg(&mut tls, 1 << 20).await else {
        return Login2::Refused {
            tags,
            sqlstate: None,
        };
    };
    tags.push(m.tag);
    if m.tag == b'E' {
        return refused(tags, &m);
    }
    let Ok(Authentication::SaslContinue(data)) = auth::parse_authentication(&m.body) else {
        panic!("continue")
    };
    let server_first = client.server_first(&data).expect("server first");
    let keys = derive_keys(
        &password(secret),
        &server_first.salt,
        server_first.iterations,
    );
    let (fin, expected) = client.client_final(&server_first, &keys);
    tls.write_all(&auth::encode_sasl_response(fin.as_bytes()))
        .await
        .expect("write");
    let mut messages = Vec::new();
    loop {
        let Some(m) = read_msg(&mut tls, 1 << 20).await else {
            return Login2::Refused {
                tags,
                sqlstate: None,
            };
        };
        tags.push(m.tag);
        match m.tag {
            b'E' => return refused(tags, &m),
            b'R' => {
                if let Ok(Authentication::SaslFinal(data)) = auth::parse_authentication(&m.body) {
                    expected
                        .check(&data)
                        .expect("the server signature of the listener");
                }
            }
            b'Z' => {
                messages.push(m);
                return Login2::Ready(Box::new(tls), messages);
            }
            _ => messages.push(m),
        }
    }
}

async fn login(h: &Harness) -> Login2 {
    login_with(
        upgrade(h.raw()).await,
        PRINCIPAL,
        PRINCIPAL_PASSWORD,
        "orders",
    )
    .await
}

/// Sends a query and reads up to its ReadyForQuery, or to an error and the end of the stream.
async fn query(tls: &mut TlsStream<DuplexStream>, sql: &str) -> Vec<Message> {
    tls.write_all(&encode_query(sql)).await.expect("write");
    read_until_ready(tls).await
}

async fn read_until_ready(tls: &mut TlsStream<DuplexStream>) -> Vec<Message> {
    let mut out = Vec::new();
    while let Some(m) = read_msg(tls, 1 << 24).await {
        let done = m.tag == b'Z';
        out.push(m);
        if done {
            break;
        }
    }
    out
}

fn last_error(messages: &[Message]) -> Option<String> {
    messages
        .iter()
        .rev()
        .find(|m| m.tag == b'E')
        .and_then(|m| error_sqlstate(&m.body))
        .map(|s| String::from_utf8_lossy(&s).into_owned())
}

async fn closed(tls: &mut TlsStream<DuplexStream>) -> bool {
    let mut byte = [0u8; 1];
    matches!(
        tokio::time::timeout(Duration::from_secs(5), tls.read(&mut byte)).await,
        Ok(Ok(0) | Err(_))
    )
}

async fn settle() {
    tokio::time::sleep(Duration::from_millis(50)).await;
}

// ---- AC-23, AC-17: a session, its records and the mapping ----

#[tokio::test]
async fn a_mapped_principal_gets_a_session_on_its_backend_login() {
    let h = Harness::new("");
    let (mut tls, greeting) = login(&h).await.ready();
    // AuthenticationOk is followed by the parameters of the backend and ReadyForQuery; never BackendKeyData.
    let tags: Vec<u8> = greeting.iter().map(|m| m.tag).collect();
    assert_eq!(tags, b"SSSZ");
    let rows = query(&mut tls, "select 1").await;
    assert_eq!(rows.iter().map(|m| m.tag).collect::<Vec<_>>(), b"TDCZ");
    // The node saw the backend login and database of the identity entry, never the principal.
    assert_eq!(
        h.log(|l| l.startups.clone()),
        [(BACKEND_LOGIN.to_string(), BACKEND_DATABASE.to_string())]
    );
    assert_eq!(h.server.sessions(), 1);
    tls.write_all(&encode_terminate()).await.expect("terminate");
    assert!(closed(&mut tls).await);
    settle().await;
    assert_eq!(h.server.sessions(), 0);
    assert_eq!(h.log(|l| l.terminated), 1);
}

#[tokio::test]
async fn a_large_row_streams_through_the_relay_buffer() {
    let h = Harness::new("");
    let (mut tls, _) = login(&h).await.ready();
    let rows = query(&mut tls, "a big row").await;
    let data = rows.iter().find(|m| m.tag == b'D').expect("a row");
    assert_eq!(data.body.len(), 2 + 4 + 200_000);
}

#[tokio::test]
async fn unsupported_and_broken_frontend_messages_end_the_session() {
    for (bytes, code) in [
        (encode_message(b'P', b"\0select 1\0\0\0"), "0A000"),
        (encode_message(b'F', &[0; 4]), "0A000"),
        (encode_message(b'd', &[1]), "0A000"),
        (encode_message(b'?', &[]), "08P01"),
        (vec![b'Q', 0, 0, 0, 3], "08P01"),
    ] {
        let h = Harness::new("");
        let (mut tls, _) = login(&h).await.ready();
        tls.write_all(&bytes).await.expect("write");
        let messages = read_until_ready(&mut tls).await;
        assert_eq!(last_error(&messages).as_deref(), Some(code));
        assert!(closed(&mut tls).await);
        settle().await;
        assert_eq!(h.log(|l| l.queries.len()), 0, "nothing reached the node");
        assert!(
            h.log(|l| l.closed + l.terminated) > 0,
            "the backend is closed"
        );
    }
}

#[tokio::test]
async fn a_copy_response_and_a_broken_backend_message_end_the_session() {
    for (sql, code) in [("copy", "0A000"), ("a broken message", "08007")] {
        let h = Harness::new("");
        let (mut tls, _) = login(&h).await.ready();
        let messages = query(&mut tls, sql).await;
        assert_eq!(last_error(&messages).as_deref(), Some(code), "{sql}");
        assert!(closed(&mut tls).await);
    }
}

// ---- AC-14: the writer check before any SQL, and the conflict ending ----

#[tokio::test]
async fn a_closed_or_stale_route_refuses_with_57p03_and_opens_no_backend() {
    let h = Harness::new("");
    h.close(false);
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "57P03");
    h.node.routes.send_replace(h.route(1, Duration::ZERO));
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "57P03");
    assert_eq!(h.opened(), 0);
    assert_eq!(h.server.counters().refused(Refusal::Closed), 2);
}

#[tokio::test]
async fn a_route_that_changed_while_the_backend_opened_refuses_at_step_9() {
    let h = Harness::new("");
    *h.node.after_open.lock().expect("lock") = Some(h.route(2, Duration::from_secs(3600)));
    let (tags, code) = login(&h).await.refused();
    assert_eq!(code, "57P03");
    assert!(!tags.contains(&b'Z'));
    assert_eq!(h.opened(), 1);
    settle().await;
    assert_eq!(h.log(|l| l.queries.len()), 0);
}

#[tokio::test]
async fn a_node_in_recovery_gives_08006() {
    let h = Harness::new("");
    *h.node.hot_standby.lock().expect("lock") = "on";
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "08006");
}

#[tokio::test]
async fn a_wrong_node_certificate_gives_08006() {
    let h = Harness::new("");
    let roots = TrustRoots::parse(&fixture("other-ca.pem")).expect("roots");
    *h.node.tls.lock().expect("lock") = Some(NodeTls::new(&roots, "node.test").expect("tls"));
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "08006");
    assert_eq!(
        h.log(|l| l.startups.len()),
        0,
        "no startup packet toward the node"
    );
}

#[tokio::test]
async fn a_positive_conflict_ends_running_sessions_and_silence_does_not() {
    let h = Harness::new("");
    let (mut idle, _) = login(&h).await.ready();
    let (mut busy, _) = login(&h).await.ready();
    busy.write_all(&encode_query("a slow statement"))
        .await
        .expect("write");
    settle().await;
    // Silence (writer_missing alone) keeps running sessions.
    h.close(false);
    settle().await;
    let rows = query(&mut idle, "select 1").await;
    assert_eq!(rows.last().map(|m| m.tag), Some(b'Z'));
    // A positive conflict ends both: 57P01 without a query in flight, 08007 with one.
    h.close(true);
    assert_eq!(
        last_error(&read_until_ready(&mut idle).await).as_deref(),
        Some("57P01")
    );
    assert_eq!(
        last_error(&read_until_ready(&mut busy).await).as_deref(),
        Some("08007")
    );
    assert!(closed(&mut idle).await && closed(&mut busy).await);
}

// ---- AC-15: startup ----

#[tokio::test]
async fn startup_without_tls_is_refused_with_28000_and_no_authentication_request() {
    let h = Harness::new("");
    let mut raw = h.raw();
    raw.write_all(&encode_startup(&[("user", PRINCIPAL.as_bytes())]))
        .await
        .expect("write");
    let message = read_msg(&mut raw, 4096)
        .await
        .expect("an error in plain text");
    assert_eq!(message.tag, b'E');
    assert_eq!(error_sqlstate(&message.body), Some(*b"28000"));
    let mut rest = Vec::new();
    let _ = raw.read_to_end(&mut rest).await;
    assert!(rest.is_empty());
}

#[tokio::test]
async fn gssenc_is_answered_with_n_and_a_version_3_2_gets_negotiation() {
    let h = Harness::new("");
    let mut raw = h.raw();
    let mut gss = 8u32.to_be_bytes().to_vec();
    gss.extend_from_slice(&GSSENC_REQUEST_CODE.to_be_bytes());
    raw.write_all(&gss).await.expect("write");
    let mut answer = [0u8; 1];
    raw.read_exact(&mut answer).await.expect("answer");
    assert_eq!(answer, *b"N");
    let mut tls = upgrade(raw).await;
    tls.write_all(&encode_startup_version(
        3,
        2,
        &[("user", PRINCIPAL.as_bytes()), ("database", b"orders")],
    ))
    .await
    .expect("write");
    let (_, greeting) = scram_and_ready(tls, PRINCIPAL_PASSWORD).await.ready();
    assert_eq!(greeting.last().map(|m| m.tag), Some(b'Z'));
}

#[tokio::test]
async fn refused_versions_and_parameters() {
    let h = Harness::new("");
    for major in [2u16, 4] {
        let mut raw = h.raw();
        raw.write_all(&encode_startup_version(major, 0, &[("user", b"x")]))
            .await
            .expect("write");
        let m = read_msg(&mut raw, 4096).await.expect("error");
        assert_eq!(error_sqlstate(&m.body), Some(*b"0A000"));
    }
    for param in [("replication", "database"), ("options", "-c x=1")] {
        let mut tls = upgrade(h.raw()).await;
        tls.write_all(&encode_startup(&[
            ("user", PRINCIPAL.as_bytes()),
            (param.0, param.1.as_bytes()),
        ]))
        .await
        .expect("write");
        let m = read_msg(&mut tls, 4096).await.expect("error");
        assert_eq!(error_sqlstate(&m.body), Some(*b"0A000"));
    }
}

#[tokio::test]
async fn a_cancel_request_is_read_and_dropped() {
    let h = Harness::new("");
    let mut raw = h.raw();
    let mut cancel = 16u32.to_be_bytes().to_vec();
    cancel.extend_from_slice(&80_877_102u32.to_be_bytes());
    cancel.extend_from_slice(&[0, 0, 0x10, 0x92, 0xde, 0xad, 0xbe, 0xef]);
    raw.write_all(&cancel).await.expect("write");
    let mut rest = Vec::new();
    let _ = tokio::time::timeout(Duration::from_secs(5), raw.read_to_end(&mut rest)).await;
    assert!(rest.is_empty(), "no answer");
    settle().await;
    assert_eq!(h.server.counters().refused(Refusal::CancelIgnored), 1);
    assert_eq!(h.opened(), 0);
}

// ---- AC-16: authentication ----

#[tokio::test]
async fn a_wrong_password_and_an_unknown_name_fail_after_the_same_sequence() {
    let h = Harness::new("");
    let (wrong_tags, wrong) = login_with(
        upgrade(h.raw()).await,
        PRINCIPAL,
        "not the password",
        "orders",
    )
    .await
    .refused();
    let (unknown_tags, unknown) = login_with(
        upgrade(h.raw()).await,
        "mallory",
        PRINCIPAL_PASSWORD,
        "orders",
    )
    .await
    .refused();
    assert_eq!((wrong.as_str(), unknown.as_str()), ("28P01", "28P01"));
    assert_eq!(wrong_tags, unknown_tags, "the same message sequence");
    assert_eq!(wrong_tags, b"RRE");
    assert_eq!(h.opened(), 0);
}

#[tokio::test]
async fn an_authentication_message_over_1024_bytes_is_refused() {
    let h = Harness::new("");
    for (size, ok) in [(1024usize, true), (1025, false)] {
        let mut tls = upgrade(h.raw()).await;
        tls.write_all(&encode_startup(&[
            ("user", PRINCIPAL.as_bytes()),
            ("database", b"orders"),
        ]))
        .await
        .expect("w");
        let sasl = read_msg(&mut tls, 4096).await.expect("sasl");
        assert_eq!(sasl.tag, b'R');
        // A client-first message padded with a long nonce to the body size.
        let head = auth::encode_sasl_initial_response(auth::SCRAM_SHA_256, b"");
        let overhead = head.len() - 5;
        let nonce = "a".repeat(size - overhead - "n,,n=,r=".len());
        let msg = auth::encode_sasl_initial_response(
            auth::SCRAM_SHA_256,
            format!("n,,n=,r={nonce}").as_bytes(),
        );
        assert_eq!(msg.len() - 5, size);
        tls.write_all(&msg).await.expect("write");
        let reply = read_msg(&mut tls, 1 << 20).await.expect("reply");
        if ok {
            assert_eq!(reply.tag, b'R', "a body of 1024 bytes is read");
        } else {
            assert_eq!(error_sqlstate(&reply.body), Some(*b"08P01"));
        }
    }
}

// ---- AC-17: the identity map ----

#[tokio::test]
async fn a_database_without_an_identity_entry_gets_28000_and_no_backend() {
    let h = Harness::new("");
    let (_, code) = login_with(
        upgrade(h.raw()).await,
        PRINCIPAL,
        PRINCIPAL_PASSWORD,
        "orders_db",
    )
    .await
    .refused();
    assert_eq!(code, "28000");
    assert_eq!(h.opened(), 0);
}

// ---- AC-19: teardown ----

#[tokio::test]
async fn a_client_that_leaves_inside_a_transaction_closes_its_backend_and_returns_its_permit() {
    let h = Harness::new("session_max_count = 1");
    let (mut tls, _) = login(&h).await.ready();
    assert_eq!(
        query(&mut tls, "begin")
            .await
            .last()
            .map(|m| m.body.clone()),
        Some(b"T".to_vec())
    );
    query(&mut tls, "insert").await;
    drop(tls);
    settle().await;
    assert_eq!(h.server.sessions(), 0);
    assert!(
        h.log(|l| l.closed + l.terminated) >= 1,
        "the backend is closed, so PostgreSQL rolls back"
    );
    // The permit came back: with one session allowed the next login succeeds.
    let (mut again, _) = login(&h).await.ready();
    assert_eq!(
        query(&mut again, "select 1").await.last().map(|m| m.tag),
        Some(b'Z')
    );
}

// ---- AC-20: limits ----

#[tokio::test]
async fn a_query_over_the_message_limit_gets_54000_and_never_reaches_the_node() {
    let h = Harness::new("frontend_message_max_bytes = 1024");
    let (mut tls, _) = login(&h).await.ready();
    let messages = query(&mut tls, &"x".repeat(1100)).await;
    assert_eq!(last_error(&messages).as_deref(), Some("54000"));
    settle().await;
    assert_eq!(h.log(|l| l.queries.len()), 0);
}

#[tokio::test]
async fn a_startup_packet_over_its_limit_is_refused() {
    let h = Harness::new("startup_packet_max_bytes = 512");
    let mut tls = upgrade(h.raw()).await;
    tls.write_all(&encode_startup(&[
        ("user", PRINCIPAL.as_bytes()),
        ("application_name", &[b'a'; 600]),
    ]))
    .await
    .expect("write");
    let m = read_msg(&mut tls, 4096).await.expect("error");
    assert_eq!(error_sqlstate(&m.body), Some(*b"54000"));
}

#[tokio::test]
async fn the_session_limit_gives_53300() {
    let h = Harness::new("session_max_count = 1");
    let (_first, _) = login(&h).await.ready();
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "53300");
    assert_eq!(h.server.counters().refused(Refusal::SessionLimit), 1);
}

#[tokio::test]
async fn waiters_and_the_acquire_deadline() {
    let h = Harness::new(
        "node_backend_max_count = 1\nbackend_wait_max_count = 1\nbackend_acquire_timeout_ms = 300",
    );
    let (_holder, _) = login(&h).await.ready();
    // One waiter waits until the deadline; one more is refused at once.
    let waiting = tokio::spawn({
        let tls = upgrade(h.raw()).await;
        async move {
            let started = std::time::Instant::now();
            let (_, code) = login_with(tls, PRINCIPAL, PRINCIPAL_PASSWORD, "orders")
                .await
                .refused();
            (code, started.elapsed())
        }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    let started = std::time::Instant::now();
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "53300");
    assert!(started.elapsed() < Duration::from_millis(250), "at once");
    let (code, waited) = waiting.await.expect("join");
    assert_eq!(code, "53300");
    assert!(
        waited >= Duration::from_millis(250) && waited < Duration::from_millis(2300),
        "{waited:?}"
    );
    assert_eq!(h.server.counters().refused(Refusal::WaitLimit), 1);
    assert_eq!(h.server.counters().refused(Refusal::AcquireTimeout), 1);
}

#[tokio::test]
async fn a_client_that_sends_nothing_is_closed_at_the_startup_deadline() {
    let h = Harness::new("startup_timeout_ms = 200");
    let mut raw = h.raw();
    let started = std::time::Instant::now();
    let mut rest = Vec::new();
    let _ = tokio::time::timeout(Duration::from_secs(5), raw.read_to_end(&mut rest)).await;
    assert!(rest.is_empty(), "closed without a message");
    assert!(started.elapsed() >= Duration::from_millis(180));
    settle().await;
    assert_eq!(h.server.counters().refused(Refusal::StartupTimeout), 1);
}

#[tokio::test]
async fn the_handshake_limit_closes_a_new_socket_at_once() {
    let h = Harness::new("handshake_max_count = 1");
    let _first = h.raw();
    let (_client, server) = tokio::io::duplex(1024);
    assert!(h.server.accept_stream(server).is_none());
    assert_eq!(h.server.counters().refused(Refusal::HandshakeLimit), 1);
}

// ---- AC-22: a lost COMMIT response ----

#[tokio::test]
async fn a_lost_commit_answer_gives_08007_and_no_second_attempt() {
    let h = Harness::new("");
    let (mut tls, _) = login(&h).await.ready();
    query(&mut tls, "begin").await;
    let messages = query(&mut tls, "commit, then the connection is cut").await;
    assert_eq!(last_error(&messages).as_deref(), Some("08007"));
    assert!(closed(&mut tls).await);
    settle().await;
    assert_eq!(h.opened(), 1, "no new backend for the session");
    assert_eq!(
        h.log(|l| l.queries.iter().filter(|q| q.starts_with("commit")).count()),
        1
    );
}

// ---- AC-25: shutdown ----

#[tokio::test]
async fn shutdown_ends_idle_sessions_with_57p01_and_waits_for_the_others() {
    let h = Harness::new("shutdown_timeout_ms = 2000");
    let (mut idle, _) = login(&h).await.ready();
    let (mut in_tx, _) = login(&h).await.ready();
    query(&mut in_tx, "begin").await;
    let shutdown = tokio::spawn({
        let server = Arc::clone(&h.server);
        async move { server.shutdown().await }
    });
    assert_eq!(
        last_error(&read_until_ready(&mut idle).await).as_deref(),
        Some("57P01")
    );
    // The session in a transaction goes on until it is idle, then it is ended too.
    assert_eq!(
        query(&mut in_tx, "commit").await.last().map(|m| m.tag),
        Some(b'Z')
    );
    assert_eq!(
        last_error(&read_until_ready(&mut in_tx).await).as_deref(),
        Some("57P01")
    );
    tokio::time::timeout(Duration::from_secs(5), shutdown)
        .await
        .expect("shutdown returns")
        .expect("join");
    let (_c, s) = tokio::io::duplex(64);
    assert!(
        h.server.accept_stream(s).is_none(),
        "no accept after shutdown"
    );
}

#[tokio::test]
async fn at_the_shutdown_deadline_a_query_in_flight_gets_08007_never_57p01() {
    let h = Harness::new("shutdown_timeout_ms = 200");
    let (mut busy, _) = login(&h).await.ready();
    busy.write_all(&encode_query("a slow statement"))
        .await
        .expect("write");
    settle().await;
    let started = std::time::Instant::now();
    h.server.shutdown().await;
    assert!(started.elapsed() >= Duration::from_millis(180));
    assert_eq!(
        last_error(&read_until_ready(&mut busy).await).as_deref(),
        Some("08007")
    );
}

#[tokio::test]
async fn shutdown_closes_connections_before_their_session_started() {
    let h = Harness::new("shutdown_timeout_ms = 1000");
    let mut raw = h.raw();
    settle().await;
    h.server.shutdown().await;
    let mut rest = Vec::new();
    let _ = tokio::time::timeout(Duration::from_secs(5), raw.read_to_end(&mut rest)).await;
    assert!(rest.is_empty());
    assert_eq!(h.server.connections(), 0);
}
