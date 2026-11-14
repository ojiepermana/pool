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
    // The unknown key starts after a multibyte quoted key on the same line,
    // so a byte count would give a larger column than a character count.
    let text = "config_version = 1\n\"é\" = 1\n";
    let found = findings(text.as_bytes());
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].0, FindingCode::Schema);
    match found[0].1 {
        Location::Position { line, column } => {
            assert_eq!(line, 2);
            // The line is 8 bytes but only 7 characters long.
            assert!((1..=7).contains(&column), "{:?}", found[0].1);
        }
        other => panic!("expected a position, got {other:?}"),
    }
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
    let text = "config_version = 2\n[listener]\nport = 6432\nresult_cache.enabled = true\n[automatic_promotion\n";
    assert_positioned(text.as_bytes(), FindingCode::Syntax, 5);
}

#[test]
fn ac7_a_schema_error_wins_over_meaning_findings() {
    let text = "config_version = 2\nresult_cache.enabled = true\n[listener]\nport = 6432\n";
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
    assert_eq!(findings(&bytes), findings(&bytes));
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
    assert!(debug.contains("Disabled"), "{debug}");
    assert_eq!(config.automatic_promotion().as_str(), "disabled");
    assert_eq!(config.result_cache().as_str(), "disabled");
}
