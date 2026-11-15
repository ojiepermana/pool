//! `pool-repo-check workspace` on trees on disk: rules W1 to W7 (0002 AC-6).

use crate::common::{Fixture, line_of};

const SCRIPT: &str = "scripts/linux-build.sh";
const SPEC_0001: &str = "docs/specs/0001-initial-architecture/index.md";

/// Builds a small workspace that passes every rule: two members, three lockfiles and their baselines, and a Linux build script that matches its spec block.
fn good_tree(name: &str) -> Fixture {
    let f = Fixture::new(name);
    let baseline = "tools/repo-check/baseline";
    f.write(
        &format!("{baseline}/lock-packages.txt"),
        "# spec 0002\npool\npool-config\nserde\n",
    );
    f.write(
        &format!("{baseline}/fuzz-lock-packages.txt"),
        "# spec 0002\n\npool-config\npool-config-fuzz\n",
    );
    f.write(
        &format!("{baseline}/tool-lock-packages.txt"),
        "# spec 0002\npool-repo-check\n",
    );
    f.write(
        &format!("{baseline}/protocol-fuzz-lock-packages.txt"),
        "# spec 0004\npool-protocol\npool-protocol-fuzz\n",
    );
    f.write(
        "crates/pool-protocol/fuzz/Cargo.lock",
        "version = 4\n\n[[package]]\nname = \"pool-protocol\"\n\n[[package]]\nname = \"pool-protocol-fuzz\"\n",
    );
    f.write(
        &format!("{baseline}/forbidden-paths.txt"),
        "# spec 0002\ncrates/pool-core\napps/pool-cli\n",
    );
    f.write(
        &format!("{baseline}/forbidden-source-patterns.txt"),
        "# spec 0002\nstd::thread\nTcpListener\n",
    );
    f.write(
        &format!("{baseline}/internal-dependencies.txt"),
        "# spec 0002\npool -> pool-config\n",
    );
    f.write(
        "Cargo.lock",
        "version = 4\n\n[[package]]\nname = \"pool\"\nversion = \"0.0.0\"\n\n[[package]]\nname = \"pool-config\"\nversion = \"0.0.0\"\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );
    f.write(
        "crates/pool-config/fuzz/Cargo.lock",
        "version = 4\n\n[[package]]\nname = \"pool-config\"\n\n[[package]]\nname = \"pool-config-fuzz\"\n",
    );
    f.write(
        "tools/repo-check/Cargo.lock",
        "version = 4\n\n[[package]]\nname = \"pool-repo-check\"\nversion = \"0.0.0\"\n",
    );
    f.write(
        "Cargo.toml",
        "[workspace]\nresolver = \"3\"\nmembers = [\n  \"apps/pool\",\n  \"crates/pool-config\",\n]\n\n[workspace.lints.rust]\nunsafe_code = \"forbid\"\n",
    );
    f.write(
        "apps/pool/Cargo.toml",
        "[package]\nname = \"pool\"\nversion = \"0.0.0\"\n\n[dependencies]\npool-config = { path = \"../../crates/pool-config\" }\nserde = \"1\"\n\n[lints]\nworkspace = true\n",
    );
    f.write("apps/pool/src/main.rs", "fn main() {}\n");
    f.write(
        "crates/pool-config/Cargo.toml",
        "[package]\nname = \"pool-config\"\nversion = \"0.0.0\"\n\n[dependencies]\nserde = \"1\"\n\n[lints]\nworkspace = true\n",
    );
    f.write("crates/pool-config/src/lib.rs", "//! Config.\n");
    f.write(
        SCRIPT,
        "#!/bin/sh\n# Linux build check.\n# --- spec 0001, Linux build ---\ndocker run --rm image cargo test\necho linux build check passed\n",
    );
    f.write(
        SPEC_0001,
        "# 0001\n\n### Linux build\n\nRun this.\n\n```sh\ndocker run --rm image cargo test\necho linux build check passed\n```\n\n```sh\nnot this block\n```\n",
    );
    f
}

#[test]
fn workspace_passes_on_a_small_correct_tree() {
    // covers: 0002 AC-6
    let f = good_tree("ws_good");
    f.check("workspace", &[]).assert_ok("workspace");
}

#[test]
fn w1_names_a_package_that_the_lockfile_has_and_the_baseline_does_not() {
    // covers: 0002 AC-6, W1
    let f = good_tree("ws_w1_extra");
    f.append(
        "Cargo.lock",
        "\n[[package]]\nname = \"zmij\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );
    f.check("workspace", &[])
        .assert_problem("workspace", "W1", &["Cargo.lock", "zmij"]);
}

#[test]
fn w1_names_a_baseline_package_missing_from_the_fuzz_lockfile() {
    // covers: 0002 AC-6, W1
    let f = good_tree("ws_w1_missing");
    f.replace(
        "crates/pool-config/fuzz/Cargo.lock",
        "\n[[package]]\nname = \"pool-config-fuzz\"\n",
        "",
    );
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W1",
        &["crates/pool-config/fuzz/Cargo.lock", "pool-config-fuzz"],
    );
}

#[test]
fn w1_counts_a_package_that_appears_twice() {
    // covers: 0002 AC-6, W1: names are compared as a sorted list, so a second copy is an extra package.
    let f = good_tree("ws_w1_twice");
    f.append(
        "Cargo.lock",
        "\n[[package]]\nname = \"serde\"\nversion = \"2.0.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
    );
    f.check("workspace", &[])
        .assert_problem("workspace", "W1", &["Cargo.lock", "serde"]);
}

#[test]
fn w1_rejects_a_second_package_in_the_tool_lockfile() {
    // covers: 0002 AC-6, W1 on tools/repo-check/Cargo.lock, which keeps this package free of dependencies.
    let f = good_tree("ws_w1_tool");
    f.append(
        "tools/repo-check/Cargo.lock",
        "\n[[package]]\nname = \"regex\"\n",
    );
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W1",
        &["tools/repo-check/Cargo.lock", "regex"],
    );
}

