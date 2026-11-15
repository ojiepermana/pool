//! Every fixture and in memory case through `pool_config::validate`
//! (spec 0001 AC-4 to AC-9, spec 0003 AC-1 to AC-6 and AC-15, spec 0004
//! AC-5 to AC-8).

use std::path::Path;
use std::time::Duration;

use pool_config::{
    CapabilityState, FindingCode, Location, MAX_CONFIG_BYTES, MAX_FINDINGS, Rejection,
    RoutingProfile, ValidatedConfig, validate,
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

// Edge and error cases added by the tester (spec 0001 AC-4 to AC-9,
// SEC-10, SEC-11). Every input is made in memory; no fixture is added.

/// Validates `text` and checks both reserved capabilities are disabled.
fn assert_text_valid(text: &str) {
    assert_disabled(&accepted(text.as_bytes()));
}

/// Checks `text` is refused with exactly one capability finding at `key`.
fn assert_capability_refused(text: &str, key: &'static str) {
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::UnsupportedCapability, Location::Key(key))],
        "{text:?}"
    );
}

#[test]
fn ac4_sec10_result_cache_left_out_validates_as_disabled() {
    assert_text_valid("config_version = 1\n");
    assert_text_valid("config_version = 1\n[automatic_promotion]\nenabled = false\n");
}

#[test]
fn ac4_sec10_automatic_promotion_left_out_validates_as_disabled() {
    assert_text_valid("config_version = 1\n");
    assert_text_valid("config_version = 1\n[result_cache]\nenabled = false\n");
}

#[test]
fn ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling() {
    assert_text_valid("config_version = 1\n[result_cache]\n");
    assert_text_valid("config_version = 1\nresult_cache = {}\n");
}

#[test]
fn ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling() {
    assert_text_valid("config_version = 1\n[automatic_promotion]\n");
    assert_text_valid("config_version = 1\nautomatic_promotion = {}\n");
}

#[test]
fn ac4_sec10_result_cache_enabled_false_validates_in_every_spelling() {
    assert_text_valid("config_version = 1\n[result_cache]\nenabled = false\n");
    assert_text_valid("config_version = 1\nresult_cache.enabled = false\n");
    assert_text_valid("config_version = 1\nresult_cache = { enabled = false }\n");
}

#[test]
fn ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling() {
    assert_text_valid("config_version = 1\n[automatic_promotion]\nenabled = false\n");
    assert_text_valid("config_version = 1\nautomatic_promotion.enabled = false\n");
    assert_text_valid("config_version = 1\nautomatic_promotion = { enabled = false }\n");
}

#[test]
fn ac5_sec10_result_cache_enabled_true_is_refused_in_the_header_spelling() {
    assert_capability_refused(
        "config_version = 1\n[result_cache]\nenabled = true\n",
        "result_cache.enabled",
    );
}

#[test]
fn ac5_sec10_result_cache_enabled_true_is_refused_in_the_dotted_spelling() {
    assert_capability_refused(
        "config_version = 1\nresult_cache.enabled = true\n",
        "result_cache.enabled",
    );
}

#[test]
fn ac5_sec10_result_cache_enabled_true_is_refused_in_the_inline_spelling() {
    assert_capability_refused(
        "config_version = 1\nresult_cache = { enabled = true }\n",
        "result_cache.enabled",
    );
}

#[test]
fn ac6_sec10_automatic_promotion_enabled_true_is_refused_in_the_header_spelling() {
    assert_capability_refused(
        "config_version = 1\n[automatic_promotion]\nenabled = true\n",
        "automatic_promotion.enabled",
    );
}

#[test]
fn ac6_sec10_automatic_promotion_enabled_true_is_refused_in_the_dotted_spelling() {
    assert_capability_refused(
        "config_version = 1\nautomatic_promotion.enabled = true\n",
        "automatic_promotion.enabled",
    );
}

#[test]
fn ac6_sec10_automatic_promotion_enabled_true_is_refused_in_the_inline_spelling() {
    assert_capability_refused(
        "config_version = 1\nautomatic_promotion = { enabled = true }\n",
        "automatic_promotion.enabled",
    );
}

#[test]
fn ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() {
    assert_capability_refused(
        "config_version = 1\nresult_cache.enabled = false\nautomatic_promotion.enabled = true\n",
        "automatic_promotion.enabled",
    );
    assert_capability_refused(
        "config_version = 1\nresult_cache = { enabled = true }\nautomatic_promotion = {}\n",
        "result_cache.enabled",
    );
}

#[test]
fn ac7_sec10_another_capability_name_is_an_unknown_key() {
    for text in [
        "config_version = 1\n[query_cache]\nenabled = true\n",
        "config_version = 1\nautomatic_failover.enabled = true\n",
        "config_version = 1\nload_balancing = { enabled = false }\n",
    ] {
        assert_positioned(text.as_bytes(), FindingCode::Schema, 2);
    }
}

#[test]
fn ac7_automatic_promotion_enabled_that_is_not_a_boolean_is_a_schema_finding() {
    for text in [
        "config_version = 1\n[automatic_promotion]\nenabled = \"yes\"\n",
        "config_version = 1\nautomatic_promotion.enabled = 1\n",
        "config_version = 1\nautomatic_promotion = { enabled = [true] }\n",
        "config_version = 1\nresult_cache.enabled = \"true\"\n",
        "config_version = 1\nresult_cache = { enabled = 0 }\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Schema, "{text:?}");
    }
}

#[test]
fn ac7_a_reserved_capability_that_is_not_a_table_is_a_schema_finding() {
    for text in [
        "config_version = 1\nautomatic_promotion = true\n",
        "config_version = 1\nautomatic_promotion = []\n",
        "config_version = 1\nresult_cache = false\n",
        "config_version = 1\n[[result_cache]]\nenabled = false\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Schema, "{text:?}");
    }
}

#[test]
fn ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding() {
    for name in ["result_cache", "automatic_promotion"] {
        for value in [
            "[false]",
            "[true]",
            "[false, \"CANARY-9f2c7b1e\", 3]",
            "[true, false]",
            "[\n  false,\n  { anything = \"goes\", here = [1, 2, 3] },\n]",
        ] {
            let text = format!("config_version = 1\n{name} = {value}\n");
            let found = findings(text.as_bytes());
            assert_eq!(found.len(), 1, "{text:?}");
            assert_eq!(found[0].0, FindingCode::Schema, "{text:?}");
            assert!(
                matches!(found[0].1, Location::Position { line: 2, .. }),
                "{text:?}: {:?}",
                found[0].1
            );
            let rendered = format!(
                "{:?}\n{}",
                rejected(text.as_bytes()),
                rejected(text.as_bytes())
            );
            assert!(!rendered.contains(CANARY), "{text:?}: {rendered}");
        }
    }
}

#[test]
fn ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location() {
    let deep_key = format!("{} = 1\n", ["a"; 81].join("."));
    let deep_header = format!("[{}]\n", ["a"; 81].join("."));
    for text in [deep_key, deep_header] {
        assert_eq!(
            findings(text.as_bytes()),
            [(FindingCode::Syntax, Location::None)],
            "{text:?}"
        );
    }
    // At the limit the parser still accepts the key, so the schema judges it.
    let at_limit = format!("{} = 1\n", ["a"; 80].join("."));
    assert_eq!(
        findings(at_limit.as_bytes()),
        [(
            FindingCode::Schema,
            Location::Position { line: 1, column: 1 }
        )]
    );
}

#[test]
fn ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding() {
    let arrays = format!("x = {}{}\n", "[".repeat(100_000), "]".repeat(100_000));
    let tables = format!("x = {}\n", "{a=".repeat(100_000));
    for text in [arrays, tables] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].0, FindingCode::Syntax);
    }
}

#[test]
fn ac7_an_unknown_key_inside_a_reserved_table_is_a_schema_finding_at_its_line() {
    for text in [
        "config_version = 1\n[automatic_promotion]\nenabled = false\ntimeout_ms = 5\n",
        "config_version = 1\n\n[result_cache]\n\n\nsize_bytes = 1\n",
    ] {
        let line = u32::try_from(text.lines().count()).expect("small");
        assert_positioned(text.as_bytes(), FindingCode::Schema, line);
    }
}

#[test]
fn ac7_negative_config_version_at_the_64_bit_edge_is_a_schema_finding() {
    for text in [
        "config_version = -9223372036854775808\n",
        "config_version = -1_000\n",
        "config_version = -0\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        // `-0` is the integer 0, which is in range, so it is a version finding.
        let expected = if text.contains("-0\n") {
            FindingCode::UnsupportedVersion
        } else {
            FindingCode::Schema
        };
        assert_eq!(found[0].0, expected, "{text:?}");
    }
}

#[test]
fn ac7_a_signed_hexadecimal_config_version_is_a_syntax_finding() {
    assert_positioned(b"config_version = -0x1\n", FindingCode::Syntax, 1);
}

#[test]
fn ac7_config_version_one_in_other_integer_spellings_is_accepted() {
    for text in [
        "config_version = 0x1\n",
        "config_version = 0o1\n",
        "config_version = 0b1\n",
        "config_version = +1\n",
    ] {
        assert_text_valid(text);
    }
}

#[test]
fn ac7_config_version_two_in_other_integer_spellings_is_unsupported() {
    for text in [
        "config_version = 0x2\n",
        "config_version = +2\n",
        "config_version = 4_294_967_295\n",
        "config_version = 0xFFFFFFFF\n",
    ] {
        assert_eq!(
            findings(text.as_bytes()),
            [(
                FindingCode::UnsupportedVersion,
                Location::Key("config_version")
            )],
            "{text:?}"
        );
    }
}

#[test]
fn ac7_config_version_of_other_types_is_a_schema_finding() {
    for text in [
        "config_version = 1979-05-27\n",
        "config_version = { major = 1 }\n",
        "config_version = 1e0\n",
        "config_version = inf\n",
        "config_version = nan\n",
        "config_version = ''\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Schema, "{text:?}");
    }
}

#[test]
fn ac7_config_version_beyond_the_64_bit_range_is_a_syntax_finding_on_its_line() {
    for text in [
        "config_version = 9223372036854775808\n",
        "config_version = -9223372036854775809\n",
        "# comment\nconfig_version = 99999999999999999999\n",
    ] {
        let line = u32::try_from(text.lines().count()).expect("small");
        assert_positioned(text.as_bytes(), FindingCode::Syntax, line);
    }
}

#[test]
fn ac7_automatic_promotion_written_twice_is_a_syntax_finding_in_every_spelling() {
    for text in [
        "config_version = 1\nautomatic_promotion.enabled = false\nautomatic_promotion.enabled = false\n",
        "config_version = 1\nautomatic_promotion = { enabled = false }\nautomatic_promotion = { enabled = false }\n",
        "config_version = 1\n[automatic_promotion]\nenabled = false\nenabled = false\n",
        "config_version = 1\n[automatic_promotion]\n[automatic_promotion]\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Syntax, "{text:?}");
    }
}

#[test]
fn ac7_a_capability_written_twice_across_spellings_is_a_syntax_finding() {
    for text in [
        "config_version = 1\nresult_cache = { enabled = false }\n[result_cache]\n",
        "config_version = 1\nautomatic_promotion = {}\nautomatic_promotion.enabled = false\n",
        "config_version = 1\n[result_cache]\nenabled = false\n[result_cache.enabled]\n",
    ] {
        let found = findings(text.as_bytes());
        assert_eq!(found.len(), 1, "{text:?}");
        assert_eq!(found[0].0, FindingCode::Syntax, "{text:?}");
    }
}

#[test]
fn ac7_a_file_of_only_whitespace_and_comments_misses_config_version() {
    for text in [
        "\n",
        "   \n\t\n",
        "\r\n\r\n",
        "# a\r\n# b\r\n",
        "# no newline",
    ] {
        assert_eq!(
            findings(text.as_bytes()),
            [(FindingCode::MissingKey, Location::Key("config_version"))],
            "{text:?}"
        );
    }
}

#[test]
fn ac7_crlf_line_endings_keep_every_fixture_line_and_outcome() {
    let crlf = |name: &str| {
        String::from_utf8(fixture(name))
            .expect("fixture is UTF-8")
            .replace('\n', "\r\n")
    };
    for name in [
        "valid-minimal.toml",
        "valid-capabilities-disabled.toml",
        "valid-capabilities-empty.toml",
    ] {
        assert_text_valid(&crlf(name));
    }
    assert_positioned(
        crlf("invalid-syntax.toml").as_bytes(),
        FindingCode::Syntax,
        2,
    );
    assert_positioned(
        crlf("secret-unknown-key.toml").as_bytes(),
        FindingCode::Schema,
        2,
    );
    assert_positioned(
        crlf("secret-syntax.toml").as_bytes(),
        FindingCode::Syntax,
        2,
    );
    assert_eq!(
        findings(crlf("invalid-multiple.toml").as_bytes()),
        findings(&fixture("invalid-multiple.toml"))
    );
}

#[test]
fn ac7_a_file_without_a_final_newline_is_judged_the_same() {
    assert_text_valid("config_version = 1");
    assert_capability_refused(
        "config_version = 1\nautomatic_promotion.enabled = true",
        "automatic_promotion.enabled",
    );
}

#[test]
fn ac7_column_counts_characters_not_bytes_before_the_error() {
    // Two two byte characters sit before the error on the same line. The
    // column is 1 plus the 9 characters before `junk`; a byte count gives 12.
    let text = "config_version = 1\nx = \"éé\" junk\n";
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::Syntax,
            Location::Position {
                line: 2,
                column: 10
            }
        )]
    );
}

#[test]
fn ac7_size_limit_is_checked_before_utf8() {
    let too_large = vec![0xFF; MAX_CONFIG_BYTES + 1];
    assert_eq!(
        findings(&too_large),
        [(FindingCode::TooLarge, Location::None)]
    );
}

/// A comment of `#` characters followed by `tail`, exactly the limit long.
fn padded(tail: &[u8]) -> Vec<u8> {
    let mut bytes = vec![b'#'; MAX_CONFIG_BYTES - tail.len()];
    bytes.extend_from_slice(tail);
    bytes
}

