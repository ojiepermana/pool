//! The built `pool` binary: exit status, exact stdout and stderr in both
//! formats (spec 0001 AC-3 to AC-7, AC-9 to AC-11).

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../crates/pool-config/tests/fixtures/"
);
const CANARY: &str = "CANARY-9f2c7b1e";
const CAPABILITY_MESSAGE: &str =
    "this capability is not supported by this build; remove the key or set it to false";

const VALID_HUMAN: &str = "configuration valid\nconfig_version: 1\nresult_cache: disabled\nautomatic_promotion: disabled\n";
const VALID_JSON: &str = "{\"output_version\":1,\"command\":\"check-config\",\"result\":\"valid\",\"config_version\":1,\"capabilities\":{\"result_cache\":\"disabled\",\"automatic_promotion\":\"disabled\"},\"findings\":[]}\n";

const HELP: &str = "\
POOL daemon

Usage:
  pool check-config --config <PATH> [--format <human|json>]
  pool --help
  pool --version

Commands:
  check-config  Validate a configuration file offline and exit

Options for check-config:
  --config <PATH>          Configuration file to validate (TOML)
  --format <human|json>    Output format (default: human)

Exit status:
  0  success
  1  the input was rejected
  2  usage error
  3  the command could not complete
";

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn pool<I, S>(args: I) -> Run
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output: Output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("run pool");
    Run {
        code: output.status.code().expect("exit status, not a signal"),
        stdout: String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("stderr is UTF-8"),
    }
}

fn fixture(name: &str) -> String {
    format!("{FIXTURES}{name}")
}

fn check(path: &str, format: &str) -> Run {
    pool(["check-config", "--config", path, "--format", format])
}

/// A file written under the target temp dir with a name of its own.
fn made_file(name: &str, bytes: &[u8]) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    std::fs::write(&path, bytes).expect("write made file");
    path
}

fn invalid_json(findings: &str) -> String {
    format!(
        "{{\"output_version\":1,\"command\":\"check-config\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings\":[{findings}]}}\n"
    )
}

fn key_finding_json(code: &str, key: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{code}\",\"key\":\"{key}\",\"line\":null,\"column\":null,\"message\":\"{message}\"}}"
    )
}

fn none_finding_json(code: &str, message: &str) -> String {
    format!(
        "{{\"code\":\"{code}\",\"key\":null,\"line\":null,\"column\":null,\"message\":\"{message}\"}}"
    )
}

fn error_json(code: &str, message: &str) -> String {
    format!(
        "{{\"output_version\":1,\"command\":\"check-config\",\"result\":\"error\",\"config_version\":null,\"capabilities\":null,\"findings\":[{}]}}\n",
        none_finding_json(code, message)
    )
}

fn assert_run(run: &Run, code: i32, stdout: &str, stderr: &str) {
    assert_eq!(
        run.code, code,
        "stdout {:?} stderr {:?}",
        run.stdout, run.stderr
    );
    assert_eq!(run.stdout, stdout);
    assert_eq!(run.stderr, stderr);
}

/// Exact output for a file rejected with one finding at a key.
fn assert_key_rejection(path: &str, code: &str, key: &str, message: &str) {
    assert_run(
        &check(path, "human"),
        1,
        &format!("configuration invalid\nfindings: 1\n{code} at {key}: {message}\n"),
        "",
    );
    assert_run(
        &check(path, "json"),
        1,
        &invalid_json(&key_finding_json(code, key, message)),
        "",
    );
}

/// Exact output for a file rejected with one finding of no location.
fn assert_none_rejection(path: &str, code: &str, message: &str) {
    assert_run(
        &check(path, "human"),
        1,
        &format!("configuration invalid\nfindings: 1\n{code}: {message}\n"),
        "",
    );
    assert_run(
        &check(path, "json"),
        1,
        &invalid_json(&none_finding_json(code, message)),
        "",
    );
}

/// Checks a positioned finding: the JSON gives the column, the human line
/// must then match exactly.
fn assert_positioned_rejection(path: &str, code: &str, line: u64, message: &str) {
    let json = check(path, "json");
    assert_eq!(json.code, 1);
    assert_eq!(json.stderr, "");
    assert!(json.stdout.ends_with('\n') && json.stdout.lines().count() == 1);
    let value: serde_json::Value = serde_json::from_str(&json.stdout).expect("one JSON object");
    assert_eq!(value["result"], "invalid");
    let findings = value["findings"].as_array().expect("findings array");
    assert_eq!(findings.len(), 1);
    let finding = &findings[0];
    assert_eq!(finding["code"], code);
    assert!(finding["key"].is_null());
    assert_eq!(finding["line"].as_u64(), Some(line));
    let column = finding["column"].as_u64().expect("column");
    assert!(column >= 1);
    assert_eq!(finding["message"], message);
    // serde_json::Value sorts keys, so field order is checked on the raw line.
    assert_eq!(
        json.stdout,
        invalid_json(&format!(
            "{{\"code\":\"{code}\",\"key\":null,\"line\":{line},\"column\":{column},\"message\":\"{message}\"}}"
        ))
    );

    assert_run(
        &check(path, "human"),
        1,
        &format!(
            "configuration invalid\nfindings: 1\n{code} at line {line}, column {column}: {message}\n"
        ),
        "",
    );
}

