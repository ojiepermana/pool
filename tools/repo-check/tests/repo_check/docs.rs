//! `pool-repo-check docs` on trees on disk and on Git revisions: rules D1 to D6 (0002 AC-8, AC-9, AC-10).

use crate::common::{Fixture, line_of, run};

const README: &str = "README.md";
const INDEX: &str = "docs/scope/index.md";
const SECURITY: &str = "docs/security/security-requirements.md";
const SPEC: &str = "docs/specs/0001-initial-architecture/index.md";

/// Builds a small documentation tree that passes every rule: a scope index with two features, the four table owner
/// files, one PRD, one spec, and a README that links and cites across them.
fn good_tree(name: &str) -> Fixture {
    let f = Fixture::new(name);
    f.write(
        INDEX,
        "# Scope\n\n| # | Feature | Phase | Status |\n|---|---|---|---|\n| 1 | [Stack and workspace](001-stack-and-workspace.md) | 1 | done |\n| 2 | [Standards and tooling](002-standards-and-tooling.md) | 1 | in-progress |\n",
    );
    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 1. Stack and workspace · done\n",
    );
    f.write(
        "docs/scope/002-standards-and-tooling.md",
        "# 2. Standards and tooling · in-progress · needs a decision\n",
    );
    f.write(
        SECURITY,
        "# Security requirements\n\n| ID | Requirement |\n|---|---|\n| SEC-1 | First |\n| SEC-2 | Second |\n",
    );
    f.write(
        "docs/plan/risk-register.md",
        "# Risks\n\n| R-1 | One risk |\n",
    );
    f.write(
        "docs/feature-matrix.md",
        "# Features\n\n| F-1 | One feature |\n",
    );
    f.write(
        "docs/security/threat-model.md",
        "# Threats\n\n| T-1 | One threat |\n",
    );
    f.write(
        "docs/prd/001-proxy.md",
        "# PRD 001\n\n## Functional requirements\n\n1. Accept.\n2. Route.\n\n## Acceptance criteria\n\n- **AC-1** (accept): it accepts.\n",
    );
    f.write(
        SPEC,
        "# 0001\n\n## Requirements\n\n- **AC-1** (build): it builds.\n- **AC-2** (test): it tests.\n\n## Safety contracts\n\n1. Writes stay on the primary.\n2. Close uncertain state.\n\n## Linux build\n\nText.\n",
    );
    f.write(
        README,
        "# Readme\n\nSee [the scope](docs/scope/index.md), [risks](docs/plan/risk-register.md#risks) and [the build](docs/specs/0001-initial-architecture/index.md#linux-build).\n\nIt meets SEC-1, R-1, F-1, T-1, PRD-001 AC-1, PRD-001 FR-2, `0001 AC-2` and 0001 SC-2.\n\n[ref]: docs/feature-matrix.md#features\n",
    );
    f
}

// --- D1 and D2: links and anchors (0002 AC-8) ---

#[test]
fn docs_passes_on_a_small_correct_tree() {
    // covers: 0002 AC-8, AC-9, AC-10
    let f = good_tree("docs_good");
    f.check("docs", &[]).assert_ok("docs");
}

#[test]
fn d1_names_the_file_line_and_target_of_a_missing_link_target() {
    // covers: 0002 AC-8, D1
    let f = good_tree("docs_d1");
    f.append(README, "\nAlso [gone](docs/missing-file.md).\n");
    let line = line_of(&f.read(README), "docs/missing-file.md");
    f.check("docs", &[]).assert_problem(
        "docs",
        "D1",
        &[&format!("README.md:{line}:"), "docs/missing-file.md"],
    );
}

