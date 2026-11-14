//! Requirement ID rules D4 to D6 of spec 0002: each ID is defined once, every citation has a definition, and no ID of the base revision disappears.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::docs::{Doc, heading};
use crate::report::Report;
use crate::tree::{Tree, file_name};

/// The family of a requirement ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    /// `SEC-<digits>`.
    Security,
    /// `R-<digits>`.
    Risk,
    /// `F-<digits>`.
    Inventory,
    /// `T-<digits>`.
    Threat,
    /// `PRD-<3 digits> AC-<digits>`.
    PrdCriterion,
    /// `PRD-<3 digits> FR-<digits>`.
    PrdRequirement,
    /// `<4 digits> AC-<digits>`.
    SpecCriterion,
    /// `<4 digits> SC-<digits>`.
    SpecContract,
}

/// The four families defined by a table row of one fixed file: prefix and owner file.
pub const TABLE_FAMILIES: [(Family, &str, &str); 4] = [
    (
        Family::Security,
        "SEC-",
        "docs/security/security-requirements.md",
    ),
    (Family::Risk, "R-", "docs/plan/risk-register.md"),
    (Family::Inventory, "F-", "docs/feature-matrix.md"),
    (Family::Threat, "T-", "docs/security/threat-model.md"),
];

/// A requirement ID: family, owning PRD or spec number (empty for the table families) and number as written.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id {
    /// The family.
    pub family: Family,
    /// The PRD number (three digits) or spec number (four digits), empty for the table families.
    pub owner: String,
    /// The number as written, leading zeros included.
    pub number: String,
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (owner, number) = (&self.owner, &self.number);
        match self.family {
            Family::Security => write!(f, "SEC-{number}"),
            Family::Risk => write!(f, "R-{number}"),
            Family::Inventory => write!(f, "F-{number}"),
            Family::Threat => write!(f, "T-{number}"),
            Family::PrdCriterion => write!(f, "PRD-{owner} AC-{number}"),
            Family::PrdRequirement => write!(f, "PRD-{owner} FR-{number}"),
            Family::SpecCriterion => write!(f, "{owner} AC-{number}"),
            Family::SpecContract => write!(f, "{owner} SC-{number}"),
        }
    }
}

/// Where an ID is defined.
#[derive(Debug, Clone)]
pub struct Site {
    /// The defining file.
    pub path: String,
    /// The 1 based line.
    pub line: usize,
}

/// Every definition found in one tree.
#[derive(Debug, Default)]
pub struct Definitions {
    /// Each ID with every place it is defined.
    pub ids: BTreeMap<Id, Vec<Site>>,
    /// The PRD numbers that exist, with their file.
    pub prds: BTreeMap<String, String>,
    /// The spec numbers that exist, with their file.
    pub specs: BTreeMap<String, String>,
    /// The number of items in each positional list, by family and owner.
    pub lists: BTreeMap<(Family, String), usize>,
}

impl Definitions {
    fn define(&mut self, id: Id, path: &str, line: usize) {
        self.ids.entry(id).or_default().push(Site {
            path: path.to_string(),
            line,
        });
    }
}

fn digits(text: &str) -> usize {
    text.bytes().take_while(u8::is_ascii_digit).count()
}

/// Returns the text after a list marker (`- `, `* `, `+ `, `<n>. ` or `<n>) `), with the number of an ordered item.
fn list_item(line: &str) -> Option<(Option<&str>, &str)> {
    let rest = line.trim_start();
    for marker in ["- ", "* ", "+ "] {
        if let Some(text) = rest.strip_prefix(marker) {
            return Some((None, text));
        }
    }
    let n = digits(rest);
    if n == 0 {
        return None;
    }
    let after = &rest[n..];
    after
        .strip_prefix(". ")
        .or_else(|| after.strip_prefix(") "))
        .map(|text| (Some(&rest[..n]), text))
}

/// Returns the number of a list item that starts with `**AC-<digits>`.
fn criterion(line: &str) -> Option<&str> {
    let (_, text) = list_item(line)?;
    let rest = text.strip_prefix("**AC-")?;
    let n = digits(rest);
    (n > 0).then(|| &rest[..n])
}

