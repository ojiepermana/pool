//! Workspace guards W1 to W10 of spec 0002.

use std::collections::{BTreeMap, BTreeSet};

use crate::report::Report;
use crate::text::{self, baseline_lines, key_value, toml_lines};
use crate::tree::{Tree, file_name, join};

/// The directory that holds the baseline files.
pub const BASELINE: &str = "tools/repo-check/baseline";

/// The four lockfiles and their baseline files (rule W1). Rule W6 refuses every other `Cargo.lock`.
pub const LOCKFILES: [(&str, &str); 4] = [
    ("Cargo.lock", "lock-packages.txt"),
    (
        "crates/pool-config/fuzz/Cargo.lock",
        "fuzz-lock-packages.txt",
    ),
    ("tools/repo-check/Cargo.lock", "tool-lock-packages.txt"),
    (
        "crates/pool-protocol/fuzz/Cargo.lock",
        "protocol-fuzz-lock-packages.txt",
    ),
];

/// The script that holds the copy of the Linux build command (rule W7).
pub const LINUX_BUILD_SCRIPT: &str = "scripts/linux-build.sh";

/// The spec that owns the Linux build command (rule W7).
pub const SPEC_0001: &str = "docs/specs/0001-initial-architecture/index.md";

/// The line after which the script holds the copied command (rule W7).
pub const MARKER: &str = "# --- spec 0001, Linux build ---";

/// Runs rules W1 to W10.
pub fn check(tree: &dyn Tree) -> Report {
    let mut report = Report::default();
    lockfiles(tree, &mut report);
    forbidden_paths(tree, &mut report);
    forbidden_text(tree, &mut report);
    let members = members(tree, &mut report);
    local_lock_packages(tree, &members, &mut report);
    dependencies(tree, &members, &mut report);
    stray_lockfiles(tree, &mut report);
    linux_build_copy(tree, &mut report);
    crate::hardening::check(tree, &mut report);
    report
}

fn baseline(tree: &dyn Tree, name: &str, report: &mut Report) -> Option<Vec<String>> {
    report
        .need(tree, &join(BASELINE, name))
        .map(|text| baseline_lines(&text))
}

/// Returns the package names of a lockfile: the `name` line that follows each `[[package]]` line.
pub fn lock_packages(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut pending = false;
    for line in text.lines() {
        let line = line.trim();
        if line == "[[package]]" {
            pending = true;
        } else if pending && let Some(("name", value)) = key_value(line) {
            names.push(value.trim_matches('"').to_string());
            pending = false;
        }
    }
    names
}

fn lockfiles(tree: &dyn Tree, report: &mut Report) {
    for (lockfile, baseline_name) in LOCKFILES {
        let Some(expected) = baseline(tree, baseline_name, report) else {
            continue;
        };
        let Some(text) = report.need(tree, lockfile) else {
            continue;
        };
        let mut counts: BTreeMap<String, i64> = BTreeMap::new();
        for name in lock_packages(&text) {
            *counts.entry(name).or_default() += 1;
        }
        for name in expected {
            *counts.entry(name).or_default() -= 1;
        }
        for (name, count) in counts {
            if count > 0 {
                report.add(
                    "W1",
                    lockfile,
                    format!(
                        "package {name} is in the lockfile but not in the baseline {baseline_name}"
                    ),
                );
            } else if count < 0 {
                report.add(
                    "W1",
                    lockfile,
                    format!(
                        "package {name} is in the baseline {baseline_name} but not in the lockfile"
                    ),
                );
            }
        }
    }
}

fn forbidden_paths(tree: &dyn Tree, report: &mut Report) {
    let Some(paths) = baseline(tree, "forbidden-paths.txt", report) else {
        return;
    };
    for path in paths {
        let clean = path.trim_end_matches('/');
        if tree.exists(clean) {
            report.add("W2", clean, format!("forbidden path {path} exists"));
        }
    }
}

/// One meaningful line of `forbidden-source-patterns.txt` (spec 0004, "Source guard").
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceRule {
    /// `<text>`: the text is forbidden in every `.rs` file under `apps/` and `crates/`.
    Everywhere(String),
    /// `deny <path> <text>`: the text is also forbidden in every `.rs` file at or under the path.
    Deny(String, String),
    /// `allow <path> <text>`: the text is allowed in every `.rs` file at or under the path.
    Allow(String, String),
}

/// Reads one baseline line of rule W3. Returns `None` for a line that has none of the three forms.
pub fn source_rule(line: &str) -> Option<SourceRule> {
    let words: Vec<&str> = line.split(' ').collect();
    if words.iter().any(|word| word.is_empty()) {
        return None;
    }
    match words.as_slice() {
        [text] if *text != "deny" && *text != "allow" => {
            Some(SourceRule::Everywhere(text.to_string()))
        }
        ["deny", path, text] => Some(SourceRule::Deny(path.to_string(), text.to_string())),
        ["allow", path, text] => Some(SourceRule::Allow(path.to_string(), text.to_string())),
        _ => None,
    }
}