#[test]
fn d1_checks_images_reference_definitions_and_links_to_directories() {
    // covers: 0002 AC-8, D1: an image and a reference definition are links; a directory is a valid target.
    let f = good_tree("docs_d1_forms");
    f.append(
        README,
        "\n![logo](docs/logo.png \"Logo\")\n\n[later]: docs/later.md\n\nThe [docs folder](docs/plan/).\n",
    );
    let out = f.check("docs", &[]);
    out.assert_problem("docs", "D1", &["docs/logo.png"]);
    assert!(out.has("D1", &["docs/later.md"]), "{out}");
    assert_eq!(
        out.problems().len(),
        2,
        "the directory link is valid:\n{out}"
    );
}

#[test]
fn d1_rejects_an_absolute_path_and_a_path_that_leaves_the_root() {
    // covers: 0002 AC-8, D1
    let f = good_tree("docs_d1_paths");
    f.append(
        README,
        "\n[abs](/docs/feature-matrix.md) and [up](../outside.md).\n",
    );
    let out = f.check("docs", &[]);
    out.assert_problem("docs", "D1", &["/docs/feature-matrix.md", "absolute path"]);
    assert!(
        out.has("D1", &["../outside.md", "leaves the repository"]),
        "{out}"
    );
}

#[test]
fn d1_skips_urls_code_spans_and_fenced_code() {
    // covers: 0002 AC-8, D1: targets with a scheme, text between backticks and fenced blocks are not read.
    let f = good_tree("docs_d1_skip");
    f.append(
        README,
        "\n[site](https://example.invalid/missing.md), [mail](mailto:a@example.invalid), `[code](docs/nope.md)`.\n\n```md\n[inside](docs/nope.md)\n```\n",
    );
    f.check("docs", &[]).assert_ok("docs");
}

#[test]
fn d1_reads_a_link_after_a_nested_fence_closes() {
    // covers: 0002 AC-8, D1: a shorter fence inside a longer one is content; the longer fence closes the block.
    let f = good_tree("docs_d1_nested");
    f.append(
        README,
        "\n````md\n```sh\n[inside](docs/nope-inside.md)\n```\n[still inside](docs/nope-too.md)\n````\n\n[after](docs/nope-after.md)\n\n~~~\n[tilde](docs/nope-tilde.md)\n~~~\n",
    );
    let out = f.check("docs", &[]);
    out.assert_problem("docs", "D1", &["docs/nope-after.md"]);
    assert_eq!(
        out.problems().len(),
        1,
        "only the link after the fences counts:\n{out}"
    );
}

#[test]
fn d1_decodes_percent_twenty_and_drops_angle_brackets() {
    // covers: 0002 AC-8, D1
    let f = good_tree("docs_d1_space");
    f.write("docs/a b.md", "# Spaced name\n");
    f.append(
        README,
        "\n[pct](docs/a%20b.md), [angle](<docs/a b.md>), [both](<docs/a b.md#spaced-name>).\n",
    );
    f.check("docs", &[]).assert_ok("docs");

    f.append(README, "\n[bad](<docs/a b.md#no-such-heading>)\n");
    f.check("docs", &[])
        .assert_problem("docs", "D2", &["no-such-heading", "docs/a b.md"]);
}

#[test]
fn d2_names_a_fragment_that_is_not_a_heading() {
    // covers: 0002 AC-8, D2
    let f = good_tree("docs_d2");
    f.append(
        README,
        "\n[bad](docs/plan/risk-register.md#no-such-heading)\n",
    );
    let line = line_of(&f.read(README), "#no-such-heading");
    f.check("docs", &[]).assert_problem(
        "docs",
        "D2",
        &[&format!("README.md:{line}:"), "no-such-heading"],
    );
}

#[test]
fn d2_checks_a_fragment_in_the_same_file() {
    // covers: 0002 AC-8, D2: an empty path means the file itself.
    let f = good_tree("docs_d2_self");
    f.append(README, "\n[top](#readme) and [nowhere](#nowhere)\n");
    f.check("docs", &[])
        .assert_problem("docs", "D2", &["README.md", "nowhere"]);
}