/// Returns the numbers written at the start of the top level ordered items under `## <title>`.
fn numbered_items<'a>(doc: &'a Doc, title: &str) -> Vec<(usize, &'a str)> {
    let mut out = Vec::new();
    let mut inside = false;
    for (number, line) in doc.prose() {
        if let Some((level, text)) = heading(line) {
            if level <= 2 {
                inside = level == 2 && text.trim() == title;
            }
            continue;
        }
        if inside
            && !line.starts_with(' ')
            && let Some((Some(n), _)) = list_item(line)
        {
            out.push((number, n));
        }
    }
    out
}

/// Reads every definition of `tree`. With `required`, a missing table owner file is a fault. Without it (the base revision) a missing file only means the base did not have it, but a file or listing that cannot be read is still a fault.
pub fn definitions(tree: &dyn Tree, required: bool, report: &mut Report) -> Definitions {
    let mut defs = Definitions::default();
    for (family, prefix, path) in TABLE_FAMILIES {
        let text = if required {
            report.need(tree, path)
        } else {
            optional_read(tree, path, report)
        };
        let Some(text) = text else {
            continue;
        };
        let doc = Doc::new(path, &text);
        for (number, line) in doc.prose() {
            let Some(row) = line.trim().strip_prefix('|') else {
                continue;
            };
            let cell = row.split('|').next().unwrap_or("").trim();
            if let Some(n) = cell.strip_prefix(prefix)
                && !n.is_empty()
                && digits(n) == n.len()
            {
                let id = Id {
                    family,
                    owner: String::new(),
                    number: n.to_string(),
                };
                defs.define(id, path, number);
            }
        }
    }
    let skip_all = |_: &str| true;
    let prds = if required {
        report.walk(tree, "docs/prd", &skip_all)
    } else {
        optional_walk(tree, "docs/prd", &skip_all, report)
    };
    for path in prds {
        let name = file_name(&path);
        if !(name.ends_with(".md") && digits(name) == 3 && name.as_bytes().get(3) == Some(&b'-')) {
            continue;
        }
        let owner = name[..3].to_string();
        let Some(text) = read(tree, &path, required, report) else {
            continue;
        };
        defs.prds.insert(owner.clone(), path.clone());
        let doc = Doc::new(&path, &text);
        positional(
            &mut defs,
            &doc,
            &owner,
            Family::PrdCriterion,
            Family::PrdRequirement,
            "Functional requirements",
        );
    }
    let specs = if required {
        report.walk(tree, "docs/specs", &|_| false)
    } else {
        optional_walk(tree, "docs/specs", &|_| false, report)
    };
    for path in specs {
        let Some(rest) = path.strip_prefix("docs/specs/") else {
            continue;
        };
        let parts: Vec<&str> = rest.split('/').collect();
        let spec_file = match parts.as_slice() {
            [dir, "index.md"] => Some(*dir),
            [file] if file.ends_with(".md") => Some(*file),
            _ => None,
        };
        let Some(name) = spec_file else {
            continue;
        };
        if !(digits(name) == 4 && name.as_bytes().get(4) == Some(&b'-')) {
            continue;
        }
        let owner = name[..4].to_string();
        let Some(text) = read(tree, &path, required, report) else {
            continue;
        };
        defs.specs.insert(owner.clone(), path.clone());
        let doc = Doc::new(&path, &text);
        positional(
            &mut defs,
            &doc,
            &owner,
            Family::SpecCriterion,
            Family::SpecContract,
            "Safety contracts",
        );
    }
    defs
}

fn read(tree: &dyn Tree, path: &str, required: bool, report: &mut Report) -> Option<String> {
    if required {
        report.need(tree, path)
    } else {
        optional_read(tree, path, report)
    }
}

/// Reads a file that may be absent. Absent is `None` with no fault; a read error is a fault.
fn optional_read(tree: &dyn Tree, path: &str, report: &mut Report) -> Option<String> {
    match tree.read(path) {
        Ok(text) => text,
        Err(e) => {
            report.fault(format!("cannot read the base revision: {e}"));
            None
        }
    }
}

