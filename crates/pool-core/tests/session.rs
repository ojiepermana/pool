//! The whole session over in memory streams (`tokio::io::duplex`): a scripted
//! client written with the encoders of `pool-protocol` and the SCRAM client
//! of this crate, and a scripted node behind the connector. No socket, no
//! container, no PostgreSQL. The hermetic halves of `0004 AC-14` to `AC-17`,
//! `AC-19`, `AC-20`, `AC-22`, `AC-23` and `AC-25`.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use pool_config::{
    AuthorityRevision, ClusterId, NodeId, SystemIdentifier, TopologyEpoch, ValidatedConfig,
};
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
/// The question of step 8 that the server asks the scripted node; the daemon passes the probe query.
const IDENTITY_QUERY: &str = "who are you";
/// The system identifier of the cluster in the configuration of these tests.
const SYSTEM_IDENTIFIER: &str = "7412345678901234567";
/// The approved writer timeline of the routes of these tests.
const TIMELINE: u32 = 1;

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!("{FIXTURES}/{name}")).expect("fixture")
}

fn password(text: &str) -> Password {
    Password::parse(text.as_bytes()).expect("password")
}

fn config(limits: &str) -> ValidatedConfig {
    config_with(limits, "")
}

/// The test configuration with more lines at the end of the `[[principal]]` table.
fn config_with(limits: &str, principal: &str) -> ValidatedConfig {
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
{principal}
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
        "a flood" => Answer::Flood,
        "half a row" => Answer::Half { cut: false },
        "half a row, then the connection is cut" => Answer::Half { cut: true },
        "stop in a header" => Answer::Stop(stop_in_a_header().concat()),
        "stop in the status byte" => Answer::Stop(stop_in_the_status_byte().concat()),
        _ => Answer::Hang,
    }
}

enum Answer {
    Rows(Vec<String>, TransactionStatus),
    Command(TransactionStatus),
    Cut,
    Copy,
    Broken,
    Flood,
    /// A `RowDescription`, then the first `HALF_ROW` bytes of a `DataRow`; then silence, or a cut.
    Half {
        cut: bool,
    },
    Hang,
    /// These bytes, then silence: the node stops in the middle of a message.
    Stop(Vec<u8>),
}

/// A whole `RowDescription`, then three of the five header bytes of a `DataRow`.
fn stop_in_a_header() -> [Vec<u8>; 2] {
    [encode_message(b'T', &[0, 1]), vec![b'D', 0, 0]]
}

/// A whole `CommandComplete`, then the five header bytes of `ReadyForQuery` without its status byte.
fn stop_in_the_status_byte() -> [Vec<u8>; 2] {
    [encode_message(b'C', b"OK\0"), vec![b'Z', 0, 0, 0, 5]]
}

/// The `RowDescription` of a half answer, then the first bytes of a `DataRow` that claims 100 bytes of body.
fn half_answer() -> Vec<u8> {
    let mut bytes = encode_message(b'T', &[0, 1]);
    bytes.extend_from_slice(&[
        b'D', 0, 0, 0, 104, 0, 1, 0, 0, 0, 94, b'h', b'a', b'l', b'f',
    ]);
    bytes
}

#[derive(Default)]
struct NodeLog {
    startups: Vec<(String, String)>,
    queries: Vec<String>,
    terminated: usize,
    closed: usize,
    /// How often the question of step 8 was asked; it is not in `queries`.
    identity_questions: usize,
}

struct NodeState {
    hot_standby: Mutex<&'static str>,
    opened: AtomicUsize,
    log: Mutex<NodeLog>,
    after_open: Mutex<Option<RouteView>>,
    routes: watch::Sender<RouteView>,
    tls: Mutex<Option<NodeTls>>,
    /// How the node answers the question of step 8.
    identity: Mutex<Identity>,
}

/// The answer of the scripted node to the question of step 8.
#[derive(Clone)]
enum Identity {
    /// These rows, then `CommandComplete` and `ReadyForQuery` with status `I`.
    Rows(Vec<Vec<Option<String>>>),
    /// An `ErrorResponse`, then `ReadyForQuery`.
    Error,
}

impl Identity {
    /// The answer of the approved writer: its system identifier, not in recovery, its timeline.
    fn writer() -> Self {
        Self::row(SYSTEM_IDENTIFIER, "false", &TIMELINE.to_string())
    }

    fn row(system_identifier: &str, in_recovery: &str, timeline: &str) -> Self {
        Self::Rows(vec![vec![
            Some(system_identifier.to_owned()),
            Some(in_recovery.to_owned()),
            Some(timeline.to_owned()),
        ]])
    }
}

fn system_identifier() -> SystemIdentifier {
    SystemIdentifier::parse(SYSTEM_IDENTIFIER).expect("system identifier")
}