#[test]
fn valid_fixtures_report_valid_in_both_formats() {
    for name in [
        "valid-minimal.toml",
        "valid-capabilities-disabled.toml",
        "valid-capabilities-empty.toml",
    ] {
        let path = fixture(name);
        assert_run(&check(&path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(&path, "json"), 0, VALID_JSON, "");
        assert_run(
            &pool(["check-config", "--config", &path]),
            0,
            VALID_HUMAN,
            "",
        );
    }
}

#[test]
fn reserved_capabilities_enabled_are_refused() {
    assert_key_rejection(
        &fixture("invalid-result-cache-enabled.toml"),
        "config.unsupported_capability",
        "result_cache.enabled",
        CAPABILITY_MESSAGE,
    );
    assert_key_rejection(
        &fixture("invalid-automatic-promotion-enabled.toml"),
        "config.unsupported_capability",
        "automatic_promotion.enabled",
        CAPABILITY_MESSAGE,
    );
}

#[test]
fn multiple_findings_are_reported_in_one_run() {
    let path = fixture("invalid-multiple.toml");
    assert_run(
        &check(&path, "human"),
        1,
        &format!(
            "configuration invalid\nfindings: 3\n\
             config.unsupported_version at config_version: this build supports config_version 1 only\n\
             config.unsupported_capability at result_cache.enabled: {CAPABILITY_MESSAGE}\n\
             config.unsupported_capability at automatic_promotion.enabled: {CAPABILITY_MESSAGE}\n"
        ),
        "",
    );
    let findings = [
        key_finding_json(
            "config.unsupported_version",
            "config_version",
            "this build supports config_version 1 only",
        ),
        key_finding_json(
            "config.unsupported_capability",
            "result_cache.enabled",
            CAPABILITY_MESSAGE,
        ),
        key_finding_json(
            "config.unsupported_capability",
            "automatic_promotion.enabled",
            CAPABILITY_MESSAGE,
        ),
    ]
    .join(",");
    assert_run(&check(&path, "json"), 1, &invalid_json(&findings), "");
}

#[test]
fn other_invalid_fixtures_fail_with_the_listed_finding() {
    assert_positioned_rejection(
        &fixture("invalid-syntax.toml"),
        "config.syntax",
        2,
        "the file is not valid TOML",
    );
    assert_positioned_rejection(
        &fixture("invalid-unknown-key.toml"),
        "config.schema",
        3,
        "unknown key, or a value of the wrong type or range",
    );
    assert_positioned_rejection(
        &fixture("invalid-wrong-type.toml"),
        "config.schema",
        4,
        "unknown key, or a value of the wrong type or range",
    );
    for name in ["invalid-missing-version.toml", "invalid-empty.toml"] {
        assert_key_rejection(
            &fixture(name),
            "config.missing_key",
            "config_version",
            "a required key is missing",
        );
    }
    assert_key_rejection(
        &fixture("invalid-unsupported-version.toml"),
        "config.unsupported_version",
        "config_version",
        "this build supports config_version 1 only",
    );
}

#[test]
fn made_inputs_check_utf8_and_the_inclusive_size_limit() {
    let not_utf8 = made_file("check-config-not-utf8.toml", &[0xFF, 0xFE, 0xFD]);
    assert_none_rejection(
        not_utf8.to_str().expect("path"),
        "config.not_utf8",
        "the file is not valid UTF-8",
    );

    let too_large = made_file("check-config-too-large.toml", &vec![b'#'; 1_048_577]);
    assert_none_rejection(
        too_large.to_str().expect("path"),
        "config.too_large",
        "the file is larger than 1048576 bytes",
    );

    let mut at_limit = vec![b'#'; 1_048_556];
    at_limit.extend_from_slice(b"\nconfig_version = 1\n");
    assert_eq!(at_limit.len(), 1_048_576);
    let at_limit = made_file("check-config-at-limit.toml", &at_limit);
    let at_limit = at_limit.to_str().expect("path");
    assert_run(&check(at_limit, "human"), 0, VALID_HUMAN, "");
    assert_run(&check(at_limit, "json"), 0, VALID_JSON, "");
}

#[test]
fn secret_fixtures_never_leak_the_canary() {
    for (name, code, line) in [
        ("secret-unknown-key.toml", "config.schema", 2),
        ("secret-wrong-type.toml", "config.schema", 1),
        ("secret-syntax.toml", "config.syntax", 2),
    ] {
        let path = fixture(name);
        for format in ["human", "json"] {
            let run = check(&path, format);
            assert_eq!(run.code, 1, "{name} {format}");
            assert!(
                !run.stdout.contains(CANARY),
                "{name} {format}: {}",
                run.stdout
            );
            assert!(
                !run.stderr.contains(CANARY),
                "{name} {format}: {}",
                run.stderr
            );
        }
        let message = if code == "config.syntax" {
            "the file is not valid TOML"
        } else {
            "unknown key, or a value of the wrong type or range"
        };
        assert_positioned_rejection(&path, code, line, message);
    }
}

#[test]
fn help_and_version() {
    for args in [
        vec!["--help"],
        vec!["-h"],
        vec!["--help", "anything"],
        vec!["check-config", "--help"],
        vec!["check-config", "-h", "--bogus"],
        vec!["check-config", "--help", "--bogus"],
        vec!["check-config", "--format", "json", "--help"],
    ] {
        assert_run(&pool(&args), 0, HELP, "");
    }
    for args in [vec!["--version"], vec!["-V"], vec!["--version", "--bogus"]] {
        assert_run(&pool(&args), 0, "pool 0.1.0\n", "");
    }
}

#[test]
fn usage_errors_exit_2_with_two_lines_on_stderr() {
    let cases: &[(&[&str], &str)] = &[
        (&[], "a command is required"),
        (&["run"], "unknown command"),
        (&["check-config", "--bogus"], "unknown option"),
        (&["check-config", "--bogus", "--help"], "unknown option"),
        (&["check-config", "--bogus=1"], "unknown option"),
        (&["check-config", "a.toml"], "unexpected argument"),
        (&["check-config", "--config"], "option requires a value"),
        (&["check-config", "--config="], "option requires a value"),
        (&["check-config", "--config", ""], "option requires a value"),
        (
            &["check-config", "--config", "a", "--format"],
            "option requires a value",
        ),
        (
            &["check-config", "--config", "a", "--format="],
            "option requires a value",
        ),
        (
            &["check-config", "--config", "a", "--config", "b"],
            "option given more than once",
        ),
        (
            &["check-config", "--config", "a", "--config"],
            "option given more than once",
        ),
        (
            &[
                "check-config",
                "--config",
                "a",
                "--format",
                "json",
                "--format",
                "json",
            ],
            "option given more than once",
        ),
        (
            &["check-config", "--config", "a", "--format", "xml"],
            "--format must be human or json",
        ),
        (
            &["check-config", "--config", "a", "--format", "JSON"],
            "--format must be human or json",
        ),
        (&["check-config"], "--config is required"),
        (
            &["check-config", "--format", "json"],
            "--config is required",
        ),
    ];
    for (args, message) in cases {
        let run = pool(*args);
        assert_run(
            &run,
            2,
            "",
            &format!("error: {message}\nRun 'pool --help' for usage.\n"),
        );
    }
}

#[test]
fn a_value_after_config_is_always_the_path() {
    let missing = "error: io.not_found: the configuration file does not exist\n";
    assert_run(
        &pool(["check-config", "--config", "--help"]),
        3,
        "",
        missing,
    );
    let inline = format!("--config={}", fixture("valid-minimal.toml"));
    assert_run(&pool(["check-config", &inline]), 0, VALID_HUMAN, "");
    assert_run(
        &pool(["check-config", "--format=json", &inline]),
        0,
        VALID_JSON,
        "",
    );
}

#[test]
fn unreadable_paths_exit_3() {
    let missing = fixture("does-not-exist.toml");
    assert_run(
        &check(&missing, "human"),
        3,
        "",
        "error: io.not_found: the configuration file does not exist\n",
    );
    assert_run(
        &check(&missing, "json"),
        3,
        &error_json("io.not_found", "the configuration file does not exist"),
        "",
    );

    assert_run(
        &check(FIXTURES, "human"),
        3,
        "",
        "error: io.read_failed: the configuration file cannot be read\n",
    );
    assert_run(
        &check(FIXTURES, "json"),
        3,
        &error_json("io.read_failed", "the configuration file cannot be read"),
        "",
    );
}

#[cfg(unix)]
#[test]
fn dev_zero_is_read_only_up_to_the_limit() {
    assert_none_rejection(
        "/dev/zero",
        "config.too_large",
        "the file is larger than 1048576 bytes",
    );
}

#[cfg(unix)]
#[test]
fn arguments_that_are_not_utf8() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let bad = || OsString::from_vec(vec![b'-', 0xFF]);
    let usage = |message: &str| format!("error: {message}\nRun 'pool --help' for usage.\n");

    let run = pool([bad()]);
    assert_run(&run, 2, "", &usage("unknown command"));

    let run = pool([OsString::from("check-config"), bad()]);
    assert_run(&run, 2, "", &usage("unexpected argument"));

    let run = pool([
        OsString::from("check-config"),
        OsString::from("--config"),
        OsString::from("a"),
        OsString::from("--format"),
        bad(),
    ]);
    assert_run(&run, 2, "", &usage("--format must be human or json"));

    let run = pool([
        OsString::from("check-config"),
        OsString::from("--config"),
        bad(),
    ]);
    assert_run(
        &run,
        3,
        "",
        "error: io.not_found: the configuration file does not exist\n",
    );
}

