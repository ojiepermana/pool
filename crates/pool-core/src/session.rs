//! The session: one task per frontend, steps 1 to 12 of spec 0004 "Session lifecycle".
//!
//! A [`Server`] holds what every session needs: the configuration, the TLS
//! acceptor, the verifiers, the credentials, the nodes, the route view, the
//! permits and the counters. [`Server::serve`] accepts TCP connections;
//! [`Server::accept_stream`] runs one connection over any stream, which the
//! tests use with in memory streams.
//!
//! The order of the steps is the contract. No byte of application SQL
//! reaches a node before step 10, and step 10 is reached only through the
//! two route checks of steps 5 and 9. A permit is given back when the value
//! that holds it is dropped, so every way out of the task returns it.

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use pool_config::{
    ClusterId, ConfigurationGeneration, CredentialGeneration, IdentityId, IdentityRoute, NodeId,
    PrincipalId, ValidatedConfig,
};
use pool_protocol::auth::{self, AUTH_MESSAGE_MAX_BYTES, TransactionStatus};
use pool_protocol::message::{
    BackendAction, FrontendAction, HEADER_LEN, HeaderReader, encode_message, encode_terminate,
};
use pool_protocol::sqlstate::PoolError;
use pool_protocol::startup::{FirstPacket, FirstPacketReader, StartupRequest};
use rustls::ServerConfig;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{Notify, OwnedSemaphorePermit, Semaphore, watch};
use tokio::time::Instant;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::server::TlsStream;

use crate::backend::{BackendError, Connector, Credential, Login, Node, Purpose};
use crate::records::{
    Lease, LeaseGeneration, PoolKey, SessionId, StartupProfile, VerifiedPrincipal,
};
use crate::report::{Counters, EndReason, Refusal};
use crate::route::{Route, RouteView};
use crate::scram::{MockKey, ScramFault, ServerCredential, ServerExchange};
use crate::secret::Verifier;
use crate::tls::TlsPolicy;
use crate::wire::{WireError, read_message};

/// The size of the relay buffer of each direction of a session.
pub const RELAY_BUFFER_BYTES: usize = 65536;
/// The most bytes each TLS connection queues for sending.
pub const TLS_SEND_BUFFER_BYTES: usize = 65536;
/// The longest the end of a session waits to close a client that does not read.
pub const CLOSE_TIMEOUT: Duration = Duration::from_secs(1);
/// The longest shutdown waits after `limits.shutdown_timeout_ms` before it drops the connection
/// tasks that are left. Every wait of a session after that deadline is cut at once or bounded by
/// [`CLOSE_TIMEOUT`], so only a defect reaches this limit; it keeps shutdown from waiting for ever.
pub const END_TIMEOUT: Duration = Duration::from_secs(2);
/// How long the listener waits after a failed accept.
pub const ACCEPT_RETRY: Duration = Duration::from_millis(100);

/// The phase of the server: running, draining after a signal, or ending every session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Accepting and serving.
    Running,
    /// Shutdown step 1 to 3: no accept, connections before step 10 closed, idle sessions ended.
    Draining,
    /// Shutdown step 4: the deadline passed; every session is ended.
    Ending,
}

/// Why a [`Server`] could not be built from its parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerError {
    /// A principal of the configuration has no verifier, or the configuration has no principal.
    MissingVerifier,
    /// An identity entry has no credential.
    MissingCredential,
    /// A node of the configuration has no TLS setup.
    MissingNode,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MissingVerifier => "a principal has no verifier",
            Self::MissingCredential => "an identity entry has no credential",
            Self::MissingNode => "a node has no tls setup",
        })
    }
}

impl std::error::Error for ServerError {}

/// What the daemon passes to build a [`Server`]: everything read and checked before it listens.
pub struct ServerParts<C> {
    /// The validated configuration; its serving view was accepted.
    pub config: Arc<ValidatedConfig>,
    /// The TLS server configuration of the listener.
    pub tls: Arc<ServerConfig>,
    /// The verifier of each principal.
    pub verifiers: Vec<(PrincipalId, Verifier)>,
    /// The credential of each identity entry.
    pub credentials: Vec<(IdentityId, Arc<Credential>)>,
    /// Each node with its TLS setup.
    pub nodes: Vec<(NodeId, Node)>,
    /// The key of the made up verifiers, made once at start.
    pub mock_key: MockKey,
    /// How backends are opened.
    pub connector: C,
    /// The route view, published by the control loop.
    pub routes: watch::Receiver<RouteView>,
    /// The origin of every `fresh_until` of the view.
    pub origin: Instant,
    /// The start sequence that the journal returned for this run.
    pub start_sequence: u64,
}

impl<C> fmt::Debug for ServerParts<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerParts")
            .field("start_sequence", &self.start_sequence)
            .finish_non_exhaustive()
    }
}

/// Counts the connection tasks that are alive, so shutdown can wait for them.
#[derive(Debug, Default)]
struct Tracker {
    live: AtomicUsize,
    sessions: AtomicUsize,
    changed: Notify,
}

struct LiveGuard(Arc<Tracker>);

impl Drop for LiveGuard {
    fn drop(&mut self) {
        self.0.live.fetch_sub(1, Ordering::SeqCst);
        self.0.changed.notify_waiters();
    }
}

struct SessionGuard(Arc<Tracker>);

impl Drop for SessionGuard {
    fn drop(&mut self) {
        self.0.sessions.fetch_sub(1, Ordering::SeqCst);
        self.0.changed.notify_waiters();
    }
}

/// Everything a session needs, shared by all sessions of the daemon.
pub struct Server<C: Connector> {
    config: Arc<ValidatedConfig>,
    acceptor: TlsAcceptor,
    verifiers: HashMap<PrincipalId, Verifier>,
    mock_key: MockKey,
    mock_iterations: u32,
    credentials: HashMap<IdentityId, Arc<Credential>>,
    nodes: HashMap<NodeId, Node>,
    connector: C,
    routes: watch::Receiver<RouteView>,
    origin: Instant,
    start_sequence: u64,
    counter: AtomicU64,
    next_lease: Mutex<Option<LeaseGeneration>>,
    handshakes: Arc<Semaphore>,
    sessions: Arc<Semaphore>,
    principals: HashMap<PrincipalId, Arc<Semaphore>>,
    node_permits: HashMap<NodeId, Arc<Semaphore>>,
    waiters: Arc<Semaphore>,
    counters: Arc<Counters>,
    phase: watch::Sender<Phase>,
    /// Set when [`END_TIMEOUT`] has passed after the deadline: every connection task is dropped.
    abort: watch::Sender<bool>,
    tracker: Arc<Tracker>,
    startup_timeout: Duration,
    acquire_timeout: Duration,
    shutdown_timeout: Duration,
    startup_packet_max: u32,
    frontend_message_max: u32,
    /// The question of the probe, asked again on the connection of each session (step 8).
    identity_query: Option<&'static str>,
}

