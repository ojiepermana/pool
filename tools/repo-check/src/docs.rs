//! Document checks D1 to D3 of spec 0002: links, anchors and scope status. The requirement ID rules D4 to D6 are in [`crate::ids`].

use std::collections::{BTreeMap, BTreeSet};

use crate::ids;
use crate::report::Report;
use crate::text::{code_lines, strip_code_spans};
use crate::tree::{Tree, file_name, join, parent};

/// The scope index that rule D3 reads.
pub const SCOPE_INDEX: &str = "docs/scope/index.md";

/// The statuses a scope feature may have.
pub const STATUSES: [&str; 4] = ["planned", "in-progress", "done", "dropped"];

/// The tags a feature heading may carry after its status.
pub const TAGS: [&str; 2] = ["needs a decision", "deferred"];

/// A Markdown file split into lines, with the lines that belong to fenced code blocks marked.
pub struct Doc {
    /// The path relative to the root.
    pub path: String,
    /// The lines of the file.
    pub lines: Vec<String>,
    /// For each line, whether it is inside a fenced code block.
    pub code: Vec<bool>,
}

impl Doc {
    /// Splits `text` into lines and marks the code lines.
    pub fn new(path: &str, text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(str::to_string).collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let code = code_lines(&refs);
        Self {
            path: path.to_string(),
            lines,
            code,
        }
    }

    /// Returns the lines outside code blocks, with their 1 based numbers.
    pub fn prose(&self) -> impl Iterator<Item = (usize, &str)> {
        self.lines
            .iter()
            .zip(&self.code)
            .enumerate()
            .filter(|(_, (_, code))| !**code)
            .map(|(index, (line, _))| (index + 1, line.as_str()))
    }
}

/// Reports whether the document checks enter a directory with this name.
pub fn skip_dir(name: &str) -> bool {
    (name.starts_with('.') && name != ".github")
        || matches!(name, "target" | "graphify-out" | "node_modules")
}

/// Runs rules D1 to D6. `base` is the tree of the base revision, when one is given.
pub fn check(tree: &dyn Tree, base: Option<&dyn Tree>) -> Report {
    let mut report = Report::default();
    let docs = load(tree, &mut report);
    let mut anchors = Anchors::default();
    for doc in &docs {
        links(tree, doc, &mut anchors, &mut report);
    }
    scope(tree, &mut report);
    ids::check(tree, &docs, base, &mut report);
    report
}

fn load(tree: &dyn Tree, report: &mut Report) -> Vec<Doc> {
    let paths = report.walk(tree, "", &skip_dir);
    paths
        .into_iter()
        .filter(|path| path.ends_with(".md"))
        .filter_map(|path| report.need(tree, &path).map(|text| Doc::new(&path, &text)))
        .collect()
}

/// A link found on a line: its target as written.
#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    /// The target as written, without angle brackets and title.
    pub target: String,
}

/// Returns the link targets of one prose line: inline links, images and reference definitions. Code spans are removed first.
pub fn find_links(line: &str) -> Vec<Link> {
    let line = strip_code_spans(line);
    let mut links = Vec::new();
    if let Some(target) = reference_definition(&line) {
        links.push(Link { target });
        return links;
    }
    let mut rest = line.as_str();
    while let Some(at) = rest.find("](") {
        let after = &rest[at + 2..];
        let (target, used) = inline_target(after);
        if let Some(target) = target {
            links.push(Link { target });
        }
        rest = &after[used..];
    }
    links
}

fn reference_definition(line: &str) -> Option<String> {
    let indent = line.len() - line.trim_start_matches(' ').len();
    if indent > 3 {
        return None;
    }
    let rest = line.trim_start_matches(' ').strip_prefix('[')?;
    let close = rest.find(']')?;
    if close == 0 || rest[..close].starts_with('^') {
        return None;
    }
    let after = rest[close + 1..].strip_prefix(':')?;
    let target = after.split_whitespace().next()?;
    let target = target
        .strip_prefix('<')
        .and_then(|t| t.strip_suffix('>'))
        .unwrap_or(target);
    Some(target.to_string())
}