/// A `DataRow` of text columns, a column `None` being NULL.
fn data_row(columns: &[Option<String>]) -> Vec<u8> {
    let mut row = u16::try_from(columns.len())
        .expect("small")
        .to_be_bytes()
        .to_vec();
    for column in columns {
        match column {
            None => row.extend_from_slice(&(-1i32).to_be_bytes()),
            Some(c) => {
                row.extend_from_slice(&u32::try_from(c.len()).expect("small").to_be_bytes());
                row.extend_from_slice(c.as_bytes());
            }
        }
    }
    encode_message(b'D', &row)
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
    // One reader per connection: it has seen the SSLRequest read above.
    let _ = reader.feed(&first);
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
    let verifier = Verifier::parse(
        verifier_text(
            &password(BACKEND_PASSWORD),
            b"node salt",
            std::num::NonZeroU32::new(4096).expect("count"),
        )
        .as_bytes(),
    )
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
        if sql == IDENTITY_QUERY {
            state.log.lock().expect("lock").identity_questions += 1;
            let identity = state.identity.lock().expect("lock").clone();
            let _ = tls.write_all(&encode_message(b'T', &[0, 3])).await;
            match identity {
                Identity::Rows(rows) => {
                    for row in &rows {
                        let _ = tls.write_all(&data_row(row)).await;
                    }
                    let _ = tls.write_all(&encode_message(b'C', b"SELECT 1\0")).await;
                }
                Identity::Error => {
                    let _ = tls
                        .write_all(&encode_message(
                            b'E',
                            b"SERROR\0VERROR\0C42501\0Mpermission denied\0\0",
                        ))
                        .await;
                }
            }
            let _ = tls
                .write_all(&auth::encode_ready_for_query(TransactionStatus::Idle))
                .await;
            let _ = tls.flush().await;
            continue;
        }
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
            Answer::Flood => {
                // 8 MB of rows of 8000 bytes: far more than any buffer between here and a client that stops reading.
                let mut row = 1u16.to_be_bytes().to_vec();
                row.extend_from_slice(&8000u32.to_be_bytes());
                row.extend_from_slice(&[b'f'; 8000]);
                let row = encode_message(b'D', &row);
                let _ = tls.write_all(&encode_message(b'T', &[0, 1])).await;
                for _ in 0..1000 {
                    if tls.write_all(&row).await.is_err() {
                        state.log.lock().expect("lock").closed += 1;
                        return;
                    }
                }
                let _ = tls.write_all(&encode_message(b'C', b"SELECT 1000\0")).await;
                let _ = tls
                    .write_all(&auth::encode_ready_for_query(TransactionStatus::Idle))
                    .await;
            }
            Answer::Half { cut } => {
                let _ = tls.write_all(&half_answer()).await;
                let _ = tls.flush().await;
                if cut {
                    return;
                }
            }
            Answer::Stop(bytes) => {
                let _ = tls.write_all(&bytes).await;
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
        Self::with_config(config(limits))
    }

    fn with_config(config: ValidatedConfig) -> Self {
        Self::build(config, Some(IDENTITY_QUERY))
    }

    /// A harness whose server got no question for step 8, as `Server::new` builds it.
    fn without_identity_query() -> Self {
        Self::build(config(""), None)
    }

    fn build(config: ValidatedConfig, identity_query: Option<&'static str>) -> Self {
        let config = Arc::new(config);
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
            identity: Mutex::new(Identity::writer()),
        });
        let chain = CertificateChain::parse(&fixture("node.pem")).expect("chain");
        let key = PrivateKey::parse(&fixture("node.key")).expect("key");
        let verifier = Verifier::parse(
            verifier_text(
                &password(PRINCIPAL_PASSWORD),
                b"principal salt",
                std::num::NonZeroU32::new(4096).expect("count"),
            )
            .as_bytes(),
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
            server: match identity_query {
                Some(query) => Server::with_identity_query(parts, query),
                None => Server::new(parts),
            }
            .expect("server"),
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
                system_identifier: system_identifier(),
                timeline: TIMELINE,
            }),
        )])
    }

    fn open(&self) {
        self.node
            .routes
            .send_replace(self.route(1, Duration::from_secs(3600)));
    }

    fn close(&self, conflict: bool) {
        self.close_with(conflict, false);
    }

    /// Closes the cluster with both flags of the view, as the control loop does.
    fn close_with(&self, conflict: bool, lost: bool) {
        self.node.routes.send_replace(RouteView::new(vec![(
            self.cluster.clone(),
            RouteState::Closed { conflict, lost },
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
        std::num::NonZeroU32::new(server_first.iterations).expect("count"),
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
        (encode_message(b'X', b"x"), "08P01"),
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

// ---- a client that stops reading in the middle of a large answer ----

/// Logs in, asks for 8 MB of rows and never reads again, so the relay waits on a write toward the client.
async fn a_stuck_client(h: &Harness) -> TlsStream<DuplexStream> {
    let (mut tls, _) = login(h).await.ready();
    tls.write_all(&encode_query("a flood"))
        .await
        .expect("write");
    tokio::time::sleep(Duration::from_millis(500)).await;
    tls
}

#[tokio::test]
async fn the_shutdown_deadline_ends_a_session_whose_client_stopped_reading() {
    let h = Harness::new("shutdown_timeout_ms = 300");
    let _stuck = a_stuck_client(&h).await;
    assert_eq!(h.server.sessions(), 1);
    let started = std::time::Instant::now();
    tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
        .await
        .expect("shutdown returns although the client does not read");
    assert!(started.elapsed() >= Duration::from_millis(280));
    assert_eq!(h.server.sessions(), 0);
    assert_eq!(h.server.connections(), 0);
    settle().await;
    assert_eq!(h.log(|l| l.closed), 1, "the backend is closed");
}

#[tokio::test]
async fn a_positive_conflict_ends_a_session_whose_client_stopped_reading() {
    let h = Harness::new("");
    let _stuck = a_stuck_client(&h).await;
    assert_eq!(h.server.sessions(), 1);
    h.close(true);
    let ended = async {
        while h.server.sessions() > 0 {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    };
    tokio::time::timeout(Duration::from_secs(3), ended)
        .await
        .expect("the session ends although the client does not read");
    settle().await;
    assert_eq!(h.log(|l| l.closed), 1, "the backend is closed");
}

// ---- added by the tests of scope 004 (`/test one real primary path`) ----

/// Reads every byte the client gets until the stream ends, for at most five seconds.
async fn read_rest(tls: &mut TlsStream<DuplexStream>) -> Vec<u8> {
    let mut rest = Vec::new();
    let _ = tokio::time::timeout(Duration::from_secs(5), tls.read_to_end(&mut rest)).await;
    rest
}

/// Waits, asking every 20 ms for at most two seconds, until `done` holds.
async fn eventually(mut done: impl FnMut() -> bool) -> bool {
    for _ in 0..100 {
        if done() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    done()
}

/// A route that the second look of step 9 sees in place of the first.
fn route_with(
    h: &Harness,
    node: &str,
    epoch: u64,
    revision: u64,
    fresh_for: Duration,
) -> RouteView {
    RouteView::new(vec![(
        h.cluster.clone(),
        RouteState::Open(Route {
            node: NodeId::parse(node).expect("node id"),
            topology_epoch: TopologyEpoch::from_stored(epoch).expect("epoch"),
            authority_revision: AuthorityRevision::from_stored(revision).expect("revision"),
            fresh_until: h.origin.elapsed() + fresh_for,
            system_identifier: system_identifier(),
            timeline: TIMELINE,
        }),
    )])
}

/// The login is refused at step 9: `57P03`, no `AuthenticationOk`, the backend that was opened is closed
/// again and got no byte after its login.
async fn refused_at_the_second_look(h: &Harness, second: RouteView) {
    *h.node.after_open.lock().expect("lock") = Some(second);
    let (tags, code) = login(h).await.refused();
    assert_eq!(code, "57P03");
    assert!(!tags.contains(&b'Z'), "no ReadyForQuery");
    assert_eq!(h.opened(), 1, "the backend was opened once");
    assert!(
        eventually(|| h.log(|l| l.closed + l.terminated) == 1).await,
        "the backend is closed again"
    );
    assert_eq!(h.log(|l| l.queries.len()), 0, "no SQL reached the node");
    assert_eq!(h.server.sessions(), 0);
    assert_eq!(h.server.counters().refused(Refusal::Closed), 1);
}

// Gap 1 of the checker: each comparison of the second route check on its own.

#[tokio::test]
async fn ac14_a_second_look_that_names_another_node_refuses_with_57p03() {
    let h = Harness::new("");
    let second = route_with(&h, "pg2", 1, 1, Duration::from_secs(3600));
    refused_at_the_second_look(&h, second).await;
}

#[tokio::test]
async fn ac14_a_second_look_with_another_topology_epoch_refuses_with_57p03() {
    let h = Harness::new("");
    let second = route_with(&h, "pg1", 2, 1, Duration::from_secs(3600));
    refused_at_the_second_look(&h, second).await;
}

#[tokio::test]
async fn ac14_a_second_look_with_another_authority_revision_refuses_with_57p03() {
    let h = Harness::new("");
    let second = route_with(&h, "pg1", 1, 2, Duration::from_secs(3600));
    refused_at_the_second_look(&h, second).await;
}

#[tokio::test]
async fn ac14_a_second_look_at_a_route_that_went_stale_refuses_with_57p03() {
    let h = Harness::new("");
    let second = route_with(&h, "pg1", 1, 1, Duration::ZERO);
    refused_at_the_second_look(&h, second).await;
}

#[tokio::test]
async fn ac14_a_second_look_at_a_closed_route_refuses_with_57p03() {
    let h = Harness::new("");
    let second = RouteView::new(vec![(
        h.cluster.clone(),
        RouteState::Closed {
            conflict: false,
            lost: false,
        },
    )]);
    refused_at_the_second_look(&h, second).await;
}

#[tokio::test]
async fn ac14_the_same_route_published_again_or_only_fresher_lets_the_session_open() {
    for fresh_for in [Duration::from_secs(3600), Duration::from_secs(7200)] {
        let h = Harness::new("");
        *h.node.after_open.lock().expect("lock") = Some(route_with(&h, "pg1", 1, 1, fresh_for));
        let (mut tls, _) = login(&h).await.ready();
        assert_eq!(
            query(&mut tls, "select 1").await.last().map(|m| m.tag),
            Some(b'Z')
        );
    }
}

// Gap 3: endings in the middle of a message, leases that are dropped, the principal limit, the
// mechanism name, connections before their session at shutdown.

#[tokio::test]
async fn ac23_a_node_that_closes_in_the_middle_of_a_message_closes_the_client_with_no_error() {
    let h = Harness::new("");
    let (mut tls, _) = login(&h).await.ready();
    tls.write_all(&encode_query("half a row, then the connection is cut"))
        .await
        .expect("write");
    // Exactly the bytes of the node, then the end: no error message after a cut message.
    assert_eq!(read_rest(&mut tls).await, half_answer());
    assert_eq!(h.opened(), 1, "no second backend");
    assert!(eventually(|| h.server.sessions() == 0).await);
}

#[tokio::test]
async fn ac14_a_positive_conflict_in_the_middle_of_a_message_closes_the_client_with_no_error() {
    let h = Harness::new("");
    let (mut tls, _) = login(&h).await.ready();
    tls.write_all(&encode_query("half a row"))
        .await
        .expect("write");
    settle().await;
    h.close(true);
    assert_eq!(read_rest(&mut tls).await, half_answer());
    assert!(eventually(|| h.server.sessions() == 0).await);
    assert!(
        eventually(|| h.log(|l| l.closed) == 1).await,
        "the backend is closed"
    );
}

#[tokio::test]
async fn ac25_the_shutdown_deadline_in_the_middle_of_a_message_closes_the_client_with_no_error() {
    let h = Harness::new("shutdown_timeout_ms = 200");
    let (mut tls, _) = login(&h).await.ready();
    tls.write_all(&encode_query("half a row"))
        .await
        .expect("write");
    settle().await;
    tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
        .await
        .expect("shutdown returns");
    assert_eq!(read_rest(&mut tls).await, half_answer());
    assert!(
        eventually(|| h.log(|l| l.closed) == 1).await,
        "the backend is closed"
    );
}

#[tokio::test]
async fn ac14_a_positive_conflict_closes_the_backend_of_every_session_it_ends() {
    let h = Harness::new("");
    let (mut first, _) = login(&h).await.ready();
    let (mut second, _) = login(&h).await.ready();
    h.close(true);
    assert_eq!(
        last_error(&read_until_ready(&mut first).await).as_deref(),
        Some("57P01")
    );
    assert_eq!(
        last_error(&read_until_ready(&mut second).await).as_deref(),
        Some("57P01")
    );
    // Every backend that was opened is closed: a lease that is never dropped keeps it open.
    assert_eq!(h.opened(), 2);
    assert!(
        eventually(|| h.log(|l| l.closed) == 2).await,
        "closed {} of 2",
        h.log(|l| l.closed)
    );
}

#[tokio::test]
async fn ac25_shutdown_closes_the_backend_of_every_session_it_ends() {
    let h = Harness::new("shutdown_timeout_ms = 2000");
    let (mut first, _) = login(&h).await.ready();
    let (mut second, _) = login(&h).await.ready();
    tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
        .await
        .expect("shutdown returns");
    for tls in [&mut first, &mut second] {
        assert_eq!(
            last_error(&read_until_ready(tls).await).as_deref(),
            Some("57P01")
        );
    }
    assert_eq!(h.opened(), 2);
    assert!(
        eventually(|| h.log(|l| l.closed) == 2).await,
        "closed {} of 2",
        h.log(|l| l.closed)
    );
}

#[tokio::test]
async fn ac20_the_limit_of_a_principal_admits_exactly_its_count() {
    let h = Harness::with_config(config_with("", "session_max_count = 2"));
    let (_one, _) = login(&h).await.ready();
    let (_two, _) = login(&h).await.ready();
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "53300");
    assert_eq!(h.server.counters().refused(Refusal::PrincipalLimit), 1);
    assert_eq!(h.server.counters().refused(Refusal::SessionLimit), 0);
    assert_eq!(h.opened(), 2, "no backend for the refused login");
}

#[tokio::test]
async fn ac16_a_mechanism_other_than_scram_sha_256_gets_08p01_and_no_server_first_message() {
    for mechanism in [
        "SCRAM-SHA-256-PLUS",
        "PLAIN",
        "scram-sha-256",
        "SCRAM-SHA-1",
        "",
    ] {
        let h = Harness::new("");
        let mut tls = upgrade(h.raw()).await;
        tls.write_all(&encode_startup(&[
            ("user", PRINCIPAL.as_bytes()),
            ("database", b"orders"),
        ]))
        .await
        .expect("write");
        let sasl = read_msg(&mut tls, 4096).await.expect("sasl");
        assert_eq!(sasl.tag, b'R');
        let (_, first) = ClientExchange::start().expect("random");
        tls.write_all(&auth::encode_sasl_initial_response(
            mechanism,
            first.as_bytes(),
        ))
        .await
        .expect("write");
        let reply = read_msg(&mut tls, 1 << 20).await.expect("a reply");
        assert_eq!(reply.tag, b'E', "{mechanism:?}: no SASLContinue");
        assert_eq!(
            error_sqlstate(&reply.body),
            Some(*b"08P01"),
            "{mechanism:?}"
        );
        assert!(closed(&mut tls).await);
        assert_eq!(h.opened(), 0);
    }
}

#[tokio::test]
async fn ac25_shutdown_closes_connections_before_their_session_without_waiting_for_the_startup_deadline()
 {
    let h = Harness::new("startup_timeout_ms = 60000\nshutdown_timeout_ms = 5000");
    // One connection that sent nothing, one that stopped after the SASL request.
    let mut silent = h.raw();
    let mut tls = upgrade(h.raw()).await;
    tls.write_all(&encode_startup(&[
        ("user", PRINCIPAL.as_bytes()),
        ("database", b"orders"),
    ]))
    .await
    .expect("write");
    assert_eq!(read_msg(&mut tls, 4096).await.expect("sasl").tag, b'R');
    settle().await;
    let started = std::time::Instant::now();
    tokio::time::timeout(Duration::from_secs(2), h.server.shutdown())
        .await
        .expect("shutdown returns");
    let mut rest = Vec::new();
    let read = tokio::time::timeout(Duration::from_secs(2), silent.read_to_end(&mut rest)).await;
    assert!(read.is_ok(), "the silent connection is closed by shutdown");
    assert!(rest.is_empty(), "closed with no byte");
    assert!(closed(&mut tls).await);
    assert!(started.elapsed() < Duration::from_secs(3));
    assert_eq!(h.server.connections(), 0);
}

// Gap 2: the log events, read through a subscriber of the test.

type Events = Arc<Mutex<Vec<Vec<(String, String)>>>>;

/// Keeps every event as its fields, in the order the code gave them. Spans are not used by this slice.
struct Capture(Events);

struct Fields(Vec<(String, String)>);

impl tracing::field::Visit for Fields {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.push((field.name().to_string(), value.to_string()));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .push((field.name().to_string(), format!("{value:?}")));
    }
}

impl tracing::Subscriber for Capture {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }

    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}

    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}

    fn event(&self, event: &tracing::Event<'_>) {
        let mut fields = Fields(Vec::new());
        event.record(&mut fields);
        self.0.lock().expect("lock").push(fields.0);
    }

    fn enter(&self, _: &tracing::span::Id) {}

    fn exit(&self, _: &tracing::span::Id) {}
}

fn capture() -> (Events, tracing::subscriber::DefaultGuard) {
    let events: Events = Arc::default();
    let guard = tracing::subscriber::set_default(Capture(Arc::clone(&events)));
    (events, guard)
}

fn named(events: &Events, name: &str) -> Vec<Vec<(String, String)>> {
    events
        .lock()
        .expect("lock")
        .iter()
        .filter(|e| e.first().is_some_and(|(k, v)| k == "event" && v == name))
        .cloned()
        .collect()
}

fn keys(event: &[(String, String)]) -> Vec<&str> {
    event.iter().map(|(k, _)| k.as_str()).collect()
}

fn value<'a>(event: &'a [(String, String)], key: &str) -> &'a str {
    event
        .iter()
        .find(|(k, _)| k == key)
        .map_or("", |(_, v)| v.as_str())
}

