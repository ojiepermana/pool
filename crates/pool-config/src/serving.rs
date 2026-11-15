//! The keys that spec 0004 adds: `[listener]`, `[limits]`, `[probe]`,
//! `[[principal]]`, the three monitor keys of a cluster and the two TLS keys
//! of a node. Their raw shapes, their checks and the records a validated file
//! exposes.
//!
//! The contract is spec 0004, section "Configuration". Every table and key is
//! optional in the schema. What a command needs beyond a valid file is
//! checked by the views in `views.rs`.

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use toml::Spanned;

use crate::topology::{At, Checker, Redacted, parse_host, parse_name, parse_path};
use crate::{Finding, FindingCode, KnownField, Location, PrincipalId, SecretReference};

const KEY_LISTENER_HOST: &str = "listener.host";
const KEY_LISTENER_PORT: &str = "listener.port";
const KEY_LISTENER_CERTIFICATE: &str = "listener.tls_certificate_file";
const KEY_LISTENER_PRIVATE_KEY: &str = "listener.tls_private_key_file";
const KEY_PROBE_PERIOD: &str = "probe.period_ms";
const KEY_PROBE_TIMEOUT: &str = "probe.timeout_ms";
const KEY_PROBE_EXPIRY: &str = "probe.observation_expiry_ms";
const KEY_PROBE_NODE_LOSS: &str = "probe.node_loss_timeout_ms";
const KEY_CLUSTER_MONITOR_LOGIN: &str = "cluster.monitor_login";
const KEY_CLUSTER_MONITOR_DATABASE: &str = "cluster.monitor_database";
const KEY_CLUSTER_MONITOR_CREDENTIAL: &str = "cluster.monitor_credential_file";
const KEY_NODE_TLS_CA: &str = "node.tls_ca_file";
const KEY_NODE_TLS_SERVER_NAME: &str = "node.tls_server_name";
const KEY_PRINCIPAL_ID: &str = "principal.id";
const KEY_PRINCIPAL_NAME: &str = "principal.name";
const KEY_PRINCIPAL_VERIFIER: &str = "principal.verifier_file";
const KEY_PRINCIPAL_SESSION_MAX: &str = "principal.session_max_count";

/// The monitor database when `cluster.monitor_database` is left out.
const DEFAULT_MONITOR_DATABASE: &str = "postgres";

/// The range of `principal.session_max_count`.
const SESSION_MAX_RANGE: RangeInclusive<u32> = 1..=100_000;

/// The validated `[listener]` table: where `pool run` accepts clients.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run. A change needs a restart.
pub struct ListenerConfig {
    host: String,
    port: u16,
    tls_certificate_file: PathBuf,
    tls_private_key: SecretReference,
}

impl ListenerConfig {
    /// The address to listen on, from `listener.host`. It never appears in
    /// default telemetry.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The TCP port to listen on, from `listener.port`. It never appears in
    /// default telemetry.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// The PEM file with the certificate chain, leaf first, from
    /// `listener.tls_certificate_file`. This crate never opens it.
    pub fn tls_certificate_file(&self) -> &Path {
        &self.tls_certificate_file
    }

    /// The reference to the private key of the certificate, from
    /// `listener.tls_private_key_file`. This crate never opens it.
    pub fn tls_private_key(&self) -> &SecretReference {
        &self.tls_private_key
    }
}

impl fmt::Debug for ListenerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListenerConfig")
            .field("host", &Redacted)
            .field("port", &Redacted)
            .field("tls_certificate_file", &Redacted)
            .field("tls_private_key", &self.tls_private_key)
            .finish()
    }
}

/// The validated `[limits]` table. Each key that is left out has its default.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run. A change needs a restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    session_max_count: u32,
    handshake_max_count: u32,
    backend_wait_max_count: u32,
    node_backend_max_count: u32,
    startup_timeout_ms: u32,
    backend_acquire_timeout_ms: u32,
    shutdown_timeout_ms: u32,
    startup_packet_max_bytes: u32,
    frontend_message_max_bytes: u32,
}

