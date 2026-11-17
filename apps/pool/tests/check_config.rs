//! The built `pool` binary: exit status, exact stdout and stderr in both
//! formats (spec 0001 AC-3 to AC-7, AC-9 to AC-11; spec 0003 AC-1 to AC-4;
//! spec 0004 AC-5 to AC-8).

use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Mutex, PoisonError};

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
  pool check-config   --config <PATH> [--format <human|json>]
  pool bootstrap      --config <PATH> [--format <human|json>]
  pool show-authority --config <PATH> [--format <human|json>]
  pool run            --config <PATH>
  pool --help
  pool --version

Commands:
  check-config    Validate a configuration file offline and exit
  bootstrap       Probe every node once and create the journal
  show-authority  Print the authority state and the journal ID
  run             Serve clients until SIGTERM or SIGINT

Options:
  --config <PATH>          Configuration file (TOML)
  --format <human|json>    Output format (default: human); not for run

Exit status:
  0  success
  1  the input was rejected
  2  usage error
  3  the command could not complete
";

/// Held while a child of this test binary starts, and while
/// `pool_into_closed_pipe` still has the read end of its pipe open.
///
/// A child that starts while that read end exists gets a copy of it: on
/// every platform until its exec closes it, and on macOS for its whole life
/// when the start lands between `pipe()` and close on exec being set (the
/// standard library cannot set both at once there). The copy keeps the pipe
/// readable, so the write that test means to fail succeeds and `pool` exits
/// 0, 1 or 2 instead of 3. Tests run in parallel and almost all of them
/// start a child, so every start in this file goes through `spawn`.
static SPAWN: Mutex<()> = Mutex::new(());

/// Starts `command` while holding `SPAWN`.
fn spawn(command: &mut Command) -> Child {
    let _no_closed_pipe_reader_open = SPAWN.lock().unwrap_or_else(PoisonError::into_inner);
    command.spawn().expect("run pool")
}

/// Like `Command::output`, started through `spawn`.
fn output(command: &mut Command) -> Output {
    spawn(command.stdout(Stdio::piped()).stderr(Stdio::piped()))
        .wait_with_output()
        .expect("run pool")
}

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
    let output = output(
        Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(args)
            .stdin(Stdio::null()),
    );
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
        (&["serve"], "unknown command"),
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
        let output = spawn(
            Command::new(env!("CARGO_BIN_EXE_pool"))
                .args([
                    "check-config",
                    "--config",
                    &fixture("valid-minimal.toml"),
                    "--format",
                    format,
                ])
                .stdin(Stdio::null())
                .stdout(full.try_clone().expect("clone /dev/full"))
                .stderr(Stdio::piped()),
        )
        .wait_with_output()
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
            &["serve"],
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
    let output = output(
        Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(["check-config", "--config", "-valid.toml"])
            .current_dir(&dir)
            .stdin(Stdio::null()),
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_HUMAN);
    assert!(output.stderr.is_empty());
}

#[test]
fn ac3_a_relative_path_is_read_from_the_working_directory() {
    let output = output(
        Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(["check-config", "--config", "valid-capabilities-empty.toml"])
            .args(["--format", "json"])
            .current_dir(FIXTURES)
            .stdin(Stdio::null()),
    );
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_JSON);
    assert!(output.stderr.is_empty());
}