#[test]
fn d2_numbers_repeated_headings_from_one() {
    // covers: 0002 AC-8, D2: the second heading gets `-1`, the third `-2`; there is no `-3`.
    let f = good_tree("docs_d2_repeat");
    f.write(
        "docs/notes.md",
        "# Notes\n\n## Step\n\n## Step\n\n## Step\n",
    );
    f.append(
        README,
        "\n[a](docs/notes.md#step) [b](docs/notes.md#step-1) [c](docs/notes.md#step-2)\n",
    );
    f.check("docs", &[]).assert_ok("docs");

    f.append(README, "\n[d](docs/notes.md#step-3)\n");
    f.check("docs", &[])
        .assert_problem("docs", "D2", &["step-3"]);
}

#[test]
fn d2_builds_anchors_from_link_text_any_script_and_ignores_headings_in_code() {
    // covers: 0002 AC-8, D2
    let f = good_tree("docs_d2_slug");
    f.write(
        "docs/slugs.md",
        "# Slugs\n\n## Where a [change](../README.md) belongs\n\n## docs/prd\n\n## Überblick 2\n\n```md\n## Hidden\n```\n",
    );
    f.append(
        README,
        "\n[a](docs/slugs.md#where-a-change-belongs) [b](docs/slugs.md#docsprd) [c](docs/slugs.md#überblick-2)\n",
    );
    f.check("docs", &[]).assert_ok("docs");

    f.append(README, "\n[d](docs/slugs.md#hidden)\n");
    f.check("docs", &[])
        .assert_problem("docs", "D2", &["hidden"]);
}

#[test]
fn d2_does_not_check_a_fragment_on_a_file_that_is_not_markdown() {
    // covers: 0002 AC-8, D2: a fragment is checked only when the target is a `.md` file.
    let f = good_tree("docs_d2_other");
    f.write("scripts/check.sh", "#!/bin/sh\n");
    f.append(README, "\n[line](scripts/check.sh#L10)\n");
    f.check("docs", &[]).assert_ok("docs");
}

// --- Directories, symlinks and unreadable files ---

#[test]
fn docs_skips_dot_target_graphify_and_node_modules_directories_but_not_github() {
    // covers: 0002 AC-8: the skipped directories are not read; `.github` is.
    let f = good_tree("docs_skip_dirs");
    for dir in [
        ".cache",
        ".claude",
        "target",
        "graphify-out",
        "node_modules",
        "apps/pool/target",
    ] {
        f.write(&format!("{dir}/bad.md"), "[gone](missing.md)\n");
    }
    f.check("docs", &[]).assert_ok("docs");

    f.write(".github/pull_request_template.md", "[gone](missing.md)\n");
    f.check("docs", &[]).assert_problem(
        "docs",
        "D1",
        &[".github/pull_request_template.md:1:", "missing.md"],
    );
}

#[test]
fn docs_reads_a_markdown_file_that_is_not_committed() {
    // covers: 0002 AC-8: the working tree is read, not the index.
    let f = good_tree("docs_uncommitted");
    f.commit_all("base");
    f.write("docs/new.md", "[gone](missing.md)\n");
    f.check("docs", &[])
        .assert_problem("docs", "D1", &["docs/new.md:1:"]);
}

#[test]
fn docs_could_not_run_on_a_markdown_file_that_is_not_utf8() {
    // covers: 0002 AC-8: a file it must read and cannot is a fault, exit 3.
    let f = good_tree("docs_not_utf8");
    f.write_bytes("docs/latin1.md", b"# Caf\xe9\n");
    f.check("docs", &[])
        .assert_not_run("docs", "docs/latin1.md");
}

#[cfg(unix)]
#[test]
fn docs_could_not_run_on_a_markdown_file_without_read_permission() {
    // covers: 0002 AC-8: exit 3 for an unreadable file. Skipped when the process can read it anyway (run as root).
    use std::os::unix::fs::PermissionsExt;
    let f = good_tree("docs_no_read");
    f.write("docs/secret.md", "# Secret\n");
    let path = f.path("docs/secret.md");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).expect("chmod");
    let readable = std::fs::read(&path).is_ok();
    let out = f.check("docs", &[]);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).expect("chmod back");
    if readable {
        return;
    }
    out.assert_not_run("docs", "docs/secret.md");
}

