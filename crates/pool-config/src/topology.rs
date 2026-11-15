//! The keys `[control]`, `[[cluster]]`, `[[node]]` and `[[identity]]`: their
//! raw shapes, their validation and the records a validated file exposes.
//!
//! The contract is spec 0003, section "Configuration", with the keys and the
//! name rule that spec 0004 adds. No text read from the file reaches a
//! finding, and the records that hold such text print it as `[REDACTED]` in
//! their `Debug` output.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use toml::Spanned;

use crate::serving::{self, RawClusterMonitor, RawNodeTls, RawServing, Serving};
use crate::{
    AuthorityId, ClusterId, Finding, FindingCode, Findings, IdentityId, KnownField, LineIndex,
    Location, NodeId, SystemIdentifier,
};

const KEY_STATE_DIRECTORY: &str = "control.state_directory";
const KEY_AUTHORITY_ID: &str = "control.authority_id";
const KEY_CLUSTER_ID: &str = "cluster.id";
const KEY_CLUSTER_SYSTEM_IDENTIFIER: &str = "cluster.system_identifier";
const KEY_CLUSTER_EXPECTED_WRITER: &str = "cluster.expected_writer";
const KEY_NODE_ID: &str = "node.id";
const KEY_NODE_CLUSTER: &str = "node.cluster";
const KEY_NODE_HOST: &str = "node.host";
const KEY_NODE_PORT: &str = "node.port";
const KEY_IDENTITY_ID: &str = "identity.id";
const KEY_IDENTITY_PRINCIPAL: &str = "identity.principal";
const KEY_IDENTITY_DATABASE: &str = "identity.database";
const KEY_IDENTITY_CLUSTER: &str = "identity.cluster";
const KEY_IDENTITY_BACKEND_DATABASE: &str = "identity.backend_database";
const KEY_IDENTITY_BACKEND_LOGIN: &str = "identity.backend_login";
const KEY_IDENTITY_CREDENTIAL_FILE: &str = "identity.credential_file";
const KEY_IDENTITY_ROUTING_PROFILE: &str = "identity.routing_profile";

const MAX_NAME_BYTES: usize = 63;
const MAX_PATH_BYTES: usize = 4096;
const MAX_HOST_BYTES: usize = 253;

/// Stands in for text from the file in a hand written `Debug`.
pub(crate) struct Redacted;

impl fmt::Debug for Redacted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

/// The validated `[control]` table.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
pub struct ControlConfig {
    state_directory: Option<PathBuf>,
    authority_id: Option<AuthorityId>,
}

impl ControlConfig {
    /// The directory that holds the journal, from `control.state_directory`.
    /// This build does not open it.
    pub fn state_directory(&self) -> Option<&Path> {
        self.state_directory.as_deref()
    }

    /// The authority witness, from `control.authority_id`: the identity of
    /// the journal this configuration belongs to.
    pub fn authority_id(&self) -> Option<AuthorityId> {
        self.authority_id
    }
}

impl fmt::Debug for ControlConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ControlConfig")
            .field(
                "state_directory",
                &self.state_directory.as_ref().map(|_| Redacted),
            )
            .field("authority_id", &self.authority_id)
            .finish()
    }
}

/// One validated `[[cluster]]` entry.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
pub struct ClusterConfig {
    id: ClusterId,
    system_identifier: SystemIdentifier,
    expected_writer: NodeId,
    monitor_login: Option<String>,
    monitor_database: String,
    monitor_credential: Option<SecretReference>,
    entry: EntryAt,
}

impl ClusterConfig {
    /// The cluster identifier.
    pub fn id(&self) -> &ClusterId {
        &self.id
    }

    /// The PostgreSQL system identifier every node of the cluster must report.
    pub fn system_identifier(&self) -> SystemIdentifier {
        self.system_identifier
    }

    /// The operator's statement of which node is the writer. It names a node
    /// of this cluster.
    pub fn expected_writer(&self) -> &NodeId {
        &self.expected_writer
    }

    /// The login the probe uses on every node of the cluster, from
    /// `cluster.monitor_login`. It never appears in default telemetry.
    pub fn monitor_login(&self) -> Option<&str> {
        self.monitor_login.as_deref()
    }

    /// The database the probe opens, from `cluster.monitor_database`, or
    /// `postgres` when the key is left out.
    pub fn monitor_database(&self) -> &str {
        &self.monitor_database
    }