#[test]
fn w2_names_a_forbidden_directory_that_exists() {
    // covers: 0002 AC-6, W2
    let f = good_tree("ws_w2");
    f.write("crates/pool-core/README.md", "later\n");
    f.check("workspace", &[])
        .assert_problem("workspace", "W2", &["crates/pool-core"]);
}

#[test]
fn w3_names_the_file_line_and_text_of_a_forbidden_pattern() {
    // covers: 0002 AC-6, W3
    let f = good_tree("ws_w3");
    let lib = "//! Config.\nuse std::thread;\n";
    f.write("crates/pool-config/src/lib.rs", lib);
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W3",
        &["crates/pool-config/src/lib.rs:2:", "std::thread"],
    );
}

#[test]
fn w3_counts_a_match_in_a_comment_and_skips_target_and_files_that_are_not_rust() {
    // covers: 0002 AC-6, W3: plain text, comments count; `target` directories and other file types are skipped.
    let f = good_tree("ws_w3_scope");
    f.write("apps/pool/src/net.rs", "// a TcpListener comes later\n");
    f.write("apps/pool/target/debug/gen.rs", "use std::thread;\n");
    f.write("crates/pool-config/notes.md", "std::thread\n");
    f.write("tools/other/src/lib.rs", "use std::thread;\n");
    let out = f.check("workspace", &[]);
    out.assert_problem(
        "workspace",
        "W3",
        &["apps/pool/src/net.rs:1:", "TcpListener"],
    );
    assert_eq!(out.problems().len(), 1, "only the comment counts:\n{out}");
}

#[test]
fn w4_names_a_member_manifest_without_lint_inheritance() {
    // covers: 0002 AC-6, W4
    let f = good_tree("ws_w4_member");
    f.replace("apps/pool/Cargo.toml", "[lints]\nworkspace = true\n", "");
    f.check("workspace", &[])
        .assert_problem("workspace", "W4", &["apps/pool/Cargo.toml"]);
}

#[test]
fn w4_names_the_root_manifest_when_unsafe_code_is_not_forbidden() {
    // covers: 0002 AC-6, W4
    let f = good_tree("ws_w4_root");
    f.replace(
        "Cargo.toml",
        "unsafe_code = \"forbid\"",
        "unsafe_code = \"deny\"",
    );
    f.check("workspace", &[])
        .assert_problem("workspace", "W4", &["Cargo.toml", "unsafe_code"]);
}

#[test]
fn w4_rejects_a_wildcard_member() {
    // covers: 0002 AC-6, W4: the guard cannot follow `*`.
    let f = good_tree("ws_w4_glob");
    f.replace("Cargo.toml", "\"crates/pool-config\"", "\"crates/*\"");
    f.check("workspace", &[])
        .assert_problem("workspace", "W4", &["Cargo.toml", "crates/*"]);
}