impl<C: Connector> fmt::Debug for Server<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server")
            .field("start_sequence", &self.start_sequence)
            .field("phase", &*self.phase.borrow())
            .finish_non_exhaustive()
    }
}

/// The error that ends a connection before its session started: the error to send, if any, and the counter.
struct Refused {
    send: Option<PoolError>,
    reason: Refusal,
}

impl Refused {
    fn with(error: PoolError, reason: Refusal) -> Self {
        Self {
            send: Some(error),
            reason,
        }
    }

    fn silent(reason: Refusal) -> Self {
        Self { send: None, reason }
    }

    fn from_protocol(error: PoolError) -> Self {
        let reason = match error {
            PoolError::TlsRequired => Refusal::Plaintext,
            PoolError::FeatureNotSupported => Refusal::Unsupported,
            PoolError::MessageTooLarge => Refusal::MessageLimit,
            _ => Refusal::Protocol,
        };
        Self::with(error, reason)
    }
}

/// A frontend after step 2: TLS, an accepted startup packet and a verified principal.
struct Frontend<S> {
    stream: TlsStream<S>,
    request: StartupRequest,
    principal: VerifiedPrincipal,
}

impl<C: Connector> Server<C> {
    /// Builds the server with the question that step 8 of the backend connection asks every node
    /// on the connection of a session: the probe query of `pool-control`, which the daemon hands in
    /// because `pool-core` cannot name `pool-control` (spec 0004, decision 85, `0004 AC-29`).
    pub fn with_identity_query(
        parts: ServerParts<C>,
        identity_query: &'static str,
    ) -> Result<Arc<Self>, ServerError> {
        Self::build(parts, Some(identity_query))
    }

    /// Builds the server without a question for step 8. It fails closed: every login is refused
    /// at step 8 with the class `identity`. The daemon uses [`Self::with_identity_query`].
    pub fn new(parts: ServerParts<C>) -> Result<Arc<Self>, ServerError> {
        Self::build(parts, None)
    }

    /// Every principal needs a verifier, every identity entry a credential and every node a TLS setup.
    fn build(
        parts: ServerParts<C>,
        identity_query: Option<&'static str>,
    ) -> Result<Arc<Self>, ServerError> {
        let config = parts.config;
        let verifiers: HashMap<PrincipalId, Verifier> = parts.verifiers.into_iter().collect();
        let first = config
            .principals()
            .first()
            .ok_or(ServerError::MissingVerifier)?;
        let mock_iterations = verifiers
            .get(first.id())
            .ok_or(ServerError::MissingVerifier)?
            .iterations();
        if config
            .principals()
            .iter()
            .any(|p| !verifiers.contains_key(p.id()))
        {
            return Err(ServerError::MissingVerifier);
        }
        let credentials: HashMap<IdentityId, Arc<Credential>> =
            parts.credentials.into_iter().collect();
        if config
            .identities()
            .iter()
            .any(|i| !credentials.contains_key(i.id()))
        {
            return Err(ServerError::MissingCredential);
        }
        let nodes: HashMap<NodeId, Node> = parts.nodes.into_iter().collect();
        if config.nodes().iter().any(|n| !nodes.contains_key(n.id())) {
            return Err(ServerError::MissingNode);
        }
        let limits = config.limits();
        let principals = config
            .principals()
            .iter()
            .map(|p| {
                (
                    p.id().clone(),
                    Arc::new(Semaphore::new(p.session_max_count() as usize)),
                )
            })
            .collect();
        let node_permits = config
            .nodes()
            .iter()
            .map(|n| {
                (
                    n.id().clone(),
                    Arc::new(Semaphore::new(limits.node_backend_max_count() as usize)),
                )
            })
            .collect();
        let (phase, _) = watch::channel(Phase::Running);
        let (abort, _) = watch::channel(false);
        Ok(Arc::new(Self {
            acceptor: TlsAcceptor::from(parts.tls),
            verifiers,
            mock_key: parts.mock_key,
            mock_iterations,
            credentials,
            nodes,
            connector: parts.connector,
            routes: parts.routes,
            origin: parts.origin,
            start_sequence: parts.start_sequence,
            counter: AtomicU64::new(0),
            next_lease: Mutex::new(Some(LeaseGeneration::FIRST)),
            handshakes: Arc::new(Semaphore::new(limits.handshake_max_count() as usize)),
            sessions: Arc::new(Semaphore::new(limits.session_max_count() as usize)),
            principals,
            node_permits,
            waiters: Arc::new(Semaphore::new(limits.backend_wait_max_count() as usize)),
            counters: Arc::new(Counters::default()),
            phase,
            abort,
            tracker: Arc::new(Tracker::default()),
            startup_timeout: limits.startup_timeout(),
            acquire_timeout: limits.backend_acquire_timeout(),
            shutdown_timeout: limits.shutdown_timeout(),
            startup_packet_max: limits.startup_packet_max_bytes(),
            frontend_message_max: limits.frontend_message_max_bytes(),
            identity_query,
            config,
        }))
    }

    /// The counters behind `session.refused` and `backend.failed`. The daemon flushes them once a second.
    pub fn counters(&self) -> &Arc<Counters> {
        &self.counters
    }

    /// The number of sessions that reached step 10 and have not ended.
    pub fn sessions(&self) -> usize {
        self.tracker.sessions.load(Ordering::SeqCst)
    }

    /// The number of connection tasks that are alive, in any step.
    pub fn connections(&self) -> usize {
        self.tracker.live.load(Ordering::SeqCst)
    }

    /// The current phase.
    pub fn phase(&self) -> Phase {
        *self.phase.borrow()
    }