/// What no event may hold (`SEC-11`): the names, the passwords, the host, the paths and the SQL of the harness.
const CANARIES: [&str; 9] = [
    PRINCIPAL,
    PRINCIPAL_PASSWORD,
    BACKEND_LOGIN,
    BACKEND_DATABASE,
    BACKEND_PASSWORD,
    "node.test",
    "/etc/pool",
    "select 1",
    "mallory",
];

#[tokio::test]
async fn ac24_session_events_carry_exactly_the_fields_of_the_spec_and_ids_in_place_of_names() {
    let (events, _guard) = capture();
    let h = Harness::new("");
    for _ in 0..3 {
        let (mut tls, _) = login(&h).await.ready();
        query(&mut tls, "select 1").await;
        tls.write_all(&encode_terminate()).await.expect("terminate");
        assert!(closed(&mut tls).await);
        assert!(eventually(|| h.server.sessions() == 0).await);
    }
    assert!(eventually(|| named(&events, "session.ended").len() == 3).await);
    let started = named(&events, "session.started");
    assert_eq!(started.len(), 3);
    for (n, event) in started.iter().enumerate() {
        assert_eq!(
            keys(event),
            [
                "event",
                "session",
                "principal_id",
                "identity_id",
                "cluster_id",
                "node_id",
                "lease_generation",
                "topology_epoch"
            ]
        );
        // The principal by its ID, never by the name the client sent.
        assert_eq!(value(event, "principal_id"), "orders-app");
        assert_eq!(value(event, "identity_id"), "orders-app");
        assert_eq!(value(event, "cluster_id"), "main");
        assert_eq!(value(event, "node_id"), "pg1");
        assert_eq!(value(event, "topology_epoch"), "1");
        // A new lease generation for each lease, rising by one.
        assert_eq!(value(event, "lease_generation"), (n + 1).to_string());
        // 32 hex characters: the start sequence (3) in the high half, the counter in the low one.
        assert_eq!(value(event, "session"), format!("{:016x}{:016x}", 3, n + 1));
    }
    let ended = named(&events, "session.ended");
    for (event, start) in ended.iter().zip(&started) {
        assert_eq!(
            keys(event),
            ["event", "session", "reason", "status", "duration_ms"]
        );
        assert_eq!(value(event, "session"), value(start, "session"));
        assert_eq!(value(event, "reason"), "client_terminate");
        assert_eq!(value(event, "status"), "idle");
        assert!(value(event, "duration_ms").parse::<u64>().is_ok());
    }
}

