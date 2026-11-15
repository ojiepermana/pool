//! Offline validation of the POOL configuration file.
//!
//! [`validate`] is the only way to obtain a [`ValidatedConfig`]. It does no
//! input or output: the caller reads the file and passes its bytes. A file is
//! accepted or rejected as a whole. A [`Rejection`] holds only finding codes,
//! static key paths and numbers, so it can never carry text from the file.

use std::fmt;

mod generation;
mod identifier;
mod topology;

pub use generation::{AuthorityRevision, StartSequence, TopologyEpoch};
pub use identifier::{AuthorityId, ClusterId, IdentityId, NodeId, SystemIdentifier};
pub use topology::{
    ClusterConfig, ControlConfig, IdentityRoute, NodeConfig, RoutingProfile, SecretReference,
};

use toml::Spanned;
use topology::{IdentityIndex, RawCluster, RawControl, RawIdentity, RawNode, RawTopology};

use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};

/// The largest configuration file accepted, in bytes. A file of exactly this
/// size is still accepted.
pub const MAX_CONFIG_BYTES: usize = 1_048_576;

/// The only `config_version` this build supports.
pub const SUPPORTED_CONFIG_VERSION: u32 = 1;

const KEY_CONFIG_VERSION: &str = "config_version";
const KEY_RESULT_CACHE_ENABLED: &str = "result_cache.enabled";
const KEY_AUTOMATIC_PROMOTION_ENABLED: &str = "automatic_promotion.enabled";

/// A configuration that passed every validation step.
///
/// Restart rule: read and validated again from the file at each start.
/// Nothing is remembered from the last run.
///
/// It has no public constructor; [`validate`] returns one only when the file
/// produced no finding. Code outside this crate cannot build one:
///
/// ```compile_fail
/// let config = pool_config::ValidatedConfig {
///     config_version: 1,
///     result_cache: pool_config::CapabilityState::Disabled,
///     automatic_promotion: pool_config::CapabilityState::Disabled,
/// };
/// ```
///
/// ```compile_fail
/// let config: pool_config::ValidatedConfig = Default::default();
/// ```
#[derive(Debug)]
pub struct ValidatedConfig {
    config_version: u32,
    result_cache: CapabilityState,
    automatic_promotion: CapabilityState,
    control: Option<ControlConfig>,
    clusters: Vec<ClusterConfig>,
    nodes: Vec<NodeConfig>,
    identities: Vec<IdentityRoute>,
    identity_index: IdentityIndex,
}

impl ValidatedConfig {
    /// The value of the file's `config_version` key.
    pub fn config_version(&self) -> u32 {
        self.config_version
    }

    /// The state of the reserved `result_cache` capability.
    pub fn result_cache(&self) -> CapabilityState {
        self.result_cache
    }

    /// The state of the reserved `automatic_promotion` capability.
    pub fn automatic_promotion(&self) -> CapabilityState {
        self.automatic_promotion
    }

    /// The `[control]` table, or `None` when the file has none.
    pub fn control(&self) -> Option<&ControlConfig> {
        self.control.as_ref()
    }

    /// The `[[cluster]]` entries, in file order.
    pub fn clusters(&self) -> &[ClusterConfig] {
        &self.clusters
    }

    /// The `[[node]]` entries, in file order.
    pub fn nodes(&self) -> &[NodeConfig] {
        &self.nodes
    }

    /// The `[[identity]]` entries, in file order.
    pub fn identities(&self) -> &[IdentityRoute] {
        &self.identities
    }

    /// The route for an exact pair of verified principal and requested
    /// database, or `None` for every other pair.
    ///
    /// Both names are compared byte for byte: nothing is folded to lowercase
    /// or trimmed. There is no wildcard and no default entry. The lookup does
    /// not walk every entry.
    pub fn resolve_identity(&self, principal: &str, database: &str) -> Option<&IdentityRoute> {
        let index = self.identity_index.get(principal, database)?;
        self.identities.get(index)
    }
}

/// The state of a reserved capability in a validated configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityState {
    /// The capability is off. This is the only state this build accepts.
    Disabled,
}

impl CapabilityState {
    /// The stable name of the state, used in reports.
    pub fn as_str(self) -> &'static str {
        match self {
            CapabilityState::Disabled => "disabled",
        }
    }
}

/// The reasons a configuration file was rejected. It is never empty.
#[derive(Debug)]
pub struct Rejection {
    findings: Vec<Finding>,
}