#[cfg(target_os = "linux")]
#[test]
fn a_failed_write_exits_3_without_stderr() {
    let full = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .expect("open /dev/full");
    for format in ["human", "json"] {
        let output = Command::new(env!("CARGO_BIN_EXE_pool"))
            .args([
                "check-config",
                "--config",
                &fixture("valid-minimal.toml"),
                "--format",
                format,
            ])
            .stdin(Stdio::null())
            .stdout(full.try_clone().expect("clone /dev/full"))
            .output()
            .expect("run pool");
        assert_eq!(output.status.code(), Some(3), "{format}");
        assert!(output.stderr.is_empty(), "{format}");
    }
}

// Edge and error cases added by the tester (spec 0001 AC-3 to AC-12,
// SEC-10, SEC-11). Made inputs live under the target temp dir, each with a
// file name of its own; no fixture is added.

/// The exact stderr of a usage error.
fn usage(message: &str) -> String {
    format!("error: {message}\nRun 'pool --help' for usage.\n")
}

/// Checks every argument list exits 2 with `message` and empty stdout.
fn assert_usage(cases: &[&[&str]], message: &str) {
    for args in cases {
        let run = pool(*args);
        assert_run(&run, 2, "", &usage(message));
    }
}

/// A valid fixture path, for argument lists that must reach the file.
fn valid() -> String {
    fixture("valid-minimal.toml")
}

#[test]
fn ac11_no_argument_at_all_needs_a_command() {
    assert_usage(&[&[]], "a command is required");
}

#[test]
fn ac11_a_first_argument_that_is_not_a_command_is_unknown() {
    assert_usage(
        &[
            &["run"],
            &[""],
            &["check"],
            &["Check-Config"],
            &["check-config="],
            &["-x"],
            &["--"],
            &["-"],
            &["--HELP"],
            &["--Version"],
            &["-v"],
            &["--config", "a.toml"],
            &["--format", "json"],
            &["help"],
            &["version"],
        ],
        "unknown command",
    );
}

#[test]
fn ac11_an_unknown_dash_argument_where_an_option_is_expected_is_an_unknown_option() {
    assert_usage(
        &[
            &["check-config", "--bogus"],
            &["check-config", "-x"],
            &["check-config", "-"],
            &["check-config", "--"],
            &["check-config", "--config-file", "a"],
            &["check-config", "--formats", "json"],
            &["check-config", "--Config", "a"],
            &["check-config", "--HELP"],
            &["check-config", "-H"],
            &["check-config", "-c", "a"],
            &["check-config", "--version"],
            &["check-config", "-V"],
            &["check-config", "--bogus=--help"],
            &["check-config", "--=a"],
        ],
        "unknown option",
    );
}

#[test]
fn ac11_any_other_argument_where_an_option_is_expected_is_unexpected() {
    let path = valid();
    assert_usage(
        &[
            &["check-config", "a.toml"],
            &["check-config", ""],
            &["check-config", "check-config"],
            &["check-config", "=a"],
            &["check-config", "config=a"],
            &["check-config", "--config", &path, "extra"],
            &[
                "check-config",
                "--config",
                &path,
                "--format",
                "json",
                "json",
            ],
        ],
        "unexpected argument",
    );
}

#[test]
fn ac11_an_option_without_a_value_requires_one() {
    assert_usage(
        &[
            &["check-config", "--config"],
            &["check-config", "--config="],
            &["check-config", "--config", ""],
            &["check-config", "--format"],
            &["check-config", "--format="],
            &["check-config", "--format", ""],
            &["check-config", "--config", "a", "--format"],
            &["check-config", "--config", "a", "--format="],
            &["check-config", "--config", "a", "--format", ""],
            &["check-config", "--format=", "--config", "a"],
        ],
        "option requires a value",
    );
}

#[test]
fn ac11_an_option_given_a_second_time_is_refused_before_its_value_is_read() {
    assert_usage(
        &[
            &["check-config", "--config", "a", "--config", "b"],
            &["check-config", "--config", "a", "--config"],
            &["check-config", "--config=a", "--config=b"],
            &["check-config", "--config", "a", "--config="],
            &["check-config", "--config", "a", "--config", ""],
            &["check-config", "--config", "a", "--config", "a"],
            &["check-config", "--format", "json", "--format", "json"],
            &["check-config", "--format=json", "--format", "human"],
            &["check-config", "--format", "json", "--format"],
            &["check-config", "--format", "json", "--format="],
            &["check-config", "--format", "json", "--format", "xml"],
            &["check-config", "--format", "human", "--format=JSON"],
        ],
        "option given more than once",
    );
}

#[test]
fn ac11_a_format_value_that_is_not_exactly_human_or_json_is_refused() {
    assert_usage(
        &[
            &["check-config", "--config", "a", "--format", "xml"],
            &["check-config", "--config", "a", "--format", "JSON"],
            &["check-config", "--config", "a", "--format=JSON"],
            &["check-config", "--config", "a", "--format", "Human"],
            &["check-config", "--config", "a", "--format", " json"],
            &["check-config", "--config", "a", "--format", "json "],
            &["check-config", "--config", "a", "--format", "jsonl"],
            &["check-config", "--config", "a", "--format=json=x"],
            &["check-config", "--config", "a", "--format", "--help"],
            &["check-config", "--config", "a", "--format", "-h"],
            &["check-config", "--format", "text"],
        ],
        "--format must be human or json",
    );
}

#[test]
fn ac11_config_is_required_once_every_argument_was_read() {
    assert_usage(
        &[
            &["check-config"],
            &["check-config", "--format", "json"],
            &["check-config", "--format=human"],
        ],
        "--config is required",
    );
}