/// Reports whether `path` is `dir` or lies under it, compared by whole path components.
pub fn covers(dir: &str, path: &str) -> bool {
    let dir = dir.trim_end_matches('/');
    path == dir
        || path
            .strip_prefix(dir)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// Returns the texts that rule W3 forbids in the file at `path`, each once, in the order of the baseline.
pub fn forbidden_in(rules: &[SourceRule], path: &str) -> Vec<String> {
    let mut texts: Vec<String> = Vec::new();
    for rule in rules {
        let text = match rule {
            SourceRule::Everywhere(text) => text,
            SourceRule::Deny(dir, text) if covers(dir, path) => text,
            _ => continue,
        };
        let allowed = rules.iter().any(|other| {
            matches!(other, SourceRule::Allow(dir, allowed) if allowed == text && covers(dir, path))
        });
        if !allowed && !texts.contains(text) {
            texts.push(text.clone());
        }
    }
    texts
}

fn forbidden_text(tree: &dyn Tree, report: &mut Report) {
    let name = "forbidden-source-patterns.txt";
    let Some(lines) = baseline(tree, name, report) else {
        return;
    };
    let mut rules = Vec::new();
    for line in lines {
        match source_rule(&line) {
            Some(rule) => rules.push(rule),
            None => report.add(
                "W3",
                &join(BASELINE, name),
                format!("line {line} is not <text>, deny <path> <text> or allow <path> <text>"),
            ),
        }
    }
    for rule in &rules {
        if let SourceRule::Deny(denied, text) = rule {
            for other in &rules {
                if let SourceRule::Allow(allowed, allowed_text) = other
                    && text == allowed_text
                    && covers(allowed, denied)
                {
                    report.add(
                        "W3",
                        &join(BASELINE, name),
                        format!("deny {denied} {text} is hidden by allow {allowed} {text}"),
                    );
                }
            }
        }
    }
    for dir in ["apps", "crates"] {
        for path in report.walk(tree, dir, &|name| name == "target") {
            if !path.ends_with(".rs") {
                continue;
            }
            let texts = forbidden_in(&rules, &path);
            if texts.is_empty() {
                continue;
            }
            let Some(text) = report.need(tree, &path) else {
                continue;
            };
            for (index, line) in text.lines().enumerate() {
                for pattern in &texts {
                    if line.contains(pattern.as_str()) {
                        report.add_at("W3", &path, index + 1, format!("forbidden text {pattern}"));
                    }
                }
            }
        }
    }
}

/// A workspace member named by its manifest.
pub struct Member {
    /// The `name` of its `[package]` table, when it has one.
    pub name: Option<String>,
}

/// Returns the quoted strings of the `members` array of the `[workspace]` table.
pub fn member_dirs(root_manifest: &str) -> Vec<String> {
    let mut dirs = Vec::new();
    let mut inside = false;
    for line in toml_lines(root_manifest) {
        if line.table != "workspace" {
            continue;
        }
        let rest = if inside {
            line.text
        } else if let Some(("members", value)) = key_value(line.text) {
            inside = true;
            value
        } else {
            continue;
        };
        let mut parts = rest.split('"');
        parts.next();
        while let Some(quoted) = parts.next() {
            dirs.push(quoted.to_string());
            parts.next();
        }
        if rest.contains(']') {
            inside = false;
        }
    }
    dirs
}

fn members(tree: &dyn Tree, report: &mut Report) -> Vec<Member> {
    let Some(root) = report.need(tree, "Cargo.toml") else {
        return Vec::new();
    };
    let forbids_unsafe = toml_lines(&root).iter().any(|line| {
        line.table == "workspace.lints.rust"
            && key_value(line.text) == Some(("unsafe_code", "\"forbid\""))
    });
    if !forbids_unsafe {
        report.add(
            "W4",
            "Cargo.toml",
            "[workspace.lints.rust] does not set unsafe_code = \"forbid\"",
        );
    }
    let mut members = Vec::new();
    for dir in member_dirs(&root) {
        if dir.contains('*') {
            report.add(
                "W4",
                "Cargo.toml",
                format!("member {dir} uses a wildcard, which the guard cannot follow"),
            );
            continue;
        }
        let manifest = join(dir.trim_end_matches('/'), "Cargo.toml");
        let Some(text) = report.need(tree, &manifest) else {
            continue;
        };
        let lines = toml_lines(&text);
        let inherits = lines.iter().any(|line| {
            line.table == "lints" && key_value(line.text) == Some(("workspace", "true"))
        });
        if !inherits {
            report.add(
                "W4",
                &manifest,
                "the manifest has no [lints] table with workspace = true",
            );
        }
        let name = lines.iter().find_map(|line| match key_value(line.text) {
            Some(("name", value)) if line.table == "package" => {
                Some(value.trim_matches('"').to_string())
            }
            _ => None,
        });
        members.push(Member { name });
    }
    members
}

/// Returns the package names of the entries of a lockfile that have no `source` line.
///
/// An entry is the lines from one `[[package]]` line to the next line that starts with `[`, or to the end of the
/// file (spec 0004, "Proof script and lockfile check"). So a table that Cargo writes after the packages, such as
/// `[[patch.unused]]`, is never part of the last entry.
pub fn local_packages(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut entry: Option<(Option<String>, bool)> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if let Some((Some(name), false)) = entry.take() {
                names.push(name);
            }
            if line == "[[package]]" {
                entry = Some((None, false));
            }
        } else if let Some((name, has_source)) = entry.as_mut() {
            match key_value(line) {
                Some(("name", value)) if name.is_none() => {
                    *name = Some(value.trim_matches('"').to_string());
                }
                Some(("source", _)) => *has_source = true,
                _ => {}
            }
        }
    }
    if let Some((Some(name), false)) = entry {
        names.push(name);
    }
    names
}

/// Rule W4, added by spec 0003: every package of the root `Cargo.lock` without a `source` line is a listed member.
fn local_lock_packages(tree: &dyn Tree, members: &[Member], report: &mut Report) {
    // A missing or unreadable lockfile is already a fault of rule W1.
    let Ok(Some(text)) = tree.read("Cargo.lock") else {
        return;
    };
    let names: BTreeSet<&str> = members
        .iter()
        .filter_map(|member| member.name.as_deref())
        .collect();
    for name in local_packages(&text) {
        if !names.contains(name.as_str()) {
            report.add(
                "W4",
                "Cargo.lock",
                format!(
                    "package {name} has no source line but is not a member written in members of Cargo.toml"
                ),
            );
        }
    }
}