/// Reads the target that follows `](`. Returns it, when the link is closed, and the number of bytes used.
fn inline_target(after: &str) -> (Option<String>, usize) {
    let text = after.trim_start_matches(' ');
    let skipped = after.len() - text.len();
    if let Some(inner) = text.strip_prefix('<') {
        return match inner.find('>') {
            Some(end) => (Some(inner[..end].to_string()), skipped + end + 2),
            None => (None, skipped),
        };
    }
    let mut depth = 0usize;
    for (index, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' if depth == 0 => return (Some(text[..index].to_string()), skipped + index + 1),
            ')' => depth -= 1,
            c if c.is_whitespace() && depth == 0 => {
                let target = text[..index].to_string();
                let tail = &text[index..];
                let close = tail.find(')').map_or(tail.len(), |i| i + 1);
                return (Some(target), skipped + index + close);
            }
            _ => {}
        }
    }
    (None, skipped + text.len())
}

/// Reports whether a target starts with a URI scheme such as `https:` or `mailto:`.
pub fn has_scheme(target: &str) -> bool {
    let Some((scheme, _)) = target.split_once(':') else {
        return false;
    };
    let mut chars = scheme.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
}

/// Resolves `path` relative to the directory `dir`. Returns `None` when it leaves the root.
pub fn resolve(dir: &str, path: &str) -> Option<String> {
    let mut parts: Vec<&str> = dir.split('/').filter(|p| !p.is_empty()).collect();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            other => parts.push(other),
        }
    }
    Some(parts.join("/"))
}

/// Returns the anchor of a heading text: links replaced by their text, lowercase, only letters, digits, spaces, hyphens and underscores kept, spaces turned into hyphens.
pub fn slug(heading: &str) -> String {
    let mut text = String::new();
    let mut rest = heading;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        let Some(close) = after.find("](") else {
            break;
        };
        let (target, used) = inline_target(&after[close + 2..]);
        if target.is_none() {
            break;
        }
        text.push_str(&rest[..open]);
        text.push_str(&after[..close]);
        rest = &after[close + 2 + used..];
    }
    text.push_str(rest);
    text.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_'))
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// Returns the heading text of a line of one to six `#` and a space.
pub fn heading(line: &str) -> Option<(usize, &str)> {
    let level = line.chars().take_while(|c| *c == '#').count();
    if !(1..=6).contains(&level) {
        return None;
    }
    let rest = &line[level..];
    rest.strip_prefix(' ').map(|text| (level, text))
}

/// Returns every anchor of a document, with `-1`, `-2` and so on added to repeated headings.
pub fn doc_anchors(doc: &Doc) -> BTreeSet<String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut anchors = BTreeSet::new();
    for (_, line) in doc.prose() {
        let Some((_, text)) = heading(line) else {
            continue;
        };
        let base = slug(text);
        let count = seen.entry(base.clone()).or_default();
        if *count == 0 {
            anchors.insert(base);
        } else {
            anchors.insert(format!("{base}-{count}"));
        }
        *count += 1;
    }
    anchors
}

#[derive(Default)]
struct Anchors {
    cache: BTreeMap<String, Option<BTreeSet<String>>>,
}

impl Anchors {
    fn of(&mut self, tree: &dyn Tree, path: &str) -> Option<&BTreeSet<String>> {
        self.cache
            .entry(path.to_string())
            .or_insert_with(|| {
                tree.read(path)
                    .ok()
                    .flatten()
                    .map(|text| doc_anchors(&Doc::new(path, &text)))
            })
            .as_ref()
    }
}

