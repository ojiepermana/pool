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
