//! The `pool` daemon executable.

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    match args.next() {
        Some(a) if a == "--version" || a == "-V" => {
            let mut out = std::io::stdout().lock();
            match writeln!(out, "pool {}", env!("CARGO_PKG_VERSION")).and_then(|()| out.flush()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(_) => ExitCode::from(3),
            }
        }
        _ => ExitCode::from(2),
    }
}
