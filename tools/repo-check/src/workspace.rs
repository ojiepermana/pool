//! Workspace guards W1 to W7 of spec 0002.

use std::collections::{BTreeMap, BTreeSet};

use crate::report::Report;
use crate::text::{self, baseline_lines, key_value, toml_lines};
use crate::tree::{Tree, file_name, join};

/// The directory that holds the baseline files.
pub const BASELINE: &str = "tools/repo-check/baseline";

/// The three lockfiles and their baseline files (rule W1).
pub const LOCKFILES: [(&str, &str); 3] = [
    ("Cargo.lock", "lock-packages.txt"),
    (
        "crates/pool-config/fuzz/Cargo.lock",
        "fuzz-lock-packages.txt",
    ),
    ("tools/repo-check/Cargo.lock", "tool-lock-packages.txt"),
];

/// The script that holds the copy of the Linux build command (rule W7).
pub const LINUX_BUILD_SCRIPT: &str = "scripts/linux-build.sh";

/// The spec that owns the Linux build command (rule W7).
pub const SPEC_0001: &str = "docs/specs/0001-initial-architecture/index.md";

/// The line after which the script holds the copied command (rule W7).
pub const MARKER: &str = "# --- spec 0001, Linux build ---";

/// Runs rules W1 to W7.
pub fn check(tree: &dyn Tree) -> Report {
    let mut report = Report::default();
    lockfiles(tree, &mut report);
    forbidden_paths(tree, &mut report);
    forbidden_text(tree, &mut report);
    let members = members(tree, &mut report);
    dependencies(tree, &members, &mut report);
    stray_lockfiles(tree, &mut report);
    linux_build_copy(tree, &mut report);
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

fn forbidden_text(tree: &dyn Tree, report: &mut Report) {
    let Some(patterns) = baseline(tree, "forbidden-source-patterns.txt", report) else {
        return;
    };
    for dir in ["apps", "crates"] {
        for path in report.walk(tree, dir, &|name| name == "target") {
            if !path.ends_with(".rs") {
                continue;
            }
            let Some(text) = report.need(tree, &path) else {
                continue;
            };
            for (index, line) in text.lines().enumerate() {
                for pattern in &patterns {
                    if line.contains(pattern.as_str()) {
                        report.add_at("W3", &path, index + 1, format!("forbidden text {pattern}"));
                    }
                }
            }
        }
    }
}

/// A workspace member: its directory and its package name.
pub struct Member {
    /// The path of its manifest.
    pub manifest: String,
    /// The `name` of its `[package]` table, when it has one.
    pub name: Option<String>,
    /// The manifest content.
    pub text: String,
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
        members.push(Member {
            manifest,
            name,
            text,
        });
    }
    members
}

const DEPENDENCY_TABLES: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];

/// One dependency declared in a manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    /// The 1 based line where the dependency is first named.
    pub line: usize,
    /// The dependency key, which is the name the code uses.
    pub key: String,
    /// The package named by `package = "..."`, when the dependency is renamed.
    pub package: Option<String>,
}

fn is_dependency_table(part: &str) -> bool {
    DEPENDENCY_TABLES.contains(&part)
}

/// Reads `package = "<name>"` from the rest of a dependency line: `.package = "x"` after the key, or a `package` key inside an inline table.
fn package_of(rest: &str) -> Option<String> {
    if let Some(sub) = rest.strip_prefix('.') {
        let (key, after) = text::leading_key(sub)?;
        return (key == "package")
            .then(|| after.strip_prefix('='))
            .flatten()
            .and_then(text::string_value);
    }
    let value = rest.strip_prefix('=')?.trim_start();
    let mut inner = value.strip_prefix('{')?;
    loop {
        let (key, after) = text::leading_key(inner)?;
        let after = after.strip_prefix('=')?.trim_start();
        if key == "package" {
            return text::string_value(after);
        }
        let comma = next_comma(after)?;
        inner = after[comma + 1..].trim_start();
    }
}

/// Returns the byte index of the next comma that is outside strings, brackets and braces.
fn next_comma(text: &str) -> Option<usize> {
    let mut quote: Option<char> = None;
    let mut depth = 0usize;
    for (index, c) in text.char_indices() {
        match quote {
            Some(q) if c == q => quote = None,
            Some(_) => {}
            None => match c {
                '"' | '\'' => quote = Some(c),
                '[' | '{' => depth += 1,
                ']' | '}' if depth == 0 => return None,
                ']' | '}' => depth -= 1,
                ',' if depth == 0 => return Some(index),
                _ => {}
            },
        }
    }
    None
}