#[test]
fn ac3_valid_output_does_not_depend_on_the_working_directory_or_environment() {
    let path = valid();
    let output = output(
        Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(["check-config", "--config", &path])
            .current_dir(env!("CARGO_TARGET_TMPDIR"))
            .env_clear()
            .env("NO_COLOR", "")
            .env("TERM", "xterm-256color")
            .env("LANG", "de_DE.UTF-8")
            .stdin(Stdio::null()),
    );
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
            let output = output(
                Command::new(env!("CARGO_BIN_EXE_pool"))
                    .arg("check-config")
                    .arg("--config")
                    .arg(&path)
                    .stdin(Stdio::null()),
            );
            assert_eq!(output.status.code(), Some(0));
            assert_eq!(String::from_utf8_lossy(&output.stdout), VALID_HUMAN);
            assert!(output.stderr.is_empty());
        }
        // Some file systems (APFS) refuse names that are not UTF-8; then
        // the path is still used as given and reported as missing.
        Err(_) => {
            let output = output(
                Command::new(env!("CARGO_BIN_EXE_pool"))
                    .arg("check-config")
                    .arg("--config")
                    .arg(&path)
                    .stdin(Stdio::null()),
            );
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
    // No child may start while the read end exists (see `SPAWN`).
    let writer = {
        let _no_child_starts = SPAWN.lock().unwrap_or_else(PoisonError::into_inner);
        let (reader, writer) = std::io::pipe().expect("pipe");
        drop(reader);
        writer
    };
    let mut command = Command::new(env!("CARGO_BIN_EXE_pool"));
    command.args(args).stdin(Stdio::null());
    if close_stdout {
        command.stdout(writer).stderr(Stdio::piped());
    } else {
        command.stdout(Stdio::piped()).stderr(writer);
    }
    spawn(&mut command).wait_with_output().expect("run pool")
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
        let output = spawn(
            Command::new(env!("CARGO_BIN_EXE_pool"))
                .args(&args)
                .stdin(Stdio::null())
                .stdout(full.try_clone().expect("clone /dev/full"))
                .stderr(Stdio::piped()),
        )
        .wait_with_output()
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

// Spec 0003: the topology fixtures through the built binary, both formats,
// byte for byte (0003 AC-1 to AC-4).

const INVALID_VALUE: &str = "a value is not in the allowed form or range";
const MISSING_KEY: &str = "a required key is missing";

/// One expected finding: code, key, position, message.
type Expected = (&'static str, &'static str, Option<(u32, u32)>, &'static str);

/// The exact human report for these findings.
fn topology_human(findings: &[Expected]) -> String {
    let mut text = format!("configuration invalid\nfindings: {}\n", findings.len());
    for (code, key, at, message) in findings {
        match at {
            Some((line, column)) => text.push_str(&format!(
                "{code} at {key}, line {line}, column {column}: {message}\n"
            )),
            None => text.push_str(&format!("{code} at {key}: {message}\n")),
        }
    }
    text
}

/// The exact JSON report for these findings.
fn topology_json(findings: &[Expected]) -> String {
    let objects: Vec<String> = findings
        .iter()
        .map(|(code, key, at, message)| match at {
            Some((line, column)) => format!(
                "{{\"code\":\"{code}\",\"key\":\"{key}\",\"line\":{line},\"column\":{column},\"message\":\"{message}\"}}"
            ),
            None => key_finding_json(code, key, message),
        })
        .collect();
    invalid_json(&objects.join(","))
}

fn assert_topology_rejection(name: &str, findings: &[Expected]) {
    let path = fixture(name);
    assert_run(&check(&path, "human"), 1, &topology_human(findings), "");
    assert_run(&check(&path, "json"), 1, &topology_json(findings), "");
}

#[test]
fn ac1_topology_fixtures_report_exactly_the_valid_report() {
    for name in [
        "valid-topology-minimal.toml",
        "valid-topology.toml",
        "valid-topology-names.toml",
    ] {
        let path = fixture(name);
        assert_run(&check(&path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(&path, "json"), 0, VALID_JSON, "");
    }
    let only_version = made_file("only-config-version.toml", b"config_version = 1\n");
    let path = only_version.to_str().expect("utf-8 path");
    assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
    assert_run(&check(path, "json"), 0, VALID_JSON, "");
}

#[test]
fn ac2_each_invalid_topology_fixture_reports_its_one_finding() {
    let table: [(&str, Expected); 8] = [
        (
            "invalid-topology-missing-key.toml",
            (
                "config.missing_key",
                "identity.backend_login",
                Some((17, 1)),
                MISSING_KEY,
            ),
        ),
        (
            "invalid-topology-missing-state-directory.toml",
            (
                "config.missing_key",
                "control.state_directory",
                None,
                MISSING_KEY,
            ),
        ),
        (
            "invalid-topology-invalid-value.toml",
            (
                "config.invalid_value",
                "node.id",
                Some((12, 6)),
                INVALID_VALUE,
            ),
        ),
        (
            "invalid-topology-duplicate-id.toml",
            (
                "config.duplicate_id",
                "node.id",
                Some((18, 6)),
                "an id is used more than once",
            ),
        ),
        (
            "invalid-topology-unknown-reference.toml",
            (
                "config.unknown_reference",
                "cluster.expected_writer",
                Some((9, 19)),
                "a reference does not name a matching entry",
            ),
        ),
        (
            "invalid-topology-duplicate-endpoint.toml",
            (
                "config.duplicate_endpoint",
                "node.host",
                Some((20, 8)),
                "two nodes have the same host and port",
            ),
        ),
        (
            "invalid-topology-duplicate-identity.toml",
            (
                "config.duplicate_identity",
                "identity.principal",
                Some((27, 13)),
                "two identity entries have the same principal and database",
            ),
        ),
        (
            "invalid-topology-unsupported-profile.toml",
            (
                "config.unsupported_value",
                "identity.routing_profile",
                Some((24, 19)),
                "this value is not supported by this build",
            ),
        ),
    ];
    for (name, finding) in table {
        assert_topology_rejection(name, &[finding]);
    }
}

#[test]
fn ac2_the_spec_examples_of_the_finding_line_are_exact() {
    let path = fixture("invalid-topology-duplicate-id.toml");
    assert_run(
        &check(&path, "human"),
        1,
        "configuration invalid\nfindings: 1\nconfig.duplicate_id at node.id, line 18, column 6: an id is used more than once\n",
        "",
    );
    assert_run(
        &check(&path, "json"),
        1,
        "{\"output_version\":1,\"command\":\"check-config\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings\":[{\"code\":\"config.duplicate_id\",\"key\":\"node.id\",\"line\":18,\"column\":6,\"message\":\"an id is used more than once\"}]}\n",
        "",
    );
}

#[test]
fn ac3_invalid_topology_multiple_reports_five_findings_in_order() {
    assert_topology_rejection(
        "invalid-topology-multiple.toml",
        &[
            (
                "config.invalid_value",
                "control.authority_id",
                None,
                INVALID_VALUE,
            ),
            (
                "config.invalid_value",
                "cluster.system_identifier",
                Some((9, 21)),
                INVALID_VALUE,
            ),
            (
                "config.invalid_value",
                "node.port",
                Some((16, 8)),
                INVALID_VALUE,
            ),
            (
                "config.missing_key",
                "identity.credential_file",
                Some((18, 1)),
                MISSING_KEY,
            ),
            (
                "config.unknown_reference",
                "identity.cluster",
                Some((22, 11)),
                "a reference does not name a matching entry",
            ),
        ],
    );
}

#[test]
fn ac4_topology_canary_never_reaches_the_output() {
    assert_topology_rejection(
        "secret-topology-values.toml",
        &[
            (
                "config.invalid_value",
                "node.host",
                Some((14, 8)),
                INVALID_VALUE,
            ),
            (
                "config.invalid_value",
                "identity.credential_file",
                Some((23, 19)),
                INVALID_VALUE,
            ),
        ],
    );
    for name in ["secret-topology-values.toml", "valid-topology-names.toml"] {
        let path = fixture(name);
        for format in ["human", "json"] {
            let run = check(&path, format);
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
fn ac4_check_config_opens_no_file_the_configuration_names() {
    let text = "config_version = 1\n\n[control]\nstate_directory = \"/does/not/exist/pool-state\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5432\n\n[[identity]]\nid = \"a\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"/does/not/exist/secret\"\n";
    let made = made_file("missing-paths.toml", text.as_bytes());
    let path = made.to_str().expect("utf-8 path");
    assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
    assert_run(&check(path, "json"), 0, VALID_JSON, "");
}

// Made topology files through the binary, added by /test: the fourth
// location shape, the key only shape for [control], every new code in one
// run, the canary in topology values, and the exit status (0003 AC-1 to
// AC-4).

#[test]
fn ac2_a_control_finding_prints_its_key_without_a_position() {
    // covers: 0003 AC-2. Findings about [control] use Location::Key.
    let made = made_file(
        "topology-control-authority-id.toml",
        b"config_version = 1\n[control]\nstate_directory = \"/p\"\nauthority_id = \"0123456789ABCDEF0123456789abcdef\"\n",
    );
    let path = made.to_str().expect("utf-8 path");
    let finding = (
        "config.invalid_value",
        "control.authority_id",
        None,
        INVALID_VALUE,
    );
    assert_run(&check(path, "human"), 1, &topology_human(&[finding]), "");
    assert_run(&check(path, "json"), 1, &topology_json(&[finding]), "");
}

#[test]
fn ac2_an_inline_entry_missing_keys_is_reported_at_its_brace_in_both_formats() {
    // covers: 0003 AC-2. Every missing key of an inline entry is at its `{`.
    let made = made_file(
        "topology-inline-missing.toml",
        b"config_version = 1\nnode = [ {} ]\n",
    );
    let path = made.to_str().expect("utf-8 path");
    let findings: Vec<Expected> = ["node.id", "node.cluster", "node.host", "node.port"]
        .into_iter()
        .map(|key| ("config.missing_key", key, Some((2, 10)), MISSING_KEY))
        .collect();
    assert_run(&check(path, "human"), 1, &topology_human(&findings), "");
    assert_run(&check(path, "json"), 1, &topology_json(&findings), "");
}

#[test]
fn ac2_ac3_every_new_code_in_one_run_in_the_validation_order() {
    // covers: 0003 AC-2, AC-3. One file with each of the six new codes, plus
    // a missing key, through the binary in both formats.
    let text = "\
config_version = 1
[control]
state_directory = \"relative\"

[[cluster]]
id = \"main\"
system_identifier = \"1\"
expected_writer = \"pg9\"

[[node]]
id = \"pg1\"
cluster = \"main\"
host = \"h\"
port = 1

[[node]]
id = \"pg1\"
cluster = \"main\"
host = \"H\"
port = 1

[[identity]]
id = \"a\"
principal = \"p\"
database = \"d\"
cluster = \"main\"
backend_login = \"l\"
credential_file = \"/c\"
routing_profile = \"replica\"

[[identity]]
id = \"b\"
principal = \"p\"
database = \"d\"
cluster = \"main\"
credential_file = \"/c\"
";
    let made = made_file("topology-every-code.toml", text.as_bytes());
    let path = made.to_str().expect("utf-8 path");
    let findings: [Expected; 7] = [
        (
            "config.invalid_value",
            "control.state_directory",
            None,
            INVALID_VALUE,
        ),
        (
            "config.unsupported_value",
            "identity.routing_profile",
            Some((29, 19)),
            "this value is not supported by this build",
        ),
        (
            "config.missing_key",
            "identity.backend_login",
            Some((31, 1)),
            MISSING_KEY,
        ),
        (
            "config.duplicate_id",
            "node.id",
            Some((17, 6)),
            "an id is used more than once",
        ),
        (
            "config.unknown_reference",
            "cluster.expected_writer",
            Some((8, 19)),
            "a reference does not name a matching entry",
        ),
        (
            "config.duplicate_endpoint",
            "node.host",
            Some((19, 8)),
            "two nodes have the same host and port",
        ),
        (
            "config.duplicate_identity",
            "identity.principal",
            Some((33, 13)),
            "two identity entries have the same principal and database",
        ),
    ];
    assert_run(&check(path, "human"), 1, &topology_human(&findings), "");
    assert_run(&check(path, "json"), 1, &topology_json(&findings), "");
}

#[test]
fn ac4_a_canary_in_any_topology_value_never_reaches_the_output() {
    // covers: 0003 AC-4. The canary sits in values that are refused and in
    // values that are accepted; neither format prints it, on exit 0 or 1.
    let refused = format!(
        "config_version = 1\n[control]\nstate_directory = \"{CANARY}\"\nauthority_id = \"{CANARY}\"\n\n[[cluster]]\nid = \"{CANARY}\"\nsystem_identifier = \"{CANARY}\"\nexpected_writer = \"{CANARY}\"\n\n[[node]]\nid = \"{CANARY}\"\ncluster = \"{CANARY}\"\nhost = \"{CANARY}_x\"\nport = 1\n\n[[identity]]\nid = \"{CANARY}\"\nprincipal = \"{CANARY}\\n\"\ndatabase = \"{CANARY}\"\ncluster = \"{CANARY}\"\nbackend_database = \"{CANARY}\\u0000\"\nbackend_login = \"{CANARY}\"\ncredential_file = \"{CANARY}\"\nrouting_profile = \"{CANARY}\"\n"
    );
    let accepted = format!(
        "config_version = 1\n# {CANARY}\n[control]\nstate_directory = \"/{CANARY}\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"{CANARY}\"\nport = 1\n\n[[identity]]\nid = \"a\"\nprincipal = \"{CANARY}\"\ndatabase = \"{CANARY}\"\ncluster = \"main\"\nbackend_login = \"{CANARY}\"\ncredential_file = \"/{CANARY}\"\n"
    );
    for (name, text, code) in [
        ("topology-canary-refused.toml", refused, 1),
        ("topology-canary-accepted.toml", accepted, 0),
    ] {
        let made = made_file(name, text.as_bytes());
        let path = made.to_str().expect("utf-8 path");
        for format in ["human", "json"] {
            let run = check(path, format);
            assert_eq!(run.code, code, "{name} {format}: {}", run.stdout);
            assert!(!run.stdout.contains(CANARY), "{}", run.stdout);
            assert!(!run.stderr.contains(CANARY), "{}", run.stderr);
            assert!(!run.stdout.contains("9f2c7b1e"), "{}", run.stdout);
        }
    }
}

#[test]
fn ac1_made_topology_files_at_the_edges_report_exactly_the_valid_report() {
    // covers: 0003 AC-1. Empty arrays, an empty [control], and values at the
    // edge of their syntax are valid, with the unchanged valid report.
    let id63 = "a".repeat(63);
    let path4096 = format!("/{}", "p".repeat(4095));
    let host253 = "h".repeat(253);
    let texts = [
        "config_version = 1\ncluster = []\nnode = []\nidentity = []\n".to_owned(),
        "config_version = 1\n[control]\n".to_owned(),
        format!(
            "config_version = 1\n[control]\nstate_directory = \"{path4096}\"\nauthority_id = \"ffffffffffffffffffffffffffffffff\"\n[[cluster]]\nid = \"{id63}\"\nsystem_identifier = \"18446744073709551615\"\nexpected_writer = \"{id63}\"\n[[node]]\nid = \"{id63}\"\ncluster = \"{id63}\"\nhost = \"{host253}\"\nport = 65535\n"
        ),
    ];
    for (index, text) in texts.iter().enumerate() {
        let made = made_file(&format!("topology-edge-{index}.toml"), text.as_bytes());
        let path = made.to_str().expect("utf-8 path");
        assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(path, "json"), 0, VALID_JSON, "");
    }
    // One byte past each edge is exit 1.
    for (index, text) in [
        texts[2].replace("port = 65535", "port = 0"),
        texts[2].replace(&host253, &format!("{host253}h")),
        texts[2].replace(&path4096, &format!("{path4096}p")),
    ]
    .iter()
    .enumerate()
    {
        let made = made_file(&format!("topology-past-edge-{index}.toml"), text.as_bytes());
        let path = made.to_str().expect("utf-8 path");
        let run = check(path, "human");
        assert_eq!(run.code, 1, "{}", run.stdout);
        assert!(
            run.stdout.contains("config.invalid_value"),
            "{}",
            run.stdout
        );
        assert_eq!(run.stderr, "");
    }
}

/// Made input 3 of the text of `0003 AC-15`: every entry inline on
/// line 2 with port 0, as many as fit in 1 MiB.
fn ac15_input_3() -> (String, usize) {
    let tail = "]\n\n[control]\nstate_directory = \"/var/lib/pool\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"7412345678901234567\"\nexpected_writer = \"n0\"\n";
    let mut text = String::from("config_version = 1\nnode = [");
    let mut count = 0;
    loop {
        let comma = if count == 0 { "" } else { "," };
        let entry = format!(
            "{comma}{{id=\"n{count}\",cluster=\"main\",host=\"h{count}.db.internal\",port=0}}"
        );
        if text.len() + entry.len() + tail.len() > 1_048_576 {
            text.push_str(tail);
            return (text, count);
        }
        text.push_str(&entry);
        count += 1;
    }
}

#[test]
fn ac15_the_binary_reports_every_finding_of_input_3_within_ten_seconds() {
    // covers: 0003 AC-15, 0003 AC-2. Each run is timed from
    // start to exit and checked before the next one starts.
    let (text, count) = ac15_input_3();
    assert_eq!((count, text.len()), (17_268, 1_048_559));
    let made = made_file("ac15-input-3.toml", text.as_bytes());
    let path = made.to_str().expect("utf-8 path");
    let limit = std::time::Duration::from_secs(10);

    let started = std::time::Instant::now();
    let human = check(path, "human");
    let elapsed = started.elapsed();
    assert!(elapsed < limit, "human run took {elapsed:?}");
    assert_eq!(human.code, 1);
    assert_eq!(human.stderr, "");
    let lines: Vec<&str> = human.stdout.lines().collect();
    assert_eq!(lines[..2], ["configuration invalid", "findings: 17268"]);
    assert_eq!(lines.len(), 2 + count);
    assert_eq!(
        lines[2],
        format!("config.invalid_value at node.port, line 2, column 60: {INVALID_VALUE}")
    );
    let prefix = "config.invalid_value at node.port, line 2, column ";
    assert!(lines[2..].iter().all(|line| line.starts_with(prefix)));

    let started = std::time::Instant::now();
    let json = check(path, "json");
    let elapsed = started.elapsed();
    assert!(elapsed < limit, "json run took {elapsed:?}");
    assert_eq!(json.code, 1);
    assert_eq!(json.stderr, "");
    assert!(json.stdout.starts_with(
        "{\"output_version\":1,\"command\":\"check-config\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings\":[{\"code\":\"config.invalid_value\",\"key\":\"node.port\",\"line\":2,\"column\":60,"
    ));
    assert_eq!(json.stdout.matches("{\"code\":").count(), count);
    assert_eq!(
        json.stdout
            .matches("\"code\":\"config.invalid_value\",\"key\":\"node.port\",\"line\":2,")
            .count(),
        count
    );
    assert!(json.stdout.ends_with("}]}\n"));
    assert_eq!(json.stdout.lines().count(), 1);
}

#[test]
fn ac4_a_canary_outside_a_value_never_reaches_the_output() {
    // covers: 0003 AC-4 (review t4, added by /test). The canary sits in a key
    // that is not in the schema inside each new table, in table names, in a
    // comment, in an array entry, in values of the wrong type and in a string
    // that does not end. Each file is refused with exit 1, and neither format
    // prints any part of the canary.
    let c = CANARY;
    let control = "[control]\nstate_directory = \"/var/lib/pool\"\n";
    let cluster =
        "[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n";
    let node = "[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5432\n";
    let identity = "[[identity]]\nid = \"a\"\nprincipal = \"p\"\ndatabase = \"d\"\ncluster = \"main\"\nbackend_login = \"l\"\ncredential_file = \"/c\"\n";
    let file = |control: &str, cluster: &str, node: &str, identity: &str, tail: &str| {
        format!("config_version = 1\n{control}\n{cluster}\n{node}\n{identity}\n{tail}")
    };
    let texts = [
        file(&format!("{control}{c} = 1\n"), cluster, node, identity, ""),
        file(
            control,
            &format!("{cluster}{c} = \"x\"\n"),
            node,
            identity,
            "",
        ),
        file(
            control,
            cluster,
            &format!("{node}\"{c}\" = true\n"),
            identity,
            "",
        ),
        file(
            control,
            cluster,
            node,
            &format!("{identity}{c}.x = 1\n"),
            "",
        ),
        file(control, cluster, node, identity, &format!("[{c}]\nx = 1\n")),
        file(
            control,
            cluster,
            node,
            identity,
            &format!("[[{c}]]\nx = 1\n"),
        ),
        file(
            control,
            cluster,
            node,
            identity,
            &format!("[control.{c}]\n"),
        ),
        file(
            &format!("# {c}\n{control}"),
            cluster,
            &node.replace("port = 5432", &format!("port = 0 # {c}")),
            identity,
            "",
        ),
        file(
            control,
            cluster,
            &node.replace("5432", &format!("[\"{c}\"]")),
            identity,
            "",
        ),
        file(
            control,
            cluster,
            &node.replace("5432", &format!("\"{c}\"")),
            identity,
            "",
        ),
        file(
            control,
            &cluster.replace("\"1\"", &format!("{{ {c} = 1 }}")),
            node,
            identity,
            "",
        ),
        file(
            control,
            cluster,
            node,
            &format!("{identity}routing_profile = [\"{c}\"]\n"),
            "",
        ),
        file(
            control,
            cluster,
            &node.replace("\"10.0.0.11\"", &format!("\"{c}")),
            identity,
            "",
        ),
    ];
    for (index, text) in texts.iter().enumerate() {
        let made = made_file(
            &format!("topology-canary-position-{index}.toml"),
            text.as_bytes(),
        );
        let path = made.to_str().expect("utf-8 path");
        for format in ["human", "json"] {
            let run = check(path, format);
            assert_eq!(run.code, 1, "{text}\n{format}: {}", run.stdout);
            assert!(!run.stdout.is_empty(), "{text}");
            for output in [&run.stdout, &run.stderr] {
                assert!(!output.contains(CANARY), "{text}\n{output}");
                assert!(!output.contains("9f2c7b1e"), "{text}\n{output}");
            }
        }
    }
}

// Spec 0004: the new keys through the built binary, the closed name rule and
// the limit on findings (0004 AC-5 to AC-8).

/// The exact human and JSON output of a file with one finding inside an
/// entry, at a key, line and column.
fn assert_key_at_rejection(path: &str, code: &str, key: &str, at: (u32, u32), message: &str) {
    let (line, column) = at;
    assert_run(
        &check(path, "human"),
        1,
        &format!(
            "configuration invalid\nfindings: 1\n{code} at {key}, line {line}, column {column}: {message}\n"
        ),
        "",
    );
    assert_run(
        &check(path, "json"),
        1,
        &invalid_json(&format!(
            "{{\"code\":\"{code}\",\"key\":\"{key}\",\"line\":{line},\"column\":{column},\"message\":\"{message}\"}}"
        )),
        "",
    );
}

#[test]
fn spec_0004_ac5_valid_serving_fixtures_give_exactly_the_valid_report() {
    // covers: 0004 AC-5, AC-8. `check-config` does not report what a
    // command still needs: the probe only fixture is simply valid.
    for name in [
        "valid-serving.toml",
        "valid-serving-limits.toml",
        "valid-serving-probe-only.toml",
        "valid-serving-secret-values.toml",
    ] {
        let path = fixture(name);
        assert_run(&check(&path, "human"), 0, VALID_HUMAN, "");
        assert_run(&check(&path, "json"), 0, VALID_JSON, "");
        assert_run(
            &pool(["check-config", "--config", path.as_str()]),
            0,
            VALID_HUMAN,
            "",
        );
    }
}

#[test]
fn spec_0004_ac5_each_invalid_serving_fixture_gives_its_finding() {
    // covers: 0004 AC-5, AC-6. The table under "Fixtures" in spec 0004.
    for (name, code, key, message) in [
        (
            "invalid-serving-port.toml",
            "config.invalid_value",
            "listener.port",
            INVALID_VALUE,
        ),
        (
            "invalid-serving-listener-key.toml",
            "config.missing_key",
            "listener.tls_private_key_file",
            MISSING_KEY,
        ),
        (
            "invalid-serving-limit.toml",
            "config.invalid_value",
            "limits.session_max_count",
            INVALID_VALUE,
        ),
        (
            "invalid-serving-expiry.toml",
            "config.invalid_value",
            "probe.observation_expiry_ms",
            INVALID_VALUE,
        ),
        (
            "invalid-serving-node-loss.toml",
            "config.invalid_value",
            "probe.node_loss_timeout_ms",
            INVALID_VALUE,
        ),
    ] {
        assert_key_rejection(&fixture(name), code, key, message);
    }
    for (name, code, key, at, message) in [
        (
            "invalid-serving-duplicate-principal.toml",
            "config.duplicate_principal",
            "principal.name",
            (66, 8),
            "two principal entries have the same name",
        ),
        (
            "invalid-name-c1.toml",
            "config.invalid_value",
            "identity.backend_login",
            (22, 17),
            INVALID_VALUE,
        ),
        (
            "invalid-name-direction.toml",
            "config.invalid_value",
            "identity.database",
            (20, 12),
            INVALID_VALUE,
        ),
    ] {
        assert_key_at_rejection(&fixture(name), code, key, at, message);
    }
    let path = fixture("invalid-serving-unknown-key.toml");
    let message = "unknown key, or a value of the wrong type or range";
    assert_run(
        &check(&path, "human"),
        1,
        &format!(
            "configuration invalid\nfindings: 1\nconfig.schema at line 14, column 1: {message}\n"
        ),
        "",
    );
    assert_run(
        &check(&path, "json"),
        1,
        &invalid_json(&format!(
            "{{\"code\":\"config.schema\",\"key\":null,\"line\":14,\"column\":1,\"message\":\"{message}\"}}"
        )),
        "",
    );
}

#[test]
fn spec_0004_ac6_the_canary_never_reaches_the_output() {
    // covers: 0004 AC-6, SEC-11. Valid files with the canary in every new
    // string value, and names refused for a listed character.
    for name in [
        "valid-serving-secret-values.toml",
        "invalid-name-c1.toml",
        "invalid-name-direction.toml",
    ] {
        for format in ["human", "json"] {
            let run = check(&fixture(name), format);
            assert!(run.code == 0 || run.code == 1, "{name}");
            for text in [&run.stdout, &run.stderr] {
                assert!(!text.contains(CANARY), "{name} {format}: {text}");
            }
        }
    }
}

/// Made input 5 of spec 0004: `identity = [`, then empty entries on line 2,
/// as many as fit in 1048576 bytes. Every entry lacks its six required keys.
fn made_input_5() -> (String, usize) {
    let mut text = String::from("config_version = 1\nidentity = [");
    let mut count = 0;
    loop {
        let entry = if count == 0 { "{}" } else { ",{}" };
        if text.len() + entry.len() + 2 > 1_048_576 {
            text.push_str("]\n");
            return (text, count);
        }
        text.push_str(entry);
        count += 1;
    }
}

#[test]
fn spec_0004_ac7_the_binary_writes_twenty_thousand_findings_and_the_omitted_count() {
    // covers: 0004 AC-7. Exactly 20000 finding lines, less than 4 MiB in each
    // format, and the omitted count is six times the entries minus 20000.
    let (text, count) = made_input_5();
    let omitted = count * 6 - 20_000;
    let made = made_file("spec-0004-made-input-5.toml", text.as_bytes());
    let path = made.to_str().expect("utf-8 path");
    let limit = 4 * 1024 * 1024;

    let human = check(path, "human");
    assert_eq!(human.code, 1);
    assert_eq!(human.stderr, "");
    assert!(human.stdout.len() < limit, "{} bytes", human.stdout.len());
    let lines: Vec<&str> = human.stdout.lines().collect();
    assert_eq!(lines.len(), 2 + 20_000 + 1);
    assert_eq!(lines[..2], ["configuration invalid", "findings: 20000"]);
    assert_eq!(
        lines[2],
        format!("config.missing_key at identity.id, line 2, column 13: {MISSING_KEY}")
    );
    assert!(
        lines[2..20_002]
            .iter()
            .all(|line| line.starts_with("config.missing_key at identity."))
    );
    assert_eq!(lines[20_002], format!("findings omitted: {omitted}"));
    assert!(
        human
            .stdout
            .ends_with(&format!("findings omitted: {omitted}\n"))
    );

    let json = check(path, "json");
    assert_eq!(json.code, 1);
    assert_eq!(json.stderr, "");
    assert!(json.stdout.len() < limit, "{} bytes", json.stdout.len());
    assert_eq!(json.stdout.lines().count(), 1);
    assert!(json.stdout.starts_with(&format!(
        "{{\"output_version\":1,\"command\":\"check-config\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings_omitted\":{omitted},\"findings\":[{{\"code\":\"config.missing_key\",\"key\":\"identity.id\",\"line\":2,\"column\":13,"
    )));
    assert_eq!(json.stdout.matches("{\"code\":").count(), 20_000);
    assert_eq!(json.stdout.matches("findings_omitted").count(), 1);
    assert!(json.stdout.ends_with("}]}\n"));
}

#[test]
fn spec_0004_ac7_a_report_of_twenty_thousand_has_no_omitted_line_or_field() {
    // covers: 0004 AC-7. 5000 empty nodes give exactly 20000 findings.
    let entries = vec!["{}"; 5000].join(",");
    let text = format!("config_version = 1\nnode = [{entries}]\n");
    let made = made_file("spec-0004-twenty-thousand.toml", text.as_bytes());
    let path = made.to_str().expect("utf-8 path");
    let human = check(path, "human");
    assert_eq!(human.code, 1);
    assert_eq!(human.stdout.lines().count(), 2 + 20_000);
    assert!(!human.stdout.contains("omitted"));
    let json = check(path, "json");
    assert_eq!(json.code, 1);
    assert!(!json.stdout.contains("findings_omitted"));
    assert!(
        json.stdout
            .contains(",\"capabilities\":null,\"findings\":[")
    );
}

// ---- added by the tests of scope 004 (`/test one real primary path`) ----

#[test]
fn ac5_every_fixture_gives_the_result_its_name_prefix_promises() {
    // covers: 0004 AC-5. `valid-*` exits 0 with the valid report, `invalid-*`
    // and `secret-*` exit 1; no fixture has another prefix or extension.
    let mut names: Vec<String> = std::fs::read_dir(FIXTURES)
        .expect("the fixture directory")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .into_string()
                .expect("utf-8")
        })
        .collect();
    names.sort();
    assert!(names.len() >= 40, "{names:?}");
    for name in &names {
        assert!(name.ends_with(".toml"), "{name}");
        let path = fixture(name);
        for format in ["human", "json"] {
            let run = check(&path, format);
            if name.starts_with("valid-") {
                let valid = if format == "human" {
                    VALID_HUMAN
                } else {
                    VALID_JSON
                };
                assert_run(&run, 0, valid, "");
            } else if name.starts_with("invalid-") || name.starts_with("secret-") {
                assert_eq!(run.code, 1, "{name} {format}: {}", run.stdout);
                assert_eq!(run.stderr, "", "{name} {format}");
                assert!(!run.stdout.contains(CANARY), "{name} {format}");
            } else {
                panic!("{name}: a fixture name starts with valid-, invalid- or secret-");
            }
        }
    }
}

/// `valid-serving.toml` with `extra` added at the end and `from` replaced by `to`.
fn serving_with(name: &str, from: &str, to: &str, extra: &str) -> PathBuf {
    let text = std::fs::read_to_string(fixture("valid-serving.toml")).expect("fixture");
    assert!(text.contains(from), "{from}");
    let text = format!("{}\n{extra}", text.replacen(from, to, 1));
    made_file(name, text.as_bytes())
}

#[test]
fn ac5_each_new_number_key_takes_its_range_edges_and_refuses_one_past_them() {
    // covers: 0004 AC-5, the table under "Keys", through the built binary.
    let probe_room = "period_ms = 100\ntimeout_ms = 100\n";
    // (table, key, lowest, highest, the other keys the table needs)
    let keys: &[(&str, &str, u64, u64, &str)] = &[
        ("limits", "session_max_count", 1, 100_000, ""),
        ("limits", "handshake_max_count", 1, 10_000, ""),
        ("limits", "backend_wait_max_count", 0, 100_000, ""),
        ("limits", "node_backend_max_count", 1, 100_000, ""),
        ("limits", "startup_timeout_ms", 100, 60_000, ""),
        ("limits", "backend_acquire_timeout_ms", 100, 60_000, ""),
        ("limits", "shutdown_timeout_ms", 0, 600_000, ""),
        ("limits", "startup_packet_max_bytes", 512, 65_536, ""),
        (
            "limits",
            "frontend_message_max_bytes",
            1024,
            1_073_741_823,
            "",
        ),
        (
            "probe",
            "period_ms",
            100,
            30_000,
            "timeout_ms = 100\nobservation_expiry_ms = 60000\nnode_loss_timeout_ms = 90000\n",
        ),
        (
            "probe",
            "timeout_ms",
            100,
            30_000,
            "period_ms = 100\nobservation_expiry_ms = 60000\nnode_loss_timeout_ms = 90000\n",
        ),
        ("probe", "observation_expiry_ms", 1000, 60_000, probe_room),
    ];
    for (table, key, low, high, others) in keys {
        let mut values = vec![(*low, true), (*high, true), (high + 1, false)];
        if *low > 0 {
            values.push((low - 1, false));
        }
        for (value, ok) in values {
            let path = serving_with(
                &format!("range-{table}-{key}-{value}.toml"),
                "",
                "",
                &format!("[{table}]\n{others}{key} = {value}\n"),
            );
            let path = path.to_str().expect("utf-8");
            if ok {
                assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
            } else {
                assert_key_rejection(
                    path,
                    "config.invalid_value",
                    &format!("{table}.{key}"),
                    INVALID_VALUE,
                );
            }
        }
    }
    // The cross rule: the expiry is not less than the period plus the timeout.
    for (expiry, ok) in [(4000u64, true), (3999, false)] {
        let path = serving_with(
            &format!("range-expiry-cross-{expiry}.toml"),
            "",
            "",
            &format!(
                "[probe]\nperiod_ms = 3000\ntimeout_ms = 1000\nobservation_expiry_ms = {expiry}\n"
            ),
        );
        let path = path.to_str().expect("utf-8");
        if ok {
            assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        } else {
            assert_key_rejection(
                path,
                "config.invalid_value",
                "probe.observation_expiry_ms",
                INVALID_VALUE,
            );
        }
    }
    // listener.port and principal[].session_max_count. A port of 65536 does not fit the integer
    // type of the key, so it is `config.schema`, as the rule under "Keys" says; 0 fits and is out of range.
    let path = serving_with(
        "range-listener-port-65536.toml",
        "port = 6432",
        "port = 65536",
        "",
    );
    let run = check(path.to_str().expect("utf-8"), "human");
    assert_eq!(run.code, 1);
    assert!(
        run.stdout.contains("\nconfig.schema at line "),
        "{}",
        run.stdout
    );
    for (value, ok) in [(1u64, true), (65_535, true), (0, false)] {
        let path = serving_with(
            &format!("range-listener-port-{value}.toml"),
            "port = 6432",
            &format!("port = {value}"),
            "",
        );
        let path = path.to_str().expect("utf-8");
        if ok {
            assert_run(&check(path, "human"), 0, VALID_HUMAN, "");
        } else {
            assert_key_rejection(path, "config.invalid_value", "listener.port", INVALID_VALUE);
        }
    }
    for (value, ok) in [(1u64, true), (100_000, true), (0, false), (100_001, false)] {
        let from = "verifier_file = \"/etc/pool/secrets/report_app.scram\"";
        let path = serving_with(
            &format!("range-principal-limit-{value}.toml"),
            from,
            &format!("{from}\nsession_max_count = {value}"),
            "",
        );
        let run = check(path.to_str().expect("utf-8"), "human");
        if ok {
            assert_run(&run, 0, VALID_HUMAN, "");
        } else {
            assert_eq!(run.code, 1, "{value}");
            assert!(
                run.stdout.starts_with(
                    "configuration invalid\nfindings: 1\nconfig.invalid_value at principal.session_max_count, line "
                ),
                "{value}: {}",
                run.stdout
            );
        }
    }
}

#[test]
fn ac6_raw_c1_direction_and_separator_characters_in_names_and_paths_are_refused_without_the_canary()
{
    // covers: 0004 AC-6. Raw characters in the file, not TOML escapes. The
    // control characters below U+0080 are refused by TOML itself, so the raw
    // cases start at U+0080.
    let characters = [
        '\u{80}', '\u{85}', '\u{9f}', '\u{61c}', '\u{200e}', '\u{200f}', '\u{202a}', '\u{202e}',
        '\u{2066}', '\u{2069}', '\u{2028}', '\u{2029}',
    ];
    // (the text replaced, the key named by the finding, whether it is in a repeated table)
    let places = [
        ("name = \"orders_app\"", "principal.name", true),
        (
            "verifier_file = \"/etc/pool/secrets/orders_app.scram\"",
            "principal.verifier_file",
            true,
        ),
        (
            "monitor_login = \"pool_monitor\"",
            "cluster.monitor_login",
            true,
        ),
        (
            "tls_certificate_file = \"/etc/pool/tls/server.crt\"",
            "listener.tls_certificate_file",
            false,
        ),
    ];
    for c in characters {
        for (from, key, repeated) in places {
            let (head, value) = from.split_once(" = ").expect("a key and a value");
            let value = value.trim_matches('"');
            let to = format!("{head} = \"{value}{c}{CANARY}\"");
            let path = serving_with(
                &format!("raw-{:04x}-{}.toml", u32::from(c), key.replace('.', "-")),
                from,
                &to,
                "",
            );
            for format in ["human", "json"] {
                let run = check(path.to_str().expect("utf-8"), format);
                assert_eq!(run.code, 1, "U+{:04X} at {key} {format}", u32::from(c));
                assert!(
                    !run.stdout.contains(CANARY),
                    "U+{:04X} at {key}",
                    u32::from(c)
                );
                assert!(!run.stdout.contains(c), "U+{:04X} at {key}", u32::from(c));
                assert_eq!(run.stderr, "");
                if format == "human" {
                    let line = if repeated {
                        format!("config.invalid_value at {key}, line ")
                    } else {
                        format!("config.invalid_value at {key}: ")
                    };
                    assert!(
                        run.stdout
                            .starts_with(&format!("configuration invalid\nfindings: 1\n{line}")),
                        "U+{:04X} at {key}: {}",
                        u32::from(c),
                        run.stdout
                    );
                }
            }
        }
    }
}