    /// Accepts TCP connections until shutdown starts. An accept that fails is counted and followed by a wait of 100 ms.
    pub async fn serve(self: &Arc<Self>, listener: TcpListener) {
        let mut phase = self.phase.subscribe();
        loop {
            if *phase.borrow_and_update() != Phase::Running {
                return;
            }
            tokio::select! {
                accepted = listener.accept() => match accepted {
                    Ok((stream, _)) => {
                        let _ = stream.set_nodelay(true);
                        self.accept_stream(stream);
                    }
                    Err(_) => {
                        self.counters.refuse(Refusal::AcceptFailed);
                        tokio::time::sleep(ACCEPT_RETRY).await;
                    }
                },
                _ = phase.changed() => {}
            }
        }
    }

    /// Step 1 for one accepted stream: takes a handshake permit and starts the connection task.
    ///
    /// Returns `None`, and drops the stream at once with no byte, when no permit
    /// is free or shutdown has started.
    pub fn accept_stream<S>(self: &Arc<Self>, stream: S) -> Option<tokio::task::JoinHandle<()>>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        if self.phase() != Phase::Running {
            return None;
        }
        let Ok(permit) = Arc::clone(&self.handshakes).try_acquire_owned() else {
            self.counters.refuse(Refusal::HandshakeLimit);
            return None;
        };
        self.tracker.live.fetch_add(1, Ordering::SeqCst);
        let guard = LiveGuard(Arc::clone(&self.tracker));
        let server = Arc::clone(self);
        let mut abort = self.abort.subscribe();
        Some(tokio::spawn(async move {
            let _guard = guard;
            // Dropping the connection future drops the lease and closes the backend.
            tokio::select! {
                biased;
                () = server.connection(stream, permit) => {}
                _ = abort.wait_for(|abort| *abort) => {}
            }
        }))
    }

    /// Shutdown steps 1 to 4: stop accepting, close connections before step 10, end idle
    /// sessions, end the others when they become idle, and every one when
    /// `limits.shutdown_timeout_ms` has passed. Returns when every connection task ended, and
    /// drops the tasks that are left when [`END_TIMEOUT`] has passed after the deadline.
    pub async fn shutdown(&self) {
        self.phase.send_replace(Phase::Draining);
        let deadline = Instant::now() + self.shutdown_timeout;
        if tokio::time::timeout_at(deadline, self.wait_idle())
            .await
            .is_err()
        {
            self.end_now().await;
        }
    }

    /// Shutdown from step 4 on, as if `limits.shutdown_timeout_ms` had passed already: for a
    /// daemon whose own task ended (spec 0004, decision 86). No accept after it; every session is
    /// ended at once (`57P01` at a message boundary with no query in flight, `08007` with one);
    /// returns when every connection task ended, and drops the tasks that are left when
    /// [`END_TIMEOUT`] has passed.
    pub async fn shutdown_now(&self) {
        self.phase.send_replace(Phase::Draining);
        self.end_now().await;
    }

    /// Step 4: every session is ended; the tasks left after [`END_TIMEOUT`] are dropped.
    async fn end_now(&self) {
        self.phase.send_replace(Phase::Ending);
        if tokio::time::timeout(END_TIMEOUT, self.wait_idle())
            .await
            .is_err()
        {
            self.abort.send_replace(true);
            self.wait_idle().await;
        }
    }

    async fn wait_idle(&self) {
        loop {
            let notified = self.tracker.changed.notified();
            if self.connections() == 0 {
                return;
            }
            notified.await;
        }
    }

    fn now(&self) -> Duration {
        self.origin.elapsed()
    }

    async fn connection<S>(self: Arc<Self>, stream: S, handshake: OwnedSemaphorePermit)
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let mut phase = self.phase.subscribe();
        // Steps 1 to 9 end when shutdown starts.
        let admitted = tokio::select! {
            admitted = self.admit(stream, handshake) => admitted,
            _ = phase.wait_for(|p| *p != Phase::Running) => return,
        };
        if let Some((frontend, lease, cluster, session_permits)) = admitted {
            self.run_session(frontend, lease, cluster, session_permits)
                .await;
        }
    }

    /// Steps 2 to 10 up to the lease. Each refusal is sent (when it has an error), counted and ends the connection.
    async fn admit<S>(
        &self,
        stream: S,
        handshake: OwnedSemaphorePermit,
    ) -> Option<(Frontend<S>, Lease<C::Stream>, ClusterId, SessionPermits)>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        // Step 2, within the startup deadline from the accept.
        let mut frontend =
            match tokio::time::timeout(self.startup_timeout, self.startup(stream)).await {
                Ok(Ok(frontend)) => frontend,
                Ok(Err(refused)) => {
                    self.counters.refuse(refused.reason);
                    return None;
                }
                Err(_) => {
                    // At the deadline the connection is closed without a message.
                    self.counters.refuse(Refusal::StartupTimeout);
                    return None;
                }
            };
        match self.after_authentication(&mut frontend, handshake).await {
            Ok((lease, cluster, permits)) => Some((frontend, lease, cluster, permits)),
            Err(refused) => {
                self.counters.refuse(refused.reason);
                if let Some(error) = refused.send {
                    send_error(&mut frontend.stream, error).await;
                }
                None
            }
        }
    }

    /// Step 2: the first packets, TLS, the startup packet and SCRAM.
    async fn startup<S>(&self, mut stream: S) -> Result<Frontend<S>, Refused>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        // One reader for the connection, before and after TLS. It decides on the
        // eight header bytes and reads a body only for a CancelRequest and, over
        // TLS, for a StartupMessage.
        let mut reader = FirstPacketReader::new(self.startup_packet_max);
        loop {
            match read_first(&mut stream, &mut reader).await? {
                Ok(FirstPacket::GssEncRequest) => write_plain(&mut stream, b"N").await?,
                Ok(FirstPacket::SslRequest) => {
                    write_plain(&mut stream, b"S").await?;
                    break;
                }
                Ok(FirstPacket::CancelRequest) => {
                    return Err(Refused::silent(Refusal::CancelIgnored));
                }
                Ok(FirstPacket::Startup(_)) => return Err(Refused::silent(Refusal::Protocol)),
                Err(error) => {
                    send_error(&mut stream, error).await;
                    return Err(Refused::from_protocol(error).without_send());
                }
            }
        }
        let mut tls = self
            .acceptor
            .accept(stream)
            .await
            .map_err(|_| Refused::silent(Refusal::Tls))?;
        tls.get_mut()
            .1
            .set_buffer_limit(Some(TLS_SEND_BUFFER_BYTES));
        let request = match read_first(&mut tls, &mut reader).await? {
            Ok(FirstPacket::Startup(request)) => request,
            Ok(FirstPacket::CancelRequest) => return Err(Refused::silent(Refusal::CancelIgnored)),
            Ok(_) => return Err(Refused::silent(Refusal::Protocol)),
            Err(error) => {
                send_error(&mut tls, error).await;
                return Err(Refused::from_protocol(error).without_send());
            }
        };
        if let Some(negotiation) = request.negotiation() {
            write_tls(&mut tls, &negotiation).await?;
        }
        let principal = self.authenticate(&mut tls, &request).await?;
        Ok(Frontend {
            stream: tls,
            request,
            principal,
        })
    }

    /// Steps 1 to 6 of "Frontend authentication".
    async fn authenticate<S>(
        &self,
        tls: &mut TlsStream<S>,
        request: &StartupRequest,
    ) -> Result<VerifiedPrincipal, Refused>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let name = std::str::from_utf8(&request.user).ok();
        let principal = name.and_then(|n| self.config.resolve_principal(n));
        let verifier = principal.and_then(|p| self.verifiers.get(p.id()));
        let credential = match verifier {
            Some(verifier) => ServerCredential::Known(verifier),
            None => ServerCredential::unknown(&self.mock_key, &request.user, self.mock_iterations),
        };
        let violation = || Refused::with(PoolError::ProtocolViolation, Refusal::Protocol);
        write_tls(
            tls,
            &auth::encode_authentication_sasl(&[auth::SCRAM_SHA_256]),
        )
        .await?;
        let initial = read_sasl(tls).await?;
        let initial = auth::parse_sasl_initial_response(&initial).map_err(|_| violation())?;
        let data = match (initial.mechanism.as_str(), initial.data) {
            (auth::SCRAM_SHA_256, Some(data)) => data,
            _ => return Err(send_now(tls, violation()).await),
        };
        let (exchange, server_first) = match ServerExchange::start(credential, &data) {
            Ok(started) => started,
            Err(ScramFault::Random) => return Err(Refused::silent(Refusal::RandomFailed)),
            Err(_) => return Err(send_now(tls, violation()).await),
        };
        write_tls(
            tls,
            &auth::encode_authentication_sasl_continue(server_first.as_bytes()),
        )
        .await?;
        let fin = read_sasl(tls).await?;
        match exchange.finish(&fin) {
            Ok(server_final) => {
                write_tls(
                    tls,
                    &auth::encode_authentication_sasl_final(server_final.as_bytes()),
                )
                .await?;
            }
            Err(ScramFault::Failed) => {
                let refused =
                    Refused::with(PoolError::AuthenticationFailed, Refusal::Authentication);
                return Err(send_now(tls, refused).await);
            }
            Err(_) => return Err(send_now(tls, violation()).await),
        }
        // A match is possible only with a real verifier, so both are known here.
        match (principal, name) {
            (Some(principal), Some(name)) => Ok(VerifiedPrincipal::new(
                principal.id().clone(),
                name.to_string(),
            )),
            _ => Err(send_now(
                tls,
                Refused::with(PoolError::AuthenticationFailed, Refusal::Authentication),
            )
            .await),
        }
    }

    /// Steps 3 to 10 up to the lease.
    async fn after_authentication<S>(
        &self,
        frontend: &mut Frontend<S>,
        handshake: OwnedSemaphorePermit,
    ) -> Result<(Lease<C::Stream>, ClusterId, SessionPermits), Refused>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        let too_many = |reason| Refused::with(PoolError::TooManyConnections, reason);
        let closed = || Refused::with(PoolError::NotAccepting, Refusal::Closed);
        // Step 3.
        let session = Arc::clone(&self.sessions)
            .try_acquire_owned()
            .map_err(|_| too_many(Refusal::SessionLimit))?;
        let principal_permits = self
            .principals
            .get(frontend.principal.id())
            .ok_or_else(|| too_many(Refusal::PrincipalLimit))?;
        let principal = Arc::clone(principal_permits)
            .try_acquire_owned()
            .map_err(|_| too_many(Refusal::PrincipalLimit))?;
        drop(handshake);
        // Step 4.
        let database = std::str::from_utf8(&frontend.request.database).ok();
        let route = database
            .and_then(|db| self.config.resolve_identity(frontend.principal.name(), db))
            .ok_or_else(|| Refused::with(PoolError::NoIdentityEntry, Refusal::Unmapped))?;
        // Step 5.
        let cluster = route.cluster().clone();
        let step5 = Instant::now();
        let admitted = self
            .routes
            .borrow()
            .fresh_route(&cluster, self.now())
            .ok_or_else(closed)?;
        // Step 6.
        let key = pool_key(route, &admitted);
        // Step 7: a backend permit of the node, waiting as one of the waiters until the deadline.
        let deadline = step5 + self.acquire_timeout;
        let node_permits = self.node_permits.get(&admitted.node).ok_or_else(closed)?;
        let backend_permit = match Arc::clone(node_permits).try_acquire_owned() {
            Ok(permit) => permit,
            Err(_) => {
                let _waiter = Arc::clone(&self.waiters)
                    .try_acquire_owned()
                    .map_err(|_| too_many(Refusal::WaitLimit))?;
                match tokio::time::timeout_at(deadline, Arc::clone(node_permits).acquire_owned())
                    .await
                {
                    Ok(Ok(permit)) => permit,
                    _ => return Err(too_many(Refusal::AcquireTimeout)),
                }
            }
        };
        // Step 8.
        let node = self.nodes.get(&admitted.node).ok_or_else(closed)?;
        let credential = self.credentials.get(route.id()).ok_or_else(closed)?;
        let mut login = Login::new(
            route.backend_login(),
            route.backend_database(),
            Arc::clone(credential),
        );
        for forwarded in &frontend.request.forwarded {
            login = login.forward(forwarded.name, &forwarded.value);
        }
        let opened = match self
            .connector
            .open(node, &login, Purpose::Session, deadline)
            .await
        {
            // Step 8 of the backend connection: the node must name the writer of the route.
            Ok(mut backend) => match self.identity_query {
                Some(query) => match tokio::time::timeout_at(
                    deadline,
                    backend.verify_identity(query, admitted.system_identifier, admitted.timeline),
                )
                .await
                {
                    Ok(Ok(())) => Ok(backend),
                    Ok(Err(error)) => Err(error),
                    Err(_) => Err(BackendError::Timeout),
                },
                None => Err(BackendError::Identity),
            },
            Err(error) => Err(error),
        };
        let backend = match opened {
            Ok(backend) => backend,
            // No random bytes for the nonce: the login ends without a message.
            Err(BackendError::Random) => return Err(Refused::silent(Refusal::RandomFailed)),
            Err(error) => {
                self.counters.backend_failed(
                    &admitted.node,
                    backend_class(error),
                    sqlstate_of(error),
                );
                return Err(Refused::with(
                    PoolError::BackendNotEstablished,
                    Refusal::BackendUnavailable,
                ));
            }
        };
        // Step 9: the route again, the same node and generations, still fresh, the same pool key.
        let again = self.routes.borrow().fresh_route(&cluster, self.now());
        let still = again.filter(|r| {
            r.node == admitted.node
                && r.topology_epoch == admitted.topology_epoch
                && r.authority_revision == admitted.authority_revision
                && pool_key(route, r) == key
                && current_configuration_generation() == ConfigurationGeneration::FIRST
        });
        if still.is_none() {
            drop(backend);
            return Err(closed());
        }
        // Step 10: the lease with a new generation.
        let generation = {
            let mut next = self
                .next_lease
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            let generation = next.ok_or_else(|| too_many(Refusal::SessionLimit))?;
            *next = generation.checked_next();
            generation
        };
        let lease = Lease::new(backend, backend_permit, generation, key, route.id().clone());
        Ok((
            lease,
            cluster,
            SessionPermits {
                _session: session,
                _principal: principal,
            },
        ))
    }

    /// Steps 10 to 12: greet the client, relay, end.
    async fn run_session<S>(
        &self,
        frontend: Frontend<S>,
        mut lease: Lease<C::Stream>,
        cluster: ClusterId,
        permits: SessionPermits,
    ) where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let Frontend {
            mut stream,
            principal,
            ..
        } = frontend;
        let counter = self.counter.fetch_add(1, Ordering::SeqCst) + 1;
        let id = SessionId::new(self.start_sequence, counter);
        let started = Instant::now();
        let mut greeting = auth::encode_authentication_ok();
        for p in &lease.backend.parameters {
            let mut body = p.name.clone();
            body.push(0);
            body.extend_from_slice(&p.value);
            body.push(0);
            greeting.extend_from_slice(&encode_message(b'S', &body));
        }
        greeting.extend_from_slice(&auth::encode_ready_for_query(lease.backend.status));
        let mut routes = self.routes.clone();
        let mut phase = self.phase.subscribe();
        // A client that does not read its greeting cannot hold the session past a cut: the
        // greeting may be partly written, so the cut sends nothing.
        match guarded(
            write_tls(&mut stream, &greeting),
            &mut routes,
            &mut phase,
            &cluster,
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(_)) | Err(_) => return,
        }
        self.tracker.sessions.fetch_add(1, Ordering::SeqCst);
        let _session = SessionGuard(Arc::clone(&self.tracker));
        tracing::info!(
            event = "session.started",
            session = %id,
            principal_id = principal.id().as_str(),
            identity_id = lease.identity().as_str(),
            cluster_id = cluster.as_str(),
            node_id = lease.key().node.as_str(),
            lease_generation = lease.generation().get(),
            topology_epoch = lease.key().topology_epoch.get(),
        );
        let status = lease.backend.status;
        let mut relay = Relay {
            client: &mut stream,
            backend: &mut lease.backend.stream,
            routes,
            phase,
            cluster: &cluster,
            frontend_max: self.frontend_message_max,
            buffer: vec![0u8; RELAY_BUFFER_BYTES],
            in_flight: false,
            status,
        };
        let reason = relay.run().await;
        let label = if relay.in_flight {
            "in_flight"
        } else {
            match relay.status {
                TransactionStatus::Idle => "idle",
                TransactionStatus::InTransaction => "in_transaction",
                TransactionStatus::Failed => "failed",
            }
        };
        drop(relay);
        // Step 12: close the backend first, then the client, then give every permit back.
        // A client that does not read cannot hold the end of its session: its close is bounded.
        drop(lease);
        let _ = tokio::time::timeout(CLOSE_TIMEOUT, stream.shutdown()).await;
        drop(permits);
        tracing::info!(
            event = "session.ended",
            session = %id,
            reason = reason.name(),
            status = label,
            duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        );
    }
}