    /// The reference to the secret of the monitor login, from
    /// `cluster.monitor_credential_file`. This crate never opens it.
    pub fn monitor_credential(&self) -> Option<&SecretReference> {
        self.monitor_credential.as_ref()
    }

    /// Where the entry starts in the file.
    pub(crate) fn entry_at(&self) -> EntryAt {
        self.entry
    }
}

impl fmt::Debug for ClusterConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterConfig")
            .field("id", &self.id)
            .field("system_identifier", &self.system_identifier)
            .field("expected_writer", &self.expected_writer)
            .field(
                "monitor_login",
                &self.monitor_login.as_ref().map(|_| Redacted),
            )
            .field("monitor_database", &Redacted)
            .field("monitor_credential", &self.monitor_credential)
            .finish()
    }
}

/// The line and column where an entry of a repeated table starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EntryAt {
    pub(crate) line: u32,
    pub(crate) column: u32,
}

impl EntryAt {
    fn new((line, column): (u32, u32)) -> Self {
        EntryAt { line, column }
    }
}

/// One validated `[[node]]` entry: a configured PostgreSQL server.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
pub struct NodeConfig {
    id: NodeId,
    cluster: ClusterId,
    host: String,
    port: u16,
    tls_ca_file: Option<PathBuf>,
    tls_server_name: String,
    entry: EntryAt,
}

impl NodeConfig {
    /// The node identifier, unique among all nodes of the file.
    pub fn id(&self) -> &NodeId {
        &self.id
    }

    /// The cluster the node belongs to.
    pub fn cluster(&self) -> &ClusterId {
        &self.cluster
    }

    /// The host name or address. It never appears in default telemetry.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The TCP port. It never appears in default telemetry.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// The PEM file with the CA certificates that the node's certificate
    /// must chain to, from `node.tls_ca_file`. This crate never opens it.
    pub fn tls_ca_file(&self) -> Option<&Path> {
        self.tls_ca_file.as_deref()
    }

    /// The name the node's certificate must hold, from
    /// `node.tls_server_name`, or the host when the key is left out. It
    /// never appears in default telemetry.
    pub fn tls_server_name(&self) -> &str {
        &self.tls_server_name
    }

    /// Where the entry starts in the file.
    pub(crate) fn entry_at(&self) -> EntryAt {
        self.entry
    }
}

impl fmt::Debug for NodeConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeConfig")
            .field("id", &self.id)
            .field("cluster", &self.cluster)
            .field("host", &Redacted)
            .field("port", &Redacted)
            .field("tls_ca_file", &self.tls_ca_file.as_ref().map(|_| Redacted))
            .field("tls_server_name", &Redacted)
            .finish()
    }
}

/// One validated `[[identity]]` entry: the backend login, credential
/// reference and routing profile for one pair of verified principal and
/// requested database.
///
/// The principal and the database have no accessor. The only way to use
/// them is [`crate::ValidatedConfig::resolve_identity`].
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
pub struct IdentityRoute {
    id: IdentityId,
    principal: String,
    cluster: ClusterId,
    backend_database: String,
    backend_login: String,
    credential: SecretReference,
    routing_profile: RoutingProfile,
    entry: EntryAt,
}

impl IdentityRoute {
    /// The identity identifier. Telemetry may name it in place of the login.
    pub fn id(&self) -> &IdentityId {
        &self.id
    }

    /// The cluster the route sends sessions to.
    pub fn cluster(&self) -> &ClusterId {
        &self.cluster
    }

    /// The database to open on the backend. It is the requested database
    /// unless `backend_database` names another.
    pub fn backend_database(&self) -> &str {
        &self.backend_database
    }

    /// The login role to use on the backend.
    pub fn backend_login(&self) -> &str {
        &self.backend_login
    }

    /// The reference to the secret of the backend login.
    pub fn credential(&self) -> &SecretReference {
        &self.credential
    }

    /// The routing profile of sessions that use this route.
    pub fn routing_profile(&self) -> RoutingProfile {
        self.routing_profile
    }

    /// The principal name, for the views of this crate only.
    pub(crate) fn principal(&self) -> &str {
        &self.principal
    }

    /// Where the entry starts in the file.
    pub(crate) fn entry_at(&self) -> EntryAt {
        self.entry
    }
}

impl fmt::Debug for IdentityRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityRoute")
            .field("id", &self.id)
            .field("cluster", &self.cluster)
            .field("backend_database", &Redacted)
            .field("backend_login", &Redacted)
            .field("credential", &self.credential)
            .field("routing_profile", &self.routing_profile)
            .finish()
    }
}

