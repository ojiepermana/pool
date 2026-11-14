//! Every fixture and in memory case through `pool_config::validate`
//! (spec 0001 AC-4 to AC-9).

use pool_config::{
    CapabilityState, FindingCode, Location, MAX_CONFIG_BYTES, Rejection, ValidatedConfig, validate,
};

const CANARY: &str = "CANARY-9f2c7b1e";

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn accepted(bytes: &[u8]) -> ValidatedConfig {
    match validate(bytes) {
        Ok(config) => config,
        Err(rejection) => panic!("expected valid, got {rejection}"),
    }
}

fn rejected(bytes: &[u8]) -> Rejection {
    match validate(bytes) {
        Ok(config) => panic!("expected a rejection, got {config:?}"),
        Err(rejection) => rejection,
    }
}

/// The findings as (code, location), for exact comparison.
fn findings(bytes: &[u8]) -> Vec<(FindingCode, Location)> {
    rejected(bytes)
        .findings()
        .iter()
        .map(|finding| (finding.code(), finding.location()))
        .collect()
}

/// Checks a single positioned finding: code, exact line, column at least 1.
fn assert_positioned(bytes: &[u8], code: FindingCode, line: u32) {
    let found = findings(bytes);
    assert_eq!(found.len(), 1, "{found:?}");
    let (found_code, location) = found[0];
    assert_eq!(found_code, code);
    match location {
        Location::Position {
            line: found_line,
            column,
        } => {
            assert_eq!(found_line, line, "{location:?}");
            assert!(column >= 1, "{location:?}");
        }
        other => panic!("expected a position, got {other:?}"),
    }
}

fn assert_disabled(config: &ValidatedConfig) {
    assert_eq!(config.config_version(), 1);
    assert_eq!(config.result_cache(), CapabilityState::Disabled);
    assert_eq!(config.automatic_promotion(), CapabilityState::Disabled);
    assert_eq!(config.result_cache().as_str(), "disabled");
}

#[test]
fn valid_fixtures_are_accepted_with_capabilities_disabled() {
    for name in [
        "valid-minimal.toml",
        "valid-capabilities-disabled.toml",
        "valid-capabilities-empty.toml",
    ] {
        assert_disabled(&accepted(&fixture(name)));
    }
}

#[test]
fn result_cache_enabled_is_refused() {
    assert_eq!(
        findings(&fixture("invalid-result-cache-enabled.toml")),
        [(
            FindingCode::UnsupportedCapability,
            Location::Key("result_cache.enabled")
        )]
    );
}

#[test]
fn automatic_promotion_enabled_is_refused() {
    assert_eq!(
        findings(&fixture("invalid-automatic-promotion-enabled.toml")),
        [(
            FindingCode::UnsupportedCapability,
            Location::Key("automatic_promotion.enabled")
        )]
    );
}

#[test]
fn enabled_true_is_refused_in_every_table_spelling() {
    for text in [
        "config_version = 1\nresult_cache.enabled = true\n",
        "config_version = 1\nresult_cache = { enabled = true }\n",
    ] {
        assert_eq!(
            findings(text.as_bytes()),
            [(
                FindingCode::UnsupportedCapability,
                Location::Key("result_cache.enabled")
            )],
            "{text:?}"
        );
    }
}

#[test]
fn multiple_findings_are_reported_in_order() {
    assert_eq!(
        findings(&fixture("invalid-multiple.toml")),
        [
            (
                FindingCode::UnsupportedVersion,
                Location::Key("config_version")
            ),
            (
                FindingCode::UnsupportedCapability,
                Location::Key("result_cache.enabled")
            ),
            (
                FindingCode::UnsupportedCapability,
                Location::Key("automatic_promotion.enabled")
            ),
        ]
    );
}

#[test]
fn other_invalid_fixtures_have_the_listed_finding() {
    assert_positioned(&fixture("invalid-syntax.toml"), FindingCode::Syntax, 2);
    assert_positioned(&fixture("invalid-unknown-key.toml"), FindingCode::Schema, 3);
    assert_positioned(&fixture("invalid-wrong-type.toml"), FindingCode::Schema, 4);
    for name in ["invalid-missing-version.toml", "invalid-empty.toml"] {
        assert_eq!(
            findings(&fixture(name)),
            [(FindingCode::MissingKey, Location::Key("config_version"))],
            "{name}"
        );
    }
    assert_eq!(
        findings(&fixture("invalid-unsupported-version.toml")),
        [(
            FindingCode::UnsupportedVersion,
            Location::Key("config_version")
        )]
    );
}

#[test]
fn crlf_line_endings_give_the_same_lines() {
    for (name, line) in [
        ("invalid-unknown-key.toml", 3),
        ("invalid-wrong-type.toml", 4),
    ] {
        let text = String::from_utf8(fixture(name)).expect("fixture is UTF-8");
        let crlf = text.replace('\n', "\r\n");
        assert_positioned(crlf.as_bytes(), FindingCode::Schema, line);
    }
}

