//! The connection to a node, for a session and for the probe (spec 0004,
//! "Backend connection").
//!
//! One attempt: TCP, `SSLRequest` and exactly one byte, the TLS handshake
//! with the node's own roots and name, a `StartupMessage` for 3.0, SCRAM as
//! a client, then the messages up to `ReadyForQuery`. There is no second
//! try and no other node (`0001 SC-6`), and no fallback to plain text
//! (`SEC-1`). The probe gets one more call on an open connection:
//! [`BackendConnection::query_one_row`].

use std::fmt;
use std::sync::Arc;

use pool_protocol::auth::{
    self, Authentication, BackendKeyData, ParameterStatus, TransactionStatus,
};
use pool_protocol::message::{self, Message};
use pool_protocol::sqlstate::error_sqlstate;
use pool_protocol::startup::{encode_ssl_request, encode_startup};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use tokio::net::{TcpSocket, TcpStream};
use tokio::time::Instant;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;

use crate::REDACTED;
use crate::scram::{ClientExchange, ScramKeys, derive_keys};
use crate::secret::Password;
use crate::session::TLS_SEND_BUFFER_BYTES;
use crate::tls::NodeTls;
use crate::wire::{WireError, read_message, write_all};

/// The largest message body POOL reads from a node from authentication up to `ReadyForQuery`.
pub const BACKEND_STARTUP_MESSAGE_MAX_BYTES: u32 = 16384;
/// The most bytes of answer, message bodies together, that the one row query of the probe reads.
pub const PROBE_ANSWER_MAX_BYTES: usize = 4096;
/// The most parameter status messages kept from a node.
pub const PARAMETER_STATUS_MAX_COUNT: usize = 64;
/// The most bytes of parameter names and values kept from a node, together.
pub const PARAMETER_STATUS_MAX_BYTES: usize = 16384;

/// Why one attempt to open a backend failed. It carries no text from the node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendError {
    /// No TCP connection to any address of the node.
    Connect,
    /// The deadline of the attempt passed.
    Timeout,
    /// The node answered `SSLRequest` with something other than `S`.
    TlsRefused,
    /// The TLS handshake failed: the chain does not lead to the CA, the name does not match, or the peer broke TLS.
    Tls,
    /// The node asked for a method other than SASL with `SCRAM-SHA-256`, or for none.
    AuthenticationMethod,
    /// The SCRAM exchange failed: `e=`, or a wrong server signature.
    Authentication,
    /// The node sent an `ErrorResponse`, with its SQLSTATE when it had a well formed one.
    Server {
        /// The five character SQLSTATE of the node, for the log event.
        sqlstate: Option<[u8; 5]>,
    },
    /// The node broke the protocol or a limit of this module.
    Protocol,
    /// `in_hot_standby` was not reported as `off`, for a session.
    NotPrimary,
    /// The first `ReadyForQuery` did not report `I`.
    NotIdle,
    /// The stream ended or failed after the TLS handshake.
    Closed,
    /// No random bytes for the nonce.
    Random,
}

impl BackendError {
    /// A constant name of the failure, for the log event.
    pub fn kind(self) -> &'static str {
        match self {
            Self::Connect => "connect",
            Self::Timeout => "timeout",
            Self::TlsRefused => "tls_refused",
            Self::Tls => "tls",
            Self::AuthenticationMethod => "authentication_method",
            Self::Authentication => "authentication",
            Self::Server { .. } => "server_error",
            Self::Protocol => "protocol",
            Self::NotPrimary => "not_primary",
            Self::NotIdle => "not_idle",
            Self::Closed => "closed",
            Self::Random => "random",
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "backend attempt failed: {}", self.kind())
    }
}

impl std::error::Error for BackendError {}

impl From<WireError> for BackendError {
    fn from(error: WireError) -> Self {
        match error {
            WireError::Closed => Self::Closed,
            WireError::Frame => Self::Protocol,
        }
    }
}

struct CachedKeys {
    salt: Vec<u8>,
    iterations: u32,
    keys: ScramKeys,
}

