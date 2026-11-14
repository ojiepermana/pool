//! The command line of `pool-repo-check`: usage errors, a root that cannot be read, and the committed tree (0002 AC-6 to AC-10).

use crate::common::{Fixture, repository_root, run};

#[test]
fn usage_errors_exit_2_with_the_usage_on_stderr_and_nothing_on_stdout() {
    // covers: 0002 AC-6, AC-7, AC-8: exit 2 for a usage error, message on stderr.
    let cases: [&[&str]; 8] = [
        &[],
        &["lint"],
        &["--root", "."],
        &["workspace", "--base", "HEAD"],
        &["ci", "--base", "HEAD"],
        &["docs", "--root"],
        &["docs", "--base"],
        &["docs", "--root", ".", "--root", "."],
    ];
    for args in cases {
        let out = run(args, &[]);
        assert_eq!(out.code, 2, "{args:?}:\n{out}");
        assert_eq!(out.stdout, "", "{args:?}:\n{out}");
        assert!(
            out.stderr.starts_with("pool-repo-check: "),
            "{args:?}:\n{out}"
        );
        assert!(
            out.stderr
                .contains("pool-repo-check docs      [--root <DIR>] [--base <REVISION>]"),
            "{args:?}:\n{out}"
        );
    }
}

#[test]
fn a_root_that_is_not_a_directory_exits_3_before_any_rule_runs() {
    // covers: 0002 AC-6, AC-7, AC-8: exit 3 when the root cannot be read; stdout stays empty.
    let f = Fixture::new("cli_root_file");
    f.write("file.txt", "not a directory\n");
    for command in ["workspace", "ci", "docs"] {
        for root in [f.path("missing"), f.path("file.txt")] {
            let root = root.to_str().expect("UTF-8 path");
            let out = run(&[command, "--root", root], &[]);
            assert_eq!(out.code, 3, "{command} {root}:\n{out}");
            assert_eq!(out.stdout, "", "{command} {root}:\n{out}");
            assert!(
                out.stderr.contains(&format!(
                    "pool-repo-check {command}: the root {root} is not a directory"
                )),
                "{command} {root}:\n{out}"
            );
        }
    }
}

#[test]
fn an_empty_root_could_not_run_for_each_command() {
    // covers: 0002 AC-6, AC-7, AC-8: every file a command needs is missing, exit 3 with the closing line.
    let f = Fixture::new("cli_empty");
    for (command, file) in [
        ("workspace", "Cargo.toml: file is missing"),
        ("ci", ".github/workflows/ci.yml: file is missing"),
        ("docs", "docs/scope/index.md: file is missing"),
    ] {
        f.check(command, &[]).assert_not_run(command, file);
    }
}

#[test]
fn the_root_defaults_to_the_current_directory() {
    // covers: 0002 AC-6: without --root the program checks the directory it runs in.
    let root = repository_root();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pool-repo-check"))
        .arg("ci")
        .current_dir(&root)
        .output()
        .expect("run pool-repo-check");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "pool-repo-check ci: ok\n"
    );
}

#[test]
fn every_command_passes_on_the_committed_tree() {
    // covers: 0002 AC-6, AC-7, AC-8, AC-9, AC-10: each command passes on this repository.
    let root = repository_root();
    let root = root.to_str().expect("UTF-8 path");
    for command in ["workspace", "ci", "docs"] {
        let out = run(&[command, "--root", root], &[]);
        assert_eq!(out.code, 0, "{command}:\n{out}");
        assert_eq!(
            out.stdout,
            format!("pool-repo-check {command}: ok\n"),
            "{command}:\n{out}"
        );
    }
}