impl Refused {
    fn without_send(mut self) -> Self {
        self.send = None;
        self
    }
}

/// The session permit and the permit of the principal, held for the whole session.
struct SessionPermits {
    _session: OwnedSemaphorePermit,
    _principal: OwnedSemaphorePermit,
}

/// This slice has one configuration generation for the whole run.
fn current_configuration_generation() -> ConfigurationGeneration {
    ConfigurationGeneration::FIRST
}

fn pool_key(route: &IdentityRoute, admitted: &Route) -> PoolKey {
    PoolKey {
        cluster: route.cluster().clone(),
        node: admitted.node.clone(),
        database: route.backend_database().to_string(),
        login: route.backend_login().to_string(),
        credential_generation: CredentialGeneration::FIRST,
        topology_epoch: admitted.topology_epoch,
        tls_policy: TlsPolicy::VerifyFull,
        startup_profile: StartupProfile::DedicatedV1,
    }
}

/// The `class` of `backend.failed`.
fn backend_class(error: BackendError) -> &'static str {
    match error {
        BackendError::Connect => "connect",
        BackendError::Tls | BackendError::TlsRefused => "tls",
        BackendError::AuthenticationMethod => "auth_method",
        BackendError::Authentication | BackendError::Server { .. } => "login",
        BackendError::NotPrimary => "not_primary",
        BackendError::Identity => "identity",
        BackendError::Timeout => "timeout",
        _ => "protocol",
    }
}