#[test]
fn ac7_a_file_of_exactly_the_limit_is_judged_on_its_content() {
    let at_limit_not_utf8 = vec![0xFF; MAX_CONFIG_BYTES];
    assert_eq!(
        findings(&at_limit_not_utf8),
        [(FindingCode::NotUtf8, Location::None)]
    );

    let at_limit_valid = padded(b"\nconfig_version = 1\nresult_cache = {}\n");
    assert_eq!(at_limit_valid.len(), MAX_CONFIG_BYTES);
    assert_text_valid(std::str::from_utf8(&at_limit_valid).expect("ASCII"));

    let at_limit_with_capability = padded(b"\nconfig_version = 1\nresult_cache.enabled = true\n");
    assert_eq!(at_limit_with_capability.len(), MAX_CONFIG_BYTES);
    assert_eq!(
        findings(&at_limit_with_capability),
        [(
            FindingCode::UnsupportedCapability,
            Location::Key("result_cache.enabled")
        )]
    );
}

#[test]
fn ac7_far_beyond_the_limit_is_still_one_too_large_finding() {
    let huge = vec![b'#'; MAX_CONFIG_BYTES * 2];
    assert_eq!(findings(&huge), [(FindingCode::TooLarge, Location::None)]);
}

#[test]
fn ac7_any_invalid_utf8_byte_sequence_is_refused_before_syntax() {
    for bytes in [
        b"config_version = 1\n\xC3".to_vec(),
        b"config_version = 1\n# \xC0\xAF\n".to_vec(),
        b"\xED\xA0\x80 = 1\n".to_vec(),
        b"config_version = \"\xF8\x88\x80\x80\x80\"\n".to_vec(),
        b"[result_cache\n\xFF".to_vec(),
    ] {
        assert_eq!(
            findings(&bytes),
            [(FindingCode::NotUtf8, Location::None)],
            "{bytes:?}"
        );
    }
}

#[test]
fn ac7_a_syntax_error_wins_over_schema_and_meaning_findings() {
    let text = "config_version = 2\n[no_table]\nport = 6432\nresult_cache.enabled = true\n[automatic_promotion\n";
    assert_positioned(text.as_bytes(), FindingCode::Syntax, 5);
}

#[test]
fn ac7_a_schema_error_wins_over_meaning_findings() {
    let text = "config_version = 2\nresult_cache.enabled = true\n[no_table]\nport = 6432\n";
    assert_positioned(text.as_bytes(), FindingCode::Schema, 3);
}

#[test]
fn ac8_a_missing_version_is_reported_before_both_capabilities() {
    let text = "automatic_promotion.enabled = true\nresult_cache = { enabled = true }\n";
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::MissingKey, Location::Key("config_version")),
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
fn ac8_finding_order_follows_the_schema_not_the_file() {
    // The file lists automatic_promotion first and config_version last.
    let text = "[automatic_promotion]\nenabled = true\n[result_cache]\nenabled = true\n";
    let reordered = format!("config_version = 7\n{text}");
    assert_eq!(
        findings(reordered.as_bytes()),
        findings(&fixture("invalid-multiple.toml"))
    );
}

#[test]
fn ac8_two_findings_leave_out_the_one_that_is_fine() {
    assert_eq!(
        findings(b"config_version = 3\nautomatic_promotion.enabled = true\n"),
        [
            (
                FindingCode::UnsupportedVersion,
                Location::Key("config_version")
            ),
            (
                FindingCode::UnsupportedCapability,
                Location::Key("automatic_promotion.enabled")
            ),
        ]
    );
}

#[test]
fn ac8_every_rejection_holds_at_least_one_finding() {
    let mut inputs: Vec<Vec<u8>> = [
        "invalid-automatic-promotion-enabled.toml",
        "invalid-empty.toml",
        "invalid-missing-version.toml",
        "invalid-multiple.toml",
        "invalid-result-cache-enabled.toml",
        "invalid-syntax.toml",
        "invalid-unknown-key.toml",
        "invalid-unsupported-version.toml",
        "invalid-wrong-type.toml",
        "secret-syntax.toml",
        "secret-unknown-key.toml",
        "secret-wrong-type.toml",
    ]
    .iter()
    .map(|name| fixture(name))
    .collect();
    inputs.push(vec![0xFF]);
    inputs.push(vec![b'#'; MAX_CONFIG_BYTES + 1]);
    for bytes in inputs {
        let rejection = rejected(&bytes);
        assert!(!rejection.findings().is_empty());
        assert!(!rejection.to_string().is_empty());
        assert_eq!(
            rejection.to_string().lines().count(),
            rejection.findings().len()
        );
    }
}

#[test]
fn ac8_validation_is_repeatable_for_the_same_bytes() {
    let bytes = fixture("invalid-multiple.toml");
    let first = findings(&bytes);
    assert!(first.len() > 1, "{first:?}");
    assert_eq!(first, findings(&bytes));
    let valid = fixture("valid-capabilities-disabled.toml");
    assert_disabled(&accepted(&valid));
    assert_disabled(&accepted(&valid));
}

#[test]
fn ac9_sec11_the_canary_is_absent_from_every_finding_debug_and_display() {
    for name in [
        "secret-unknown-key.toml",
        "secret-wrong-type.toml",
        "secret-syntax.toml",
    ] {
        let rejection = rejected(&fixture(name));
        for finding in rejection.findings() {
            let debug = format!("{finding:?}");
            let pretty = format!("{finding:#?}");
            let display = finding.to_string();
            let location = format!("{:?}", finding.location());
            for text in [&debug, &pretty, &display, &location] {
                assert!(!text.contains(CANARY), "{name}: {text}");
            }
        }
        let pretty = format!("{rejection:#?}");
        assert!(!pretty.contains(CANARY), "{name}: {pretty}");
    }
}

#[test]
fn ac9_sec11_no_part_of_the_secret_value_reaches_the_rejection_text() {
    for name in [
        "secret-unknown-key.toml",
        "secret-wrong-type.toml",
        "secret-syntax.toml",
    ] {
        let rejection = rejected(&fixture(name));
        let text = format!("{rejection:?}\n{rejection}");
        for piece in [
            "9f2c7b1e",
            "not-a-real-secret",
            "backend_password",
            "CANARY",
        ] {
            assert!(!text.contains(piece), "{name}: {piece} in {text}");
        }
    }
}

#[test]
fn ac9_sec11_a_canary_in_other_places_does_not_reach_the_rejection_text() {
    let canary_value = "CANARY-9f2c7b1e-not-a-real-secret";
    for text in [
        format!("config_version = 1\n[result_cache]\nenabled = \"{canary_value}\"\n"),
        format!("config_version = 1\n[\"{canary_value}\"]\n"),
        format!("config_version = 1\nresult_cache = {{ \"{canary_value}\" = true }}\n"),
        format!("config_version = 1\n# {canary_value}\nresult_cache.enabled = true\n"),
        format!("config_version = 1\nautomatic_promotion.enabled = {canary_value}\n"),
        format!("config_version = '{canary_value}'\n"),
    ] {
        let rejection = rejected(text.as_bytes());
        let rendered = format!("{rejection:?}\n{rejection:#?}\n{rejection}");
        assert!(!rendered.contains(CANARY), "{text:?}: {rendered}");
    }
}

#[test]
fn ac4_validated_config_debug_holds_only_states() {
    let config = accepted(&fixture("valid-capabilities-disabled.toml"));
    let debug = format!("{config:?}");
    assert_eq!(
        debug,
        "ValidatedConfig { config_version: 1, result_cache: Disabled, automatic_promotion: Disabled, \
         control: None, clusters: [], nodes: [], identities: [], identity_index: IdentityIndex { entries: 0 }, \
         listener: None, limits: Limits { session_max_count: 1000, handshake_max_count: 64, \
         backend_wait_max_count: 100, node_backend_max_count: 100, startup_timeout_ms: 5000, \
         backend_acquire_timeout_ms: 5000, shutdown_timeout_ms: 30000, startup_packet_max_bytes: 8192, \
         frontend_message_max_bytes: 16777216 }, probe: ProbeConfig { period_ms: 2000, timeout_ms: 1000, \
         observation_expiry_ms: 6000 }, principals: [], principal_index: PrincipalIndex { entries: 0 } }"
    );
    assert_eq!(config.automatic_promotion().as_str(), "disabled");
    assert_eq!(config.result_cache().as_str(), "disabled");
}

// Spec 0003: the keys [control], [[cluster]], [[node]] and [[identity]]
// (0003 AC-1 to AC-5).

/// The fifteen line base of the spec 0003 fixtures.
const BASE: &str = "config_version = 1

[control]
state_directory = \"/var/lib/pool\"

[[cluster]]
id = \"main\"
system_identifier = \"7412345678901234567\"
expected_writer = \"pg1\"

[[node]]
id = \"pg1\"
cluster = \"main\"
host = \"10.0.0.11\"
port = 5432
";

/// The base with line `number` (counted from 1) replaced by `line`.
fn base_with_line(number: usize, line: &str) -> String {
    let mut lines: Vec<&str> = BASE.lines().collect();
    lines[number - 1] = line;
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

/// The base, an empty line, then `extra`.
fn base_plus(extra: &str) -> String {
    format!("{BASE}\n{extra}")
}

/// A complete identity entry with one line replaced or added.
fn identity_with(key: &str, value: &str) -> String {
    let mut lines = vec![
        "[[identity]]".to_owned(),
        "id = \"orders-app\"".to_owned(),
        "principal = \"orders_app\"".to_owned(),
        "database = \"orders\"".to_owned(),
        "cluster = \"main\"".to_owned(),
        "backend_login = \"orders_rw\"".to_owned(),
        "credential_file = \"/etc/pool/secrets/orders_rw\"".to_owned(),
    ];
    let prefix = format!("{key} =");
    let line = format!("{key} = {value}");
    match lines
        .iter()
        .position(|existing| existing.starts_with(&prefix))
    {
        Some(index) => lines[index] = line,
        None => lines.push(line),
    }
    base_plus(&(lines.join("\n") + "\n"))
}

fn key_at(key: &'static str, line: u32, column: u32) -> Location {
    Location::KeyAt { key, line, column }
}

/// The single finding of `text`, as (code, key).
fn single_code_and_key(text: &str) -> (FindingCode, Option<&'static str>) {
    let found = findings(text.as_bytes());
    assert_eq!(found.len(), 1, "{text}\n{found:?}");
    let key = match found[0].1 {
        Location::Key(key) | Location::KeyAt { key, .. } => Some(key),
        Location::None | Location::Position { .. } => None,
    };
    (found[0].0, key)
}

fn single_code(text: &str) -> FindingCode {
    single_code_and_key(text).0
}

#[test]
fn ac1_topology_fixtures_are_accepted() {
    for name in [
        "valid-topology-minimal.toml",
        "valid-topology.toml",
        "valid-topology-names.toml",
    ] {
        assert_disabled(&accepted(&fixture(name)));
    }
    assert_text_valid("config_version = 1\n");
}

#[test]
fn ac1_valid_topology_exposes_its_records_in_file_order() {
    let config = accepted(&fixture("valid-topology.toml"));
    let control = config.control().expect("control table");
    assert_eq!(
        control.state_directory(),
        Some(std::path::Path::new("/var/lib/pool"))
    );
    assert_eq!(
        control.authority_id().map(|id| id.to_string()),
        Some("0123456789abcdef0123456789abcdef".to_owned())
    );

    let clusters = config.clusters();
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].id().as_str(), "main");
    assert_eq!(
        clusters[0].system_identifier().get(),
        7_412_345_678_901_234_567
    );
    assert_eq!(clusters[0].expected_writer().as_str(), "pg1");

    let nodes: Vec<(&str, &str, &str, u16)> = config
        .nodes()
        .iter()
        .map(|node| {
            (
                node.id().as_str(),
                node.cluster().as_str(),
                node.host(),
                node.port(),
            )
        })
        .collect();
    assert_eq!(
        nodes,
        [
            ("pg1", "main", "10.0.0.11", 5432),
            ("pg2", "main", "10.0.0.12", 5432),
            ("pg3", "main", "pg3.db.internal", 5433),
        ]
    );

    let identities = config.identities();
    assert_eq!(identities.len(), 2);
    let orders = &identities[0];
    assert_eq!(orders.id().as_str(), "orders-app");
    assert_eq!(orders.cluster().as_str(), "main");
    assert_eq!(orders.backend_database(), "orders");
    assert_eq!(orders.backend_login(), "orders_rw");
    assert_eq!(
        orders.credential().path(),
        std::path::Path::new("/etc/pool/secrets/orders_rw")
    );
    assert_eq!(orders.routing_profile(), RoutingProfile::Primary);
    assert_eq!(orders.routing_profile().as_str(), "primary");
    let report = &identities[1];
    assert_eq!(report.id().as_str(), "report-app");
    assert_eq!(report.backend_database(), "orders_reporting");
    assert_eq!(report.backend_login(), "report_ro");
}

#[test]
fn ac1_a_file_without_topology_has_no_records() {
    let config = accepted(&fixture("valid-minimal.toml"));
    assert!(config.control().is_none());
    assert!(config.clusters().is_empty());
    assert!(config.nodes().is_empty());
    assert!(config.identities().is_empty());
    assert!(config.resolve_identity("orders_app", "orders").is_none());
}

#[test]
fn ac2_each_invalid_topology_fixture_has_its_one_finding() {
    let table = [
        (
            "invalid-topology-missing-key.toml",
            FindingCode::MissingKey,
            key_at("identity.backend_login", 17, 1),
        ),
        (
            "invalid-topology-missing-state-directory.toml",
            FindingCode::MissingKey,
            Location::Key("control.state_directory"),
        ),
        (
            "invalid-topology-invalid-value.toml",
            FindingCode::InvalidValue,
            key_at("node.id", 12, 6),
        ),
        (
            "invalid-topology-duplicate-id.toml",
            FindingCode::DuplicateId,
            key_at("node.id", 18, 6),
        ),
        (
            "invalid-topology-unknown-reference.toml",
            FindingCode::UnknownReference,
            key_at("cluster.expected_writer", 9, 19),
        ),
        (
            "invalid-topology-duplicate-endpoint.toml",
            FindingCode::DuplicateEndpoint,
            key_at("node.host", 20, 8),
        ),
        (
            "invalid-topology-duplicate-identity.toml",
            FindingCode::DuplicateIdentity,
            key_at("identity.principal", 27, 13),
        ),
        (
            "invalid-topology-unsupported-profile.toml",
            FindingCode::UnsupportedValue,
            key_at("identity.routing_profile", 24, 19),
        ),
    ];
    for (name, code, location) in table {
        assert_eq!(findings(&fixture(name)), [(code, location)], "{name}");
    }
}