fn dependencies(tree: &dyn Tree, members: &[Member], report: &mut Report) {
    let Some(allowed) = baseline(tree, "internal-dependencies.txt", report) else {
        return;
    };
    let Some(lock) = report.need(tree, "Cargo.lock") else {
        return;
    };
    let allowed: BTreeSet<String> = allowed.into_iter().collect();
    let names: BTreeSet<&str> = members.iter().filter_map(|m| m.name.as_deref()).collect();
    let mut found = BTreeSet::new();
    for package in crate::hardening::lock_entries(&lock) {
        if !names.contains(package.name.as_str()) {
            continue;
        }
        for to in package.dependencies {
            if !names.contains(to.as_str()) {
                continue;
            }
            let pair = format!("{} -> {to}", package.name);
            let excluded = matches!(
                (package.name.as_str(), to.as_str()),
                ("pool-core", "pool-control") | ("pool-control", "pool-core")
            );
            if excluded || !allowed.contains(&pair) {
                report.add(
                    "W5",
                    "Cargo.lock",
                    format!(
                        "dependency {pair} is not allowed{}",
                        if excluded {
                            " (core/control exclusion cannot be changed by the baseline)"
                        } else {
                            " by internal-dependencies.txt"
                        }
                    ),
                );
            }
            found.insert(pair);
        }
    }
    for pair in allowed.difference(&found) {
        report.add(
            "W5",
            &join(BASELINE, "internal-dependencies.txt"),
            format!("dependency {pair} is listed but missing from Cargo.lock"),
        );
    }
}

fn stray_lockfiles(tree: &dyn Tree, report: &mut Report) {
    let known: BTreeSet<&str> = LOCKFILES.iter().map(|(path, _)| *path).collect();
    let skip = |name: &str| name == "target" || name.starts_with('.');
    for path in report.walk(tree, "", &skip) {
        if file_name(&path) == "Cargo.lock" && !known.contains(path.as_str()) {
            report.add("W6", &path, "unexpected Cargo.lock");
        }
    }
}

/// Returns the lines of the first fenced block after the heading `### Linux build`, or `None` when there is no such heading or block.
pub fn linux_build_block(spec: &str) -> Option<Vec<&str>> {
    let mut lines = spec
        .lines()
        .skip_while(|line| line.trim_end() != "### Linux build");
    lines.next()?;
    let mut fence = None;
    for line in lines.by_ref() {
        if let Some(open) = text::fence_open(line) {
            fence = Some(open);
            break;
        }
    }
    let (mark, len) = fence?;
    let mut block = Vec::new();
    for line in lines {
        if text::fence_closes(line, mark, len) {
            return Some(block);
        }
        block.push(line);
    }
    None
}

fn split_lines(text: &str) -> Vec<&str> {
    let body = text.strip_suffix('\n').unwrap_or(text);
    if body.is_empty() {
        Vec::new()
    } else {
        body.split('\n').collect()
    }
}