fn links(tree: &dyn Tree, doc: &Doc, anchors: &mut Anchors, report: &mut Report) {
    let dir = parent(&doc.path);
    for (number, line) in doc.prose() {
        for link in find_links(line) {
            let written = link.target.as_str();
            if has_scheme(written) {
                continue;
            }
            let decoded = written.replace("%20", " ");
            let (path, fragment) = decoded.split_once('#').unwrap_or((&decoded, ""));
            let target = if path.is_empty() {
                doc.path.clone()
            } else if path.starts_with('/') {
                report.add_at(
                    "D1",
                    &doc.path,
                    number,
                    format!("link target {written} is an absolute path; use a relative path"),
                );
                continue;
            } else {
                match resolve(dir, path) {
                    Some(resolved) => resolved,
                    None => {
                        report.add_at(
                            "D1",
                            &doc.path,
                            number,
                            format!("link target {written} leaves the repository"),
                        );
                        continue;
                    }
                }
            };
            if !tree.exists(&target) {
                report.add_at(
                    "D1",
                    &doc.path,
                    number,
                    format!("link target {written} does not exist"),
                );
                continue;
            }
            if fragment.is_empty() || !target.ends_with(".md") {
                continue;
            }
            match anchors.of(tree, &target) {
                Some(set) if set.contains(fragment) => {}
                Some(_) => report.add_at(
                    "D2",
                    &doc.path,
                    number,
                    format!("fragment {fragment} is not a heading of {target}"),
                ),
                None => report.fault(format!("{target}: cannot read the link target")),
            }
        }
    }
}

/// A feature row of the scope index.
struct Row {
    line: usize,
    number: String,
    name: String,
    file: String,
    status: String,
}

fn rows(doc: &Doc) -> Vec<Row> {
    let mut rows = Vec::new();
    for (number, line) in doc.prose() {
        let line = line.trim();
        let Some(inner) = line.strip_prefix('|').and_then(|l| l.strip_suffix('|')) else {
            continue;
        };
        let cells: Vec<&str> = inner.split('|').map(str::trim).collect();
        let [n, link, _phase, status] = cells.as_slice() else {
            continue;
        };
        if n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let Some((name, file)) = link
            .strip_prefix('[')
            .and_then(|l| l.split_once("]("))
            .and_then(|(name, rest)| rest.strip_suffix(')').map(|file| (name, file)))
        else {
            continue;
        };
        rows.push(Row {
            line: number,
            number: n.to_string(),
            name: name.to_string(),
            file: file.to_string(),
            status: status.to_string(),
        });
    }
    rows
}

/// Reports whether a name in `docs/scope/` is a feature file: three digits, a hyphen, lowercase letters, digits and hyphens, then `.md`.
pub fn is_feature_file(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    let bytes = stem.as_bytes();
    bytes.len() > 4
        && bytes[..3].iter().all(u8::is_ascii_digit)
        && bytes[3] == b'-'
        && bytes[4..]
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-')
}

fn scope(tree: &dyn Tree, report: &mut Report) {
    let Some(text) = report.need(tree, SCOPE_INDEX) else {
        return;
    };
    let index = Doc::new(SCOPE_INDEX, &text);
    let rows = rows(&index);
    let mut numbers: BTreeMap<&str, usize> = BTreeMap::new();
    let mut files: BTreeMap<&str, usize> = BTreeMap::new();
    for row in &rows {
        *numbers.entry(row.number.as_str()).or_default() += 1;
        *files.entry(row.file.as_str()).or_default() += 1;
        if !STATUSES.contains(&row.status.as_str()) {
            report.add_at(
                "D3",
                SCOPE_INDEX,
                row.line,
                format!(
                    "status {} of feature {} is not one of planned, in-progress, done, dropped",
                    row.status, row.number
                ),
            );
        }
        feature_heading(tree, row, report);
    }
    for (number, count) in numbers {
        if count > 1 {
            report.add(
                "D3",
                SCOPE_INDEX,
                format!("number {number} appears in {count} rows"),
            );
        }
    }
    let listed = report.walk(tree, "docs/scope", &|_| true);
    for path in listed {
        let name = file_name(&path);
        if !is_feature_file(name) {
            continue;
        }
        match files.get(name).copied().unwrap_or(0) {
            1 => {}
            0 => report.add(
                "D3",
                &path,
                "the feature file has no row in docs/scope/index.md",
            ),
            n => report.add(
                "D3",
                &path,
                format!("the feature file has {n} rows in docs/scope/index.md"),
            ),
        }
    }
}