#[test]
fn ac3_multiple_topology_mistakes_are_reported_in_one_run_in_order() {
    assert_eq!(
        findings(&fixture("invalid-topology-multiple.toml")),
        [
            (
                FindingCode::InvalidValue,
                Location::Key("control.authority_id")
            ),
            (
                FindingCode::InvalidValue,
                key_at("cluster.system_identifier", 9, 21)
            ),
            (FindingCode::InvalidValue, key_at("node.port", 16, 8)),
            (
                FindingCode::MissingKey,
                key_at("identity.credential_file", 18, 1)
            ),
            (
                FindingCode::UnknownReference,
                key_at("identity.cluster", 22, 11)
            ),
        ]
    );
}

#[test]
fn ac3_key_at_findings_display_the_spec_line() {
    let rejection = rejected(&fixture("invalid-topology-duplicate-id.toml"));
    assert_eq!(
        rejection.to_string(),
        "config.duplicate_id at node.id, line 18, column 6: an id is used more than once"
    );
}

#[test]
fn ac2_which_code_a_bad_value_gets() {
    let schema = [
        "config_version = 1\n[node]\nid = \"pg2\"\n".to_owned(),
        "config_version = 1\n[cluster]\nid = \"main\"\n".to_owned(),
        "config_version = 1\n[identity]\nid = \"a\"\n".to_owned(),
        "config_version = 1\nnode = [1]\n".to_owned(),
        "config_version = 1\ncluster = [\"main\"]\n".to_owned(),
        "config_version = 1\ncontrol = 1\n".to_owned(),
        "config_version = 1\ncontrol = []\n".to_owned(),
        "config_version = 1\n[control]\nweight = 1\n".to_owned(),
        base_plus("[[node]]\nid = \"pg2\"\nweight = 1\n"),
        (identity_with("weight", "1")),
        base_with_line(7, "id = 7"),
        base_with_line(15, "port = \"5432\""),
        base_with_line(15, "port = 70000"),
        base_with_line(15, "port = -1"),
        base_with_line(8, "system_identifier = 7412345678901234567"),
        base_with_line(4, "state_directory = 1"),
        (identity_with("principal", "1")),
        (identity_with("routing_profile", "true")),
    ];
    for text in &schema {
        assert_eq!(single_code(text), FindingCode::Schema, "{text}");
    }

    let invalid: Vec<(String, &str)> = vec![
        (base_with_line(15, "port = 0"), "node.port"),
        (base_with_line(7, "id = \"\""), "cluster.id"),
        (
            base_with_line(12, &format!("id = \"{}\"", "a".repeat(64))),
            "node.id",
        ),
        (base_with_line(12, "id = \"1pg\""), "node.id"),
        (base_with_line(12, "id = \"Pg1\""), "node.id"),
        ((identity_with("id", "\"Orders\"")), "identity.id"),
        (
            base_with_line(8, "system_identifier = \"0\""),
            "cluster.system_identifier",
        ),
        (
            base_with_line(8, "system_identifier = \"007\""),
            "cluster.system_identifier",
        ),
        (
            base_with_line(8, "system_identifier = \"\""),
            "cluster.system_identifier",
        ),
        (
            base_with_line(8, "system_identifier = \"18446744073709551616\""),
            "cluster.system_identifier",
        ),
        (
            base_with_line(8, "system_identifier = \"74a\""),
            "cluster.system_identifier",
        ),
        (
            base_with_line(
                4,
                "state_directory = \"/var/lib/pool\"\nauthority_id = \"0123456789abcdef0123456789abcde\"",
            ),
            "control.authority_id",
        ),
        (
            base_with_line(
                4,
                "state_directory = \"/var/lib/pool\"\nauthority_id = \"0123456789abcdef0123456789abcdef0\"",
            ),
            "control.authority_id",
        ),
        (
            base_with_line(
                4,
                "state_directory = \"/var/lib/pool\"\nauthority_id = \"0123456789ABCDEF0123456789abcdef\"",
            ),
            "control.authority_id",
        ),
        (
            base_with_line(4, "state_directory = \"var/lib/pool\""),
            "control.state_directory",
        ),
        (
            (identity_with("credential_file", "\"secrets/orders\"")),
            "identity.credential_file",
        ),
        ((identity_with("principal", "\"\"")), "identity.principal"),
        (
            (identity_with("database", &format!("\"{}\"", "d".repeat(64)))),
            "identity.database",
        ),
        (
            (identity_with("backend_database", "\"a\\nb\"")),
            "identity.backend_database",
        ),
        (
            (identity_with("backend_login", "\"\"")),
            "identity.backend_login",
        ),
        (base_with_line(14, "host = \"db host\""), "node.host"),
        (base_with_line(14, "host = \"db_host\""), "node.host"),
        (
            (identity_with("routing_profile", "\"Primary\"")),
            "identity.routing_profile",
        ),
    ];
    for (text, key) in &invalid {
        assert_eq!(
            single_code_and_key(text),
            (FindingCode::InvalidValue, Some(*key)),
            "{text}"
        );
    }

    assert_eq!(
        single_code_and_key(&(identity_with("routing_profile", "\"replica\""))),
        (
            FindingCode::UnsupportedValue,
            Some("identity.routing_profile")
        )
    );

    for text in [
        "config_version = 1\ncluster = []\nnode = []\nidentity = []\n".to_owned(),
        "config_version = 1\n[control]\n".to_owned(),
        base_with_line(8, "system_identifier = \"18446744073709551615\""),
        identity_with("id", &format!("\"{}\"", "a".repeat(63))),
        (identity_with("principal", &format!("\"{}\"", "é".repeat(31)))),
        (identity_with("routing_profile", "\"primary\"")),
        base_plus("[[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"::1\"\nport = 65535\n"),
        "config_version = 1\nnode = [{ id = \"pg1\", cluster = \"main\", host = \"h\", port = 1 }]\ncluster = [{ id = \"main\", system_identifier = \"1\", expected_writer = \"pg1\" }]\n[control]\nstate_directory = \"/p\"\n".to_owned(),
    ] {
        assert_text_valid(&text);
    }
}

#[test]
fn ac2_a_missing_key_in_an_inline_entry_is_at_its_brace() {
    let text = "config_version = 1\n[control]\nstate_directory = \"/p\"\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"h\"\nport = 1\n";
    assert_text_valid(text);
    let inline = "config_version = 1\n[control]\nstate_directory = \"/p\"\ncluster = [{ id = \"main\", system_identifier = \"1\" }]\n";
    let found = findings(inline.as_bytes());
    // `cluster` after `[control]` is a key of the control table.
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].0, FindingCode::Schema);
    let top = "config_version = 1\ncluster = [{ id = \"main\", system_identifier = \"1\" }]\n[control]\nstate_directory = \"/p\"\n";
    assert_eq!(
        findings(top.as_bytes()),
        [(
            FindingCode::MissingKey,
            key_at("cluster.expected_writer", 2, 12)
        )]
    );
}

#[test]
fn ac2_cross_checks_skip_to_keep_one_finding_per_mistake() {
    // An invalid cluster id skips the node.cluster and identity.cluster checks.
    let text = (identity_with("cluster", "\"main\"")).replace("id = \"main\"", "id = \"Main\"");
    assert_eq!(
        single_code_and_key(&text),
        (FindingCode::InvalidValue, Some("cluster.id"))
    );

    // A node naming an unknown cluster skips the expected_writer check.
    let text = base_with_line(13, "cluster = \"other\"");
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::UnknownReference,
            key_at("node.cluster", 13, 11)
        )]
    );

    // An invalid node id skips the expected_writer check.
    let text = base_with_line(12, "id = \"PG1\"");
    assert_eq!(
        single_code_and_key(&text),
        (FindingCode::InvalidValue, Some("node.id"))
    );

    // expected_writer naming a node of another cluster.
    let text = base_plus(
        "[[cluster]]\nid = \"other\"\nsystem_identifier = \"2\"\nexpected_writer = \"pg1\"\n",
    );
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::UnknownReference,
            key_at("cluster.expected_writer", 20, 19)
        )]
    );

    // A repeated node id resolves to the first entry.
    let text = base_plus(
        "[[cluster]]\nid = \"other\"\nsystem_identifier = \"2\"\nexpected_writer = \"pg1\"\n\n[[node]]\nid = \"pg1\"\ncluster = \"other\"\nhost = \"10.0.0.12\"\nport = 5432\n",
    );
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("node.id", 23, 6)),
            (
                FindingCode::UnknownReference,
                key_at("cluster.expected_writer", 20, 19)
            ),
        ]
    );
}

#[test]
fn ac2_endpoints_compare_hosts_without_ascii_case_and_names_byte_for_byte() {
    let text = base_plus(
        "[[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5433\n",
    );
    assert_text_valid(&text);
    let host = base_with_line(14, "host = \"db.example\"");
    let text = format!(
        "{host}\n[[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"DB.Example\"\nport = 5432\n"
    );
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::DuplicateEndpoint, key_at("node.host", 20, 8))]
    );

    let first = identity_with("principal", "\"orders_app\"");
    let second = "\n[[identity]]\nid = \"orders-app-2\"\nprincipal = \"Orders_app\"\ndatabase = \"orders\"\ncluster = \"main\"\nbackend_login = \"orders_rw\"\ncredential_file = \"/etc/pool/secrets/orders_rw\"\n";
    assert_text_valid(&format!("{first}{second}"));
}

#[test]
fn ac4_canary_never_reaches_rejection_or_config_text() {
    let rejection = rejected(&fixture("secret-topology-values.toml"));
    assert_eq!(
        rejection
            .findings()
            .iter()
            .map(|finding| (finding.code(), finding.location()))
            .collect::<Vec<_>>(),
        [
            (FindingCode::InvalidValue, key_at("node.host", 14, 8)),
            (
                FindingCode::InvalidValue,
                key_at("identity.credential_file", 23, 19)
            ),
        ]
    );
    for text in [format!("{rejection:?}"), rejection.to_string()] {
        assert!(!text.contains(CANARY), "{text}");
    }

    let config = accepted(&fixture("valid-topology-names.toml"));
    let debug = format!("{config:?}");
    assert!(!debug.contains(CANARY), "{debug}");
    assert!(debug.contains("canary-app"), "{debug}");
    for route in config.identities() {
        let debug = format!("{route:?} {:?}", route.credential());
        assert!(!debug.contains(CANARY), "{debug}");
    }
    for node in config.nodes() {
        let debug = format!("{node:?}");
        assert!(!debug.contains("10.0.0.11"), "{debug}");
        assert!(!debug.contains("5432"), "{debug}");
    }
    let control = format!("{:?}", config.control());
    assert!(!control.contains("/var/lib/pool"), "{control}");
}

#[test]
fn ac4_validate_opens_no_file() {
    let text = format!(
        "{}\n{}",
        BASE.replace("/var/lib/pool", "/does/not/exist/pool-state"),
        "[[identity]]\nid = \"a\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"/does/not/exist/secret\"\n"
    );
    let config = accepted(text.as_bytes());
    assert_eq!(config.identities().len(), 1);

    // Paths that exist are not read either: the credential file below holds
    // the canary and invalid TOML, and the state directory is a directory.
    // A read would show as a changed answer or as the canary in the text.
    let manifest = env!("CARGO_MANIFEST_DIR");
    let secret = format!("{manifest}/tests/fixtures/secret-topology-values.toml");
    assert!(
        std::fs::read_to_string(&secret)
            .expect("the fixture exists")
            .contains(CANARY)
    );
    let text = format!(
        "{}\n[[identity]]\nid = \"a\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"{secret}\"\n",
        BASE.replace("/var/lib/pool", manifest),
    );
    let config = accepted(text.as_bytes());
    assert_eq!(config.identities().len(), 1);
    let debug = format!("{config:?} {:?}", config.identities()[0].credential());
    assert!(!debug.contains(CANARY), "{debug}");
}

#[test]
fn ac5_resolve_identity_matches_the_exact_pair_only() {
    let config = accepted(&fixture("valid-topology.toml"));
    let found = config
        .resolve_identity("orders_app", "orders")
        .map(|route| route.id().as_str());
    assert_eq!(found, Some("orders-app"));
    let found = config
        .resolve_identity("report_app", "orders")
        .map(|route| route.id().as_str());
    assert_eq!(found, Some("report-app"));
    for (principal, database) in [
        ("orders_app", "orders_reporting"),
        ("orders_app", "other"),
        ("Orders_app", "orders"),
        ("orders_app", "Orders"),
        ("ORDERS_APP", "ORDERS"),
        ("orders", "orders"),
        ("orders_ap", "orders"),
        ("orders_app", "order"),
        ("orders_app ", "orders"),
        ("", "orders"),
        ("orders_app", ""),
        ("", ""),
        ("*", "*"),
        ("report_app", "orders_reporting"),
    ] {
        assert!(
            config.resolve_identity(principal, database).is_none(),
            "{principal:?} {database:?}"
        );
    }
}

#[test]
fn ac5_resolve_identity_keeps_names_byte_for_byte() {
    let text = identity_with("principal", "\" Orders_App \"");
    let config = accepted(text.as_bytes());
    assert!(config.resolve_identity(" Orders_App ", "orders").is_some());
    assert!(config.resolve_identity("Orders_App", "orders").is_none());
    assert!(config.resolve_identity("orders_app", "orders").is_none());
}

#[test]
fn new_finding_codes_and_messages_are_stable() {
    let table = [
        (
            FindingCode::InvalidValue,
            "config.invalid_value",
            "a value is not in the allowed form or range",
        ),
        (
            FindingCode::DuplicateId,
            "config.duplicate_id",
            "an id is used more than once",
        ),
        (
            FindingCode::UnknownReference,
            "config.unknown_reference",
            "a reference does not name a matching entry",
        ),
        (
            FindingCode::DuplicateEndpoint,
            "config.duplicate_endpoint",
            "two nodes have the same host and port",
        ),
        (
            FindingCode::DuplicateIdentity,
            "config.duplicate_identity",
            "two identity entries have the same principal and database",
        ),
        (
            FindingCode::UnsupportedValue,
            "config.unsupported_value",
            "this value is not supported by this build",
        ),
    ];
    for (code, text, message) in table {
        assert_eq!(code.as_str(), text);
        assert_eq!(code.message(), message);
    }
}

// Edge cases beyond the table rows, added by /test. Each test names the
// criterion it locks.