fn sqlstate_of(error: BackendError) -> Option<[u8; 5]> {
    match error {
        BackendError::Server { sqlstate } => sqlstate,
        _ => None,
    }
}

async fn write_plain<S: AsyncWrite + Unpin>(stream: &mut S, bytes: &[u8]) -> Result<(), Refused> {
    stream
        .write_all(bytes)
        .await
        .map_err(|_| Refused::silent(Refusal::Protocol))?;
    stream
        .flush()
        .await
        .map_err(|_| Refused::silent(Refusal::Protocol))
}

async fn write_tls<S: AsyncWrite + Unpin>(stream: &mut S, bytes: &[u8]) -> Result<(), Refused> {
    write_plain(stream, bytes).await
}

/// Sends one generated error and closes the stream. Errors on the way are ignored: the connection ends anyway.
async fn send_error<S: AsyncWrite + Unpin>(stream: &mut S, error: PoolError) {
    let _ = stream.write_all(&error.encode()).await;
    let _ = stream.flush().await;
    let _ = stream.shutdown().await;
}

async fn send_now<S: AsyncWrite + Unpin>(stream: &mut S, refused: Refused) -> Refused {
    if let Some(error) = refused.send {
        send_error(stream, error).await;
    }
    refused.without_send()
}

/// Reads exactly what the first packet reader asks for, until it decides. A closed stream ends the connection silently.
async fn read_first<S: AsyncRead + Unpin>(
    stream: &mut S,
    reader: &mut FirstPacketReader,
) -> Result<Result<FirstPacket, PoolError>, Refused> {
    let mut buffer = [0u8; 4096];
    loop {
        let want = reader.wanted().clamp(1, buffer.len());
        stream
            .read_exact(&mut buffer[..want])
            .await
            .map_err(|_| Refused::silent(Refusal::Protocol))?;
        if let (_, Some(result)) = reader.feed(&buffer[..want]) {
            return Ok(result);
        }
    }
}

