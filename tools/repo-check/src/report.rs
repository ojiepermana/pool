//! Problems found by the rules, and the faults that stop a check from running.

use std::fmt;

use crate::tree::Tree;

/// One rule violation, printed as `<rule>: <path>[:<line>]: <message>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// The rule ID, such as `W1` or `D5`.
    pub rule: &'static str,
    /// The file the problem is in, relative to the root.
    pub path: String,
    /// The 1 based line number, when the problem has one.
    pub line: Option<usize>,
    /// What is wrong, in lowercase and without a final period.
    pub message: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}: {}:{}: {}", self.rule, self.path, line, self.message),
            None => write!(f, "{}: {}: {}", self.rule, self.path, self.message),
        }
    }
}

/// The result of one command: its problems and its faults.
#[derive(Debug, Default)]
pub struct Report {
    /// Rule violations, in the order the rules found them.
    pub problems: Vec<Problem>,
    /// Reasons the check could not run completely (a missing or unreadable file, a failed `git`).
    pub faults: Vec<String>,
}

impl Report {
    /// Records a problem without a line number.
    pub fn add(&mut self, rule: &'static str, path: &str, message: impl Into<String>) {
        self.problems.push(Problem {
            rule,
            path: path.to_string(),
            line: None,
            message: message.into(),
        });
    }

    /// Records a problem at a 1 based line number.
    pub fn add_at(
        &mut self,
        rule: &'static str,
        path: &str,
        line: usize,
        message: impl Into<String>,
    ) {
        self.problems.push(Problem {
            rule,
            path: path.to_string(),
            line: Some(line),
            message: message.into(),
        });
    }

    /// Records a fault.
    pub fn fault(&mut self, message: impl Into<String>) {
        self.faults.push(message.into());
    }

    /// Reads a file the check needs. A missing or unreadable file is a fault, and the result is `None`.
    pub fn need(&mut self, tree: &dyn Tree, path: &str) -> Option<String> {
        match tree.read(path) {
            Ok(Some(text)) => Some(text),
            Ok(None) => {
                self.fault(format!("{path}: file is missing"));
                None
            }
            Err(e) => {
                self.fault(format!("cannot read {e}"));
                None
            }
        }
    }

    /// Lists files with [`Tree::walk`]. A failure is a fault, and the result is empty.
    pub fn walk(
        &mut self,
        tree: &dyn Tree,
        dir: &str,
        skip_dir: &dyn Fn(&str) -> bool,
    ) -> Vec<String> {
        match tree.walk(dir, skip_dir) {
            Ok(files) => files,
            Err(e) => {
                self.fault(format!("cannot list {e}"));
                Vec::new()
            }
        }
    }

    /// Reports whether any problem has the given rule ID and contains every given text. Used by tests.
    #[cfg(test)]
    pub fn has(&self, rule: &str, texts: &[&str]) -> bool {
        self.problems.iter().any(|problem| {
            let line = problem.to_string();
            problem.rule == rule && texts.iter().all(|text| line.contains(text))
        })
    }

    /// Returns every problem as printed, one per line. Used by tests in their failure messages.
    #[cfg(test)]
    pub fn dump(&self) -> String {
        let mut lines: Vec<String> = self.problems.iter().map(ToString::to_string).collect();
        lines.extend(self.faults.iter().map(|fault| format!("fault: {fault}")));
        lines.join("\n")
    }
}