/// A TOML basic string holding `text`, with `"`, `\` and control
/// characters escaped.
fn toml_string(text: &str) -> String {
    let mut out = String::from("\"");
    for scalar in text.chars() {
        match scalar {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c < ' ' || c == '\u{7f}' => out.push_str(&format!("\\u{:04X}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Asserts `text` gives exactly one `config.invalid_value` at `key`.
#[track_caller]
fn assert_invalid_at(text: &str, key: &'static str) {
    assert_eq!(
        single_code_and_key(text),
        (FindingCode::InvalidValue, Some(key)),
        "{text}"
    );
}

#[test]
fn ac2_port_edges() {
    // covers: 0003 AC-2
    for port in ["1", "65535", "0x1538", "5_432"] {
        assert_text_valid(&base_with_line(15, &format!("port = {port}")));
    }
    assert_eq!(
        findings(base_with_line(15, "port = 0").as_bytes()),
        [(FindingCode::InvalidValue, key_at("node.port", 15, 8))]
    );
    for port in ["65536", "4294967296", "5432.0", "true", "\"\"", "[5432]"] {
        let text = base_with_line(15, &format!("port = {port}"));
        assert_eq!(single_code(&text), FindingCode::Schema, "{text}");
    }
    // `-0` is the integer zero in TOML, so it is the zero port.
    assert_invalid_at(&base_with_line(15, "port = -0"), "node.port");
}

#[test]
fn ac2_host_edges() {
    // covers: 0003 AC-2
    for host in [
        "a".repeat(253),
        "::1".to_owned(),
        "fe80::1:2".to_owned(),
        "DB-1.Example.COM".to_owned(),
        "10.0.0.11.".to_owned(),
        "-".to_owned(),
    ] {
        assert_text_valid(&base_with_line(
            14,
            &format!("host = {}", toml_string(&host)),
        ));
    }
    for host in [
        String::new(),
        "a".repeat(254),
        "[::1]".to_owned(),
        "db/1".to_owned(),
        "db\t1".to_owned(),
        "dé.example".to_owned(),
        "db\u{200b}1".to_owned(),
        "10.0.0.11%en0".to_owned(),
    ] {
        assert_invalid_at(
            &base_with_line(14, &format!("host = {}", toml_string(&host))),
            "node.host",
        );
    }
}

#[test]
fn ac2_ac6_id_edges_through_validate() {
    // covers: 0003 AC-2, AC-6. The ID syntax counts bytes and allows only
    // ASCII; letters that look the same in another script are refused.
    for id in [
        "a".repeat(63),
        "a-".to_owned(),
        "a_".to_owned(),
        "a-_9".to_owned(),
    ] {
        assert_text_valid(
            &base_with_line(7, &format!("id = {}", toml_string(&id))).replace(
                "cluster = \"main\"",
                &format!("cluster = {}", toml_string(&id)),
            ),
        );
    }
    for id in [
        "a".repeat(64),
        "\u{440}g1".to_owned(),
        "\u{ff50}\u{ff47}1".to_owned(),
        "pg\u{200b}1".to_owned(),
        "pg1 ".to_owned(),
        " pg1".to_owned(),
        "pg1\n".to_owned(),
        "_pg1".to_owned(),
        "-pg1".to_owned(),
        "9pg".to_owned(),
        "p\u{e9}".to_owned(),
    ] {
        assert_invalid_at(
            &base_with_line(12, &format!("id = {}", toml_string(&id))),
            "node.id",
        );
        assert_invalid_at(&identity_with("id", &toml_string(&id)), "identity.id");
    }
}

#[test]
fn ac2_system_identifier_edges_through_validate() {
    // covers: 0003 AC-2
    for value in ["1", "9", "10", "18446744073709551615"] {
        assert_text_valid(&base_with_line(
            8,
            &format!("system_identifier = \"{value}\""),
        ));
    }
    for value in [
        "+1",
        "-1",
        " 1",
        "1 ",
        "0x1",
        "1e3",
        "1_000",
        "\u{661}",
        "99999999999999999999",
        "00",
    ] {
        assert_invalid_at(
            &base_with_line(8, &format!("system_identifier = {}", toml_string(value))),
            "cluster.system_identifier",
        );
    }
    let text = base_with_line(8, "system_identifier = 1.5");
    assert_eq!(single_code(&text), FindingCode::Schema, "{text}");
}

#[test]
fn ac2_authority_id_edges_through_validate() {
    // covers: 0003 AC-2
    let with = |value: &str| {
        base_with_line(
            4,
            &format!("state_directory = \"/var/lib/pool\"\nauthority_id = {value}"),
        )
    };
    let config = accepted(with(&format!("\"{}\"", "0".repeat(32))).as_bytes());
    assert_eq!(
        config
            .control()
            .and_then(|control| control.authority_id())
            .map(|id| *id.as_bytes()),
        Some([0; 16])
    );
    for value in [
        "\"0123456789abcdef0123456789abcdeg\"".to_owned(),
        format!("\"{}\"", "\u{e9}".repeat(16)),
        "\" 123456789abcdef0123456789abcdef\"".to_owned(),
        "\"0123456789abcdef0123456789abcdef\\n\"".to_owned(),
        "\"\"".to_owned(),
    ] {
        assert_eq!(
            findings(with(&value).as_bytes()),
            [(
                FindingCode::InvalidValue,
                Location::Key("control.authority_id")
            )],
            "{value}"
        );
    }
    let text = with("1");
    assert_eq!(single_code(&text), FindingCode::Schema, "{text}");
}

#[test]
fn ac2_name_syntax_edges() {
    // covers: 0003 AC-2, 0004 AC-6. 1 to 63 bytes of UTF-8, none of the
    // characters that spec 0004 lists under "Names and paths". Bytes are
    // counted, not characters.
    let valid = [
        "a".repeat(63),
        format!("a{}", "\u{e9}".repeat(31)),
        " ".to_owned(),
        "orders app".to_owned(),
        "\u{a0}".to_owned(),
        "\u{1f600}".repeat(15),
    ];
    for key in ["principal", "database", "backend_database", "backend_login"] {
        for name in &valid {
            assert_text_valid(&identity_with(key, &toml_string(name)));
        }
    }
    let invalid = [
        "\u{e9}".repeat(32),
        format!("{}a", "\u{1f600}".repeat(16)),
        "a\\tb".to_owned(),
        "a\\u0000".to_owned(),
        "a\\u007F".to_owned(),
        "a\\u001F".to_owned(),
        "a\\r".to_owned(),
        "a\\u0085".to_owned(),
        "a\\u009F".to_owned(),
        "a\\u061C".to_owned(),
        "a\\u202E".to_owned(),
        "a\\u2028".to_owned(),
    ];
    for (key, location) in [
        ("principal", "identity.principal"),
        ("database", "identity.database"),
        ("backend_database", "identity.backend_database"),
        ("backend_login", "identity.backend_login"),
    ] {
        for name in &invalid {
            // The escapes above are TOML escapes, so the value is written as is.
            assert_invalid_at(&identity_with(key, &format!("\"{name}\"")), location);
        }
    }
}

#[test]
fn ac2_path_syntax_edges() {
    // covers: 0003 AC-2. 1 to 4096 bytes, starts with `/`, no control
    // character. Nothing checks that the path exists.
    let long = format!("/{}", "p".repeat(4095));
    let too_long = format!("/{}", "p".repeat(4096));
    for path in ["/", "//x", "/a b/\u{e9}", long.as_str()] {
        assert_text_valid(&base_with_line(
            4,
            &format!("state_directory = {}", toml_string(path)),
        ));
        assert_text_valid(&identity_with("credential_file", &toml_string(path)));
    }
    for path in [
        "",
        "./pool",
        "~/pool",
        " /pool",
        "\\\\pool",
        too_long.as_str(),
    ] {
        assert_eq!(
            findings(
                base_with_line(4, &format!("state_directory = {}", toml_string(path))).as_bytes()
            ),
            [(
                FindingCode::InvalidValue,
                Location::Key("control.state_directory")
            )],
            "{path:?}"
        );
        assert_invalid_at(
            &identity_with("credential_file", &toml_string(path)),
            "identity.credential_file",
        );
    }
    for escaped in ["/a\\tb", "/a\\u007F", "/a\\nb"] {
        assert_invalid_at(
            &identity_with("credential_file", &format!("\"{escaped}\"")),
            "identity.credential_file",
        );
    }
}

#[test]
fn ac2_routing_profile_edges() {
    // covers: 0003 AC-2. Only the exact text "replica" is unsupported; any
    // other text but "primary" is invalid.
    for value in [
        "", " primary", "primary ", "PRIMARY", "REPLICA", "replica ", "Replica",
    ] {
        assert_invalid_at(
            &identity_with("routing_profile", &toml_string(value)),
            "identity.routing_profile",
        );
    }
    let config = accepted(identity_with("routing_profile", "\"primary\"").as_bytes());
    assert_eq!(
        config.identities()[0].routing_profile(),
        RoutingProfile::Primary
    );
    assert_eq!(RoutingProfile::Primary.as_str(), "primary");
    let config = accepted(identity_with("id", "\"orders-app\"").as_bytes());
    assert_eq!(
        config.identities()[0].routing_profile(),
        RoutingProfile::Primary
    );
}

#[test]
fn ac1_ac2_empty_and_inline_tables() {
    // covers: 0003 AC-1, AC-2
    for text in [
        "config_version = 1\ncontrol = {}\n",
        "config_version = 1\ncontrol = { state_directory = \"/p\" }\n",
        "config_version = 1\ncluster = []\n",
        "config_version = 1\nidentity = []\n[control]\nstate_directory = \"/p\"\n",
        "config_version = 1\ncontrol = {}\nnode = []\n",
    ] {
        assert_text_valid(text);
    }
    // A cluster without any [control] table misses the state directory.
    let text = "config_version = 1\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"h\"\nport = 1\n";
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::MissingKey,
            Location::Key("control.state_directory")
        )]
    );
    // An empty inline control table with a cluster misses it too.
    let text = format!("config_version = 1\ncontrol = {{}}\n{}", &text[19..]);
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::MissingKey,
            Location::Key("control.state_directory")
        )]
    );
    // An empty node entry misses its four keys, in key order, at its header.
    let text = base_plus("[[node]]\n");
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::MissingKey, key_at("node.id", 17, 1)),
            (FindingCode::MissingKey, key_at("node.cluster", 17, 1)),
            (FindingCode::MissingKey, key_at("node.host", 17, 1)),
            (FindingCode::MissingKey, key_at("node.port", 17, 1)),
        ]
    );
    // An empty identity entry misses its six required keys, in key order.
    let text = base_plus("[[identity]]\n");
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::MissingKey, key_at("identity.id", 17, 1)),
            (FindingCode::MissingKey, key_at("identity.principal", 17, 1)),
            (FindingCode::MissingKey, key_at("identity.database", 17, 1)),
            (FindingCode::MissingKey, key_at("identity.cluster", 17, 1)),
            (
                FindingCode::MissingKey,
                key_at("identity.backend_login", 17, 1)
            ),
            (
                FindingCode::MissingKey,
                key_at("identity.credential_file", 17, 1)
            ),
        ]
    );
    // An empty inline node is at its brace; a header with spaces before it is
    // at its first bracket.
    let text = "config_version = 1\nnode = [ {} ]\n";
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::MissingKey, key_at("node.id", 2, 10)),
            (FindingCode::MissingKey, key_at("node.cluster", 2, 10)),
            (FindingCode::MissingKey, key_at("node.host", 2, 10)),
            (FindingCode::MissingKey, key_at("node.port", 2, 10)),
        ]
    );
    let text = base_plus("  [[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"h\"\n");
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::MissingKey, key_at("node.port", 17, 3))]
    );
}

#[test]
fn ac3_findings_follow_the_validation_order_not_the_file_order() {
    // covers: 0003 AC-3. The identity comes first in the file, then the node,
    // then the cluster, and the control table last; the findings come in the
    // order of the validation steps, and the same on every run.
    let text = "config_version = 1\n\n[[identity]]\nid = \"Bad\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"/c\"\n\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"h\"\nport = 0\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"0\"\nexpected_writer = \"pg1\"\n\n[control]\nauthority_id = \"x\"\n";
    let want = [
        (
            FindingCode::MissingKey,
            Location::Key("control.state_directory"),
        ),
        (
            FindingCode::InvalidValue,
            Location::Key("control.authority_id"),
        ),
        (
            FindingCode::InvalidValue,
            key_at("cluster.system_identifier", 19, 21),
        ),
        (FindingCode::InvalidValue, key_at("node.port", 15, 8)),
        (FindingCode::InvalidValue, key_at("identity.id", 4, 6)),
    ];
    for _ in 0..3 {
        assert_eq!(findings(text.as_bytes()), want);
    }
}

#[test]
fn ac3_each_repeat_is_reported_at_each_later_entry() {
    // covers: 0003 AC-2, AC-3. Three nodes with one id give two findings;
    // a repeated identity id and a repeated pair give one finding each, the
    // id first (cross check 1 before 6).
    let node =
        |line: &str| format!("[[node]]\nid = \"pg1\"\ncluster = \"main\"\n{line}\nport = 5432\n");
    let text = base_plus(&format!(
        "{}\n{}",
        node("host = \"10.0.0.12\""),
        node("host = \"10.0.0.13\"")
    ));
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("node.id", 18, 6)),
            (FindingCode::DuplicateId, key_at("node.id", 24, 6)),
        ]
    );
    let entry = "[[identity]]\nid = \"a\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"/c\"\n";
    let text = base_plus(&format!("{entry}\n{entry}"));
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("identity.id", 26, 6)),
            (
                FindingCode::DuplicateIdentity,
                key_at("identity.principal", 27, 13)
            ),
        ]
    );
    // The same pair with a different backend is still a duplicate pair.
    let second = entry
        .replace("id = \"a\"", "id = \"b\"")
        .replace("backend_login = \"l\"", "backend_login = \"other\"");
    let text = base_plus(&format!("{entry}\n{second}"));
    assert_eq!(
        findings(text.as_bytes()),
        [(
            FindingCode::DuplicateIdentity,
            key_at("identity.principal", 27, 13)
        )]
    );
    // The same principal with another database, or the reverse, is fine.
    let other_db = second.replace("database = \"d\"", "database = \"d2\"");
    let other_principal = entry
        .replace("id = \"a\"", "id = \"c\"")
        .replace("principal = \"p\"", "principal = \"p2\"");
    assert_text_valid(&base_plus(&format!(
        "{entry}\n{other_db}\n{other_principal}"
    )));
}