impl Rejection {
    /// The findings, in the order they were found. Never empty.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    fn single(code: FindingCode, location: Location) -> Self {
        Rejection {
            findings: vec![Finding { code, location }],
        }
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, finding) in self.findings.iter().enumerate() {
            if index > 0 {
                f.write_str("\n")?;
            }
            write!(f, "{finding}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Rejection {}

/// One reason a configuration file was rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    code: FindingCode,
    location: Location,
}

impl Finding {
    /// The stable code of the finding.
    pub fn code(&self) -> FindingCode {
        self.code
    }

    /// Where in the file the finding applies.
    pub fn location(&self) -> Location {
        self.location
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code.as_str();
        let message = self.code.message();
        match self.location {
            Location::None => write!(f, "{code}: {message}"),
            Location::Key(key) => write!(f, "{code} at {key}: {message}"),
            Location::Position { line, column } => {
                write!(f, "{code} at line {line}, column {column}: {message}")
            }
            Location::KeyAt { key, line, column } => {
                write!(
                    f,
                    "{code} at {key}, line {line}, column {column}: {message}"
                )
            }
        }
    }
}

/// The stable codes of configuration findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingCode {
    /// The file is larger than [`MAX_CONFIG_BYTES`].
    TooLarge,
    /// The bytes are not valid UTF-8.
    NotUtf8,
    /// The text is not valid TOML.
    Syntax,
    /// A key is not in the schema, or a value has the wrong type or range.
    Schema,
    /// A required key is absent.
    MissingKey,
    /// `config_version` is not [`SUPPORTED_CONFIG_VERSION`].
    UnsupportedVersion,
    /// A reserved capability has `enabled = true`.
    UnsupportedCapability,
    /// A value has the right type but not the allowed syntax or range.
    InvalidValue,
    /// An `id` repeats an earlier `id` of the same table.
    DuplicateId,
    /// A key names a cluster or node that the file does not define, or
    /// `expected_writer` names a node of another cluster.
    UnknownReference,
    /// A node has the host and port of an earlier node.
    DuplicateEndpoint,
    /// An identity has the principal and database of an earlier identity.
    DuplicateIdentity,
    /// A value is known but not supported by this build.
    UnsupportedValue,
}

impl FindingCode {
    /// The stable code text, for example `config.syntax`.
    pub fn as_str(self) -> &'static str {
        match self {
            FindingCode::TooLarge => "config.too_large",
            FindingCode::NotUtf8 => "config.not_utf8",
            FindingCode::Syntax => "config.syntax",
            FindingCode::Schema => "config.schema",
            FindingCode::MissingKey => "config.missing_key",
            FindingCode::UnsupportedVersion => "config.unsupported_version",
            FindingCode::UnsupportedCapability => "config.unsupported_capability",
            FindingCode::InvalidValue => "config.invalid_value",
            FindingCode::DuplicateId => "config.duplicate_id",
            FindingCode::UnknownReference => "config.unknown_reference",
            FindingCode::DuplicateEndpoint => "config.duplicate_endpoint",
            FindingCode::DuplicateIdentity => "config.duplicate_identity",
            FindingCode::UnsupportedValue => "config.unsupported_value",
        }
    }

    /// The constant message for the code.
    ///
    /// The numbers in the texts repeat [`MAX_CONFIG_BYTES`] and
    /// [`SUPPORTED_CONFIG_VERSION`]; change both together.
    pub fn message(self) -> &'static str {
        match self {
            FindingCode::TooLarge => "the file is larger than 1048576 bytes",
            FindingCode::NotUtf8 => "the file is not valid UTF-8",
            FindingCode::Syntax => "the file is not valid TOML",
            FindingCode::Schema => "unknown key, or a value of the wrong type or range",
            FindingCode::MissingKey => "a required key is missing",
            FindingCode::UnsupportedVersion => "this build supports config_version 1 only",
            FindingCode::UnsupportedCapability => {
                "this capability is not supported by this build; remove the key or set it to false"
            }
            FindingCode::InvalidValue => "a value is not in the allowed form or range",
            FindingCode::DuplicateId => "an id is used more than once",
            FindingCode::UnknownReference => "a reference does not name a matching entry",
            FindingCode::DuplicateEndpoint => "two nodes have the same host and port",
            FindingCode::DuplicateIdentity => {
                "two identity entries have the same principal and database"
            }
            FindingCode::UnsupportedValue => "this value is not supported by this build",
        }
    }
}

/// Where a finding applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// The finding applies to the whole file.
    None,
    /// The finding applies to a schema key path, such as `result_cache.enabled`.
    Key(&'static str),
    /// The finding applies to a position in the text. Both numbers start at 1.
    Position {
        /// The line, counted in line feed characters.
        line: u32,
        /// The column, counted in Unicode scalar values.
        column: u32,
    },
    /// The finding applies to a key of one entry of a repeated table, such as
    /// `node.id`, at a position in the text. The position is where the value
    /// starts, or where the entry starts when the key is missing.
    KeyAt {
        /// The key path without an index, such as `node.id`.
        key: &'static str,
        /// The line, counted as for [`Location::Position`].
        line: u32,
        /// The column, counted as for [`Location::Position`].
        column: u32,
    },
}