#[tokio::test]
async fn ac24_the_end_reason_and_status_name_how_a_session_ended() {
    let (events, _guard) = capture();
    let h = Harness::new("");
    // A client that leaves inside a transaction, one whose backend is lost with a query in flight, and a conflict.
    let (mut tls, _) = login(&h).await.ready();
    query(&mut tls, "begin").await;
    drop(tls);
    let (mut tls, _) = login(&h).await.ready();
    query(&mut tls, "commit, then the connection is cut").await;
    let (mut idle, _) = login(&h).await.ready();
    h.close(true);
    read_until_ready(&mut idle).await;
    assert!(eventually(|| named(&events, "session.ended").len() == 3).await);
    let mut seen: Vec<(String, String)> = named(&events, "session.ended")
        .iter()
        .map(|e| {
            (
                value(e, "reason").to_string(),
                value(e, "status").to_string(),
            )
        })
        .collect();
    seen.sort();
    assert_eq!(
        seen,
        [
            ("backend_lost".to_string(), "in_flight".to_string()),
            ("client_closed".to_string(), "in_transaction".to_string()),
            ("conflict".to_string(), "idle".to_string()),
        ]
    );
}

#[tokio::test]
async fn ac24_refusals_are_written_once_per_reason_with_a_count_and_no_name() {
    let (events, _guard) = capture();
    let h = Harness::new("");
    // Two authentication failures, one unmapped database, one closed route.
    for (user, database) in [(PRINCIPAL, "orders"), ("mallory", "orders")] {
        login_with(upgrade(h.raw()).await, user, "not the password", database)
            .await
            .refused();
    }
    login_with(
        upgrade(h.raw()).await,
        PRINCIPAL,
        PRINCIPAL_PASSWORD,
        BACKEND_DATABASE,
    )
    .await
    .refused();
    h.close(false);
    login(&h).await.refused();
    h.server.counters().flush();
    let refused = named(&events, "session.refused");
    let mut seen: Vec<(String, String)> = refused
        .iter()
        .map(|e| {
            assert_eq!(keys(e), ["event", "reason", "count"]);
            (
                value(e, "reason").to_string(),
                value(e, "count").to_string(),
            )
        })
        .collect();
    seen.sort();
    assert_eq!(
        seen,
        [
            ("authentication".to_string(), "2".to_string()),
            ("closed".to_string(), "1".to_string()),
            ("unmapped".to_string(), "1".to_string()),
        ]
    );
}

