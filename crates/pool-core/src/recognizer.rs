//! Cleanup eligibility only: no routing, rewriting or rejection of application SQL.
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct Recognizer {
    tables: HashSet<String>,
    prepared: HashSet<String>,
    cursors: HashSet<String>,
}
impl Recognizer {
    pub(crate) fn recognizes(&mut self, query: &[u8]) -> bool {
        if query.len() > 4096 || query.last() != Some(&0) {
            return false;
        }
        let Some(mut t) = tokens(&query[..query.len() - 1]) else {
            return false;
        };
        if t.last().is_some_and(|t| t == ";") {
            t.pop();
        }
        let words: Vec<&str> = t.iter().map(String::as_str).collect();
        let w = words.as_slice();
        if matches!(
            w,
            ["begin"]
                | ["commit"]
                | ["rollback"]
                | ["select", "1"]
                | [
                    "select",
                    "1",
                    "operator",
                    "(",
                    "pg_catalog",
                    ".",
                    "/",
                    ")",
                    "0"
                ]
                | ["select", "pg_catalog", ".", "pg_backend_pid", "(", ")"]
                | [
                    "select",
                    "pg_catalog",
                    ".",
                    "pg_advisory_unlock_all",
                    "(",
                    ")"
                ]
                | ["set", "application_name", "to", "'"]
                | ["set", "time", "zone", "'"]
                | ["set", "search_path", "to", "pg_catalog"]
                | ["reset", "all"]
                | ["show", "application_name"]
                | ["show", "timezone"]
                | ["show", "search_path"]
                | ["unlisten", "*"]
                | ["deallocate", "all"]
                | ["close", "all"]
        ) {
            return true;
        }
        match w {
            [
                "select",
                "pg_catalog",
                ".",
                "pg_advisory_lock" | "pg_advisory_unlock",
                "(",
                n,
                ")",
            ] => integer(n),
            ["listen" | "unlisten", n] | ["notify", n] | ["notify", n, ",", "'"] => name(n),
            [
                "create",
                "temp",
                "table",
                n,
                "(",
                "value",
                "pg_catalog",
                ".",
                "int4",
                ")",
            ] => remember(&mut self.tables, n),
            [
                "insert",
                "into",
                "pg_temp",
                ".",
                n,
                "values",
                "(",
                value,
                ")",
            ] => self.tables.contains(*n) && integer(value) && value.parse::<i32>().is_ok(),
            ["drop", "table", "pg_temp", ".", n]
            | ["select", "value", "from", "pg_temp", ".", n] => self.tables.contains(*n),
            ["prepare", n, "as", "select", "1"] => remember(&mut self.prepared, n),
            ["deallocate", n] => self.prepared.contains(*n),
            ["declare", n, "cursor", "with", "hold", "for", "select", "1"] => {
                remember(&mut self.cursors, n)
            }
            ["fetch", "all", "from", n] | ["close", n] => self.cursors.contains(*n),
            _ => false,
        }
    }
}
fn remember(set: &mut HashSet<String>, n: &str) -> bool {
    name(n) && (set.contains(n) || (set.len() < 256 && set.insert(n.to_owned())))
}
fn name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 63
        && (s.as_bytes()[0].is_ascii_alphabetic() || s.as_bytes()[0] == b'_')
        && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
fn integer(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}
fn tokens(mut s: &[u8]) -> Option<Vec<String>> {
    std::str::from_utf8(s).ok()?;
    let mut out = Vec::new();
    while let Some((&b, tail)) = s.split_first() {
        if b.is_ascii_whitespace() {
            s = tail;
            continue;
        }
        if b == b'\'' {
            s = tail;
            loop {
                let (&next, tail) = s.split_first()?;
                if next == b'\'' {
                    if tail.first() == Some(&b'\'') {
                        s = &tail[1..];
                        continue;
                    }
                    s = tail;
                    break;
                }
                if next.is_ascii_control() || next == b'\\' {
                    return None;
                }
                s = tail;
            }
            out.push("'".to_owned());
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let n = s
                .iter()
                .take_while(|b| b.is_ascii_alphanumeric() || **b == b'_')
                .count();
            out.push(std::str::from_utf8(&s[..n]).ok()?.to_ascii_lowercase());
            s = &s[n..];
        } else if b.is_ascii_digit() || (b == b'-' && tail.first().is_some_and(u8::is_ascii_digit))
        {
            let start = usize::from(b == b'-');
            let n = start + s[start..].iter().take_while(|b| b.is_ascii_digit()).count();
            let ntext = std::str::from_utf8(&s[..n]).ok()?;
            if !integer(ntext) {
                return None;
            }
            out.push(ntext.to_owned());
            s = &s[n..];
        } else if b"().,/*;".contains(&b) {
            out.push(char::from(b).to_string());
            s = tail;
        } else {
            return None;
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grammar_is_closed_and_names_require_known_provenance() {
        let mut r = Recognizer::default();
        for q in [
            "SELECT 1",
            "begin;",
            "SELECT 1 OPERATOR(pg_catalog./) 0",
            "SET application_name TO 'it''s safe'",
            "SET application_name TO 'naïve'",
            "LISTEN abc",
            "CREATE TEMP TABLE abc (value pg_catalog.int4)",
            "INSERT INTO pg_temp.abc VALUES (-1)",
            "SELECT value FROM pg_temp.abc",
            "PREPARE p AS SELECT 1",
            "DEALLOCATE p",
            "DECLARE c CURSOR WITH HOLD FOR SELECT 1",
            "FETCH ALL FROM c",
        ] {
            assert!(r.recognizes(format!("{q}\0").as_bytes()), "{q}");
        }
        for q in [
            "",
            "SELECT 1; SELECT 1",
            "SELECT 1 --x",
            "SELECT /*x*/ 1",
            "SELECT f()",
            "SET application_name TO E'x'",
            "SET application_name TO 'x\\y'",
            "SET application_name TO 'x\ny'",
            "SELECT $$1$$",
            "SELECT value FROM pg_temp.unknown",
            "FETCH ALL FROM unknown",
            "PREPARE p AS SELECT f()",
            "SELECT 1\0",
            "SELECT 9223372036854775808",
            "LISTEN \"abc\"",
            "SELECT 1;;",
        ] {
            assert!(!r.recognizes(format!("{q}\0").as_bytes()), "{q}");
        }
    }
}
