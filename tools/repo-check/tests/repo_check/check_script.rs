//! The exit status of `scripts/check.sh` (0002 AC-1), with stub `cargo` and `git` commands on `PATH`.
//!
//! Each test copies the script to `<fixture>/scripts/check.sh`, so the script changes to the fixture and never to
//! this repository. `PATH` holds only a stub directory: links to the few system tools the script needs and, when the
//! test wants them, stub `cargo` and `git` scripts that log their arguments and answer from environment variables.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common::{Fixture, Outcome, repository_root};

/// The system tools the script itself calls outside any check.
const SYSTEM_TOOLS: [&str; 2] = ["dirname", "cat"];

/// A stub `cargo`. `cargo deny --version` prints `$STUB_DENY_VERSION`, or fails when it is empty. Every other call
/// exits 1 when its subcommand is in `$STUB_FAIL`, 3 when it is in `$STUB_NOT_RUN`, and 0 otherwise.
const CARGO: &str = r#"#!/bin/sh
echo "cargo $*" >> "$STUB_LOG"
if [ "$1" = deny ] && [ "${2:-}" = --version ]; then
  [ -n "${STUB_DENY_VERSION:-}" ] || exit 101
  echo "cargo-deny $STUB_DENY_VERSION"
  exit 0
fi
for name in ${STUB_FAIL:-}; do
  if [ "$1" = "$name" ]; then echo "stub: $1 failed" >&2; exit 1; fi
done
for name in ${STUB_NOT_RUN:-}; do
  if [ "$1" = "$name" ]; then echo "stub: $1 could not run" >&2; exit 3; fi
done
exit 0
"#;

/// A stub `git` that has no `origin/main`.
const GIT: &str = r#"#!/bin/sh
echo "git $*" >> "$STUB_LOG"
exit 1
"#;

struct Script {
    fixture: Fixture,
    bin: PathBuf,
    log: PathBuf,
}

impl Script {
    /// Copies the script into a fresh fixture and builds the stub directory. `with` names the stubs to install.
    fn new(name: &str, with: &[&str]) -> Self {
        let fixture = Fixture::new(name);
        let source = repository_root().join("scripts/check.sh");
        fixture.write(
            "scripts/check.sh",
            &fs::read_to_string(source).expect("read check.sh"),
        );
        fixture.write("tools/repo-check/Cargo.toml", "[package]\n");
        let bin = fixture.path("stub-bin");
        fs::create_dir_all(&bin).expect("create the stub dir");
        for tool in SYSTEM_TOOLS {
            let found = ["/usr/bin", "/bin"]
                .iter()
                .map(|dir| Path::new(dir).join(tool))
                .find(|path| path.is_file())
                .unwrap_or_else(|| panic!("{tool} is not in /usr/bin or /bin"));
            symlink(found, bin.join(tool)).expect("link a system tool");
        }
        for stub in with {
            let body = match *stub {
                "cargo" => CARGO,
                "git" => GIT,
                other => panic!("no stub named {other}"),
            };
            let path = bin.join(stub);
            fs::write(&path, body).expect("write the stub");
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("chmod the stub");
        }
        let log = fixture.path("stub.log");
        fs::write(&log, "").expect("create the log");
        Self { fixture, bin, log }
    }

    /// Runs `sh scripts/check.sh <args>` with only the stub directory on `PATH`.
    fn run(&self, args: &[&str], env: &[(&str, &str)]) -> Outcome {
        let mut command = Command::new("/bin/sh");
        command
            .arg(self.fixture.path("scripts/check.sh"))
            .args(args)
            .env_clear()
            .env("PATH", &self.bin)
            .env("STUB_LOG", &self.log)
            .env("STUB_DENY_VERSION", "0.20.2")
            .current_dir(&self.fixture.root);
        for (name, value) in env {
            command.env(name, value);
        }
        let output = command.output().expect("run check.sh");
        Outcome {
            code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8(output.stdout).expect("stdout is UTF-8"),
            stderr: String::from_utf8(output.stderr).expect("stderr is UTF-8"),
        }
    }

    /// Returns the logged stub calls, one per line.
    fn calls(&self) -> String {
        fs::read_to_string(&self.log).expect("read the log")
    }
}