#[tokio::test]
async fn ac24_no_event_of_a_whole_run_holds_a_name_a_password_a_host_a_path_or_sql() {
    let (events, _guard) = capture();
    let h = Harness::new("session_max_count = 2");
    let (mut tls, _) = login(&h).await.ready();
    query(&mut tls, "select 1").await;
    login_with(
        upgrade(h.raw()).await,
        "mallory",
        PRINCIPAL_PASSWORD,
        "orders",
    )
    .await
    .refused();
    *h.node.hot_standby.lock().expect("lock") = "on";
    login(&h).await.refused();
    *h.node.hot_standby.lock().expect("lock") = "off";
    h.close(true);
    read_until_ready(&mut tls).await;
    h.server.counters().flush();
    assert!(eventually(|| !named(&events, "session.ended").is_empty()).await);
    let all = events.lock().expect("lock").clone();
    assert!(all.len() >= 4, "{all:?}");
    for event in &all {
        let event_name = value(event, "event");
        assert!(
            [
                "session.started",
                "session.ended",
                "session.refused",
                "backend.failed"
            ]
            .contains(&event_name),
            "an event outside the table: {event:?}"
        );
        for (key, text) in event {
            for canary in CANARIES {
                assert!(
                    !text.contains(canary),
                    "{key} of {event_name} holds {canary:?}"
                );
            }
            assert_ne!(
                text, "orders",
                "{key} of {event_name} holds the database name"
            );
        }
    }
    let failed = named(&events, "backend.failed");
    assert_eq!(failed.len(), 1);
    assert_eq!(keys(&failed[0]), ["event", "node_id", "class", "count"]);
    assert_eq!(value(&failed[0], "class"), "not_primary");
}

