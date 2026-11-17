//! The log events of the data plane (spec 0004, "Log events").
//!
//! `session.started` and `session.ended` are written as they happen.
//! Refusals and failed backends are counted and written once a second, one
//! line per reason (and per node and class), so that a flood of bad
//! connections cannot flood the log. No event carries a principal name, a
//! login, a database name, a host, a port, a path, SQL, a parameter value or
//! text from PostgreSQL (`SEC-11`).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, PoisonError};

use pool_config::NodeId;

/// Why a connection was refused before its session started; the `reason` of `session.refused`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Refusal {
    /// No handshake permit was free.
    HandshakeLimit,
    /// The startup deadline passed.
    StartupTimeout,
    /// A startup packet without TLS.
    Plaintext,
    /// The TLS handshake failed.
    Tls,
    /// A packet or message broke the protocol.
    Protocol,
    /// A protocol feature that this build does not support.
    Unsupported,
    /// A startup packet above its limit.
    MessageLimit,
    /// A wrong password, or a name that is no principal.
    Authentication,
    /// No identity entry for the principal and database.
    Unmapped,
    /// No session permit was free.
    SessionLimit,
    /// No permit of the principal was free.
    PrincipalLimit,
    /// The route of the cluster was closed or stale, at step 5 or step 9.
    Closed,
    /// No room among the waiters for a backend.
    WaitLimit,
    /// The wait for a backend ran out.
    AcquireTimeout,
    /// The backend could not be opened.
    BackendUnavailable,
    /// A `CancelRequest` was read and dropped.
    CancelIgnored,
    /// An accept of the listener failed.
    AcceptFailed,
    /// The operating system gave no random bytes for the login.
    RandomFailed,
}

impl Refusal {
    /// The constant name in the log.
    pub fn name(self) -> &'static str {
        match self {
            Self::HandshakeLimit => "handshake_limit",
            Self::StartupTimeout => "startup_timeout",
            Self::Plaintext => "plaintext",
            Self::Tls => "tls",
            Self::Protocol => "protocol",
            Self::Unsupported => "unsupported",
            Self::MessageLimit => "message_limit",
            Self::Authentication => "authentication",
            Self::Unmapped => "unmapped",
            Self::SessionLimit => "session_limit",
            Self::PrincipalLimit => "principal_limit",
            Self::Closed => "closed",
            Self::WaitLimit => "wait_limit",
            Self::AcquireTimeout => "acquire_timeout",
            Self::BackendUnavailable => "backend_unavailable",
            Self::CancelIgnored => "cancel_ignored",
            Self::AcceptFailed => "accept_failed",
            Self::RandomFailed => "random_failed",
        }
    }
}

/// Why a session ended; the `reason` of `session.ended`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndReason {
    /// The client sent `Terminate`.
    ClientTerminate,
    /// The client closed, or a read or write toward it failed.
    ClientClosed,
    /// The backend was lost.
    BackendLost,
    /// The client sent a message this slice does not support, or the backend a COPY response.
    Unsupported,
    /// The client broke the protocol.
    Protocol,
    /// A `Query` above `limits.frontend_message_max_bytes`.
    MessageLimit,
    /// The cluster closed with a positive conflict.
    Conflict,
    /// The control loop marked the approved writer of the cluster as lost (spec 0004, decision 84).
    NodeLost,
    /// The daemon shut down.
    Shutdown,
}

impl EndReason {
    /// The constant name in the log.
    pub fn name(self) -> &'static str {
        match self {
            Self::ClientTerminate => "client_terminate",
            Self::ClientClosed => "client_closed",
            Self::BackendLost => "backend_lost",
            Self::Unsupported => "unsupported",
            Self::Protocol => "protocol",
            Self::MessageLimit => "message_limit",
            Self::Conflict => "conflict",
            Self::NodeLost => "node_lost",
            Self::Shutdown => "shutdown",
        }
    }
}

/// One line of `backend.failed`: the node, the class and the SQLSTATE PostgreSQL sent.
type BackendFailure = (NodeId, &'static str, Option<[u8; 5]>);

/// The counters behind `session.refused` and `backend.failed`.
#[derive(Debug, Default)]
pub struct Counters {
    refused: Mutex<BTreeMap<Refusal, u64>>,
    backend: Mutex<BTreeMap<BackendFailure, u64>>,
    refused_total: Mutex<BTreeMap<Refusal, u64>>,
    pool: [AtomicU64; 5],
}

/// Saturating lifetime counts of the bounded physical-backend owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolCounts {
    /// Successful physical opens, including dedicated connections.
    pub opened: u64,
    /// Successful later handoffs of a clean physical backend.
    pub reused: u64,
    /// Cleanup candidates closed because their proof failed.
    pub reset_closed: u64,
    /// Idle sockets closed by their idle or physical age limit.
    pub expired: u64,
    /// Acquisitions refused by admission or its deadline.
    pub admission_refused: u64,
}

impl Counters {
    pub(crate) fn pool_event(&self, index: usize) {
        let _ = self.pool[index].try_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            Some(n.saturating_add(1))
        });
    }
    /// Lifetime physical-backend counts. No peer values are captured.
    pub fn pool_counts(&self) -> PoolCounts {
        let counts = self.pool.each_ref().map(|n| n.load(Ordering::Relaxed));
        PoolCounts {
            opened: counts[0],
            reused: counts[1],
            reset_closed: counts[2],
            expired: counts[3],
            admission_refused: counts[4],
        }
    }

    /// Counts one refusal.
    pub fn refuse(&self, reason: Refusal) {
        if matches!(reason, Refusal::WaitLimit | Refusal::AcquireTimeout) {
            self.pool_event(4);
        }
        for map in [&self.refused, &self.refused_total] {
            *map.lock()
                .unwrap_or_else(PoisonError::into_inner)
                .entry(reason)
                .or_default() += 1;
        }
    }

    /// Counts one failed backend attempt of a node, with its class and the SQLSTATE PostgreSQL sent.
    pub fn backend_failed(&self, node: &NodeId, class: &'static str, sqlstate: Option<[u8; 5]>) {
        *self
            .backend
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry((node.clone(), class, sqlstate))
            .or_default() += 1;
    }

    /// How many times this reason was counted since the start, for tests and diagnostics.
    pub fn refused(&self, reason: Refusal) -> u64 {
        self.refused_total
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&reason)
            .copied()
            .unwrap_or(0)
    }

    /// Writes one `session.refused` line per reason and one `backend.failed` line per node, class
    /// and SQLSTATE for what was counted since the last flush. The daemon calls it once a second.
    pub fn flush(&self) {
        let refused =
            std::mem::take(&mut *self.refused.lock().unwrap_or_else(PoisonError::into_inner));
        for (reason, count) in refused {
            tracing::warn!(event = "session.refused", reason = reason.name(), count);
        }
        let backend =
            std::mem::take(&mut *self.backend.lock().unwrap_or_else(PoisonError::into_inner));
        for ((node, class, sqlstate), count) in backend {
            match sqlstate.and_then(|s| String::from_utf8(s.to_vec()).ok()) {
                Some(sqlstate) => tracing::warn!(
                    event = "backend.failed",
                    node_id = node.as_str(),
                    class,
                    sqlstate = sqlstate.as_str(),
                    count
                ),
                None => tracing::warn!(
                    event = "backend.failed",
                    node_id = node.as_str(),
                    class,
                    count
                ),
            }
        }
    }
}
