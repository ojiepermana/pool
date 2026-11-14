//! Argument parsing with the standard library only.

use std::ffi::OsString;

/// The output format of `check-config`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Plain text lines.
    Human,
    /// One JSON object on one line.
    Json,
}

/// What the command line asks for.
#[derive(Debug, PartialEq, Eq)]
pub enum Invocation {
    /// Print the help text and exit 0.
    Help,
    /// Print the version and exit 0.
    Version,
    /// Validate one configuration file.
    CheckConfig {
        /// The path exactly as given.
        config: OsString,
        /// The chosen output format.
        format: Format,
    },
    /// A usage error; the message is constant text.
    Usage(&'static str),
}

const NO_COMMAND: &str = "a command is required";
const UNKNOWN_COMMAND: &str = "unknown command";
const UNKNOWN_OPTION: &str = "unknown option";
const UNEXPECTED_ARGUMENT: &str = "unexpected argument";
const REQUIRES_VALUE: &str = "option requires a value";
const GIVEN_TWICE: &str = "option given more than once";
const BAD_FORMAT: &str = "--format must be human or json";
const CONFIG_REQUIRED: &str = "--config is required";

/// Reads the arguments after the program name, once, from left to right.
/// The first usage error ends the run.
pub fn parse(args: impl IntoIterator<Item = OsString>) -> Invocation {
    let mut args = args.into_iter();
    let Some(first) = args.next() else {
        return Invocation::Usage(NO_COMMAND);
    };
    match first.to_str() {
        Some("--help" | "-h") => Invocation::Help,
        Some("--version" | "-V") => Invocation::Version,
        Some("check-config") => parse_check_config(args),
        _ => Invocation::Usage(UNKNOWN_COMMAND),
    }
}

/// The value of an option: the text after the first equals sign, or else the
/// next argument. `None` when there is no value or it is empty.
fn take_value(inline: Option<&str>, args: &mut impl Iterator<Item = OsString>) -> Option<OsString> {
    let value = match inline {
        Some(value) => OsString::from(value),
        None => args.next()?,
    };
    (!value.is_empty()).then_some(value)
}

fn parse_check_config(mut args: impl Iterator<Item = OsString>) -> Invocation {
    let mut config: Option<OsString> = None;
    let mut format: Option<Format> = None;

    while let Some(arg) = args.next() {
        let Some(text) = arg.to_str() else {
            return Invocation::Usage(UNEXPECTED_ARGUMENT);
        };
        let (name, inline) = match text.split_once('=') {
            Some((name, value)) => (name, Some(value)),
            None => (text, None),
        };
        match name {
            "--help" | "-h" => return Invocation::Help,
            "--config" => {
                if config.is_some() {
                    return Invocation::Usage(GIVEN_TWICE);
                }
                let Some(value) = take_value(inline, &mut args) else {
                    return Invocation::Usage(REQUIRES_VALUE);
                };
                config = Some(value);
            }
            "--format" => {
                if format.is_some() {
                    return Invocation::Usage(GIVEN_TWICE);
                }
                let Some(value) = take_value(inline, &mut args) else {
                    return Invocation::Usage(REQUIRES_VALUE);
                };
                format = Some(match value.to_str() {
                    Some("human") => Format::Human,
                    Some("json") => Format::Json,
                    _ => return Invocation::Usage(BAD_FORMAT),
                });
            }
            _ if text.starts_with('-') => return Invocation::Usage(UNKNOWN_OPTION),
            _ => return Invocation::Usage(UNEXPECTED_ARGUMENT),
        }
    }

    match config {
        Some(config) => Invocation::CheckConfig {
            config,
            format: format.unwrap_or(Format::Human),
        },
        None => Invocation::Usage(CONFIG_REQUIRED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(args: &[&str]) -> Invocation {
        parse(args.iter().map(OsString::from))
    }

    #[test]
    fn equals_sign_splits_only_once() {
        assert_eq!(
            run(&["check-config", "--config=a=b", "--format=json"]),
            Invocation::CheckConfig {
                config: OsString::from("a=b"),
                format: Format::Json,
            }
        );
    }

    #[test]
    fn value_may_start_with_a_dash() {
        assert_eq!(
            run(&["check-config", "--config", "--help"]),
            Invocation::CheckConfig {
                config: OsString::from("--help"),
                format: Format::Human,
            }
        );
    }

    #[test]
    fn repeat_is_checked_before_value() {
        assert_eq!(
            run(&["check-config", "--config", "a", "--config"]),
            Invocation::Usage(GIVEN_TWICE)
        );
    }
}
