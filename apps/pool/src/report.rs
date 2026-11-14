//! Rendering of the `check-config` result in the human and JSON formats.
//!
//! All text comes from constants, schema key paths and numbers. Nothing read
//! from the configuration file reaches the output.

use std::io::{self, Write};

use pool_config::{Finding, Location, Rejection, ValidatedConfig};
use serde::Serialize;

const OUTPUT_VERSION: u32 = 1;
const COMMAND: &str = "check-config";

/// A failure to read the configuration file (exit status 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoFinding {
    /// The path does not exist.
    NotFound,
    /// The operating system refused the read.
    PermissionDenied,
    /// Any other failure to open or read.
    ReadFailed,
}

impl IoFinding {
    /// Maps an operating system error kind to its finding.
    pub fn from_kind(kind: io::ErrorKind) -> Self {
        match kind {
            io::ErrorKind::NotFound => IoFinding::NotFound,
            io::ErrorKind::PermissionDenied => IoFinding::PermissionDenied,
            _ => IoFinding::ReadFailed,
        }
    }

    fn code(self) -> &'static str {
        match self {
            IoFinding::NotFound => "io.not_found",
            IoFinding::PermissionDenied => "io.permission_denied",
            IoFinding::ReadFailed => "io.read_failed",
        }
    }

    fn message(self) -> &'static str {
        match self {
            IoFinding::NotFound => "the configuration file does not exist",
            IoFinding::PermissionDenied => {
                "the configuration file cannot be read: permission denied"
            }
            IoFinding::ReadFailed => "the configuration file cannot be read",
        }
    }
}

/// The outcome of reading and judging the file.
pub enum Outcome {
    /// The file was accepted.
    Valid(ValidatedConfig),
    /// The file was rejected.
    Invalid(Rejection),
    /// The file could not be read.
    Error(IoFinding),
}

impl Outcome {
    /// The exit status this outcome maps to.
    pub fn exit_status(&self) -> u8 {
        match self {
            Outcome::Valid(_) => 0,
            Outcome::Invalid(_) => 1,
            Outcome::Error(_) => 3,
        }
    }
}

#[derive(Serialize)]
struct JsonReport<'a> {
    output_version: u32,
    command: &'static str,
    result: &'static str,
    config_version: Option<u32>,
    capabilities: Option<JsonCapabilities>,
    findings: &'a [JsonFinding],
}

#[derive(Serialize)]
struct JsonCapabilities {
    result_cache: &'static str,
    automatic_promotion: &'static str,
}

#[derive(Serialize)]
struct JsonFinding {
    code: &'static str,
    key: Option<&'static str>,
    line: Option<u32>,
    column: Option<u32>,
    message: &'static str,
}

impl JsonFinding {
    fn from_finding(finding: &Finding) -> Self {
        let (key, line, column) = match finding.location() {
            Location::None => (None, None, None),
            Location::Key(key) => (Some(key), None, None),
            Location::Position { line, column } => (None, Some(line), Some(column)),
        };
        JsonFinding {
            code: finding.code().as_str(),
            key,
            line,
            column,
            message: finding.code().message(),
        }
    }
}

/// Writes the human format: the result on stdout, an io failure on stderr.
pub fn write_human(
    outcome: &Outcome,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> io::Result<()> {
    match outcome {
        Outcome::Valid(config) => {
            writeln!(stdout, "configuration valid")?;
            writeln!(stdout, "config_version: {}", config.config_version())?;
            writeln!(stdout, "result_cache: {}", config.result_cache().as_str())?;
            writeln!(
                stdout,
                "automatic_promotion: {}",
                config.automatic_promotion().as_str()
            )?;
            stdout.flush()
        }
        Outcome::Invalid(rejection) => {
            writeln!(stdout, "configuration invalid")?;
            writeln!(stdout, "findings: {}", rejection.findings().len())?;
            for finding in rejection.findings() {
                writeln!(stdout, "{finding}")?;
            }
            stdout.flush()
        }
        Outcome::Error(io_finding) => {
            writeln!(
                stderr,
                "error: {}: {}",
                io_finding.code(),
                io_finding.message()
            )?;
            stderr.flush()
        }
    }
}

/// Writes the JSON format: one object on one line of stdout.
pub fn write_json(outcome: &Outcome, stdout: &mut impl Write) -> io::Result<()> {
    let (result, config_version, capabilities, findings) = match outcome {
        Outcome::Valid(config) => (
            "valid",
            Some(config.config_version()),
            Some(JsonCapabilities {
                result_cache: config.result_cache().as_str(),
                automatic_promotion: config.automatic_promotion().as_str(),
            }),
            Vec::new(),
        ),
        Outcome::Invalid(rejection) => (
            "invalid",
            None,
            None,
            rejection
                .findings()
                .iter()
                .map(JsonFinding::from_finding)
                .collect(),
        ),
        Outcome::Error(io_finding) => (
            "error",
            None,
            None,
            vec![JsonFinding {
                code: io_finding.code(),
                key: None,
                line: None,
                column: None,
                message: io_finding.message(),
            }],
        ),
    };
    let report = JsonReport {
        output_version: OUTPUT_VERSION,
        command: COMMAND,
        result,
        config_version,
        capabilities,
        findings: &findings,
    };
    serde_json::to_writer(&mut *stdout, &report).map_err(io::Error::from)?;
    writeln!(stdout)?;
    stdout.flush()
}
