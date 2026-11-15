//! The reports of `bootstrap` and `show-authority` (spec 0004, "Commands"
//! and "Findings of the new commands"), in the human and JSON shapes of
//! spec 0001.
//!
//! All text comes from constants, schema key paths, reason names, numbers
//! and the authority ID. Nothing read from the configuration, from a file it
//! names or from a node reaches the output.

use std::io::{self, Write};

use pool_config::{AuthorityId, Finding, Location, Rejection};
use serde::Serialize;

use crate::cli::{Command, Format};

const OUTPUT_VERSION: u32 = 1;

/// One finding as the reports print it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Item {
    code: &'static str,
    key: Option<&'static str>,
    line: Option<u32>,
    column: Option<u32>,
    message: &'static str,
}

impl Item {
    /// A finding of the configuration.
    pub fn from_finding(finding: &Finding) -> Item {
        let (key, line, column) = match finding.location() {
            Location::None => (None, None, None),
            Location::Key(key) => (Some(key), None, None),
            Location::Position { line, column } => (None, Some(line), Some(column)),
            Location::KeyAt { key, line, column } => (Some(key), Some(line), Some(column)),
        };
        Item {
            code: finding.code().as_str(),
            key,
            line,
            column,
            message: finding.code().message(),
        }
    }

    /// A finding about the whole input, with no key.
    pub fn plain(code: &'static str, message: &'static str) -> Item {
        Item {
            code,
            key: None,
            line: None,
            column: None,
            message,
        }
    }

    /// A finding about the file a configuration key names. It carries the
    /// key, never the path of the file.
    pub fn at_key(code: &'static str, key: &'static str, message: &'static str) -> Item {
        Item {
            code,
            key: Some(key),
            line: None,
            column: None,
            message,
        }
    }

    /// The finding line, in the four forms of spec 0001 and spec 0003.
    fn line(&self) -> String {
        let Item {
            code,
            key,
            line,
            column,
            message,
        } = *self;
        match (key, line, column) {
            (Some(key), Some(line), Some(column)) => {
                format!("{code} at {key}, line {line}, column {column}: {message}")
            }
            (None, Some(line), Some(column)) => {
                format!("{code} at line {line}, column {column}: {message}")
            }
            (Some(key), _, _) => format!("{code} at {key}: {message}"),
            (None, _, _) => format!("{code}: {message}"),
        }
    }
}

/// What a command reports.
#[derive(Debug, PartialEq, Eq)]
pub enum Report {
    /// Exit 1: the configuration is invalid, or a view found it incomplete.
    Invalid {
        /// The findings that were kept.
        items: Vec<Item>,
        /// The findings beyond the limit, counted and not kept.
        omitted: u64,
    },
    /// Exit 1: the input (the configuration with the state it points at) was
    /// refused.
    Refused(Vec<Item>),
    /// Exit 3: the command could not complete.
    Error(Item),
    /// Exit 0: `bootstrap` wrote a journal.
    Bootstrapped {
        /// The ID of the new journal.
        authority_id: AuthorityId,
        /// The number of clusters it holds.
        clusters: usize,
    },
    /// Exit 0: `show-authority` read the state.
    Authority {
        /// The name of the state.
        state: &'static str,
        /// The ID of the journal that was read, when one could be read.
        authority_id: Option<AuthorityId>,
    },
}

impl Report {
    /// The report of a rejected configuration.
    pub fn invalid(rejection: &Rejection) -> Report {
        Report::Invalid {
            items: rejection
                .findings()
                .iter()
                .map(Item::from_finding)
                .collect(),
            omitted: rejection.omitted(),
        }
    }

    /// The exit status of the report.
    pub fn exit_status(&self) -> u8 {
        match self {
            Report::Invalid { .. } | Report::Refused(_) => 1,
            Report::Error(_) => 3,
            Report::Bootstrapped { .. } | Report::Authority { .. } => 0,
        }
    }
}

