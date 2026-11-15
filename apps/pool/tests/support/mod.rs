//! Helpers of the fixture tests of Build plan step 11 (spec 0004, "The
//! integration check"): a client of POOL that logs in with the backend code
//! of `pool-core` and then writes raw messages, readers for the messages it
//! gets, and the relay that sits between POOL and PostgreSQL.
//!
//! Everything runs on a runtime of the test: the source guard keeps
//! threads of the standard library out of the binary and of its tests.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::time::Duration;

use pool_core::backend::{self, BackendConnection, BackendError, Credential, Login, Node, Purpose};
use pool_core::secret::{Password, TrustRoots};
use pool_core::tls::NodeTls;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Notify;
use tokio::time::Instant;

/// A multi thread runtime for the test.
pub fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("a runtime for the test")
}

/// One message from POOL: its tag and its body.
#[derive(Debug)]
pub struct Message {
    pub tag: u8,
    pub body: Vec<u8>,
}

impl Message {
    /// The SQLSTATE of an `ErrorResponse`, its field `C`.
    pub fn sqlstate(&self) -> Option<String> {
        if self.tag != b'E' {
            return None;
        }
        error_fields(&self.body)
            .into_iter()
            .find(|(field, _)| *field == b'C')
            .map(|(_, value)| value)
    }
}

fn error_fields(body: &[u8]) -> Vec<(u8, String)> {
    let mut fields = Vec::new();
    let mut rest = body;
    while let Some((&code, tail)) = rest.split_first() {
        if code == 0 {
            break;
        }
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        fields.push((code, String::from_utf8_lossy(&tail[..end]).into_owned()));
        rest = tail.get(end + 1..).unwrap_or_default();
    }
    fields
}

/// Reads one message: a tag, a length that counts itself, and the body.
pub async fn read_message<S: AsyncRead + Unpin>(stream: &mut S) -> io::Result<Message> {
    let mut header = [0u8; 5];
    stream.read_exact(&mut header).await?;
    let length = u32::from_be_bytes([header[1], header[2], header[3], header[4]]);
    let length = usize::try_from(length).unwrap_or(0).saturating_sub(4);
    let mut body = vec![0u8; length];
    stream.read_exact(&mut body).await?;
    Ok(Message {
        tag: header[0],
        body,
    })
}

/// Reads messages until an `ErrorResponse` or the end of the stream, and
/// returns the SQLSTATE of the error, or `None` when none came.
pub async fn next_error<S: AsyncRead + Unpin>(stream: &mut S) -> Option<String> {
    loop {
        match read_message(stream).await {
            Ok(message) if message.tag == b'E' => return message.sqlstate(),
            Ok(_) => {}
            Err(_) => return None,
        }
    }
}

/// The bytes of a `Query` message.
pub fn query(sql: &str) -> Vec<u8> {
    let length = u32::try_from(4 + sql.len() + 1).expect("small");
    let mut bytes = vec![b'Q'];
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(sql.as_bytes());
    bytes.push(0);
    bytes
}

/// Reads everything POOL sends until it closes, with a deadline. A reset
/// also ends the read: POOL closes without reading what it does not need,
/// so the bytes it sent before are kept.
pub async fn read_to_close(stream: &mut TcpStream, wait: Duration) -> Option<Vec<u8>> {
    let mut all = Vec::new();
    let read = async {
        let mut buffer = [0u8; 4096];
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) | Err(_) => return,
                Ok(n) => all.extend_from_slice(&buffer[..n]),
            }
        }
    };
    tokio::time::timeout(wait, read).await.ok().map(|()| all)
}

/// Logs in to POOL as a client, with TLS against `ca` and `server_name` and
/// SCRAM with `password`, and returns the connection after `ReadyForQuery`.
pub async fn login(
    host: &str,
    port: u16,
    ca: &[u8],
    server_name: &str,
    user: &str,
    database: &str,
    password: &str,
) -> Result<BackendConnection<TcpStream>, BackendError> {
    let roots = TrustRoots::parse(ca).expect("the test CA");
    let tls = NodeTls::new(&roots, server_name).expect("the server name");
    let node = Node::new(host, port, tls);
    let credential = Credential::new(Password::parse(password.as_bytes()).expect("password"));
    let login = Login::new(user, database, credential);
    backend::connect(
        &node,
        &login,
        Purpose::Probe,
        Instant::now() + Duration::from_secs(10),
    )
    .await
}

/// Writes all bytes to a stream.
pub async fn send<S: AsyncWrite + Unpin>(stream: &mut S, bytes: &[u8]) -> io::Result<()> {
    stream.write_all(bytes).await?;
    stream.flush().await
}

const PASS: u8 = 0;
const ARMED: u8 = 1;
const SENT: u8 = 2;
const CUT: u8 = 3;