#[test]
fn w4_reads_a_lints_line_with_a_trailing_comment() {
    // covers: 0002 AC-6, W4: `workspace = true # comment` is valid TOML and inherits the lints, so it passes (review m13).
    let f = good_tree("ws_w4_comment");
    f.replace(
        "crates/pool-config/Cargo.toml",
        "workspace = true\n",
        "workspace = true # inherit\n",
    );
    f.check("workspace", &[]).assert_ok("workspace");

    f.replace(
        "crates/pool-config/Cargo.toml",
        "workspace = true # inherit\n",
        "workspace = false # inherit\n",
    );
    f.check("workspace", &[])
        .assert_problem("workspace", "W4", &["crates/pool-config/Cargo.toml"]);
}

#[test]
fn w5_names_a_dependency_pair_that_is_not_allowed() {
    // covers: 0002 AC-6, W5
    let f = good_tree("ws_w5_extra");
    f.replace(
        "crates/pool-config/Cargo.toml",
        "serde = \"1\"\n",
        "serde = \"1\"\npool = { path = \"../../apps/pool\" }\n",
    );
    let manifest = f.read("crates/pool-config/Cargo.toml");
    let line = line_of(&manifest, "pool = {");
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W5",
        &[
            &format!("crates/pool-config/Cargo.toml:{line}:"),
            "pool-config -> pool",
            "not allowed",
        ],
    );
}

#[test]
fn w5_names_a_listed_pair_that_no_manifest_declares() {
    // covers: 0002 AC-6, W5
    let f = good_tree("ws_w5_missing");
    f.replace(
        "apps/pool/Cargo.toml",
        "pool-config = { path = \"../../crates/pool-config\" }\n",
        "",
    );
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W5",
        &[
            "internal-dependencies.txt",
            "pool -> pool-config",
            "missing",
        ],
    );
}

#[test]
fn w5_reads_dependency_tables_in_every_spelling() {
    // covers: 0002 AC-6, W5: `[dev-dependencies]`, `[target.<cfg>.dependencies]` and `[build-dependencies.<key>]` all count.
    let f = good_tree("ws_w5_tables");
    f.append(
        "crates/pool-config/Cargo.toml",
        "\n[dev-dependencies]\npool.path = \"../../apps/pool\"\n\n[target.'cfg(unix)'.dependencies]\npool = { path = \"../../apps/pool\" }\n\n[build-dependencies.pool]\npath = \"../../apps/pool\"\n",
    );
    let out = f.check("workspace", &[]);
    out.assert_problem("workspace", "W5", &["pool-config -> pool"]);
    assert_eq!(out.problems().len(), 3, "one problem per table:\n{out}");
}

#[test]
fn w5_follows_a_member_dependency_under_another_key() {
    // covers: 0002 AC-6, W5: a member renamed with `package = "pool"` is still the package pool (review M3).
    let f = good_tree("ws_w5_rename");
    f.replace(
        "crates/pool-config/Cargo.toml",
        "serde = \"1\"\n",
        "serde = \"1\"\ndaemon = { package = \"pool\", path = \"../../apps/pool\" }\n",
    );
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W5",
        &[
            "crates/pool-config/Cargo.toml",
            "pool-config -> pool",
            "not allowed",
        ],
    );

    f.replace(
        "crates/pool-config/Cargo.toml",
        "daemon = { package = \"pool\", path = \"../../apps/pool\" }\n",
        "",
    );
    f.replace(
        "apps/pool/Cargo.toml",
        "pool-config = {",
        "config = { package = \"pool-config\",",
    );
    f.check("workspace", &[]).assert_ok("workspace");
}

#[test]
fn w5_sees_a_dependency_under_a_header_with_a_comment() {
    // covers: 0002 AC-6, W5: `[dependencies] # runtime` is still the dependency table (review M3).
    let f = good_tree("ws_w5_header_comment");
    f.replace(
        "crates/pool-config/Cargo.toml",
        "[dependencies]\n",
        "[dependencies] # runtime\npool = { path = \"../../apps/pool\" }\n",
    );
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W5",
        &["pool-config -> pool", "not allowed"],
    );
}

#[test]
fn w5_sees_a_quoted_key_and_a_target_table_for_one_dependency() {
    // covers: 0002 AC-6, W5: `"pool" = {...}` and `[target.'cfg(unix)'.dependencies.pool]` (review M3).
    let f = good_tree("ws_w5_quoted");
    f.replace(
        "crates/pool-config/Cargo.toml",
        "serde = \"1\"\n",
        "serde = \"1\"\n\"pool\" = { path = \"../../apps/pool\" }\n",
    );
    f.append(
        "crates/pool-config/Cargo.toml",
        "\n[target.'cfg(unix)'.dependencies.pool]\npath = \"../../apps/pool\"\n",
    );
    let out = f.check("workspace", &[]);
    out.assert_problem("workspace", "W5", &["pool-config -> pool"]);
    assert_eq!(out.problems().len(), 2, "one problem per spelling:\n{out}");
}