// ---- a node that stops in the middle of a message (review finding c1) ----

/// The longest one of these tests may take: a session that cannot be cut fails the test instead of hanging the run.
const TEST_LIMIT: Duration = Duration::from_secs(15);

/// How the session is cut while the node is silent.
#[derive(Clone, Copy)]
enum Cut {
    Conflict,
    Shutdown,
}

/// Sends `sql`, lets the node stop in the middle of a message, cuts the session, and checks the end:
/// the client gets the whole messages, then `08007` (a query is in flight and nothing partial was
/// forwarded), then the end; the session ends, the backend is closed, and shutdown returns.
async fn a_node_that_stops_in_the_middle(sql: &str, [whole, _]: [Vec<u8>; 2], cut: Cut) {
    let h = Harness::new("shutdown_timeout_ms = 200");
    let (mut tls, _) = login(&h).await.ready();
    tls.write_all(&encode_query(sql)).await.expect("write");
    settle().await;
    assert_eq!(h.server.sessions(), 1);
    match cut {
        Cut::Conflict => h.close(true),
        Cut::Shutdown => tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
            .await
            .expect("shutdown returns while the node is silent"),
    }
    let rest = read_rest(&mut tls).await;
    assert!(
        rest.starts_with(&whole),
        "the whole messages come first, unchanged"
    );
    let error = &rest[whole.len()..];
    assert_eq!(
        error.first(),
        Some(&b'E'),
        "then one error and nothing else"
    );
    assert!(
        error.windows(7).any(|w| w == b"C08007\0"),
        "the error is 08007"
    );
    assert!(
        eventually(|| h.server.sessions() == 0).await,
        "the session ends"
    );
    assert!(
        eventually(|| h.log(|l| l.closed) == 1).await,
        "the backend is closed"
    );
    assert_eq!(h.opened(), 1, "no second backend");
    if let Cut::Conflict = cut {
        tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
            .await
            .expect("shutdown returns");
    }
}