/// The password of a backend login or a monitor login, with the SCRAM keys derived from it.
///
/// The two keys are kept together with the salt and the count they were
/// made for, so a later login with the same salt and count derives nothing
/// (decision 43 of the spec rationale). `Debug` prints `[REDACTED]`.
pub struct Credential {
    password: Password,
    /// One entry per credential. The lock is held across the derivation, so at most one runs at a time.
    cache: tokio::sync::Mutex<Option<CachedKeys>>,
    derivations: std::sync::atomic::AtomicU32,
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(REDACTED)
    }
}

impl Credential {
    /// A credential with no derived keys yet.
    pub fn new(password: Password) -> Arc<Self> {
        Arc::new(Self {
            password,
            cache: tokio::sync::Mutex::new(None),
            derivations: std::sync::atomic::AtomicU32::new(0),
        })
    }

    /// The keys for this salt and count: from the cache, or derived with PBKDF2 inside `spawn_blocking`.
    ///
    /// At most one derivation per credential runs at a time: the others wait
    /// for the lock and then find its result in the cache. When the login
    /// that derives is cancelled (its deadline passed), the lock is released
    /// and the next waiter derives.
    async fn keys(
        self: &Arc<Self>,
        salt: &[u8],
        iterations: u32,
    ) -> Result<ScramKeys, BackendError> {
        let mut cache = self.cache.lock().await;
        if let Some(cached) = cache
            .as_ref()
            .filter(|c| c.salt == salt && c.iterations == iterations)
        {
            return Ok(cached.keys.clone());
        }
        // The parser of the server-first message accepts 1 to 600000 only.
        let count = std::num::NonZeroU32::new(iterations).ok_or(BackendError::Protocol)?;
        let credential = Arc::clone(self);
        let owned_salt = salt.to_vec();
        let keys = tokio::task::spawn_blocking(move || {
            credential
                .derivations
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            derive_keys(&credential.password, &owned_salt, count)
        })
        .await
        .map_err(|_| BackendError::Closed)?;
        *cache = Some(CachedKeys {
            salt: salt.to_vec(),
            iterations,
            keys: keys.clone(),
        });
        Ok(keys)
    }

    /// How many times keys were derived for this credential in this run.
    pub fn derivations(&self) -> u32 {
        self.derivations.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// Where a node is and how its certificate is checked.
#[derive(Clone)]
pub struct Node {
    host: String,
    port: u16,
    tls: NodeTls,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Host and port are kept out of default telemetry.
        f.debug_struct("Node")
            .field("tls", &self.tls)
            .finish_non_exhaustive()
    }
}

impl Node {
    /// A node at `host` and `port`, from `node[].host`, `node[].port` and its TLS setup.
    pub fn new(host: &str, port: u16, tls: NodeTls) -> Self {
        Self {
            host: host.to_string(),
            port,
            tls,
        }
    }

    /// The TLS setup toward this node.
    pub fn tls(&self) -> &NodeTls {
        &self.tls
    }
}

/// Who POOL logs in as: the backend login or the monitor login, its database and its credential.
#[derive(Clone)]
pub struct Login {
    user: String,
    database: String,
    forwarded: Vec<(&'static str, Vec<u8>)>,
    credential: Arc<Credential>,
}

impl fmt::Debug for Login {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Login")
            .field("forwarded", &self.forwarded.len())
            .finish_non_exhaustive()
    }
}

impl Login {
    /// A login with no forwarded parameter.
    pub fn new(user: &str, database: &str, credential: Arc<Credential>) -> Self {
        Self {
            user: user.to_string(),
            database: database.to_string(),
            forwarded: Vec::new(),
            credential,
        }
    }

    /// Adds a parameter that the client sent and POOL forwards unchanged: `application_name` or `client_encoding`.
    pub fn forward(mut self, name: &'static str, value: &[u8]) -> Self {
        self.forwarded.push((name, value.to_vec()));
        self
    }
}

/// What the connection is for. A session also needs `in_hot_standby` to be `off`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Purpose {
    /// A client session: step 7 of the backend connection applies.
    Session,
    /// The probe of the control loop.
    Probe,
}

/// An open backend after its first `ReadyForQuery` with status `I`.
pub struct BackendConnection<S> {
    pub(crate) stream: TlsStream<S>,
    pub(crate) parameters: Vec<ParameterStatus>,
    pub(crate) key: Option<BackendKeyData>,
    pub(crate) status: TransactionStatus,
}