#[test]
fn ac11_an_error_met_before_help_still_wins() {
    let cases: &[(&[&str], &str)] = &[
        (&["check-config", "--bogus", "--help"], "unknown option"),
        (&["check-config", "a.toml", "--help"], "unexpected argument"),
        (
            &["check-config", "--format", "xml", "--help"],
            "--format must be human or json",
        ),
        (
            &["check-config", "--config", "a", "--config", "b", "-h"],
            "option given more than once",
        ),
        (
            &["check-config", "--config=", "--help"],
            "option requires a value",
        ),
    ];
    for (args, message) in cases {
        assert_run(&pool(*args), 2, "", &usage(message));
    }
}

#[test]
fn ac11_usage_errors_look_the_same_whatever_the_format_says() {
    for format in ["--format=json", "--format=human"] {
        assert_run(
            &pool(["check-config", format, "--bogus"]),
            2,
            "",
            &usage("unknown option"),
        );
        assert_run(
            &pool(["check-config", format, "stray"]),
            2,
            "",
            &usage("unexpected argument"),
        );
        assert_run(
            &pool(["check-config", format]),
            2,
            "",
            &usage("--config is required"),
        );
    }
}

#[test]
fn ac11_a_usage_error_never_repeats_what_was_typed() {
    let typed = "CANARY-9f2c7b1e-typed";
    for args in [
        vec![typed],
        vec!["check-config", typed],
        vec!["check-config", "--CANARY-9f2c7b1e-typed"],
        vec!["check-config", "--config", "a", "--format", typed],
    ] {
        let run = pool(&args);
        assert_eq!(run.code, 2, "{args:?}");
        assert_eq!(run.stdout, "", "{args:?}");
        assert!(!run.stderr.contains(CANARY), "{args:?}: {}", run.stderr);
        assert_eq!(run.stderr.lines().count(), 2, "{args:?}");
    }
}

#[test]
fn ac11_help_wherever_an_option_is_expected_prints_the_help() {
    let path = valid();
    for args in [
        vec!["check-config", "-h"],
        vec!["check-config", "--config", &path, "--help"],
        vec!["check-config", "--config", &path, "--format", "json", "-h"],
        vec!["check-config", "--help", "--config"],
        vec!["check-config", "--help", "--format", "xml"],
        vec!["check-config", "-h", "a.toml"],
        vec!["check-config", "--help=anything"],
        vec!["check-config", "-h="],
    ] {
        assert_run(&pool(&args), 0, HELP, "");
    }
}

#[test]
fn ac11_help_and_version_as_first_argument_ignore_everything_after() {
    for args in [
        vec!["-h", "--bogus"],
        vec!["--help", "--version"],
        vec!["-h", "check-config", "--config"],
    ] {
        assert_run(&pool(&args), 0, HELP, "");
    }
    for args in [
        vec!["-V", "anything"],
        vec!["--version", "--help"],
        vec!["-V", "check-config", "--format", "xml"],
    ] {
        assert_run(
            &pool(&args),
            0,
            &format!("pool {}\n", env!("CARGO_PKG_VERSION")),
            "",
        );
    }
}

#[test]
fn ac11_version_and_a_dash_after_check_config_is_an_option_error() {
    // Help is honored after check-config; version is not.
    assert_run(
        &pool(["check-config", "--config", "a", "--version"]),
        2,
        "",
        &usage("unknown option"),
    );
}

#[test]
fn ac3_options_may_come_in_any_order_and_either_spelling() {
    let path = valid();
    let inline = format!("--config={path}");
    for args in [
        vec!["check-config", "--format", "json", "--config", &path],
        vec!["check-config", "--format=json", "--config", &path],
        vec!["check-config", &inline, "--format", "json"],
    ] {
        assert_run(&pool(&args), 0, VALID_JSON, "");
    }
    for args in [
        vec!["check-config", "--format", "human", "--config", &path],
        vec!["check-config", &inline, "--format=human"],
        vec!["check-config", &inline],
    ] {
        assert_run(&pool(&args), 0, VALID_HUMAN, "");
    }
}

#[test]
fn ac3_a_path_after_the_first_equals_sign_keeps_its_own_equals_signs() {
    let path = made_file("check-config-a=b=c.toml", b"config_version = 1\n");
    let inline = format!("--config={}", path.to_str().expect("path"));
    assert_run(&pool(["check-config", &inline]), 0, VALID_HUMAN, "");
    assert_run(
        &pool(["check-config", &inline, "--format=json"]),
        0,
        VALID_JSON,
        "",
    );
}

#[test]
fn ac3_a_path_that_starts_with_a_dash_is_taken_as_given() {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("check-config-dash-dir");
    std::fs::create_dir_all(&dir).expect("make dir");
    std::fs::write(dir.join("-valid.toml"), b"config_version = 1\n").expect("write");
    let output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(["check-config", "--config", "-valid.toml"])
        .current_dir(&dir)
        .stdin(Stdio::null())
        .output()
        .expect("run pool");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_HUMAN);
    assert!(output.stderr.is_empty());
}

#[test]
fn ac3_a_relative_path_is_read_from_the_working_directory() {
    let output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(["check-config", "--config", "valid-capabilities-empty.toml"])
        .args(["--format", "json"])
        .current_dir(FIXTURES)
        .stdin(Stdio::null())
        .output()
        .expect("run pool");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn ac3_valid_output_does_not_depend_on_the_working_directory_or_environment() {
    let path = valid();
    let output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(["check-config", "--config", &path])
        .current_dir(env!("CARGO_TARGET_TMPDIR"))
        .env_clear()
        .env("NO_COLOR", "")
        .env("TERM", "xterm-256color")
        .env("LANG", "de_DE.UTF-8")
        .stdin(Stdio::null())
        .output()
        .expect("run pool");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_HUMAN);
    assert!(output.stderr.is_empty());
}

#[test]
fn ac4_sec10_result_cache_left_out_reports_disabled() {
    let path = made_file(
        "check-config-ac4-cache-left-out.toml",
        b"config_version = 1\n[automatic_promotion]\nenabled = false\n",
    );
    let path = path.to_str().expect("path");
    assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
    assert_run(&check(path, "json"), 0, VALID_JSON, "");
}

#[test]
fn ac4_sec10_automatic_promotion_left_out_reports_disabled() {
    let path = made_file(
        "check-config-ac4-promotion-left-out.toml",
        b"config_version = 1\n[result_cache]\nenabled = false\n",
    );
    let path = path.to_str().expect("path");
    assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
    assert_run(&check(path, "json"), 0, VALID_JSON, "");
}