impl Limits {
    /// The defaults of the table under "Keys" in spec 0004.
    pub(crate) const DEFAULT: Limits = Limits {
        session_max_count: 1000,
        handshake_max_count: 64,
        backend_wait_max_count: 100,
        node_backend_max_count: 100,
        startup_timeout_ms: 5000,
        backend_acquire_timeout_ms: 5000,
        shutdown_timeout_ms: 30_000,
        startup_packet_max_bytes: 8192,
        frontend_message_max_bytes: 16_777_216,
    };

    /// The most sessions open at once, from `limits.session_max_count`.
    pub fn session_max_count(&self) -> u32 {
        self.session_max_count
    }

    /// The most clients in the handshake at once, from
    /// `limits.handshake_max_count`.
    pub fn handshake_max_count(&self) -> u32 {
        self.handshake_max_count
    }

    /// The most sessions waiting for a backend at once, from
    /// `limits.backend_wait_max_count`. It may be 0.
    pub fn backend_wait_max_count(&self) -> u32 {
        self.backend_wait_max_count
    }

    /// The most backend connections to one node, from
    /// `limits.node_backend_max_count`.
    pub fn node_backend_max_count(&self) -> u32 {
        self.node_backend_max_count
    }

    /// The longest a client may take from connect to login, from
    /// `limits.startup_timeout_ms`.
    pub fn startup_timeout(&self) -> Duration {
        millis(self.startup_timeout_ms)
    }

    /// The longest a session waits for a backend, from
    /// `limits.backend_acquire_timeout_ms`.
    pub fn backend_acquire_timeout(&self) -> Duration {
        millis(self.backend_acquire_timeout_ms)
    }

    /// The longest a shutdown waits for sessions to end, from
    /// `limits.shutdown_timeout_ms`. It may be 0.
    pub fn shutdown_timeout(&self) -> Duration {
        millis(self.shutdown_timeout_ms)
    }

    /// The largest first packet of a client, from
    /// `limits.startup_packet_max_bytes`.
    pub fn startup_packet_max_bytes(&self) -> u32 {
        self.startup_packet_max_bytes
    }

    /// The largest message from a client after login, from
    /// `limits.frontend_message_max_bytes`.
    pub fn frontend_message_max_bytes(&self) -> u32 {
        self.frontend_message_max_bytes
    }
}

/// The validated `[probe]` table. Each key that is left out has its default.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run. A change needs a restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeConfig {
    period_ms: u32,
    timeout_ms: u32,
    observation_expiry_ms: u32,
    node_loss_timeout_ms: u32,
}

impl ProbeConfig {
    /// The defaults of the table under "Keys" in spec 0004.
    pub(crate) const DEFAULT: ProbeConfig = ProbeConfig {
        period_ms: 2000,
        timeout_ms: 1000,
        observation_expiry_ms: 6000,
        node_loss_timeout_ms: 30_000,
    };

    /// The time between two probes of one node, from `probe.period_ms`.
    pub fn period(&self) -> Duration {
        millis(self.period_ms)
    }

    /// The longest one probe may take, from `probe.timeout_ms`.
    pub fn timeout(&self) -> Duration {
        millis(self.timeout_ms)
    }

    /// How long an observation stays fresh, from
    /// `probe.observation_expiry_ms`. It is never less than the period plus
    /// the timeout.
    pub fn observation_expiry(&self) -> Duration {
        millis(self.observation_expiry_ms)
    }

    /// How long the approved writer of a cluster may give the probe no answer at all before
    /// its node is lost and its running sessions end, from `probe.node_loss_timeout_ms`
    /// (spec 0004, decision 84). It is never less than twice the period plus the timeout
    /// (decision 109).
    pub fn node_loss_timeout(&self) -> Duration {
        millis(self.node_loss_timeout_ms)
    }
}

fn millis(value: u32) -> Duration {
    Duration::from_millis(u64::from(value))
}

/// One validated `[[principal]]` entry: a frontend identity that may log in.
///
/// The name has no accessor. The only way to use it is
/// [`crate::ValidatedConfig::resolve_principal`].
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run. A change needs a restart.
pub struct PrincipalConfig {
    id: PrincipalId,
    verifier: SecretReference,
    session_max_count: u32,
}