// The structures read from the file accept a table only. A derived
// `Deserialize` also accepts a sequence that gives the fields by position, so
// `result_cache = [false]` would pass; these hand written visitors implement
// `visit_map` and nothing else, and every other shape is a type error with a
// span. Unknown keys are rejected, as with `deny_unknown_fields`. A key given
// twice never reaches them: the syntax step already refuses it.

const RAW_CONFIG_FIELDS: &[&str] = &[
    "config_version",
    "result_cache",
    "automatic_promotion",
    "control",
    "cluster",
    "node",
    "identity",
];
const RAW_CAPABILITY_FIELDS: &[&str] = &["enabled"];

struct RawConfig {
    config_version: Option<u32>,
    result_cache: Option<RawCapability>,
    automatic_promotion: Option<RawCapability>,
    control: Option<RawControl>,
    cluster: Option<Vec<Spanned<RawCluster>>>,
    node: Option<Vec<Spanned<RawNode>>>,
    identity: Option<Vec<Spanned<RawIdentity>>>,
}

impl<'de> Deserialize<'de> for RawConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RawConfigVisitor;

        impl<'de> Visitor<'de> for RawConfigVisitor {
            type Value = RawConfig;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a table")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<RawConfig, A::Error> {
                let mut raw = RawConfig {
                    config_version: None,
                    result_cache: None,
                    automatic_promotion: None,
                    control: None,
                    cluster: None,
                    node: None,
                    identity: None,
                };
                while let Some(field) = map.next_key_seed(KnownField(RAW_CONFIG_FIELDS))? {
                    match field {
                        0 => raw.config_version = Some(map.next_value()?),
                        1 => raw.result_cache = Some(map.next_value()?),
                        2 => raw.automatic_promotion = Some(map.next_value()?),
                        3 => raw.control = Some(map.next_value()?),
                        4 => raw.cluster = Some(map.next_value()?),
                        5 => raw.node = Some(map.next_value()?),
                        _ => raw.identity = Some(map.next_value()?),
                    }
                }
                Ok(raw)
            }
        }

        deserializer.deserialize_struct("RawConfig", RAW_CONFIG_FIELDS, RawConfigVisitor)
    }
}

struct RawCapability {
    enabled: Option<bool>,
}

impl<'de> Deserialize<'de> for RawCapability {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RawCapabilityVisitor;

        impl<'de> Visitor<'de> for RawCapabilityVisitor {
            type Value = RawCapability;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a table")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<RawCapability, A::Error> {
                let mut raw = RawCapability { enabled: None };
                while map
                    .next_key_seed(KnownField(RAW_CAPABILITY_FIELDS))?
                    .is_some()
                {
                    raw.enabled = Some(map.next_value()?);
                }
                Ok(raw)
            }
        }

        deserializer.deserialize_struct(
            "RawCapability",
            RAW_CAPABILITY_FIELDS,
            RawCapabilityVisitor,
        )
    }
}

/// Reads a table key and gives its index in the list of known keys. An
/// unknown key is an error raised on the key itself, so the parser reports
/// the key's position. The key text is used only for matching and is never
/// stored.
pub(crate) struct KnownField(pub(crate) &'static [&'static str]);

impl<'de> DeserializeSeed<'de> for KnownField {
    type Value = usize;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<usize, D::Error> {
        deserializer.deserialize_identifier(self)
    }
}

impl Visitor<'_> for KnownField {
    type Value = usize;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a known key")
    }

    fn visit_str<E: de::Error>(self, key: &str) -> Result<usize, E> {
        self.0
            .iter()
            .position(|known| *known == key)
            .ok_or_else(|| E::unknown_field(key, self.0))
    }
}