/// A relay between POOL and PostgreSQL: it listens on a free port, passes
/// bytes both ways, and is the `host` and `port` of the node. TLS passes
/// through it unchanged. Connections are numbered from 1 in the order they
/// arrive. Once armed for one connection, it passes the next bytes from POOL
/// on to PostgreSQL, waits until PostgreSQL answered, drops that answer and
/// closes the connection to POOL.
pub struct Relay {
    pub port: u16,
    connections: Arc<AtomicUsize>,
    armed_for: Arc<AtomicUsize>,
    stage: Arc<AtomicU8>,
    cut: Arc<Notify>,
    refuse_tls_from: Arc<AtomicUsize>,
}

impl Relay {
    /// Starts the relay on the runtime toward `target_port` on 127.0.0.1.
    pub fn start(runtime: &tokio::runtime::Runtime, target_port: u16) -> Relay {
        let listener = runtime
            .block_on(TcpListener::bind("127.0.0.1:0"))
            .expect("bind the relay");
        let port = listener.local_addr().expect("address").port();
        let relay = Relay {
            port,
            connections: Arc::new(AtomicUsize::new(0)),
            armed_for: Arc::new(AtomicUsize::new(0)),
            stage: Arc::new(AtomicU8::new(PASS)),
            cut: Arc::new(Notify::new()),
            refuse_tls_from: Arc::new(AtomicUsize::new(usize::MAX)),
        };
        let shared = relay.shared();
        runtime.spawn(async move {
            while let Ok((pool_side, _)) = listener.accept().await {
                let index = shared.connections.fetch_add(1, Ordering::SeqCst) + 1;
                let shared = shared.clone();
                tokio::spawn(async move {
                    if index >= shared.refuse_tls_from.load(Ordering::SeqCst) {
                        refuse_tls(pool_side).await;
                        return;
                    }
                    if let Ok(pg_side) = TcpStream::connect(("127.0.0.1", target_port)).await {
                        pass(index, pool_side, pg_side, shared).await;
                    }
                });
            }
        });
        relay
    }

    fn shared(&self) -> Shared {
        Shared {
            armed_for: Arc::clone(&self.armed_for),
            stage: Arc::clone(&self.stage),
            cut: Arc::clone(&self.cut),
            connections: Arc::clone(&self.connections),
            refuse_tls_from: Arc::clone(&self.refuse_tls_from),
        }
    }

    /// The number of connections it accepted so far.
    pub fn connections(&self) -> usize {
        self.connections.load(Ordering::SeqCst)
    }

    /// Arms the cut for connection `index`.
    pub fn arm(&self, index: usize) {
        self.stage.store(ARMED, Ordering::SeqCst);
        self.armed_for.store(index, Ordering::SeqCst);
    }

    /// Whether the cut happened.
    pub fn was_cut(&self) -> bool {
        self.stage.load(Ordering::SeqCst) == CUT
    }

    /// From connection `index` on, answers the `SSLRequest` of POOL with `N`
    /// and closes, so TLS never starts.
    pub fn refuse_tls_from(&self, index: usize) {
        self.refuse_tls_from.store(index, Ordering::SeqCst);
    }
}

#[derive(Clone)]
struct Shared {
    armed_for: Arc<AtomicUsize>,
    stage: Arc<AtomicU8>,
    cut: Arc<Notify>,
    connections: Arc<AtomicUsize>,
    refuse_tls_from: Arc<AtomicUsize>,
}

/// Reads the `SSLRequest` of POOL and answers `N`. A startup packet that
/// followed would be read and dropped; POOL must send none.
async fn refuse_tls(mut pool_side: TcpStream) {
    let mut request = [0u8; 8];
    if pool_side.read_exact(&mut request).await.is_ok() {
        let _ = pool_side.write_all(b"N").await;
        let mut rest = Vec::new();
        let _ =
            tokio::time::timeout(Duration::from_secs(2), pool_side.read_to_end(&mut rest)).await;
    }
}

async fn pass(index: usize, mut pool_side: TcpStream, mut pg_side: TcpStream, shared: Shared) {
    let mut up = vec![0u8; 65536];
    let mut down = vec![0u8; 65536];
    loop {
        // Whether this connection is armed is asked after each read, so an
        // arm that happens while the relay waits counts for the next bytes.
        let armed = || shared.armed_for.load(Ordering::SeqCst) == index;
        tokio::select! {
            read = pool_side.read(&mut up) => {
                let Ok(n) = read else { return };
                if n == 0 { return; }
                if armed() && shared.stage.load(Ordering::SeqCst) == ARMED {
                    shared.stage.store(SENT, Ordering::SeqCst);
                }
                if pg_side.write_all(&up[..n]).await.is_err() { return; }
            }
            read = pg_side.read(&mut down) => {
                let Ok(n) = read else { return };
                if n == 0 { return; }
                if armed() && shared.stage.load(Ordering::SeqCst) == SENT {
                    // PostgreSQL answered: the answer is dropped and both
                    // connections are closed.
                    shared.stage.store(CUT, Ordering::SeqCst);
                    shared.cut.notify_waiters();
                    return;
                }
                if pool_side.write_all(&down[..n]).await.is_err() { return; }
            }
        }
    }
}