#[test]
fn ac4_sec10_result_cache_as_an_empty_table_reports_disabled() {
    for (name, text) in [
        (
            "check-config-ac4-cache-empty-header.toml",
            "config_version = 1\n[result_cache]\n",
        ),
        (
            "check-config-ac4-cache-empty-inline.toml",
            "config_version = 1\nresult_cache = {}\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        let path = path.to_str().expect("path");
        assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(path, "json"), 0, VALID_JSON, "");
    }
}

#[test]
fn ac4_sec10_automatic_promotion_as_an_empty_table_reports_disabled() {
    for (name, text) in [
        (
            "check-config-ac4-promotion-empty-header.toml",
            "config_version = 1\n[automatic_promotion]\n",
        ),
        (
            "check-config-ac4-promotion-empty-inline.toml",
            "config_version = 1\nautomatic_promotion = {}\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        let path = path.to_str().expect("path");
        assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(path, "json"), 0, VALID_JSON, "");
    }
}

#[test]
fn ac4_sec10_result_cache_enabled_false_reports_disabled_in_every_spelling() {
    for (name, text) in [
        (
            "check-config-ac4-cache-false-dotted.toml",
            "config_version = 1\nresult_cache.enabled = false\n",
        ),
        (
            "check-config-ac4-cache-false-inline.toml",
            "config_version = 1\nresult_cache = { enabled = false }\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        let path = path.to_str().expect("path");
        assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(path, "json"), 0, VALID_JSON, "");
    }
}

#[test]
fn ac4_sec10_automatic_promotion_enabled_false_reports_disabled_in_every_spelling() {
    for (name, text) in [
        (
            "check-config-ac4-promotion-false-dotted.toml",
            "config_version = 1\nautomatic_promotion.enabled = false\n",
        ),
        (
            "check-config-ac4-promotion-false-inline.toml",
            "config_version = 1\nautomatic_promotion = { enabled = false }\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        let path = path.to_str().expect("path");
        assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(path, "json"), 0, VALID_JSON, "");
    }
}

#[test]
fn ac5_sec10_result_cache_enabled_true_is_refused_in_the_dotted_and_inline_spellings() {
    for (name, text) in [
        (
            "check-config-ac5-dotted.toml",
            "config_version = 1\nresult_cache.enabled = true\n",
        ),
        (
            "check-config-ac5-inline.toml",
            "config_version = 1\nresult_cache = { enabled = true }\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        assert_key_rejection(
            path.to_str().expect("path"),
            "config.unsupported_capability",
            "result_cache.enabled",
            CAPABILITY_MESSAGE,
        );
    }
}

#[test]
fn ac6_sec10_automatic_promotion_enabled_true_is_refused_in_the_dotted_and_inline_spellings() {
    for (name, text) in [
        (
            "check-config-ac6-dotted.toml",
            "config_version = 1\nautomatic_promotion.enabled = true\n",
        ),
        (
            "check-config-ac6-inline.toml",
            "config_version = 1\nautomatic_promotion = { enabled = true }\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        assert_key_rejection(
            path.to_str().expect("path"),
            "config.unsupported_capability",
            "automatic_promotion.enabled",
            CAPABILITY_MESSAGE,
        );
    }
}

#[test]
fn ac7_crlf_line_endings_give_the_same_lines_through_the_binary() {
    for (name, code, line, message) in [
        (
            "invalid-unknown-key.toml",
            "config.schema",
            3,
            "unknown key, or a value of the wrong type or range",
        ),
        (
            "invalid-wrong-type.toml",
            "config.schema",
            4,
            "unknown key, or a value of the wrong type or range",
        ),
        (
            "invalid-syntax.toml",
            "config.syntax",
            2,
            "the file is not valid TOML",
        ),
    ] {
        let text = std::fs::read_to_string(fixture(name)).expect("fixture");
        let path = made_file(
            &format!("check-config-crlf-{name}"),
            text.replace('\n', "\r\n").as_bytes(),
        );
        assert_positioned_rejection(path.to_str().expect("path"), code, line, message);
    }
    let text =
        std::fs::read_to_string(fixture("valid-capabilities-disabled.toml")).expect("fixture");
    let path = made_file(
        "check-config-crlf-valid.toml",
        text.replace('\n', "\r\n").as_bytes(),
    );
    assert_run(
        &check(path.to_str().expect("path"), "json"),
        0,
        VALID_JSON,
        "",
    );
}

