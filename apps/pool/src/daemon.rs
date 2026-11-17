//! `pool run` (spec 0004, "pool run"): the ten steps from the file to a
//! serving daemon, and the shutdown.
//!
//! Until step 8 every failure is a finding with exit 1 or 3, written as
//! spec 0001 says. From step 8 on every line on stderr is a log event, and
//! nothing is written to stdout while the daemon serves.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use pool_config::{ConfigurationGeneration, IdentityId, ValidatedConfig};
use pool_control::{AuthorityState, Controller, Journal};
use pool_core::backend::{Credential, Login, TcpConnector};
use pool_core::route::RouteView;
use pool_core::scram::MockKey;
use pool_core::secret::{self, SecretError};
use pool_core::session::{Server, ServerError, ServerParts};
use pool_core::{load, tls};
use tokio::net::{TcpListener, TcpSocket};
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::{mpsc, watch};
use tokio::time::Instant;

use crate::commands::{
    RUNTIME_FAILED, RUNTIME_FAILED_MESSAGE, load_config, secret_error, state_error,
};
use crate::control::ControlLoop;
use crate::log;
use crate::output::{Item, Report};
use crate::probe::{self, Target};
use crate::state::StateDirectory;

const NOT_SERVING: &str = "the journal state does not allow serving";
const JOURNAL_FAILED: &str = "journal.failed";
const JOURNAL_FAILED_MESSAGE: &str = "the journal could not be written";
const LISTENER_FAILED: &str = "listener.failed";
const LISTENER_FAILED_MESSAGE: &str = "the listener address could not be bound";
const RANDOM_FAILED: &str = "random.failed";
const RANDOM_FAILED_MESSAGE: &str = "random bytes could not be obtained";

/// How often the counters of `session.refused` and `backend.failed` are written.
const COUNTER_FLUSH: Duration = Duration::from_secs(1);

/// The finding code of a start that was not classified `Ready`.
fn not_ready(state: &AuthorityState) -> &'static str {
    match state.name() {
        "uninitialized" => "authority.uninitialized",
        "unconfirmed" => "authority.unconfirmed",
        "missing" => "authority.missing",
        "mismatch" => "authority.mismatch",
        "restore_pending" => "authority.restore_pending",
        _ => "authority.unreadable",
    }
}

/// Everything steps 1 to 5 read, ready for the runtime.
struct Prepared {
    config: Arc<ValidatedConfig>,
    _state: StateDirectory,
    server_parts: ServerParts<TcpConnector>,
    targets: Vec<Target>,
    journal: Journal,
    controller: Controller,
    routes: watch::Sender<RouteView>,
    origin: Instant,
    start_sequence: u64,
    authority_id: String,
}

/// How `pool run` ended.
pub enum Outcome {
    /// A failure before step 8, written as a finding.
    Report(Report),
    /// A signal stopped the daemon cleanly: exit 0.
    Stopped,
    /// A task of the daemon ended without a shutdown asking for it. The
    /// event `daemon.failed` was written, and the exit status is 3.
    Failed,
}

/// Runs `pool run` on the file at `path`.
pub fn run_file(path: &Path) -> Outcome {
    let prepared = match prepare(path) {
        Ok(prepared) => prepared,
        Err(report) => return Outcome::Report(report),
    };
    // 6. The runtime, with at most four threads for blocking work.
    let Ok(runtime) = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .max_blocking_threads(4)
        .build()
    else {
        return Outcome::Report(runtime_failed());
    };
    let outcome = runtime.block_on(serve(prepared));
    // A protected-file load already in a blocking syscall cannot be aborted.
    // It has no publication authority after serve ends and must not hold exit forever.
    runtime.shutdown_timeout(pool_core::session::END_TIMEOUT);
    outcome
}

