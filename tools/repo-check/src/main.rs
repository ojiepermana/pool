//! `pool-repo-check`: the workspace guards and CI definition guards of spec 0002.
//!
//! It reads files under the root. It writes nothing and opens no socket.

mod ci;
mod report;
mod text;
mod tree;
mod workspace;

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use report::Report;
use tree::FsTree;

const USAGE: &str = "usage:
  pool-repo-check workspace [--root <DIR>]
  pool-repo-check ci        [--root <DIR>]";

/// A parsed command line.
#[derive(Debug, PartialEq, Eq)]
struct Args {
    command: String,
    root: PathBuf,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut iter = args.iter();
    let command = iter.next().ok_or("no command given")?.clone();
    if !matches!(command.as_str(), "workspace" | "ci") {
        return Err(format!("unknown command {command}"));
    }
    let mut root = None;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--root" if root.is_none() => {
                root = Some(PathBuf::from(
                    iter.next().ok_or("--root needs a directory")?,
                ));
            }
            other => return Err(format!("unexpected argument {other}")),
        }
    }
    Ok(Args {
        command,
        root: root.unwrap_or_else(|| PathBuf::from(".")),
    })
}

fn run(args: &Args) -> Report {
    let tree = FsTree::new(args.root.clone());
    match args.command.as_str() {
        "workspace" => workspace::check(&tree),
        _ => ci::check(&tree),
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
            args(&["ci", "--root", "r"]),
            Ok(Args {
                command: "ci".into(),
                root: PathBuf::from("r"),
            })
        );
        assert_eq!(args(&["workspace"]).map(|a| a.root), Ok(PathBuf::from(".")));
    }

    #[test]
    fn refuses_bad_usage() {
        assert!(args(&[]).is_err());
        assert!(args(&["lint"]).is_err());
        assert!(args(&["workspace", "--base", "x"]).is_err());
        assert!(args(&["ci", "--root"]).is_err());
        assert!(args(&["ci", "--root", "a", "--root", "b"]).is_err());
    }
}