/// Writes the report: on stdout, except the human form of an error, which
/// goes to stderr.
pub fn write(
    command: Command,
    report: &Report,
    format: Format,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> io::Result<()> {
    match format {
        Format::Human => write_human(command, report, stdout, stderr),
        Format::Json => write_json(command, report, stdout),
    }
}

fn write_human(
    command: Command,
    report: &Report,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> io::Result<()> {
    match report {
        Report::Invalid { items, omitted } => {
            writeln!(stdout, "configuration invalid")?;
            write_items(stdout, items)?;
            if *omitted > 0 {
                writeln!(stdout, "findings omitted: {omitted}")?;
            }
            stdout.flush()
        }
        Report::Refused(items) => {
            writeln!(stdout, "{} refused", command.name())?;
            write_items(stdout, items)?;
            stdout.flush()
        }
        Report::Error(item) => {
            writeln!(stderr, "error: {}", item.line())?;
            stderr.flush()
        }
        Report::Bootstrapped {
            authority_id,
            clusters,
        } => {
            writeln!(stdout, "bootstrap complete")?;
            writeln!(stdout, "authority_id: {authority_id}")?;
            writeln!(stdout, "clusters: {clusters}")?;
            writeln!(
                stdout,
                "next: add control.authority_id to the configuration"
            )?;
            stdout.flush()
        }
        Report::Authority {
            state,
            authority_id,
        } => {
            writeln!(stdout, "authority state: {state}")?;
            if let Some(authority_id) = authority_id {
                writeln!(stdout, "authority_id: {authority_id}")?;
            }
            stdout.flush()
        }
    }
}

fn write_items(stdout: &mut impl Write, items: &[Item]) -> io::Result<()> {
    writeln!(stdout, "findings: {}", items.len())?;
    for item in items {
        writeln!(stdout, "{}", item.line())?;
    }
    Ok(())
}

/// The JSON object of a report that has findings: the shape of spec 0001.
#[derive(Serialize)]
struct JsonFindings<'a> {
    output_version: u32,
    command: &'static str,
    result: &'static str,
    config_version: Option<u32>,
    capabilities: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    findings_omitted: Option<u64>,
    findings: &'a [Item],
}

#[derive(Serialize)]
struct JsonBootstrapped {
    output_version: u32,
    command: &'static str,
    result: &'static str,
    authority_id: String,
    clusters: usize,
    findings: [Item; 0],
}

#[derive(Serialize)]
struct JsonAuthority {
    output_version: u32,
    command: &'static str,
    result: &'static str,
    authority_id: Option<String>,
    findings: [Item; 0],
}

fn write_findings(
    stdout: &mut impl Write,
    command: Command,
    result: &'static str,
    items: &[Item],
    omitted: u64,
) -> serde_json::Result<()> {
    serde_json::to_writer(
        stdout,
        &JsonFindings {
            output_version: OUTPUT_VERSION,
            command: command.name(),
            result,
            config_version: None,
            capabilities: None,
            findings_omitted: (omitted > 0).then_some(omitted),
            findings: items,
        },
    )
}

fn write_json(command: Command, report: &Report, stdout: &mut impl Write) -> io::Result<()> {
    let written = match report {
        Report::Invalid { items, omitted } => {
            write_findings(&mut *stdout, command, "invalid", items, *omitted)
        }
        Report::Refused(items) => write_findings(&mut *stdout, command, "refused", items, 0),
        Report::Error(item) => write_findings(
            &mut *stdout,
            command,
            "error",
            std::slice::from_ref(item),
            0,
        ),
        Report::Bootstrapped {
            authority_id,
            clusters,
        } => serde_json::to_writer(
            &mut *stdout,
            &JsonBootstrapped {
                output_version: OUTPUT_VERSION,
                command: command.name(),
                result: "created",
                authority_id: authority_id.to_string(),
                clusters: *clusters,
                findings: [],
            },
        ),
        Report::Authority {
            state,
            authority_id,
        } => serde_json::to_writer(
            &mut *stdout,
            &JsonAuthority {
                output_version: OUTPUT_VERSION,
                command: command.name(),
                result: state,
                authority_id: authority_id.map(|id| id.to_string()),
                findings: [],
            },
        ),
    };
    written.map_err(io::Error::from)?;
    writeln!(stdout)?;
    stdout.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(command: Command, report: &Report, format: Format) -> (String, String) {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        write(command, report, format, &mut out, &mut err).expect("write");
        (
            String::from_utf8(out).expect("utf-8"),
            String::from_utf8(err).expect("utf-8"),
        )
    }

    fn id() -> AuthorityId {
        AuthorityId::parse("0123456789abcdef0123456789abcdef").expect("id")
    }

    #[test]
    fn bootstrap_complete_matches_the_spec_text() {
        let report = Report::Bootstrapped {
            authority_id: id(),
            clusters: 1,
        };
        assert_eq!(
            render(Command::Bootstrap, &report, Format::Human),
            (
                "bootstrap complete\nauthority_id: 0123456789abcdef0123456789abcdef\nclusters: 1\nnext: add control.authority_id to the configuration\n".to_owned(),
                String::new()
            )
        );
        assert_eq!(
            render(Command::Bootstrap, &report, Format::Json).0,
            "{\"output_version\":1,\"command\":\"bootstrap\",\"result\":\"created\",\"authority_id\":\"0123456789abcdef0123456789abcdef\",\"clusters\":1,\"findings\":[]}\n"
        );
        assert_eq!(report.exit_status(), 0);
    }

    #[test]
    fn show_authority_matches_the_spec_text() {
        let report = Report::Authority {
            state: "unconfirmed",
            authority_id: Some(id()),
        };
        assert_eq!(
            render(Command::ShowAuthority, &report, Format::Human).0,
            "authority state: unconfirmed\nauthority_id: 0123456789abcdef0123456789abcdef\n"
        );
        assert_eq!(
            render(Command::ShowAuthority, &report, Format::Json).0,
            "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"unconfirmed\",\"authority_id\":\"0123456789abcdef0123456789abcdef\",\"findings\":[]}\n"
        );
        let none = Report::Authority {
            state: "uninitialized",
            authority_id: None,
        };
        assert_eq!(
            render(Command::ShowAuthority, &none, Format::Human).0,
            "authority state: uninitialized\n"
        );
        assert_eq!(
            render(Command::ShowAuthority, &none, Format::Json).0,
            "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"uninitialized\",\"authority_id\":null,\"findings\":[]}\n"
        );
    }

    #[test]
    fn refusals_and_errors_follow_the_shapes_of_spec_0001() {
        let refused = Report::Refused(vec![Item::plain(
            "bootstrap.store_present",
            "bootstrap is not possible in this state",
        )]);
        assert_eq!(
            render(Command::Bootstrap, &refused, Format::Human).0,
            "bootstrap refused\nfindings: 1\nbootstrap.store_present: bootstrap is not possible in this state\n"
        );
        assert_eq!(
            render(Command::Bootstrap, &refused, Format::Json).0,
            "{\"output_version\":1,\"command\":\"bootstrap\",\"result\":\"refused\",\"config_version\":null,\"capabilities\":null,\"findings\":[{\"code\":\"bootstrap.store_present\",\"key\":null,\"line\":null,\"column\":null,\"message\":\"bootstrap is not possible in this state\"}]}\n"
        );
        let error = Report::Error(Item::at_key(
            "secret.permissions",
            "principal.verifier_file",
            "a secret file has the wrong type, mode or owner",
        ));
        assert_eq!(
            render(Command::Bootstrap, &error, Format::Human),
            (
                String::new(),
                "error: secret.permissions at principal.verifier_file: a secret file has the wrong type, mode or owner\n".to_owned()
            )
        );
        assert_eq!(
            render(Command::ShowAuthority, &error, Format::Json).0,
            "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"error\",\"config_version\":null,\"capabilities\":null,\"findings\":[{\"code\":\"secret.permissions\",\"key\":\"principal.verifier_file\",\"line\":null,\"column\":null,\"message\":\"a secret file has the wrong type, mode or owner\"}]}\n"
        );
        assert_eq!(error.exit_status(), 3);
        assert_eq!(refused.exit_status(), 1);
    }
}