#[test]
fn ac7_bad_values_get_the_listed_code_through_the_binary() {
    let schema = "unknown key, or a value of the wrong type or range";
    for (name, text, code, line, message) in [
        (
            "check-config-bad-negative.toml",
            "config_version = -1\n",
            "config.schema",
            1,
            schema,
        ),
        (
            "check-config-bad-u32-overflow.toml",
            "config_version = 4294967296\n",
            "config.schema",
            1,
            schema,
        ),
        (
            "check-config-bad-float.toml",
            "config_version = 1.0\n",
            "config.schema",
            1,
            schema,
        ),
        (
            "check-config-bad-string.toml",
            "config_version = \"1\"\n",
            "config.schema",
            1,
            schema,
        ),
        (
            "check-config-bad-i64-overflow.toml",
            "config_version = 9223372036854775808\n",
            "config.syntax",
            1,
            "the file is not valid TOML",
        ),
        (
            "check-config-bad-enabled-type.toml",
            "config_version = 1\nautomatic_promotion.enabled = \"no\"\n",
            "config.schema",
            2,
            schema,
        ),
        (
            "check-config-bad-not-a-table.toml",
            "config_version = 1\n\nautomatic_promotion = true\n",
            "config.schema",
            3,
            schema,
        ),
        (
            "check-config-bad-twice.toml",
            "config_version = 1\nconfig_version = 1\n",
            "config.syntax",
            2,
            "the file is not valid TOML",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        assert_positioned_rejection(path.to_str().expect("path"), code, line, message);
    }
    for (name, text) in [
        ("check-config-bad-zero.toml", "config_version = 0\n"),
        (
            "check-config-bad-u32-max.toml",
            "config_version = 4294967295\n",
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        assert_key_rejection(
            path.to_str().expect("path"),
            "config.unsupported_version",
            "config_version",
            "this build supports config_version 1 only",
        );
    }
    let path = made_file("check-config-bad-comments.toml", b"# only\n# comments\n");
    assert_key_rejection(
        path.to_str().expect("path"),
        "config.missing_key",
        "config_version",
        "a required key is missing",
    );
}

#[test]
fn ac7_size_is_checked_before_utf8_through_the_binary() {
    let too_large = made_file(
        "check-config-too-large-not-utf8.toml",
        &vec![0xFF; 1_048_577],
    );
    assert_none_rejection(
        too_large.to_str().expect("path"),
        "config.too_large",
        "the file is larger than 1048576 bytes",
    );
    let at_limit = made_file(
        "check-config-at-limit-not-utf8.toml",
        &vec![0xFF; 1_048_576],
    );
    assert_none_rejection(
        at_limit.to_str().expect("path"),
        "config.not_utf8",
        "the file is not valid UTF-8",
    );
}

#[test]
fn ac7_a_file_far_beyond_the_limit_is_still_too_large() {
    let huge = made_file("check-config-huge.toml", &vec![b'#'; 3 * 1_048_576]);
    assert_none_rejection(
        huge.to_str().expect("path"),
        "config.too_large",
        "the file is larger than 1048576 bytes",
    );
}

#[test]
fn ac7_a_file_at_the_limit_with_a_capability_on_is_still_refused_by_key() {
    let tail = b"\nconfig_version = 1\nautomatic_promotion.enabled = true\n";
    let mut bytes = vec![b'#'; 1_048_576 - tail.len()];
    bytes.extend_from_slice(tail);
    assert_eq!(bytes.len(), 1_048_576);
    let path = made_file("check-config-at-limit-capability.toml", &bytes);
    assert_key_rejection(
        path.to_str().expect("path"),
        "config.unsupported_capability",
        "automatic_promotion.enabled",
        CAPABILITY_MESSAGE,
    );
}

#[test]
fn ac7_an_empty_made_file_misses_config_version() {
    let path = made_file("check-config-made-empty.toml", b"");
    assert_key_rejection(
        path.to_str().expect("path"),
        "config.missing_key",
        "config_version",
        "a required key is missing",
    );
}

#[test]
fn ac8_findings_follow_the_schema_order_even_when_the_file_is_reordered() {
    let path = made_file(
        "check-config-ac8-reordered.toml",
        b"automatic_promotion.enabled = true\nresult_cache = { enabled = true }\n",
    );
    let path = path.to_str().expect("path");
    let findings = [
        key_finding_json(
            "config.missing_key",
            "config_version",
            "a required key is missing",
        ),
        key_finding_json(
            "config.unsupported_capability",
            "result_cache.enabled",
            CAPABILITY_MESSAGE,
        ),
        key_finding_json(
            "config.unsupported_capability",
            "automatic_promotion.enabled",
            CAPABILITY_MESSAGE,
        ),
    ]
    .join(",");
    assert_run(&check(path, "json"), 1, &invalid_json(&findings), "");
    assert_run(
        &check(path, "human"),
        1,
        &format!(
            "configuration invalid\nfindings: 3\n\
             config.missing_key at config_version: a required key is missing\n\
             config.unsupported_capability at result_cache.enabled: {CAPABILITY_MESSAGE}\n\
             config.unsupported_capability at automatic_promotion.enabled: {CAPABILITY_MESSAGE}\n"
        ),
        "",
    );
}

#[test]
fn ac8_a_schema_finding_hides_the_meaning_findings() {
    let path = made_file(
        "check-config-ac8-schema-first.toml",
        b"config_version = 2\nresult_cache.enabled = true\nport = 6432\n",
    );
    assert_positioned_rejection(
        path.to_str().expect("path"),
        "config.schema",
        3,
        "unknown key, or a value of the wrong type or range",
    );
}

#[test]
fn ac9_sec11_the_canary_never_reaches_the_output_in_any_spelling_of_the_option() {
    for name in [
        "secret-unknown-key.toml",
        "secret-wrong-type.toml",
        "secret-syntax.toml",
    ] {
        let path = fixture(name);
        let inline = format!("--config={path}");
        for args in [
            vec!["check-config", &inline],
            vec!["check-config", "--format=json", &inline],
            vec!["check-config", "--format", "json", "--config", &path],
        ] {
            let run = pool(&args);
            assert_eq!(run.code, 1, "{name} {args:?}");
            for text in [&run.stdout, &run.stderr] {
                for piece in [
                    "9f2c7b1e",
                    "not-a-real-secret",
                    "backend_password",
                    "CANARY",
                ] {
                    assert!(!text.contains(piece), "{name}: {piece} in {text}");
                }
            }
            assert_eq!(run.stderr, "", "{name} {args:?}");
        }
    }
}

#[test]
fn ac9_sec11_a_canary_in_a_value_key_or_comment_never_reaches_the_output() {
    let canary = "CANARY-9f2c7b1e-not-a-real-secret";
    for (name, text) in [
        (
            "check-config-canary-enabled-value.toml",
            format!("config_version = 1\nresult_cache.enabled = \"{canary}\"\n"),
        ),
        (
            "check-config-canary-table-name.toml",
            format!("config_version = 1\n[\"{canary}\"]\n"),
        ),
        (
            "check-config-canary-comment.toml",
            format!("config_version = 2\n# {canary}\nautomatic_promotion.enabled = true\n"),
        ),
        (
            "check-config-canary-bare-value.toml",
            format!("config_version = {canary}\n"),
        ),
    ] {
        let path = made_file(name, text.as_bytes());
        for format in ["human", "json"] {
            let run = check(path.to_str().expect("path"), format);
            assert_eq!(run.code, 1, "{name} {format}");
            assert!(
                !run.stdout.contains(CANARY),
                "{name} {format}: {}",
                run.stdout
            );
            assert!(
                !run.stderr.contains(CANARY),
                "{name} {format}: {}",
                run.stderr
            );
        }
    }
}

#[test]
fn ac9_sec11_a_canary_in_the_path_is_not_echoed_on_exit_3() {
    let missing = fixture("CANARY-9f2c7b1e-missing.toml");
    for format in ["human", "json"] {
        let run = check(&missing, format);
        assert_eq!(run.code, 3, "{format}");
        assert!(!run.stdout.contains(CANARY), "{format}: {}", run.stdout);
        assert!(!run.stderr.contains(CANARY), "{format}: {}", run.stderr);
    }
}

/// The names of the top level fields of a one line JSON report, in order.
fn top_level_keys(line: &str) -> Vec<String> {
    let value: serde_json::Value = serde_json::from_str(line).expect("one JSON object");
    let object = value.as_object().expect("an object");
    let mut keys: Vec<(usize, String)> = object
        .keys()
        .map(|key| {
            let needle = format!("\"{key}\":");
            (line.find(&needle).expect("key in line"), key.clone())
        })
        .collect();
    keys.sort();
    keys.into_iter().map(|(_, key)| key).collect()
}

#[test]
fn ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order() {
    let mut paths: Vec<String> = [
        "valid-minimal.toml",
        "valid-capabilities-disabled.toml",
        "valid-capabilities-empty.toml",
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
        "does-not-exist.toml",
    ]
    .iter()
    .map(|name| fixture(name))
    .collect();
    paths.push(FIXTURES.to_owned());
    let expected_keys = [
        "output_version",
        "command",
        "result",
        "config_version",
        "capabilities",
        "findings",
    ];
    let finding_keys = ["code", "key", "line", "column", "message"];
    for path in paths {
        let run = check(&path, "json");
        assert_eq!(run.stderr, "", "{path}");
        assert!(run.stdout.ends_with('\n'), "{path}");
        assert_eq!(run.stdout.matches('\n').count(), 1, "{path}");
        let line = run.stdout.trim_end_matches('\n');
        assert_eq!(top_level_keys(line), expected_keys, "{path}");

        let value: serde_json::Value = serde_json::from_str(line).expect("JSON");
        assert_eq!(value["output_version"], 1, "{path}");
        assert_eq!(value["command"], "check-config", "{path}");
        let result = value["result"].as_str().expect("result string");
        let expected_result = match run.code {
            0 => "valid",
            1 => "invalid",
            3 => "error",
            other => panic!("{path}: exit {other}"),
        };
        assert_eq!(result, expected_result, "{path}");
        let findings = value["findings"].as_array().expect("findings array");
        match run.code {
            0 => {
                assert!(findings.is_empty(), "{path}");
                assert_eq!(value["config_version"], 1, "{path}");
                assert!(value["capabilities"].is_object(), "{path}");
            }
            1 => {
                assert!(!findings.is_empty(), "{path}");
                assert!(value["config_version"].is_null(), "{path}");
                assert!(value["capabilities"].is_null(), "{path}");
            }
            _ => {
                assert_eq!(findings.len(), 1, "{path}");
                assert!(value["config_version"].is_null(), "{path}");
                assert!(value["capabilities"].is_null(), "{path}");
                let code = findings[0]["code"].as_str().expect("code");
                assert!(code.starts_with("io."), "{path}: {code}");
            }
        }
        // serde_json sorts keys when it prints a Value, so each finding's
        // own field order is checked in the raw line. The search resumes
        // after the previous finding, so two findings with the same code are
        // each checked.
        let mut cursor = line.find("\"findings\":[").expect("findings field");
        for finding in findings {
            let object = finding.as_object().expect("finding object");
            assert_eq!(object.len(), finding_keys.len(), "{path}");
            let raw = finding.to_string();
            let code = finding["code"].as_str().expect("code");
            let start = cursor
                + line[cursor..]
                    .find(&format!("{{\"code\":\"{code}\""))
                    .expect("finding starts with code");
            let end = line[start..].find('}').map_or(line.len(), |at| start + at);
            let rest = &line[start..end];
            let mut last = 0;
            for key in finding_keys {
                let at = rest.find(&format!("\"{key}\":")).expect("field");
                assert!(at >= last, "{path}: {key} out of order in {raw}");
                last = at;
            }
            cursor = end;
        }
    }
}

#[test]
fn ac10_valid_capabilities_object_lists_result_cache_first() {
    let run = check(&valid(), "json");
    let at_cache = run.stdout.find("\"result_cache\"").expect("result_cache");
    let at_promotion = run
        .stdout
        .find("\"automatic_promotion\"")
        .expect("automatic_promotion");
    assert!(at_cache < at_promotion, "{}", run.stdout);
}

#[test]
fn ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked() {
    let run = pool(["check-config", "--format", "json", "--help"]);
    assert_run(&run, 0, HELP, "");
    assert!(serde_json::from_str::<serde_json::Value>(&run.stdout).is_err());
    let run = pool(["check-config", "--format", "json"]);
    assert_run(&run, 2, "", &usage("--config is required"));
}

#[test]
fn ac11_a_path_through_a_regular_file_cannot_be_read() {
    let through_file = format!("{}/x.toml", valid());
    assert_run(
        &check(&through_file, "human"),
        3,
        "",
        "error: io.read_failed: the configuration file cannot be read\n",
    );
    assert_run(
        &check(&through_file, "json"),
        3,
        &error_json("io.read_failed", "the configuration file cannot be read"),
        "",
    );
}

#[test]
fn ac11_a_path_under_a_missing_directory_is_not_found() {
    let missing = fixture("no-such-dir/also-missing.toml");
    assert_run(
        &check(&missing, "human"),
        3,
        "",
        "error: io.not_found: the configuration file does not exist\n",
    );
    assert_run(
        &check(&missing, "json"),
        3,
        &error_json("io.not_found", "the configuration file does not exist"),
        "",
    );
}

#[test]
fn ac11_a_directory_given_without_a_trailing_slash_cannot_be_read() {
    let dir = FIXTURES.trim_end_matches('/');
    assert_run(
        &check(dir, "human"),
        3,
        "",
        "error: io.read_failed: the configuration file cannot be read\n",
    );
    assert_run(
        &check(dir, "json"),
        3,
        &error_json("io.read_failed", "the configuration file cannot be read"),
        "",
    );
}

#[cfg(unix)]
#[test]
fn ac11_a_symlink_is_followed_and_a_dangling_one_is_not_found() {
    let tmp = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let link = tmp.join("check-config-link-valid.toml");
    let dangling = tmp.join("check-config-link-dangling.toml");
    for path in [&link, &dangling] {
        let _ = std::fs::remove_file(path);
    }
    std::os::unix::fs::symlink(valid(), &link).expect("symlink");
    std::os::unix::fs::symlink(tmp.join("check-config-no-target.toml"), &dangling)
        .expect("symlink");
    assert_run(
        &check(link.to_str().expect("path"), "json"),
        0,
        VALID_JSON,
        "",
    );
    assert_run(
        &check(dangling.to_str().expect("path"), "human"),
        3,
        "",
        "error: io.not_found: the configuration file does not exist\n",
    );
}

#[cfg(unix)]
#[test]
fn ac12_dev_zero_is_too_large_in_the_json_format_too() {
    assert_run(
        &check("/dev/zero", "json"),
        1,
        &invalid_json(&none_finding_json(
            "config.too_large",
            "the file is larger than 1048576 bytes",
        )),
        "",
    );
}

#[cfg(unix)]
#[test]
fn ac12_dev_null_is_an_empty_file_that_misses_config_version() {
    assert_key_rejection(
        "/dev/null",
        "config.missing_key",
        "config_version",
        "a required key is missing",
    );
}

#[cfg(unix)]
#[test]
fn ac11_arguments_that_are_not_utf8_anywhere_else_are_unexpected() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let bad = |prefix: &[u8]| {
        let mut bytes = prefix.to_vec();
        bytes.push(0xFF);
        OsString::from_vec(bytes)
    };
    let arg = OsString::from;

    assert_run(&pool([bad(b"")]), 2, "", &usage("unknown command"));
    assert_run(
        &pool([bad(b"check-config")]),
        2,
        "",
        &usage("unknown command"),
    );
    for args in [
        vec![arg("check-config"), bad(b"")],
        vec![arg("check-config"), bad(b"--bogus")],
        vec![arg("check-config"), bad(b"--config=")],
        vec![arg("check-config"), bad(b"--format=")],
        vec![
            arg("check-config"),
            arg("--config"),
            arg("a"),
            bad(b"--format="),
        ],
        vec![arg("check-config"), arg("--config"), arg("a"), bad(b"")],
    ] {
        assert_run(&pool(&args), 2, "", &usage("unexpected argument"));
    }
    assert_run(
        &pool([arg("check-config"), bad(b"--help")]),
        2,
        "",
        &usage("unexpected argument"),
    );
}

#[cfg(unix)]
#[test]
fn ac11_an_argument_that_is_not_utf8_is_ignored_after_help_or_version() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let bad = OsString::from_vec(vec![0xFF]);
    assert_run(&pool([OsString::from("--help"), bad.clone()]), 0, HELP, "");
    assert_run(
        &pool([OsString::from("-V"), bad.clone()]),
        0,
        "pool 0.1.0\n",
        "",
    );
    assert_run(
        &pool([OsString::from("check-config"), OsString::from("-h"), bad]),
        0,
        HELP,
        "",
    );
}