#[test]
fn no_name_exits_2_with_the_usage_on_stderr_and_runs_nothing() {
    // covers: 0002 AC-1: a usage error exits 2 and runs nothing.
    let s = Script::new("sh_no_name", &["cargo", "git"]);
    let out = s.run(&[], &[]);
    assert_eq!(out.code, 2, "{out}");
    assert_eq!(out.stdout, "", "{out}");
    assert!(
        out.stderr
            .starts_with("usage: scripts/check.sh <name> [<name>...]"),
        "{out}"
    );
    assert_eq!(s.calls(), "", "nothing ran");
}

#[test]
fn an_unknown_name_exits_2_before_any_check_runs() {
    // covers: 0002 AC-1: names are validated first, so `fmt lint` does not run fmt.
    let s = Script::new("sh_unknown", &["cargo", "git"]);
    let out = s.run(&["fmt", "lint"], &[]);
    assert_eq!(out.code, 2, "{out}");
    assert_eq!(out.stdout, "", "{out}");
    assert!(
        out.stderr.contains("scripts/check.sh: unknown check: lint"),
        "{out}"
    );
    assert!(out.stderr.contains("usage: scripts/check.sh"), "{out}");
    assert_eq!(s.calls(), "", "nothing ran");
}

#[test]
fn a_passing_check_ends_with_its_ok_line_and_exits_0() {
    // covers: 0002 AC-1: each of the six checks ends with `== ok: <name>` when its commands pass.
    for name in ["fmt", "clippy", "test", "deny", "guards", "docs"] {
        let s = Script::new(&format!("sh_ok_{name}"), &["cargo", "git"]);
        let out = s.run(&[name], &[]);
        assert_eq!(out.code, 0, "{name}:\n{out}");
        assert!(
            out.stdout.starts_with(&format!("== check: {name}\n")),
            "{name}:\n{out}"
        );
        assert!(
            out.stdout.ends_with(&format!("== ok: {name}\n")),
            "{name}:\n{out}"
        );
    }
}

#[test]
fn all_runs_the_six_checks_in_order_and_prints_a_summary() {
    // covers: 0002 AC-1
    let s = Script::new("sh_all_ok", &["cargo", "git"]);
    let out = s.run(&["all"], &[]);
    assert_eq!(out.code, 0, "{out}");
    let checks: Vec<&str> = out
        .stdout
        .lines()
        .filter_map(|line| line.strip_prefix("== check: "))
        .collect();
    assert_eq!(checks, ["fmt", "clippy", "test", "deny", "guards", "docs"]);
    assert!(
        out.stdout.ends_with(
            "== summary\n== ok: fmt\n== ok: clippy\n== ok: test\n== ok: deny\n== ok: guards\n== ok: docs\n"
        ),
        "{out}"
    );
    assert!(
        !out.stdout.contains("linux-build"),
        "all leaves out linux-build:\n{out}"
    );
}

#[test]
fn all_keeps_going_after_a_failure_and_exits_1() {
    // covers: 0002 AC-1: `all` does not stop at a failure; one failed check makes the status 1.
    let s = Script::new("sh_all_fail", &["cargo", "git"]);
    let out = s.run(&["all"], &[("STUB_FAIL", "fmt")]);
    assert_eq!(out.code, 1, "{out}");
    assert!(
        out.stdout.ends_with(
            "== summary\n== FAILED: fmt\n== ok: clippy\n== ok: test\n== ok: deny\n== ok: guards\n== ok: docs\n"
        ),
        "{out}"
    );
}

#[test]
fn a_missing_cargo_exits_3_with_one_line_that_names_it() {
    // covers: 0002 AC-1: a missing tool exits 3 with one line that says what is missing.
    let s = Script::new("sh_no_cargo", &[]);
    let out = s.run(&["fmt"], &[]);
    assert_eq!(out.code, 3, "{out}");
    assert_eq!(
        out.stderr, "fmt: cargo is required and was not found on PATH\n",
        "{out}"
    );
    assert_eq!(out.stdout, "== check: fmt\n== FAILED: fmt\n", "{out}");
}

#[test]
fn a_missing_git_makes_docs_exit_3() {
    // covers: 0002 AC-1
    let s = Script::new("sh_no_git", &["cargo"]);
    let out = s.run(&["docs"], &[]);
    assert_eq!(out.code, 3, "{out}");
    assert_eq!(
        out.stderr, "docs: git is required and was not found on PATH\n",
        "{out}"
    );
}