#[test]
fn w6_names_a_lockfile_that_is_not_one_of_the_three() {
    // covers: 0002 AC-6, W6
    let f = good_tree("ws_w6");
    f.write("crates/pool-config/extra/Cargo.lock", "version = 4\n");
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W6",
        &[
            "crates/pool-config/extra/Cargo.lock",
            "unexpected Cargo.lock",
        ],
    );
}

#[test]
fn w6_skips_target_and_dot_directories() {
    // covers: 0002 AC-6, W6
    let f = good_tree("ws_w6_skip");
    f.write("target/package/x/Cargo.lock", "version = 4\n");
    f.write("apps/pool/target/Cargo.lock", "version = 4\n");
    f.write(".cache/Cargo.lock", "version = 4\n");
    f.check("workspace", &[]).assert_ok("workspace");
}

#[test]
fn w7_names_the_first_script_line_that_differs_from_the_spec_block() {
    // covers: 0002 AC-6, W7
    let f = good_tree("ws_w7");
    f.replace(SCRIPT, "echo linux build check passed", "echo passed");
    let line = line_of(&f.read(SCRIPT), "echo passed");
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W7",
        &[&format!("scripts/linux-build.sh:{line}:")],
    );
}

#[test]
fn w7_reports_an_extra_line_after_the_copy() {
    // covers: 0002 AC-6, W7: the copy must end where the block ends.
    let f = good_tree("ws_w7_extra");
    f.append(SCRIPT, "rm -rf target\n");
    let line = line_of(&f.read(SCRIPT), "rm -rf target");
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W7",
        &[&format!("scripts/linux-build.sh:{line}:")],
    );
}

#[test]
fn w7_reports_a_missing_marker_line() {
    // covers: 0002 AC-6, W7
    let f = good_tree("ws_w7_marker");
    f.replace(SCRIPT, "# --- spec 0001, Linux build ---\n", "");
    f.check("workspace", &[]).assert_problem(
        "workspace",
        "W7",
        &["scripts/linux-build.sh", "marker"],
    );
}

#[test]
fn w7_reports_a_spec_without_the_linux_build_block() {
    // covers: 0002 AC-6, W7
    let f = good_tree("ws_w7_block");
    f.write(SPEC_0001, "# 0001\n\n### Linux build\n\nNo block yet.\n");
    f.check("workspace", &[])
        .assert_problem("workspace", "W7", &[SPEC_0001]);
}

#[test]
fn w7_ignores_one_missing_trailing_newline() {
    // covers: 0002 AC-6, W7
    let f = good_tree("ws_w7_newline");
    let script = f.read(SCRIPT);
    f.write(SCRIPT, script.trim_end_matches('\n'));
    f.check("workspace", &[]).assert_ok("workspace");
}

#[test]
fn workspace_could_not_run_without_a_baseline_file() {
    // covers: 0002 AC-6: a file the check must read is missing, exit 3.
    let f = good_tree("ws_no_baseline");
    f.remove("tools/repo-check/baseline/forbidden-paths.txt");
    f.check("workspace", &[])
        .assert_not_run("workspace", "forbidden-paths.txt: file is missing");
}

#[test]
fn workspace_could_not_run_on_a_source_file_that_is_not_utf8() {
    // covers: 0002 AC-6: an unreadable file is a fault, exit 3, and the problems found before it are still printed.
    let f = good_tree("ws_not_utf8");
    f.write_bytes("crates/pool-config/src/bad.rs", b"// \xff\xfe\n");
    f.write("crates/pool-core/README.md", "later\n");
    let out = f.check("workspace", &[]);
    out.assert_not_run("workspace", "crates/pool-config/src/bad.rs");
    assert!(out.has("W2", &["crates/pool-core"]), "{out}");
}

#[test]
fn workspace_reports_every_problem_before_the_count() {
    // covers: 0002 AC-6: one line per problem, then `<n> problems`.
    let f = good_tree("ws_many");
    f.write("crates/pool-core/README.md", "later\n");
    f.write("apps/pool-cli/README.md", "later\n");
    f.write("crates/pool-config/src/lib.rs", "use std::thread;\n");
    let out = f.check("workspace", &[]);
    out.assert_problem("workspace", "W2", &["apps/pool-cli"]);
    assert_eq!(out.last_line(), "pool-repo-check workspace: 3 problems");
}

// ---- added by the tests of scope 004 (`/test one real primary path`) ----