/// Steps 1 to 5: nothing listens and nothing runs yet.
fn prepare(path: &Path) -> Result<Prepared, Report> {
    // 1. The file, the view for serving and the configuration generation.
    let config = load_config(path)?;
    let view = config
        .serving()
        .map_err(|rejection| Report::invalid(&rejection))?;
    // 2. The state directory and its lock, kept until the process ends.
    let state = StateDirectory::open(view.state_directory()).map_err(state_error)?;
    let owner = state.owner();
    // 3. The journal, read without writing, and classified once.
    let classified = Journal::read(&state.journal()).classify(&config);
    let AuthorityState::Ready(authority) = &classified else {
        return Err(Report::Refused(vec![Item::plain(
            not_ready(&classified),
            NOT_SERVING,
        )]));
    };
    // 4. Every protected file, and the key of the made up verifiers.
    let listener = view.listener();
    let chain = secret::read_certificate_chain(listener.tls_certificate_file(), owner)
        .map_err(|error| secret_error(error, "listener.tls_certificate_file"))?;
    let key = secret::read_private_key(listener.tls_private_key().path(), owner)
        .map_err(|error| secret_error(error, "listener.tls_private_key_file"))?;
    let tls = tls::server_config(&chain, &key)
        .map_err(|error| secret_error(error, "listener.tls_private_key_file"))?;
    let mut verifiers = Vec::new();
    for principal in config.principals() {
        let verifier = load::verifier(principal, owner)
            .map_err(|error| secret_error(error, "principal.verifier_file"))?;
        verifiers.push((principal.id().clone(), verifier));
    }
    let credentials = read_identity_credentials(&config, owner)
        .map_err(|error| secret_error(error, "identity.credential_file"))?;
    let mut nodes = Vec::new();
    for node in config.nodes() {
        let loaded =
            load::node(node, owner).map_err(|error| secret_error(error, "node.tls_ca_file"))?;
        nodes.push((node.id().clone(), loaded));
    }
    let mut targets = Vec::new();
    for cluster in config.clusters() {
        let reference = cluster.monitor_credential().ok_or_else(|| {
            secret_error(SecretError::NotFound, "cluster.monitor_credential_file")
        })?;
        let credential = load::credential(reference, owner)
            .map_err(|error| secret_error(error, "cluster.monitor_credential_file"))?;
        let user = cluster.monitor_login().unwrap_or_default();
        let login = Login::new(user, cluster.monitor_database(), credential);
        for (id, node) in &nodes {
            let belongs = config
                .nodes()
                .iter()
                .any(|entry| entry.id() == id && entry.cluster() == cluster.id());
            if belongs {
                targets.push(Target::new(id.clone(), node.clone(), login.clone()));
            }
        }
    }
    let mock_key = MockKey::generate()
        .map_err(|_| Report::Error(Item::plain(RANDOM_FAILED, RANDOM_FAILED_MESSAGE)))?;
    // 5. The journal, opened for writing, with its start sequence advanced.
    let (journal, start_sequence) = Journal::open_ready(&state.journal(), authority)
        .map_err(|_| Report::Error(Item::plain(JOURNAL_FAILED, JOURNAL_FAILED_MESSAGE)))?;
    let cluster_ids: Vec<_> = config.clusters().iter().map(|c| c.id().clone()).collect();
    let (routes, receiver) = watch::channel(RouteView::closed(&cluster_ids));
    let origin = Instant::now();
    let authority_id = authority.authority_id().to_string();
    let config = Arc::new(config);
    let server_parts = ServerParts {
        config: Arc::clone(&config),
        tls,
        verifiers,
        credentials,
        nodes,
        mock_key,
        connector: TcpConnector,
        routes: receiver,
        origin,
        start_sequence: start_sequence.get(),
    };
    let controller = Controller::new(classified.clone());
    Ok(Prepared {
        config,
        _state: state,
        server_parts,
        targets,
        journal,
        controller,
        routes,
        origin,
        start_sequence: start_sequence.get(),
        authority_id,
    })
}