impl PrincipalConfig {
    /// The principal identifier. Logs may show it in place of the name.
    pub fn id(&self) -> &PrincipalId {
        &self.id
    }

    /// The reference to the SCRAM verifier of the principal, from
    /// `principal.verifier_file`. This crate never opens it.
    pub fn verifier(&self) -> &SecretReference {
        &self.verifier
    }

    /// The most sessions of this principal open at once, from
    /// `principal.session_max_count`, or `limits.session_max_count` when the
    /// key is left out.
    pub fn session_max_count(&self) -> u32 {
        self.session_max_count
    }
}

impl fmt::Debug for PrincipalConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrincipalConfig")
            .field("id", &self.id)
            .field("session_max_count", &self.session_max_count)
            .finish()
    }
}

/// The lookup of principals by name. Its `Debug` prints the number of
/// entries and nothing else.
pub(crate) struct PrincipalIndex(BTreeMap<String, usize>);

impl PrincipalIndex {
    pub(crate) fn get(&self, name: &str) -> Option<usize> {
        self.0.get(name).copied()
    }
}

impl fmt::Debug for PrincipalIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrincipalIndex")
            .field("entries", &self.0.len())
            .finish()
    }
}

// Raw shapes read from the file, written with the same macro as the tables
// of spec 0003: a map only, and no unknown key.

crate::topology::raw_table!(RawListener {
    host: Spanned<String>,
    port: Spanned<u16>,
    tls_certificate_file: Spanned<String>,
    tls_private_key_file: Spanned<String>,
});

crate::topology::raw_table!(RawLimits {
    session_max_count: Spanned<u32>,
    handshake_max_count: Spanned<u32>,
    backend_wait_max_count: Spanned<u32>,
    node_backend_max_count: Spanned<u32>,
    startup_timeout_ms: Spanned<u32>,
    backend_acquire_timeout_ms: Spanned<u32>,
    shutdown_timeout_ms: Spanned<u32>,
    startup_packet_max_bytes: Spanned<u32>,
    frontend_message_max_bytes: Spanned<u32>,
});

crate::topology::raw_table!(RawProbe {
    period_ms: Spanned<u32>,
    timeout_ms: Spanned<u32>,
    observation_expiry_ms: Spanned<u32>,
    node_loss_timeout_ms: Spanned<u32>,
});

crate::topology::raw_table!(RawPrincipal {
    id: Spanned<String>,
    name: Spanned<String>,
    verifier_file: Spanned<String>,
    session_max_count: Spanned<u32>,
});

/// The new top level tables as read from the file.
pub(crate) struct RawServing {
    pub(crate) listener: Option<RawListener>,
    pub(crate) limits: Option<RawLimits>,
    pub(crate) probe: Option<RawProbe>,
    pub(crate) principals: Vec<Spanned<RawPrincipal>>,
}

/// The three monitor keys of one cluster, when each that is present passed.
pub(crate) struct ClusterMonitor {
    pub(crate) login: Option<String>,
    pub(crate) database: String,
    pub(crate) credential: Option<SecretReference>,
}

/// The two TLS keys of one node, when each that is present passed. A server
/// name of `None` means the key was left out and the host stands in.
pub(crate) struct NodeTls {
    pub(crate) ca_file: Option<PathBuf>,
    pub(crate) server_name: Option<String>,
}

/// The monitor keys as read for one cluster entry.
pub(crate) struct RawClusterMonitor<'a> {
    pub(crate) login: Option<&'a Spanned<String>>,
    pub(crate) database: Option<&'a Spanned<String>>,
    pub(crate) credential_file: Option<&'a Spanned<String>>,
}

/// The TLS keys as read for one node entry.
pub(crate) struct RawNodeTls<'a> {
    pub(crate) ca_file: Option<&'a Spanned<String>>,
    pub(crate) server_name: Option<&'a Spanned<String>>,
}

struct PrincipalEntry {
    id: Option<At<PrincipalId>>,
    name: Option<At<String>>,
    verifier: Option<PathBuf>,
    /// `Some(None)` when the key is left out, `None` when it failed.
    session_max_count: Option<Option<u32>>,
}