/// A reference to a secret file, from `identity.credential_file`,
/// `cluster.monitor_credential_file`, `principal.verifier_file` or
/// `listener.tls_private_key_file`. This crate checks its form only and never
/// opens the file.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
pub struct SecretReference {
    path: PathBuf,
}

impl SecretReference {
    pub(crate) fn new(path: PathBuf) -> Self {
        SecretReference { path }
    }

    /// The path of the secret file. It never appears in default telemetry.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Debug for SecretReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretReference")
            .field("path", &Redacted)
            .finish()
    }
}

/// The routing profile of an identity route.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingProfile {
    /// Sessions go to the eligible primary of the cluster.
    Primary,
}

impl RoutingProfile {
    /// The stable name of the profile, as written in the file.
    pub fn as_str(self) -> &'static str {
        match self {
            RoutingProfile::Primary => "primary",
        }
    }
}

/// The lookup of the identity map: principal, then database, to the index of
/// the route. Its `Debug` prints the number of entries and nothing else.
pub(crate) struct IdentityIndex(BTreeMap<String, BTreeMap<String, usize>>);

impl IdentityIndex {
    pub(crate) fn get(&self, principal: &str, database: &str) -> Option<usize> {
        self.0.get(principal)?.get(database).copied()
    }
}

impl fmt::Debug for IdentityIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries: usize = self.0.values().map(BTreeMap::len).sum();
        f.debug_struct("IdentityIndex")
            .field("entries", &entries)
            .finish()
    }
}

/// The validated topology part of a configuration.
pub(crate) struct Topology {
    pub(crate) control: Option<ControlConfig>,
    pub(crate) clusters: Vec<ClusterConfig>,
    pub(crate) nodes: Vec<NodeConfig>,
    pub(crate) identities: Vec<IdentityRoute>,
    pub(crate) identity_index: IdentityIndex,
    pub(crate) serving: Serving,
}

// Raw shapes read from the file. Like the structures of spec 0001, each one
// that stands for a table accepts a map only and refuses unknown keys.

macro_rules! raw_table {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        pub(crate) struct $name {
            $(pub(crate) $field: Option<$ty>,)*
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                const FIELDS: &[&str] = &[$(stringify!($field)),*];

                struct RawVisitor;

                impl<'de> Visitor<'de> for RawVisitor {
                    type Value = $name;

                    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("a table")
                    }

                    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<$name, A::Error> {
                        let mut raw = $name { $($field: None,)* };
                        while let Some(index) = map.next_key_seed(KnownField(FIELDS))? {
                            let name = FIELDS.get(index).copied().unwrap_or_default();
                            $(
                                if name == stringify!($field) {
                                    raw.$field = Some(map.next_value()?);
                                    continue;
                                }
                            )*
                            return Err(de::Error::custom("unknown field"));
                        }
                        Ok(raw)
                    }
                }

                deserializer.deserialize_struct(stringify!($name), FIELDS, RawVisitor)
            }
        }
    };
}

pub(crate) use raw_table;

raw_table!(RawControl {
    state_directory: Spanned<String>,
    authority_id: Spanned<String>,
});

raw_table!(RawCluster {
    id: Spanned<String>,
    system_identifier: Spanned<String>,
    expected_writer: Spanned<String>,
    monitor_login: Spanned<String>,
    monitor_database: Spanned<String>,
    monitor_credential_file: Spanned<String>,
});

raw_table!(RawNode {
    id: Spanned<String>,
    cluster: Spanned<String>,
    host: Spanned<String>,
    port: Spanned<u16>,
    tls_ca_file: Spanned<String>,
    tls_server_name: Spanned<String>,
});

raw_table!(RawIdentity {
    id: Spanned<String>,
    principal: Spanned<String>,
    database: Spanned<String>,
    cluster: Spanned<String>,
    backend_database: Spanned<String>,
    backend_login: Spanned<String>,
    credential_file: Spanned<String>,
    routing_profile: Spanned<String>,
});

/// The topology tables as read from the file.
pub(crate) struct RawTopology {
    pub(crate) control: Option<RawControl>,
    pub(crate) clusters: Vec<Spanned<RawCluster>>,
    pub(crate) nodes: Vec<Spanned<RawNode>>,
    pub(crate) identities: Vec<Spanned<RawIdentity>>,
    pub(crate) serving: RawServing,
}

