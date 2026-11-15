//! The probe of one node (spec 0004, "The probe"): one connection as the
//! monitor login of the cluster, over TLS with the CA and server name of the
//! node, and one query, the whole round within `probe.timeout_ms`.
//!
//! `pool-core` opens the connection and runs the query. `pool-control` holds
//! the query text and turns the row into an observation. This module joins
//! the two, which is why it lives in the binary.

use std::time::Duration;

use pool_config::NodeId;
use pool_control::{
    MonotonicTime, Observation, ObservationOutcome, PROBE_QUERY, observation_from_row,
};
use pool_core::backend::{self, BackendConnection, BackendError, Login, Node, Purpose};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, watch};
use tokio::time::Instant;

/// The value of `application_name` that the probe connection sets.
const APPLICATION_NAME: &[u8] = b"pool-probe";

/// One node to probe, with the login of its cluster.
pub struct Target {
    /// The node identifier, for the observation.
    pub id: NodeId,
    /// The address and the TLS setup of the node.
    pub node: Node,
    /// The monitor login, its database and its credential.
    pub login: Login,
}

impl Target {
    /// A target for `node` as the monitor `login` of its cluster.
    pub fn new(id: NodeId, node: Node, login: Login) -> Target {
        Target {
            id,
            node,
            login: login.forward("application_name", APPLICATION_NAME),
        }
    }
}

/// Why a probe round gave no evidence, as the event `probe.failed` names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureClass {
    /// No TCP connection.
    Connect,
    /// The TLS handshake failed or was refused.
    Tls,
    /// The login was refused, or the node broke the startup.
    Login,
    /// The deadline of the round passed.
    Timeout,
    /// The query failed or its answer had the wrong shape.
    Answer,
}

impl FailureClass {
    /// The class name of the event.
    pub fn name(self) -> &'static str {
        match self {
            FailureClass::Connect => "connect",
            FailureClass::Tls => "tls",
            FailureClass::Login => "login",
            FailureClass::Timeout => "timeout",
            FailureClass::Answer => "answer",
        }
    }

    fn of_connect(error: BackendError) -> FailureClass {
        match error {
            BackendError::Connect => FailureClass::Connect,
            BackendError::Timeout => FailureClass::Timeout,
            BackendError::Tls | BackendError::TlsRefused => FailureClass::Tls,
            _ => FailureClass::Login,
        }
    }
}

/// The connection a probe keeps between rounds, when it has one.
type Kept = Option<BackendConnection<TcpStream>>;

/// One round: connect when there is no connection, ask the question, and
/// read the row, all before `timeout`. A connection that failed is dropped,
/// so the next round opens a new one. The sample time is the time since
/// `origin` when the answer or the failure arrived.
pub async fn round(
    target: &Target,
    kept: &mut Kept,
    timeout: Duration,
    origin: Instant,
) -> (Observation, Option<FailureClass>) {
    let deadline = Instant::now() + timeout;
    let class = match ask(target, kept, deadline).await {
        Ok(row) => {
            let sampled_at = MonotonicTime::from_origin(origin.elapsed());
            let columns: Vec<Option<&str>> = row.iter().map(Option::as_deref).collect();
            let observation = observation_from_row(target.id.clone(), sampled_at, Some(&columns));
            if observation.outcome() == ObservationOutcome::Failed {
                *kept = None;
                return (observation, Some(FailureClass::Answer));
            }
            return (observation, None);
        }
        Err(class) => class,
    };
    *kept = None;
    let sampled_at = MonotonicTime::from_origin(origin.elapsed());
    (
        observation_from_row(target.id.clone(), sampled_at, None),
        Some(class),
    )
}

async fn ask(
    target: &Target,
    kept: &mut Kept,
    deadline: Instant,
) -> Result<Vec<Option<String>>, FailureClass> {
    if kept.is_none() {
        let connection = backend::connect(&target.node, &target.login, Purpose::Probe, deadline)
            .await
            .map_err(FailureClass::of_connect)?;
        *kept = Some(connection);
    }
    let Some(connection) = kept.as_mut() else {
        return Err(FailureClass::Connect);
    };
    match tokio::time::timeout_at(deadline, connection.query_one_row(PROBE_QUERY)).await {
        Ok(Ok(Some(row))) => Ok(row),
        Ok(Ok(None) | Err(_)) => Err(FailureClass::Answer),
        Err(_) => Err(FailureClass::Timeout),
    }
}

/// Probes every target once, all at the same time, each on a connection of
/// its own that is closed afterwards, and returns one observation per target.
pub async fn probe_all(
    targets: Vec<Target>,
    timeout: Duration,
    origin: Instant,
) -> Vec<Observation> {
    let mut tasks = Vec::with_capacity(targets.len());
    for target in targets {
        tasks.push(tokio::spawn(async move {
            let mut kept = None;
            round(&target, &mut kept, timeout, origin).await.0
        }));
    }
    let mut observations = Vec::with_capacity(tasks.len());
    for task in tasks {
        if let Ok(observation) = task.await {
            observations.push(observation);
        }
    }
    observations
}

/// The probe task of one node in the daemon: one round every `period`, the
/// sample sent to the control loop, until `stop` changes or the loop is gone.
/// It writes `probe.failed` when the class of a failure changes and
/// `probe.recovered` when the node answers again.
pub async fn run(
    target: Target,
    period: Duration,
    timeout: Duration,
    origin: Instant,
    samples: mpsc::Sender<Observation>,
    mut stop: watch::Receiver<bool>,
) {
    let mut kept = None;
    let mut last: Option<FailureClass> = None;
    loop {
        let started = Instant::now();
        let (observation, class) = round(&target, &mut kept, timeout, origin).await;
        if class != last {
            match class {
                Some(class) => tracing::warn!(
                    event = "probe.failed",
                    node_id = target.id.as_str(),
                    class = class.name()
                ),
                None => tracing::info!(event = "probe.recovered", node_id = target.id.as_str()),
            }
            last = class;
        }
        if samples.send(observation).await.is_err() {
            return;
        }
        tokio::select! {
            () = tokio::time::sleep_until(started + period) => {}
            _ = stop.changed() => return,
        }
    }
}
