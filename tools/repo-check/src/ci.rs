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
        pinned_actions(path, text, &mut report);
        forbidden_triggers(path, text, &mut report);
    }
    if let Some(ci) = &ci {
        if let Some(ruleset) = &ruleset {
            jobs_match_checks(ci, ruleset, &mut report);
        }
        runs(ci, &mut report);
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

fn forbidden_triggers(path: &str, text: &str, report: &mut Report) {
    for (index, line) in text.lines().enumerate() {
        for word in ["pull_request_target", "secrets."] {
            if line.contains(word) {
                report.add_at("C5", path, index + 1, format!("the workflow uses {word}"));
            }
        }
    }
}

/// Returns the job ids of a workflow with their line numbers: lines of two spaces, an id and a colon after the `jobs:` line.
pub fn job_ids(ci: &str) -> Vec<(usize, String)> {
    let mut ids = Vec::new();
    let mut in_jobs = false;
    for (index, line) in ci.lines().enumerate() {
        let line = line.trim_end();
        if line == "jobs:" {
            in_jobs = true;
            continue;
        }
        if !in_jobs {
            continue;
        }
        if let Some(rest) = line.strip_prefix("  ")
            && let Some(id) = rest.strip_suffix(':')
            && !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        {
            ids.push((index + 1, id.to_string()));
        }
    }
    ids
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

fn runs(ci: &str, report: &mut Report) {
    let jobs = job_ids(ci);
    let mut last_run: Vec<Option<(usize, String)>> = vec![None; jobs.len()];
    for (index, line) in ci.lines().enumerate() {
        let Some(value) = value_of(line, "run") else {
            continue;
        };
        let number = index + 1;
        let job = jobs.iter().rposition(|(start, _)| *start < number);
        let id = job.map_or("(none)", |j| jobs[j].1.as_str());
        let allowed = value == "rustup toolchain install"
            || value
                .strip_prefix("scripts/check.sh ")
                .is_some_and(|name| !name.is_empty() && !name.contains(char::is_whitespace));
        if !allowed {
            report.add_at(
                "C3",
                CI,
                number,
                format!("job {id} runs {value}, which is neither rustup toolchain install nor scripts/check.sh <name>"),
            );
        }
        match job {
            Some(j) => last_run[j] = Some((number, value.to_string())),
            None => report.add_at("C3", CI, number, "run: before the first job"),
        }
    }
    for ((line, id), last) in jobs.iter().zip(last_run) {
        let wanted = format!("scripts/check.sh {id}");
        match last {
            Some((_, value)) if value == wanted => {}
            Some((at, value)) => report.add_at(
                "C3",
                CI,
                at,
                format!("the last run of job {id} is {value}, not {wanted}"),
            ),
            None => report.add_at(
                "C3",
                CI,
                *line,
                format!("job {id} has no run step {wanted}"),
            ),
        }
    }
}

fn runners(ci: &str, report: &mut Report) {
    for (index, line) in ci.lines().enumerate() {
        if let Some(value) = value_of(line, "runs-on")
            && value != RUNNER
        {
            report.add_at(
                "C4",
                CI,
                index + 1,
                format!("runs-on is {value}, not {RUNNER}"),
            );
        }
    }
}

fn permissions(ci: &str, report: &mut Report) {
    let lines: Vec<&str> = ci.lines().collect();
    let found: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| {
            value_of(line, "permissions").is_some() && !line.trim_start().starts_with('#')
        })
        .map(|(index, _)| index)
        .collect();
    let [at] = found.as_slice() else {
        report.add(
            "C5",
            CI,
            format!(
                "the workflow has {} permissions: lines, not one",
                found.len()
            ),
        );
        return;
    };
    let at = *at;
    let top_level = lines[at].trim_end() == "permissions:";
    let read_only = lines.get(at + 1).map(|l| l.trim_end()) == Some("  contents: read");
    let nothing_more = lines
        .get(at + 2)
        .is_none_or(|next| !next.starts_with(' ') || next.trim().is_empty());
    if !(top_level && read_only && nothing_more) {
        report.add_at(
            "C5",
            CI,
            at + 1,
            "permissions: must start a line and be followed only by   contents: read",
        );
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
        let mut tree = good();
        tree.set(
            ".github/workflows/other.yml",
            &format!("jobs:\n  x:\n    steps:\n      - uses: actions/cache@{HASH} # v6.1.0\n"),
        );
        assert_ok(&tree);
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
        assert_problem(&tree, "C5", &[CI, "2 permissions"]);
    }

    #[test]
    fn c5_secrets_and_target_trigger() {
        let mut tree = good();
        tree.replace(CI, "  pull_request:\n", "  pull_request_target:\n");
        assert_problem(&tree, "C5", &[CI, "pull_request_target"]);
        let mut tree = good();
        tree.set(
            ".github/workflows/release.yml",
            "env:\n  TOKEN: ${{ secrets.TOKEN }}\n",
        );
        assert_problem(&tree, "C5", &[".github/workflows/release.yml", "secrets."]);
    }
}