impl<S> fmt::Debug for BackendConnection<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BackendConnection")
            .field("parameters", &self.parameters.len())
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl<S: AsyncRead + AsyncWrite + Unpin> BackendConnection<S> {
    /// The parameter status messages of the node, in the order it sent them.
    pub fn parameters(&self) -> &[ParameterStatus] {
        &self.parameters
    }

    /// The real cancellation key of the node, when it sent one. It is never sent to a client.
    pub fn backend_key(&self) -> Option<&BackendKeyData> {
        self.key.as_ref()
    }

    /// The transaction status of the last `ReadyForQuery`.
    pub fn status(&self) -> TransactionStatus {
        self.status
    }

    /// The TLS stream, for the relay of a session.
    pub fn stream_mut(&mut self) -> &mut TlsStream<S> {
        &mut self.stream
    }

    /// Sends one constant query and returns the text columns of its one row.
    ///
    /// `None` when the query returned no row or more than one. A column is
    /// `None` when it is NULL. An `ErrorResponse` is [`BackendError::Server`].
    /// An answer of more than 4096 bytes of message bodies is
    /// [`BackendError::Protocol`], and the connection must then be closed.
    /// For the probe only.
    pub async fn query_one_row(
        &mut self,
        sql: &str,
    ) -> Result<Option<Vec<Option<String>>>, BackendError> {
        write_all(&mut self.stream, &message::encode_query(sql)).await?;
        let mut row = None;
        let mut rows = 0usize;
        let mut error = None;
        let mut answer = 0usize;
        loop {
            // The answer stops at 4096 bytes: a longer one is never read on.
            let left = PROBE_ANSWER_MAX_BYTES.saturating_sub(answer);
            let limit = u32::try_from(left).unwrap_or(0);
            let Message { tag, body } = read_message(&mut self.stream, limit).await?;
            answer += body.len();
            match tag {
                b'T' | b'C' | b'N' | b'S' | b'I' => {}
                b'D' => {
                    rows += 1;
                    if rows == 1 {
                        row = Some(data_row(&body)?);
                    }
                }
                b'E' => error = Some(error_sqlstate(&body)),
                b'Z' => {
                    self.status =
                        auth::parse_ready_for_query(&body).map_err(|_| BackendError::Protocol)?;
                    if let Some(sqlstate) = error {
                        return Err(BackendError::Server { sqlstate });
                    }
                    return Ok(if rows == 1 { row } else { None });
                }
                _ => return Err(BackendError::Protocol),
            }
        }
    }
}

/// Reads the text columns of a `DataRow`.
fn data_row(body: &[u8]) -> Result<Vec<Option<String>>, BackendError> {
    let count = body.get(..2).ok_or(BackendError::Protocol)?;
    let count = u16::from_be_bytes([count[0], count[1]]);
    let mut rest = &body[2..];
    let mut columns = Vec::new();
    for _ in 0..count {
        let len = rest.get(..4).ok_or(BackendError::Protocol)?;
        let len = i32::from_be_bytes([len[0], len[1], len[2], len[3]]);
        rest = &rest[4..];
        if len == -1 {
            columns.push(None);
            continue;
        }
        let len = usize::try_from(len).map_err(|_| BackendError::Protocol)?;
        let value = rest.get(..len).ok_or(BackendError::Protocol)?;
        columns.push(Some(
            std::str::from_utf8(value)
                .map_err(|_| BackendError::Protocol)?
                .to_string(),
        ));
        rest = &rest[len..];
    }
    if !rest.is_empty() {
        return Err(BackendError::Protocol);
    }
    Ok(columns)
}

/// How a session opens its backend. The daemon uses [`TcpConnector`]; the
/// session tests use a connector over in memory streams.
pub trait Connector: Send + Sync + 'static {
    /// The stream under TLS.
    type Stream: AsyncRead + AsyncWrite + Unpin + Send + 'static;

    /// One attempt to open a backend to `node` as `login` before `deadline`.
    fn open(
        &self,
        node: &Node,
        login: &Login,
        purpose: Purpose,
        deadline: Instant,
    ) -> impl std::future::Future<Output = Result<BackendConnection<Self::Stream>, BackendError>> + Send;
}