#[cfg(unix)]
#[test]
fn docs_follows_a_symlink_as_a_link_target_but_does_not_read_a_symlinked_file() {
    // covers: 0002 AC-8.
    // Gap: the walk reads only regular files and directories, so a Markdown file reached through a symlink, or a
    // directory that is a symlink, is not checked. A link to a symlink resolves through it, and a dangling symlink is a missing target.
    use std::os::unix::fs::symlink;
    let f = good_tree("docs_symlink");
    f.write("elsewhere/real.md", "# Real\n\n[gone](../missing.md)\n");
    f.write("elsewhere/README.md", "# Elsewhere\n");
    symlink(f.path("elsewhere/real.md"), f.path("docs/alias.md")).expect("symlink");
    symlink(f.path("elsewhere"), f.path("docs/linked-dir")).expect("symlink");
    f.write(
        "notes.md",
        "[alias](docs/alias.md#real) [dir](docs/linked-dir/README.md#elsewhere)\n",
    );
    let out = f.check("docs", &[]);
    // `elsewhere/real.md` is a regular file and is read; its alias is not read a second time.
    out.assert_problem("docs", "D1", &["elsewhere/real.md:3:", "../missing.md"]);
    assert_eq!(out.problems().len(), 1, "{out}");

    symlink(f.path("elsewhere/nothing.md"), f.path("docs/dangling.md")).expect("symlink");
    f.append("notes.md", "[dangling](docs/dangling.md)\n");
    let out = f.check("docs", &[]);
    assert!(out.has("D1", &["notes.md:2:", "docs/dangling.md"]), "{out}");
}

// --- D3: scope status (0002 AC-9) ---

#[test]
fn d3_names_the_status_that_disagrees() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_status");
    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 1. Stack and workspace · in-progress\n",
    );
    f.check("docs", &[]).assert_problem(
        "docs",
        "D3",
        &["docs/scope/001-stack-and-workspace.md", "status"],
    );
}

#[test]
fn d3_names_the_name_and_the_number_that_disagree() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_name");
    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 3. Stack and tools · done\n",
    );
    let out = f.check("docs", &[]);
    out.assert_problem(
        "docs",
        "D3",
        &["001-stack-and-workspace.md", "name disagrees"],
    );
    assert!(
        out.has("D3", &["001-stack-and-workspace.md", "number disagrees"]),
        "{out}"
    );
}

#[test]
fn d3_rejects_a_status_that_is_not_in_the_list() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_unknown");
    f.replace(INDEX, "| 1 | done |", "| 1 | shipped |");
    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 1. Stack and workspace · shipped\n",
    );
    let out = f.check("docs", &[]);
    out.assert_problem("docs", "D3", &["docs/scope/index.md", "shipped"]);
    assert!(
        out.has("D3", &["001-stack-and-workspace.md", "shipped"]),
        "{out}"
    );
}

#[test]
fn d3_names_a_feature_file_without_a_row() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_row");
    f.write(
        "docs/scope/003-first-slice.md",
        "# 3. First slice · planned\n",
    );
    f.check("docs", &[])
        .assert_problem("docs", "D3", &["docs/scope/003-first-slice.md", "no row"]);
}

#[test]
fn d3_ignores_files_in_the_scope_folder_that_are_not_feature_files() {
    // covers: 0002 AC-9, D3: only `<3 digits>-<lowercase>.md` direct children are feature files.
    let f = good_tree("docs_d3_other");
    f.write("docs/scope/coordination/003-notes.md", "# Notes\n");
    f.write("docs/scope/003-Notes.md", "# Notes\n");
    f.write("docs/scope/readme.md", "# Readme\n");
    f.check("docs", &[]).assert_ok("docs");
}

