//! The `pool` daemon executable.
//!
//! This build has one working command, `check-config`, which validates a
//! configuration file offline and exits. It opens no socket and starts no
//! thread or async runtime.

mod cli;
mod report;

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::ExitCode;

use cli::{Format, Invocation};
use pool_config::MAX_CONFIG_BYTES;
use report::{IoFinding, Outcome};

const EXIT_USAGE: u8 = 2;
const EXIT_INCOMPLETE: u8 = 3;

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
        Invocation::CheckConfig { config, format } => check_config(Path::new(&config), format),
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
            Ok(config) => Outcome::Valid(config),
            Err(rejection) => Outcome::Invalid(rejection),
        },
        Err(error) => Outcome::Error(IoFinding::from_kind(error.kind())),
    };
    let mut stdout = io::stdout().lock();
    match format {
        Format::Human => report::write_human(&outcome, &mut stdout, &mut io::stderr().lock())?,
        Format::Json => report::write_json(&outcome, &mut stdout)?,
    }
    Ok(outcome.exit_status())
}

/// Reads at most one byte more than the limit, so an oversized file is never
/// held in full and is still reported as too large.
fn read_bounded(path: &Path) -> io::Result<Vec<u8>> {
    // The limit fits in a u64 on every target; the fallback keeps the bound.
    let limit = u64::try_from(MAX_CONFIG_BYTES).map_or(1_048_577, |max| max.saturating_add(1));
    let mut bytes = Vec::new();
    File::open(path)?.take(limit).read_to_end(&mut bytes)?;
    Ok(bytes)
}
fn  prove_fmt( ){}