/// A value that passed its own check, with the position where it starts.
pub(crate) struct At<T> {
    pub(crate) value: T,
    pub(crate) line: u32,
    pub(crate) column: u32,
}

/// Collects findings and turns spans into positions.
pub(crate) struct Checker<'a> {
    positions: LineIndex<'a>,
    pub(crate) findings: &'a mut Findings,
}

impl Checker<'_> {
    pub(crate) fn push(&mut self, code: FindingCode, key: &'static str, line: u32, column: u32) {
        self.findings.push(Finding {
            code,
            location: Location::KeyAt { key, line, column },
        });
    }

    pub(crate) fn start<T>(&self, spanned: &Spanned<T>) -> (u32, u32) {
        self.positions.line_column(spanned.span().start)
    }

    /// Checks a required string key of an entry that starts at `entry`.
    pub(crate) fn required<T>(
        &mut self,
        raw: Option<&Spanned<String>>,
        entry: (u32, u32),
        key: &'static str,
        parse: impl Fn(&str) -> Option<T>,
    ) -> Option<At<T>> {
        let Some(raw) = raw else {
            self.push(FindingCode::MissingKey, key, entry.0, entry.1);
            return None;
        };
        self.optional(raw, key, parse)
    }

    /// Checks a string key that is present.
    pub(crate) fn optional<T>(
        &mut self,
        raw: &Spanned<String>,
        key: &'static str,
        parse: impl Fn(&str) -> Option<T>,
    ) -> Option<At<T>> {
        let (line, column) = self.start(raw);
        match parse(raw.get_ref()) {
            Some(value) => Some(At {
                value,
                line,
                column,
            }),
            None => {
                self.push(FindingCode::InvalidValue, key, line, column);
                None
            }
        }
    }
}

/// Name syntax: 1 to 63 bytes of UTF-8 with no refused character.
pub(crate) fn parse_name(text: &str) -> Option<String> {
    let fits = (1..=MAX_NAME_BYTES).contains(&text.len()) && !has_control(text);
    fits.then(|| text.to_owned())
}

/// Path syntax: 1 to 4096 bytes, starts with `/`, no refused character.
pub(crate) fn parse_path(text: &str) -> Option<PathBuf> {
    let fits = text.len() <= MAX_PATH_BYTES && text.starts_with('/') && !has_control(text);
    fits.then(|| PathBuf::from(text))
}

/// Host syntax: 1 to 253 bytes, each an ASCII letter, a digit, `.`, `-` or `:`.
pub(crate) fn parse_host(text: &str) -> Option<String> {
    let fits = (1..=MAX_HOST_BYTES).contains(&text.len())
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b':'));
    fits.then(|| text.to_owned())
}

/// The closed list of characters that no name and no path may hold (spec
/// 0004, "Names and paths"): the control characters U+0000 to U+001F and
/// U+007F to U+009F, the characters that change the direction of text, and
/// the line and paragraph separators. It needs no Unicode table.
fn has_control(text: &str) -> bool {
    text.chars().any(|scalar| {
        matches!(
            scalar,
            '\u{0}'..='\u{1f}'
                | '\u{7f}'..='\u{9f}'
                | '\u{61c}'
                | '\u{200e}'
                | '\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2066}'..='\u{2069}'
                | '\u{2028}'
                | '\u{2029}'
        )
    })
}

struct ClusterEntry {
    entry: EntryAt,
    id: Option<At<ClusterId>>,
    system_identifier: Option<SystemIdentifier>,
    expected_writer: Option<At<NodeId>>,
}

struct NodeEntry {
    entry: EntryAt,
    id: Option<At<NodeId>>,
    cluster: Option<At<ClusterId>>,
    host: Option<At<String>>,
    port: Option<u16>,
}

struct IdentityEntry {
    entry: EntryAt,
    id: Option<At<IdentityId>>,
    principal: Option<At<String>>,
    database: Option<String>,
    cluster: Option<At<ClusterId>>,
    backend_database: Option<String>,
    backend_login: Option<String>,
    credential_file: Option<PathBuf>,
    routing_profile: Option<RoutingProfile>,
}