/// The connector of the daemon: TCP to the node, then [`establish`].
#[derive(Debug, Clone, Copy, Default)]
pub struct TcpConnector;

impl Connector for TcpConnector {
    type Stream = TcpStream;

    fn open(
        &self,
        node: &Node,
        login: &Login,
        purpose: Purpose,
        deadline: Instant,
    ) -> impl std::future::Future<Output = Result<BackendConnection<TcpStream>, BackendError>> + Send
    {
        connect(node, login, purpose, deadline)
    }
}

/// Opens a backend to `node` as `login`, the whole attempt before `deadline`.
pub async fn connect(
    node: &Node,
    login: &Login,
    purpose: Purpose,
    deadline: Instant,
) -> Result<BackendConnection<TcpStream>, BackendError> {
    tokio::time::timeout_at(deadline, async {
        let tcp = connect_tcp(&node.host, node.port).await?;
        let _ = tcp.set_nodelay(true);
        handshake(tcp, &node.tls, login, purpose).await
    })
    .await
    .map_err(|_| BackendError::Timeout)?
}

/// Step 1: resolves the name at each attempt and tries the addresses in the order they come back,
/// each on a socket with TCP keepalive on before it connects (spec 0004, "A peer that vanished").
async fn connect_tcp(host: &str, port: u16) -> Result<TcpStream, BackendError> {
    let addresses = tokio::net::lookup_host((host, port))
        .await
        .map_err(|_| BackendError::Connect)?;
    for address in addresses {
        let Ok(socket) = node_socket(address) else {
            continue;
        };
        if let Ok(tcp) = socket.connect(address).await {
            return Ok(tcp);
        }
    }
    Err(BackendError::Connect)
}

/// A socket for one address of a node, with TCP keepalive on, not connected yet.
pub(crate) fn node_socket(address: std::net::SocketAddr) -> std::io::Result<TcpSocket> {
    let socket = if address.is_ipv4() {
        TcpSocket::new_v4()?
    } else {
        TcpSocket::new_v6()?
    };
    socket.set_keepalive(true)?;
    Ok(socket)
}

/// Steps 2 to 7 of the backend connection over an open stream, before `deadline`.
pub async fn establish<S: AsyncRead + AsyncWrite + Unpin>(
    stream: S,
    tls: &NodeTls,
    login: &Login,
    purpose: Purpose,
    deadline: Instant,
) -> Result<BackendConnection<S>, BackendError> {
    tokio::time::timeout_at(deadline, handshake(stream, tls, login, purpose))
        .await
        .map_err(|_| BackendError::Timeout)?
}

