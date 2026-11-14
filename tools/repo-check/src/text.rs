//! Line based readers shared by the rules: baseline files, manifest tables and fenced code blocks.

/// Returns the meaningful lines of a baseline file: trimmed, without empty lines and `#` comments.
pub fn baseline_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// One line of a manifest, with the table it belongs to.
pub struct TomlLine<'a> {
    /// The name of the enclosing table header, such as `workspace.lints.rust`, or the empty string before the first header.
    pub table: String,
    /// The trimmed line.
    pub text: &'a str,
}

/// Splits a manifest into its non empty, non comment lines, each tagged with its table. Header lines themselves are not returned.
pub fn toml_lines(text: &str) -> Vec<TomlLine<'_>> {
    let mut table = String::new();
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = table_header(line) {
            table = name;
            continue;
        }
        out.push(TomlLine {
            table: table.clone(),
            text: line,
        });
    }
    out
}

/// Returns the table name of a header line `[name]` or `[[name]]`, without spaces around the name.
pub fn table_header(line: &str) -> Option<String> {
    let inner = line
        .strip_prefix("[[")
        .and_then(|rest| rest.strip_suffix("]]"))
        .or_else(|| {
            line.strip_prefix('[')
                .and_then(|rest| rest.strip_suffix(']'))
        })?;
    let name: String = inner
        .split('.')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(".");
    if name.is_empty() { None } else { Some(name) }
}

/// Splits `key = value` into a trimmed key and a trimmed value.
pub fn key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once('=')?;
    Some((key.trim(), value.trim()))
}

/// Returns, for each line of a Markdown file, whether it belongs to a fenced code block. Fence lines count as code.
pub fn code_lines(lines: &[&str]) -> Vec<bool> {
    let mut out = Vec::with_capacity(lines.len());
    let mut open: Option<(char, usize)> = None;
    for line in lines {
        match open {
            None => {
                if let Some(fence) = fence_open(line) {
                    open = Some(fence);
                }
                out.push(open.is_some());
            }
            Some((mark, len)) => {
                if fence_closes(line, mark, len) {
                    open = None;
                }
                out.push(true);
            }
        }
    }
    out
}

/// Returns the fence character and its length when `line` opens a fenced code block.
pub fn fence_open(line: &str) -> Option<(char, usize)> {
    let rest = line.trim_start_matches(' ');
    let mark = rest.chars().next()?;
    if mark != '`' && mark != '~' {
        return None;
    }
    let len = rest.chars().take_while(|c| *c == mark).count();
    (len >= 3).then_some((mark, len))
}

/// Reports whether `line` closes a fence opened with `len` copies of `mark`.
pub fn fence_closes(line: &str, mark: char, len: usize) -> bool {
    let rest = line.trim();
    rest.chars().count() >= len && rest.chars().all(|c| c == mark)
}

/// Removes text between backticks on one line. An unmatched backtick and the text after it are kept.
pub fn strip_code_spans(line: &str) -> String {
    let parts: Vec<&str> = line.split('`').collect();
    let mut out = String::new();
    let closed = if parts.len() % 2 == 1 {
        parts.len()
    } else {
        parts.len() - 1
    };
    for (index, part) in parts.iter().enumerate() {
        if index >= closed {
            out.push('`');
            out.push_str(part);
        } else if index % 2 == 0 {
            out.push_str(part);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_skips_comments_and_blank_lines() {
        assert_eq!(
            baseline_lines("# spec 0002\n\n a \n#b\nc\n"),
            vec!["a", "c"]
        );
    }

    #[test]
    fn fences_nest_by_length() {
        let lines = ["text", "````text", "```toml", "x", "```", "````", "after"];
        assert_eq!(
            code_lines(&lines),
            vec![false, true, true, true, true, true, false]
        );
    }

    #[test]
    fn tilde_fence_is_not_closed_by_backticks() {
        let lines = ["~~~", "```", "~~~", "out"];
        assert_eq!(code_lines(&lines), vec![true, true, true, false]);
    }

    #[test]
    fn code_spans_are_removed() {
        assert_eq!(strip_code_spans("a `b` c `d"), "a  c `d");
    }

    #[test]
    fn headers_are_normalized() {
        assert_eq!(
            table_header("[ workspace . lints.rust ]").as_deref(),
            Some("workspace.lints.rust")
        );
        assert_eq!(table_header("[[package]]").as_deref(), Some("package"));
        assert_eq!(table_header("x = [1]"), None);
    }
}