/// Reads one SASL message (tag `p`) of at most 1024 bytes of body. Anything else is `08P01`.
async fn read_sasl<S: AsyncRead + AsyncWrite + Unpin>(tls: &mut S) -> Result<Vec<u8>, Refused> {
    let violation = Refused::with(PoolError::ProtocolViolation, Refusal::Protocol);
    match read_message(tls, AUTH_MESSAGE_MAX_BYTES).await {
        Ok(message) if message.tag == b'p' => Ok(message.body),
        Ok(_) | Err(WireError::Frame) => Err(send_now(tls, violation).await),
        Err(WireError::Closed) => Err(Refused::silent(Refusal::Protocol)),
    }
}

/// What woke the relay.
enum Wake {
    Client(std::io::Result<usize>),
    Backend(std::io::Result<usize>),
    Changed,
}

/// The relay of step 11: one task, one buffer per direction, the rules of "Messages after login".
struct Relay<'a, F, B> {
    client: &'a mut F,
    backend: &'a mut B,
    routes: watch::Receiver<RouteView>,
    phase: watch::Receiver<Phase>,
    cluster: &'a ClusterId,
    frontend_max: u32,
    buffer: Vec<u8>,
    in_flight: bool,
    status: TransactionStatus,
}

/// How the relay ends, at a message boundary or not.
enum Stop {
    /// End now; at a message boundary send the error that fits the state.
    End(EndReason),
}