/// The finding of a server that could not be built: the key whose file it
/// misses. The serving view and step 4 make this unreachable.
fn server_error(error: ServerError) -> Report {
    let key = match error {
        ServerError::MissingVerifier => "principal.verifier_file",
        ServerError::MissingCredential => "identity.credential_file",
        ServerError::MissingNode => "node.tls_ca_file",
    };
    secret_error(SecretError::Invalid, key)
}

/// The task that ended first without a shutdown asking for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ended {
    Signal,
    Task(&'static str),
}

/// Steps 7 to 10, inside the runtime.
async fn serve(prepared: Prepared) -> Outcome {
    let Prepared {
        config,
        _state,
        server_parts,
        targets,
        journal,
        controller,
        routes,
        origin,
        start_sequence,
        authority_id,
    } = prepared;
    let server = match Server::with_identity_query(server_parts, pool_control::PROBE_QUERY) {
        Ok(server) => server,
        Err(error) => return Outcome::Report(server_error(error)),
    };
    // 7. The listener: the first address of `listener.host` that binds.
    let Some(listener_config) = config.listener() else {
        return Outcome::Report(Report::Error(Item::plain(
            LISTENER_FAILED,
            LISTENER_FAILED_MESSAGE,
        )));
    };
    let Some(listener) = bind(listener_config.host(), listener_config.port()).await else {
        return Outcome::Report(Report::Error(Item::plain(
            LISTENER_FAILED,
            LISTENER_FAILED_MESSAGE,
        )));
    };
    // The handlers for SIGTERM and SIGINT, before step 9: a daemon that
    // could not be stopped cleanly does not start.
    let (Ok(mut terminate), Ok(mut interrupt), Ok(mut hangup)) = (
        signal(SignalKind::terminate()),
        signal(SignalKind::interrupt()),
        signal(SignalKind::hangup()),
    ) else {
        return Outcome::Report(runtime_failed());
    };
    // 8. From here every line on stderr is a log event.
    let mut log = log::install();
    // 9. The control loop with an empty board, the first view (every
    //    cluster closed, published at creation), the probes and the listener.
    let (stop, stopped) = watch::channel(false);
    let (samples, received) = mpsc::channel(targets.len().saturating_mul(2).max(1));
    let probe = config.probe();
    let mut probes = tokio::task::JoinSet::new();
    for target in targets {
        probes.spawn(probe::run(
            target,
            probe.period(),
            probe.timeout(),
            origin,
            samples.clone(),
            stopped.clone(),
        ));
    }
    drop(samples);
    let mut control = tokio::spawn(
        ControlLoop::new(Arc::clone(&config), controller, journal, routes, origin)
            .run(received, stopped.clone()),
    );
    let mut serving = tokio::spawn({
        let server = Arc::clone(&server);
        async move { server.serve(listener).await }
    });
    let flushing = tokio::spawn({
        let server = Arc::clone(&server);
        let mut stopped = stopped.clone();
        async move {
            let mut tick = tokio::time::interval(COUNTER_FLUSH);
            loop {
                tokio::select! {
                    _ = tick.tick() => server.counters().flush(),
                    _ = stopped.changed() => return,
                }
            }
        }
    });
    tracing::info!(
        event = "daemon.started",
        start_sequence,
        configuration_generation = ConfigurationGeneration::FIRST.get(),
        authority_id = authority_id.as_str()
    );
    // 10. SIGTERM or SIGINT, or a task that ended without being asked to.
    //     A second signal does nothing more.
    // A join handle that returned is not awaited again.
    let mut journal = None;
    let mut control_done = false;
    let mut serving_done = false;
    let owner = _state.owner();
    let mut reloading = None;
    let ended = loop {
        tokio::select! {
            observed = supervise(
                &mut control,
                &mut probes,
                &mut serving,
                log_writer(&mut log),
                async { tokio::select! { _ = terminate.recv() => {}, _ = interrupt.recv() => {} } },
            ) => {
                match observed.ended {
                    Ended::Task("control") => { control_done = true; journal = observed.journal; },
                    Ended::Task("listener") => serving_done = true,
                    _ => {},
                }
                break observed.ended;
            }
            _ = hangup.recv() => {
                // At most one protected-file load exists, even under a signal flood.
                if reloading.is_none() {
                    let config = Arc::clone(&config);
                    reloading = Some(tokio::task::spawn_blocking(move || read_identity_credentials(&config, owner)));
                }
            }
            returned = reload_result(&mut reloading) => {
                reloading = None;
                match returned {
                    Ok(Ok(credentials)) => match server.rotate_credentials(credentials) {
                        Ok(()) => tracing::info!(event = "credential.reloaded", identity_count = config.identities().len() as u64),
                        Err(_) => tracing::warn!(event = "credential.reload_failed", reason = "generation"),
                    },
                    _ => tracing::warn!(event = "credential.reload_failed", reason = "files"),
                }
            }
        }
    };
    if let Some(reloading) = reloading {
        // A pending load has no publication authority once shutdown starts.
        reloading.abort();
    }
    let plan = stop_plan(ended, server.sessions());
    // A signal drains; a failed supervised task cuts sessions immediately.
    match plan {
        StopPlan::Drain => server.shutdown().await,
        StopPlan::Immediate => server.shutdown_now().await,
    }
    if !serving_done {
        let _ = serving.await;
    }
    let _ = stop.send(true);
    if !control_done {
        journal = control.await.ok().flatten();
    }
    drop(journal);
    probes.shutdown().await;
    let _ = flushing.await;
    server.counters().flush();
    tracing::info!(event = "daemon.stopped");
    if let Some(log) = &log {
        log.flush().await;
    }
    outcome(plan)
}