#[test]
fn ac5_resolve_identity_refuses_names_that_only_look_alike() {
    // covers: 0003 AC-5. The pair is compared byte for byte: no Unicode
    // normalization, no case folding, no trimming, no prefix.
    let text = base_plus(
        "[[identity]]\nid = \"cafe\"\nprincipal = \"caf\u{e9}\"\ndatabase = \"orders\"\ncluster = \"main\"\nbackend_login = \"orders_rw\"\ncredential_file = \"/c\"\n\n[[identity]]\nid = \"cafe-reports\"\nprincipal = \"caf\u{e9}\"\ndatabase = \"reports\"\ncluster = \"main\"\nbackend_database = \"reports_db\"\nbackend_login = \"reports_ro\"\ncredential_file = \"/r\"\n",
    );
    let config = accepted(text.as_bytes());
    let found = |principal: &str, database: &str| {
        config
            .resolve_identity(principal, database)
            .map(|route| route.id().as_str())
    };
    assert_eq!(found("caf\u{e9}", "orders"), Some("cafe"));
    assert_eq!(found("caf\u{e9}", "reports"), Some("cafe-reports"));
    for (principal, database) in [
        ("cafe\u{301}", "orders"),
        ("CAF\u{c9}", "orders"),
        ("caf\u{e9}", "\u{43e}rders"),
        ("caf\u{e9}", "orders\0"),
        ("caf\u{e9}\0", "orders"),
        ("caf\u{e9}", "orders\t"),
        ("caf\u{e9}", "\u{ff4f}rders"),
        ("caf\u{e9}", "or\u{200b}ders"),
        ("caf", "orders"),
        ("caf\u{e9}", "ord"),
        ("caf\u{e9}", "orders_more"),
        ("caf\u{e9}", "reports_db"),
        ("orders_rw", "orders"),
        ("caf\u{e9}", "*"),
        ("%", "%"),
    ] {
        assert_eq!(
            found(principal, database),
            None,
            "{principal:?} {database:?}"
        );
    }
    // The route carries the backend values, with the default backend
    // database taken from `database`.
    let route = config
        .resolve_identity("caf\u{e9}", "orders")
        .expect("route");
    assert_eq!(route.backend_database(), "orders");
    assert_eq!(route.backend_login(), "orders_rw");
    assert_eq!(route.credential().path(), std::path::Path::new("/c"));
    let route = config
        .resolve_identity("caf\u{e9}", "reports")
        .expect("route");
    assert_eq!(route.backend_database(), "reports_db");
    assert_eq!(route.cluster().as_str(), "main");
}

#[test]
fn ac5_an_empty_identity_map_resolves_nothing() {
    // covers: 0003 AC-5
    for text in [
        "config_version = 1\n".to_owned(),
        "config_version = 1\nidentity = []\n".to_owned(),
        BASE.to_owned(),
    ] {
        let config = accepted(text.as_bytes());
        for (principal, database) in [("", ""), ("orders_app", "orders"), ("*", "*")] {
            assert!(config.resolve_identity(principal, database).is_none());
        }
    }
}

// Large files: time and positions. The four made inputs follow the text
// of `0003 AC-15` ("Validation time", first written in commit c72cdfc of
// arc-f3-spec) and the existing rule under "Keys" that the
// checks must not take time that grows with the square of the number of
// entries. Asked for by dev-f3-build as the regression tests of the fix
// "position lookup no longer rescans the file".

/// The position of the first byte of `marker` in `text`, by the rule of the
/// validation order of spec 0001: line = 1 + line feeds before the offset,
/// column = 1 + Unicode scalars from the line start (a CR counts).
fn position_of(text: &str, marker: &str) -> (u32, u32) {
    assert_eq!(
        text.matches(marker).count(),
        1,
        "marker {marker:?} is unique"
    );
    position_at(text, text.find(marker).expect("marker"))
}

fn position_at(text: &str, offset: usize) -> (u32, u32) {
    let before = &text[..offset];
    let line = 1 + before.matches('\n').count();
    let line_start = before.rfind('\n').map_or(0, |index| index + 1);
    let column = 1 + text[line_start..offset].chars().count();
    (
        u32::try_from(line).expect("line"),
        u32::try_from(column).expect("column"),
    )
}

/// Lines 1 to 9 of made input 1.
const AC15_HEAD: &str = "config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"7412345678901234567\"\nexpected_writer = \"n0\"\n";

const AC15_LIMIT: std::time::Duration = std::time::Duration::from_secs(10);

/// Made input 1: one `[[node]]` table per entry, as many as fit.
fn ac15_input_1() -> (String, usize) {
    let mut text = String::from(AC15_HEAD);
    let mut count = 0;
    loop {
        let entry = format!(
            "\n[[node]]\nid = \"n{count}\"\ncluster = \"main\"\nhost = \"h{count}.db.internal\"\nport = 5432\n"
        );
        if text.len() + entry.len() > MAX_CONFIG_BYTES {
            return (text, count);
        }
        text.push_str(&entry);
        count += 1;
    }
}

/// Made inputs 2 and 3: every entry inline on line 2, with this port.
fn ac15_inline(port: u16) -> (String, usize) {
    let tail = format!("]\n{}", &AC15_HEAD[19..]);
    let mut text = String::from("config_version = 1\nnode = [");
    let mut count = 0;
    loop {
        let comma = if count == 0 { "" } else { "," };
        let entry = format!(
            "{comma}{{id=\"n{count}\",cluster=\"main\",host=\"h{count}.db.internal\",port={port}}}"
        );
        if text.len() + entry.len() + tail.len() > MAX_CONFIG_BYTES {
            text.push_str(&tail);
            return (text, count);
        }
        text.push_str(&entry);
        count += 1;
    }
}

#[test]
fn ac15_made_inputs_have_the_listed_size() {
    // covers: 0003 AC-15. Fast: builds the inputs only.
    let sizes = [ac15_input_1(), ac15_inline(5432), ac15_inline(0)]
        .map(|(text, count)| (count, text.len()));
    assert_eq!(
        sizes,
        [
            (13_217, 1_048_511),
            (16_471, 1_048_558),
            (17_268, 1_048_559)
        ]
    );
}

#[test]
fn ac15_large_valid_inputs_validate_within_ten_seconds() {
    // covers: 0003 AC-15, 0003 AC-1. Inputs 1 and 2 are valid.
    // Each call is timed and checked before the next one starts.
    for (name, (text, count)) in [("input 1", ac15_input_1()), ("input 2", ac15_inline(5432))] {
        let started = std::time::Instant::now();
        let config = accepted(text.as_bytes());
        let elapsed = started.elapsed();
        assert!(elapsed < AC15_LIMIT, "{name}: validate took {elapsed:?}");
        assert_eq!(config.nodes().len(), count, "{name}");
    }
}

#[test]
fn ac15_one_finding_per_entry_on_one_line_at_each_value() {
    // covers: 0003 AC-15, 0003 AC-2, AC-3. Input 3: every entry
    // has port 0, so every entry gives `config.invalid_value` at `node.port`,
    // in file order, on line 2, at the column where its value starts.
    let (text, count) = ac15_inline(0);
    let started = std::time::Instant::now();
    let rejection = rejected(text.as_bytes());
    let elapsed = started.elapsed();
    assert!(elapsed < AC15_LIMIT, "input 3: validate took {elapsed:?}");
    let want: Vec<(FindingCode, Location)> = text
        .match_indices("port=0")
        .map(|(offset, _)| {
            let (line, column) = position_at(&text, offset + "port=".len());
            (FindingCode::InvalidValue, key_at("node.port", line, column))
        })
        .collect();
    assert_eq!(want.len(), count);
    assert_eq!(want[0].1, key_at("node.port", 2, 60));
    assert!(
        want.iter()
            .all(|(_, at)| matches!(at, Location::KeyAt { line: 2, .. }))
    );
    let found: Vec<(FindingCode, Location)> = rejection
        .findings()
        .iter()
        .map(|finding| (finding.code(), finding.location()))
        .collect();
    assert_eq!(found.len(), count);
    assert!(
        found == want,
        "the first difference: {:?}",
        found.iter().zip(&want).find(|(a, b)| a != b)
    );
}

#[test]
fn ac15_findings_follow_the_validation_order_against_the_file_order() {
    // covers: 0003 AC-15, 0003 AC-3. Input 4: the node checks of
    // step 4 come before the duplicate id of step 6.
    let text = format!(
        "{}\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.12\"\nport = 0\n",
        String::from_utf8(fixture("valid-topology-minimal.toml")).expect("utf-8")
    );
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::InvalidValue, key_at("node.port", 21, 8)),
            (FindingCode::DuplicateId, key_at("node.id", 18, 6)),
        ]
    );
}

#[test]
fn ac3_large_file_positions_stay_exact_deep_in_many_lines() {
    // covers: 0003 AC-2, AC-3. A repeated id and an invalid host near the
    // end of input 1, more than 79000 lines down.
    let (text, count) = ac15_input_1();
    let late = count - 10;
    let last = count - 1;
    let text = text
        .replace(&format!("id = \"n{late}\"\n"), "id = \"n5\"\n")
        .replace(&format!("\"h{last}.db.internal\""), "\"bad_host\"");
    let late_at = text.rfind("id = \"n5\"").expect("late id") + "id = ".len();
    let (dup_line, dup_column) = position_at(&text, late_at);
    let (host_line, host_column) = position_of(&text, "\"bad_host\"");
    assert!(host_line > 79_000, "{host_line}");
    assert_eq!((dup_column, host_column), (6, 8));
    let started = std::time::Instant::now();
    let found = findings(text.as_bytes());
    let elapsed = started.elapsed();
    assert!(elapsed < AC15_LIMIT, "validate took {elapsed:?}");
    assert_eq!(
        found,
        [
            (
                FindingCode::InvalidValue,
                key_at("node.host", host_line, host_column)
            ),
            (
                FindingCode::DuplicateId,
                key_at("node.id", dup_line, dup_column)
            ),
        ]
    );
}

#[test]
fn ac3_a_finding_after_a_long_comment_has_its_exact_position() {
    // covers: 0003 AC-2, AC-3. An entry after a comment of 900 KB on one
    // line.
    let text = format!(
        "config_version = 1\n# {}\n{}",
        "x".repeat(900_000),
        &BASE[19..]
    )
    .replace("port = 5432", "port = 0");
    let (line, column) = position_of(&text, "0\n");
    assert_eq!((line, column), (16, 8));
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::InvalidValue, key_at("node.port", line, column))]
    );
}

#[test]
fn ac3_positions_count_scalars_with_crlf_and_at_block_boundaries() {
    // covers: 0003 AC-2, AC-3. (d) CRLF line ends, multi byte characters in
    // a comment and in a value before the finding on the same line, and the
    // finding's offset on and next to a 256 byte boundary.
    let head = "config_version = 1\r\n# \u{e9}\u{20ac}\u{1f600} comment\r\n[control]\r\nstate_directory = \"/p\"\r\n[[cluster]]\r\nid = \"main\"\r\nsystem_identifier = \"1\"\r\nexpected_writer = \"pg1\"\r\nnode = [{ id = \"pg1\", cluster = \"main\", host = \"h\", port = 1 }]\r\n";
    // `node` after `[[cluster]]` would belong to the cluster; put it first.
    let head = head.replace(
        "[control]\r\n",
        "node = [{ id = \"pg1\", cluster = \"main\", host = \"h\", port = 1 }]\r\n[control]\r\n",
    );
    let head = head.replace(
        "expected_writer = \"pg1\"\r\nnode = [{ id = \"pg1\", cluster = \"main\", host = \"h\", port = 1 }]\r\n",
        "expected_writer = \"pg1\"\r\n",
    );
    // The identity line: a principal with multi byte characters, then the
    // invalid value. The identity array goes before [control] too.
    let line_start = "identity = [{ id = \"a\", principal = \"\u{e9}\u{20ac}\u{1f600}\", database = \"d\", cluster = \"main\", backend_login = \"l\", credential_file = ";
    for target in [255_usize, 256, 257, 511, 512, 513] {
        // Pad with an ASCII comment line so that the opening quote of the
        // bad value lands at byte offset `target`.
        let fixed = line_start.len() + "#\r\n".len();
        let (before_control, after_control) =
            head.split_at(head.find("[control]").expect("control"));
        let base_offset = before_control.len() + fixed;
        // Every target is reachable, so no target is skipped.
        assert!(base_offset <= target, "{base_offset} > {target}");
        let pad = "x".repeat(target - base_offset);
        let text =
            format!("{before_control}#{pad}\r\n{line_start}\"relative\" }}]\r\n{after_control}");
        assert_eq!(text.find("\"relative\""), Some(target), "{text}");
        let (line, column) = position_of(&text, "\"relative\"");
        // Line 1 the version, 2 the comment with é€😀, 3 the node, 4 the
        // padding comment, 5 the identity.
        assert_eq!(line, 5);
        assert_eq!(
            usize::try_from(column).expect("column"),
            1 + line_start.chars().count()
        );
        assert_eq!(
            findings(text.as_bytes()),
            [(
                FindingCode::InvalidValue,
                key_at("identity.credential_file", line, column)
            )],
            "target {target}"
        );
    }
}

// Gaps named by the review of 2026-11-15 (t1, t2, t4), added by /test. Each
// case fails if the rule it names is removed from the cross checks or from
// the finding text.

#[test]
fn ac2_a_repeated_cluster_id_is_reported_at_each_later_entry() {
    // covers: 0003 AC-2, AC-3 (review t1). Cross check 1 for clusters.
    let cluster = |sys: &str| {
        format!(
            "[[cluster]]\nid = \"main\"\nsystem_identifier = \"{sys}\"\nexpected_writer = \"pg1\"\n"
        )
    };
    let text = base_plus(&cluster("2"));
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::DuplicateId, key_at("cluster.id", 18, 6))]
    );
    let text = base_plus(&format!("{}\n{}", cluster("2"), cluster("3")));
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("cluster.id", 18, 6)),
            (FindingCode::DuplicateId, key_at("cluster.id", 23, 6)),
        ]
    );
}