/// The result of the checks of this module. The records are complete only
/// when no finding was added; the caller must not use them otherwise.
pub(crate) struct Serving {
    pub(crate) listener: Option<ListenerConfig>,
    pub(crate) limits: Limits,
    pub(crate) probe: ProbeConfig,
    pub(crate) monitors: Vec<Option<ClusterMonitor>>,
    pub(crate) node_tls: Vec<Option<NodeTls>>,
    pub(crate) principals: Vec<PrincipalConfig>,
    pub(crate) principal_index: PrincipalIndex,
}

/// What a number key held.
enum Number<T> {
    Absent,
    Valid(T),
    Invalid,
}

/// Checks a number key against its range.
fn number<T: Copy + PartialOrd>(raw: Option<&Spanned<T>>, range: &RangeInclusive<T>) -> Number<T> {
    match raw {
        None => Number::Absent,
        Some(value) if range.contains(value.get_ref()) => Number::Valid(*value.get_ref()),
        Some(_) => Number::Invalid,
    }
}

/// Adds the finding of a top level key, which has no position.
fn push_key(checker: &mut Checker<'_>, code: FindingCode, key: &'static str) {
    checker.findings.push(Finding {
        code,
        location: Location::Key(key),
    });
}

/// Checks a number key of a top level table and gives its value, the default
/// when it is absent, or `None` when it failed.
fn top_number(
    checker: &mut Checker<'_>,
    raw: Option<&Spanned<u32>>,
    key: &'static str,
    default: u32,
    range: RangeInclusive<u32>,
) -> Option<u32> {
    match number(raw, &range) {
        Number::Absent => Some(default),
        Number::Valid(value) => Some(value),
        Number::Invalid => {
            push_key(checker, FindingCode::InvalidValue, key);
            None
        }
    }
}

/// Checks a string key of a top level table that is required when the table
/// exists.
fn top_required<T>(
    checker: &mut Checker<'_>,
    raw: Option<&Spanned<String>>,
    key: &'static str,
    parse: impl Fn(&str) -> Option<T>,
) -> Option<T> {
    let Some(raw) = raw else {
        push_key(checker, FindingCode::MissingKey, key);
        return None;
    };
    let parsed = parse(raw.get_ref());
    if parsed.is_none() {
        push_key(checker, FindingCode::InvalidValue, key);
    }
    parsed
}