impl<F, B> Relay<'_, F, B>
where
    F: AsyncRead + AsyncWrite + Unpin,
    B: AsyncRead + AsyncWrite + Unpin,
{
    /// Whether the session must end now, at a message boundary.
    fn stop_now(&self) -> Option<EndReason> {
        if let Some(reason) = route_cut(&self.routes.borrow(), self.cluster) {
            return Some(reason);
        }
        match *self.phase.borrow() {
            Phase::Ending => Some(EndReason::Shutdown),
            Phase::Draining if !self.in_flight && self.status == TransactionStatus::Idle => {
                Some(EndReason::Shutdown)
            }
            _ => None,
        }
    }

    /// The error for a cut at a message boundary. A lost node ends the session as backend loss
    /// does (`08007` or `08006`); a conflict and shutdown end it with [`Self::ending_error`].
    fn cut_error(&self, reason: EndReason) -> PoolError {
        match reason {
            EndReason::NodeLost => self.loss_error(),
            _ => self.ending_error(),
        }
    }

    /// The error for an end at a message boundary: `08007` with a query in flight, `57P01` without.
    fn ending_error(&self) -> PoolError {
        if self.in_flight {
            PoolError::OutcomeUnknown
        } else {
            PoolError::EndedByProxy
        }
    }

    async fn run(&mut self) -> EndReason {
        let mut client_header = HeaderReader::new();
        let mut backend_header = HeaderReader::new();
        let mut client_bytes = [0u8; HEADER_LEN];
        let mut backend_bytes = [0u8; HEADER_LEN];
        let mut routes_alive = true;
        let mut phase_alive = true;
        loop {
            if let Some(reason) = self.stop_now() {
                self.send(self.cut_error(reason)).await;
                return reason;
            }
            let in_flight = self.in_flight;
            let client_want = client_header.wanted();
            let backend_want = backend_header.wanted();
            let wake = tokio::select! {
                read = self.client.read(&mut client_bytes[..client_want]), if !in_flight => Wake::Client(read),
                read = self.backend.read(&mut backend_bytes[..backend_want]) => Wake::Backend(read),
                changed = self.routes.changed(), if routes_alive => {
                    routes_alive = changed.is_ok();
                    Wake::Changed
                }
                changed = self.phase.changed(), if phase_alive => {
                    phase_alive = changed.is_ok();
                    Wake::Changed
                }
            };
            match wake {
                Wake::Changed => {}
                Wake::Client(Ok(0) | Err(_)) => {
                    // The client went away with no query in flight: Terminate first, then close.
                    let _ = self.write_backend(&encode_terminate()).await;
                    return EndReason::ClientClosed;
                }
                Wake::Client(Ok(n)) => match client_header.feed(&client_bytes[..n]) {
                    (_, None) => {}
                    (_, Some(Ok(header))) => {
                        if let Err(Stop::End(reason)) = self.frontend_message(raw(header)).await {
                            return reason;
                        }
                    }
                    (_, Some(Err(_))) => {
                        // A length below 4.
                        self.send(PoolError::ProtocolViolation).await;
                        let _ = self.write_backend(&encode_terminate()).await;
                        return EndReason::Protocol;
                    }
                },
                Wake::Backend(Ok(0) | Err(_)) => {
                    self.send(self.loss_error()).await;
                    return EndReason::BackendLost;
                }
                Wake::Backend(Ok(n)) => match backend_header.feed(&backend_bytes[..n]) {
                    (_, None) => {}
                    (_, Some(Ok(header))) => {
                        if let Err(Stop::End(reason)) = self.backend_message(raw(header)).await {
                            return reason;
                        }
                    }
                    (_, Some(Err(_))) => {
                        self.send(self.loss_error()).await;
                        return EndReason::BackendLost;
                    }
                },
            }
        }
    }

    /// The error at a message boundary after backend loss.
    fn loss_error(&self) -> PoolError {
        if self.in_flight {
            PoolError::OutcomeUnknown
        } else {
            PoolError::BackendLost
        }
    }

    async fn frontend_message(&mut self, header: [u8; HEADER_LEN]) -> Result<(), Stop> {
        match pool_protocol::message::classify_frontend(header, self.frontend_max) {
            FrontendAction::Query { body_len } => {
                // A query is in flight from the first byte written toward the backend.
                self.in_flight = true;
                match self.write_backend(&header).await {
                    Ok(()) => {}
                    Err(None) => {
                        self.send(self.loss_error()).await;
                        return Err(Stop::End(EndReason::BackendLost));
                    }
                    Err(Some(reason)) => {
                        self.send(self.cut_error(reason)).await;
                        return Err(Stop::End(reason));
                    }
                }
                match self.copy_to_backend(body_len).await {
                    Copied::Done => {}
                    Copied::SourceGone => return Err(Stop::End(EndReason::ClientClosed)),
                    Copied::DestinationGone => {
                        self.send(self.loss_error()).await;
                        return Err(Stop::End(EndReason::BackendLost));
                    }
                    Copied::Cut(reason) => {
                        self.send(self.cut_error(reason)).await;
                        return Err(Stop::End(reason));
                    }
                }
                match self.write_backend(&[]).await {
                    Ok(()) => Ok(()),
                    Err(None) => {
                        self.send(self.loss_error()).await;
                        Err(Stop::End(EndReason::BackendLost))
                    }
                    Err(Some(reason)) => {
                        self.send(self.cut_error(reason)).await;
                        Err(Stop::End(reason))
                    }
                }
            }
            FrontendAction::Terminate => {
                let _ = self.write_backend(&header).await;
                Err(Stop::End(EndReason::ClientTerminate))
            }
            FrontendAction::Refuse(error) => {
                self.send(error).await;
                let _ = self.write_backend(&encode_terminate()).await;
                Err(Stop::End(match error {
                    PoolError::FeatureNotSupported => EndReason::Unsupported,
                    PoolError::MessageTooLarge => EndReason::MessageLimit,
                    _ => EndReason::Protocol,
                }))
            }
        }
    }

    async fn backend_message(&mut self, header: [u8; HEADER_LEN]) -> Result<(), Stop> {
        match pool_protocol::message::classify_backend(header) {
            BackendAction::Forward { body_len } => {
                // A cut while the header waits leaves a partial message: close without a message.
                self.write_client(&header, false).await?;
                // From here until the body is written the session is not at a message boundary.
                match self.copy_to_client(body_len).await {
                    Copied::Done => {}
                    Copied::SourceGone => return Err(Stop::End(EndReason::BackendLost)),
                    Copied::DestinationGone => return Err(Stop::End(EndReason::ClientClosed)),
                    Copied::Cut(reason) => return Err(Stop::End(reason)),
                }
                self.write_client(&[], true).await
            }
            BackendAction::ReadyForQuery => {
                let mut status = [0u8; 1];
                // The header is not forwarded yet, so a cut here is at a message boundary.
                let read = self.backend.read_exact(&mut status);
                match guarded(read, &mut self.routes, &mut self.phase, self.cluster).await {
                    Ok(Ok(_)) => {}
                    Ok(Err(_)) => {
                        self.send(self.loss_error()).await;
                        return Err(Stop::End(EndReason::BackendLost));
                    }
                    Err(reason) => {
                        self.send(self.cut_error(reason)).await;
                        return Err(Stop::End(reason));
                    }
                }
                let Ok(parsed) = auth::parse_ready_for_query(&status) else {
                    self.send(self.loss_error()).await;
                    return Err(Stop::End(EndReason::BackendLost));
                };
                let mut message = header.to_vec();
                message.push(status[0]);
                self.write_client(&message, true).await?;
                self.status = parsed;
                self.in_flight = false;
                Ok(())
            }
            BackendAction::CopyResponse => {
                self.send(PoolError::FeatureNotSupported).await;
                Err(Stop::End(EndReason::Unsupported))
            }
            BackendAction::Lost => {
                self.send(self.loss_error()).await;
                Err(Stop::End(EndReason::BackendLost))
            }
        }
    }

    /// Sends one error to the client and closes it, unless the session must be cut while the client does not read.
    async fn send(&mut self, error: PoolError) {
        let client = &mut *self.client;
        let _ = guarded(
            send_error(client, error),
            &mut self.routes,
            &mut self.phase,
            self.cluster,
        )
        .await;
    }

    /// Writes bytes to the client (and flushes when asked), raced against the cut.
    async fn write_client(&mut self, bytes: &[u8], flush: bool) -> Result<(), Stop> {
        let client = &mut *self.client;
        let work = async move {
            client.write_all(bytes).await?;
            if flush {
                client.flush().await?;
            }
            Ok::<(), std::io::Error>(())
        };
        match guarded(work, &mut self.routes, &mut self.phase, self.cluster).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err(Stop::End(EndReason::ClientClosed)),
            Err(reason) => Err(Stop::End(reason)),
        }
    }

    /// Writes bytes to the backend and flushes, raced against the cut. `false` when the backend is gone or the cut came.
    async fn write_backend(&mut self, bytes: &[u8]) -> Result<(), Option<EndReason>> {
        let backend = &mut *self.backend;
        let work = async move {
            backend.write_all(bytes).await?;
            backend.flush().await
        };
        match guarded(work, &mut self.routes, &mut self.phase, self.cluster).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err(None),
            Err(reason) => Err(Some(reason)),
        }
    }

    async fn copy_to_backend(&mut self, len: u32) -> Copied {
        copy_exact(
            self.client,
            self.backend,
            len,
            &mut self.buffer,
            &mut self.routes,
            &mut self.phase,
            self.cluster,
        )
        .await
    }

    async fn copy_to_client(&mut self, len: u32) -> Copied {
        copy_exact(
            self.backend,
            self.client,
            len,
            &mut self.buffer,
            &mut self.routes,
            &mut self.phase,
            self.cluster,
        )
        .await
    }
}