/// Lists a folder that may be absent. A listing error is a fault.
fn optional_walk(
    tree: &dyn Tree,
    dir: &str,
    skip_dir: &dyn Fn(&str) -> bool,
    report: &mut Report,
) -> Vec<String> {
    match tree.walk(dir, skip_dir) {
        Ok(files) => files,
        Err(e) => {
            report.fault(format!("cannot list the base revision: {e}"));
            Vec::new()
        }
    }
}

fn positional(
    defs: &mut Definitions,
    doc: &Doc,
    owner: &str,
    criteria: Family,
    list: Family,
    title: &str,
) {
    for (number, line) in doc.prose() {
        if let Some(n) = criterion(line) {
            let id = Id {
                family: criteria,
                owner: owner.to_string(),
                number: n.to_string(),
            };
            defs.define(id, &doc.path, number);
        }
    }
    let items = numbered_items(doc, title);
    if !items.is_empty() {
        defs.lists.insert((list, owner.to_string()), items.len());
    }
    for (number, n) in items {
        let id = Id {
            family: list,
            owner: owner.to_string(),
            number: n.to_string(),
        };
        defs.define(id, &doc.path, number);
    }
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Returns every citation on one line, as IDs. A citation starts at a word boundary and its number ends at one.
pub fn citations(line: &str) -> Vec<Id> {
    let mut out = Vec::new();
    let mut previous: Option<char> = None;
    for (at, c) in line.char_indices() {
        let boundary = previous.is_none_or(|p| !is_word(p));
        previous = Some(c);
        if !boundary {
            continue;
        }
        if let Some(id) = citation_at(&line[at..]) {
            out.push(id);
        }
    }
    out
}

/// Reads `<digits>` at the start of `text` and checks that a word boundary follows.
fn number_at(text: &str) -> Option<&str> {
    let n = digits(text);
    if n == 0 || text[n..].chars().next().is_some_and(is_word) {
        return None;
    }
    Some(&text[..n])
}

fn citation_at(text: &str) -> Option<Id> {
    let make = |family, owner: &str, number: &str| Id {
        family,
        owner: owner.to_string(),
        number: number.to_string(),
    };
    if let Some(rest) = text.strip_prefix("PRD-") {
        if digits(rest) != 3 {
            return None;
        }
        let owner = &rest[..3];
        let tail = &rest[3..];
        if let Some(n) = tail.strip_prefix(" AC-").and_then(number_at) {
            return Some(make(Family::PrdCriterion, owner, n));
        }
        if let Some(n) = tail.strip_prefix(" FR-").and_then(number_at) {
            return Some(make(Family::PrdRequirement, owner, n));
        }
        return None;
    }
    if digits(text) == 4 {
        let owner = &text[..4];
        let tail = &text[4..];
        if let Some(n) = tail.strip_prefix(" AC-").and_then(number_at) {
            return Some(make(Family::SpecCriterion, owner, n));
        }
        if let Some(n) = tail.strip_prefix(" SC-").and_then(number_at) {
            return Some(make(Family::SpecContract, owner, n));
        }
        return None;
    }
    for (family, prefix, _) in TABLE_FAMILIES {
        if let Some(n) = text.strip_prefix(prefix).and_then(number_at) {
            return Some(make(family, "", n));
        }
    }
    None
}

/// Runs rules D4, D5 and, with a base tree, D6.
pub fn check(tree: &dyn Tree, docs: &[Doc], base: Option<&dyn Tree>, report: &mut Report) {
    let defs = definitions(tree, true, report);
    for (id, sites) in &defs.ids {
        let first = &sites[0];
        for site in &sites[1..] {
            report.add_at(
                "D4",
                &site.path,
                site.line,
                format!(
                    "{id} is defined again; it is first defined at {}:{}",
                    first.path, first.line
                ),
            );
        }
    }
    for doc in docs {
        for (number, line) in doc.prose() {
            for id in citations(line) {
                if defs.ids.contains_key(&id) {
                    continue;
                }
                let message = match id.family {
                    Family::PrdCriterion | Family::PrdRequirement
                        if !defs.prds.contains_key(&id.owner) =>
                    {
                        format!("{id} is cited, but PRD-{} does not exist", id.owner)
                    }
                    Family::SpecCriterion | Family::SpecContract
                        if !defs.specs.contains_key(&id.owner) =>
                    {
                        format!("{id} is cited, but spec {} does not exist", id.owner)
                    }
                    _ => format!("{id} is cited but not defined"),
                };
                report.add_at("D5", &doc.path, number, message);
            }
        }
    }
    let Some(base) = base else {
        return;
    };
    let mut base_report = Report::default();
    let old = definitions(base, false, &mut base_report);
    for fault in base_report.faults {
        report.fault(fault);
    }
    let mut gone = BTreeSet::new();
    for (id, sites) in &old.ids {
        if !defs.ids.contains_key(id) {
            gone.insert(id.clone());
            report.add(
                "D6",
                &sites[0].path,
                format!("{id} is defined at the base revision but no longer defined"),
            );
        }
    }
    for ((family, owner), count) in &old.lists {
        let now = defs
            .lists
            .get(&(*family, owner.clone()))
            .copied()
            .unwrap_or(0);
        if now < *count {
            let path = match family {
                Family::PrdRequirement => old.prds.get(owner),
                _ => old.specs.get(owner),
            };
            let kind = if *family == Family::PrdRequirement {
                format!("PRD-{owner} FR")
            } else {
                format!("{owner} SC")
            };
            report.add(
                "D6",
                path.map_or("", String::as_str),
                format!("the {kind} list has {now} items, the base revision has {count}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs::{self, tests::good};
    use crate::tree::MemTree;

    const PRD: &str = "# PRD-001\n\n## Functional requirements\n\n1. Negotiate.\n2. Terminate.\n   1. Nested item is not counted.\n\n## Acceptance criteria\n\n1. **AC-1:** Works.\n2. **AC-2:** Also.\n\n3. Not a functional requirement.\n";
    const SPEC: &str = "# 0001\n\n## Requirements\n\n- **AC-1** (first): text.\n- **AC-12** (second): text.\n\n## Safety contracts\n\n1. One.\n2. Two.\n\n### Detail\n\n3. Three, still in the list.\n\n## Other\n\n4. Not a contract.\n";

    fn tree() -> MemTree {
        let mut tree = good();
        tree.set("docs/prd/001-core-proxy.md", PRD);
        tree.set("docs/specs/0001-initial-architecture/index.md", SPEC);
        tree.set(
            "docs/specs/0001-initial-architecture/verify.md",
            "# Verify\n\n- **AC-1** is not a definition here.\n",
        );
        tree.set("docs/specs/0007-small.md", "# 0007\n\n- **AC-3** text.\n");
        tree.set(
            "docs/index.md",
            "# Index\n\nCite `PRD-001 AC-1`, PRD-001 FR-2, `0001 SC-3`, 0001 AC-12, 0007 AC-3, `SEC-1`, `F-018`, `R-07`, `T-05`. Not citations: RUSTSEC-0000-0001, AC-3, SEC-1x, 10001 AC-1.\n\n```text\nSEC-99\n```\n",
        );
        tree
    }

    fn run(tree: &MemTree, base: Option<&MemTree>) -> Report {
        docs::check(tree, base.map(|b| b as &dyn Tree))
    }

    fn assert_ok(tree: &MemTree, base: Option<&MemTree>) {
        let report = run(tree, base);
        assert!(
            report.problems.is_empty() && report.faults.is_empty(),
            "{}",
            report.dump()
        );
    }

    fn assert_problem(tree: &MemTree, base: Option<&MemTree>, rule: &str, texts: &[&str]) {
        let report = run(tree, base);
        assert!(
            report.has(rule, texts),
            "expected {rule} {texts:?}, got:\n{}",
            report.dump()
        );
    }

    #[test]
    fn good_tree_passes_with_and_without_base() {
        let tree = tree();
        assert_ok(&tree, None);
        assert_ok(&tree, Some(&tree));
    }

    #[test]
    fn citations_are_parsed() {
        let found: Vec<String> = citations(
            "`SEC-13`, R-16 and 0001 AC-12; PRD-001 FR-2 x0001 AC-1 RUSTSEC-0000-0001 T-1a",
        )
        .iter()
        .map(ToString::to_string)
        .collect();
        assert_eq!(found, vec!["SEC-13", "R-16", "0001 AC-12", "PRD-001 FR-2"]);
    }

    #[test]
    fn positional_lists_follow_the_heading() {
        let mut report = Report::default();
        let defs = definitions(&tree(), true, &mut report);
        let numbers = |family| -> Vec<String> {
            defs.ids
                .keys()
                .filter(|id| id.family == family)
                .map(ToString::to_string)
                .collect()
        };
        assert_eq!(
            numbers(Family::PrdRequirement),
            vec!["PRD-001 FR-1", "PRD-001 FR-2"]
        );
        assert_eq!(
            numbers(Family::SpecContract),
            vec!["0001 SC-1", "0001 SC-2", "0001 SC-3"]
        );
        assert_eq!(
            numbers(Family::PrdCriterion),
            vec!["PRD-001 AC-1", "PRD-001 AC-2"]
        );
    }

    #[test]
    fn d4_duplicate_definition() {
        let mut tree = tree();
        tree.replace(
            "docs/security/security-requirements.md",
            "| SEC-14 |",
            "| SEC-1 |",
        );
        assert_problem(
            &tree,
            None,
            "D4",
            &["docs/security/security-requirements.md:4", "SEC-1 "],
        );
        let mut tree = self::tree();
        tree.replace(
            "docs/specs/0001-initial-architecture/index.md",
            "**AC-12**",
            "**AC-1**",
        );
        assert_problem(&tree, None, "D4", &["0001 AC-1 "]);
    }

    #[test]
    fn d5_cited_but_not_defined() {
        let mut tree = tree();
        let text = tree.get("README.md");
        tree.set("README.md", &format!("{text}`SEC-99`\n"));
        assert_problem(&tree, None, "D5", &["README.md:", "SEC-99"]);
    }

    #[test]
    fn d5_leading_zero_is_another_id() {
        let mut tree = tree();
        tree.set("docs/notes.md", "R-7\n");
        assert_problem(&tree, None, "D5", &["docs/notes.md:1", "R-7"]);
    }

    #[test]
    fn d5_missing_spec_and_prd() {
        let mut tree = tree();
        tree.set("docs/notes.md", "`0004 AC-1` and PRD-011 AC-1\n");
        let report = run(&tree, None);
        assert!(
            report.has("D5", &["docs/notes.md:1", "0004 AC-1", "spec 0004"]),
            "{}",
            report.dump()
        );
        assert!(
            report.has("D5", &["PRD-011 AC-1", "PRD-011 does not exist"]),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn d6_removed_row() {
        let base = tree();
        let mut now = tree();
        now.replace(
            "docs/security/security-requirements.md",
            "| SEC-14 | Review |\n",
            "",
        );
        assert_problem(&now, Some(&base), "D6", &["SEC-14"]);
    }

    #[test]
    fn d6_shorter_list_and_renamed_criterion() {
        let base = tree();
        let mut now = tree();
        now.replace(
            "docs/specs/0001-initial-architecture/index.md",
            "3. Three, still in the list.\n",
            "",
        );
        assert_problem(&now, Some(&base), "D6", &["0001 SC-3"]);
        assert_problem(&now, Some(&base), "D6", &["0001 SC list has 2 items"]);
        let mut now = tree();
        now.replace("docs/prd/001-core-proxy.md", "**AC-2:**", "**AC-3:**");
        assert_problem(&now, Some(&base), "D6", &["PRD-001 AC-2"]);
    }

    #[test]
    fn d6_unreadable_base_file_is_a_fault() {
        let mut base = tree();
        base.set_unreadable("docs/security/security-requirements.md");
        let report = run(&tree(), Some(&base));
        assert!(
            report
                .faults
                .iter()
                .any(|f| f.contains("base revision") && f.contains("security-requirements.md")),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn d6_base_without_an_owner_file_is_not_a_fault() {
        let mut base = tree();
        base.remove("docs/feature-matrix.md");
        assert_ok(&tree(), Some(&base));
    }

    #[test]
    fn d6_new_ids_are_fine() {
        let base = tree();
        let mut now = tree();
        now.replace(
            "docs/security/security-requirements.md",
            "| SEC-14 | Review |\n",
            "| SEC-14 | Review |\n| SEC-15 | New |\n",
        );
        assert_ok(&now, Some(&base));
    }
}