#[test]
fn a_wrong_cargo_deny_version_exits_3_and_names_the_tools_command() {
    // covers: 0002 AC-1, AC-4: the version check counts as could not run.
    let s = Script::new("sh_deny_version", &["cargo", "git"]);
    for version in ["0.20.1", ""] {
        let out = s.run(&["deny"], &[("STUB_DENY_VERSION", version)]);
        assert_eq!(out.code, 3, "version {version:?}:\n{out}");
        assert_eq!(
            out.stderr, "cargo-deny 0.20.2 is required; run scripts/check.sh tools\n",
            "version {version:?}:\n{out}"
        );
    }
    assert!(
        !s.calls().contains("cargo deny --locked"),
        "deny never ran its check"
    );
}

#[test]
fn exit_3_from_pool_repo_check_counts_as_could_not_run() {
    // covers: 0002 AC-1: exit 3 from pool-repo-check is could not run; any other failure is a failed check.
    let s = Script::new("sh_guards_3", &["cargo", "git"]);
    let out = s.run(&["guards"], &[("STUB_NOT_RUN", "run")]);
    assert_eq!(out.code, 3, "{out}");
    assert!(out.stdout.ends_with("== FAILED: guards\n"), "{out}");

    let out = s.run(&["guards"], &[("STUB_FAIL", "run")]);
    assert_eq!(out.code, 1, "{out}");
}

#[test]
fn a_cargo_exit_3_outside_pool_repo_check_is_a_failed_check() {
    // covers: 0002 AC-1: only pool-repo-check and linux-build.sh map exit 3 to could not run.
    let s = Script::new("sh_fmt_3", &["cargo", "git"]);
    let out = s.run(&["fmt"], &[("STUB_NOT_RUN", "fmt")]);
    assert_eq!(out.code, 1, "{out}");
}

#[test]
fn a_failed_check_wins_over_one_that_could_not_run() {
    // covers: 0002 AC-1: if both 1 and 3 occur in one call, the status is 1, in either order.
    let s = Script::new("sh_1_over_3", &["cargo", "git"]);
    let out = s.run(
        &["fmt", "deny"],
        &[("STUB_FAIL", "fmt"), ("STUB_DENY_VERSION", "")],
    );
    assert_eq!(out.code, 1, "{out}");
    assert!(
        out.stdout
            .ends_with("== summary\n== FAILED: fmt\n== FAILED: deny\n"),
        "{out}"
    );

    let out = s.run(
        &["deny", "fmt"],
        &[("STUB_FAIL", "fmt"), ("STUB_DENY_VERSION", "")],
    );
    assert_eq!(out.code, 1, "{out}");

    let out = s.run(&["deny", "clippy"], &[("STUB_DENY_VERSION", "")]);
    assert_eq!(out.code, 3, "3 alone stays 3:\n{out}");
}

#[test]
fn a_missing_repo_check_manifest_makes_guards_exit_3() {
    // covers: 0002 AC-1
    let s = Script::new("sh_no_manifest", &["cargo", "git"]);
    s.fixture.remove("tools/repo-check/Cargo.toml");
    let out = s.run(&["guards"], &[]);
    assert_eq!(out.code, 3, "{out}");
    assert!(
        out.stderr
            .contains("tools/repo-check/Cargo.toml does not exist"),
        "{out}"
    );
}

#[test]
fn docs_passes_the_base_from_pool_check_base_unless_it_is_forty_zeros() {
    // covers: 0002 AC-1, AC-10: how `docs` picks the revision it passes as --base.
    let s = Script::new("sh_docs_base", &["cargo", "git"]);
    s.run(&["docs"], &[("POOL_CHECK_BASE", "abc123")]);
    s.run(&["docs"], &[("POOL_CHECK_BASE", &"0".repeat(40))]);
    s.run(&["docs"], &[("POOL_CHECK_BASE", "")]);
    let runs: Vec<String> = s
        .calls()
        .lines()
        .filter(|line| line.starts_with("cargo run "))
        .map(|line| line.rsplit("-- ").next().unwrap_or("").to_string())
        .collect();
    assert_eq!(
        runs,
        ["docs --base abc123", "docs", "docs"],
        "{}",
        s.calls()
    );
}