fn linux_build_copy(tree: &dyn Tree, report: &mut Report) {
    let spec = report.need(tree, SPEC_0001);
    let script = report.need(tree, LINUX_BUILD_SCRIPT);
    let (Some(spec), Some(script)) = (spec, script) else {
        return;
    };
    let Some(block) = linux_build_block(&spec) else {
        report.add(
            "W7",
            SPEC_0001,
            "no fenced code block follows the heading ### Linux build",
        );
        return;
    };
    let lines = split_lines(&script);
    let Some(marker) = lines.iter().position(|line| *line == MARKER) else {
        report.add(
            "W7",
            LINUX_BUILD_SCRIPT,
            format!("the marker line {MARKER} is missing"),
        );
        return;
    };
    let copy = &lines[marker + 1..];
    let differs = (0..copy.len().max(block.len())).find(|&i| copy.get(i) != block.get(i));
    if let Some(i) = differs {
        report.add_at(
            "W7",
            LINUX_BUILD_SCRIPT,
            marker + 2 + i,
            format!("scripts/linux-build.sh differs from the first code block under ### Linux build in {SPEC_0001}"),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::MemTree;

    const ROOT_MANIFEST: &str = "[workspace]\nresolver = \"3\"\nmembers = [\"apps/pool\", \"crates/pool-config\"]\n\n[workspace.dependencies]\npool-config = { path = \"crates/pool-config\" }\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n";
    const POOL_MANIFEST: &str = "[package]\nname = \"pool\"\nversion.workspace = true\n\n[dependencies]\npool-config.workspace = true\nserde_json.workspace = true\n\n[lints]\nworkspace = true\n";
    const CONFIG_MANIFEST: &str = "[package]\nname = \"pool-config\"\n\n[dependencies]\nserde.workspace = true\n\n[lints]\nworkspace = true\n";
    const SPEC: &str = "# 0001\n\n### Linux build\n\nRun it.\n\n```bash\ngit archive HEAD | docker run x\necho \"linux build check passed\"\n```\n\nMore text.\n\n```bash\nother\n```\n";
    const SCRIPT: &str = "#!/bin/sh\nset -eu\n# --- spec 0001, Linux build ---\ngit archive HEAD | docker run x\necho \"linux build check passed\"\n";

    const REGISTRY: &str = "registry+https://github.com/rust-lang/crates.io-index";

    /// Writes a lockfile. Every name that is not a member of [`ROOT_MANIFEST`] gets a `source` line.
    fn lock(names: &[&str]) -> String {
        names
            .iter()
            .map(|name| {
                let source = if ["pool", "pool-config"].contains(name) {
                    String::new()
                } else {
                    format!("source = \"{REGISTRY}\"\n")
                };
                format!(
                    "[[package]]\nname = \"{name}\"\nversion = \"1.0.0\"\n{source}{}\n",
                    if *name == "pool" {
                        "dependencies = [\n \"pool-config\",\n]\n"
                    } else {
                        ""
                    }
                )
            })
            .collect::<String>()
    }

    fn good() -> MemTree {
        let mut tree = MemTree::of(&[
            ("Cargo.toml", ROOT_MANIFEST),
            ("apps/pool/Cargo.toml", POOL_MANIFEST),
            ("apps/pool/src/main.rs", "fn main() {}\n"),
            ("crates/pool-config/Cargo.toml", CONFIG_MANIFEST),
            ("crates/pool-config/src/lib.rs", "//! config\n"),
            ("crates/pool-config/target/debug/x.rs", "use std::thread;\n"),
            (SPEC_0001, SPEC),
            (LINUX_BUILD_SCRIPT, SCRIPT),
            (
                "tools/repo-check/baseline/lock-packages.txt",
                "# spec 0002\npool\npool-config\nserde\n",
            ),
            (
                "tools/repo-check/baseline/fuzz-lock-packages.txt",
                "# spec 0002\nlibfuzzer-sys\npool-config\npool-config-fuzz\n",
            ),
            (
                "tools/repo-check/baseline/tool-lock-packages.txt",
                "# spec 0002\npool-repo-check\n",
            ),
            (
                "tools/repo-check/baseline/protocol-fuzz-lock-packages.txt",
                "# spec 0004\npool-protocol\n",
            ),
            (
                "crates/pool-protocol/fuzz/Cargo.lock",
                "[[package]]\nname = \"pool-protocol\"\n",
            ),
            (
                "tools/repo-check/baseline/forbidden-paths.txt",
                "# spec 0002\napps/pool-cli\ncrates/pool-core\n",
            ),
            (
                "tools/repo-check/baseline/forbidden-source-patterns.txt",
                "# spec 0002\nstd::net\nstd::thread\ntokio\n",
            ),
            (
                "tools/repo-check/baseline/internal-dependencies.txt",
                "# spec 0002\npool -> pool-config\n",
            ),
            ("target/debug/Cargo.lock", ""),
            (".hidden/Cargo.lock", ""),
        ]);
        tree.set("Cargo.lock", &lock(&["pool", "pool-config", "serde"]));
        tree.set(
            "crates/pool-config/fuzz/Cargo.lock",
            &lock(&["libfuzzer-sys", "pool-config", "pool-config-fuzz"]),
        );
        tree.set("tools/repo-check/Cargo.lock", &lock(&["pool-repo-check"]));
        tree.set(
            ".cargo/config.toml",
            include_str!("../../../.cargo/config.toml"),
        );
        tree.set("NOTICE", "");
        tree.set("tools/repo-check/baseline/native-library-versions.txt", "");
        tree
    }

    fn assert_ok(tree: &MemTree) {
        let report = check(tree);
        assert!(
            report.problems.is_empty() && report.faults.is_empty(),
            "{}",
            report.dump()
        );
    }

    fn assert_problem(tree: &MemTree, rule: &str, texts: &[&str]) {
        let report = check(tree);
        assert!(
            report.has(rule, texts),
            "expected {rule} {texts:?}, got:\n{}",
            report.dump()
        );
    }

    #[test]
    fn good_tree_passes() {
        assert_ok(&good());
    }

    #[test]
    fn w1_extra_package_in_lockfile() {
        let mut tree = good();
        tree.set(
            "Cargo.lock",
            &lock(&["pool", "pool-config", "serde", "zmij"]),
        );
        assert_problem(&tree, "W1", &["Cargo.lock", "zmij", "not in the baseline"]);
    }

    #[test]
    fn w1_package_missing_from_lockfile() {
        let mut tree = good();
        tree.set("tools/repo-check/Cargo.lock", &lock(&[]));
        assert_problem(
            &tree,
            "W1",
            &[
                "tools/repo-check/Cargo.lock",
                "pool-repo-check",
                "not in the lockfile",
            ],
        );
    }

    #[test]
    fn w1_counts_duplicates() {
        let mut tree = good();
        tree.set(
            "crates/pool-config/fuzz/Cargo.lock",
            &lock(&[
                "libfuzzer-sys",
                "pool-config",
                "pool-config",
                "pool-config-fuzz",
            ]),
        );
        assert_problem(
            &tree,
            "W1",
            &["crates/pool-config/fuzz/Cargo.lock", "pool-config"],
        );
    }

    #[test]
    fn w1_missing_lockfile_is_a_fault() {
        let mut tree = good();
        tree.remove("crates/pool-config/fuzz/Cargo.lock");
        let report = check(&tree);
        assert!(
            report
                .faults
                .iter()
                .any(|f| f.contains("crates/pool-config/fuzz/Cargo.lock"))
        );
    }

    #[test]
    fn w2_forbidden_directory_exists() {
        let mut tree = good();
        tree.set("crates/pool-core/.keep", "");
        assert_problem(&tree, "W2", &["crates/pool-core"]);
    }

    #[test]
    fn w2_similar_name_is_not_forbidden() {
        let mut tree = good();
        tree.set("crates/pool-core-notes.md", "");
        assert_ok(&tree);
    }

    #[test]
    fn w3_forbidden_text_in_source() {
        let mut tree = good();
        tree.set("apps/pool/src/main.rs", "fn main() {}\n// std::thread\n");
        assert_problem(&tree, "W3", &["apps/pool/src/main.rs:2", "std::thread"]);
    }

    #[test]
    fn w3_ignores_target_and_other_files() {
        let mut tree = good();
        tree.set("crates/pool-config/README.md", "tokio\n");
        tree.set("docs/notes.rs", "tokio\n");
        assert_ok(&tree);
    }

    #[test]
    fn w4_member_without_lint_inheritance() {
        let mut tree = good();
        tree.replace("apps/pool/Cargo.toml", "[lints]\nworkspace = true\n", "");
        assert_problem(&tree, "W4", &["apps/pool/Cargo.toml"]);
    }

    #[test]
    fn w4_root_without_unsafe_forbid() {
        let mut tree = good();
        tree.replace(
            "Cargo.toml",
            "unsafe_code = \"forbid\"",
            "unsafe_code = \"deny\"",
        );
        assert_problem(&tree, "W4", &["Cargo.toml", "unsafe_code"]);
    }

    #[test]
    fn w4_lockfile_package_without_source_that_is_a_member_passes() {
        // covers: 0003 AC-13
        let mut tree = good();
        assert_eq!(
            local_packages(&lock(&["pool", "pool-config", "serde"])),
            ["pool", "pool-config"]
        );
        tree.set(
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"pool\"\nversion = \"0.1.0\"\ndependencies = [\n \"pool-config\",\n]\n\n[[package]]\nname = \"pool-config\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
        );
        assert_ok(&tree);
    }

    #[test]
    fn w4_lockfile_package_without_source_that_is_not_a_member_fails() {
        // covers: 0003 AC-13
        let mut tree = good();
        tree.set(
            "Cargo.lock",
            &format!(
                "{}[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\n",
                lock(&["pool", "pool-config"])
            ),
        );
        assert_problem(
            &tree,
            "W4",
            &["Cargo.lock", "package serde has no source line"],
        );
    }

    #[test]
    fn w4_wildcard_member() {
        let mut tree = good();
        tree.replace("Cargo.toml", "\"crates/pool-config\"", "\"crates/*\"");
        assert_problem(&tree, "W4", &["Cargo.toml", "crates/*"]);
    }

    #[test]
    fn w4_reads_multiline_members() {
        let mut tree = good();
        tree.replace(
            "Cargo.toml",
            "members = [\"apps/pool\", \"crates/pool-config\"]",
            "members = [\n  \"apps/pool\",\n  \"crates/pool-config\",\n]",
        );
        assert_ok(&tree);
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[lints]\nworkspace = true",
            "[lints]\nworkspace = false",
        );
        assert_problem(&tree, "W4", &["crates/pool-config/Cargo.toml"]);
    }

    #[test]
    fn w5_uses_locked_package_names_and_versions() {
        let mut tree = good();
        tree.replace(
            "Cargo.lock",
            "name = \"pool-config\"",
            "name = \"pool-config\"\ndependencies = [\n \"pool 1.0.0\",\n]",
        );
        assert_problem(
            &tree,
            "W5",
            &["Cargo.lock", "pool-config -> pool", "not allowed"],
        );
    }

    #[test]
    fn w5_core_control_exclusion_overrides_even_an_edited_baseline_both_ways() {
        let members = [
            Member {
                name: Some("pool-core".into()),
            },
            Member {
                name: Some("pool-control".into()),
            },
        ];
        for (from, to) in [("pool-core", "pool-control"), ("pool-control", "pool-core")] {
            let lock = format!(
                "[[package]]\nname = \"{from}\"\ndependencies = [\n \"{to}\",\n]\n[[package]]\nname = \"{to}\"\n"
            );
            let baseline = format!("{from} -> {to}\n");
            let tree = MemTree::of(&[
                ("Cargo.lock", &lock),
                (
                    "tools/repo-check/baseline/internal-dependencies.txt",
                    &baseline,
                ),
            ]);
            let mut report = Report::default();
            dependencies(&tree, &members, &mut report);
            assert!(
                report.has(
                    "W5",
                    &[&format!("{from} -> {to}"), "core/control exclusion"]
                ),
                "{}",
                report.dump()
            );
        }
    }

    #[test]
    fn w5_missing_locked_edge_is_rejected() {
        let mut tree = good();
        tree.replace("Cargo.lock", "dependencies = [\n \"pool-config\",\n]\n", "");
        assert_problem(&tree, "W5", &["pool -> pool-config", "missing"]);
    }

    #[test]
    fn w4_comments_are_not_values() {
        let mut tree = good();
        tree.replace(
            "apps/pool/Cargo.toml",
            "[lints]\nworkspace = true\n",
            "[lints] # inherit\nworkspace = true # inherit\n",
        );
        tree.replace(
            "Cargo.toml",
            "unsafe_code = \"forbid\"",
            "unsafe_code = \"forbid\" # reason",
        );
        tree.replace(
            "Cargo.toml",
            "members = [\"apps/pool\", \"crates/pool-config\"]",
            "members = [\"apps/pool\", \"crates/pool-config\"] # \"crates/later\"",
        );
        assert_ok(&tree);
    }

    #[test]
    fn w1_and_w6_know_the_lockfile_of_the_protocol_fuzz_package() {
        // Spec 0004, "Fuzzing": the fourth lockfile has its own baseline.
        let mut tree = good();
        tree.set(
            "crates/pool-protocol/fuzz/Cargo.lock",
            "[[package]]\nname = \"pool-protocol\"\n\n[[package]]\nname = \"ghost\"\n",
        );
        assert_problem(
            &tree,
            "W1",
            &[
                "crates/pool-protocol/fuzz/Cargo.lock",
                "ghost",
                "protocol-fuzz-lock-packages.txt",
            ],
        );
        let mut tree = good();
        tree.remove("crates/pool-protocol/fuzz/Cargo.lock");
        let report = check(&tree);
        assert!(
            report
                .faults
                .iter()
                .any(|f| f.contains("crates/pool-protocol/fuzz/Cargo.lock")),
            "{}",
            report.dump()
        );
        // The file at its place is no stray lockfile; one beside it is.
        let mut tree = good();
        tree.set("crates/pool-protocol/Cargo.lock", "");
        assert_problem(&tree, "W6", &["crates/pool-protocol/Cargo.lock"]);
    }

    #[test]
    fn w6_unexpected_lockfile() {
        let mut tree = good();
        tree.set("apps/pool/Cargo.lock", "");
        assert_problem(&tree, "W6", &["apps/pool/Cargo.lock"]);
    }

    #[test]
    fn w7_copy_differs() {
        let mut tree = good();
        tree.replace(LINUX_BUILD_SCRIPT, "check passed", "check skipped");
        assert_problem(&tree, "W7", &["scripts/linux-build.sh", ":5"]);
    }

    #[test]
    fn w7_extra_line_in_copy() {
        let mut tree = good();
        let script = format!("{SCRIPT}echo extra\n");
        tree.set(LINUX_BUILD_SCRIPT, &script);
        assert_problem(&tree, "W7", &["scripts/linux-build.sh:6"]);
    }

    #[test]
    fn w7_missing_marker() {
        let mut tree = good();
        tree.replace(LINUX_BUILD_SCRIPT, MARKER, "# marker");
        assert_problem(&tree, "W7", &["scripts/linux-build.sh", "marker"]);
    }

    #[test]
    fn w7_missing_block() {
        let mut tree = good();
        tree.replace(SPEC_0001, "### Linux build", "### Linux builds");
        assert_problem(&tree, "W7", &[SPEC_0001]);
    }

    #[test]
    fn w7_accepts_script_without_final_newline() {
        let mut tree = good();
        tree.set(LINUX_BUILD_SCRIPT, SCRIPT.trim_end_matches('\n'));
        assert_ok(&tree);
    }
}

#[cfg(test)]
mod lockfile_edge_tests {
    //! Edges of the W4 lockfile check of spec 0003, added by /test
    //! (0003 AC-13). Since spec 0004 (`0004 AC-4`) an entry is the lines
    //! from one `[[package]]` line to the next line that starts with `[`,
    //! or to the end of the file.

    use super::*;
    use crate::tree::MemTree;

    const REGISTRY: &str = "source = \"registry+https://github.com/rust-lang/crates.io-index\"\n";

    fn local(name: &str) -> String {
        format!(
            "[[package]]\nname = \"{name}\"\nversion = \"0.1.0\"\n{}\n",
            if name == "pool" {
                "dependencies = [\n \"pool-config\",\n]\n"
            } else {
                ""
            }
        )
    }

    fn registry(name: &str) -> String {
        format!(
            "[[package]]\nname = \"{name}\"\nversion = \"1.0.0\"\n{REGISTRY}checksum = \"00\"\n\n"
        )
    }

    /// A tree that passes every rule, with `Cargo.lock` set to `lock` and
    /// every name of it in the W1 baseline, so that only W4 can object.
    fn tree(lock: &str, names: &[&str]) -> MemTree {
        let mut baseline = String::from("# spec 0002\n");
        for name in names {
            baseline.push_str(name);
            baseline.push('\n');
        }
        let mut tree = MemTree::of(&[
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"apps/pool\", \"crates/pool-config\"]\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
            ),
            (
                "apps/pool/Cargo.toml",
                "[package]\nname = \"pool\"\n\n[dependencies]\npool-config.workspace = true\n\n[lints]\nworkspace = true\n",
            ),
            (
                "crates/pool-config/Cargo.toml",
                "[package]\nname = \"pool-config\"\n\n[lints]\nworkspace = true\n",
            ),
            (
                SPEC_0001,
                "# 0001\n\n### Linux build\n\n```bash\nrun\n```\n",
            ),
            (
                LINUX_BUILD_SCRIPT,
                "#!/bin/sh\n# --- spec 0001, Linux build ---\nrun\n",
            ),
            (
                "tools/repo-check/baseline/fuzz-lock-packages.txt",
                "# spec 0002\npool-config\n",
            ),
            (
                "tools/repo-check/baseline/tool-lock-packages.txt",
                "# spec 0002\npool-repo-check\n",
            ),
            (
                "tools/repo-check/baseline/protocol-fuzz-lock-packages.txt",
                "# spec 0004\npool-protocol\n",
            ),
            (
                "crates/pool-protocol/fuzz/Cargo.lock",
                "[[package]]\nname = \"pool-protocol\"\n",
            ),
            (
                "tools/repo-check/baseline/forbidden-paths.txt",
                "# spec 0002\n",
            ),
            (
                "tools/repo-check/baseline/forbidden-source-patterns.txt",
                "# spec 0002\nstd::thread\n",
            ),
            (
                "tools/repo-check/baseline/internal-dependencies.txt",
                "# spec 0002\npool -> pool-config\n",
            ),
            (
                "crates/pool-config/fuzz/Cargo.lock",
                "[[package]]\nname = \"pool-config\"\n",
            ),
            (
                "tools/repo-check/Cargo.lock",
                "[[package]]\nname = \"pool-repo-check\"\n",
            ),
        ]);
        tree.set("tools/repo-check/baseline/lock-packages.txt", &baseline);
        tree.set("Cargo.lock", lock);
        tree.set(
            ".cargo/config.toml",
            include_str!("../../../.cargo/config.toml"),
        );
        tree.set("NOTICE", "");
        tree.set("tools/repo-check/baseline/native-library-versions.txt", "");
        tree
    }

    fn w4_lines(tree: &MemTree) -> Vec<String> {
        let report = check(tree);
        assert!(report.faults.is_empty(), "{}", report.dump());
        report
            .problems
            .iter()
            .map(ToString::to_string)
            .inspect(|line| assert!(line.starts_with("W4"), "only W4 may object: {line}"))
            .collect()
    }

    const GHOST: &str = "W4: Cargo.lock: package ghost has no source line but is not a member written in members of Cargo.toml";

    #[test]
    fn w4_a_local_entry_that_is_not_a_member_is_found_at_the_start_middle_and_end() {
        let names = ["ghost", "pool", "pool-config", "serde", "syn"];
        let start = format!(
            "version = 4\n\n{}{}{}{}{}",
            local("ghost"),
            local("pool"),
            local("pool-config"),
            registry("serde"),
            registry("syn")
        );
        let middle = format!(
            "version = 4\n\n{}{}{}{}{}",
            local("pool"),
            registry("serde"),
            local("ghost"),
            registry("syn"),
            local("pool-config")
        );
        let end = format!(
            "version = 4\n\n{}{}{}{}{}",
            local("pool"),
            local("pool-config"),
            registry("serde"),
            registry("syn"),
            local("ghost").trim_end()
        );
        for lock in [start, middle, end] {
            assert_eq!(
                local_packages(&lock)
                    .iter()
                    .filter(|n| *n == "ghost")
                    .count(),
                1
            );
            assert_eq!(w4_lines(&tree(&lock, &names)), [GHOST], "{lock}");
        }
    }

    #[test]
    fn w4_members_without_source_pass_wherever_they_sit() {
        let names = ["pool", "pool-config", "serde"];
        for lock in [
            format!(
                "{}{}{}",
                local("pool"),
                local("pool-config"),
                registry("serde")
            ),
            format!(
                "{}{}{}",
                registry("serde"),
                local("pool"),
                local("pool-config")
            ),
            format!(
                "{}{}{}",
                local("pool"),
                registry("serde"),
                local("pool-config")
            ),
        ] {
            assert_eq!(
                w4_lines(&tree(&lock, &names)),
                Vec::<String>::new(),
                "{lock}"
            );
        }
    }

    #[test]
    fn w4_reads_an_entry_line_by_line() {
        // The source line before the name, CRLF line ends, indented lines,
        // and a dependency list that holds member names all keep their
        // meaning.
        assert_eq!(
            local_packages(&format!(
                "[[package]]\n{REGISTRY}name = \"serde\"\n\n[[package]]\r\nname = \"ghost\"\r\nversion = \"0.1.0\"\r\ndependencies = [\r\n \"pool\",\r\n \"source\",\r\n]\r\n\r\n  [[package]]\n  name = \"spaced\"\n"
            )),
            ["ghost", "spaced"]
        );
        // An entry without a name line is not reported; the first name wins.
        assert_eq!(
            local_packages(
                "[[package]]\nversion = \"0.1.0\"\n\n[[package]]\nname = \"first\"\nname = \"second\"\n"
            ),
            ["first"]
        );
        // Text before the first entry is not part of any entry.
        assert_eq!(
            local_packages(&format!("version = 4\n{REGISTRY}\n{}", local("ghost"))),
            ["ghost"]
        );
        assert_eq!(local_packages(""), Vec::<String>::new());
    }

    #[test]
    fn w4_a_table_after_the_last_entry_ends_it() {
        // Spec 0004, "Proof script and lockfile check": an entry ends at the
        // next line that starts with `[`. Cargo writes `[[patch.unused]]`
        // tables after the packages, with their own `source` line; such a
        // table no longer hides a local last entry from W4.
        let unused =
            format!("\n[[patch.unused]]\nname = \"other\"\nversion = \"1.0.0\"\n{REGISTRY}");
        let names = ["pool", "pool-config", "serde", "ghost"];
        let found = format!(
            "{}{}{}{}{unused}",
            local("pool"),
            local("pool-config"),
            registry("serde"),
            local("ghost")
        );
        assert_eq!(local_packages(&found), ["pool", "pool-config", "ghost"]);
        assert_eq!(w4_lines(&tree(&found, &names)), [GHOST]);

        // Any table ends the entry, also one without a `source` line.
        let metadata = format!(
            "{}{}{}{}\n[metadata]\nnote = \"no source here\"\n",
            local("pool"),
            local("pool-config"),
            registry("serde"),
            local("ghost")
        );
        assert_eq!(w4_lines(&tree(&metadata, &names)), [GHOST]);

        // A `source` line after a table belongs to no entry: it does not
        // hide the local entry before the table.
        let after_table = format!("{}{}[metadata]\n{REGISTRY}", local("pool"), local("ghost"));
        assert_eq!(local_packages(&after_table), ["pool", "ghost"]);

        // A table without a `name` line is not an entry, and its lines are
        // not read as one.
        let unused_only = format!("{}{unused}", local("pool"));
        assert_eq!(local_packages(&unused_only), ["pool"]);

        // After a registry last entry the table changes nothing.
        let after_registry = format!(
            "{}{}{}{unused}",
            local("pool"),
            local("pool-config"),
            registry("serde")
        );
        assert_eq!(local_packages(&after_registry), ["pool", "pool-config"]);
    }

    #[test]
    fn w4_one_line_per_local_package_that_is_not_a_member() {
        let lock = format!(
            "{}{}{}{}",
            local("pool"),
            local("ghost"),
            local("pool-config"),
            local("extra")
        );
        let lines = w4_lines(&tree(&lock, &["pool", "ghost", "pool-config", "extra"]));
        assert_eq!(lines.len(), 2, "{lines:?}");
        assert_eq!(lines[0], GHOST);
        assert!(
            lines[1].contains("package extra has no source line"),
            "{}",
            lines[1]
        );
        // A member name that only differs in case is not a member.
        let lock = format!("{}{}{}", local("Pool"), local("pool"), local("pool-config"));
        let lines = w4_lines(&tree(&lock, &["Pool", "pool", "pool-config"]));
        assert!(
            lines
                .iter()
                .any(|l| l.contains("package Pool has no source line")),
            "{lines:?}"
        );
    }
}