#[test]
fn bad_values_get_the_listed_code() {
    for version in ["0", "2", "4294967295"] {
        let text = format!("config_version = {version}\n");
        assert_eq!(
            findings(text.as_bytes()),
            [(
                FindingCode::UnsupportedVersion,
                Location::Key("config_version")
            )],
            "{text:?}"
        );
    }
    for text in [
        "config_version = -1\n",
        "config_version = 4294967296\n",
        "config_version = 9223372036854775807\n",
        "config_version = 1.0\n",
        "config_version = \"1\"\n",
        "config_version = true\n",
        "config_version = [1]\n",
        "config_version = 1\n[result_cache]\nenabled = 1\n",
        "config_version = 1\nresult_cache = 1\n",
        "config_version = 1\nautomatic_promotion = \"x\"\n",
        "config_version = 1\n[result_cache]\nmode = \"on\"\n",
        "config_version = 1\n[connection_pool]\n",
    ] {
        assert_eq!(rejected(text.as_bytes()).findings().len(), 1, "{text:?}");
        assert_eq!(
            rejected(text.as_bytes()).findings()[0].code(),
            FindingCode::Schema,
            "{text:?}"
        );
    }
    for text in [
        "config_version = 9223372036854775808\n",
        "config_version = 1\nconfig_version = 1\n",
        "config_version = 1\nresult_cache.enabled = false\nresult_cache.enabled = false\n",
        "config_version = 1\nresult_cache = { enabled = false }\nresult_cache = { enabled = false }\n",
        "config_version = 1\n[result_cache]\nenabled = false\nenabled = false\n",
        "config_version = 1\n[result_cache]\n[result_cache]\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Syntax, "{text:?}");
    }
    for text in ["", "# only a comment\n# and another\n"] {
        assert_eq!(
            findings(text.as_bytes()),
            [(FindingCode::MissingKey, Location::Key("config_version"))],
            "{text:?}"
        );
    }
}

#[test]
fn not_utf8_is_refused() {
    assert_eq!(
        findings(&[0xFF, 0xFE, 0xFD]),
        [(FindingCode::NotUtf8, Location::None)]
    );
}

#[test]
fn size_limit_is_inclusive() {
    let too_large = vec![b'#'; MAX_CONFIG_BYTES + 1];
    assert_eq!(
        findings(&too_large),
        [(FindingCode::TooLarge, Location::None)]
    );

    let mut at_limit = vec![b'#'; 1_048_556];
    at_limit.extend_from_slice(b"\nconfig_version = 1\n");
    assert_eq!(at_limit.len(), MAX_CONFIG_BYTES);
    assert_disabled(&accepted(&at_limit));
}

#[test]
fn secret_fixtures_never_reach_the_rejection_text() {
    for (name, code, line) in [
        ("secret-unknown-key.toml", FindingCode::Schema, 2),
        ("secret-wrong-type.toml", FindingCode::Schema, 1),
        ("secret-syntax.toml", FindingCode::Syntax, 2),
    ] {
        let bytes = fixture(name);
        assert_positioned(&bytes, code, line);
        let rejection = rejected(&bytes);
        let debug = format!("{rejection:?}");
        let display = rejection.to_string();
        assert!(!debug.contains(CANARY), "{name}: {debug}");
        assert!(!display.contains(CANARY), "{name}: {display}");
    }
}

#[test]
fn rejection_display_joins_finding_lines() {
    let rejection = rejected(&fixture("invalid-multiple.toml"));
    assert_eq!(
        rejection.to_string(),
        "config.unsupported_version at config_version: this build supports config_version 1 only\n\
         config.unsupported_capability at result_cache.enabled: this capability is not supported by this build; remove the key or set it to false\n\
         config.unsupported_capability at automatic_promotion.enabled: this capability is not supported by this build; remove the key or set it to false"
    );
    let error: &dyn std::error::Error = &rejection;
    assert!(error.source().is_none());
}

#[test]
fn finding_codes_and_messages_are_stable() {
    let table = [
        (
            FindingCode::TooLarge,
            "config.too_large",
            "the file is larger than 1048576 bytes",
        ),
        (
            FindingCode::NotUtf8,
            "config.not_utf8",
            "the file is not valid UTF-8",
        ),
        (
            FindingCode::Syntax,
            "config.syntax",
            "the file is not valid TOML",
        ),
        (
            FindingCode::Schema,
            "config.schema",
            "unknown key, or a value of the wrong type or range",
        ),
        (
            FindingCode::MissingKey,
            "config.missing_key",
            "a required key is missing",
        ),
        (
            FindingCode::UnsupportedVersion,
            "config.unsupported_version",
            "this build supports config_version 1 only",
        ),
        (
            FindingCode::UnsupportedCapability,
            "config.unsupported_capability",
            "this capability is not supported by this build; remove the key or set it to false",
        ),
    ];
    for (code, text, message) in table {
        assert_eq!(code.as_str(), text);
        assert_eq!(code.message(), message);
    }
}
