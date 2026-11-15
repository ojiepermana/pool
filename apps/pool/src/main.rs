//! The `pool` daemon executable.
//!
//! This build has four commands. `check-config` validates a configuration
//! file offline. `bootstrap` probes every node once and creates the journal.
//! `show-authority` reads the journal and prints its state and ID. `run` is
//! the daemon: it serves clients until SIGTERM or SIGINT. The three last
//! ones check and lock the state directory first.

mod cli;
mod commands;
mod control;
mod daemon;
mod log;
mod output;
mod probe;
mod report;
mod state;

use std::fs::File;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use std::process::ExitCode;

use cli::{Command, Format, Invocation};
use pool_config::MAX_CONFIG_BYTES;
use report::{IoFinding, Outcome};

const EXIT_USAGE: u8 = 2;
const EXIT_INCOMPLETE: u8 = 3;

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

fn main() -> ExitCode {
    let status = match cli::parse(std::env::args_os().skip(1)) {
        Invocation::Help => write_stdout(HELP.as_bytes()).map(|()| 0),
        Invocation::Version => {
            let line = format!("pool {}\n", env!("CARGO_PKG_VERSION"));
            write_stdout(line.as_bytes()).map(|()| 0)
        }
        Invocation::Usage(message) => {
            let mut stderr = io::stderr().lock();
            writeln!(stderr, "error: {message}")
                .and_then(|()| writeln!(stderr, "Run 'pool --help' for usage."))
                .and_then(|()| stderr.flush())
                .map(|()| EXIT_USAGE)
        }
        Invocation::Command {
            command: Command::CheckConfig,
            config,
            format,
        } => check_config(Path::new(&config), format),
        Invocation::Command {
            command,
            config,
            format,
        } => run_command(command, Path::new(&config), format),
    };
    ExitCode::from(status.unwrap_or(EXIT_INCOMPLETE))
}

fn write_stdout(bytes: &[u8]) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(bytes)?;
    stdout.flush()
}

fn check_config(path: &Path, format: Format) -> io::Result<u8> {
    let outcome = match read_bounded(path) {
        Ok(bytes) => match pool_config::validate(&bytes) {
            Ok(config) => Outcome::Valid(Box::new(config)),
            Err(rejection) => Outcome::Invalid(rejection),
        },
        Err(error) => Outcome::Error(IoFinding::from_kind(error.kind())),
    };
    // One buffer around the locked handle, so a report of 20000 findings is
    // not one system call per line. Each writer flushes it at its end, and a
    // failed flush is exit status 3 like any failed write.
    let mut stdout = BufWriter::new(io::stdout().lock());
    match format {
        Format::Human => report::write_human(&outcome, &mut stdout, &mut io::stderr().lock())?,
        Format::Json => report::write_json(&outcome, &mut stdout)?,
    }
    stdout.flush()?;
    Ok(outcome.exit_status())
}

/// Runs `bootstrap` or `show-authority` and writes its report.
fn run_command(command: Command, path: &Path, format: Format) -> io::Result<u8> {
    let report = match command {
        Command::Bootstrap => commands::bootstrap_file(path),
        Command::ShowAuthority => commands::show_authority_file(path),
        Command::Run => match daemon::run_file(path) {
            Some(report) => report,
            None => return Ok(0),
        },
        Command::CheckConfig => return check_config(path, format),
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    output::write(
        command,
        &report,
        format,
        &mut stdout,
        &mut io::stderr().lock(),
    )?;
    stdout.flush()?;
    Ok(report.exit_status())
}

/// Reads at most one byte more than the limit, so an oversized file is never
/// held in full and is still reported as too large.
pub(crate) fn read_bounded(path: &Path) -> io::Result<Vec<u8>> {
    // The limit fits in a u64 on every target; the fallback keeps the bound.
    let limit = u64::try_from(MAX_CONFIG_BYTES).map_or(1_048_577, |max| max.saturating_add(1));
    let mut bytes = Vec::new();
    File::open(path)?.take(limit).read_to_end(&mut bytes)?;
    Ok(bytes)
}