/// Validates the bytes of a configuration file.
///
/// Returns the [`ValidatedConfig`] when the file has no finding, otherwise a
/// [`Rejection`] with every finding the last step collected.
pub fn validate(bytes: &[u8]) -> Result<ValidatedConfig, Rejection> {
    if bytes.len() > MAX_CONFIG_BYTES {
        return Err(Rejection::single(FindingCode::TooLarge, Location::None));
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Err(Rejection::single(FindingCode::NotUtf8, Location::None));
    };
    if let Err(error) = text.parse::<toml::Table>() {
        return Err(Rejection::single(
            FindingCode::Syntax,
            error_location(text, &error),
        ));
    }
    let raw: RawConfig = match toml::from_str(text) {
        Ok(raw) => raw,
        Err(error) => {
            return Err(Rejection::single(
                FindingCode::Schema,
                error_location(text, &error),
            ));
        }
    };

    let mut findings = Vec::new();
    let mut config_version = SUPPORTED_CONFIG_VERSION;
    match raw.config_version {
        None => findings.push(Finding {
            code: FindingCode::MissingKey,
            location: Location::Key(KEY_CONFIG_VERSION),
        }),
        Some(version) if version != SUPPORTED_CONFIG_VERSION => findings.push(Finding {
            code: FindingCode::UnsupportedVersion,
            location: Location::Key(KEY_CONFIG_VERSION),
        }),
        Some(version) => config_version = version,
    }
    let result_cache = capability_state(
        raw.result_cache.as_ref(),
        KEY_RESULT_CACHE_ENABLED,
        &mut findings,
    );
    let automatic_promotion = capability_state(
        raw.automatic_promotion.as_ref(),
        KEY_AUTOMATIC_PROMOTION_ENABLED,
        &mut findings,
    );
    let topology = topology::check(
        text,
        RawTopology {
            control: raw.control,
            clusters: raw.cluster.unwrap_or_default(),
            nodes: raw.node.unwrap_or_default(),
            identities: raw.identity.unwrap_or_default(),
        },
        &mut findings,
    );

    if !findings.is_empty() {
        return Err(Rejection { findings });
    }
    Ok(ValidatedConfig {
        config_version,
        result_cache,
        automatic_promotion,
        control: topology.control,
        clusters: topology.clusters,
        nodes: topology.nodes,
        identities: topology.identities,
        identity_index: topology.identity_index,
    })
}

/// Records a finding when the capability asks to be enabled.
///
/// It returns `Disabled` even after recording that finding. That is safe only
/// because [`validate`] returns the findings, and no [`ValidatedConfig`], as
/// soon as any finding exists.
fn capability_state(
    raw: Option<&RawCapability>,
    key: &'static str,
    findings: &mut Vec<Finding>,
) -> CapabilityState {
    if raw.and_then(|capability| capability.enabled) == Some(true) {
        findings.push(Finding {
            code: FindingCode::UnsupportedCapability,
            location: Location::Key(key),
        });
    }
    CapabilityState::Disabled
}

/// Takes only the span from a parser error, never its text.
fn error_location(text: &str, error: &toml::de::Error) -> Location {
    match error.span() {
        Some(span) => position(text, span.start),
        None => Location::None,
    }
}

/// Converts a byte offset into a 1 based line and column.
///
/// The line is 1 plus the line feeds before the offset. The column is 1 plus
/// the Unicode scalar values between the start of that line and the offset.
fn position(text: &str, offset: usize) -> Location {
    let (line, column) = line_column(text, offset);
    Location::Position { line, column }
}

/// The 1 based line and column of a byte offset, counted as [`position`]
/// counts them.
pub(crate) fn line_column(text: &str, offset: usize) -> (u32, u32) {
    let bytes = text.as_bytes();
    let prefix = &bytes[..offset.min(bytes.len())];
    let line_feeds = prefix.iter().filter(|&&byte| byte == b'\n').count();
    let line_start = prefix
        .iter()
        .rposition(|&byte| byte == b'\n')
        .map_or(0, |index| index + 1);
    let scalars = prefix[line_start..]
        .iter()
        .filter(|&&byte| byte & 0xC0 != 0x80)
        .count();
    (
        to_u32(line_feeds.saturating_add(1)),
        to_u32(scalars.saturating_add(1)),
    )
}

fn to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_counts_lines_and_scalars() {
        assert_eq!(
            position("a\nbc", 3),
            Location::Position { line: 2, column: 2 }
        );
        assert_eq!(
            position("é = 1", 3),
            Location::Position { line: 1, column: 3 }
        );
        assert_eq!(
            position("a\r\nb", 3),
            Location::Position { line: 2, column: 1 }
        );
    }

    #[test]
    fn position_clamps_an_offset_past_the_end() {
        assert_eq!(
            position("ab", 10),
            Location::Position { line: 1, column: 3 }
        );
    }

    #[test]
    fn finding_display_has_three_shapes() {
        let none = Finding {
            code: FindingCode::TooLarge,
            location: Location::None,
        };
        let key = Finding {
            code: FindingCode::MissingKey,
            location: Location::Key(KEY_CONFIG_VERSION),
        };
        let at = Finding {
            code: FindingCode::Syntax,
            location: Location::Position { line: 2, column: 5 },
        };
        assert_eq!(
            none.to_string(),
            "config.too_large: the file is larger than 1048576 bytes"
        );
        assert_eq!(
            key.to_string(),
            "config.missing_key at config_version: a required key is missing"
        );
        assert_eq!(
            at.to_string(),
            "config.syntax at line 2, column 5: the file is not valid TOML"
        );
    }
}