#[test]
fn ac3_repeated_ids_are_reported_clusters_then_nodes_then_identities() {
    // covers: 0003 AC-3 (review t1). Cross check 1 runs over clusters, then
    // nodes, then identities, whatever the order of the tables in the file.
    let text = "config_version = 1

[[identity]]
id = \"a\"
principal = \"p\"
database = \"d\"
cluster = \"main\"
backend_login = \"l\"
credential_file = \"/c\"

[[identity]]
id = \"a\"
principal = \"p2\"
database = \"d\"
cluster = \"main\"
backend_login = \"l\"
credential_file = \"/c\"

[[node]]
id = \"pg1\"
cluster = \"main\"
host = \"10.0.0.11\"
port = 5432

[[node]]
id = \"pg1\"
cluster = \"main\"
host = \"10.0.0.12\"
port = 5432

[[cluster]]
id = \"main\"
system_identifier = \"1\"
expected_writer = \"pg1\"

[[cluster]]
id = \"main\"
system_identifier = \"2\"
expected_writer = \"pg1\"

[control]
state_directory = \"/var/lib/pool\"
";
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("cluster.id", 37, 6)),
            (FindingCode::DuplicateId, key_at("node.id", 26, 6)),
            (FindingCode::DuplicateId, key_at("identity.id", 12, 6)),
        ]
    );
}

#[test]
fn ac2_a_bad_cluster_id_skips_only_the_writer_check_of_that_cluster() {
    // covers: 0003 AC-2 (review t2, `topology.rs:731`). The expected_writer
    // check still runs for every other cluster.
    let bad = base_with_line(7, "id = \"Main\"");
    let text = format!(
        "{bad}\n[[cluster]]\nid = \"other\"\nsystem_identifier = \"2\"\nexpected_writer = \"pg1\"\n"
    );
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::InvalidValue, key_at("cluster.id", 7, 6)),
            (
                FindingCode::UnknownReference,
                key_at("cluster.expected_writer", 20, 19)
            ),
        ]
    );
    // The other cluster with a writer of its own gives no finding.
    let text = format!(
        "{bad}\n[[cluster]]\nid = \"other\"\nsystem_identifier = \"2\"\nexpected_writer = \"pg2\"\n\n[[node]]\nid = \"pg2\"\ncluster = \"other\"\nhost = \"10.0.0.12\"\nport = 5432\n"
    );
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::InvalidValue, key_at("cluster.id", 7, 6))]
    );
}

#[test]
fn ac2_a_node_with_a_bad_cluster_skips_every_writer_check() {
    // covers: 0003 AC-2 (review t2, `topology.rs:728`). A node whose cluster
    // failed its check skips cross check 3 as a whole.
    let text = base_with_line(13, "cluster = \"Main\"");
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::InvalidValue, key_at("node.cluster", 13, 11))]
    );
    // Also when the bad node is not the writer, and another cluster names a
    // writer that does not exist.
    let text = base_plus(
        "[[node]]\nid = \"pg2\"\ncluster = \"Main\"\nhost = \"10.0.0.12\"\nport = 5432\n\n[[cluster]]\nid = \"other\"\nsystem_identifier = \"2\"\nexpected_writer = \"pg9\"\n",
    );
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::InvalidValue, key_at("node.cluster", 19, 11))]
    );
}

#[test]
fn ac2_a_node_whose_port_failed_is_left_out_of_the_endpoint_check() {
    // covers: 0003 AC-2, AC-15 (review t2, `topology.rs:772`). Two nodes with
    // the same host and port 0 give the two port findings and nothing else.
    let second = |port: &str| {
        format!("[[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = {port}\n")
    };
    let text = format!("{}\n{}", base_with_line(15, "port = 0"), second("0"));
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::InvalidValue, key_at("node.port", 15, 8)),
            (FindingCode::InvalidValue, key_at("node.port", 21, 8)),
        ]
    );
    let text = format!("{}\n{}", base_with_line(15, "port = 0"), second("5432"));
    assert_eq!(
        findings(text.as_bytes()),
        [(FindingCode::InvalidValue, key_at("node.port", 15, 8))]
    );
}

/// The canary in positions other than a string value of a known key: keys
/// that are not in the schema, table names, comments, array entries, values
/// of the wrong type and a string that does not end. Each text is refused.
fn canary_positions() -> Vec<String> {
    let c = CANARY;
    let node = |line: &str| {
        format!(
            "[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5432\n{line}\n"
        )
    };
    let base_without_node = BASE
        .split("[[node]]")
        .next()
        .expect("the base has a node")
        .to_owned();
    vec![
        // A key that is not in the schema, inside each table.
        base_with_line(4, &format!("state_directory = \"/var/lib/pool\"\n{c} = 1")),
        base_with_line(9, &format!("expected_writer = \"pg1\"\n{c} = \"x\"")),
        format!("{base_without_node}{}", node(&format!("{c} = true"))),
        identity_with(c, "1"),
        format!("{base_without_node}{}", node(&format!("\"{c}\" = 1"))),
        base_with_line(
            4,
            &format!("state_directory = \"/var/lib/pool\"\n{c}.x = 1"),
        ),
        format!(
            "{base_without_node}node = [{{ id = \"pg1\", cluster = \"main\", host = \"h\", port = 1, {c} = 1 }}]\n"
        ),
        // A table name.
        base_plus(&format!("[{c}]\nx = 1\n")),
        base_plus(&format!("[[{c}]]\nx = 1\n")),
        base_plus(&format!("[control.{c}]\nx = 1\n")),
        base_plus(&format!("[\"{c}\"]\n")),
        // A comment in a file that is refused.
        format!("# {c}\n{}", base_with_line(15, &format!("port = 0 # {c}"))),
        // An array entry, and values of the wrong type.
        base_with_line(15, &format!("port = [\"{c}\"]")),
        base_with_line(15, &format!("port = \"{c}\"")),
        base_with_line(12, &format!("id = [\"{c}\"]")),
        base_with_line(14, &format!("host = {{ {c} = 1 }}")),
        base_with_line(8, &format!("system_identifier = [1, \"{c}\"]")),
        identity_with("routing_profile", &format!("[\"{c}\"]")),
        // A string that does not end, and a key with no value.
        base_with_line(14, &format!("host = \"{c}")),
        base_plus(&format!("[[identity]]\n{c}\n")),
    ]
}

#[test]
fn ac4_a_canary_outside_a_value_never_reaches_rejection_text() {
    // covers: 0003 AC-4 (review t4). Each text is refused, and neither the
    // Debug nor the Display of the rejection holds any part of the canary.
    for text in canary_positions() {
        let rejection = rejected(text.as_bytes());
        assert!(!rejection.findings().is_empty(), "{text}");
        for rendered in [format!("{rejection:?}"), rejection.to_string()] {
            assert!(!rendered.contains(CANARY), "{text}\n{rendered}");
            assert!(!rendered.contains("9f2c7b1e"), "{text}\n{rendered}");
        }
    }
}

// Spec 0004: the keys [listener], [limits], [probe], [[principal]], the
// monitor and TLS keys, the closed name rule, the limit on findings and the
// two views (0004 AC-5 to AC-8). Positions were read from the fixture files
// when they were made.

/// The serving base of spec 0004, as text.
fn serving_base() -> String {
    String::from_utf8(fixture("valid-serving.toml")).expect("utf-8")
}

/// The serving base with the first `old` replaced by `new`.
fn serving_with(old: &str, new: &str) -> String {
    let base = serving_base();
    assert!(base.contains(old), "{old:?} is in the serving base");
    base.replacen(old, new, 1)
}

/// The serving base with `table` added before the first cluster.
fn serving_with_table(table: &str) -> String {
    serving_with("[[cluster]]", &format!("{table}\n\n[[cluster]]"))
}

fn incomplete(location: Location) -> (FindingCode, Location) {
    (FindingCode::Incomplete, location)
}

fn view_findings(rejection: Rejection) -> Vec<(FindingCode, Location)> {
    assert_eq!(rejection.omitted(), 0);
    rejection
        .findings()
        .iter()
        .map(|finding| (finding.code(), finding.location()))
        .collect()
}

#[test]
fn ac5_serving_fixtures_are_accepted() {
    // covers: 0004 AC-5, AC-8.
    for name in [
        "valid-serving.toml",
        "valid-serving-limits.toml",
        "valid-serving-probe-only.toml",
        "secret-serving-values.toml",
    ] {
        let config = accepted(&fixture(name));
        assert_disabled(&config);
        assert!(config.probing().is_ok(), "{name}");
    }
    for name in [
        "valid-serving.toml",
        "valid-serving-limits.toml",
        "secret-serving-values.toml",
    ] {
        assert!(accepted(&fixture(name)).serving().is_ok(), "{name}");
    }
}

#[test]
fn ac5_each_invalid_serving_fixture_has_its_one_finding() {
    // covers: 0004 AC-5. The table under "Fixtures" in spec 0004.
    let cases = [
        (
            "invalid-serving-port.toml",
            FindingCode::InvalidValue,
            Location::Key("listener.port"),
        ),
        (
            "invalid-serving-listener-key.toml",
            FindingCode::MissingKey,
            Location::Key("listener.tls_private_key_file"),
        ),
        (
            "invalid-serving-limit.toml",
            FindingCode::InvalidValue,
            Location::Key("limits.session_max_count"),
        ),
        (
            "invalid-serving-expiry.toml",
            FindingCode::InvalidValue,
            Location::Key("probe.observation_expiry_ms"),
        ),
        (
            "invalid-serving-duplicate-principal.toml",
            FindingCode::DuplicatePrincipal,
            key_at("principal.name", 66, 8),
        ),
        (
            "invalid-serving-unknown-key.toml",
            FindingCode::Schema,
            Location::Position {
                line: 14,
                column: 1,
            },
        ),
        (
            "invalid-name-c1.toml",
            FindingCode::InvalidValue,
            key_at("identity.backend_login", 22, 17),
        ),
        (
            "invalid-name-direction.toml",
            FindingCode::InvalidValue,
            key_at("identity.database", 20, 12),
        ),
    ];
    for (name, code, location) in cases {
        assert_eq!(findings(&fixture(name)), [(code, location)], "{name}");
    }
}

#[test]
fn ac5_left_out_tables_and_keys_have_their_defaults() {
    // covers: 0004 AC-5. A file of spec 0003 has none of the new tables.
    let config = accepted(&fixture("valid-topology.toml"));
    assert!(config.listener().is_none());
    assert!(config.principals().is_empty());
    assert!(config.resolve_principal("orders_app").is_none());
    let limits = config.limits();
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
    let probe = config.probe();
    assert_eq!(probe.period(), Duration::from_millis(2000));
    assert_eq!(probe.timeout(), Duration::from_millis(1000));
    assert_eq!(probe.observation_expiry(), Duration::from_millis(6000));
    let cluster = &config.clusters()[0];
    assert_eq!(cluster.monitor_login(), None);
    assert_eq!(cluster.monitor_database(), "postgres");
    assert!(cluster.monitor_credential().is_none());
    for node in config.nodes() {
        assert_eq!(node.tls_ca_file(), None);
        assert_eq!(node.tls_server_name(), node.host());
    }
    // Empty tables are the same as absent ones.
    let empty = accepted(format!("{}\n[limits]\n[probe]\n", serving_base()).as_bytes());
    assert_eq!(empty.limits(), config.limits());
    assert_eq!(empty.probe(), config.probe());
}

#[test]
fn ac5_valid_serving_exposes_every_new_key() {
    // covers: 0004 AC-5, AC-8.
    let config = accepted(&fixture("valid-serving.toml"));
    let listener = config.listener().expect("a listener");
    assert_eq!(listener.host(), "0.0.0.0");
    assert_eq!(listener.port(), 6432);
    assert_eq!(
        listener.tls_certificate_file(),
        Path::new("/etc/pool/tls/server.crt")
    );
    assert_eq!(
        listener.tls_private_key().path(),
        Path::new("/etc/pool/tls/server.key")
    );
    let cluster = &config.clusters()[0];
    assert_eq!(cluster.monitor_login(), Some("pool_monitor"));
    assert_eq!(cluster.monitor_database(), "postgres");
    assert_eq!(
        cluster.monitor_credential().map(|secret| secret.path()),
        Some(Path::new("/etc/pool/secrets/pool_monitor"))
    );
    for node in config.nodes() {
        assert_eq!(node.tls_ca_file(), Some(Path::new("/etc/pool/tls/ca.crt")));
        assert_eq!(node.tls_server_name(), node.host());
    }
    let ids: Vec<&str> = config
        .principals()
        .iter()
        .map(|principal| principal.id().as_str())
        .collect();
    assert_eq!(ids, ["orders-app", "report-app"]);
    let orders = config.resolve_principal("orders_app").expect("orders_app");
    assert_eq!(orders.id().as_str(), "orders-app");
    assert_eq!(
        orders.verifier().path(),
        Path::new("/etc/pool/secrets/orders_app.scram")
    );
    assert_eq!(orders.session_max_count(), 1000);
    // Byte for byte, like `resolve_identity`.
    for name in ["Orders_app", "orders_app ", " orders_app", "orders", ""] {
        assert!(config.resolve_principal(name).is_none(), "{name:?}");
    }
}

#[test]
fn ac5_valid_serving_limits_has_every_lowest_value() {
    // covers: 0004 AC-5.
    let config = accepted(&fixture("valid-serving-limits.toml"));
    let limits = config.limits();
    assert_eq!(limits.session_max_count(), 1);
    assert_eq!(limits.handshake_max_count(), 1);
    assert_eq!(limits.backend_wait_max_count(), 0);
    assert_eq!(limits.node_backend_max_count(), 1);
    assert_eq!(limits.startup_timeout(), Duration::from_millis(100));
    assert_eq!(limits.backend_acquire_timeout(), Duration::from_millis(100));
    assert_eq!(limits.shutdown_timeout(), Duration::ZERO);
    assert_eq!(limits.startup_packet_max_bytes(), 512);
    assert_eq!(limits.frontend_message_max_bytes(), 1024);
    let probe = config.probe();
    assert_eq!(probe.period(), Duration::from_millis(100));
    assert_eq!(probe.timeout(), Duration::from_millis(100));
    assert_eq!(probe.observation_expiry(), Duration::from_millis(1000));
    let names: Vec<&str> = config
        .nodes()
        .iter()
        .map(|node| node.tls_server_name())
        .collect();
    assert_eq!(names, ["10.0.0.11", "10.0.0.12", "pg3.example.internal"]);
    // The principal default follows `limits.session_max_count`.
    for principal in config.principals() {
        assert_eq!(principal.session_max_count(), 1);
    }
}