fn feature_heading(tree: &dyn Tree, row: &Row, report: &mut Report) {
    let path = join("docs/scope", &row.file);
    let Ok(n) = row.number.parse::<u32>() else {
        return;
    };
    if row.file.contains('/') || !tree.exists(&path) {
        report.add_at(
            "D3",
            SCOPE_INDEX,
            row.line,
            format!(
                "feature file {} of feature {} does not exist in docs/scope/",
                row.file, row.number
            ),
        );
        return;
    }
    if !row.file.starts_with(&format!("{n:03}-")) {
        report.add(
            "D3",
            &path,
            format!(
                "number: the file name does not start with {n:03}- for feature {}",
                row.number
            ),
        );
    }
    let Some(text) = report.need(tree, &path) else {
        return;
    };
    let first = text.lines().next().unwrap_or("");
    let Some((number, rest)) = first.strip_prefix("# ").and_then(|l| l.split_once(". ")) else {
        report.add_at(
            "D3",
            &path,
            1,
            "the first line is not # <n>. <name> · <status>",
        );
        return;
    };
    let mut parts = rest.split(" · ");
    let name = parts.next().unwrap_or("");
    let status = parts.next().unwrap_or("");
    if number != row.number {
        report.add_at(
            "D3",
            &path,
            1,
            format!(
                "number disagrees: {number} in the heading, {} in docs/scope/index.md",
                row.number
            ),
        );
    }
    if name != row.name {
        report.add_at(
            "D3",
            &path,
            1,
            format!(
                "name disagrees: {name} in the heading, {} in docs/scope/index.md",
                row.name
            ),
        );
    }
    if status != row.status {
        report.add_at(
            "D3",
            &path,
            1,
            format!(
                "status disagrees: {status} in the heading, {} in docs/scope/index.md",
                row.status
            ),
        );
    } else if !STATUSES.contains(&status) {
        report.add_at(
            "D3",
            &path,
            1,
            format!("status {status} is not one of planned, in-progress, done, dropped"),
        );
    }
    for tag in parts {
        if !TAGS.contains(&tag) {
            report.add_at(
                "D3",
                &path,
                1,
                format!("tag {tag} is not one of needs a decision, deferred"),
            );
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tree::MemTree;

    const INDEX: &str = "# Scope\n\n| # | Feature | Phase | Status |\n|---|---|---|---|\n| 1 | [Stack and workspace](001-stack-and-workspace.md) | Foundation | done |\n| 3 | [Control and session data model](003-control-and-session-data-model.md) | Foundation | planned |\n";

    pub(crate) fn good() -> MemTree {
        MemTree::of(&[
            (SCOPE_INDEX, INDEX),
            (
                "docs/scope/001-stack-and-workspace.md",
                "# 1. Stack and workspace · done\n",
            ),
            (
                "docs/scope/003-control-and-session-data-model.md",
                "# 3. Control and session data model · planned · needs a decision\n",
            ),
            ("docs/scope/coordination/001-notes.md", "# notes\n"),
            (
                "docs/security/security-requirements.md",
                "| ID | Text |\n|---|---|\n| SEC-1 | TLS |\n| SEC-14 | Review |\n",
            ),
            (
                "docs/plan/risk-register.md",
                "| ID |\n|---|\n| R-07 | x |\n",
            ),
            ("docs/feature-matrix.md", "| ID |\n|---|\n| F-018 | x |\n"),
            (
                "docs/security/threat-model.md",
                "| ID |\n|---|\n| T-05 | x |\n",
            ),
            (
                "docs/glossary.md",
                "# Glossary\n\n## Lease\n\nText.\n\n## Where a change belongs\n\n## docs/prd\n\n## Lease\n\n## [Linked](../README.md) heading\n",
            ),
            (
                "README.md",
                "# POOL\n\nSee the [glossary](docs/glossary.md#lease), [again](docs/glossary.md#lease-1), [where](docs/glossary.md#where-a-change-belongs), [prd](docs/glossary.md#docsprd), [linked](docs/glossary.md#linked-heading), [dir](docs/scope/), [self](#pool), [web](https://example.org/x.md#nothing), [mail](mailto:a@example.org) and `[code](docs/nothing.md)`.\n\n[ref]: <docs/glossary.md#lease>\n\n```text\n[broken](docs/missing-file.md)\n```\n\n![image](docs/scope/index.md \"title\")\n",
            ),
        ])
    }

    fn assert_ok(tree: &MemTree) {
        let report = check(tree, None);
        assert!(
            report.problems.is_empty() && report.faults.is_empty(),
            "{}",
            report.dump()
        );
    }

    fn assert_problem(tree: &MemTree, rule: &str, texts: &[&str]) {
        let report = check(tree, None);
        assert!(
            report.has(rule, texts),
            "expected {rule} {texts:?}, got:\n{}",
            report.dump()
        );
    }

    fn append(tree: &mut MemTree, path: &str, line: &str) {
        let text = tree.get(path);
        tree.set(path, &format!("{text}{line}\n"));
    }

    #[test]
    fn good_tree_passes() {
        assert_ok(&good());
    }

    #[test]
    fn slugs_follow_the_spec_examples() {
        assert_eq!(slug("Where a change belongs"), "where-a-change-belongs");
        assert_eq!(slug("docs/prd"), "docsprd");
        assert_eq!(slug("`0001 AC-12`, the check"), "0001-ac-12-the-check");
        assert_eq!(slug("Été und Größe"), "été-und-größe");
    }

    #[test]
    fn links_are_found() {
        let targets: Vec<String> =
            find_links("a [x](b.md#c \"t\") and ![y](<d e.md>) `[z](f.md)` [w](g(h).md)")
                .into_iter()
                .map(|l| l.target)
                .collect();
        assert_eq!(targets, vec!["b.md#c", "d e.md", "g(h).md"]);
        assert!(has_scheme("https://x"));
        assert!(!has_scheme("docs/a.md#b:c"));
    }

    #[test]
    fn links_inside_double_backtick_spans_are_not_links() {
        assert!(find_links("see ``[label](docs/nope.md)`` here").is_empty());
        assert!(find_links("``a ` [x](y.md) b``").is_empty());
        let targets: Vec<String> = find_links("odd `` run [x](docs/nope.md)")
            .into_iter()
            .map(|l| l.target)
            .collect();
        assert_eq!(targets, vec!["docs/nope.md"]);
    }

    #[test]
    fn d1_missing_file() {
        let mut tree = good();
        append(&mut tree, "README.md", "[broken](docs/missing-file.md)");
        assert_problem(&tree, "D1", &["README.md:", "docs/missing-file.md"]);
    }

    #[test]
    fn d1_relative_to_the_linking_file() {
        let mut tree = good();
        append(
            &mut tree,
            "docs/glossary.md",
            "[ok](scope/index.md) [up](../README.md)",
        );
        assert_ok(&tree);
        append(&mut tree, "docs/glossary.md", "[bad](docs/scope/index.md)");
        assert_problem(&tree, "D1", &["docs/glossary.md:", "docs/scope/index.md"]);
    }

    #[test]
    fn d1_absolute_and_escaping_paths() {
        let mut tree = good();
        append(&mut tree, "README.md", "[abs](/docs/glossary.md)");
        assert_problem(&tree, "D1", &["/docs/glossary.md", "absolute"]);
        let mut tree = good();
        append(&mut tree, "README.md", "[out](../outside.md)");
        assert_problem(&tree, "D1", &["../outside.md"]);
    }

    #[test]
    fn d1_reference_definition_and_percent_space() {
        let mut tree = good();
        append(&mut tree, "README.md", "[label]: docs/no%20file.md");
        assert_problem(&tree, "D1", &["docs/no%20file.md"]);
    }

    #[test]
    fn d1_skips_dot_directories_but_not_github() {
        let mut tree = good();
        tree.set(".claude/notes.md", "[broken](missing.md)\n");
        assert_ok(&tree);
        tree.set(".github/pull_request_template.md", "[broken](missing.md)\n");
        assert_problem(
            &tree,
            "D1",
            &[".github/pull_request_template.md:1", "missing.md"],
        );
    }

    #[test]
    fn d2_missing_heading() {
        let mut tree = good();
        append(
            &mut tree,
            "README.md",
            "[broken](docs/glossary.md#no-such-heading)",
        );
        assert_problem(&tree, "D2", &["README.md:", "no-such-heading"]);
    }

    #[test]
    fn d2_same_file_fragment_and_code_headings() {
        let mut tree = good();
        append(&mut tree, "README.md", "```\n# Hidden\n```\n[x](#hidden)");
        assert_problem(&tree, "D2", &["README.md:", "hidden"]);
    }

    #[test]
    fn d3_status_disagrees() {
        let mut tree = good();
        tree.replace(
            "docs/scope/003-control-and-session-data-model.md",
            "planned",
            "done",
        );
        assert_problem(
            &tree,
            "D3",
            &["docs/scope/003-control-and-session-data-model.md", "status"],
        );
    }

    #[test]
    fn d3_name_and_number_disagree() {
        let mut tree = good();
        tree.replace(
            "docs/scope/001-stack-and-workspace.md",
            "# 1. Stack",
            "# 2. Stacks",
        );
        let report = check(&tree, None);
        assert!(
            report.has("D3", &["001-stack-and-workspace.md", "number"]),
            "{}",
            report.dump()
        );
        assert!(
            report.has("D3", &["001-stack-and-workspace.md", "name"]),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn d3_unknown_status_and_tag() {
        let mut tree = good();
        tree.replace(SCOPE_INDEX, "| done |", "| shipped |");
        tree.replace("docs/scope/001-stack-and-workspace.md", "done", "shipped");
        assert_problem(&tree, "D3", &[SCOPE_INDEX, "shipped"]);
        let mut tree = good();
        tree.replace(
            "docs/scope/001-stack-and-workspace.md",
            "done",
            "done · urgent",
        );
        assert_problem(&tree, "D3", &["001-stack-and-workspace.md", "urgent"]);
    }

    #[test]
    fn d3_feature_file_without_row() {
        let mut tree = good();
        tree.set(
            "docs/scope/004-one-real-primary-path.md",
            "# 4. One real primary path · planned\n",
        );
        assert_problem(
            &tree,
            "D3",
            &["docs/scope/004-one-real-primary-path.md", "no row"],
        );
    }

    #[test]
    fn d3_duplicate_number_and_missing_file() {
        let mut tree = good();
        append(
            &mut tree,
            SCOPE_INDEX,
            "| 3 | [Other](003-other.md) | Foundation | planned |",
        );
        let report = check(&tree, None);
        assert!(
            report.has("D3", &[SCOPE_INDEX, "number 3"]),
            "{}",
            report.dump()
        );
        assert!(
            report.has("D3", &[SCOPE_INDEX, "003-other.md"]),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn d3_file_name_number() {
        let mut tree = good();
        tree.replace(
            SCOPE_INDEX,
            "| 1 | [Stack and workspace](001-",
            "| 2 | [Stack and workspace](001-",
        );
        tree.replace("docs/scope/001-stack-and-workspace.md", "# 1.", "# 2.");
        assert_problem(&tree, "D3", &["001-stack-and-workspace.md", "002-"]);
    }
}