#[test]
fn d3_accepts_a_heading_with_two_tags_and_rejects_an_unknown_tag() {
    // covers: 0002 AC-9, D3: tags follow the status and are not in the index row.
    let f = good_tree("docs_d3_tags");
    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 1. Stack and workspace · done · needs a decision · deferred\n",
    );
    f.check("docs", &[]).assert_ok("docs");

    f.write(
        "docs/scope/001-stack-and-workspace.md",
        "# 1. Stack and workspace · done · blocked\n",
    );
    f.check("docs", &[]).assert_problem(
        "docs",
        "D3",
        &["001-stack-and-workspace.md", "tag blocked"],
    );
}

#[test]
fn d3_names_a_number_used_by_two_rows() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_twice");
    f.replace(INDEX, "| 2 | [Standards", "| 1 | [Standards");
    f.write(
        "docs/scope/002-standards-and-tooling.md",
        "# 1. Standards and tooling · in-progress\n",
    );
    f.check("docs", &[]).assert_problem(
        "docs",
        "D3",
        &["docs/scope/index.md", "number 1 appears in 2 rows"],
    );
}

#[test]
fn d3_names_a_row_whose_file_does_not_exist() {
    // covers: 0002 AC-9, D3
    let f = good_tree("docs_d3_file");
    f.remove("docs/scope/002-standards-and-tooling.md");
    f.check("docs", &[]).assert_problem(
        "docs",
        "D3",
        &[
            "docs/scope/index.md",
            "002-standards-and-tooling.md",
            "does not exist",
        ],
    );
}

#[test]
fn docs_could_not_run_without_the_scope_index() {
    // covers: 0002 AC-9: the index is a file the check must read.
    let f = good_tree("docs_no_index");
    f.remove(INDEX);
    f.check("docs", &[])
        .assert_not_run("docs", "docs/scope/index.md: file is missing");
}

// --- D4 and D5: requirement IDs (0002 AC-10) ---

#[test]
fn d4_names_an_id_defined_twice() {
    // covers: 0002 AC-10, D4
    let f = good_tree("docs_d4");
    f.append(SECURITY, "| SEC-2 | Again |\n");
    let line = f.read(SECURITY).lines().count();
    f.check("docs", &[])
        .assert_problem("docs", "D4", &[&format!("{SECURITY}:{line}:"), "SEC-2"]);
}

#[test]
fn d4_names_a_spec_criterion_defined_twice() {
    // covers: 0002 AC-10, D4: the owner of a criterion is its spec number.
    let f = good_tree("docs_d4_spec");
    f.replace(SPEC, "- **AC-2** (test)", "- **AC-1** (test)");
    f.replace(README, "`0001 AC-2`", "`0001 AC-1`");
    f.check("docs", &[])
        .assert_problem("docs", "D4", &[SPEC, "0001 AC-1"]);
}

#[test]
fn d5_names_a_cited_id_that_has_no_definition() {
    // covers: 0002 AC-10, D5
    let f = good_tree("docs_d5");
    f.append(README, "\nIt also meets SEC-14.\n");
    let line = line_of(&f.read(README), "SEC-14");
    f.check("docs", &[])
        .assert_problem("docs", "D5", &[&format!("README.md:{line}:"), "SEC-14"]);
}

#[test]
fn d5_names_a_citation_of_a_spec_or_prd_that_does_not_exist() {
    // covers: 0002 AC-10, D5
    let f = good_tree("docs_d5_owner");
    f.append(README, "\nSee 0009 AC-1 and PRD-042 FR-1.\n");
    let out = f.check("docs", &[]);
    out.assert_problem("docs", "D5", &["0009 AC-1", "spec 0009 does not exist"]);
    assert!(
        out.has("D5", &["PRD-042 FR-1", "PRD-042 does not exist"]),
        "{out}"
    );
}