/// Runs steps 2 to 6 of the validation order of spec 0003, with the checks
/// that spec 0004 adds after its item 5 and at the end of its item 6, and
/// returns the records. The records are complete only when no finding was
/// added; the caller must not use them otherwise.
pub(crate) fn check(text: &str, raw: RawTopology, findings: &mut Findings) -> Topology {
    let mut checker = Checker {
        positions: LineIndex::new(text),
        findings,
    };

    let control = raw
        .control
        .map(|raw_control| check_control(&mut checker, &raw_control, !raw.clusters.is_empty()));
    if control.is_none() && !raw.clusters.is_empty() {
        checker.findings.push(Finding {
            code: FindingCode::MissingKey,
            location: Location::Key(KEY_STATE_DIRECTORY),
        });
    }

    let clusters: Vec<ClusterEntry> = raw
        .clusters
        .iter()
        .map(|entry| check_cluster(&mut checker, entry))
        .collect();
    let nodes: Vec<NodeEntry> = raw
        .nodes
        .iter()
        .map(|entry| check_node(&mut checker, entry))
        .collect();
    let identities: Vec<IdentityEntry> = raw
        .identities
        .iter()
        .map(|entry| check_identity(&mut checker, entry))
        .collect();

    let monitors: Vec<RawClusterMonitor<'_>> = raw
        .clusters
        .iter()
        .map(|entry| {
            let raw = entry.get_ref();
            RawClusterMonitor {
                login: raw.monitor_login.as_ref(),
                database: raw.monitor_database.as_ref(),
                credential_file: raw.monitor_credential_file.as_ref(),
            }
        })
        .collect();
    let node_tls: Vec<RawNodeTls<'_>> = raw
        .nodes
        .iter()
        .map(|entry| {
            let raw = entry.get_ref();
            RawNodeTls {
                ca_file: raw.tls_ca_file.as_ref(),
                server_name: raw.tls_server_name.as_ref(),
            }
        })
        .collect();
    let serving = serving::check(&mut checker, &raw.serving, &monitors, &node_tls);

    cross_check(&mut checker, &clusters, &nodes, &identities);
    serving::cross_check(&mut checker, &raw.serving);
    build(control, clusters, nodes, identities, serving)
}

fn check_control(checker: &mut Checker<'_>, raw: &RawControl, has_cluster: bool) -> ControlConfig {
    let state_directory = match &raw.state_directory {
        None => {
            if has_cluster {
                checker.findings.push(Finding {
                    code: FindingCode::MissingKey,
                    location: Location::Key(KEY_STATE_DIRECTORY),
                });
            }
            None
        }
        Some(value) => {
            let parsed = parse_path(value.get_ref());
            if parsed.is_none() {
                checker.findings.push(Finding {
                    code: FindingCode::InvalidValue,
                    location: Location::Key(KEY_STATE_DIRECTORY),
                });
            }
            parsed
        }
    };
    let authority_id = raw.authority_id.as_ref().and_then(|value| {
        let parsed = AuthorityId::parse(value.get_ref());
        if parsed.is_none() {
            checker.findings.push(Finding {
                code: FindingCode::InvalidValue,
                location: Location::Key(KEY_AUTHORITY_ID),
            });
        }
        parsed
    });
    ControlConfig {
        state_directory,
        authority_id,
    }
}

fn check_cluster(checker: &mut Checker<'_>, entry: &Spanned<RawCluster>) -> ClusterEntry {
    let at = checker.start(entry);
    let raw = entry.get_ref();
    ClusterEntry {
        entry: EntryAt::new(at),
        id: checker.required(raw.id.as_ref(), at, KEY_CLUSTER_ID, ClusterId::parse),
        system_identifier: checker
            .required(
                raw.system_identifier.as_ref(),
                at,
                KEY_CLUSTER_SYSTEM_IDENTIFIER,
                SystemIdentifier::parse,
            )
            .map(|found| found.value),
        expected_writer: checker.required(
            raw.expected_writer.as_ref(),
            at,
            KEY_CLUSTER_EXPECTED_WRITER,
            NodeId::parse,
        ),
    }
}

fn check_node(checker: &mut Checker<'_>, entry: &Spanned<RawNode>) -> NodeEntry {
    let at = checker.start(entry);
    let raw = entry.get_ref();
    let id = checker.required(raw.id.as_ref(), at, KEY_NODE_ID, NodeId::parse);
    let cluster = checker.required(raw.cluster.as_ref(), at, KEY_NODE_CLUSTER, ClusterId::parse);
    let host = checker.required(raw.host.as_ref(), at, KEY_NODE_HOST, parse_host);
    let port = match &raw.port {
        None => {
            checker.push(FindingCode::MissingKey, KEY_NODE_PORT, at.0, at.1);
            None
        }
        Some(port) if *port.get_ref() == 0 => {
            let (line, column) = checker.start(port);
            checker.push(FindingCode::InvalidValue, KEY_NODE_PORT, line, column);
            None
        }
        Some(port) => Some(*port.get_ref()),
    };
    NodeEntry {
        entry: EntryAt::new(at),
        id,
        cluster,
        host,
        port,
    }
}