/// Returns the dependencies a manifest declares in every dependency table: `[dependencies]` and the dev and build forms, under any prefix such as `target.<cfg>` or `workspace`, and the tables for one dependency such as `[target.<cfg>.dependencies.<key>]`. Comments are cut, quoted keys are unquoted, and a `package` rename is read.
pub fn dependencies_of(manifest: &str) -> Vec<Dependency> {
    enum Table {
        Other,
        Many(usize),
        One(usize),
    }
    let mut found: Vec<Dependency> = Vec::new();
    let mut table = Table::Other;
    for (index, raw) in manifest.lines().enumerate() {
        let line = text::strip_comment(raw).trim();
        if line.is_empty() {
            continue;
        }
        if let Some(parts) = text::table_parts(line) {
            let n = parts.len();
            table = if is_dependency_table(&parts[n - 1]) {
                Table::Many(found.len())
            } else if n >= 2 && is_dependency_table(&parts[n - 2]) {
                found.push(Dependency {
                    line: index + 1,
                    key: parts[n - 1].clone(),
                    package: None,
                });
                Table::One(found.len() - 1)
            } else {
                Table::Other
            };
            continue;
        }
        let Some((key, rest)) = text::leading_key(line) else {
            continue;
        };
        if !(rest.starts_with('.') || rest.starts_with('=')) {
            continue;
        }
        match table {
            Table::Other => {}
            Table::One(at) => {
                if key == "package"
                    && let Some(value) = rest.strip_prefix('=').and_then(text::string_value)
                {
                    found[at].package = Some(value);
                }
            }
            Table::Many(start) => {
                let package = package_of(rest);
                if let Some(entry) = found[start..].iter_mut().find(|d| d.key == key) {
                    if package.is_some() {
                        entry.package = package;
                    }
                } else {
                    found.push(Dependency {
                        line: index + 1,
                        key,
                        package,
                    });
                }
            }
        }
    }
    found
}

/// Returns the renames of the root `[workspace.dependencies]` table: key to package, for the keys that have `package = "..."`. A member that inherits such a key with `workspace = true` gets that package.
fn workspace_renames(root_manifest: &str) -> BTreeMap<String, String> {
    let mut renames = BTreeMap::new();
    let mut in_workspace = false;
    let mut table_lines = String::new();
    for raw in root_manifest.lines() {
        let line = text::strip_comment(raw).trim();
        if let Some(parts) = text::table_parts(line) {
            in_workspace = parts.first().map(String::as_str) == Some("workspace")
                && parts.get(1).map(String::as_str) == Some("dependencies");
        }
        if in_workspace {
            table_lines.push_str(raw);
            table_lines.push('\n');
        }
    }
    for dependency in dependencies_of(&table_lines) {
        if let Some(package) = dependency.package {
            renames.insert(dependency.key, package);
        }
    }
    renames
}