#[test]
fn d5_ignores_bare_criteria_word_joined_ids_and_fenced_code() {
    // covers: 0002 AC-10, D5: `AC-3` without a document number, `RUSTSEC-0000-0001` and fenced blocks are not citations.
    let f = good_tree("docs_d5_skip");
    f.append(
        README,
        "\nThis meets AC-3 and RUSTSEC-0000-0001 and XR-9.\n\n```text\nSEC-99 and 0009 AC-1\n```\n",
    );
    f.check("docs", &[]).assert_ok("docs");
}

#[test]
fn d5_treats_a_leading_zero_as_another_id() {
    // covers: 0002 AC-10, D5: IDs are compared as text.
    let f = good_tree("docs_d5_zero");
    f.append(README, "\nSee SEC-01.\n");
    f.check("docs", &[])
        .assert_problem("docs", "D5", &["SEC-01"]);
}

#[test]
fn docs_could_not_run_without_a_table_owner_file() {
    // covers: 0002 AC-10: the four table owner files must exist.
    let f = good_tree("docs_no_owner");
    f.remove("docs/security/threat-model.md");
    f.check("docs", &[])
        .assert_not_run("docs", "docs/security/threat-model.md: file is missing");
}

// --- D6 and --base: the Git revision reader (0002 AC-10) ---

#[test]
fn d6_passes_when_every_base_id_is_still_defined() {
    // covers: 0002 AC-10, D6
    let f = good_tree("docs_d6_same");
    let base = f.commit_all("base");
    f.check("docs", &["--base", &base]).assert_ok("docs");
    f.check("docs", &["--base", "HEAD"]).assert_ok("docs");
}

#[test]
fn d6_names_an_id_that_the_base_revision_defines_and_the_tree_does_not() {
    // covers: 0002 AC-10, D6
    let f = good_tree("docs_d6_gone");
    let base = f.commit_all("base");
    f.replace(SECURITY, "| SEC-2 | Second |\n", "");
    f.check("docs", &["--base", &base]).assert_problem(
        "docs",
        "D6",
        &[SECURITY, "SEC-2", "base revision"],
    );
}

#[test]
fn d6_compares_with_the_base_revision_and_not_the_working_tree_or_a_later_commit() {
    // covers: 0002 AC-10, D6: the base is read with git ls-tree and git show at that revision.
    let f = good_tree("docs_d6_rev");
    f.append(SECURITY, "| SEC-3 | Third |\n");
    let first = f.commit_all("with SEC-3");
    f.replace(SECURITY, "| SEC-3 | Third |\n", "");
    let second = f.commit_all("without SEC-3");
    f.check("docs", &["--base", &second]).assert_ok("docs");
    f.check("docs", &["--base", &first])
        .assert_problem("docs", "D6", &["SEC-3"]);
}

#[test]
fn d6_names_a_criterion_whose_spec_or_prd_moved_away() {
    // covers: 0002 AC-10, D6: the same ID must stay defined under the same spec number.
    let f = good_tree("docs_d6_spec");
    let base = f.commit_all("base");
    f.replace(SPEC, "- **AC-2** (test): it tests.\n", "");
    f.replace(README, "`0001 AC-2`", "`0001 AC-1`");
    f.check("docs", &["--base", &base])
        .assert_problem("docs", "D6", &["0001 AC-2"]);
}

#[test]
fn d6_names_a_positional_list_that_got_shorter() {
    // covers: 0002 AC-10, D6: a safety contract or functional requirement list must not shrink.
    let f = good_tree("docs_d6_list");
    let base = f.commit_all("base");
    f.replace(SPEC, "2. Close uncertain state.\n", "");
    f.replace(README, " and 0001 SC-2", "");
    let out = f.check("docs", &["--base", &base]);
    out.assert_problem("docs", "D6", &["0001 SC-2"]);
    assert!(
        out.has(
            "D6",
            &[SPEC, "0001 SC list has 1 items", "base revision has 2"]
        ),
        "{out}"
    );
}