async fn handshake<S: AsyncRead + AsyncWrite + Unpin>(
    mut stream: S,
    tls: &NodeTls,
    login: &Login,
    purpose: Purpose,
) -> Result<BackendConnection<S>, BackendError> {
    // Step 2: SSLRequest, then exactly one byte. Anything but S ends the attempt.
    write_all(&mut stream, &encode_ssl_request())
        .await
        .map_err(|_| BackendError::TlsRefused)?;
    let mut answer = [0u8; 1];
    stream
        .read_exact(&mut answer)
        .await
        .map_err(|_| BackendError::TlsRefused)?;
    if answer[0] != b'S' {
        return Err(BackendError::TlsRefused);
    }
    // Step 3: TLS with the node's own roots and name.
    let mut stream = TlsConnector::from(Arc::clone(&tls.config))
        .connect(tls.name.clone(), stream)
        .await
        .map_err(|_| BackendError::Tls)?;
    // The memory budget: at most 64 KiB queued for sending on each TLS connection.
    stream
        .get_mut()
        .1
        .set_buffer_limit(Some(TLS_SEND_BUFFER_BYTES));
    // Step 4: StartupMessage for 3.0.
    let mut params: Vec<(&str, &[u8])> = vec![
        ("user", login.user.as_bytes()),
        ("database", login.database.as_bytes()),
    ];
    for (name, value) in &login.forwarded {
        params.push((name, value));
    }
    write_all(&mut stream, &encode_startup(&params)).await?;
    // Step 5: SCRAM-SHA-256 as a client.
    authenticate(&mut stream, &login.credential).await?;
    // Step 6: up to ReadyForQuery.
    let mut parameters: Vec<ParameterStatus> = Vec::new();
    let mut parameter_bytes = 0usize;
    let mut key = None;
    let status = loop {
        let Message { tag, body } =
            read_message(&mut stream, BACKEND_STARTUP_MESSAGE_MAX_BYTES).await?;
        match tag {
            b'S' => {
                let parameter =
                    auth::parse_parameter_status(&body).map_err(|_| BackendError::Protocol)?;
                parameter_bytes += parameter.name.len() + parameter.value.len();
                if parameters.len() == PARAMETER_STATUS_MAX_COUNT
                    || parameter_bytes > PARAMETER_STATUS_MAX_BYTES
                {
                    return Err(BackendError::Protocol);
                }
                parameters.push(parameter);
            }
            b'K' if key.is_none() => {
                key =
                    Some(auth::parse_backend_key_data(&body).map_err(|_| BackendError::Protocol)?);
            }
            b'N' => {}
            b'E' => {
                return Err(BackendError::Server {
                    sqlstate: error_sqlstate(&body),
                });
            }
            b'Z' => {
                break auth::parse_ready_for_query(&body).map_err(|_| BackendError::Protocol)?;
            }
            _ => return Err(BackendError::Protocol),
        }
    };
    if status != TransactionStatus::Idle {
        return Err(BackendError::NotIdle);
    }
    // Step 7: a session needs a node that reports it is not in recovery.
    // The last report of the parameter before ReadyForQuery decides.
    let in_hot_standby = parameters
        .iter()
        .rev()
        .find(|p| p.name == b"in_hot_standby")
        .map(|p| p.value.as_slice());
    if purpose == Purpose::Session && in_hot_standby != Some(b"off".as_slice()) {
        return Err(BackendError::NotPrimary);
    }
    Ok(BackendConnection {
        stream,
        parameters,
        key,
        status,
    })
}

/// Reads one `Authentication` message; an `ErrorResponse` ends the attempt with its SQLSTATE.
async fn read_authentication<S: AsyncRead + Unpin>(
    stream: &mut S,
) -> Result<Authentication, BackendError> {
    let Message { tag, body } = read_message(stream, BACKEND_STARTUP_MESSAGE_MAX_BYTES).await?;
    match tag {
        b'R' => auth::parse_authentication(&body).map_err(|_| BackendError::Protocol),
        b'E' => Err(BackendError::Server {
            sqlstate: error_sqlstate(&body),
        }),
        _ => Err(BackendError::Protocol),
    }
}

async fn authenticate<S: AsyncRead + AsyncWrite + Unpin>(
    stream: &mut S,
    credential: &Arc<Credential>,
) -> Result<(), BackendError> {
    match read_authentication(stream).await? {
        Authentication::Sasl(mechanisms) if mechanisms.iter().any(|m| m == auth::SCRAM_SHA_256) => {
        }
        _ => return Err(BackendError::AuthenticationMethod),
    }
    let (exchange, first) = ClientExchange::start().map_err(|_| BackendError::Random)?;
    write_all(
        stream,
        &auth::encode_sasl_initial_response(auth::SCRAM_SHA_256, first.as_bytes()),
    )
    .await?;
    let Authentication::SaslContinue(data) = read_authentication(stream).await? else {
        return Err(BackendError::Protocol);
    };
    let server_first = exchange
        .server_first(&data)
        .map_err(|_| BackendError::Protocol)?;
    let keys = credential
        .keys(&server_first.salt, server_first.iterations)
        .await?;
    let (fin, expected) = exchange.client_final(&server_first, &keys);
    write_all(stream, &auth::encode_sasl_response(fin.as_bytes())).await?;
    let Authentication::SaslFinal(data) = read_authentication(stream).await? else {
        return Err(BackendError::Protocol);
    };
    expected
        .check(&data)
        .map_err(|_| BackendError::Authentication)?;
    match read_authentication(stream).await? {
        Authentication::Ok => Ok(()),
        _ => Err(BackendError::Protocol),
    }
}

#[cfg(test)]
mod tests;
