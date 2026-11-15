//! The probe of one node (spec 0004, "The probe"): one connection as the
//! monitor login of the cluster, over TLS with the CA and server name of the
//! node, and one query, the whole round within `probe.timeout_ms`.
//!
//! `pool-core` opens the connection and runs the query. `pool-control` holds
//! the query text and turns the row into an observation. This module joins
//! the two, which is why it lives in the binary.

use std::time::Duration;

use pool_config::NodeId;
use pool_control::{MonotonicTime, Observation, PROBE_QUERY, observation_from_row};
use pool_core::backend::{self, Login, Node, Purpose};
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

/// Probes every target once, all at the same time, and returns one
/// observation per target, in the order of `targets`.
///
/// The sample time is the time since `origin`, read when the answer (or the
/// failure) arrived. Anything but exactly one row of the right shape within
/// `timeout` is a failed sample.
pub async fn probe_all(
    targets: Vec<Target>,
    timeout: Duration,
    origin: Instant,
) -> Vec<Observation> {
    let mut tasks = Vec::with_capacity(targets.len());
    for target in targets {
        tasks.push(tokio::spawn(probe_one(target, timeout, origin)));
    }
    let mut observations = Vec::with_capacity(tasks.len());
    for task in tasks {
        if let Ok(observation) = task.await {
            observations.push(observation);
        }
    }
    observations
}

/// One round of the probe of one node.
pub async fn probe_one(target: Target, timeout: Duration, origin: Instant) -> Observation {
    let deadline = Instant::now() + timeout;
    let answer = tokio::time::timeout_at(deadline, async {
        let mut connection =
            backend::connect(&target.node, &target.login, Purpose::Probe, deadline).await?;
        connection.query_one_row(PROBE_QUERY).await
    })
    .await;
    let sampled_at = MonotonicTime::from_origin(origin.elapsed());
    match answer {
        Ok(Ok(Some(row))) => {
            let columns: Vec<Option<&str>> = row.iter().map(Option::as_deref).collect();
            observation_from_row(target.id, sampled_at, Some(&columns))
        }
        _ => observation_from_row(target.id, sampled_at, None),
    }
}