#[test]
fn d6_reads_an_id_file_that_the_base_had_and_the_tree_lost() {
    // covers: 0002 AC-10, D6: a deleted PRD takes its IDs with it.
    let f = good_tree("docs_d6_prd");
    let base = f.commit_all("base");
    f.remove("docs/prd/001-proxy.md");
    f.replace(README, ", PRD-001 AC-1, PRD-001 FR-2", "");
    let out = f.check("docs", &["--base", &base]);
    out.assert_problem("docs", "D6", &["PRD-001 AC-1"]);
    assert!(out.has("D6", &["PRD-001 FR-2"]), "{out}");
}

#[test]
fn base_could_not_run_for_a_revision_that_does_not_exist() {
    // covers: 0002 AC-10: `git` failed for --base, exit 3.
    let f = good_tree("docs_base_missing");
    f.commit_all("base");
    f.check("docs", &["--base", "no-such-revision"])
        .assert_not_run("docs", "git rev-parse: no-such-revision is not a commit");
}

#[test]
fn base_refuses_a_revision_that_starts_with_a_hyphen_as_not_a_commit() {
    // covers: 0002 AC-10: the revision is resolved with `git rev-parse --end-of-options` first, so a value that
    // starts with a hyphen is never read as an option of `git ls-tree`; it is not a commit, exit 3.
    let f = good_tree("docs_base_hyphen");
    f.commit_all("base");
    for revision in ["-h", "--all", "--full-tree"] {
        f.check("docs", &["--base", revision]).assert_not_run(
            "docs",
            &format!("git rev-parse: {revision} is not a commit"),
        );
    }
}

#[test]
fn base_could_not_run_when_the_root_is_not_a_git_repository() {
    // covers: 0002 AC-10: exit 3 when `git` cannot find a repository at the root.
    let f = good_tree("docs_base_no_repo");
    let ceiling = f.root.parent().expect("the fixture has a parent");
    let root = f.root.to_str().expect("UTF-8 path");
    let out = run(
        &["docs", "--root", root, "--base", "HEAD"],
        &[(
            "GIT_CEILING_DIRECTORIES",
            ceiling.to_str().expect("UTF-8 path"),
        )],
    );
    out.assert_not_run("docs", "git rev-parse");
}

#[test]
fn base_could_not_run_when_the_root_is_a_folder_inside_a_repository() {
    // covers: 0002 AC-10: `--base` needs the top of the work tree. An untracked folder inside another repository
    // would list nothing at the base and pass D6 unseen, so it exits 3; a tracked folder exits 3 the same way.
    let outer = Fixture::new("docs_base_outer");
    outer.write("README.md", "outer\n");
    outer.commit_all("outer");
    let inner = good_tree("docs_base_outer/inner");
    inner.replace(SECURITY, "| SEC-2 | Second |\n", "");
    inner.check("docs", &["--base", "HEAD"]).assert_not_run(
        "docs",
        "is the folder inner/ of a Git work tree, not its top",
    );

    outer.commit_all("track inner");
    inner
        .check("docs", &["--base", "HEAD"])
        .assert_not_run("docs", "--base needs the repository root");
}

#[test]
fn base_could_not_run_when_an_owner_file_at_the_base_cannot_be_read() {
    // covers: 0002 AC-10, D6: a base whose owner file is not UTF-8 cannot be compared, so the run exits 3
    // instead of treating the file as absent and passing (review M5).
    let f = good_tree("docs_base_not_utf8");
    let good = f.read(SECURITY);
    f.write_bytes(SECURITY, b"| ID |\n|---|\n| SEC-1 | Caf\xe9 |\n");
    let base = f.commit_all("base with a latin-1 owner file");
    f.write(SECURITY, &good);
    f.check("docs", &["--base", &base])
        .assert_not_run("docs", "cannot read the base revision");
}