/// The repository this program lives in.
fn repository(path: &str) -> String {
    std::fs::read_to_string(format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The lines of a baseline file that count: not empty, not a comment.
fn baseline_lines(text: &str) -> Vec<&str> {
    text.lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

#[test]
fn ac2_the_committed_w3_baseline_is_the_block_of_spec_0004() {
    // covers: 0004 AC-2. The block after "The file of this slice:" under "Source guard".
    let spec = repository("docs/specs/0004-one-real-primary-path/index.md");
    let section = spec
        .split_once("\n### Source guard\n")
        .expect("the section")
        .1;
    let block = section
        .split_once("The file of this slice:\n\n```text\n")
        .expect("the block")
        .1
        .split_once("\n```")
        .expect("the end of the block")
        .0;
    let committed = repository("tools/repo-check/baseline/forbidden-source-patterns.txt");
    assert_eq!(baseline_lines(&committed), baseline_lines(block));
}

#[test]
fn ac2_the_committed_w3_baseline_allows_and_refuses_each_text_where_the_spec_says() {
    // covers: 0004 AC-2, through the real binary on a tree with the committed baseline.
    let f = good_tree("ws_ac2_matrix");
    f.write(
        "tools/repo-check/baseline/forbidden-source-patterns.txt",
        &repository("tools/repo-check/baseline/forbidden-source-patterns.txt"),
    );
    // (file, text, refused)
    let cases: &[(&str, &str, bool)] = &[
        // Sockets and the runtime: allowed in pool-core and apps/pool only.
        ("crates/pool-core/src/a.rs", "tokio", false),
        ("crates/pool-core/tests/a.rs", "TcpStream", false),
        ("crates/pool-core/src/b.rs", "std::net", false),
        ("apps/pool/src/a.rs", "TcpListener", false),
        ("apps/pool/tests/a.rs", "tokio", false),
        ("crates/pool-config/src/a.rs", "tokio", true),
        ("crates/pool-config/tests/a.rs", "TcpStream", true),
        ("crates/pool-control/src/a.rs", "TcpListener", true),
        ("crates/pool-control/tests/a.rs", "tokio", true),
        ("crates/pool-protocol/src/a.rs", "std::net", true),
        ("crates/pool-protocol/fuzz/fuzz_targets/a.rs", "tokio", true),
        // Paths are whole components.
        ("crates/pool-core-x/src/lib.rs", "tokio", true),
        ("apps/pool-cli/src/main.rs", "TcpStream", true),
        // Rejected everywhere.
        ("crates/pool-core/src/c.rs", "std::thread", true),
        ("apps/pool/src/b.rs", "UdpSocket", true),
        ("apps/pool/tests/b.rs", "UnixListener", true),
        ("crates/pool-core/src/d.rs", "UnixStream", true),
        // No clock, file or environment under pool-control/src; its tests may.
        ("crates/pool-control/src/b.rs", "Instant::now", true),
        ("crates/pool-control/src/c.rs", "SystemTime", true),
        ("crates/pool-control/src/d.rs", "std::fs", true),
        ("crates/pool-control/src/e.rs", "std::env", true),
        ("crates/pool-control/tests/b.rs", "std::fs", false),
        ("crates/pool-core/src/e.rs", "std::fs", false),
        // rusqlite: journal.rs only, within pool-control/src.
        ("crates/pool-control/src/journal.rs", "rusqlite", false),
        ("crates/pool-control/src/f.rs", "rusqlite", true),
        ("crates/pool-control/tests/c.rs", "rusqlite", false),
        // The five names of pool-control, refused outside it.
        ("crates/pool-control/src/g.rs", "StoredAuthority", false),
        ("crates/pool-control/tests/d.rs", "writer_admission", false),
        ("apps/pool/src/c.rs", "StoredAuthority", true),
        ("apps/pool/tests/c.rs", "StoreProbe", true),
        ("crates/pool-core/src/f.rs", "classify_startup", true),
        ("crates/pool-config/src/b.rs", "plan_bootstrap", true),
        ("crates/pool-protocol/src/b.rs", "writer_admission", true),
    ];
    for (file, text, _) in cases {
        f.write(file, &format!("// {text}\n"));
    }
    let out = f.check("workspace", &[]);
    for (file, text, refused) in cases {
        assert_eq!(
            out.has("W3", &[&format!("{file}:"), text]),
            *refused,
            "{file} with {text}:\n{out}"
        );
    }
    let w3 = out
        .problems()
        .iter()
        .filter(|l| l.starts_with("W3: "))
        .count();
    assert_eq!(
        w3,
        cases.iter().filter(|c| c.2).count(),
        "one W3 problem per refused case:\n{out}"
    );
}