fn dependencies(tree: &dyn Tree, members: &[Member], report: &mut Report) {
    let Some(allowed) = baseline(tree, "internal-dependencies.txt", report) else {
        return;
    };
    let renames = tree
        .read("Cargo.toml")
        .ok()
        .flatten()
        .map(|root| workspace_renames(&root))
        .unwrap_or_default();
    let allowed: BTreeSet<String> = allowed.into_iter().collect();
    let names: BTreeSet<&str> = members.iter().filter_map(|m| m.name.as_deref()).collect();
    let mut found = BTreeSet::new();
    for member in members {
        let Some(from) = member.name.as_deref() else {
            continue;
        };
        for dependency in dependencies_of(&member.text) {
            let to = match &dependency.package {
                Some(package) => package.as_str(),
                None => renames
                    .get(&dependency.key)
                    .map_or(dependency.key.as_str(), String::as_str),
            };
            if !names.contains(to) {
                continue;
            }
            let pair = format!("{from} -> {to}");
            if !allowed.contains(&pair) {
                report.add_at(
                    "W5",
                    &member.manifest,
                    dependency.line,
                    format!("dependency {pair} is not allowed by internal-dependencies.txt"),
                );
            }
            found.insert(pair);
        }
    }
    for pair in allowed.difference(&found) {
        report.add(
            "W5",
            &join(BASELINE, "internal-dependencies.txt"),
            format!("dependency {pair} is listed but missing from the manifests"),
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

    fn lock(names: &[&str]) -> String {
        names
            .iter()
            .map(|name| format!("[[package]]\nname = \"{name}\"\nversion = \"1.0.0\"\n\n"))
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
    fn w5_dependency_not_allowed() {
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies]\npool = { path = \"../../apps/pool\" }\n",
        );
        assert_problem(
            &tree,
            "W5",
            &[
                "crates/pool-config/Cargo.toml:5",
                "pool-config -> pool",
                "not allowed",
            ],
        );
    }

    #[test]
    fn w5_dependency_in_dotted_table_forms() {
        let mut tree = good();
        let base = tree.get("crates/pool-config/Cargo.toml");
        tree.set(
            "crates/pool-config/Cargo.toml",
            &format!("{base}\n[dev-dependencies.pool]\npath = \"../../apps/pool\"\n"),
        );
        assert_problem(&tree, "W5", &["pool-config -> pool"]);
        let mut tree = good();
        tree.set(
            "crates/pool-config/Cargo.toml",
            &format!("{base}\n[target.'cfg(unix)'.build-dependencies]\npool.workspace = true\n"),
        );
        assert_problem(&tree, "W5", &["pool-config -> pool"]);
    }

    #[test]
    fn w5_listed_dependency_missing() {
        let mut tree = good();
        tree.replace("apps/pool/Cargo.toml", "pool-config.workspace = true\n", "");
        assert_problem(&tree, "W5", &["pool -> pool-config", "missing"]);
    }

    #[test]
    fn w5_header_with_a_comment() {
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies] # runtime\npool = { path = \"../../apps/pool\" }\n",
        );
        assert_problem(
            &tree,
            "W5",
            &["crates/pool-config/Cargo.toml:5", "pool-config -> pool"],
        );
    }

    #[test]
    fn w5_quoted_key_and_target_table_for_one_dependency() {
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies]\n\"pool\" = { path = \"../../apps/pool\" }\n",
        );
        assert_problem(&tree, "W5", &["pool-config -> pool"]);
        let mut tree = good();
        let base = tree.get("crates/pool-config/Cargo.toml");
        tree.set(
            "crates/pool-config/Cargo.toml",
            &format!(
                "{base}\n[target.'cfg(unix)'.dependencies.pool]\npath = \"../../apps/pool\"\n"
            ),
        );
        assert_problem(&tree, "W5", &["pool-config -> pool"]);
    }

    #[test]
    fn w5_renamed_dependency() {
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies]\ndaemon = { path = \"../../apps/pool\", package = \"pool\" }\n",
        );
        assert_problem(&tree, "W5", &["pool-config -> pool", "not allowed"]);
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies]\ndaemon.path = \"../../apps/pool\"\ndaemon.package = 'pool'\n",
        );
        assert_problem(
            &tree,
            "W5",
            &["crates/pool-config/Cargo.toml:5", "pool-config -> pool"],
        );
        let mut tree = good();
        let base = tree.get("crates/pool-config/Cargo.toml");
        tree.set(
            "crates/pool-config/Cargo.toml",
            &format!("{base}\n[dev-dependencies.daemon]\npackage = \"pool\"\npath = \"../../apps/pool\"\n"),
        );
        assert_problem(&tree, "W5", &["pool-config -> pool"]);
    }

    #[test]
    fn w5_rename_inherited_from_the_workspace() {
        let mut tree = good();
        tree.replace(
            "Cargo.toml",
            "pool-config = { path = \"crates/pool-config\" }",
            "config = { package = \"pool-config\", path = \"crates/pool-config\" }",
        );
        tree.replace(
            "apps/pool/Cargo.toml",
            "pool-config.workspace = true",
            "config.workspace = true",
        );
        assert_ok(&tree);
    }

    #[test]
    fn w5_rename_to_a_package_that_is_not_a_member() {
        let mut tree = good();
        tree.replace(
            "crates/pool-config/Cargo.toml",
            "[dependencies]\n",
            "[dependencies]\npool = { package = \"pool-helpers\", version = \"1\" }\n",
        );
        assert_ok(&tree);
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