fn check_identity(checker: &mut Checker<'_>, entry: &Spanned<RawIdentity>) -> IdentityEntry {
    let at = checker.start(entry);
    let raw = entry.get_ref();
    let id = checker.required(raw.id.as_ref(), at, KEY_IDENTITY_ID, IdentityId::parse);
    let principal = checker.required(
        raw.principal.as_ref(),
        at,
        KEY_IDENTITY_PRINCIPAL,
        parse_name,
    );
    let database = checker
        .required(raw.database.as_ref(), at, KEY_IDENTITY_DATABASE, parse_name)
        .map(|found| found.value);
    let cluster = checker.required(
        raw.cluster.as_ref(),
        at,
        KEY_IDENTITY_CLUSTER,
        ClusterId::parse,
    );
    let backend_database = match &raw.backend_database {
        None => database.clone(),
        Some(value) => checker
            .optional(value, KEY_IDENTITY_BACKEND_DATABASE, parse_name)
            .map(|found| found.value),
    };
    let backend_login = checker
        .required(
            raw.backend_login.as_ref(),
            at,
            KEY_IDENTITY_BACKEND_LOGIN,
            parse_name,
        )
        .map(|found| found.value);
    let credential_file = checker
        .required(
            raw.credential_file.as_ref(),
            at,
            KEY_IDENTITY_CREDENTIAL_FILE,
            parse_path,
        )
        .map(|found| found.value);
    let routing_profile = match &raw.routing_profile {
        None => Some(RoutingProfile::Primary),
        Some(value) => match value.get_ref().as_str() {
            "primary" => Some(RoutingProfile::Primary),
            other => {
                let code = if other == "replica" {
                    FindingCode::UnsupportedValue
                } else {
                    FindingCode::InvalidValue
                };
                let (line, column) = checker.start(value);
                checker.push(code, KEY_IDENTITY_ROUTING_PROFILE, line, column);
                None
            }
        },
    };
    IdentityEntry {
        entry: EntryAt::new(at),
        id,
        principal,
        database,
        cluster,
        backend_database,
        backend_login,
        credential_file,
        routing_profile,
    }
}

/// Step 6 of the validation order: the cross checks, in their order. Each
/// reads only values that passed their own check.
fn cross_check(
    checker: &mut Checker<'_>,
    clusters: &[ClusterEntry],
    nodes: &[NodeEntry],
    identities: &[IdentityEntry],
) {
    // 1. Duplicate ids, at the id of the second and each later entry.
    duplicate_ids(
        checker,
        clusters.iter().map(|entry| entry.id.as_ref()),
        KEY_CLUSTER_ID,
    );
    duplicate_ids(
        checker,
        nodes.iter().map(|entry| entry.id.as_ref()),
        KEY_NODE_ID,
    );
    duplicate_ids(
        checker,
        identities.iter().map(|entry| entry.id.as_ref()),
        KEY_IDENTITY_ID,
    );

    let every_cluster_id_valid = clusters.iter().all(|entry| entry.id.is_some());
    let cluster_ids: HashSet<&ClusterId> = clusters
        .iter()
        .filter_map(|entry| entry.id.as_ref().map(|found| &found.value))
        .collect();

    // 2. A node names a cluster the file does not define.
    let before = checker.findings.total();
    if every_cluster_id_valid {
        for cluster in nodes.iter().filter_map(|entry| entry.cluster.as_ref()) {
            if !cluster_ids.contains(&cluster.value) {
                checker.push(
                    FindingCode::UnknownReference,
                    KEY_NODE_CLUSTER,
                    cluster.line,
                    cluster.column,
                );
            }
        }
    }
    let node_cluster_found_something = checker.findings.total() > before;

    // 3. `expected_writer` names no node of its own cluster. When an id
    // repeats, the reference resolves to the first entry.
    let every_node_complete = nodes
        .iter()
        .all(|entry| entry.id.is_some() && entry.cluster.is_some());
    if every_node_complete && !node_cluster_found_something {
        let mut node_clusters: HashMap<&NodeId, &ClusterId> = HashMap::new();
        for entry in nodes {
            if let (Some(id), Some(cluster)) = (&entry.id, &entry.cluster) {
                node_clusters.entry(&id.value).or_insert(&cluster.value);
            }
        }
        for entry in clusters {
            // A cluster whose own id failed its check has already given its
            // finding; judging its writer would give a second one.
            if let (Some(id), Some(writer)) = (&entry.id, &entry.expected_writer)
                && node_clusters.get(&writer.value) != Some(&&id.value)
            {
                checker.push(
                    FindingCode::UnknownReference,
                    KEY_CLUSTER_EXPECTED_WRITER,
                    writer.line,
                    writer.column,
                );
            }
        }
    }

    // 4. An identity names a cluster the file does not define.
    if every_cluster_id_valid {
        for cluster in identities.iter().filter_map(|entry| entry.cluster.as_ref()) {
            if !cluster_ids.contains(&cluster.value) {
                checker.push(
                    FindingCode::UnknownReference,
                    KEY_IDENTITY_CLUSTER,
                    cluster.line,
                    cluster.column,
                );
            }
        }
    }

    // 5. Two nodes with the same host, without regard to ASCII letter case,
    // and the same port.
    let mut endpoints: HashSet<(String, u16)> = HashSet::new();
    for entry in nodes {
        if let (Some(host), Some(port)) = (&entry.host, entry.port)
            && !endpoints.insert((host.value.to_ascii_lowercase(), port))
        {
            checker.push(
                FindingCode::DuplicateEndpoint,
                KEY_NODE_HOST,
                host.line,
                host.column,
            );
        }
    }

    // 6. Two identities with the same principal and database, byte for byte.
    let mut pairs: HashSet<(&str, &str)> = HashSet::new();
    for entry in identities {
        if let (Some(principal), Some(database)) = (&entry.principal, &entry.database)
            && !pairs.insert((principal.value.as_str(), database.as_str()))
        {
            checker.push(
                FindingCode::DuplicateIdentity,
                KEY_IDENTITY_PRINCIPAL,
                principal.line,
                principal.column,
            );
        }
    }
}

