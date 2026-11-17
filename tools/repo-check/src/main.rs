//! `pool-repo-check`: the workspace guards, CI definition guards and document checks of spec 0002.
//!
//! It reads files under the root and, for `docs --base`, runs `git`. It writes nothing and opens no socket.

mod ci;
mod docs;
mod hardening;
mod ids;
mod report;
mod text;
mod tree;
mod workspace;

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use report::Report;
use tree::{FsTree, GitTree, Tree};

const USAGE: &str = "usage:
  pool-repo-check workspace [--root <DIR>]
  pool-repo-check ci        [--root <DIR>]
  pool-repo-check docs      [--root <DIR>] [--base <REVISION>]";

/// A parsed command line.
#[derive(Debug, PartialEq, Eq)]
struct Args {
    command: String,
    root: PathBuf,
    base: Option<String>,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut iter = args.iter();
    let command = iter.next().ok_or("no command given")?.clone();
    if !matches!(command.as_str(), "workspace" | "ci" | "docs") {
        return Err(format!("unknown command {command}"));
    }
    let mut root = None;
    let mut base = None;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--root" if root.is_none() => {
                root = Some(PathBuf::from(
                    iter.next().ok_or("--root needs a directory")?,
                ));
            }
            "--base" if base.is_none() && command == "docs" => {
                base = Some(iter.next().ok_or("--base needs a revision")?.clone());
            }
            other => return Err(format!("unexpected argument {other}")),
        }
    }
    Ok(Args {
        command,
        root: root.unwrap_or_else(|| PathBuf::from(".")),
        base,
    })
}

fn run(args: &Args) -> Report {
    let tree = FsTree::new(args.root.clone());
    match args.command.as_str() {
        "workspace" => workspace::check(&tree),
        "ci" => ci::check(&tree),
        _ => match &args.base {
            None => docs::check(&tree, None),
            Some(revision) => match GitTree::open(&args.root, revision) {
                Ok(base) => docs::check(&tree, Some(&base as &dyn Tree)),
                Err(e) => {
                    let mut report = Report::default();
                    report.fault(e);
                    report
                }
            },
        },
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse(&argv) {
        Ok(args) => args,
        Err(e) => {
            let _ = writeln!(io::stderr(), "pool-repo-check: {e}\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    if !args.root.is_dir() {
        let _ = writeln!(
            io::stderr(),
            "pool-repo-check {}: the root {} is not a directory",
            args.command,
            args.root.display()
        );
        return ExitCode::from(3);
    }
    let report = run(&args);
    let mut out = io::stdout().lock();
    for problem in &report.problems {
        let _ = writeln!(out, "{problem}");
    }
    let mut err = io::stderr().lock();
    for fault in &report.faults {
        let _ = writeln!(
            err,
            "pool-repo-check {}: could not run: {fault}",
            args.command
        );
    }
    if !report.faults.is_empty() {
        let _ = writeln!(
            out,
            "pool-repo-check {}: could not run, {} problems found so far",
            args.command,
            report.problems.len()
        );
        ExitCode::from(3)
    } else if !report.problems.is_empty() {
        let _ = writeln!(
            out,
            "pool-repo-check {}: {} problems",
            args.command,
            report.problems.len()
        );
        ExitCode::from(1)
    } else {
        let _ = writeln!(out, "pool-repo-check {}: ok", args.command);
        ExitCode::SUCCESS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Result<Args, String> {
        parse(&list.iter().map(ToString::to_string).collect::<Vec<_>>())
    }

    #[test]
    fn parses_commands_and_options() {
        assert_eq!(
            args(&["docs", "--root", "r", "--base", "abc"]),
            Ok(Args {
                command: "docs".into(),
                root: PathBuf::from("r"),
                base: Some("abc".into()),
            })
        );
        assert_eq!(args(&["ci"]).map(|a| a.root), Ok(PathBuf::from(".")));
    }

    #[test]
    fn refuses_bad_usage() {
        assert!(args(&[]).is_err());
        assert!(args(&["lint"]).is_err());
        assert!(args(&["workspace", "--base", "x"]).is_err());
        assert!(args(&["docs", "--root"]).is_err());
        assert!(args(&["docs", "--root", "a", "--root", "b"]).is_err());
    }
}