#[cfg(unix)]
#[test]
fn ac11_a_path_that_is_not_utf8_is_used_as_given() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("check-config-non-utf8-dir");
    std::fs::create_dir_all(&dir).expect("make dir");
    let name = OsString::from_vec(vec![b'c', 0xFF, b'.', b't', b'o', b'm', b'l']);
    let path = dir.join(&name);
    match std::fs::write(&path, b"config_version = 1\n") {
        Ok(()) => {
            let output = Command::new(env!("CARGO_BIN_EXE_pool"))
                .arg("check-config")
                .arg("--config")
                .arg(&path)
                .stdin(Stdio::null())
                .output()
                .expect("run pool");
            assert_eq!(output.status.code(), Some(0));
            assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_HUMAN);
            assert!(output.stderr.is_empty());
        }
        // Some file systems (APFS) refuse names that are not UTF-8; then
        // the path is still used as given and reported as missing.
        Err(_) => {
            let output = Command::new(env!("CARGO_BIN_EXE_pool"))
                .arg("check-config")
                .arg("--config")
                .arg(&path)
                .stdin(Stdio::null())
                .output()
                .expect("run pool");
            assert_eq!(output.status.code(), Some(3));
            assert!(output.stdout.is_empty());
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                stderr == "error: io.not_found: the configuration file does not exist\n"
                    || stderr == "error: io.read_failed: the configuration file cannot be read\n",
                "{stderr}"
            );
        }
    }
}