/// A complete protected-file snapshot; callers publish only after every read passed.
type Credentials = Vec<(IdentityId, Arc<Credential>)>;
type Reload = tokio::task::JoinHandle<Result<Credentials, SecretError>>;

fn read_identity_credentials(
    config: &ValidatedConfig,
    owner: u32,
) -> Result<Credentials, SecretError> {
    config
        .identities()
        .iter()
        .map(|identity| {
            load::credential(identity.credential(), owner)
                .map(|credential| (identity.id().clone(), credential))
        })
        .collect()
}

async fn reload_result(
    reloading: &mut Option<Reload>,
) -> Result<Result<Credentials, SecretError>, tokio::task::JoinError> {
    match reloading {
        Some(task) => task.await,
        None => std::future::pending().await,
    }
}

struct ObservedEnd {
    ended: Ended,
    journal: Option<Journal>,
}

/// One cancellable watch over every supervised task. A returned join handle is never awaited twice.
async fn supervise<L, S>(
    control: &mut tokio::task::JoinHandle<Option<Journal>>,
    probes: &mut tokio::task::JoinSet<()>,
    serving: &mut tokio::task::JoinHandle<()>,
    log: L,
    signal: S,
) -> ObservedEnd
where
    L: std::future::Future<Output = ()>,
    S: std::future::Future<Output = ()>,
{
    tokio::select! {
        _ = signal => ObservedEnd { ended: Ended::Signal, journal: None },
        returned = control => ObservedEnd { ended: Ended::Task("control"), journal: returned.ok().flatten() },
        Some(_) = probes.join_next() => ObservedEnd { ended: Ended::Task("probe"), journal: None },
        _ = serving => ObservedEnd { ended: Ended::Task("listener"), journal: None },
        _ = log => ObservedEnd { ended: Ended::Task("log"), journal: None },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StopPlan {
    Drain,
    Immediate,
}

fn stop_plan(ended: Ended, sessions: usize) -> StopPlan {
    match ended {
        Ended::Signal => {
            tracing::info!(event = "daemon.stopping", sessions);
            StopPlan::Drain
        }
        Ended::Task(task) => {
            if task == "log" {
                log::write_log_failed();
            } else {
                tracing::error!(event = "daemon.failed", task);
            }
            StopPlan::Immediate
        }
    }
}

fn outcome(plan: StopPlan) -> Outcome {
    match plan {
        StopPlan::Drain => Outcome::Stopped,
        StopPlan::Immediate => Outcome::Failed,
    }
}

/// Waits until the task of the log writer ended, or forever when there is
/// none.
async fn log_writer(log: &mut Option<log::LogHandle>) {
    match log {
        Some(log) => {
            let _ = log.writer().await;
        }
        None => std::future::pending::<()>().await,
    }
}

/// How many connections the kernel may hold for the listener before they
/// are accepted.
const BACKLOG: u32 = 1024;

/// Step 7: binds the first address of `host` that works. The socket has
/// TCP keepalive on before it listens, and every socket it accepts takes
/// that option from it (spec 0004, "A peer that vanished", decision 84).
async fn bind(host: &str, port: u16) -> Option<TcpListener> {
    for address in tokio::net::lookup_host((host, port)).await.ok()? {
        let socket = match address {
            std::net::SocketAddr::V4(_) => TcpSocket::new_v4(),
            std::net::SocketAddr::V6(_) => TcpSocket::new_v6(),
        };
        let Ok(socket) = socket else { continue };
        let ready = socket
            .set_reuseaddr(true)
            .and_then(|()| socket.set_keepalive(true))
            .and_then(|()| socket.bind(address));
        if ready.is_err() {
            continue;
        }
        if let Ok(listener) = socket.listen(BACKLOG) {
            return Some(listener);
        }
    }
    None
}

/// The finding of a runtime, or a signal handler, that could not be set up.
fn runtime_failed() -> Report {
    Report::Error(Item::plain(RUNTIME_FAILED, RUNTIME_FAILED_MESSAGE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    type Events = Arc<Mutex<Vec<Vec<(String, String)>>>>;

    /// Keeps every event as its fields, in order.
    struct Capture(Events);

    struct Fields(Vec<(String, String)>);

    impl tracing::field::Visit for Fields {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push((field.name().to_owned(), value.to_owned()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0.push((field.name().to_owned(), format!("{value:?}")));
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

    #[test]
    fn each_state_that_is_not_ready_gives_its_own_finding() {
        // covers: 0004 AC-13 (decision 101 of spec 0004): the six states
        // that are not `Ready` each give the code of their own name, and no
        // two share one.
        let cases = [
            (AuthorityState::Uninitialized, "authority.uninitialized"),
            (AuthorityState::Unconfirmed, "authority.unconfirmed"),
            (AuthorityState::Missing, "authority.missing"),
            (AuthorityState::Unreadable, "authority.unreadable"),
            (AuthorityState::Mismatch, "authority.mismatch"),
            (AuthorityState::Held, "authority.restore_pending"),
        ];
        for (state, code) in &cases {
            assert_eq!(not_ready(state), *code, "{}", state.name());
        }
        let mut codes: Vec<&str> = cases.iter().map(|(state, _)| not_ready(state)).collect();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), cases.len());
    }

    // Added by the tests of scope 004, round eight (S14 of the checker run of step 15).
    #[tokio::test]
    async fn ac30_the_listener_socket_has_keepalive_on_and_hands_it_to_the_sockets_it_accepts() {
        // covers: 0004 AC-30, the first half of its first sentence. The
        // option is read with `TcpSocket::keepalive` after a safe conversion
        // through `OwnedFd`; no `unsafe`.
        use std::os::fd::OwnedFd;
        let listener = bind("127.0.0.1", 0).await.expect("bind");
        let address = listener.local_addr().expect("address");
        let client = tokio::net::TcpStream::connect(address)
            .await
            .expect("connect");
        let (accepted, _) = listener.accept().await.expect("accept");
        let accepted = TcpSocket::from_std_stream(accepted.into_std().expect("std"));
        let listener: OwnedFd = listener.into_std().expect("std").into();
        let listener = TcpSocket::from_std_stream(std::net::TcpStream::from(listener));
        assert!(listener.keepalive().expect("keepalive"), "the listener");
        // Linux and macOS both hand the option of the listener to every accepted socket.
        assert!(
            accepted.keepalive().expect("keepalive"),
            "an accepted socket"
        );
        drop(client);
    }
    #[tokio::test]
    async fn scope005_every_supervised_task_ending_selects_immediate_failure() {
        // covers: 0005 AC-9. Real join handles exercise the production watcher.
        let (events, _capture) = capture();
        for task in ["control", "probe", "listener", "log"] {
            let mut control = if task == "control" {
                tokio::spawn(async { None })
            } else {
                tokio::spawn(std::future::pending::<Option<Journal>>())
            };
            let mut probes = tokio::task::JoinSet::new();
            if task == "probe" {
                probes.spawn(async {});
            } else {
                probes.spawn(std::future::pending::<()>());
            }
            let mut listener = if task == "listener" {
                tokio::spawn(async {})
            } else {
                tokio::spawn(std::future::pending::<()>())
            };
            let observed = tokio::time::timeout(
                Duration::from_secs(1),
                supervise(
                    &mut control,
                    &mut probes,
                    &mut listener,
                    async {
                        if task != "log" {
                            std::future::pending::<()>().await
                        }
                    },
                    std::future::pending::<()>(),
                ),
            )
            .await
            .expect("task exit is noticed immediately");
            assert_eq!(observed.ended, Ended::Task(task));
            // The logging branch and shutdown choice are the same production function.
            let plan = stop_plan(observed.ended, 7);
            if task != "log" {
                assert!(
                    events.lock().expect("events").iter().any(|fields| {
                        fields.contains(&("event".to_owned(), "daemon.failed".to_owned()))
                            && fields.contains(&("task".to_owned(), task.to_owned()))
                    }),
                    "the actual failure event identifies {task}"
                );
            }
            assert_eq!(plan, StopPlan::Immediate);
            assert!(matches!(outcome(plan), Outcome::Failed));
            control.abort();
            listener.abort();
            probes.shutdown().await;
        }
    }

    #[tokio::test]
    async fn scope005_a_signal_drains_while_an_empty_probe_set_never_looks_failed() {
        // covers: 0005 AC-9. An empty join set is not a completed probe.
        let mut control = tokio::spawn(std::future::pending::<Option<Journal>>());
        let mut probes = tokio::task::JoinSet::new();
        let mut listener = tokio::spawn(std::future::pending::<()>());
        let observed = supervise(
            &mut control,
            &mut probes,
            &mut listener,
            std::future::pending::<()>(),
            async {},
        )
        .await;
        assert_eq!(observed.ended, Ended::Signal);
        let plan = stop_plan(observed.ended, 0);
        assert_eq!(plan, StopPlan::Drain);
        assert!(matches!(outcome(plan), Outcome::Stopped));
        control.abort();
        listener.abort();
    }

    #[tokio::test]
    async fn scope005_cancelling_a_watch_does_not_lose_the_next_task_result() {
        // covers: 0005 AC-9. SIGHUP interrupts the watcher without taking task ownership.
        let (release, waiting) = tokio::sync::oneshot::channel::<()>();
        let mut control = tokio::spawn(async {
            let _ = waiting.await;
            None
        });
        let mut probes = tokio::task::JoinSet::new();
        let mut listener = tokio::spawn(std::future::pending::<()>());
        assert!(
            tokio::time::timeout(
                Duration::from_millis(10),
                supervise(
                    &mut control,
                    &mut probes,
                    &mut listener,
                    std::future::pending::<()>(),
                    std::future::pending::<()>(),
                )
            )
            .await
            .is_err()
        );
        release.send(()).expect("release");
        let observed = supervise(
            &mut control,
            &mut probes,
            &mut listener,
            std::future::pending::<()>(),
            std::future::pending::<()>(),
        )
        .await;
        assert_eq!(observed.ended, Ended::Task("control"));
        listener.abort();
    }
}