/// Each number key of `[limits]` and `[probe]`: its table, its name, its
/// lowest and its highest allowed value, from the table under "Keys".
const NUMBER_KEYS: [(&str, &str, u64, u64); 12] = [
    ("limits", "session_max_count", 1, 100_000),
    ("limits", "handshake_max_count", 1, 10_000),
    ("limits", "backend_wait_max_count", 0, 100_000),
    ("limits", "node_backend_max_count", 1, 100_000),
    ("limits", "startup_timeout_ms", 100, 60_000),
    ("limits", "backend_acquire_timeout_ms", 100, 60_000),
    ("limits", "shutdown_timeout_ms", 0, 600_000),
    ("limits", "startup_packet_max_bytes", 512, 65_536),
    ("limits", "frontend_message_max_bytes", 1024, 1_073_741_823),
    ("probe", "period_ms", 100, 60_000),
    ("probe", "timeout_ms", 100, 30_000),
    ("probe", "observation_expiry_ms", 1000, 60_000),
];

/// The key path of a number key, as a finding names it.
fn number_key(table: &str, key: &str) -> &'static str {
    let path = format!("{table}.{key}");
    [
        "limits.session_max_count",
        "limits.handshake_max_count",
        "limits.backend_wait_max_count",
        "limits.node_backend_max_count",
        "limits.startup_timeout_ms",
        "limits.backend_acquire_timeout_ms",
        "limits.shutdown_timeout_ms",
        "limits.startup_packet_max_bytes",
        "limits.frontend_message_max_bytes",
        "probe.period_ms",
        "probe.timeout_ms",
        "probe.observation_expiry_ms",
    ]
    .into_iter()
    .find(|known| *known == path)
    .expect("a known key")
}

/// A `[probe]` table that keeps the cross rule true whatever one key holds.
fn probe_table(key: &str, value: &str) -> String {
    let mut lines = [
        "[probe]".to_owned(),
        "period_ms = 100".to_owned(),
        "timeout_ms = 100".to_owned(),
        "observation_expiry_ms = 60000".to_owned(),
    ];
    let line = format!("{key} = {value}");
    let index = lines
        .iter()
        .position(|existing| existing.starts_with(&format!("{key} =")))
        .expect("a probe key");
    lines[index] = line;
    lines.join("\n")
}

fn number_table(table: &str, key: &str, value: &str) -> String {
    if table == "probe" {
        probe_table(key, value)
    } else {
        format!("[{table}]\n{key} = {value}")
    }
}

#[test]
fn ac5_number_keys_accept_their_range_and_refuse_what_is_outside() {
    // covers: 0004 AC-5. Below or above the range is `config.invalid_value`;
    // what the type cannot hold, or another type, is `config.schema`.
    for (table, key, low, high) in NUMBER_KEYS {
        let path = number_key(table, key);
        for value in [low, high] {
            let text = serving_with_table(&number_table(table, key, &value.to_string()));
            if path == "probe.period_ms" && value == high {
                // A period of 60000 passes its own range, but no expiry can
                // cover it: the timeout is at least 100 and the expiry at
                // most 60000. Only the cross rule speaks.
                assert_eq!(
                    findings(text.as_bytes()),
                    [(
                        FindingCode::InvalidValue,
                        Location::Key("probe.observation_expiry_ms")
                    )]
                );
                continue;
            }
            assert_text_valid(&text);
        }
        let mut outside = vec![high + 1];
        if low > 0 {
            outside.push(low - 1);
        }
        for value in outside {
            let text = serving_with_table(&number_table(table, key, &value.to_string()));
            assert_eq!(
                findings(text.as_bytes()),
                [(FindingCode::InvalidValue, Location::Key(path))],
                "{path} = {value}"
            );
        }
        for value in ["-1", "4294967296", "\"5\"", "1.5", "true"] {
            let text = serving_with_table(&number_table(table, key, value));
            assert_eq!(single_code(&text), FindingCode::Schema, "{path} = {value}");
        }
    }
}

#[test]
fn ac5_the_expiry_must_cover_the_period_and_the_timeout() {
    // covers: 0004 AC-5 (point n2 of spec 0003 Follow-up 4).
    let at = |period: u32, timeout: u32, expiry: u32| {
        serving_with_table(&format!(
            "[probe]\nperiod_ms = {period}\ntimeout_ms = {timeout}\nobservation_expiry_ms = {expiry}"
        ))
    };
    assert_text_valid(&at(5000, 2000, 7000));
    assert_text_valid(&at(100, 100, 1000));
    assert_eq!(
        findings(at(5000, 2000, 6999).as_bytes()),
        [(
            FindingCode::InvalidValue,
            Location::Key("probe.observation_expiry_ms")
        )]
    );
    assert_eq!(
        findings(at(60_000, 30_000, 60_000).as_bytes()),
        [(
            FindingCode::InvalidValue,
            Location::Key("probe.observation_expiry_ms")
        )]
    );
    // A key that failed its own check is not judged again by the cross rule.
    assert_eq!(
        findings(at(70_000, 2000, 6000).as_bytes()),
        [(FindingCode::InvalidValue, Location::Key("probe.period_ms"))]
    );
    assert_eq!(
        findings(at(5000, 2000, 500).as_bytes()),
        [(
            FindingCode::InvalidValue,
            Location::Key("probe.observation_expiry_ms")
        )]
    );
    // The defaults count when a key is left out: 2000 + 1000 > 2500.
    assert_eq!(
        findings(serving_with_table("[probe]\nobservation_expiry_ms = 2999").as_bytes()),
        [(
            FindingCode::InvalidValue,
            Location::Key("probe.observation_expiry_ms")
        )]
    );
    assert_text_valid(&serving_with_table("[probe]\nobservation_expiry_ms = 3000"));
}

#[test]
fn ac5_listener_keys_are_required_together_and_checked_in_order() {
    // covers: 0004 AC-5.
    let listener = "[listener]\nhost = \"0.0.0.0\"\nport = 6432\ntls_certificate_file = \"/etc/pool/tls/server.crt\"\ntls_private_key_file = \"/etc/pool/tls/server.key\"\n";
    assert_eq!(
        findings(serving_with(listener, "[listener]\n").as_bytes()),
        [
            (FindingCode::MissingKey, Location::Key("listener.host")),
            (FindingCode::MissingKey, Location::Key("listener.port")),
            (
                FindingCode::MissingKey,
                Location::Key("listener.tls_certificate_file")
            ),
            (
                FindingCode::MissingKey,
                Location::Key("listener.tls_private_key_file")
            ),
        ]
    );
    let bad = "[listener]\nhost = \"0.0.0.0/8\"\nport = 0\ntls_certificate_file = \"server.crt\"\ntls_private_key_file = \"/etc/pool/\u{202e}key\"\n";
    assert_eq!(
        findings(serving_with(listener, bad).as_bytes()),
        [
            (FindingCode::InvalidValue, Location::Key("listener.host")),
            (FindingCode::InvalidValue, Location::Key("listener.port")),
            (
                FindingCode::InvalidValue,
                Location::Key("listener.tls_certificate_file")
            ),
            (
                FindingCode::InvalidValue,
                Location::Key("listener.tls_private_key_file")
            ),
        ]
    );
    for port in ["1", "65535"] {
        assert_text_valid(&serving_with("port = 6432", &format!("port = {port}")));
    }
    for port in ["65536", "-1", "\"6432\""] {
        let text = serving_with("port = 6432", &format!("port = {port}"));
        assert_eq!(single_code(&text), FindingCode::Schema, "{port}");
    }
    let text = serving_with("port = 6432", "port = 6432\nbacklog_count = 1");
    assert_eq!(single_code(&text), FindingCode::Schema);
}

#[test]
fn ac5_monitor_tls_and_principal_keys_are_checked_at_their_value() {
    // covers: 0004 AC-5. A finding inside an entry uses `Location::KeyAt`.
    let cases = [
        (
            "monitor_login = \"pool_monitor\"",
            "monitor_login = \"\"",
            key_at("cluster.monitor_login", 17, 17),
        ),
        (
            "monitor_login = \"pool_monitor\"",
            "monitor_login = \"pool_monitor\"\nmonitor_database = \"\"",
            key_at("cluster.monitor_database", 18, 20),
        ),
        (
            "monitor_credential_file = \"/etc/pool/secrets/pool_monitor\"",
            "monitor_credential_file = \"pool_monitor\"",
            key_at("cluster.monitor_credential_file", 18, 27),
        ),
        (
            "port = 5432\ntls_ca_file = \"/etc/pool/tls/ca.crt\"",
            "port = 5432\ntls_ca_file = \"ca.crt\"",
            key_at("node.tls_ca_file", 25, 15),
        ),
        (
            "port = 5432\ntls_ca_file = \"/etc/pool/tls/ca.crt\"",
            "port = 5432\ntls_ca_file = \"/etc/pool/tls/ca.crt\"\ntls_server_name = \"pg 1\"",
            key_at("node.tls_server_name", 26, 19),
        ),
        (
            "id = \"orders-app\"\nname",
            "id = \"Orders-App\"\nname",
            key_at("principal.id", 60, 6),
        ),
        (
            "name = \"orders_app\"",
            "name = \"orders\u{85}app\"",
            key_at("principal.name", 61, 8),
        ),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "verifier_file = \"orders_app.scram\"",
            key_at("principal.verifier_file", 62, 17),
        ),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"\nsession_max_count = 0",
            key_at("principal.session_max_count", 63, 21),
        ),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"\nsession_max_count = 100001",
            key_at("principal.session_max_count", 63, 21),
        ),
    ];
    for (old, new, location) in cases {
        let text = serving_with(old, new);
        assert_eq!(
            findings(text.as_bytes()),
            [(FindingCode::InvalidValue, location)],
            "{new}"
        );
    }
    let text = serving_with(
        "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
        "verifier_file = \"/etc/pool/secrets/orders_app.scram\"\nsession_max_count = 100000",
    );
    let config = accepted(text.as_bytes());
    let session_max: Vec<u32> = config
        .principals()
        .iter()
        .map(|principal| principal.session_max_count())
        .collect();
    assert_eq!(session_max, [100_000, 1000]);
    let text = serving_with(
        "monitor_login",
        "monitor_database = \"monitor\"\nmonitor_login",
    );
    let config = accepted(text.as_bytes());
    assert_eq!(config.clusters()[0].monitor_database(), "monitor");
}

#[test]
fn ac5_a_principal_lacking_its_keys_is_reported_at_its_entry() {
    // covers: 0004 AC-5.
    let text = format!("{}\n[[principal]]\n", serving_base());
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::MissingKey, key_at("principal.id", 69, 1)),
            (FindingCode::MissingKey, key_at("principal.name", 69, 1)),
            (
                FindingCode::MissingKey,
                key_at("principal.verifier_file", 69, 1)
            ),
        ]
    );
    let text = format!("{}\nprincipal = 1\n", "config_version = 1");
    assert_eq!(single_code(&text), FindingCode::Schema);
    let text = format!(
        "{}\n[[principal]]\nid = \"a\"\nrole = \"x\"\n",
        serving_base()
    );
    assert_eq!(single_code(&text), FindingCode::Schema);
}

#[test]
fn ac5_principal_repeats_are_reported_ids_then_names() {
    // covers: 0004 AC-5. Two cross checks at the end of item 6: duplicate
    // ids first, then duplicate names, each at the second and later entry.
    let extra = "\n[[principal]]\nid = \"orders-app\"\nname = \"report_app\"\nverifier_file = \"/v/1\"\n\n[[principal]]\nid = \"third\"\nname = \"orders_app\"\nverifier_file = \"/v/2\"\n";
    let text = format!("{}{extra}", serving_base());
    assert_eq!(
        findings(text.as_bytes()),
        [
            (FindingCode::DuplicateId, key_at("principal.id", 70, 6)),
            (
                FindingCode::DuplicatePrincipal,
                key_at("principal.name", 71, 8)
            ),
            (
                FindingCode::DuplicatePrincipal,
                key_at("principal.name", 76, 8)
            ),
        ]
    );
    // Names are compared byte for byte, so a different case is no repeat.
    let text = serving_with(
        "id = \"report-app\"\nname = \"report_app\"",
        "id = \"report-app\"\nname = \"Orders_app\"",
    );
    assert_text_valid(&text);
}

#[test]
fn ac5_new_checks_come_after_item_5_and_before_the_cross_checks() {
    // covers: 0004 AC-5, 0003 AC-3. The order of spec 0004: identity keys
    // (item 5), then listener, limits, probe, the monitor keys of each
    // cluster, the TLS keys of each node, each principal, then the cross
    // checks of spec 0003 and the two of spec 0004.
    let mut text = serving_base();
    for (old, new) in [
        ("port = 6432", "port = 0"),
        ("monitor_login = \"pool_monitor\"", "monitor_login = \"\""),
        (
            "port = 5433\ntls_ca_file = \"/etc/pool/tls/ca.crt\"",
            "port = 5433\ntls_ca_file = \"ca.crt\"",
        ),
        (
            "credential_file = \"/etc/pool/secrets/orders_rw\"",
            "credential_file = \"x\"",
        ),
        (
            "id = \"report-app\"\nname = \"report_app\"",
            "id = \"orders-app\"\nname = \"report_app\"",
        ),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "verifier_file = \"v\"",
        ),
        ("id = \"pg2\"", "id = \"pg1\""),
    ] {
        assert!(text.contains(old), "{old}");
        text = text.replacen(old, new, 1);
    }
    text = text.replacen(
        "[[cluster]]",
        "[limits]\nhandshake_max_count = 0\n\n[probe]\ntimeout_ms = 1\n\n[[cluster]]",
        1,
    );
    assert_eq!(
        findings(text.as_bytes()),
        [
            (
                FindingCode::InvalidValue,
                key_at("identity.credential_file", 53, 19)
            ),
            (FindingCode::InvalidValue, Location::Key("listener.port")),
            (
                FindingCode::InvalidValue,
                Location::Key("limits.handshake_max_count")
            ),
            (FindingCode::InvalidValue, Location::Key("probe.timeout_ms")),
            (
                FindingCode::InvalidValue,
                key_at("cluster.monitor_login", 23, 17)
            ),
            (
                FindingCode::InvalidValue,
                key_at("node.tls_ca_file", 45, 15)
            ),
            (
                FindingCode::InvalidValue,
                key_at("principal.verifier_file", 68, 17)
            ),
            (FindingCode::DuplicateId, key_at("node.id", 34, 6)),
            (FindingCode::DuplicateId, key_at("principal.id", 71, 6)),
        ]
    );
}