fn duplicate_ids<'a, T: Eq + std::hash::Hash + 'a>(
    checker: &mut Checker<'_>,
    ids: impl Iterator<Item = Option<&'a At<T>>>,
    key: &'static str,
) {
    let mut seen: HashSet<&T> = HashSet::new();
    for id in ids.flatten() {
        if !seen.insert(&id.value) {
            checker.push(FindingCode::DuplicateId, key, id.line, id.column);
        }
    }
}

/// Builds the records from entries whose every value passed. An entry with a
/// value that failed is left out; that can happen only when a finding exists.
fn build(
    control: Option<ControlConfig>,
    clusters: Vec<ClusterEntry>,
    nodes: Vec<NodeEntry>,
    identities: Vec<IdentityEntry>,
    mut serving: Serving,
) -> Topology {
    let monitors = std::mem::take(&mut serving.monitors);
    let clusters = clusters
        .into_iter()
        .zip(monitors)
        .filter_map(|(entry, monitor)| {
            let monitor = monitor?;
            Some(ClusterConfig {
                id: entry.id?.value,
                system_identifier: entry.system_identifier?,
                expected_writer: entry.expected_writer?.value,
                monitor_login: monitor.login,
                monitor_database: monitor.database,
                monitor_credential: monitor.credential,
                entry: entry.entry,
            })
        })
        .collect();
    let node_tls = std::mem::take(&mut serving.node_tls);
    let nodes = nodes
        .into_iter()
        .zip(node_tls)
        .filter_map(|(entry, tls)| {
            let tls = tls?;
            let host = entry.host?.value;
            Some(NodeConfig {
                id: entry.id?.value,
                cluster: entry.cluster?.value,
                tls_server_name: tls.server_name.unwrap_or_else(|| host.clone()),
                host,
                port: entry.port?,
                tls_ca_file: tls.ca_file,
                entry: entry.entry,
            })
        })
        .collect();
    let mut index: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut routes = Vec::with_capacity(identities.len());
    for entry in identities {
        let route = (|| {
            let principal = entry.principal?.value;
            let database = entry.database?;
            let route = IdentityRoute {
                id: entry.id?.value,
                principal: principal.clone(),
                cluster: entry.cluster?.value,
                backend_database: entry.backend_database?,
                backend_login: entry.backend_login?,
                credential: SecretReference::new(entry.credential_file?),
                routing_profile: entry.routing_profile?,
                entry: entry.entry,
            };
            Some((principal, database, route))
        })();
        if let Some((principal, database, route)) = route {
            index
                .entry(principal)
                .or_default()
                .insert(database, routes.len());
            routes.push(route);
        }
    }
    Topology {
        control,
        clusters,
        nodes,
        identities: routes,
        identity_index: IdentityIndex(index),
        serving,
    }
}