/// The outcome of copying a message body.
enum Copied {
    Done,
    SourceGone,
    DestinationGone,
    Cut(EndReason),
}

/// Whether the route view ends the running sessions of the cluster: a positive conflict, which
/// wins, or an approved writer that the control loop marked as lost.
fn route_cut(view: &RouteView, cluster: &ClusterId) -> Option<EndReason> {
    if view.is_conflict(cluster) {
        Some(EndReason::Conflict)
    } else if view.is_lost(cluster) {
        Some(EndReason::NodeLost)
    } else {
        None
    }
}

fn must_cut(
    routes: &watch::Receiver<RouteView>,
    phase: &watch::Receiver<Phase>,
    cluster: &ClusterId,
) -> Option<EndReason> {
    if let Some(reason) = route_cut(&routes.borrow(), cluster) {
        Some(reason)
    } else if *phase.borrow() == Phase::Ending {
        Some(EndReason::Shutdown)
    } else {
        None
    }
}

/// Resolves when the session must be cut: a positive conflict of its cluster, its node marked as
/// lost, or the end of shutdown.
/// It never resolves when neither can happen any more.
async fn wait_cut(
    routes: &mut watch::Receiver<RouteView>,
    phase: &mut watch::Receiver<Phase>,
    cluster: &ClusterId,
) -> EndReason {
    let mut routes_alive = true;
    let mut phase_alive = true;
    loop {
        if let Some(reason) = must_cut(routes, phase, cluster) {
            return reason;
        }
        if !routes_alive && !phase_alive {
            std::future::pending::<()>().await;
        }
        tokio::select! {
            changed = routes.changed(), if routes_alive => routes_alive = changed.is_ok(),
            changed = phase.changed(), if phase_alive => phase_alive = changed.is_ok(),
        }
    }
}

/// Runs `work` (a write toward a peer that may not read, or a read from a peer that may not
/// write) unless the session must be cut first.
///
/// `work` is polled first, so a write that fits completes even when the cut is
/// already due; a write that waits is abandoned as soon as the cut comes.
async fn guarded<F: std::future::Future>(
    work: F,
    routes: &mut watch::Receiver<RouteView>,
    phase: &mut watch::Receiver<Phase>,
    cluster: &ClusterId,
) -> Result<F::Output, EndReason> {
    tokio::select! {
        biased;
        done = work => Ok(done),
        reason = wait_cut(routes, phase, cluster) => Err(reason),
    }
}

/// Copies exactly `len` bytes through the relay buffer, one buffer at a time, and stops when a conflict or
/// the end of shutdown arrives in the middle.
async fn copy_exact<R, W>(
    source: &mut R,
    destination: &mut W,
    len: u32,
    buffer: &mut [u8],
    routes: &mut watch::Receiver<RouteView>,
    phase: &mut watch::Receiver<Phase>,
    cluster: &ClusterId,
) -> Copied
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut left = len as usize;
    let mut routes_alive = true;
    let mut phase_alive = true;
    while left > 0 {
        if let Some(reason) = must_cut(routes, phase, cluster) {
            return Copied::Cut(reason);
        }
        let want = left.min(buffer.len());
        let read = tokio::select! {
            read = source.read(&mut buffer[..want]) => read,
            changed = routes.changed(), if routes_alive => {
                routes_alive = changed.is_ok();
                continue;
            }
            changed = phase.changed(), if phase_alive => {
                phase_alive = changed.is_ok();
                continue;
            }
        };
        let n = match read {
            Ok(0) | Err(_) => return Copied::SourceGone,
            Ok(n) => n,
        };
        match guarded(destination.write_all(&buffer[..n]), routes, phase, cluster).await {
            Ok(Ok(())) => {}
            Ok(Err(_)) => return Copied::DestinationGone,
            Err(reason) => return Copied::Cut(reason),
        }
        left -= n;
    }
    Copied::Done
}

/// The five header bytes of a checked header.
fn raw(header: pool_protocol::message::Header) -> [u8; HEADER_LEN] {
    let len = header.len.to_be_bytes();
    [header.tag, len[0], len[1], len[2], len[3]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pool_key_has_eight_parts_and_hides_the_database_and_the_login() {
        let config = pool_config::validate(
            b"config_version = 1\n[control]\nstate_directory = \"/var/lib/pool\"\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"h\"\nport = 5432\n[[identity]]\nid = \"app\"\nprincipal = \"app\"\ndatabase = \"orders\"\ncluster = \"main\"\nbackend_database = \"secret_db\"\nbackend_login = \"secret_login\"\ncredential_file = \"/x\"\n",
        )
        .expect("config");
        let route = Route {
            node: config.nodes()[0].id().clone(),
            topology_epoch: pool_config::TopologyEpoch::FIRST,
            authority_revision: pool_config::AuthorityRevision::FIRST,
            fresh_until: Duration::from_secs(1),
            system_identifier: pool_config::SystemIdentifier::parse("1")
                .expect("system identifier"),
            timeline: 1,
        };
        let key = pool_key(&config.identities()[0], &route);
        assert_eq!(key.cluster.as_str(), "main");
        assert_eq!(key.node.as_str(), "pg1");
        assert_eq!(
            (key.database.as_str(), key.login.as_str()),
            ("secret_db", "secret_login")
        );
        assert_eq!(key.credential_generation, CredentialGeneration::FIRST);
        assert_eq!(key.topology_epoch, pool_config::TopologyEpoch::FIRST);
        assert_eq!(key.tls_policy, TlsPolicy::VerifyFull);
        assert_eq!(key.startup_profile, StartupProfile::DedicatedV1);
        let debug = format!("{key:?}");
        assert!(
            !debug.contains("secret_db") && !debug.contains("secret_login"),
            "{debug}"
        );
        let next = Route {
            topology_epoch: pool_config::TopologyEpoch::from_stored(2).expect("epoch"),
            ..route
        };
        assert_ne!(
            pool_key(&config.identities()[0], &next),
            key,
            "the epoch is a part of the key"
        );
    }
}