#[test]
fn ac5_ac6_every_new_string_value_keeps_the_canary_out_of_every_text() {
    // covers: 0004 AC-6, SEC-11. The canary is in every new string value.
    let config = accepted(&fixture("secret-serving-values.toml"));
    let listener = config.listener().expect("a listener");
    assert!(listener.host().contains(CANARY));
    let mut texts = vec![
        format!("{config:?}"),
        format!("{listener:?}"),
        format!("{:?}", listener.tls_private_key()),
        format!("{:?}", config.limits()),
        format!("{:?}", config.probe()),
        format!("{:?}", config.probing().expect("probing")),
        format!("{:?}", config.serving().expect("serving")),
    ];
    for cluster in config.clusters() {
        texts.push(format!("{cluster:?}"));
    }
    for node in config.nodes() {
        texts.push(format!("{node:?}"));
    }
    for principal in config.principals() {
        texts.push(format!("{principal:?} {:?}", principal.verifier()));
    }
    for text in texts {
        assert!(!text.contains(CANARY), "{text}");
        assert!(!text.contains("6432"), "{text}");
        assert!(!text.contains("0.0.0.0"), "{text}");
    }
    assert!(config.resolve_principal("CANARY-9f2c7b1e-orders").is_some());
    // The same values, each made invalid, never reach the rejection.
    let mut text = String::from_utf8(fixture("secret-serving-values.toml")).expect("utf-8");
    text = text.replace("CANARY-9f2c7b1e", "CANARY-9f2c7b1e\u{2028}");
    let rejection = rejected(text.as_bytes());
    assert!(rejection.findings().len() > 10);
    for text in [format!("{rejection:?}"), rejection.to_string()] {
        assert!(!text.contains(CANARY), "{text}");
    }
}

/// The scalar values that no name and no path may hold, written out from
/// the list under "Names and paths" in spec 0004.
fn refused_scalars() -> Vec<char> {
    (0_u32..=0x1f)
        .chain(0x7f..=0x9f)
        .chain([0x61c, 0x200e, 0x200f])
        .chain(0x202a..=0x202e)
        .chain(0x2066..=0x2069)
        .chain([0x2028, 0x2029])
        .map(|scalar| char::from_u32(scalar).expect("a scalar"))
        .collect()
}

#[test]
fn ac6_every_name_and_path_key_refuses_each_listed_character() {
    // covers: 0004 AC-6. Every key with name or path syntax, old and new.
    // The value holds the canary too, which must never reach the text.
    let keys: [(&str, &str, &str, bool); 14] = [
        (
            "state_directory = \"/var/lib/pool\"",
            "state_directory",
            "control.state_directory",
            true,
        ),
        (
            "monitor_login = \"pool_monitor\"",
            "monitor_login",
            "cluster.monitor_login",
            false,
        ),
        (
            "monitor_login",
            "monitor_database",
            "cluster.monitor_database",
            false,
        ),
        (
            "monitor_credential_file = \"/etc/pool/secrets/pool_monitor\"",
            "monitor_credential_file",
            "cluster.monitor_credential_file",
            true,
        ),
        (
            "tls_ca_file = \"/etc/pool/tls/ca.crt\"",
            "tls_ca_file",
            "node.tls_ca_file",
            true,
        ),
        (
            "principal = \"orders_app\"",
            "principal",
            "identity.principal",
            false,
        ),
        (
            "database = \"orders\"",
            "database",
            "identity.database",
            false,
        ),
        (
            "backend_database = \"orders_reporting\"",
            "backend_database",
            "identity.backend_database",
            false,
        ),
        (
            "backend_login = \"orders_rw\"",
            "backend_login",
            "identity.backend_login",
            false,
        ),
        (
            "credential_file = \"/etc/pool/secrets/orders_rw\"",
            "credential_file",
            "identity.credential_file",
            true,
        ),
        ("name = \"orders_app\"", "name", "principal.name", false),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "verifier_file",
            "principal.verifier_file",
            true,
        ),
        (
            "tls_certificate_file = \"/etc/pool/tls/server.crt\"",
            "tls_certificate_file",
            "listener.tls_certificate_file",
            true,
        ),
        (
            "tls_private_key_file = \"/etc/pool/tls/server.key\"",
            "tls_private_key_file",
            "listener.tls_private_key_file",
            true,
        ),
    ];
    for (old, key, path, is_path) in keys {
        for scalar in refused_scalars() {
            let value = if is_path {
                format!("/a{scalar}{CANARY}")
            } else {
                format!("a{scalar}{CANARY}")
            };
            let line = format!("{key} = {}", toml_string(&value));
            // `monitor_database` is not in the base, so it is added.
            let new = if key == "monitor_database" {
                format!("{line}\nmonitor_login")
            } else {
                line
            };
            let text = serving_with(old, &new);
            let rejection = rejected(text.as_bytes());
            let found: Vec<FindingCode> = rejection
                .findings()
                .iter()
                .map(|finding| finding.code())
                .collect();
            assert_eq!(found, [FindingCode::InvalidValue], "{path} {scalar:?}");
            let at = match rejection.findings()[0].location() {
                Location::Key(key) | Location::KeyAt { key, .. } => key,
                other => panic!("{other:?}"),
            };
            assert_eq!(at, path, "{scalar:?}");
            let shown = format!("{rejection:?} {rejection}");
            assert!(!shown.contains(CANARY), "{shown}");
        }
    }
}

#[test]
fn ac6_the_neighbours_of_each_listed_range_stay_allowed() {
    // covers: 0004 AC-6. The list is closed: the scalar just before and just
    // after each range is still allowed in a name.
    for scalar in [
        '\u{a0}', '\u{61b}', '\u{61d}', '\u{200d}', '\u{2010}', '\u{2027}', '\u{202f}', '\u{2065}',
        '\u{206a}',
    ] {
        let text = identity_with("backend_login", &toml_string(&format!("a{scalar}b")));
        assert_text_valid(&text);
    }
}

/// Made input 5 of spec 0004: `identity = [`, then empty entries on line 2,
/// as many as fit in the size limit.
fn made_input_5() -> (String, usize) {
    let head = "config_version = 1\nidentity = [";
    let tail = "]\n";
    let mut text = String::from(head);
    let mut count = 0;
    loop {
        let entry = if count == 0 { "{}" } else { ",{}" };
        if text.len() + entry.len() + tail.len() > MAX_CONFIG_BYTES {
            text.push_str(tail);
            return (text, count);
        }
        text.push_str(entry);
        count += 1;
    }
}

#[test]
fn ac7_validate_keeps_the_first_twenty_thousand_and_counts_the_rest() {
    // covers: 0004 AC-7. Made input 5: six findings per entry.
    let (text, count) = made_input_5();
    // One more entry (three bytes) would not fit.
    assert!(text.len() <= MAX_CONFIG_BYTES && text.len() + 3 > MAX_CONFIG_BYTES);
    let started = std::time::Instant::now();
    let rejection = rejected(text.as_bytes());
    let elapsed = started.elapsed();
    assert!(elapsed < AC15_LIMIT, "validate took {elapsed:?}");
    assert_eq!(rejection.findings().len(), MAX_FINDINGS);
    assert_eq!(MAX_FINDINGS, 20_000);
    let total = u64::try_from(count * 6).expect("count");
    assert_eq!(rejection.omitted(), total - 20_000);
    // The kept findings are the first ones in the validation order: the six
    // keys of the first entry, then the next entry, and so on.
    let keys = [
        "identity.id",
        "identity.principal",
        "identity.database",
        "identity.cluster",
        "identity.backend_login",
        "identity.credential_file",
    ];
    for (index, finding) in rejection.findings().iter().enumerate() {
        let entry = index / 6;
        let column = u32::try_from(13 + entry * 3).expect("column");
        assert_eq!(
            (finding.code(), finding.location()),
            (FindingCode::MissingKey, key_at(keys[index % 6], 2, column)),
            "finding {index}"
        );
    }
    assert_eq!(rejection.to_string().lines().count(), MAX_FINDINGS);
}

#[test]
fn ac7_at_twenty_thousand_nothing_is_omitted_and_one_more_is_counted() {
    // covers: 0004 AC-7. 5000 empty nodes give 4 findings each.
    let nodes = |version: u32, count: usize| {
        let entries = vec!["{}"; count].join(",");
        format!("config_version = {version}\nnode = [{entries}]\n")
    };
    let rejection = rejected(nodes(1, 5000).as_bytes());
    assert_eq!(
        (rejection.findings().len(), rejection.omitted()),
        (20_000, 0)
    );
    let rejection = rejected(nodes(2, 5000).as_bytes());
    assert_eq!(
        (rejection.findings().len(), rejection.omitted()),
        (20_000, 1)
    );
    assert_eq!(
        rejection.findings()[0].location(),
        Location::Key("config_version")
    );
    assert_eq!(
        rejection.findings()[19_999].location(),
        key_at("node.host", 2, u32::try_from(9 + 4999 * 3).expect("column"))
    );
    let rejection = rejected(nodes(1, 4999).as_bytes());
    assert_eq!(
        (rejection.findings().len(), rejection.omitted()),
        (19_996, 0)
    );
}

#[test]
fn ac8_views_need_what_each_command_needs() {
    // covers: 0004 AC-8. The rows of the table under "What a command needs",
    // in its order. Each missing piece is one `config.incomplete`.
    let probe_only = accepted(&fixture("valid-serving-probe-only.toml"));
    let probing = probe_only.probing().expect("probing");
    assert_eq!(probing.state_directory(), Path::new("/var/lib/pool"));
    assert_eq!(probing.config().clusters().len(), 1);
    assert_eq!(
        view_findings(probe_only.serving().expect_err("serving")),
        [
            incomplete(Location::Key("listener.host")),
            incomplete(key_at("identity.principal", 35, 1)),
            incomplete(key_at("identity.principal", 43, 1)),
        ]
    );

    let topology = accepted(&fixture("valid-topology.toml"));
    let both = [
        incomplete(key_at("cluster.monitor_login", 7, 1)),
        incomplete(key_at("cluster.monitor_credential_file", 7, 1)),
        incomplete(key_at("node.tls_ca_file", 12, 1)),
        incomplete(key_at("node.tls_ca_file", 18, 1)),
        incomplete(key_at("node.tls_ca_file", 24, 1)),
    ];
    assert_eq!(
        view_findings(topology.probing().expect_err("probing")),
        both
    );
    let mut serving = both.to_vec();
    serving.extend([
        incomplete(Location::Key("listener.host")),
        incomplete(key_at("identity.principal", 30, 1)),
        incomplete(key_at("identity.principal", 38, 1)),
    ]);
    assert_eq!(
        view_findings(topology.serving().expect_err("serving")),
        serving
    );

    let minimal = accepted(&fixture("valid-minimal.toml"));
    assert_eq!(
        view_findings(minimal.probing().expect_err("probing")),
        [
            incomplete(Location::Key("control.state_directory")),
            incomplete(Location::Key("cluster.id")),
        ]
    );
    assert_eq!(
        view_findings(minimal.serving().expect_err("serving")),
        [
            incomplete(Location::Key("control.state_directory")),
            incomplete(Location::Key("cluster.id")),
            incomplete(Location::Key("listener.host")),
            incomplete(Location::Key("identity.id")),
        ]
    );

    let config = accepted(&fixture("valid-serving.toml"));
    let serving = config.serving().expect("serving");
    assert_eq!(serving.state_directory(), Path::new("/var/lib/pool"));
    assert_eq!(serving.listener().port(), 6432);
    assert!(std::ptr::eq(serving.config(), &config));
    assert!(std::ptr::eq(
        serving.listener(),
        config.listener().expect("listener")
    ));
    assert_eq!(
        format!("{serving:?}"),
        "ServingView { clusters: 1, nodes: 3, identities: 2, principals: 2 }"
    );
    assert_eq!(
        format!("{:?}", config.probing().expect("probing")),
        "ProbingView { clusters: 1, nodes: 3 }"
    );
}

#[test]
fn ac8_one_missing_monitor_key_or_principal_gives_one_finding() {
    // covers: 0004 AC-8.
    let cases = [
        (
            "monitor_login = \"pool_monitor\"\n",
            vec![incomplete(key_at("cluster.monitor_login", 13, 1))],
        ),
        (
            "monitor_credential_file = \"/etc/pool/secrets/pool_monitor\"\n",
            vec![incomplete(key_at("cluster.monitor_credential_file", 13, 1))],
        ),
        (
            "port = 5433\ntls_ca_file = \"/etc/pool/tls/ca.crt\"\n",
            vec![incomplete(key_at("node.tls_ca_file", 34, 1))],
        ),
    ];
    for (old, want) in cases {
        let new = old.split_once('\n').map_or("", |(first, _)| {
            if first.starts_with("port") {
                "port = 5433\n"
            } else {
                ""
            }
        });
        let text = serving_with(old, new);
        let config = accepted(text.as_bytes());
        assert_eq!(
            view_findings(config.probing().expect_err("probing")),
            want,
            "{old}"
        );
        assert_eq!(
            view_findings(config.serving().expect_err("serving")),
            want,
            "{old}"
        );
    }
    // A principal of another name does not stand in for the one an identity
    // names: the lookup is byte for byte.
    let text = serving_with("name = \"report_app\"", "name = \"Report_app\"");
    let config = accepted(text.as_bytes());
    assert!(config.probing().is_ok());
    assert_eq!(
        view_findings(config.serving().expect_err("serving")),
        [incomplete(key_at("identity.principal", 49, 1))]
    );
}

#[test]
fn ac8_validate_never_gives_incomplete() {
    // covers: 0004 AC-8. `pool check-config` does not report these.
    for name in [
        "valid-minimal.toml",
        "valid-topology.toml",
        "valid-serving-probe-only.toml",
    ] {
        assert!(validate(&fixture(name)).is_ok(), "{name}");
    }
}

#[test]
fn spec_0004_finding_codes_and_messages_are_stable() {
    // covers: 0004 AC-5, AC-8. The table under "Findings" in spec 0004.
    assert_eq!(
        FindingCode::DuplicatePrincipal.as_str(),
        "config.duplicate_principal"
    );
    assert_eq!(
        FindingCode::DuplicatePrincipal.message(),
        "two principal entries have the same name"
    );
    assert_eq!(FindingCode::Incomplete.as_str(), "config.incomplete");
    assert_eq!(
        FindingCode::Incomplete.message(),
        "a key that this command needs is missing"
    );
}