#[cfg(test)]
mod edge_tests {
    //! Edges of the value syntaxes, added by /test (0003 AC-2, AC-4).

    use super::*;

    #[test]
    fn ac2_name_syntax_counts_bytes_and_refuses_only_the_listed_characters() {
        assert_eq!(parse_name(&"a".repeat(63)).map(|n| n.len()), Some(63));
        assert_eq!(parse_name(&"a".repeat(64)), None);
        assert_eq!(parse_name(""), None);
        assert_eq!(parse_name(&"\u{e9}".repeat(32)), None);
        assert!(parse_name(&format!("a{}", "\u{e9}".repeat(31))).is_some());
        // covers: 0004 AC-6. The closed list, written out rather than taken
        // from the code.
        let refused = (0_u32..=0x1f)
            .chain(0x7f..=0x9f)
            .chain([0x61c, 0x200e, 0x200f])
            .chain(0x202a..=0x202e)
            .chain(0x2066..=0x2069)
            .chain([0x2028, 0x2029]);
        for scalar in refused {
            let text = format!("a{}b", char::from_u32(scalar).expect("scalar"));
            assert_eq!(parse_name(&text), None, "{scalar:#x}");
            assert_eq!(parse_path(&format!("/{text}")), None, "{scalar:#x}");
        }
        // The neighbours of each range stay allowed.
        for text in [" ", " a ", "\u{e9}"] {
            assert_eq!(parse_name(text), Some(text.to_owned()), "{text:?}");
        }
        for scalar in [
            0xa0_u32, 0x61b, 0x61d, 0x200d, 0x2010, 0x2027, 0x202f, 0x2065, 0x206a,
        ] {
            let text = char::from_u32(scalar).expect("scalar").to_string();
            assert!(parse_name(&text).is_some(), "{scalar:#x}");
            assert!(parse_path(&format!("/{text}")).is_some(), "{scalar:#x}");
        }
    }

    #[test]
    fn ac2_path_syntax_counts_bytes_and_needs_a_leading_slash() {
        assert!(parse_path("/").is_some());
        assert!(parse_path(&format!("/{}", "p".repeat(4095))).is_some());
        assert_eq!(parse_path(&format!("/{}", "p".repeat(4096))), None);
        assert_eq!(parse_path(&format!("/{}", "\u{e9}".repeat(2048))), None);
        for text in [
            "", "p", "./p", "~/p", " /p", "\\p", "/p\n", "/p\u{7f}", "/p\0",
        ] {
            assert_eq!(parse_path(text), None, "{text:?}");
        }
    }

    #[test]
    fn ac2_host_syntax_counts_bytes_and_allows_only_its_characters() {
        assert!(parse_host(&"a".repeat(253)).is_some());
        assert_eq!(parse_host(&"a".repeat(254)), None);
        assert_eq!(parse_host(""), None);
        for text in ["::1", "a.b-c:d", "A.B", "1", "."] {
            assert!(parse_host(text).is_some(), "{text:?}");
        }
        // The allowed bytes, written out rather than taken from the code.
        let allowed_bytes = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.-:";
        for byte in 0_u8..=0x7f {
            let allowed = allowed_bytes.contains(&byte);
            let text = format!("a{}b", char::from(byte));
            assert_eq!(parse_host(&text).is_some(), allowed, "{byte:#x}");
        }
        assert_eq!(parse_host("d\u{e9}"), None);
    }

    #[test]
    fn ac4_identity_index_debug_prints_only_the_count() {
        let mut map = BTreeMap::new();
        map.insert(
            "CANARY-9f2c7b1e".to_owned(),
            BTreeMap::from([("CANARY-9f2c7b1e".to_owned(), 0), ("db".to_owned(), 1)]),
        );
        map.insert("other".to_owned(), BTreeMap::from([("db".to_owned(), 2)]));
        let index = IdentityIndex(map);
        assert_eq!(format!("{index:?}"), "IdentityIndex { entries: 3 }");
        assert_eq!(index.get("CANARY-9f2c7b1e", "db"), Some(1));
        assert_eq!(index.get("other", "CANARY-9f2c7b1e"), None);
        assert_eq!(index.get("db", "other"), None);
    }
}