#[cfg(test)]
mod source_guard_tests {
    //! The three kinds of line of rule W3, spec 0004 "Source guard"
    //! (`0004 AC-2`): one passing and one failing case for each kind.

    use super::*;
    use crate::tree::MemTree;

    const PATTERNS: &str = "tools/repo-check/baseline/forbidden-source-patterns.txt";

    /// A tree that passes every other rule, with the W3 baseline set to
    /// `patterns`.
    fn tree(patterns: &str) -> MemTree {
        let mut tree = MemTree::of(&[
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"apps/pool\", \"crates/pool-config\"]\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
            ),
            (
                "apps/pool/Cargo.toml",
                "[package]\nname = \"pool\"\n\n[dependencies]\npool-config.workspace = true\n\n[lints]\nworkspace = true\n",
            ),
            (
                "crates/pool-config/Cargo.toml",
                "[package]\nname = \"pool-config\"\n\n[lints]\nworkspace = true\n",
            ),
            (
                "Cargo.lock",
                "[[package]]\nname = \"pool\"\ndependencies = [\n \"pool-config\",\n]\n\n[[package]]\nname = \"pool-config\"\n",
            ),
            (
                SPEC_0001,
                "# 0001\n\n### Linux build\n\n```bash\nrun\n```\n",
            ),
            (
                LINUX_BUILD_SCRIPT,
                "#!/bin/sh\n# --- spec 0001, Linux build ---\nrun\n",
            ),
            (
                "tools/repo-check/baseline/lock-packages.txt",
                "# spec 0002\npool\npool-config\n",
            ),
            (
                "tools/repo-check/baseline/fuzz-lock-packages.txt",
                "# spec 0002\npool-config\n",
            ),
            (
                "tools/repo-check/baseline/tool-lock-packages.txt",
                "# spec 0002\npool-repo-check\n",
            ),
            (
                "tools/repo-check/baseline/protocol-fuzz-lock-packages.txt",
                "# spec 0004\npool-protocol\n",
            ),
            (
                "crates/pool-protocol/fuzz/Cargo.lock",
                "[[package]]\nname = \"pool-protocol\"\n",
            ),
            (
                "tools/repo-check/baseline/forbidden-paths.txt",
                "# spec 0002\n",
            ),
            (
                "tools/repo-check/baseline/internal-dependencies.txt",
                "# spec 0002\npool -> pool-config\n",
            ),
            (
                "crates/pool-config/fuzz/Cargo.lock",
                "[[package]]\nname = \"pool-config\"\n",
            ),
            (
                "tools/repo-check/Cargo.lock",
                "[[package]]\nname = \"pool-repo-check\"\n",
            ),
            ("apps/pool/src/main.rs", "fn main() {}\n"),
            ("crates/pool-config/src/lib.rs", "//! Config.\n"),
        ]);
        tree.set(PATTERNS, patterns);
        tree.set(
            ".cargo/config.toml",
            include_str!("../../../.cargo/config.toml"),
        );
        tree.set("NOTICE", "");
        tree.set("tools/repo-check/baseline/native-library-versions.txt", "");
        tree
    }

    fn w3_lines(tree: &MemTree) -> Vec<String> {
        let report = check(tree);
        assert!(report.faults.is_empty(), "{}", report.dump());
        report
            .problems
            .iter()
            .map(ToString::to_string)
            .inspect(|line| assert!(line.starts_with("W3"), "only W3 may object: {line}"))
            .collect()
    }

    #[test]
    fn a_bare_line_forbids_the_text_everywhere() {
        let mut t = tree("# spec 0004\ntokio\n");
        assert_eq!(w3_lines(&t), Vec::<String>::new());
        t.set("crates/pool-config/src/lib.rs", "//! Config.\nuse tokio;\n");
        t.set("apps/pool/src/run.rs", "// tokio\n");
        let lines = w3_lines(&t);
        assert_eq!(lines.len(), 2, "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("apps/pool/src/run.rs:1") && l.contains("forbidden text tokio")));
        assert!(
            lines
                .iter()
                .any(|l| l.contains("crates/pool-config/src/lib.rs:2"))
        );
    }

    #[test]
    fn a_deny_line_forbids_the_text_only_at_or_under_its_path() {
        let patterns = "deny crates/pool-config/src std::fs\n";
        let mut t = tree(patterns);
        t.set("crates/pool-config/build.rs", "use std::fs;\n");
        t.set("crates/pool-config-x/src/lib.rs", "use std::fs;\n");
        t.set("apps/pool/src/main.rs", "use std::fs;\nfn main() {}\n");
        assert_eq!(w3_lines(&t), Vec::<String>::new());
        t.set("crates/pool-config/src/inner/read.rs", "\nuse std::fs;\n");
        let lines = w3_lines(&t);
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert!(
            lines[0].contains("crates/pool-config/src/inner/read.rs:2"),
            "{}",
            lines[0]
        );
        assert!(lines[0].contains("std::fs"), "{}", lines[0]);
    }

    #[test]
    fn an_allow_line_lifts_the_text_only_at_or_under_its_path() {
        let patterns = "tokio\nallow apps/pool tokio\ndeny crates/pool-config/src rusqlite\nallow crates/pool-config/src/journal.rs rusqlite\n";
        let mut t = tree(patterns);
        t.set("apps/pool/src/run.rs", "use tokio;\n");
        t.set("crates/pool-config/src/journal.rs", "use rusqlite;\n");
        assert_eq!(w3_lines(&t), Vec::<String>::new());
        // The allowed path is compared by whole components, a file is a
        // path of its own, and an allow line lifts only its own text.
        t.set("apps/pool-x/src/lib.rs", "use tokio;\n");
        t.set("crates/pool-config/src/journal.rs.rs", "use rusqlite;\n");
        t.set("crates/pool-config/src/other.rs", "use rusqlite;\n");
        t.set("apps/pool/src/thread.rs", "use rusqlite;\n");
        let lines = w3_lines(&t);
        assert_eq!(lines.len(), 3, "{lines:?}");
        assert!(
            lines
                .iter()
                .any(|l| l.contains("apps/pool-x/src/lib.rs:1") && l.contains("tokio"))
        );
        assert!(
            lines
                .iter()
                .any(|l| l.contains("crates/pool-config/src/journal.rs.rs:1"))
        );
        assert!(
            lines
                .iter()
                .any(|l| l.contains("crates/pool-config/src/other.rs:1"))
        );
    }

    #[test]
    fn a_text_forbidden_twice_is_reported_once_per_line() {
        let mut t = tree("std::env\ndeny apps/pool std::env\n");
        t.set("apps/pool/src/main.rs", "use std::env;\nfn main() {}\n");
        assert_eq!(w3_lines(&t).len(), 1);
    }

    #[test]
    fn a_line_of_no_known_form_is_a_problem_of_the_baseline() {
        for line in [
            "deny crates/pool-config",
            "allow a b c",
            "permit apps/pool tokio",
            "deny  apps/pool tokio",
            "deny",
        ] {
            let lines = w3_lines(&tree(&format!("{line}\n")));
            assert_eq!(lines.len(), 1, "{line}: {lines:?}");
            assert!(lines[0].contains(PATTERNS), "{}", lines[0]);
            assert!(lines[0].contains(line), "{}", lines[0]);
        }
    }

    #[test]
    fn an_allow_covering_a_whole_explicit_deny_is_rejected() {
        for allowed in ["crates/pool-config", "crates/pool-config/src"] {
            let t = tree(&format!(
                "deny crates/pool-config/src std::fs\nallow {allowed} std::fs\n"
            ));
            assert!(
                w3_lines(&t)
                    .iter()
                    .any(|line| line.contains("is hidden by allow"))
            );
        }
    }

    #[test]
    fn source_rule_reads_the_three_forms() {
        assert_eq!(
            source_rule("tokio"),
            Some(SourceRule::Everywhere("tokio".into()))
        );
        assert_eq!(
            source_rule("deny crates/x std::fs"),
            Some(SourceRule::Deny("crates/x".into(), "std::fs".into()))
        );
        assert_eq!(
            source_rule("allow crates/x tokio"),
            Some(SourceRule::Allow("crates/x".into(), "tokio".into()))
        );
        assert_eq!(source_rule("two words"), None);
        assert!(covers("crates/pool-core", "crates/pool-core/src/lib.rs"));
        assert!(covers("crates/pool-core/", "crates/pool-core/src/lib.rs"));
        assert!(!covers("crates/pool-core", "crates/pool-core-x/src/lib.rs"));
        assert!(covers("crates/a/journal.rs", "crates/a/journal.rs"));
    }
}