/// Runs `pool` with stdout, or stderr, sent into a pipe whose reader is
/// already closed, so every write fails.
#[cfg(unix)]
fn pool_into_closed_pipe(args: &[&str], close_stdout: bool) -> Output {
    let (reader, writer) = std::io::pipe().expect("pipe");
    drop(reader);
    let mut command = Command::new(env!("CARGO_BIN_EXE_pool"));
    command.args(args).stdin(Stdio::null());
    if close_stdout {
        command.stdout(writer).stderr(Stdio::piped());
    } else {
        command.stdout(Stdio::piped()).stderr(writer);
    }
    command.output().expect("run pool")
}

#[cfg(unix)]
#[test]
fn ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic() {
    let path = valid();
    let invalid = fixture("invalid-multiple.toml");
    for args in [
        vec!["check-config", "--config", &path],
        vec!["check-config", "--config", &path, "--format", "json"],
        vec!["check-config", "--config", &invalid],
        vec!["check-config", "--config", &invalid, "--format", "json"],
        vec!["--help"],
        vec!["--version"],
    ] {
        let output = pool_into_closed_pipe(&args, true);
        assert_eq!(output.status.code(), Some(3), "{args:?}");
        assert!(
            output.stderr.is_empty(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[cfg(unix)]
#[test]
fn ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic() {
    // Each of these would exit 2 if the write to stderr succeeded.
    for args in [
        vec!["check-config", "--bogus"],
        vec!["check-config"],
        vec![],
    ] {
        let output = pool_into_closed_pipe(&args, false);
        assert_eq!(output.status.code(), Some(3), "{args:?}");
        assert!(output.stdout.is_empty(), "{args:?}");
    }
}

#[cfg(target_os = "linux")]
#[test]
fn ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr() {
    let invalid = fixture("invalid-multiple.toml");
    let full = std::fs::OpenOptions::new()
        .write(true)
        .open("/dev/full")
        .expect("open /dev/full");
    for args in [
        vec!["--help"],
        vec!["--version"],
        vec!["check-config", "--config", &invalid],
        vec!["check-config", "--config", &invalid, "--format", "json"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(&args)
            .stdin(Stdio::null())
            .stdout(full.try_clone().expect("clone /dev/full"))
            .output()
            .expect("run pool");
        assert_eq!(output.status.code(), Some(3), "{args:?}");
        assert!(output.stderr.is_empty(), "{args:?}");
    }
}

#[test]
fn ac7_a_reserved_capability_written_as_an_array_is_rejected_by_the_binary() {
    for (name, text) in [
        (
            "array-cache-false",
            "config_version = 1\nresult_cache = [false]\n",
        ),
        (
            "array-cache-true",
            "config_version = 1\nresult_cache = [true]\n",
        ),
        (
            "array-promotion-canary",
            "config_version = 1\nautomatic_promotion = [false, \"CANARY-9f2c7b1e\", 3]\n",
        ),
    ] {
        let path = made_file(&format!("check-config-{name}.toml"), text.as_bytes());
        let path = path.to_str().expect("UTF-8 temp path");
        for format in ["human", "json"] {
            let run = check(path, format);
            assert_eq!(run.code, 1, "{name} {format}");
            assert_eq!(run.stderr, "", "{name} {format}");
            assert_eq!(run.stdout.matches("config.").count(), 1, "{name} {format}");
            assert!(run.stdout.contains("config.schema"), "{name} {format}");
            assert!(!run.stdout.contains(CANARY), "{name} {format}");
        }
        assert_positioned_rejection(
            path,
            "config.schema",
            2,
            "unknown key, or a value of the wrong type or range",
        );
    }
}

#[cfg(unix)]
#[test]
fn ac11_a_file_that_cannot_be_opened_is_permission_denied() {
    use std::os::unix::fs::PermissionsExt;

    let path = made_file("check-config-no-permission.toml", b"config_version = 1\n");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).expect("chmod 000");
    // A process that can still open the file (root) cannot test this path.
    if std::fs::File::open(&path).is_ok() {
        let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        return;
    }
    let text = path.to_str().expect("UTF-8 temp path");
    let human = check(text, "human");
    let json = check(text, "json");
    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
    assert_run(
        &human,
        3,
        "",
        "error: io.permission_denied: the configuration file cannot be read: permission denied\n",
    );
    assert_run(
        &json,
        3,
        &error_json(
            "io.permission_denied",
            "the configuration file cannot be read: permission denied",
        ),
        "",
    );
}