/// Runs the checks that spec 0004 adds to step 5 of the validation order,
/// after item 5 of spec 0003: `listener`, `limits`, `probe` with its cross
/// rule last, the monitor keys of each cluster, the TLS keys of each node,
/// then each principal.
pub(crate) fn check(
    checker: &mut Checker<'_>,
    raw: &RawServing,
    clusters: &[RawClusterMonitor<'_>],
    nodes: &[RawNodeTls<'_>],
) -> Serving {
    let listener = raw
        .listener
        .as_ref()
        .and_then(|listener| check_listener(checker, listener));
    let limits = check_limits(checker, raw.limits.as_ref());
    let probe = check_probe(checker, raw.probe.as_ref());
    let monitors = clusters
        .iter()
        .map(|cluster| check_monitor(checker, cluster))
        .collect();
    let node_tls = nodes
        .iter()
        .map(|node| check_node_tls(checker, node))
        .collect();
    let entries: Vec<PrincipalEntry> = raw
        .principals
        .iter()
        .map(|entry| check_principal(checker, entry))
        .collect();
    let (principals, principal_index) = build_principals(entries, limits.session_max_count);
    Serving {
        listener,
        limits,
        probe,
        monitors,
        node_tls,
        principals,
        principal_index,
    }
}

fn check_listener(checker: &mut Checker<'_>, raw: &RawListener) -> Option<ListenerConfig> {
    let host = top_required(checker, raw.host.as_ref(), KEY_LISTENER_HOST, parse_host);
    let port = match &raw.port {
        None => {
            push_key(checker, FindingCode::MissingKey, KEY_LISTENER_PORT);
            None
        }
        Some(port) if *port.get_ref() == 0 => {
            push_key(checker, FindingCode::InvalidValue, KEY_LISTENER_PORT);
            None
        }
        Some(port) => Some(*port.get_ref()),
    };
    let certificate = top_required(
        checker,
        raw.tls_certificate_file.as_ref(),
        KEY_LISTENER_CERTIFICATE,
        parse_path,
    );
    let private_key = top_required(
        checker,
        raw.tls_private_key_file.as_ref(),
        KEY_LISTENER_PRIVATE_KEY,
        parse_path,
    );
    Some(ListenerConfig {
        host: host?,
        port: port?,
        tls_certificate_file: certificate?,
        tls_private_key: SecretReference::new(private_key?),
    })
}

fn check_limits(checker: &mut Checker<'_>, raw: Option<&RawLimits>) -> Limits {
    let default = Limits::DEFAULT;
    let Some(raw) = raw else {
        return default;
    };
    // Each key in the order of the table under "Keys". A value that failed
    // keeps its default: the file is rejected, so it is never used.
    let mut take = |value: Option<&Spanned<u32>>,
                    key: &'static str,
                    fallback: u32,
                    range: RangeInclusive<u32>| {
        top_number(checker, value, key, fallback, range).unwrap_or(fallback)
    };
    Limits {
        session_max_count: take(
            raw.session_max_count.as_ref(),
            "limits.session_max_count",
            default.session_max_count,
            1..=100_000,
        ),
        handshake_max_count: take(
            raw.handshake_max_count.as_ref(),
            "limits.handshake_max_count",
            default.handshake_max_count,
            1..=10_000,
        ),
        backend_wait_max_count: take(
            raw.backend_wait_max_count.as_ref(),
            "limits.backend_wait_max_count",
            default.backend_wait_max_count,
            0..=100_000,
        ),
        node_backend_max_count: take(
            raw.node_backend_max_count.as_ref(),
            "limits.node_backend_max_count",
            default.node_backend_max_count,
            1..=100_000,
        ),
        startup_timeout_ms: take(
            raw.startup_timeout_ms.as_ref(),
            "limits.startup_timeout_ms",
            default.startup_timeout_ms,
            100..=60_000,
        ),
        backend_acquire_timeout_ms: take(
            raw.backend_acquire_timeout_ms.as_ref(),
            "limits.backend_acquire_timeout_ms",
            default.backend_acquire_timeout_ms,
            100..=60_000,
        ),
        shutdown_timeout_ms: take(
            raw.shutdown_timeout_ms.as_ref(),
            "limits.shutdown_timeout_ms",
            default.shutdown_timeout_ms,
            0..=600_000,
        ),
        startup_packet_max_bytes: take(
            raw.startup_packet_max_bytes.as_ref(),
            "limits.startup_packet_max_bytes",
            default.startup_packet_max_bytes,
            512..=65_536,
        ),
        frontend_message_max_bytes: take(
            raw.frontend_message_max_bytes.as_ref(),
            "limits.frontend_message_max_bytes",
            default.frontend_message_max_bytes,
            1024..=1_073_741_823,
        ),
    }
}

fn check_probe(checker: &mut Checker<'_>, raw: Option<&RawProbe>) -> ProbeConfig {
    let default = ProbeConfig::DEFAULT;
    let Some(raw) = raw else {
        return default;
    };
    let period = top_number(
        checker,
        raw.period_ms.as_ref(),
        KEY_PROBE_PERIOD,
        default.period_ms,
        // 30000 like the timeout, so that the period plus the timeout never
        // passes the largest expiry (spec 0004, decision 49).
        100..=30_000,
    );
    let timeout = top_number(
        checker,
        raw.timeout_ms.as_ref(),
        KEY_PROBE_TIMEOUT,
        default.timeout_ms,
        100..=30_000,
    );
    let expiry = top_number(
        checker,
        raw.observation_expiry_ms.as_ref(),
        KEY_PROBE_EXPIRY,
        default.observation_expiry_ms,
        1000..=60_000,
    );
    let node_loss = top_number(
        checker,
        raw.node_loss_timeout_ms.as_ref(),
        KEY_PROBE_NODE_LOSS,
        default.node_loss_timeout_ms,
        1000..=3_600_000,
    );
    // The cross rule comes last and reads only values that passed. An expiry
    // below the period plus the timeout could never see a fresh sample.
    if let (Some(period), Some(timeout), Some(expiry)) = (period, timeout, expiry)
        && u64::from(expiry) < u64::from(period) + u64::from(timeout)
    {
        push_key(checker, FindingCode::InvalidValue, KEY_PROBE_EXPIRY);
    }
    // Then the loss time (spec 0004, decision 109): below twice the period
    // plus the timeout, one probe that stumbles could end every session
    // before a second probe had asked and had its whole timeout.
    if let (Some(period), Some(timeout), Some(node_loss)) = (period, timeout, node_loss)
        && u64::from(node_loss) < 2 * u64::from(period) + u64::from(timeout)
    {
        push_key(checker, FindingCode::InvalidValue, KEY_PROBE_NODE_LOSS);
    }
    ProbeConfig {
        period_ms: period.unwrap_or(default.period_ms),
        timeout_ms: timeout.unwrap_or(default.timeout_ms),
        observation_expiry_ms: expiry.unwrap_or(default.observation_expiry_ms),
        node_loss_timeout_ms: node_loss.unwrap_or(default.node_loss_timeout_ms),
    }
}

/// Checks an optional string key of an entry: `Some(None)` when it is absent,
/// `None` when it failed.
fn entry_optional<T>(
    checker: &mut Checker<'_>,
    raw: Option<&Spanned<String>>,
    key: &'static str,
    parse: impl Fn(&str) -> Option<T>,
) -> Option<Option<T>> {
    match raw {
        None => Some(None),
        Some(raw) => checker
            .optional(raw, key, parse)
            .map(|found| Some(found.value)),
    }
}

fn check_monitor(checker: &mut Checker<'_>, raw: &RawClusterMonitor<'_>) -> Option<ClusterMonitor> {
    let login = entry_optional(checker, raw.login, KEY_CLUSTER_MONITOR_LOGIN, parse_name);
    let database = entry_optional(
        checker,
        raw.database,
        KEY_CLUSTER_MONITOR_DATABASE,
        parse_name,
    );
    let credential = entry_optional(
        checker,
        raw.credential_file,
        KEY_CLUSTER_MONITOR_CREDENTIAL,
        parse_path,
    );
    Some(ClusterMonitor {
        login: login?,
        database: database?.unwrap_or_else(|| DEFAULT_MONITOR_DATABASE.to_owned()),
        credential: credential?.map(SecretReference::new),
    })
}

fn check_node_tls(checker: &mut Checker<'_>, raw: &RawNodeTls<'_>) -> Option<NodeTls> {
    let ca_file = entry_optional(checker, raw.ca_file, KEY_NODE_TLS_CA, parse_path);
    let server_name = entry_optional(
        checker,
        raw.server_name,
        KEY_NODE_TLS_SERVER_NAME,
        parse_host,
    );
    Some(NodeTls {
        ca_file: ca_file?,
        server_name: server_name?,
    })
}

fn check_principal(checker: &mut Checker<'_>, entry: &Spanned<RawPrincipal>) -> PrincipalEntry {
    let at = checker.start(entry);
    let raw = entry.get_ref();
    let id = checker.required(raw.id.as_ref(), at, KEY_PRINCIPAL_ID, PrincipalId::parse);
    let name = checker.required(raw.name.as_ref(), at, KEY_PRINCIPAL_NAME, parse_name);
    let verifier = checker
        .required(
            raw.verifier_file.as_ref(),
            at,
            KEY_PRINCIPAL_VERIFIER,
            parse_path,
        )
        .map(|found| found.value);
    let session_max_count = match number(raw.session_max_count.as_ref(), &SESSION_MAX_RANGE) {
        Number::Absent => Some(None),
        Number::Valid(value) => Some(Some(value)),
        Number::Invalid => {
            if let Some(value) = &raw.session_max_count {
                let (line, column) = checker.start(value);
                checker.push(
                    FindingCode::InvalidValue,
                    KEY_PRINCIPAL_SESSION_MAX,
                    line,
                    column,
                );
            }
            None
        }
    };
    PrincipalEntry {
        id,
        name,
        verifier,
        session_max_count,
    }
}

/// The two cross checks that spec 0004 adds at the end of item 6:
/// `config.duplicate_id` at `principal.id`, then
/// `config.duplicate_principal` at `principal.name` of the second and each
/// later entry with the same name, compared byte for byte.
pub(crate) fn cross_check(checker: &mut Checker<'_>, raw: &RawServing) {
    let mut ids: HashSet<PrincipalId> = HashSet::new();
    let mut names: HashSet<&str> = HashSet::new();
    let mut repeated_names: Vec<(u32, u32)> = Vec::new();
    for entry in &raw.principals {
        let entry = entry.get_ref();
        if let Some(value) = &entry.id
            && let Some(id) = PrincipalId::parse(value.get_ref())
            && !ids.insert(id)
        {
            let (line, column) = checker.start(value);
            checker.push(FindingCode::DuplicateId, KEY_PRINCIPAL_ID, line, column);
        }
        if let Some(value) = &entry.name
            && parse_name(value.get_ref()).is_some()
            && !names.insert(value.get_ref().as_str())
        {
            repeated_names.push(checker.start(value));
        }
    }
    for (line, column) in repeated_names {
        checker.push(
            FindingCode::DuplicatePrincipal,
            KEY_PRINCIPAL_NAME,
            line,
            column,
        );
    }
}

/// Builds the principal records from entries whose every value passed.
fn build_principals(
    entries: Vec<PrincipalEntry>,
    default_session_max_count: u32,
) -> (Vec<PrincipalConfig>, PrincipalIndex) {
    let mut index = BTreeMap::new();
    let mut principals = Vec::with_capacity(entries.len());
    for entry in entries {
        let built = (|| {
            let name = entry.name?.value;
            let session_max_count = entry
                .session_max_count?
                .unwrap_or(default_session_max_count);
            let principal = PrincipalConfig {
                id: entry.id?.value,
                verifier: SecretReference::new(entry.verifier?),
                session_max_count,
            };
            Some((name, principal))
        })();
        if let Some((name, principal)) = built {
            index.entry(name).or_insert(principals.len());
            principals.push(principal);
        }
    }
    (principals, PrincipalIndex(index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_follow_the_table_of_keys() {
        // covers: 0004 AC-5.
        let limits = Limits::DEFAULT;
        assert_eq!(limits.session_max_count(), 1000);
        assert_eq!(limits.handshake_max_count(), 64);
        assert_eq!(limits.backend_wait_max_count(), 100);
        assert_eq!(limits.node_backend_max_count(), 100);
        assert_eq!(limits.startup_timeout(), Duration::from_millis(5000));
        assert_eq!(
            limits.backend_acquire_timeout(),
            Duration::from_millis(5000)
        );
        assert_eq!(limits.shutdown_timeout(), Duration::from_millis(30_000));
        assert_eq!(limits.startup_packet_max_bytes(), 8192);
        assert_eq!(limits.frontend_message_max_bytes(), 16_777_216);
        let probe = ProbeConfig::DEFAULT;
        assert_eq!(probe.period(), Duration::from_millis(2000));
        assert_eq!(probe.timeout(), Duration::from_millis(1000));
        assert_eq!(probe.observation_expiry(), Duration::from_millis(6000));
        assert_eq!(probe.node_loss_timeout(), Duration::from_millis(30_000));
    }

    #[test]
    fn principal_index_debug_prints_only_the_count() {
        let index = PrincipalIndex(BTreeMap::from([
            ("CANARY-9f2c7b1e".to_owned(), 0),
            ("other".to_owned(), 1),
        ]));
        assert_eq!(format!("{index:?}"), "PrincipalIndex { entries: 2 }");
        assert_eq!(index.get("CANARY-9f2c7b1e"), Some(0));
        assert_eq!(index.get("canary-9f2c7b1e"), None);
    }
}
