//! Shared helpers: small repository trees on disk under `CARGO_TARGET_TMPDIR`, and runs of the real `pool-repo-check` binary.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Environment variables that would point `git` at another repository, for example when the suite runs from a Git hook.
const GIT_ENV: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
];

/// A directory of its own for one test, emptied when the test starts.
pub struct Fixture {
    /// The directory, which tests usually pass as `--root`.
    pub root: PathBuf,
}

impl Fixture {
    /// Creates an empty directory named after the test under `CARGO_TARGET_TMPDIR`.
    pub fn new(name: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR"))
            .join("repo-check-it")
            .join(name);
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove the old fixture");
        }
        fs::create_dir_all(&root).expect("create the fixture");
        Self { root }
    }

    /// Returns the full path of a file in the fixture.
    pub fn path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    /// Writes a file, creating its directories.
    pub fn write(&self, path: &str, content: &str) {
        self.write_bytes(path, content.as_bytes());
    }

    /// Writes raw bytes, creating the directories.
    pub fn write_bytes(&self, path: &str, content: &[u8]) {
        let full = self.path(path);
        fs::create_dir_all(full.parent().expect("a file has a parent")).expect("create dirs");
        fs::write(full, content).expect("write the fixture file");
    }

    /// Returns the content of a fixture file.
    pub fn read(&self, path: &str) -> String {
        fs::read_to_string(self.path(path)).expect("read the fixture file")
    }

    /// Replaces the first occurrence of `from` in a file.
    pub fn replace(&self, path: &str, from: &str, to: &str) {
        let text = self.read(path);
        assert!(text.contains(from), "{path} does not contain {from:?}");
        self.write(path, &text.replacen(from, to, 1));
    }

    /// Appends text to a file.
    pub fn append(&self, path: &str, more: &str) {
        let text = self.read(path);
        self.write(path, &format!("{text}{more}"));
    }

    /// Removes a file.
    pub fn remove(&self, path: &str) {
        fs::remove_file(self.path(path)).expect("remove the fixture file");
    }

    /// Runs `pool-repo-check <command> --root <fixture> <extra>...`.
    pub fn check(&self, command: &str, extra: &[&str]) -> Outcome {
        let mut args = vec![command, "--root"];
        let root = self.root.to_str().expect("UTF-8 fixture path");
        args.push(root);
        args.extend_from_slice(extra);
        run(&args, &[])
    }

    /// Runs `git` in the fixture with a fixed identity and no hooks, and asserts it succeeds.
    pub fn git(&self, args: &[&str]) -> String {
        let mut command = Command::new("git");
        command
            .arg("-C")
            .arg(&self.root)
            .args([
                "-c",
                "user.name=repo-check test",
                "-c",
                "user.email=repo-check@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "-c",
                "init.defaultBranch=main",
            ])
            .args(args);
        for name in GIT_ENV {
            command.env_remove(name);
        }
        let output = command.output().expect("run git");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    /// Makes the fixture a Git repository with one commit of everything in it.
    pub fn commit_all(&self, message: &str) -> String {
        if !self.path(".git").exists() {
            self.git(&["init", "-q"]);
        }
        self.git(&["add", "-A"]);
        self.git(&["commit", "-q", "-m", message]);
        self.git(&["rev-parse", "HEAD"]).trim().to_string()
    }
}

/// The result of one run of the binary.
#[derive(Debug)]
pub struct Outcome {
    /// The exit status, `-1` when the process was killed by a signal.
    pub code: i32,
    /// Everything printed on stdout.
    pub stdout: String,
    /// Everything printed on stderr.
    pub stderr: String,
}

impl Outcome {
    /// Returns the last line of stdout.
    pub fn last_line(&self) -> &str {
        self.stdout.lines().last().unwrap_or("")
    }

    /// Returns the problem lines of stdout: every line except the last.
    pub fn problems(&self) -> Vec<&str> {
        let lines: Vec<&str> = self.stdout.lines().collect();
        lines[..lines.len().saturating_sub(1)].to_vec()
    }

    /// Reports whether a problem line starts with `<rule>: ` and contains every text.
    pub fn has(&self, rule: &str, texts: &[&str]) -> bool {
        let prefix = format!("{rule}: ");
        self.problems()
            .iter()
            .any(|line| line.starts_with(&prefix) && texts.iter().all(|t| line.contains(t)))
    }

    /// Asserts exit 0 and the `ok` line as the only output.
    pub fn assert_ok(&self, command: &str) {
        assert_eq!(self.code, 0, "expected exit 0:\n{self}");
        assert_eq!(
            self.stdout,
            format!("pool-repo-check {command}: ok\n"),
            "expected only the ok line:\n{self}"
        );
    }

    /// Asserts exit 1, a problem line for the rule with every text, and the count as the last line.
    pub fn assert_problem(&self, command: &str, rule: &str, texts: &[&str]) {
        assert_eq!(self.code, 1, "expected exit 1:\n{self}");
        assert!(
            self.has(rule, texts),
            "expected a {rule} line with {texts:?}:\n{self}"
        );
        assert_eq!(
            self.last_line(),
            format!(
                "pool-repo-check {command}: {} problems",
                self.problems().len()
            ),
            "expected the problem count as the last line:\n{self}"
        );
    }

    /// Asserts exit 3, a `could not run` line on stderr that contains `text`, and the closing line on stdout.
    pub fn assert_not_run(&self, command: &str, text: &str) {
        assert_eq!(self.code, 3, "expected exit 3:\n{self}");
        assert!(
            self.stderr.lines().any(|line| line
                .starts_with(&format!("pool-repo-check {command}: could not run: "))
                && line.contains(text)),
            "expected a could not run line on stderr with {text:?}:\n{self}"
        );
        assert_eq!(
            self.last_line(),
            format!(
                "pool-repo-check {command}: could not run, {} problems found so far",
                self.problems().len()
            ),
            "expected the could not run line as the last line:\n{self}"
        );
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "exit {}\n--- stdout\n{}--- stderr\n{}",
            self.code, self.stdout, self.stderr
        )
    }
}

/// Runs the real binary with the given arguments and extra environment.
pub fn run(args: &[&str], env: &[(&str, &str)]) -> Outcome {
    let mut command = Command::new(env!("CARGO_BIN_EXE_pool-repo-check"));
    command.args(args);
    for name in GIT_ENV {
        command.env_remove(name);
    }
    for (name, value) in env {
        command.env(name, value);
    }
    let output = command.output().expect("run pool-repo-check");
    Outcome {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("stderr is UTF-8"),
    }
}

/// Returns the 1 based number of the first line of `text` that contains `needle`.
pub fn line_of(text: &str, needle: &str) -> usize {
    text.lines()
        .position(|line| line.contains(needle))
        .unwrap_or_else(|| panic!("no line contains {needle:?}"))
        + 1
}

/// The repository this suite belongs to: two levels above the package.
pub fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repository root exists")
}
