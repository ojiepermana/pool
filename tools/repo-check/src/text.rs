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

/// Splits a manifest into its non empty, non comment lines, each tagged with its table. A comment after a value or a header is cut first. Header lines themselves are not returned.
pub fn toml_lines(text: &str) -> Vec<TomlLine<'_>> {
    let mut table = String::new();
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = strip_comment(raw).trim();
        if line.is_empty() {
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

/// Cuts a TOML comment: the first `#` that is not inside a basic (`"..."`) or literal (`'...'`) string, and everything after it.
pub fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (index, c) in line.char_indices() {
        match quote {
            Some('"') if escaped => escaped = false,
            Some('"') if c == '\\' => escaped = true,
            Some(q) if c == q => quote = None,
            Some(_) => {}
            None if c == '"' || c == '\'' => quote = Some(c),
            None if c == '#' => return &line[..index],
            None => {}
        }
    }
    line
}

/// Splits a dotted TOML key, such as `target.'cfg(unix)'.dependencies.pool`, into its parts, with spaces around each part and quotes removed. A dot inside quotes does not split.
pub fn dotted_parts(key: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    for c in key.chars() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => current.push(c),
            None if c == '"' || c == '\'' => quote = Some(c),
            None if c == '.' => parts.push(std::mem::take(&mut current).trim().to_string()),
            None => current.push(c),
        }
    }
    parts.push(current.trim().to_string());
    parts
}

/// Returns the parts of the table name of a header line `[name]` or `[[name]]`. A trailing comment must already be cut.
pub fn table_parts(line: &str) -> Option<Vec<String>> {
    let line = line.trim();
    let inner = line
        .strip_prefix("[[")
        .and_then(|rest| rest.strip_suffix("]]"))
        .or_else(|| {
            line.strip_prefix('[')
                .and_then(|rest| rest.strip_suffix(']'))
        })?;
    let parts = dotted_parts(inner);
    if parts.iter().any(String::is_empty) {
        None
    } else {
        Some(parts)
    }
}

/// Returns the table name of a header line `[name]` or `[[name]]`, its parts joined by dots without quotes or spaces. A trailing comment must already be cut.
pub fn table_header(line: &str) -> Option<String> {
    table_parts(line).map(|parts| parts.join("."))
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

/// Removes code spans on one line: a run of backticks opens a span and the next run of the same length closes it, and both runs go with the text between them. A run with no partner of the same length stays as plain text.
pub fn strip_code_spans(line: &str) -> String {
    let runs = backtick_runs(line);
    let mut out = String::new();
    let mut copied = 0;
    let mut next = 0;
    while next < runs.len() {
        let (start, len) = runs[next];
        match runs[next + 1..].iter().position(|&(_, other)| other == len) {
            Some(offset) => {
                let close = next + 1 + offset;
                let (close_start, close_len) = runs[close];
                out.push_str(&line[copied..start]);
                copied = close_start + close_len;
                next = close + 1;
            }
            None => next += 1,
        }
    }
    out.push_str(&line[copied..]);
    out
}

/// Returns the start and length in bytes of every run of backticks on a line.
fn backtick_runs(line: &str) -> Vec<(usize, usize)> {
    let bytes = line.as_bytes();
    let mut runs = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'`' {
            let start = index;
            while index < bytes.len() && bytes[index] == b'`' {
                index += 1;
            }
            runs.push((start, index - start));
        } else {
            index += 1;
        }
    }
    runs
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
        assert_eq!(strip_code_spans("x `a` y `b` z"), "x  y  z");
        assert_eq!(strip_code_spans("plain"), "plain");
    }

    #[test]
    fn double_backtick_spans_are_removed_whole() {
        assert_eq!(strip_code_spans("a ``[l](docs/nope.md)`` b"), "a  b");
        assert_eq!(strip_code_spans("``a ` b`` c"), " c");
        assert_eq!(strip_code_spans("`a `` b` c"), " c");
    }

    #[test]
    fn unpaired_runs_stay_plain_text() {
        assert_eq!(strip_code_spans("a `` b ` c"), "a `` b ` c");
        assert_eq!(strip_code_spans("x ``` [l](t.md)"), "x ``` [l](t.md)");
        assert_eq!(strip_code_spans("`a` ``b"), " ``b");
    }

    #[test]
    fn comments_are_cut_outside_strings() {
        assert_eq!(strip_comment("[dependencies] # runtime"), "[dependencies] ");
        assert_eq!(
            strip_comment("workspace = true # inherit"),
            "workspace = true "
        );
        assert_eq!(strip_comment(r#"a = "x # y" # z"#), r#"a = "x # y" "#);
        assert_eq!(strip_comment("a = 'x # y'"), "a = 'x # y'");
        assert_eq!(
            strip_comment(r#"a = "q\" # in" # out"#),
            r#"a = "q\" # in" "#
        );
    }

    #[test]
    fn dotted_keys_keep_quoted_dots() {
        assert_eq!(
            table_parts("[target.'cfg(unix)'.dependencies.pool]"),
            Some(vec![
                "target".to_string(),
                "cfg(unix)".to_string(),
                "dependencies".to_string(),
                "pool".to_string()
            ])
        );
        assert_eq!(
            table_parts(r#"[ "a.b" . c ]"#),
            Some(vec!["a.b".to_string(), "c".to_string()])
        );
        assert_eq!(table_parts("[a..b]"), None);
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