#[tokio::test]
async fn c1_a_positive_conflict_ends_a_session_whose_node_stopped_before_the_status_byte() {
    let run = a_node_that_stops_in_the_middle(
        "stop in the status byte",
        stop_in_the_status_byte(),
        Cut::Conflict,
    );
    tokio::time::timeout(TEST_LIMIT, run)
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_before_the_status_byte() {
    let run = a_node_that_stops_in_the_middle(
        "stop in the status byte",
        stop_in_the_status_byte(),
        Cut::Shutdown,
    );
    tokio::time::timeout(TEST_LIMIT, run)
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn c1_a_positive_conflict_ends_a_session_whose_node_stopped_in_a_header() {
    let run =
        a_node_that_stops_in_the_middle("stop in a header", stop_in_a_header(), Cut::Conflict);
    tokio::time::timeout(TEST_LIMIT, run)
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_in_a_header() {
    let run =
        a_node_that_stops_in_the_middle("stop in a header", stop_in_a_header(), Cut::Shutdown);
    tokio::time::timeout(TEST_LIMIT, run)
        .await
        .expect("within the test limit");
}

/// The node stops in the body of a `DataRow`. Part of the message reached the client, so the cut
/// sends no error: the client gets exactly the bytes of the node, then the end.
async fn a_node_that_stops_in_a_body(cut: Cut) {
    let h = Harness::new("shutdown_timeout_ms = 200");
    let (mut tls, _) = login(&h).await.ready();
    tls.write_all(&encode_query("half a row"))
        .await
        .expect("write");
    settle().await;
    match cut {
        Cut::Conflict => h.close(true),
        Cut::Shutdown => tokio::time::timeout(Duration::from_secs(5), h.server.shutdown())
            .await
            .expect("shutdown returns while the node is silent"),
    }
    assert_eq!(read_rest(&mut tls).await, half_answer());
    assert!(
        eventually(|| h.server.sessions() == 0).await,
        "the session ends"
    );
    assert!(
        eventually(|| h.log(|l| l.closed) == 1).await,
        "the backend is closed"
    );
}

#[tokio::test]
async fn c1_a_positive_conflict_ends_a_session_whose_node_stopped_in_a_body() {
    tokio::time::timeout(TEST_LIMIT, a_node_that_stops_in_a_body(Cut::Conflict))
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_in_a_body() {
    tokio::time::timeout(TEST_LIMIT, a_node_that_stops_in_a_body(Cut::Shutdown))
        .await
        .expect("within the test limit");
}

// ---- step 8 of the backend connection: the node names the writer of the route (0004 AC-29) ----

/// A login against a node that answers the question of step 8 with `identity`: refused with
/// `08006` before `AuthenticationOk`, the backend closed, no `Query` of the client at the node,
/// and `backend.failed` with the class `identity`.
async fn refused_at_step_8(h: &Harness, identity: Identity) {
    *h.node.identity.lock().expect("lock") = identity;
    let (events, _guard) = capture();
    let (tags, code) = login(h).await.refused();
    assert_eq!(code, "08006");
    assert!(!tags.contains(&b'Z'), "no ReadyForQuery");
    assert_eq!(h.opened(), 1, "one backend, no second try");
    assert_eq!(
        h.log(|l| l.identity_questions),
        1,
        "the question was asked once"
    );
    assert_eq!(
        h.log(|l| l.queries.len()),
        0,
        "no Query of the client reached the node"
    );
    assert!(
        eventually(|| h.log(|l| l.closed + l.terminated) == 1).await,
        "the backend is closed"
    );
    assert_eq!(h.server.sessions(), 0);
    assert_eq!(h.server.counters().refused(Refusal::BackendUnavailable), 1);
    h.server.counters().flush();
    let failed = named(&events, "backend.failed");
    assert_eq!(failed.len(), 1, "{failed:?}");
    assert_eq!(value(&failed[0], "class"), "identity");
}

#[tokio::test]
async fn ac29_the_answer_of_the_approved_writer_lets_the_session_open() {
    let h = Harness::new("");
    let (mut tls, _) = login(&h).await.ready();
    assert_eq!(h.log(|l| l.identity_questions), 1);
    assert_eq!(
        query(&mut tls, "select 1").await.last().map(|m| m.tag),
        Some(b'Z')
    );
    assert_eq!(h.log(|l| l.queries.clone()), ["select 1"]);
}

#[tokio::test]
async fn ac29_another_system_identifier_is_refused_with_08006() {
    refused_at_step_8(
        &Harness::new(""),
        Identity::row("7412345678901234568", "false", "1"),
    )
    .await;
}

#[tokio::test]
async fn ac29_another_timeline_is_refused_with_08006() {
    refused_at_step_8(
        &Harness::new(""),
        Identity::row(SYSTEM_IDENTIFIER, "false", "2"),
    )
    .await;
}

#[tokio::test]
async fn ac29_a_node_in_recovery_is_refused_with_08006() {
    refused_at_step_8(
        &Harness::new(""),
        Identity::row(SYSTEM_IDENTIFIER, "true", "1"),
    )
    .await;
}

#[tokio::test]
async fn ac29_an_error_is_refused_with_08006() {
    refused_at_step_8(&Harness::new(""), Identity::Error).await;
}

#[tokio::test]
async fn ac29_two_rows_are_refused_with_08006() {
    let Identity::Rows(mut rows) = Identity::writer() else {
        unreachable!()
    };
    rows.push(rows[0].clone());
    refused_at_step_8(&Harness::new(""), Identity::Rows(rows)).await;
}

#[tokio::test]
async fn ac29_a_null_is_refused_with_08006() {
    let rows = vec![vec![
        Some(SYSTEM_IDENTIFIER.to_owned()),
        Some("false".to_owned()),
        None,
    ]];
    refused_at_step_8(&Harness::new(""), Identity::Rows(rows)).await;
}

#[tokio::test]
async fn ac29_no_row_is_refused_with_08006() {
    refused_at_step_8(&Harness::new(""), Identity::Rows(Vec::new())).await;
}

#[tokio::test]
async fn ac29_a_server_built_without_the_question_refuses_every_login() {
    let h = Harness::without_identity_query();
    let (events, _guard) = capture();
    let (_, code) = login(&h).await.refused();
    assert_eq!(code, "08006");
    assert_eq!(h.log(|l| l.queries.len() + l.identity_questions), 0);
    h.server.counters().flush();
    assert_eq!(
        value(&named(&events, "backend.failed")[0], "class"),
        "identity"
    );
}

// ---- a node marked as lost ends its running sessions as backend loss does (0004 AC-30) ----

/// The limit of each of these tests, so a session that is not ended fails instead of hanging.
const LOST_LIMIT: Duration = Duration::from_secs(15);

#[tokio::test]
async fn ac30_a_query_in_flight_on_a_lost_node_gets_08007() {
    let run = async {
        let (events, _guard) = capture();
        let h = Harness::new("");
        let (mut tls, _) = login(&h).await.ready();
        tls.write_all(&encode_query("a statement that never ends"))
            .await
            .expect("write");
        settle().await;
        h.close_with(false, true);
        assert_eq!(
            last_error(&read_until_ready(&mut tls).await).as_deref(),
            Some("08007")
        );
        assert!(eventually(|| h.server.sessions() == 0).await);
        assert!(
            eventually(|| h.log(|l| l.closed) == 1).await,
            "the backend is closed"
        );
        assert_eq!(h.opened(), 1, "no second backend");
        assert!(eventually(|| !named(&events, "session.ended").is_empty()).await);
        assert_eq!(
            value(&named(&events, "session.ended")[0], "reason"),
            "node_lost"
        );
    };
    tokio::time::timeout(LOST_LIMIT, run)
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn ac30_an_idle_session_on_a_lost_node_gets_08006() {
    let run = async {
        // Every test that ends a session holds a subscriber, so the event callsites stay enabled for the one that reads them.
        let (_events, _guard) = capture();
        let h = Harness::new("");
        let (mut tls, _) = login(&h).await.ready();
        h.close_with(false, true);
        assert_eq!(
            last_error(&read_until_ready(&mut tls).await).as_deref(),
            Some("08006")
        );
        assert!(eventually(|| h.server.sessions() == 0).await);
        assert!(
            eventually(|| h.log(|l| l.closed + l.terminated) == 1).await,
            "the backend is closed"
        );
    };
    tokio::time::timeout(LOST_LIMIT, run)
        .await
        .expect("within the test limit");
}

#[tokio::test]
async fn ac30_silence_without_the_mark_ends_no_session() {
    let run = async {
        // Every test that ends a session holds a subscriber, so the event callsites stay enabled for the one that reads them.
        let (_events, _guard) = capture();
        let h = Harness::new("");
        let (mut tls, _) = login(&h).await.ready();
        h.close_with(false, false);
        settle().await;
        assert_eq!(h.server.sessions(), 1);
        assert_eq!(
            query(&mut tls, "select 1").await.last().map(|m| m.tag),
            Some(b'Z')
        );
        // A positive conflict wins over the mark: 57P01 for an idle session, not 08006.
        h.close_with(true, true);
        assert_eq!(
            last_error(&read_until_ready(&mut tls).await).as_deref(),
            Some("57P01")
        );
    };
    tokio::time::timeout(LOST_LIMIT, run)
        .await
        .expect("within the test limit");
}
