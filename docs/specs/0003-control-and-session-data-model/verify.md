# 0003. Verification record

This file holds the acceptance evidence for [spec 0003](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

## Milestone 1: Workspace member and guards

Build plan step 1, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commit `8309282` (step 1), on top of the accepted spec at `dc04352`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Docker was running (`docker info` exit 0).

### UI / manual

None for this milestone.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo metadata --no-deps --format-version 1` | three members, `pool`, `pool-config`, `pool-control`; `pool-control` depends only on `pool-config` | `pool` (`pool-config`, `serde`, `serde_json`), `pool-config` (`serde`, `toml`), `pool-control` (`pool-config`) | pass |
| `Cargo.lock` compared with `main` | one new entry, `pool-control`, no source line; the sixteen third party packages unchanged | one new entry `pool-control 0.1.0` with the dependency `pool-config`; 19 entries in all, 3 workspace and 16 from the registry | pass |
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` | exit 0 | exit 0, 95 unit and 110 integration tests passed, 0 failed | pass |
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok` (see the note below) | pass |
| `scripts/prove-checks.sh --with-linux-build` on commit `8309282` | every case rejects its change | `all checks reject their bad change`, cases 1 to 28 `PASS`, case 17 now on `docs/scope/004-one-real-primary-path.md`, case 20 run with Docker | pass |

**Note on the cargo warning.** Cargo prints `warning: unused workspace dependency pool-control` for the root `Cargo.toml`. The spec asks for the `[workspace.dependencies]` entry in this step and says that `apps/pool` takes the dependency in scope 004. The warning is from Cargo's manifest reader, not from rustc or Clippy, so `-D warnings` does not turn it into an error, and every check passes. It goes away when a member takes `pool-control.workspace = true`.

**Note on case 17.** The scope 003 heading on this branch is still `planned`. The case now rewrites the first line of the scope 004 file, which stays `planned` while feature 3 moves to `in-progress`, so the case changes the file in both states.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0003 AC-13` | `cargo metadata` and `Cargo.lock` rows above; baselines `lock-packages.txt` (adds `pool-control`), `forbidden-paths.txt` (drops `crates/pool-control`), `internal-dependencies.txt` (adds `pool-control -> pool-config`); W4 tests `w4_lockfile_package_without_source_that_is_a_member_passes` and `w4_lockfile_package_without_source_that_is_not_a_member_fails` in `tools/repo-check/src/workspace.rs`; `scripts/check.sh all` and `scripts/prove-checks.sh` pass | pass on the workstation; CI evidence waits for the pull request |
| `0003 AC-1` to `0003 AC-12`, `0003 AC-14` | Build plan steps 2 to 7, other milestones | not run |

### Checker run: `/check verify`, milestone 1

Recorded by the checker session `chk-f3-verify` on 2026-11-15 at revision `60fcb08`, on macOS (workstation, not Linux and not CI). The code of step 1 is commit `8309282`, and `60fcb08` adds only this file. The checker did not write the code. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Docker was running (`docker info` exit 0). Every probe that changes a file ran in a private clone of `60fcb08` outside the repository, and the clone was removed afterward.

**Verdict: PASS for milestone 1.** It covers `0003 AC-13` on the workstation. CI evidence waits for the pull request. `0003 AC-1` to `0003 AC-12` and `0003 AC-14` belong to other milestones and were not run.

**The five commands of the record**

- [x] `cargo metadata --no-deps --format-version 1` → exit 0, three workspace members. `pool` depends on `pool-config`, `serde` and `serde_json`. `pool-config` depends on `serde` and `toml`. `pool-control` depends on `pool-config` and on nothing else → `0003 AC-13`
- [x] `Cargo.lock` compared with `git show main:Cargo.lock`, entry by entry → 18 entries before, 19 now. The one new entry is `pool-control 0.1.0` with no `source` line and the dependency `pool-config`. No entry was removed. The sixteen registry entries are the same by name, version, source and checksum → `0003 AC-13`
- [x] `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` → exit 0, 95 unit and 110 integration tests passed, 0 failed. `w4_lockfile_package_without_source_that_is_a_member_passes` and `w4_lockfile_package_without_source_that_is_not_a_member_fails` are both in the list as `ok` → `0003 AC-13`
- [x] `scripts/check.sh all` → exit 0, `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. Cargo prints two warnings, see the readings below → `0003 AC-13`
- [x] `scripts/prove-checks.sh --with-linux-build` at `60fcb08` → exit 0, `all checks reject their bad change`, cases 1 to 28 `PASS`. Case 17 matched `D3: docs/scope/004-one-real-primary-path.md:1: status disagrees: done in the heading, planned in docs/scope/index.md`. Case 20 ran with Docker → `0003 AC-13`

**Extra checks**

- [x] `git diff main...HEAD -- tools/repo-check/baseline` → three files, two lines added and one removed: `pool-control` in `lock-packages.txt`, `pool-control -> pool-config` in `internal-dependencies.txt`, and `crates/pool-control` gone from `forbidden-paths.txt`. `forbidden-source-patterns.txt` is unchanged → `0003 AC-13`
- [x] W4 message probe: an entry `ghost` with no `source` line added to `Cargo.lock`, and `ghost` added to `lock-packages.txt` so that only W4 can object. `pool-repo-check workspace` → exit 1, one problem: `W4: Cargo.lock: package ghost has no source line but is not a member written in members of Cargo.toml`. The message names `Cargo.lock` and the package, as spec 0003 and the note on W4 under "Workspace guards" of spec 0002 ask → `0003 AC-13`
- [x] W4 probe with the real case: a new package `crates/extra` that is not written in `members`, taken by `pool-control` with a path dependency. `cargo metadata --offline --format-version 1` wrote the entry `extra` with no `source` line into `Cargo.lock`, and Cargo then lists four workspace members. `pool-repo-check workspace` → exit 1, two problems: `W1: Cargo.lock: package extra is in the lockfile but not in the baseline lock-packages.txt` and `W4: Cargo.lock: package extra has no source line but is not a member written in members of Cargo.toml`. This is the gap of Follow-up 10 of spec 0002, and the guard now sees it → `0003 AC-13`
- [x] Missing lockfile probe: `Cargo.lock` removed → exit 3, `pool-repo-check workspace: could not run: Cargo.lock: file is missing`, and no W4 line → `0003 AC-13`
- [x] The older rules on the new member, one probe each → the `[lints]` table removed from `crates/pool-control/Cargo.toml` gives `W4: crates/pool-control/Cargo.toml: the manifest has no [lints] table with workspace = true`. The line `pool-control -> pool-config` removed from the baseline gives `W5: crates/pool-control/Cargo.toml:10: dependency pool-control -> pool-config is not allowed by internal-dependencies.txt`. `pool-control.workspace = true` added to `apps/pool/Cargo.toml` gives `W5: apps/pool/Cargo.toml:11: dependency pool -> pool-control is not allowed by internal-dependencies.txt`. `crates/pool-control` put back into `forbidden-paths.txt` gives `W2: crates/pool-control: forbidden path crates/pool-control exists`. Exit 1 each time → `0003 AC-13`
- [x] Case 17 in the second state: in the private clone, the heading of the scope 003 file and its row in `docs/scope/index.md` were set to `in-progress` and committed there only. `scripts/prove-checks.sh` on that commit → exit 0, `all checks reject their bad change (linux-build not proven)`, cases 1 to 19 and 21 to 28 `PASS`, and case 17 matched the same D3 line as above. So the case holds with feature 3 `planned` and with feature 3 `in-progress` → `0003 AC-13`
- [x] `crates/pool-control/src/lib.rs` → eleven lines of crate documentation and one plain comment line, no item. `apps/pool/Cargo.toml` does not name `pool-control`. Line 4 of the spec is `**Status**: In Progress`

**Readings the checker assessed**

- **Two Cargo warnings, not one.** `scripts/check.sh all` prints `warning: unused workspace dependency pool-control` for `Cargo.toml:14`, which the developer's note covers, and also `warning: unused dependency pool-config` for `crates/pool-control/Cargo.toml:10`, which the note does not name. Both come from Cargo's own manifest lints (`cargo::unused_workspace_dependencies` and `cargo::unused_dependencies`, `warn` by default). The `-D warnings` of the `clippy` check goes to rustc and Clippy only, so the exit status stays 0. Both manifest lines are written decisions of this spec: [Workspace and guards](index.md#workspace-and-guards) asks for the root entry now and says that scope 004 adds the pair for `apps/pool`, and `0003 AC-13` asks that `pool-control` depend on `pool-config`. The rule under "Adopted dependencies" of spec 0001 lists third party packages, and none was added. The checker reads this as no breach of `0003 AC-13`. The second warning should end when the crate's code first uses `pool-config`, and the first one in scope 004. Until then CI will print both.
- **The lockfile check when `Cargo.lock` is missing.** The new check stays quiet when the file is missing or cannot be read. The probe above shows that the program then stops with `could not run` and exit 3 from the read of rule W1, so the fault is reported once. The checker accepts this reading.
- **The `source` line in the integration test helper.** The spec names one test helper. There are two, `lock` beside the unit tests and `good_tree` in `tests/repo_check/workspace.rs`. Without the `source` line in the second one its good tree would fail the new check. The change follows the purpose the spec gives ("so that the existing good tree still passes"), and all 110 integration tests pass. The checker accepts this reading.
- **The comment line in `lib.rs`.** After the crate documentation the file has one line, `// provisional: F3 (Build plan step 5 of spec 0003 adds the authority model)`. It is a comment, not code, so the file still holds no item. No document of this repository defines a `provisional:` marker. The checker reads the Build plan wording ("holds only the crate documentation") as met, and leaves the marker to the review.
- **Where a lockfile entry ends.** The code follows the spec: an entry runs to the next `[[package]]` line or to the end of the file. A table that Cargo writes after the last package, such as `[[patch.unused]]` with its own `source` line, would count as part of the last entry. Today's lockfile has no such table and its last entry is a registry package. This is a note for the review, not a failed step.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0003 AC-13` | met on the workstation: three members, `pool-control` depends only on `pool-config`, the lockfile holds three workspace packages and the same sixteen registry packages, `scripts/check.sh all` and `scripts/prove-checks.sh` pass with the changed baselines, and the addition to rule W4 has one passing and one failing test. CI evidence waits for the pull request |
| All other criteria | outside this milestone, not run |

## Milestone 2: Identifiers, configuration keys and the finding line

Build plan steps 2, 3 and 4, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commits `2ead7fa` (step 2), `22216a2` (step 3) and `23f114f` (step 4), on top of the accepted spec at `dc04352`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. No dependency was added: `Cargo.toml` and `Cargo.lock` are unchanged, and `toml::Spanned` comes from the `toml` 1.2.1 already locked.

### UI / manual

None for this milestone. Nothing here needs Docker, PostgreSQL or GitHub.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `23f114f`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-config --locked` | exit 0 | exit 0. Unit tests 14 passed, `tests/validate.rs` 75 passed, doc tests 4 passed (two of them the new `compile_fail` tests) | pass |
| `cargo build --workspace --locked` after the step 3 commit | exit 0 | exit 0 | pass |
| `cargo test -p pool --locked` | exit 0 | exit 0. Unit tests 3 passed, `tests/check_config.rs` 72 passed | pass |
| `scripts/check.sh all` | every check ok | `ok` for fmt, clippy, test, deny, guards and docs. 371 tests passed, 0 failed. `deny` shows only the known `license-exception-not-encountered` warning | pass |
| `scripts/check.sh fuzz` | exit 0, no crash | exit 0, 1436704 runs in 61 seconds, no crash file | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-topology.toml` | exit 0 | exit 0, the valid report of spec 0001 | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0 | pass |
| Every `*topology*` fixture through the built `pool check-config` in the human format, read by eye against the table under [Fixtures](index.md#fixtures) | the listed exit, codes, keys, lines and columns | identical for all thirteen fixtures | pass |

**Note on an existing test.** `ac4_validated_config_debug_holds_only_states` in `crates/pool-config/tests/validate.rs` compared the derived `Debug` of `ValidatedConfig` with its spec 0001 text. The spec keeps the derived `Debug` and adds fields, so the expected text now also has `control: None, clusters: [], nodes: [], identities: [], identity_index: IdentityIndex { entries: 0 }`. The test still checks that only states, identifiers and a count are printed.

**Reading of the validation order.** Cross check 3 (`cluster.expected_writer`) skips a cluster whose own `id` failed its check. The spec skips the whole check only for node problems. Without the per cluster skip, an invalid cluster `id` would give a second finding at `expected_writer` for the same mistake, which the sentence "The skips keep one mistake from producing two findings" rules out. The test `ac2_cross_checks_skip_to_keep_one_finding_per_mistake` covers it.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0003 AC-1` | `ac1_topology_fixtures_are_accepted` and `ac1_valid_topology_exposes_its_records_in_file_order` (library). `ac1_topology_fixtures_report_exactly_the_valid_report` (binary, both formats, byte for byte, plus a file with `config_version = 1` alone). The spec 0001 fixture tests in both files are unchanged and pass | pass |
| `0003 AC-2` | `ac2_each_invalid_topology_fixture_has_its_one_finding` and `ac2_which_code_a_bad_value_gets` (every row of [Which code a bad value gets](index.md#which-code-a-bad-value-gets), in memory). `ac2_each_invalid_topology_fixture_reports_its_one_finding` and `ac2_the_spec_examples_of_the_finding_line_are_exact` (binary). Extra cases: inline table entries, skips of the cross checks, case of hosts and names | pass |
| `0003 AC-3` | `ac3_multiple_topology_mistakes_are_reported_in_one_run_in_order` (library) and `ac3_invalid_topology_multiple_reports_five_findings_in_order` (binary). `validate` stays the only constructor: the existing `compile_fail` doc tests on `ValidatedConfig` still pass, and no new type has a public constructor, `Default` or `Deserialize` | pass |
| `0003 AC-4` | `ac4_canary_never_reaches_rejection_or_config_text` (`Debug` and `Display` of the `Rejection`, `Debug` of the `ValidatedConfig` and of each record). `ac4_topology_canary_never_reaches_the_output` (stdout and stderr, both formats). `ac4_validate_opens_no_file` and `ac4_check_config_opens_no_file_the_configuration_names` (paths that do not exist are valid) | pass |
| `0003 AC-5` | `ac5_resolve_identity_matches_the_exact_pair_only` (another database, letter case, prefix, empty name, `*`) and `ac5_resolve_identity_keeps_names_byte_for_byte`. The lookup goes through a private `BTreeMap` index, not a walk over the entries. The refusal on a real connection waits for scope 004 | library half pass |
| `0003 AC-6` | Unit tests in `crates/pool-config/src/identifier.rs` and `crates/pool-config/src/generation.rs` (syntax accepted and refused, exact comparison, `FIRST`, `from_stored` bounds, `checked_next` returning `None` at 9223372036854775807). `compile_fail` doc tests on `ClusterId` (compared with a `NodeId`) and on `TopologyEpoch` (compared with an `AuthorityRevision`) | pass |
| `0003 AC-7` to `0003 AC-14` | Not in this milestone. AC-13 is milestone 1 (step 1). AC-7 to AC-12 and the code half of AC-14 are step 5. The record types of `pool-config` built here (`ControlConfig`, `ClusterConfig`, `NodeConfig`, `IdentityRoute`, `SecretReference`) already state their restart rule in their documentation comment | waiting for other milestones |

### Checker run: `/check verify`, milestone 2

Recorded by the checker session `chk-f3-verify` on 2026-11-15 at revision `198a1cd`, on macOS 27.0.1, arm64 (workstation, not Linux and not CI). `198a1cd` is the developer's merge of milestone 1 (`60fcb08`) with milestone 2 (`4f2a937`), and the checker merged that commit into its own branch. The code of steps 2 to 4 is `2ead7fa`, `22216a2` and `23f114f`. The checker did not write the code. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. Every probe file and every probe program was written outside the repository and removed afterward. No file of the repository was changed for a probe.

**Verdict: PASS for milestone 2.** It covers `0003 AC-1` to `0003 AC-6` on the workstation (`0003 AC-5` is the library half, as the criterion says). One finding lies outside the letter of these criteria and is listed under "Finding outside the criteria": the time of `validate` grows with the square of the number of entries. `0003 AC-7` to `0003 AC-14` belong to other milestones and were not run.

**The commands of the record**

- [x] `cargo test -p pool-config --locked` → exit 0. Unit tests 14 passed, `tests/validate.rs` 75 passed, doc tests 4 passed. The two new doc tests are `identifier::ClusterId (line 62) - compile fail` and `generation::TopologyEpoch (line 58) - compile fail` → `0003 AC-1` to `0003 AC-6`
- [x] `cargo build --workspace --locked` → exit 0 → `0003 AC-1`
- [x] `cargo test -p pool --locked` → exit 0. Unit tests 3 passed, `tests/check_config.rs` 72 passed → `0003 AC-1` to `0003 AC-4`
- [x] `scripts/check.sh all` → exit 0, `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. 373 tests passed, 0 failed. The developer counted 371 on `23f114f`, which did not yet hold the two W4 tests of milestone 1 (93 unit tests of `pool-repo-check` there, 95 here). `deny` shows only the known `license-exception-not-encountered` warning. Cargo prints the two manifest warnings that the milestone 1 record explains → `0003 AC-1` to `0003 AC-6`
- [x] `scripts/check.sh fuzz` → exit 0, `Done 1336124 runs in 61 second(s)`, no file under `crates/pool-config/fuzz/artifacts/validate/`. The target runs with `-max_len=4096`
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-topology.toml` → exit 0, the valid report of spec 0001 → `0003 AC-1`
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` → exit 0 → `0003 AC-1`
- [x] Every `*topology*` fixture through the built `pool check-config`, in both formats, compared with the table under [Fixtures](index.md#fixtures) → the exit status, and every code, key, line and column, are as listed for all thirteen fixtures. Standard error was empty each time, and the human and the JSON format agree → `0003 AC-1`, `0003 AC-2`, `0003 AC-3`
- [x] `git diff dc04352 4f2a937 --stat -- Cargo.toml Cargo.lock crates/pool-config/Cargo.toml apps/pool/Cargo.toml` → empty. No dependency was added. `toml` is still `1.2.1` with the features `parse` and `serde`

**Extra checks**

- [x] The thirteen fixtures against the spec text → each file was built again from the text under [Fixtures](index.md#fixtures) and compared with the committed file. All thirteen are the same, byte for byte, and no other `*topology*` fixture exists → `0003 AC-1`, `0003 AC-2`
- [x] The fixtures of spec 0001 before and after → the binary built from `dc04352` (in a private clone) and the binary of this revision each ran the fifteen fixtures of spec 0001 in both formats. Standard output, standard error and exit status are the same for all 90 files. Exit 0 for the three `valid-*` fixtures and exit 1 for the other twelve → `0003 AC-1`
- [x] The valid report → for `valid-topology-minimal.toml`, `valid-topology.toml`, `valid-topology-names.toml` and a file that holds only `config_version = 1`, both formats give the same bytes as `valid-minimal.toml`: four lines in the human format and one JSON object with `"output_version":1` and `"config_version":1`. No cluster, node, identity or count is shown → `0003 AC-1`
- [x] The two examples of the spec → the last line of the human output of `invalid-topology-duplicate-id.toml` and its whole JSON output are the same, byte for byte, as the two examples under [Findings](index.md#findings) → `0003 AC-2`
- [x] Every row of [Which code a bad value gets](index.md#which-code-a-bad-value-gets) through the binary, 110 probe files in both formats → each row gives the listed code. `config.schema` for a table in place of an array of tables, for `node = [1]`, for `control = 1`, for an unknown key in `[control]`, in a cluster, in a node and in an identity, for a number given to a string key, for a string given to `port`, for `port = 70000`, `65536`, `-1` and `5432.0`, and for `system_identifier` as an integer. `config.invalid_value` for `port = 0`, for each bad `id` of the three tables, for each bad `system_identifier`, `authority_id`, path, name and host, and for `routing_profile = "Primary"`. `config.unsupported_value` for `"replica"`. The edges hold: `port` 1 and 65535, an `id` of 63 bytes, a name of 63 bytes (also as 21 scalars of three bytes), a path of 4096 bytes, a host of 253 bytes and `"18446744073709551615"` are valid, and one byte more is refused. Empty arrays and an empty `[control]` with no cluster are valid → `0003 AC-2`
- [x] The validation order, with 29 more probe files for this step and the next → a file with one mistake in every key of a node, written in reverse order, reports `id`, `cluster`, `host`, `port` in that order. A file with its tables in reverse order reports control, clusters, nodes, identities in that order. The three checks of spec 0001 come first. A missing key is reported at the first `[` of the header, also when the header is indented (line 6, column 4), and at the `{` of an inline table. A column counts scalars, not bytes (column 39 after three scalars of two bytes) → `0003 AC-2`, `0003 AC-3`
- [x] The cross checks → three nodes with one `id` give two findings, at the second and the third. A node that names an unknown cluster gives one finding and check 3 stays quiet. A node with a bad `id` or a missing `cluster` stops check 3 as a whole. An `expected_writer` that names a node of another cluster is refused. With a repeated node `id` the reference resolves to the first entry. `DB.Example` and `db.eXAMPLE` on one port are one endpoint, and the same host on another port is not. Principals that differ only in letter case are two entries → `0003 AC-2`
- [x] `invalid-topology-multiple.toml` → five findings in one run, in the order of the spec: `control.authority_id`, `cluster.system_identifier` (line 9, column 21), `node.port` (line 16, column 8), `identity.credential_file` (line 18, column 1), `identity.cluster` (line 22, column 11), in both formats → `0003 AC-3`
- [x] `validate` is the only constructor → a probe crate outside the repository, with `pool-config` as a path dependency, holds 40 small programs that must not compile and one that must. The one compiles. None of the 40 does, each for the expected reason: a struct literal or `Default` of `ValidatedConfig`, `ControlConfig`, `ClusterConfig`, `NodeConfig`, `IdentityRoute`, `SecretReference` and of each identifier and generation type, `Deserialize` for `ValidatedConfig`, `NodeConfig`, `IdentityRoute` and `NodeId`, `Clone` of `ValidatedConfig`, and the private names `IdentityIndex` and `topology` → `0003 AC-3`
- [x] The canary in the output → over the output files of all 28 fixtures in both formats, `grep` finds `CANARY`, `9f2c7b1e` and `not-a-real` in none, and no value of a fixture (a host, a path, a login, a name) in any. Eleven more files put the canary where an error could repeat it (an unknown key in a node and in `[control]`, a string given to `port`, an `id`, `system_identifier`, `authority_id`, `routing_profile`, a reference, a table name inside an identity, a repeated principal and database, a repeated host). None of them prints it → `0003 AC-4`
- [x] The canary in `Debug` and `Display` → the probe program printed the `Debug` text (plain and pretty) of the `ValidatedConfig` of the three valid fixtures and of every record in them, and the `Debug` and `Display` text of the `Rejection` and of each `Finding` of `secret-topology-values.toml` and `invalid-topology-multiple.toml`. The canary is in none, and neither is a host, a port, a path, a principal, a database or a login. The `Debug` text of a file with no topology is `ValidatedConfig { config_version: 1, result_cache: Disabled, automatic_promotion: Disabled, control: None, clusters: [], nodes: [], identities: [], identity_index: IdentityIndex { entries: 0 } }` → `0003 AC-4`
- [x] `validate` opens no file → `grep -rnE 'std::fs|File::|std::env|std::net|std::process|\.exists\(|metadata\(' crates/pool-config/src` prints nothing, and the only `File::open` of `apps/pool/src` reads the configuration file. `/var/lib/pool` and `/etc/pool/secrets/*` do not exist on this machine and the fixtures are valid. A file whose `state_directory` and `credential_file` lie under a directory with mode 000, and one whose `credential_file` does not exist, is valid with exit 0 → `0003 AC-4`
- [x] `resolve_identity` → on `valid-topology.toml` the pairs `("orders_app", "orders")` and `("report_app", "orders")` return their routes, the second with the backend database `orders_reporting`. 23 other pairs return `None`: another database, each name in another letter case, a prefix, a longer name, an empty name, `*`, `%`, a space before or after, the backend database, the backend login and the identity `id` in place of the principal, and the two names swapped. A file with no identity returns `None`. Names are kept byte for byte: the composed and the decomposed form of one name are two entries, and a database with spaces around it matches only with those spaces → `0003 AC-5`
- [x] Identifier types → the three name types accept `a`, `main`, `a_b-c9` and 63 bytes, and refuse an empty text, 64 bytes, a first digit, `-` or `_`, an uppercase letter, a space, a dot, a letter outside ASCII and a line feed. `SystemIdentifier` accepts 1 and 18446744073709551615 and refuses `0`, `007`, an empty text, 18446744073709551616, a sign, a space, `_` and digits outside ASCII. `AuthorityId` refuses 31 and 33 characters, an uppercase letter, `g` and 32 bytes that are 16 scalars, and its `Display` gives back the 32 characters → `0003 AC-6`
- [x] Generation types → `FIRST` is 1, 1 and 0. `from_stored` refuses a value below `FIRST` and every value above 9223372036854775807. `checked_next` at 9223372036854775806 gives 9223372036854775807, and at 9223372036854775807 it gives `None` for all three types. A generation has no `+`, no `From<u64>`, no `Into<u64>` and no `Default` → `0003 AC-6`
- [x] Two kinds never compare → these do not compile, each with `E0308: mismatched types`: `NodeId == ClusterId`, `NodeId < IdentityId`, `ClusterId == IdentityId`, `NodeId == &str`, `TopologyEpoch == AuthorityRevision`, `TopologyEpoch < StartSequence`, `AuthorityRevision == StartSequence`, `TopologyEpoch == u64`. A conversion from one kind into another, and `From<String>` and `FromStr` for an identifier, do not compile either (`E0277`) → `0003 AC-6`
- [x] `scripts/check.sh guards` → `pool-repo-check workspace: ok` and `pool-repo-check ci: ok`. `forbidden-source-patterns.txt` is unchanged since `dc04352`, so rule W3 holds with the new source files

**Finding outside the criteria**

- **The time of `validate` grows with the square of the number of entries.** `Checker::start` in `crates/pool-config/src/topology.rs` calls `line_column` once for every entry and once for every string value, also for a valid one, and `line_column` counts the line feeds from the start of the file each time. Measured with `cargo build --release --locked -p pool` on valid files (exit 0 each time): 4000 `[[node]]` entries (253905 bytes) take 0.61 seconds, 8000 (509905 bytes) take 3.22 seconds, and 16000 (1033905 bytes, below the 1 MiB limit) take 11.99 seconds. 12000 nodes as inline tables on one line (745921 bytes) take 16.18 seconds. 1000 nodes after 909 KB of comment lines take 1.55 seconds, and the same 1000 nodes before those comment lines take 0.07 seconds. The debug build needs 13.8 seconds for 4000 nodes, 74.3 seconds for 8000 and 241.4 seconds for 16000. `sample` shows the time in `pool_config::line_column` (debug) and in `topology::check_node` and `Checker::required` (release). The uniqueness and reference checks themselves use a hash set or a map, so the letter of the rule under [Keys](index.md#keys) ("Uniqueness and reference checks must not take time that grows with the square of the number of entries") is met, and no criterion of this milestone names a time. The purpose of that rule, and the sentence of spec 0001 that a file at the limit "costs nothing you would notice", are not met. The fuzz check cannot see it, because its inputs stop at 4096 bytes. The checker sent the finding to the developer session and to the lead. It does not change the verdict of `0003 AC-1` to `0003 AC-6`.

**Readings the checker assessed**

- **Cross check 3 skips a cluster whose own `id` failed.** The checker accepts this reading without a change to the spec. The first sentence of step 6 of the [validation order](index.md#validation-order) says that each cross check "reads only values that passed steps 3 to 5". Check 3 compares the cluster of the named node with the `id` of the cluster, so for a cluster whose `id` did not pass there is nothing it may read. Observed: a cluster with `id = "MAIN"` gives one finding, at `cluster.id`. With a second cluster whose `id` is bad and a first cluster whose `expected_writer` names no node, both findings appear, so the skip is for that one cluster and check 3 still runs for the others. A sentence that says so in check 3 would make the text plainer. That is an offer to the architect, not a need.
- **The changed text in `ac4_validated_config_debug_holds_only_states`.** The checker accepts the change. Between `dc04352` and `4f2a937` the existing tests of both files lost three lines: a module comment, an import line and this one expected text. Spec 0001 says only that `ValidatedConfig` derives `Debug`, and this spec keeps the derived `Debug` and adds five fields, so the text had to grow. "Every fixture of spec 0001 keeps its result" in `0003 AC-1` is about what a fixture gives (exit status, output, findings), and the comparison of the two binaries above shows that this is unchanged for all fifteen. The test still proves its point: the text holds states, empty lists and one count.
- **What the `Debug` text can show.** `ControlConfig` prints `state_directory` as `Some([REDACTED])` and prints `authority_id`. `NodeConfig` prints its `id` and `cluster`, and `[REDACTED]` for `host` and `port`. `IdentityRoute` prints its `id`, `cluster` and `routing_profile`, and `[REDACTED]` for `backend_database`, `backend_login` and the path of the `SecretReference`. The principal and the requested database are not fields of `IdentityRoute` at all. They are only keys of the private `IdentityIndex`, and its `Debug` prints `entries` and a number. `ClusterConfig` keeps a derived `Debug` and prints `id`, `system_identifier` and `expected_writer`. All of this follows the rule under [Public boundary added to pool-config](index.md#public-boundary-added-to-pool-config) ("prints identifiers and the routing profile only"), because `SystemIdentifier` and `AuthorityId` are identifier types of this spec, and their syntax cannot hold the canary. One gap in the spec text, not in the code: the table under [What may appear in default telemetry](index.md#what-may-appear-in-default-telemetry) names the system identifier in neither column, and the authority ID only in the paragraph below it. The checker suggests that the architect add both to a column.
- **The identity index.** `IdentityIndex` is a `BTreeMap` from principal to a `BTreeMap` from database to the position of the route. The lookup takes two map reads with borrowed text and does not walk the entries. The type cannot be named from outside the crate (`E0603`), and its `Debug` prints the number of entries only. The checker accepts this as what [Public boundary added to pool-config](index.md#public-boundary-added-to-pool-config) asks.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0003 AC-1` | met on the workstation: the three `valid-topology*` fixtures and a file with `config_version = 1` alone give the valid report of spec 0001 byte for byte in both formats, and all fifteen fixtures of spec 0001 give the same output and exit status as before |
| `0003 AC-2` | met on the workstation: each of the eight single mistake fixtures gives exit 1 and its one finding with the listed code, key, line and column, and every row of the table of bad values gives the listed code through the binary |
| `0003 AC-3` | met on the workstation: `invalid-topology-multiple.toml` gives its five findings in one run in the listed order, and no type of the boundary can be built, defaulted, cloned or deserialized from outside |
| `0003 AC-4` | met on the workstation: no canary text in standard output or standard error for any fixture in either format, none in the `Debug` or `Display` text of a `Rejection` or in the `Debug` text of a `ValidatedConfig`, and paths that do not exist or cannot be read are valid |
| `0003 AC-5` | library half met: the route for the exact pair, and `None` for 23 other pairs. The refusal on a real connection waits for scope 004, as the criterion says |
| `0003 AC-6` | met on the workstation: syntax and exact comparison of each identifier, the three operations of each generation, `None` at 9223372036854775807, and no comparison or conversion between two kinds |
| `0003 AC-7` to `0003 AC-14` | outside this milestone, not run (`0003 AC-13` was milestone 1) |

## Milestone 3: Authority model and the four walks

Build plan step 5, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commit `f98d634` (step 5), on top of the merge of milestones 1 and 2 at `198a1cd`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. No dependency was added: `Cargo.toml`, `Cargo.lock` and `crates/pool-control/Cargo.toml` are unchanged, and `pool-control` still depends on `pool-config` alone.

### UI / manual

None for this milestone. Nothing here needs Docker, PostgreSQL or GitHub.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `f98d634`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-control --locked walk_` | exit 0, the four walks pass | exit 0. `walk_clean_startup`, `walk_missing_journal`, `walk_conflicting_evidence`, `walk_restore_before_promotion` passed | pass |
| `cargo test -p pool-control --locked open_only_on_full_agreement` | exit 0 | exit 0, 1 passed. The test counts 8192 cases | pass |
| `cargo test -p pool-control --locked --doc` | exit 0 | exit 0. 4 `compile_fail,E0451` tests passed: struct literals of `Authority`, `EligiblePrimary`, `Closed` and `BootstrapPlan` | pass |
| `cargo test -p pool-control --locked` | exit 0 | exit 0. `tests/walkthroughs.rs` 7 passed (the four walks plus `freshness_rule`, `inconsistent_journal_is_unreadable`, `bootstrap_refusals_need_every_node_and_a_cluster`), `tests/admission_model.rs` 1 passed, doc tests 4 passed | pass |
| `grep -rnE 'Instant::now\|SystemTime\|std::fs\|std::env' crates/pool-control/src` | no output | no output, exit 1 | pass |
| `scripts/check.sh all` | every check ok | `ok` for fmt, clippy, test, deny, guards and docs. 385 tests passed, 0 failed. `deny` shows only the known `license-exception-not-encountered` warning | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0 | pass |
| Mutation check, not kept: drop the quarantine filter from checks 9 and 10 in `crates/pool-control/src/admission.rs` and run `open_only_on_full_agreement` | the test fails | it failed; the file was put back and the suite passed again | pass |

Cargo prints `warning: unused workspace dependency pool-control` for the root `Cargo.toml`. It comes from milestone 1, which declares the workspace dependency before `apps/pool` uses it ([Workspace and guards](index.md#workspace-and-guards)). It is a warning and fails no check.

**Readings of the spec.** These are the choices the text left to the build, each taken from the closest wording.

- Fresh: for each node only the sample with the latest `sampled_at` counts. When that sample is later than `now`, stale or failed, the node has no evidence, and an older sample is not used instead. Two samples at the latest time give no evidence. A sample of a node that the configuration does not list for the cluster is ignored.
- `plan_bootstrap` reports every refusal that applies, so a present journal or a witness does not stop the evidence checks. With healthy observations, walk 2 gives exactly the listed reasons.
- The structural check of a `StoredAuthority` runs before the witness is looked at, so an inconsistent journal is `Unreadable` with or without a witness.
- `Authority` has three read only accessors (`authority_id`, `authority_revision`, `start_sequence`) for the daemon of scope 004. It still has no public constructor.
- The `compile_fail` tests name error `E0451` (private field), so they cannot pass for an unrelated compile error.
- `ValidatedConfig` in `crates/pool-config/src/lib.rs` gained the one line restart rule in its documentation comment, because the [Records](index.md#records) table marks it as built in this slice. No code of `pool-config` changed.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0003 AC-7` | `walk_clean_startup`: every row of walk 1, `Uninitialized` closed with healthy observations, the plan of step 2, the five refusals 2a to 2e, `Unconfirmed` closed, `Ready` open. `bootstrap_refusals_need_every_node_and_a_cluster`: `bootstrap.no_cluster`, each reason once and in order, check 8 skipped when the writer has no evidence. The remaining refusals (`store_present`, `witness_present`) are covered by AC-8 | pass |
| `0003 AC-8` | `walk_missing_journal`: `Missing`, `Unreadable` and `Mismatch` closed with their reasons; bootstrap refused with `bootstrap.witness_present`, or with `bootstrap.store_present` and `bootstrap.witness_present`, while every node is healthy | pass |
| `0003 AC-9` | `walk_conflicting_evidence`: rows a to p with the listed result and reasons, and `latches_conflict()` true only for rows c, d and i. Row o is also checked with `pg2` out of sight. A cluster that neither side knows gives `authority.cluster_unknown` alone | pass |
| `0003 AC-10` | `walk_restore_before_promotion`: step 1 `Held` and closed with `authority.restore_pending` for healthy, empty and conflicting observations and for an updated configuration; steps 2 to 4 give the listed reasons, each asking for the latch | pass |
| `0003 AC-11` | Row e of walk 3 (age equal to the expiry), `freshness_rule` (age just below the expiry is fresh, a sample later than now is no evidence, the latest sample wins, a tie gives no evidence). The `grep` above prints nothing | pass |
| `0003 AC-12` | `open_only_on_full_agreement` over the 8192 cases, against the test's own rule. The four `compile_fail,E0451` doc tests | pass |
| `0003 AC-14`, code half | Each record type the [Records](index.md#records) table builds in 003, with the line of its restart rule: `ClusterConfig` `crates/pool-config/src/topology.rs:88`, `NodeConfig` `topology.rs:117`, `IdentityRoute` `topology.rs:166`, `SecretReference` `topology.rs:226`, `ValidatedConfig` `crates/pool-config/src/lib.rs:38`, `Observation` `crates/pool-control/src/observation.rs:74`, journal identity `StoredAuthority` `crates/pool-control/src/authority.rs:31`, approved topology `ApprovedTopology` `authority.rs:65`, `Quarantine` `authority.rs:90`. Also stated: `ControlConfig` `topology.rs:53`, `Hold` `authority.rs:52`, `Authority` `authority.rs:134`, `EligiblePrimary` `crates/pool-control/src/admission.rs:100`, `Closed` `admission.rs:143`, `BootstrapPlan` `crates/pool-control/src/bootstrap.rs:63`, `BootstrapRefused` `bootstrap.rs:81`, and the three generations in `crates/pool-config/src/generation.rs:54`, `:71`, `:80`. The row of the shared conventions is step 6 | code half pass |
| `0003 AC-1` to `0003 AC-6`, `0003 AC-13` | Milestones 1 and 2 above | see there |

### Checker run: `/check verify`, milestone 3

Recorded by the checker session `chk-f3-verify` on 2026-11-15 at revision `d26016c` (`main`), on macOS 27.0.1, arm64 (workstation, not Linux and not CI). `git diff f98d634 d26016c` is empty for `crates/`, `apps/`, `tools/`, `scripts/`, `Cargo.toml`, `Cargo.lock`, `deny.toml` and `.github/`, so the code is the code of step 5. The checker did not write the code. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Every probe test, probe program and mutation was written and run in a copy of the workspace outside the repository. No file of the repository was changed for a probe.

**Verdict: PASS for milestone 3.** It covers `0003 AC-7` to `0003 AC-12` and the code half of `0003 AC-14` on the workstation. The code gives the answer of the spec tables in every case the checker tried, also in the checker's own model of 65536 cases. Thirteen mutations of the code are caught by no test of the repository. They are listed under "Mutations" and went to the tester session. They do not change the verdict, because the code under test is right in each of those places. One point for the architect is listed under "Finding outside the criteria".

**The commands of the record**

- [x] `cargo test -p pool-control --locked walk_` → exit 0, 4 passed: `walk_clean_startup`, `walk_missing_journal`, `walk_conflicting_evidence`, `walk_restore_before_promotion` → `0003 AC-7` to `0003 AC-10`
- [x] `cargo test -p pool-control --locked open_only_on_full_agreement` → exit 0, 1 passed → `0003 AC-12`
- [x] `cargo test -p pool-control --locked --doc` → exit 0, 4 passed, each marked `compile fail`: `authority::Authority (line 128)`, `admission::EligiblePrimary (line 89)`, `admission::Closed (line 137)`, `bootstrap::BootstrapPlan (line 57)` → `0003 AC-12`
- [x] `cargo test -p pool-control --locked` → exit 0. `tests/walkthroughs.rs` 7 passed, `tests/admission_model.rs` 1 passed, doc tests 4 passed → `0003 AC-7` to `0003 AC-12`
- [x] `grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src` → no output, exit 1 → `0003 AC-11`
- [x] `scripts/check.sh all` → exit 0, `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. 385 tests passed, 0 failed. `deny` shows only the known `license-exception-not-encountered` warning
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` → exit 0
- [x] The developer's mutation (drop the quarantine filter of checks 9 and 10) → `open_only_on_full_agreement` and `walk_conflicting_evidence` fail. It is mutation A15 below

**Extra checks**

- [x] The tests against the spec tables, by reading → every row of walk 1 (1, 2, 2a to 2e, 3, 4), walk 2 (1, 1a, 2, 2a, 3), walk 3 (a to p) and walk 4 (1 to 4) has an assertion in `tests/walkthroughs.rs` with the setup and the result of the table: the same nodes, sample times, timelines and identifiers, the same reason names in the same order, and `latches_conflict()` true for rows c, d and i of walk 3 and steps 2 to 4 of walk 4 only → `0003 AC-7` to `0003 AC-10`
- [x] The model test, by reading → `expected` in `tests/admission_model.rs` works on the test's own values (two writer numbers, two flags and three `Seen` values). It calls nothing of `pool-control`. The loop counts 8192 cases and the domain is the one under [Tests and who owns them](index.md#tests-and-who-owns-them). Two limits: for a closed answer it compares only `latches_conflict()` and that the list is not empty, and it asserts `opened > 0` where the exact number for this domain is 130 → `0003 AC-12`
- [x] The checker's own model, 65536 cases → written from rows 2 to 10 of the table under [Writer admission](index.md#writer-admission), not from the code and not from the developer's tests. It loops over `expected_writer` in `pg1`, `pg2`; the approved writer in `pg1`, `pg2`; the journal timeline 1 or 2; no quarantine or one of the three nodes; the latch; the configured system identifier equal to the journal's or not; and the eight observations for each node. For every case the code gives exactly the list of reasons of the model, in the order of the table, `latches_conflict()` is true exactly when the list holds `evidence.second_primary` or `evidence.timeline_ahead`, and an open answer names the approved writer. The number of open answers is 474, which is the number the rule predicts by hand → `0003 AC-9`, `0003 AC-12`
- [x] Start classification, the whole table → `Absent` gives `Uninitialized` or `Missing`. `Unreadable` gives `Unreadable` in both columns. `Present` without a witness is `Unconfirmed`, also when the hold is `restore_pending` and also for a configuration with no `[control]` table. A witness that differs gives `Mismatch`, also when the hold is set, so the witness is compared before the hold. The matching witness gives `Held` or `Ready`. A journal that is not consistent (a member twice in a second cluster, a writer outside the members of a second cluster, no members at all, a quarantine row for a cluster it does not hold) is `Unreadable` whatever the witness and the hold say. Each of the six states that are not `Ready` closes with its one reason, with healthy observations and with none, and never asks for the latch → `0003 AC-7`, `0003 AC-8`, `0003 AC-10`
- [x] Freshness with more than one sample for a node → an age equal to the expiry is no evidence, one nanosecond less is fresh, age 0 is fresh, one nanosecond in the future is no evidence, and an expiry of 0 makes nothing fresh. A failed latest sample does not fall back to an older fresh one, in either order of the list. A latest sample in the future does not fall back either. An older failed sample does not hide a later good one. A tie among older samples does not matter. A tie at the latest time gives no evidence, also when the two samples differ, with three samples, and when one of them failed. The same holds for a node that is not the writer: a tie, a stale sample or a future sample of `pg2` that says primary does not close, and a later failed sample hides an older one that said primary → `0003 AC-11`
- [x] Two clusters (`main` with three nodes, `b` with two) → each cluster opens by itself. The primary of `b` is not a second primary of `main`, and its timeline 3 is not ahead of timeline 1 of `main`, also when it reports the system identifier of `main`. The latch, the quarantine and the missing evidence of one cluster do not touch the other. A quarantine row `(b, pg2)` does not quarantine `pg2` of `main`. A cluster that only the journal holds, or only the configuration, gives `authority.cluster_unknown` alone, also when its latch is set. A member of the journal that the configuration lists under another cluster gives `authority.config_conflict` → `0003 AC-9`
- [x] Timelines below the journal's → with a journal timeline of 3, a writer on timeline 2 closes with `evidence.writer_timeline`, a replica on timeline 2 or 3 does not close, and a replica on timeline 4 closes with `evidence.timeline_ahead` → `0003 AC-9`
- [x] A configured node that is not a member in the journal is still looked at → it does not close while it is in recovery, and it closes with `evidence.second_primary` when it says primary → `0003 AC-9`
- [x] What `Open` carries → with epoch 7 for `main`, epoch 9 for `b` and revision 41 in the journal, the open answers carry 7 and 41, and 9 and 41. The three accessors of `Authority` return what the journal held → `0003 AC-12`
- [x] Every reason at once → reasons 2, 3, 4, 5, 9, 10 in one answer, and 2, 3, 4, 7, 8, 9, 10 in another, each reason once though two nodes cause 9 and 10, in the order of the table → `0003 AC-9`
- [x] Bootstrap, the whole refusal table with two clusters → the plan holds one approved topology for each cluster in the order of the configuration, with the timeline each writer reported (1 and 5), epoch 1, every configured node as a member and no latch. Each of the refusals 4 to 8 is given when only the second cluster causes it, and when only the first does. A stale sample, a future sample, a failed sample and a missing sample each give `bootstrap.evidence_missing`. A node with another system identifier that says primary on a later timeline gives `bootstrap.identity_mismatch` and nothing more. Check 8 is skipped for a cluster whose writer does not fit and still runs for the other cluster. Refusals 1 and 2 do not stop the evidence checks: one call gives all of 1, 2, 4, 5, 6, 7 and 8 in order, and a configuration with no cluster gives 1, 2 and 3. A sample of a node outside the configuration is ignored → `0003 AC-7`, `0003 AC-8`
- [x] The types from outside the crate → small programs that use `pool-control` as a dependency do not compile, each for the expected reason: a struct literal of `BootstrapRefused` (`E0451`), `Authority { ..a }` and `EligiblePrimary { node, ..e }` (`E0451`), `Default` for `Authority`, `EligiblePrimary`, `Closed` and `BootstrapPlan` (`E0277`), writing through `Ready(ref mut a)` into `a.stored` (`E0616`), reading or clearing `Closed.reasons` and `BootstrapPlan.clusters` (`E0616`), and the tuple constructors `TimelineId(0)` and `MonotonicTime(..)` (`E0423`). The only public functions that return one of these types are `classify_startup`, `writer_admission` and `plan_bootstrap` → `0003 AC-12`
- [x] The derives → every public type has `Debug, Clone, PartialEq, Eq`. The seven types the spec names also have `Copy`, and `TimelineId` and `MonotonicTime` have `PartialOrd, Ord`. `TimelineId::from_observed(0)` is `None`. Both also derive `Hash`, which the spec does not list and which harms nothing
- [x] Rule W3 on the new code → in a private clone, `pool-repo-check workspace` is `ok` on the clean tree and gives `W3: crates/pool-control/src/admission.rs:303: forbidden text ...` with exit 1 for each of `std::thread`, `tokio`, `TcpStream` and `std::net` added to `admission.rs`, and for `std::thread` added to `tests/walkthroughs.rs`
- [x] The dependencies of `pool-control` → `cargo metadata --no-deps --format-version 1` and `cargo tree -p pool-control -e normal,build,dev` show `pool-config` alone. The manifest has no `dev-dependencies` and the package has no build script
- [x] The restart rules, by reading → each line that the developer lists for `0003 AC-14` holds a documentation comment that starts with `Restart rule:` and sits on the named public type. The nine records that the [Records](index.md#records) table builds in this slice are `ClusterConfig`, `NodeConfig`, `IdentityRoute`, `SecretReference`, `ValidatedConfig`, `Observation`, `StoredAuthority`, `ApprovedTopology` and `Quarantine`, and each rule says what the table says. The change to `crates/pool-config/src/lib.rs` in `f98d634` is three comment lines and no code → `0003 AC-14`, code half

**Mutations**

60 changes to `crates/pool-control/src`, one at a time, each run with `cargo test -p pool-control --locked --tests` in the copy. 46 make a test of the repository fail. One is a control that changes nothing (the open answer names `expected_writer`, which is equal to the approved writer whenever the answer is open). 13 are caught by no test of the repository, and each of them is caught by the checker's probe file.

The seven the lead named are all caught by the repository tests: the conflict latch dropped (`walk_conflicting_evidence`, `open_only_on_full_agreement`), the missing member dropped (`walk_conflicting_evidence`), the later timeline dropped (`walk_conflicting_evidence`, `walk_restore_before_promotion`, `open_only_on_full_agreement`), another system identifier for the writer, for the configuration and for another node (`walk_conflicting_evidence`, `open_only_on_full_agreement`), another `expected_writer` (three tests), `restore_pending` ignored (`walk_restore_before_promotion`), and an age equal to the expiry taken as fresh (`walk_conflicting_evidence`, row e).

Not caught by any test of the repository:

| Mutation | Why no test sees it |
|---|---|
| Check 8 closes only for a higher writer timeline, so a writer on a lower timeline than the journal is accepted | Every journal in the tests records timeline 1 |
| Check 10 also counts a lower timeline of another node as ahead | Same |
| A quarantine row is matched by its node alone, and its cluster is ignored | Every test has one cluster |
| `Open` carries epoch 1, not the epoch of the journal | Every test uses epoch 1 |
| `Open` carries revision 1, not the revision of the journal | Every test uses revision 1 |
| Checks 2, 9 and 10 look at the nodes of every cluster | One cluster |
| A failed latest sample falls back to an older fresh sample | No test gives a node a failed sample and an older one |
| A latest sample in the future falls back to an older fresh sample | No test gives a node a future sample and an older one |
| A tie among older samples counts as a tie | No test has three samples for one node |
| A held journal of another identity is `Held`, not `Mismatch` | No test combines the hold with a wrong witness |
| Bootstrap checks 6 to 8 also look at a node with another system identifier | Row 2d uses a node in recovery on timeline 1 |
| Bootstrap checks and plans only the first configured cluster | One cluster |
| Bootstrap treats the nodes of every cluster as nodes of each cluster | One cluster |

**Finding outside the criteria**

- **A `Ready` state is not tied to the configuration that confirmed it.** `writer_admission` takes the configuration and the state as two arguments and, in `Ready`, does not look at the witness again. Observed: a state classified `Ready` with witness `X` gives `Open` for `pg1` when it is passed together with a configuration that has no witness, or one with witness `Y`. Classified fresh, those two configurations give `Unconfirmed` and `Mismatch`. This is what the spec text asks: the table of checks for `Ready` has no witness row, and the function takes both values. It cannot happen in scope 004, which classifies once at the start with the one configuration it has. It matters when a reload exists (scope 009): a reload that changes or removes `control.authority_id` has to classify again, or be refused. The checker suggests that the architect say so in the spec, under [Writer admission](index.md#writer-admission) or in the Follow-up for 009. It does not change the verdict of `0003 AC-12`.

**Readings the checker assessed**

- **Fresh: the latest sample only.** The checker accepts the reading. It is the text of the paragraph "Fresh" under [The authority model in pool-control](index.md#the-authority-model-in-pool-control): the functions use the observation with the latest `sampled_at`, a tie at that time gives no evidence, and an observation that is not fresh or that failed is no evidence. Nothing there lets an older sample stand in. Observed for the writer and for another node, in admission and in bootstrap. A sample of a node that the configuration does not list for the cluster is ignored, as the last sentence of that paragraph says.
- **`plan_bootstrap` reports every refusal, and the structural check comes before the witness.** The checker accepts both. "Every reason that applies is reported, in this order" stands above the refusal table, and row 2a of walk 2 lists two reasons. The sentence on a journal that is not consistent has no condition on the witness, and the row `Unreadable` of the classification table has the same answer in both columns. Observed: such a journal is `Unreadable` with no witness, with the matching witness, with another witness and with the hold set.
- **The three accessors of `Authority`.** The checker accepts them. They return copies of three numbers and give no way to build or change an `Authority`. On the question of another way to an open answer from outside the crate: the checker found two. The first is a `StoreProbe::Present` built by hand, which [The authority model in pool-control](index.md#the-authority-model-in-pool-control) names and binds to the journal reader as the only builder. The second is the finding above. No struct literal, update syntax, `Default`, field access or tuple constructor works, and `forbid(unsafe_code)` holds for the workspace.
- **The comment in `crates/pool-config/src/lib.rs`.** The checker accepts it. The [Records](index.md#records) table marks the type `ValidatedConfig` as built in 003, so `0003 AC-14` asks for its restart rule. The edit adds comment lines only.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0003 AC-7` | met on the workstation: no journal and no witness is `Uninitialized` and closed with `authority.uninitialized` with healthy observations; a plan only when every node of every cluster has fresh, fitting evidence; each row of the refusal table gives its reason, with one and with two clusters; a journal without a witness is `Unconfirmed` and closed |
| `0003 AC-8` | met on the workstation: `Missing`, `Unreadable` and `Mismatch` with their reasons, and bootstrap refused with `bootstrap.witness_present`, `bootstrap.store_present` or both, with healthy observations and with bad ones |
| `0003 AC-9` | met on the workstation: rows a to p of walk 3 give the listed result and reasons, only full agreement opens over 65536 cases, the latch is asked for exactly when a second primary or a later timeline is reported, and a latched journal stays closed with `authority.conflict_latched` whatever the evidence |
| `0003 AC-10` | met on the workstation: a journal with the hold is `Held` and closed with `authority.restore_pending` for healthy, empty and conflicting observations; steps 1 to 4 of walk 4 give the listed results |
| `0003 AC-11` | met on the workstation: an age equal to or above the expiry and a sample later than now are no evidence, to the nanosecond; the `grep` prints nothing |
| `0003 AC-12` | met on the workstation: over the small domain and over the checker's wider one, `Open` is given only for the node that is the approved writer and the `expected_writer`, and never with a listed conflict; a struct literal of `Authority`, `EligiblePrimary`, `Closed` and `BootstrapPlan` does not compile outside the crate |
| `0003 AC-14`, code half | met by reading: the nine records built in this slice are public types at the named places, and each documentation comment states its restart rule |
| `0003 AC-1` to `0003 AC-6`, `0003 AC-13` | outside this milestone |

## Milestone 4: Document corrections and evidence

Build plan steps 6 and 7, on 2026-11-15, on macOS (workstation, not Linux and not CI). Docker was running (`docker info` exit 0). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`.

**Revision under test: `0a41060`.** That is the commit of step 6, on top of milestones 1 to 3 (`2648ae1` on `main`, `23f114f` and `f98d634` on this branch, merged at `af218e6`). The commit of step 7 adds only this section and one row of `docs/index.md`, so it changes no code, manifest, script or baseline. If a later fix to milestone 2 or 3 changes the code, run the commands below again, and replace the revision on this line and the Observed column of the table.

### UI / manual

None. The Linux build ran inside `scripts/prove-checks.sh --with-linux-build` (case 20), because Docker was running. A build on macOS is not Linux evidence, and CI evidence waits for the pull request.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `0a41060`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh docs` after the edits of step 6 | exit 0 | exit 0, `pool-repo-check docs: ok` | pass |
| `scripts/check.sh all` | every check ok | exit 0, `ok` for fmt, clippy, test, deny, guards and docs. 385 tests passed, 0 failed. `deny` shows only the known `license-exception-not-encountered` warning | pass |
| `scripts/prove-checks.sh --with-linux-build` | every case rejects its change | exit 0, `prove-checks: commit 0a4106037dcc3785c51f15246451f2edf91f2816`, cases 1 to 28 `PASS`, no `FAIL`, `all checks reject their bad change`. Case 17 matched the D3 line on `docs/scope/004-one-real-primary-path.md`. Case 20 ran the Linux build in Docker | pass |
| `scripts/check.sh fuzz` | exit 0, no crash | exit 0, 1126664 runs in 61 seconds, no crash file | pass |
| `cargo metadata --no-deps --format-version 1` | three members; `pool-control` depends only on `pool-config` | exit 0, three workspace members: `pool` (`pool-config`, `serde`, `serde_json`), `pool-config` (`serde`, `toml`), `pool-control` (`pool-config`) | pass |
| `Cargo.lock`, `Cargo.toml` and `crates/pool-control/Cargo.toml` compared with `main` | unchanged since milestone 1 | no difference; `Cargo.lock` holds 19 entries | pass |
| `cargo test -p pool-control --locked walk_` | the four walks pass | exit 0, 4 passed | pass |
| `cargo test -p pool-control --locked open_only_on_full_agreement` | exit 0 | exit 0, 1 passed | pass |
| `cargo test -p pool-control --locked --doc` | exit 0 | exit 0, 4 passed | pass |
| `grep -rnE 'Instant::now\|SystemTime\|std::fs\|std::env' crates/pool-control/src` | no output | no output, exit 1 | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-topology.toml` | exit 0 | exit 0 | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0 | pass |

**Note on the Cargo warnings.** On `0a41060` Cargo prints one manifest warning, `warning: unused workspace dependency pool-control`, for the root `Cargo.toml`. It ends in scope 004, when `apps/pool` takes the dependency ([Workspace and guards](index.md#workspace-and-guards)). The second warning that the checker of milestone 1 read, `warning: unused dependency pool-config` for `crates/pool-control/Cargo.toml`, no longer appears: the code of step 5 uses `pool-config`. Neither warning fails a check.

**The edits of step 6.** Items 1 to 4 of [Document corrections](index.md#document-corrections):

1. `docs/engineering-standards.md`, shared conventions, row "Identifier and generation types": `Pending` is replaced by the rule (one type per identifier and generation, built only by a checking function, generations advance by one and fail closed at the top, shared types in `pool-config`, memory generations scoped by the start sequence) and a link to [Identifier and generation types](index.md#identifier-and-generation-types).
2. `docs/engineering-standards.md`, first paragraph: the workspace is `apps/pool`, `crates/pool-config` and `crates/pool-control`.
3. `docs/repository-structure.md`, first paragraph: `crates/pool-control` is among the paths that exist.
4. `docs/glossary.md`, rows "Observation" and "Approved writer": backend becomes node.

Item 5 is step 7: the row for this file is in `docs/index.md`. `crates/pool-control` has no context file yet, so `docs/index.md` gets no section for it. Item 6 belongs to the "Document it" box of scope 003.

### Acceptance-criteria coverage

Every criterion on the one revision `0a41060`. The test names are in the milestone that built each criterion; `scripts/check.sh all` ran all of them again here.

| Criterion | Evidence on `0a41060` | Result |
|---|---|---|
| `0003 AC-1` to `0003 AC-4` | the tests listed under milestone 2, in `scripts/check.sh all`; `check-config` on `valid-topology.toml` exit 0 | pass |
| `0003 AC-5` | the tests listed under milestone 2, in `scripts/check.sh all`. The refusal on a real connection waits for scope 004 | library half pass |
| `0003 AC-6` | unit tests and `compile_fail` doc tests of `pool-config`, in `scripts/check.sh all` | pass |
| `0003 AC-7` to `0003 AC-10` | `cargo test -p pool-control --locked walk_` and the rest of `tests/walkthroughs.rs` in `scripts/check.sh all` | pass |
| `0003 AC-11` | row e of walk 3 and `freshness_rule`; the `grep` above prints nothing | pass |
| `0003 AC-12` | `open_only_on_full_agreement` and the four `compile_fail,E0451` doc tests | pass |
| `0003 AC-13` | `cargo metadata` and the manifest comparison above; `scripts/check.sh all` and `scripts/prove-checks.sh --with-linux-build` pass | pass on the workstation; CI evidence waits for the pull request |
| `0003 AC-14` | code half: the types and lines listed under milestone 3, unchanged since `f98d634`. Document half: the row "Identifier and generation types" of the [shared conventions](../../engineering-standards.md#shared-conventions) names the rule and links to [Identifier and generation types](index.md#identifier-and-generation-types); `scripts/check.sh docs` passes | pass |

### Checker run: `/check verify`, milestone 4

Recorded by the checker session `chk-f3-verify` on 2026-11-15 at revision `d26016c` (`main`), on macOS 27.0.1, arm64 (workstation, not Linux and not CI). Docker was running (`docker info` exit 0). `git diff 0a41060 d26016c --stat` names only `docs/index.md`, this file and `graphify-out/`, so every code file, manifest, script and baseline is the one of the revision the developer recorded. The checker did not write the code or the document edits. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`.

**Verdict: PASS for milestone 4.** The five document corrections are in place as the table under [Document corrections](index.md#document-corrections) asks, both halves of `0003 AC-14` are met, and the evidence of step 7 was reproduced on `main`. This is not the full verification of the feature. The fix to the time of `validate` (the finding of milestone 2) is still open, so the fixture steps, `0003 AC-2` and `0003 AC-3` are run again in the full verification after that fix is merged.

**The commands of the record**

- [x] `scripts/check.sh docs` → exit 0, `pool-repo-check docs: ok` → `0003 AC-14`
- [x] `scripts/check.sh all` → exit 0, `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. 385 tests passed, 0 failed. `deny` shows only the known `license-exception-not-encountered` warning
- [x] `scripts/prove-checks.sh --with-linux-build` → exit 0, `prove-checks: commit d26016c9863fb956452bb26d935f1ac2e2103fe7`, cases 1 to 28 `PASS`, no `FAIL`, `all checks reject their bad change`. Case 17 matched `D3: docs/scope/004-one-real-primary-path.md:1: status disagrees: done in the heading, planned in docs/scope/index.md`. Case 20 ran the Linux build in Docker → `0003 AC-13`
- [x] `scripts/check.sh fuzz` → exit 0, `Done 1096245 runs in 61 second(s)`, no file under `crates/pool-config/fuzz/artifacts/validate/`
- [x] `cargo metadata --no-deps --format-version 1` → exit 0, three workspace members: `pool` (`pool-config`, `serde`, `serde_json`), `pool-config` (`serde`, `toml`), `pool-control` (`pool-config`) → `0003 AC-13`
- [x] `git diff 2648ae1 d26016c --stat -- Cargo.lock Cargo.toml crates/pool-control/Cargo.toml` → empty, so the three files are as milestone 1 left them. `Cargo.lock` holds 19 entries: the three workspace packages and sixteen others → `0003 AC-13`
- [x] `cargo test -p pool-control --locked walk_` → exit 0, 4 passed
- [x] `cargo test -p pool-control --locked open_only_on_full_agreement` → exit 0, 1 passed
- [x] `cargo test -p pool-control --locked --doc` → exit 0, 4 passed
- [x] `grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src` → no output, exit 1
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-topology.toml` → exit 0, the valid report of spec 0001
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` → exit 0

**The document corrections, read against the table**

- [x] Item 1 → `git show 0a41060 -- docs/engineering-standards.md`: in the row "Identifier and generation types" of the shared conventions, `Pending` is replaced. The new text holds the five rules of the table: one type per identifier and per generation, an identifier built only by a function that checks it, a generation advances by one and fails closed at the top, shared types live in `pool-config`, and a generation kept in memory compares only inside one start sequence. It ends with a link to `specs/0003-control-and-session-data-model/index.md#identifier-and-generation-types`, and that heading exists (line 135 of the spec; rule D2 of `scripts/check.sh docs` checks the fragment) → `0003 AC-14`
- [x] Item 2 → the first paragraph of `docs/engineering-standards.md` names `apps/pool`, `crates/pool-config` and `crates/pool-control`
- [x] Item 3 → the first paragraph of `docs/repository-structure.md` lists `crates/pool-control` among the paths that exist
- [x] Item 4 → in `docs/glossary.md` the rows "Observation" and "Approved writer" say node where they said backend. No other word of the two rows changed
- [x] Item 5 → `docs/index.md` line 177 holds the row for this file. `crates/pool-control/AGENTS.md` does not exist, so there is no section to add
- [x] The commit of step 6 changes these three files and nothing else (5 lines added, 5 removed). The commit of step 7 changes `docs/index.md` and this file. Items 6 and 7 belong to other steps and were not checked here

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0003 AC-14` | met: the code half as recorded under milestone 3, and the row of the shared conventions names the rule and links to [Identifier and generation types](index.md#identifier-and-generation-types) |
| `0003 AC-7` to `0003 AC-12` | met on the workstation, as recorded under milestone 3, on the same revision |
| `0003 AC-13` | the commands of step 7 pass on the workstation, and the Linux build ran in Docker inside case 20. CI evidence waits for the pull request |
| `0003 AC-1` to `0003 AC-6` | not run again by hand here. Their tests are part of the 385 that passed. The full verification after the fix to `validate` runs their steps again |

## Milestone 5: Validation time and the witness in writer admission

Build plan steps 8 and 9, code part, on 2026-11-15, on macOS (workstation, not Linux and not CI). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. No dependency was added, and `Cargo.toml` and `Cargo.lock` are unchanged.

**Revision under test: `e79f8b2`** (`fix(config): spec 0003 step 8, positions are worked out without rescanning the file`), on top of `79de174` on `main`. The old code is `79de174`. `main` took the same change as `be8bcf1`, through `ebc9228`: the same subject as in the first brief, and the same tree under `crates/` as `e79f8b2`. It changes only `crates/pool-config/src/lib.rs` and `crates/pool-config/src/topology.rs`, outside every `#[cfg(test)]` module. The `ac15_` tests of step 8 are not part of this revision. The tester session `tst-f3-test` writes them on its own branch. Their run is recorded when that branch is merged.

**What changed.** `topology::Checker` turned the span of every entry and every string value into a line and a column with `line_column`, which counts from the top of the file. A `LineIndex` is now built once, in time linear in the file. It holds where each line starts and the number of Unicode scalar values before every 256th byte. A lookup is a binary search over the line starts, plus a scan of fewer than 256 bytes from the nearest stored count, once for the offset and once for the start of its line. So nothing counts from the top of the file or from the start of a line, and the work is at most n log n, as [Validation time](index.md#validation-time) asks ("any other way with the same bound"). The one position of a syntax or schema error still uses `line_column`, at most once per run.

**Step 9, revision under test: `8135cca`** (`fix(control): spec 0003 step 9, a ready state stays tied to its witness`), on top of `c10e550` on `main`. The old code is `c10e550`. It changes only `crates/pool-control/src/admission.rs`, outside every `#[cfg(test)]` module. The two `ac16_` tests of step 9 come, like the `ac15_` tests, from the branch of `tst-f3-test`.

**What changed in step 9.** In `Ready`, `writer_admission` now compares the witness `control.authority_id` of the configuration it is given with the authority ID of the state first. No witness gives `Closed` with `authority.unconfirmed` alone, and another witness gives `Closed` with `authority.mismatch` alone. Then no other check runs, the observations are not read, and the cluster is not looked up. No signature or reason name changed. The documentation comment of `writer_admission` now says this, in place of "every rule runs in `Ready`".

### UI / manual

None for this milestone.

### Commands

The made inputs were built by a probe script outside the repository, as [Validation time](index.md#validation-time) describes them. Times are wall clock from the start of `pool check-config` to its exit, one run each, on the workstation. They are not a benchmark.

| Command | Expected | Observed | Result |
|---|---|---|---|
| Made inputs 1, 2 and 3: entries and bytes | 13217 / 1048511, 16471 / 1048558, 17268 / 1048559 | the same | pass |
| `pool check-config` on inputs 1, 2 and 3, debug build of `e79f8b2`, `human` and `json` | each run within 10 s; exit 0, 0, 1 | 0.62 s, 0.62 s, 0.77 s at most; exit 0, 0, 1 | pass |
| The same, release build of `e79f8b2` | within 10 s | 0.08 s, 0.08 s, 0.12 s at most | pass |
| The same, release build of the old code | (for comparison) | 4.7 s, 39 to 44 s, 39 to 41 s | the fault |
| Input 3, findings | 17268 `config.invalid_value` at `node.port`, all on line 2, the first at column 60 | the same; output byte for byte equal to the old code in both formats | pass |
| Made input 4 | `node.port` line 21 column 8, then `node.id` line 18 column 6 | the same, equal to the old code | pass |
| Probe files of the checker's finding, debug, old code → `e79f8b2` | (for comparison) | 4000 nodes 15.92 → 0.49 s; 8000 nodes 70.16 → 0.52 s; 16000 nodes (938 KB) 284.63 → 0.67 s; 12000 nodes on one line 436.01 → 0.43 s; 1000 nodes after a 974 KB comment 42.65 → 0.14 s | pass |
| The 28 fixtures and the 6 probe files, binary of the old code against `e79f8b2`, `human` and `json` | stdout, stderr and exit equal | 68 comparisons, 0 differences | pass |
| Four large invalid files (duplicate id and bad host deep in the file, an entry after a 900 KB comment with a multi byte id, one long inline line with a column of 571793, CRLF line ends with `é€😀` and a tab), old code against `e79f8b2`, both formats | equal | equal | pass |
| `LineIndex::line_column` against `line_column`, every offset (also past the end) of random texts with CRLF, tabs and multi byte scalars, on and around 256 byte boundaries, and a 200 KB line | equal | equal | pass |
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs `ok` | pass |
| `scripts/check.sh fuzz` | exit 0 | exit 0 | pass |
| `cargo test -p pool-config --locked ac15_`, `cargo test -p pool --locked ac15_` | the step 8 tests pass | not run here: the tests are on the tester's branch | not run |
| Step 9 probe: a crate outside the repository calls `writer_admission` on a state classified `Ready` from witness `X` with the journal of walk 3, for configurations that differ only in `control.authority_id`, with healthy observations, with `pg2` observed as not in recovery, and for the cluster `other` | no witness: `authority.unconfirmed` alone; witness `Y`: `authority.mismatch` alone; `latches_conflict()` false; witness `X`: as before | `8135cca`: no witness gives `["authority.unconfirmed"]` and witness `Y` gives `["authority.mismatch"]` in all three calls, latches false. Witness `X` gives `Open pg1`, `["evidence.second_primary"]` with latches true, and `["authority.cluster_unknown"]`. The old code `c10e550` gave the same three answers for every witness, so it opened `pg1` without a witness and with witness `Y` | pass |
| `cargo test -p pool-control --locked` on `8135cca` | the four walks and the small domain of 8192 cases pass unchanged | 7 + 4 + 1 tests and 4 compile fail doc tests passed, `walk_*` and `open_only_on_full_agreement` among them | pass |
| `grep -rnE 'Instant::now\|SystemTime\|std::fs\|std::env' crates/pool-control/src` | no match | no match | pass |
| `scripts/check.sh all` on `8135cca` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs `ok` | pass |
| `cargo test -p pool-control --locked ac16_` | the step 9 tests pass | not run here: the tests are on the tester's branch | not run |

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0003 AC-15` | the code of `e79f8b2` and the probe rows above | pass on the workstation for the code. The `ac15_` tests and their run come from `tst-f3-test` |
| `0003 AC-2`, `0003 AC-3` (positions) | the fixture and large file comparisons above: no position changed | pass; the checker repeats them |
| `0003 AC-16` | the code of `8135cca` and the step 9 probe row above | pass on the workstation for the code. The `ac16_` tests and their run come from `tst-f3-test` |
| `0003 AC-7` to `0003 AC-12` | the walks and the small domain pass unchanged on `8135cca` | pass |

### Checker run: `/check verify`, milestone 5

Recorded by the checker session `chk-f3-verify` on 2026-11-15 at revision `aaad537` (`main`), on macOS 27.0.1, arm64 (workstation, not Linux and not CI). Docker was running (`docker info` exit 0, server 29.9.2). The code of step 8 is `be8bcf1` and the code of step 9 is `8135cca`. The `ac15_` tests are in `32a7b24` and the `ac16_` tests in `26210ce`, both from the tester. The checker wrote neither the code nor the tests. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. Every probe file, probe program and mutation lived in private clones outside the repository. No file of the repository was changed for a probe.

The workstation was busy with other work during this run (load average 20 to 29 on 10 cores). So every time below is an upper bound, and single slow runs are noise. Where a number looked off, the run was repeated and the repeats are named.

**Verdict: PASS for step 8 (`0003 AC-15`) and PASS for step 9 (`0003 AC-16`).** The two rows that the developer left as "not run" (`ac15_` and `ac16_`) were run here and pass. Three points lie outside the letter of the criteria and are listed under "Findings outside the criteria".

**Step 8, the commands of the record**

- [x] The made inputs, built by the checker's own program from the text under [Validation time](index.md#validation-time), not by the tests → input 1 has 13217 entries and 1048511 bytes, input 2 has 16471 and 1048558, input 3 has 17268 and 1048559 → `0003 AC-15`
- [x] `cargo test -p pool-config -p pool --locked ac15_` → exit 0. `tests/validate.rs` 4 passed in 1.77 s, `tests/check_config.rs` 1 passed in 1.29 s → `0003 AC-15`
- [x] `pool check-config` on inputs 1, 2 and 3, debug build, from start to exit → 0.45 s, 0.54 s and 0.51 s in the human format, 0.45 s, 0.45 s and 0.57 s in the JSON format. Exit 0, 0 and 1. The release build needs at most 0.18 s for each. All far below 10 seconds → `0003 AC-15`
- [x] Input 3 through the binary, both formats → `configuration invalid`, `findings: 17268`, then 17268 lines, each `config.invalid_value` at `node.port` on line 2. The checker counted the column of every `port=` value in the input by itself and compared all 17268, in order: the first is column 60 and the last is column 1048402. The JSON output is one line with 17268 objects and the same positions → `0003 AC-15`
- [x] Input 4 → `config.invalid_value` at `node.port`, line 21, column 8, then `config.duplicate_id` at `node.id`, line 18, column 6. That is against file order → `0003 AC-15`
- [x] The 28 fixtures, binary of `d26016c` (before the fix) against the binary of `aaad537`, both formats → standard output, standard error and exit status compared: 168 comparisons, 0 differences. The debug build gives the same bytes as the release build → `0003 AC-1`, `0003 AC-2`, `0003 AC-3`
- [x] `scripts/check.sh all` → exit 0, `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. 448 tests passed, 0 failed
- [x] `scripts/check.sh fuzz` → exit 0, `Done 95672 runs in 61 second(s)`, no file under `crates/pool-config/fuzz/artifacts/validate/`. The run count is about a twelfth of the earlier runs. The corpus has grown to 8714 files and the machine was busy. No crash and no timeout

**Step 8, extra checks**

- [x] The `ac15_` tests fail on the code before the fix → in a private clone of `d26016c`, with `crates/pool-config/tests/validate.rs` and `apps/pool/tests/check_config.rs` taken from `aaad537`, `cargo test -p pool-config --locked --test validate ac15_ -- --test-threads 1` gives exit 101: `ac15_large_valid_inputs_validate_within_ten_seconds ... FAILED` with `input 1: validate took 239.244506292s`, and `ac15_one_finding_per_entry_on_one_line_at_each_value ... FAILED` with `input 3: validate took 1098.076026583s`. `ac15_made_inputs_have_the_listed_size` and `ac15_findings_follow_the_validation_order_against_the_file_order` pass there, as they should: they are not about time. The run took 1337 seconds. The test through the binary, `ac15_the_binary_reports_every_finding_of_input_3_within_ten_seconds`, was started on the old code too, and the checker stopped it after 10 minutes without a result. Run by hand, the old debug binary had not exited after 60 seconds on input 3, in the human format and in the JSON format, so that test cannot pass there either → `0003 AC-15`
- [x] The old release binary on the three inputs, for comparison → input 1 takes 4.8 to 6.9 s, input 2 takes 30 to 31 s, input 3 takes 39 to 42 s. The new one takes at most 0.1 s. The outputs are the same byte for byte, also the 17268 findings of input 3
- [x] `LineIndex` against `line_column` and against a third count written by the checker → a test module added to a private clone compares the three for every byte offset, also 600 offsets past the end, of 56 texts: empty, one scalar, CRLF, tabs, scalars of two, three and four bytes, lines of 255, 256 and 257 bytes, a line of 8000 bytes, 3000 empty lines, and 40 texts from a fixed seed. 156160 offsets, all equal → `0003 AC-2`, `0003 AC-3`
- [x] Positions through the binary on the checker's own large files, each compared with the checker's own count of line and column → all equal, in both formats and both builds:

  | Shape, at the 1 MiB limit | Findings | Last position |
  |---|---|---|
  | Input 3 after one comment line of 524 KB with `é€😀` | 8772 | line 3, column 524106 |
  | Inline entries 200 spaces apart on one line | 4040 | line 2, column 1048186 |
  | One line of identities with `é€😀` in each name, `routing_profile = "replica"` | 7503 | line 2, column 958166 |
  | Input 1 with CRLF line ends and `port = 0` | 12745 | line 76479, column 8 |
  | Nodes after 8900 comment lines, `port = 0` | 2097 | line 21503, column 8 |
  | Nodes after one comment line of 524 KB, `port = 0` | 6925 | line 41560, column 8 |
  | Nodes after more than a million empty lines | 542 | line 1030201, column 11 |
  | Every node names an unknown cluster | 13216 | line 79309, column 11 |
  | Every identity names an unknown cluster | 7018 | line 56157, column 11 |
  | Every node repeats one `id` | 15812 | each later `id` |
  | Every identity repeats one pair | 7202 | line 57635, column 13 |
  | `node = [{},{},...]` on one line | 1398064 | line 2, column 1048554 |
  | `[[node]]` and nothing else, with LF and with CRLF | 466024 and 419420 | line 116507 and line 104856, column 1 |

- [x] Time against size → each shape was built at a quarter, at half and at the full limit, and run through the debug and the release build in both formats. 37 shapes in the first set, 9 more in a second set. For every shape the time about doubles when the size doubles. Three rows that looked worse in the first pass (`crlf` with findings, repeated endpoints, `[[node]]` with CRLF) were run three more times and are linear. Debug build, human format, seconds:

  | Shape | Quarter | Half | Full |
  |---|---|---|---|
  | Input 1, many lines | 0.12 | 0.22 | 0.45 |
  | Input 2, one line | 0.12 | 0.23 | 0.54 |
  | Input 3, one line, every entry wrong | 0.13 | 0.26 | 0.51 |
  | CRLF, every entry wrong | 0.12 | 0.24 | 0.49 |
  | One comment line of half the file in front, every entry wrong | 0.08 | 0.15 | 0.33 |
  | One line of identities with multi byte names, every entry wrong | 0.11 | 0.22 | 0.42 |
  | Every node repeats one `id` | 0.15 | 0.29 | 0.56 |
  | Every node repeats one endpoint | 0.13 | 0.26 | 0.54 |
  | Every node names an unknown cluster | 0.12 | 0.24 | 0.47 |
  | Identities only, valid | 0.10 | 0.20 | 0.39 |
  | One cluster per node, valid | 0.11 | 0.22 | 0.46 |
  | `[[node]]` and nothing else (466024 findings) | 0.45 | 1.09 | 2.78 |
  | `node = [{},...]` (1398064 findings) | 1.12 | 2.09 | 4.50 |
  | `identity = [{},...]` (2097084 findings) | 1.53 | 3.00 | 6.47 |
  | One syntax error at the end of input 1 | 0.04 | 0.07 | 0.15 |
  | One unknown key at the end of input 2 | 0.11 | 0.18 | 0.35 |
  | `[t0]`, `[t1]`, ... (parser only, then one schema error) | 0.20 | 0.40 | 0.78 |
  | `[x.t0]`, `[x.t1]`, ... | 0.18 | 0.36 | 0.75 |
  | One inline table with every key different | 0.17 | 0.35 | 0.70 |

  The other shapes are: comment lines in front of valid entries, a million empty lines, a duplicate key at the end, an unknown key in the last entry, a repeated key in every `[[node]]`, dotted keys, an array of integers, an array of empty arrays, nested arrays of tables, headers that grow deeper, tables under a parent that is never written, a `state_directory` of 1 MiB as one string and as a multi line string, a string of escapes, and comment lines of one byte. None takes longer than 1 second in the debug build at the limit, and none grows faster than the file → `0003 AC-15`
- [x] The old binary against the new one on the checker's shapes → 74 comparisons at a quarter of the limit and 14 at the full limit (inputs 1 to 4, CRLF with findings, the multi byte line, the long comment line): standard output, standard error and exit status are equal in all 88
- [x] `std::time::Instant` → `grep -rn 'Instant\|SystemTime' crates apps tools --include='*.rs'` names two files only, `crates/pool-config/tests/validate.rs` and `apps/pool/tests/check_config.rs`. No file under `crates/pool-config/src` or `crates/pool-control/src` reads a clock, also not in a test module → `0003 AC-11`, `0003 AC-15`
- [x] The tests against the spec text, by reading → the tests build inputs 1 to 3 by the rule of the spec, check the time after each input with the constant 10 seconds, read the columns of input 3 from the text they built (every finding, not the first and the last only), and build input 4 from `valid-topology-minimal.toml`. The binary test writes its input under `env!("CARGO_TARGET_TMPDIR")` and checks both formats. This is what [Validation time](index.md#validation-time), [Value sourcing](index.md#value-sourcing) and the table under [Tests and who owns them](index.md#tests-and-who-owns-them) ask

**Step 9, the commands of the record**

- [x] `cargo test -p pool-control --locked ac16_` → exit 0, 3 passed: `ac16_a_ready_state_with_a_configuration_without_witness_is_unconfirmed`, `ac16_a_ready_state_with_a_configuration_of_another_witness_is_a_mismatch`, `ac16_the_witness_check_comes_before_every_other_reason` → `0003 AC-16`
- [x] `cargo test -p pool-control --locked` → exit 0. `tests/walkthroughs.rs` 26 passed, `tests/admission_model.rs` 3 passed, doc tests 4 passed. The four walks and `open_only_on_full_agreement` are among them → `0003 AC-7` to `0003 AC-12`
- [x] `grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src` → no output, exit 1 → `0003 AC-11`
- [x] The code, by reading → `writer_admission` in `crates/pool-control/src/admission.rs` returns the reason of the state for every state but `Ready`. In `Ready` it compares `config.control()` and its `authority_id()` with `authority.authority_id()` before it builds the evidence or looks up the cluster. `None` gives `Unconfirmed` alone, another value gives `Mismatch` alone. The documentation comment says so and no longer says that every rule runs in `Ready`. No signature and no reason name changed (the only code file in `git diff c10e550 8135cca` is this one, 17 lines added and 4 removed) → `0003 AC-16`

**Step 9, extra checks**

- [x] The checker's own probe, written from the paragraph on the witness under [Writer admission](index.md#writer-admission) → the whole domain of the milestone 3 probe (65536 cases: `expected_writer`, approved writer, journal timeline, quarantine, latch, configured system identifier, and eight observations for each of three nodes) was run with a state classified `Ready` from witness `X` and a configuration that differs only in `control.authority_id`. With no witness, every case gives `authority.unconfirmed` alone. With witness `Y`, every case gives `authority.mismatch` alone. Each case was asked for the cluster `main` and for a cluster the configuration does not have, so 262144 calls in all, and `latches_conflict()` is false in every one → `0003 AC-16`
- [x] More shapes → a configuration with no `[control]` table and no cluster gives `authority.unconfirmed` alone. So does one with a `state_directory`, no witness and no cluster. A witness one hexadecimal digit away gives `authority.mismatch`. A configuration that also differs in its clusters, nodes and `expected_writer` gives the one witness reason for every cluster asked. A second configuration value with the same witness `X` opens `pg1` as before. A state that is not `Ready` keeps its own reason with every configuration: `Uninitialized`, `Missing`, `Unreadable`, `Unconfirmed`, `Mismatch` and `Held` were each asked with the configurations of witness `X`, of witness `Y`, of no witness and of no `[control]` table → `0003 AC-16`
- [x] With the configuration the state was classified from, nothing changed → the checker's model of milestone 3 passes unchanged on `aaad537`: the exact list of reasons in 65536 cases, 474 open answers, the classification table, the freshness cases, two clusters and the bootstrap table → `0003 AC-7` to `0003 AC-12`, `0003 AC-16`
- [x] Mutations of the witness check, in a private clone, against the tests of the repository → `admission.rs` of `c10e550` (before step 9) fails all three `ac16_` tests. A missing witness let through fails two of them, another witness let through fails two, and the witness compared after the cluster lookup fails all three. No other test of `tests/walkthroughs.rs` or `tests/admission_model.rs` fails, so the walks do not depend on the change. The file was put back and all tests pass again

**Findings outside the criteria**

- **A file within the limit can give two million findings and 190 MB of output.** `identity = [{},{},...]` on one line holds 349514 empty entries in 1 MiB. Each one gives six `config.missing_key` findings, 2097084 in all. The time is linear in the file, so the rule under [Validation time](index.md#validation-time) holds. But the report is 190709698 bytes in the human format and 251525214 bytes in the JSON format, and the human format writes one line per call to the operating system. Measured on the busy machine: 6.5 to 8.2 seconds in the debug build and 9.4 and 13.8 seconds in two runs of the release build for the human format, 1.5 and 3.3 seconds for the JSON format in the release build. `0003 AC-15` names inputs 1 to 3 only, and they stay below 1 second, so the criterion is met. The checker leaves two questions to the review and the architect: whether standard output should be written through a buffer, and whether a run should stop collecting after some number of findings. Neither is needed for this slice
- **The record of the tester counts 91 tests in `crates/pool-config/tests/validate.rs` and 257 in all.** The tree has 95 and 261: `grep -c '#\[test\]'` gives 75 for `79de174` and 95 for `32a7b24`, `26210ce` and `aaad537`, and `scripts/check.sh all` runs 95 in that file. The 20 new tests are the 16 that the table "What each group locks" names for `validate.rs` plus the four `ac15_` tests. The table under "Test counts" is off by four. It changes no result. The tester owns that section
- **A word in three test comments.** The comments of the `ac15_` tests say `0003 AC-15 (proposed)`. The criterion is in the accepted spec now. A note for the review

**Readings the checker assessed**

- **A stored count every 256 bytes plus a binary search is "any other way with the same bound".** The checker accepts it. A lookup is one binary search over the line starts and two scans of at most 255 bytes each, one from the stored count before the offset and one from the stored count before the start of its line. So no lookup counts from the top of the file or from the start of a line, which is the first rule of [Validation time](index.md#validation-time), and a line that holds the whole file costs the same as a short one, which is the second. Building the index walks the file twice. The total is the size of the file plus the number of lookups times a constant plus a logarithm, so it stays inside the bound. The measurements agree for every shape tried. The one position of a syntax or schema error still uses the old `line_column`, once per run, as the spec allows
- **Is another path still quadratic below 1 MiB?** The checker found none. The cross checks use hash sets and maps (measured: 15812 repeated ids, 13759 repeated endpoints, 13216 unknown references, 7202 repeated pairs, 6427 wrong writers, each linear). The identity index is a sorted map. The report is one pass. The `toml` parser stayed linear on every parser shape above. The rule leaves the parser out, and the checker did not find a reason to bring it in
- **The witness check runs before the cluster lookup, so `authority.cluster_unknown` does not appear with a wrong witness.** The checker accepts it. It is the text of `0003 AC-16` ("when the cluster asked for is not in the configuration: the checks for `Ready` do not run") and of the paragraph under [Writer admission](index.md#writer-admission) ("The reason is reported alone"). It is also the safe order: a configuration that does not belong to the journal should not be asked which clusters it has
- **The tester wrote three `ac16_` tests where Build plan step 9 names two cases.** The checker accepts it. The two cases of the plan are the first two tests. The third adds a configuration that differs in more than the witness. The developer wrote no test for step 9, and the brief of the coordination gave the tests to the tester

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0003 AC-15` | met on the workstation: the three inputs have the listed entries and bytes, `validate` and the binary stay far below 10 seconds in the debug build, input 3 gives one finding per entry at the column of its value, the binary exits 1 in both formats with 17268 findings, and input 4 gives its two findings against file order. The tests fail on the code before the fix |
| `0003 AC-16` | met on the workstation: no witness gives `authority.unconfirmed` alone and another witness gives `authority.mismatch` alone, for healthy observations, for a second primary and for an unknown cluster, never with the latch, over 262144 calls. With the configuration of the state, every earlier result is unchanged |
| `0003 AC-2`, `0003 AC-3` (positions) | met on the workstation: no position changed for any fixture or probe file, and every position agrees with the checker's own count |
| `0003 AC-7` to `0003 AC-12` | met on the workstation: unchanged with the matching witness |

## Test it: tests added by /test

Written by `tst-f3-test` (tester, `claude-opus-5-5 · high`) on 2026-11-15. The tests are in commits `32a7b24` (steps 1 to 8) and `26210ce` (step 9); every command below without another revision ran on `26210ce`, which holds `main` at `eaaef95`, with steps 8 and 9. The tests add edges and error cases and do not repeat the walks. Every new test names the criterion it locks in its name, in a `covers:` comment or in the comment of its test module. No production line, manifest, `Cargo.lock`, fixture or baseline changed: the source files of `pool-config` and `pool-repo-check` only gained `#[cfg(test)]` modules at their end.

### Commands

| Command | Revision | Expected | Observed | Result |
|---|---|---|---|---|
| `scripts/check.sh all` | `26210ce` | every check ok | exit 0, `ok` for fmt, clippy, test, deny, guards and docs; 448 tests passed, 0 failed | pass |
| `scripts/prove-checks.sh` | `26210ce` | every case rejects its change | exit 0, cases 1 to 19 and 21 to 28 `PASS`, case 20 skipped (run without `--with-linux-build`), `all checks reject their bad change` | pass |
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` | `26210ce` | every test passes | 100 and 110 passed | pass |
| `cargo test -p pool-config -p pool --locked ac15_` | `26210ce` | every `ac15_` test passes within its limit | 4 passed in 1.82 s (`validate.rs`), 1 passed in 1.08 s (`check_config.rs`) | pass |
| `cargo test -p pool-control --locked ac16_` | `26210ce` | the three `ac16_` tests pass | 3 passed | pass |
| `cargo test -p pool-control --locked` | `26210ce` | the walks and the small domain unchanged | `walkthroughs.rs` 26 passed, `admission_model.rs` 3 passed | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | `26210ce` | exit 0 | exit 0, the valid report | pass |
| `cargo test -p pool-config --locked --test validate -- ac15_ ac3_large --test-threads 1` | `32a7b24`, before step 8 reached `main` | the `ac15_` tests fail on the old position lookup | `ac15_large_valid_inputs_validate_within_ten_seconds ... FAILED`; the run took 13 minutes | fail as expected, finding 1 |
| The same tests, `scripts/check.sh all` and `scripts/prove-checks.sh` with `be8bcf1` merged on `32a7b24`, in a scratch copy | `176bd7b` | pass | `ac15_` 4 passed in 2.79 s and 1 passed in 1.31 s; `check.sh all` exit 0 with 445 tests; `prove-checks.sh` exit 0 | pass |
| Mutation probes, see below | `32a7b24` and `26210ce` | each mutation fails a test | 111 of 112 caught; the one left is equivalent | pass |

### Test counts

`#[test]` functions per file, counted with `grep -c '#\[test\]'`: before on `79de174`, after the first round on `26210ce`, and after the round for the review gaps (below). The first version of this table said 91 and 257 for the second column; `validate.rs` has 95 on `26210ce`, so the total is 261 (review n6). The row for `lib.rs` is new; without it the totals are 198, 261 and 268.

| File | Before | After `26210ce` | After the review gaps |
|---|---|---|---|
| `crates/pool-config/tests/validate.rs` | 75 | 95 | 101 |
| `crates/pool-config/src/lib.rs` (unit) | 3 | 3 | 6 |
| `crates/pool-config/src/identifier.rs` (unit) | 7 | 11 | 11 |
| `crates/pool-config/src/generation.rs` (unit) | 4 | 7 | 7 |
| `crates/pool-config/src/topology.rs` (unit) | 0 | 4 | 4 |
| `apps/pool/tests/check_config.rs` | 74 | 80 | 81 |
| `crates/pool-control/tests/walkthroughs.rs` | 7 | 26 | 26 |
| `crates/pool-control/tests/admission_model.rs` | 1 | 3 | 3 |
| `tools/repo-check/src/workspace.rs` (unit) | 30 | 35 | 35 |
| Total | 201 | 264 | 274 |

The existing `open_only_on_full_agreement` now asserts the exact number of open answers, 130, worked out by hand from the rules (65 per approved writer), where it asserted only that one opened. On every closed answer of its 8192 cases it also compares the exact list of reasons with `expected_reasons`, a rule of the test written from the table under "Writer admission", where it checked only that the list was not empty. `scripts/check.sh all` passes after that change too: 448 tests, 0 failed.

### What each group locks

| Group | Criteria |
|---|---|
| `walkthroughs.rs`: `ac11_freshness_is_exact_at_the_expiry_and_at_now`, `ac11_bootstrap_uses_the_same_freshness_rule`, `ac11_the_latest_sample_decides_even_when_it_is_from_the_future`, `ac11_a_tie_hides_evidence_only_at_the_latest_time` | `0003 AC-11`: age equal to the expiry and one nanosecond below, a sample at now and one nanosecond later, an expiry of zero, a future latest sample hiding an older fresh one, ties only at the latest time, a failed latest sample after a good one, the same rules in bootstrap |
| `walkthroughs.rs`: `ac9_a_configured_node_outside_the_journal_members_still_counts`, `ac9_a_second_cluster_does_not_affect_the_first`, `ac9_cluster_unknown_is_reported_alone`, `ac9_every_ready_reason_is_reported_once_in_the_listed_order`, `ac9_a_writer_with_another_identity_skips_the_recovery_and_timeline_checks`, `ac9_other_nodes_with_another_system_identifier_are_not_a_conflict`, `ac9_a_quarantined_node_never_closes_and_never_opens`, `ac9_timelines_compare_with_the_journal_not_with_each_other` | `0003 AC-9`, `0003 AC-12`: a second cluster with the same system identifier, quarantine rows of another cluster, a configured node that is not a journal member, every reason of checks 2 to 10 at once in order, each reason once however many nodes cause it, the skips after checks 5 and 6, a journal timeline other than 1, the largest timeline |
| `walkthroughs.rs`: `ac7_ac8_ac10_start_classification_edges`, `ac7_ac8_bootstrap_reasons_combine_in_the_listed_order`, `ac7_bootstrap_checks_six_to_eight_look_only_at_fitting_nodes`, `ac7_bootstrap_judges_each_cluster_on_its_own_and_ignores_unconfigured_nodes` | `0003 AC-7`, `AC-8`, `AC-10`: `Unreadable` without a witness, a held journal without a witness (`Unconfirmed`) and of another identity (`Mismatch`), a witness one bit off, `Ready` carrying the stored revision and start sequence, two and seven bootstrap reasons together in order, two clusters planned in file order |
| `walkthroughs.rs`: `ac16_a_ready_state_with_a_configuration_without_witness_is_unconfirmed`, `ac16_a_ready_state_with_a_configuration_of_another_witness_is_a_mismatch`, `ac16_the_witness_check_comes_before_every_other_reason` | `0003 AC-16`: one reason and no latch for healthy observations, for a second primary and for a cluster the configuration does not have; unchanged answers with the configuration the state was classified from; the witness reason alone also when the configuration differs in more than the witness |
| `admission_model.rs`: `the_answer_does_not_depend_on_the_order_or_on_older_samples`, `open_carries_the_epoch_and_revision_of_the_journal` | `0003 AC-11`, `AC-12`: the domain answers again with the samples reversed and with an older conflicting sample of every node in front and behind; `Open` carries epoch 9223372036854775807 and revision 42 from the journal |
| `validate.rs`: `ac2_port_edges`, `ac2_host_edges`, `ac2_ac6_id_edges_through_validate`, `ac2_system_identifier_edges_through_validate`, `ac2_authority_id_edges_through_validate`, `ac2_name_syntax_edges`, `ac2_path_syntax_edges`, `ac2_routing_profile_edges`, `ac1_ac2_empty_and_inline_tables` | `0003 AC-1`, `AC-2`, `AC-6`: port 1, 65535, 0, `-0`, 65536; host of 253 and 254 bytes; IDs of 63 and 64 bytes and letters of other scripts that look like ASCII; system identifiers with a sign, spaces, other digits; an authority ID of 32 bytes that are not 32 hex digits; names and paths counted in bytes with every listed control character; routing profile spellings; empty and inline tables, a cluster without `[control]`, empty entries |
| `validate.rs`: `ac3_findings_follow_the_validation_order_not_the_file_order`, `ac3_each_repeat_is_reported_at_each_later_entry`, `ac5_resolve_identity_refuses_names_that_only_look_alike`, `ac5_an_empty_identity_map_resolves_nothing` | `0003 AC-3`, `AC-5`: finding order when the tables come in reverse order, three equal ids, an id and a pair repeated at once; lookups with NFD, other scripts, NUL, tab, zero width space, prefixes, the backend names |
| `validate.rs`: `ac15_made_inputs_have_the_listed_size`, `ac15_large_valid_inputs_validate_within_ten_seconds`, `ac15_one_finding_per_entry_on_one_line_at_each_value`, `ac15_findings_follow_the_validation_order_against_the_file_order`, `ac3_large_file_positions_stay_exact_deep_in_many_lines`, `ac3_a_finding_after_a_long_comment_has_its_exact_position`, `ac3_positions_count_scalars_with_crlf_and_at_block_boundaries` | `0003 AC-15`, `AC-2`, `AC-3`: the four made inputs of "Validation time" with the listed entries and bytes (13217 and 1048511, 16471 and 1048558, 17268 and 1048559), every column of input 3 read from the built text, input 4; positions deep in many lines, after a comment of 900 KB, with CRLF and é€😀 before the finding, and at byte offsets 255, 256, 257, 511, 512 and 513 |
| `identifier.rs`, `generation.rs`, `topology.rs` unit modules | `0003 AC-2`, `AC-4`, `AC-6`: each generation type at 9223372036854775806 and 9223372036854775807, and 1000 steps of one; IDs of other scripts; every byte value of an authority ID through its text; the name, path and host syntax for every ASCII byte and control character; the `Debug` of the identity index prints the count only |
| `check_config.rs`: `ac2_a_control_finding_prints_its_key_without_a_position`, `ac2_an_inline_entry_missing_keys_is_reported_at_its_brace_in_both_formats`, `ac2_ac3_every_new_code_in_one_run_in_the_validation_order`, `ac4_a_canary_in_any_topology_value_never_reaches_the_output`, `ac1_made_topology_files_at_the_edges_report_exactly_the_valid_report`, `ac15_the_binary_reports_every_finding_of_input_3_within_ten_seconds` | `0003 AC-1` to `AC-4`, `AC-15`: the key only line and object, the fourth location shape at a brace, the six new codes and a missing key in one run in both formats, the canary in every topology value on exit 0 and 1, values at their edge and one byte past it, input 3 through the binary |
| `workspace.rs` unit module `lockfile_edge_tests` | `0003 AC-13`: a local package that is not a member at the start, middle and end of `Cargo.lock`, members anywhere, CRLF, indentation, `source` before `name`, a member name in another letter case, one line per package, and the table after the last entry (finding 2 below) |

### Mutation probes

Each mutation changed one line or one clause of production code in the worktree, ran the package's tests, and was put back; `git status` showed only test files afterwards. Nothing of it was committed.

| Package | Mutations | Caught | Not caught |
|---|---|---|---|
| `pool-control`, the tester's list, on `32a7b24` | 31: each closed reason dropped one by one (quarantine, conflict latch, a member missing from the configuration, a later timeline, another system identifier, `expected_writer` against the journal, the `restore_pending` hold), the expiry boundary, future samples, ties, the latest sample, quarantine and nodes of another cluster, every bootstrap refusal, the consistency check, the latch for a later timeline, a failed sample as evidence | 31 | none |
| `pool-control`, the 60 of `chk-f3-verify` (milestone 3), 13 of which no test caught before, on `32a7b24` | 60 | 59 | `A26`, which the checker marks as equivalent (an open answer names `expected_writer`, which then equals the approved writer) |
| `pool-control`, the witness check of step 9, on `26210ce` | 3: no witness check, no mismatch check, the witness checked after the cluster | 3 | none |
| `pool-config`, on `32a7b24` | 18: port 0, the byte limits of host, name, path and ID, DEL and U+001F, `_` in a host, endpoint case, `replica` without case, a leading digit, a leading zero, both generation bounds, a cluster without `[control]`, the duplicate pair without the database, the skip of check 2, an upper case authority ID | 18 | none |

The `pool-config` run skipped the three slow tests, because on `32a7b24` they took minutes.

### Findings

1. **`validate` took time that grew with the square of the number of entries** (`0003 AC-15`). On `32a7b24`, before step 8, `ac15_large_valid_inputs_validate_within_ten_seconds` failed. Step 8 (`be8bcf1`, on `main` through `ebc9228`) fixes it; on `26210ce` every `ac15_` test passes in under 2 seconds.
2. **W4 counts a table after the last entry as part of that entry.** `w4_a_table_after_the_last_entry_counts_as_part_of_it` pins it: a local package that is not a member, written last and followed by a `[[patch.unused]]` table with its own `source` line, is not reported. By the text of spec 0003 ("an entry is the lines from one `[[package]]` line to the next, or to the end of the file") this is the specified behaviour, not a defect. It is a gap: Cargo writes such tables after the packages. Changing it changes the rule text, so it is for the architect.

### Review gaps: tests added after the review of 2026-11-15

Written by `tst-f3-test` (tester, `claude-opus-5-5 · high`) on 2026-11-15, for t1, t2, t4, t5 and the test nits n5, n6 and n17 of [the review](../../reviews/2026-11-15-scope-003-control-and-session-data-model.md). The tests were written on `main` at `d2e8548` (commit `8425a28`); `scripts/check.sh all` ran again with the same result on `3f1bb9b`, which merges `main` at `455b1a5` (document changes only). No production line changed: `crates/pool-config/src/lib.rs` gained the module `line_index_tests` at its end, and the `edge_tests` module of `topology.rs` changed inside.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | every check ok | exit 0, `ok` for fmt, clippy, test, deny, guards and docs; 458 tests passed, 0 failed | pass |
| `cargo test -p pool-config --locked --test validate -- --skip ac15_`, once per mutation of `topology.rs` below | each mutation fails a new test | 4 of 4 caught | pass |
| `cargo test -p pool-config --locked --lib line_index`, once per mutation of `LineIndex` below | each mutation fails a new test | 3 of 3 caught | pass |
| `cargo test -p pool-control --locked --doc` with `compile_fail,E0451` at `admission.rs:89` changed to `E0308` | the doc test fails | 4 passed: the code after the comma is not checked | as the review says, t3 |

| Group | Criteria | Mutation it catches (put back after each run) |
|---|---|---|
| `validate.rs`: `ac2_a_repeated_cluster_id_is_reported_at_each_later_entry`, `ac3_repeated_ids_are_reported_clusters_then_nodes_then_identities` | `0003 AC-2`, `AC-3`: a repeated cluster `id` at each later entry; cross check 1 runs over clusters, nodes, identities even when the file has the tables in the reverse order (t1) | the call of `duplicate_ids` for clusters removed (`topology.rs:688`) |
| `validate.rs`: `ac2_a_bad_cluster_id_skips_only_the_writer_check_of_that_cluster` | `0003 AC-2`: the `expected_writer` check still runs for every other cluster (t2) | `every_cluster_id_valid &&` added to the condition at `topology.rs:731` |
| `validate.rs`: `ac2_a_node_with_a_bad_cluster_skips_every_writer_check` | `0003 AC-2`: a node with a bad `cluster` skips cross check 3 as a whole, also when it is not the writer (t2) | `&& entry.cluster.is_some()` removed at `topology.rs:728` |
| `validate.rs`: `ac2_a_node_whose_port_failed_is_left_out_of_the_endpoint_check` | `0003 AC-2`, `AC-15`: two nodes with one host and port 0 give the two port findings only (t2) | `entry.port` read as `Some(entry.port.unwrap_or(0))` at `topology.rs:772` |
| `validate.rs`: `ac4_a_canary_outside_a_value_never_reaches_rejection_text`; `check_config.rs`: `ac4_a_canary_outside_a_value_never_reaches_the_output` | `0003 AC-4`: the canary in a key that is not in the schema inside `[control]`, `[[cluster]]`, `[[node]]`, `[[identity]]` and an inline entry, a quoted and a dotted key, a table and an array table name, `[control.<canary>]`, a comment, an array entry, values of the wrong type, a string that does not end and a key without a value; refused, and no part of the canary in `Debug`, `Display` or either output format (t4) | none run; the code prints no file text today |
| `lib.rs` unit module `line_index_tests` | `0003 AC-3`, `AC-15`: `LineIndex::line_column` equals `line_column` at every byte offset up to two past the end of texts with CRLF, tabs, empty lines, lines of more than 256 and 512 bytes, lengths of 256 and 512, and two, three and four byte characters across the strides at 256, 512 and 768; `line_column` equals a count by `str` methods at every char boundary; offsets at and past the end, `usize::MAX`, the empty text (t5) | the tail counted in bytes instead of scalars; `start < offset` in place of `start <= offset`; the offset clamped one byte short of the end |
| `validate.rs`: `ac3_positions_count_scalars_with_crlf_and_at_block_boundaries`, `ac4_validate_opens_no_file`; `walkthroughs.rs`: `bootstrap_refusals_need_every_node_and_a_cluster`; `topology.rs` `edge_tests` | n17: the loop asserts every target instead of skipping one with `continue`; `validate` also accepts paths that exist (a fixture holding the canary as the credential file, the package directory as the state directory) and the canary stays out of `Debug`; the exact name `bootstrap.no_cluster`; the allowed host bytes written out instead of taken from the code | none run |

Eight comment lines in the tests said `0003 AC-15` was proposed or not yet accepted (n5); they now name `0003 AC-15`.

The `compile_fail` doc tests (`admission.rs:89`, `:137`, `authority.rs:128`, `bootstrap.rs:57`, `lib.rs:44`) prove only that their snippet does not compile. Rust 1.100.0 on the stable channel does not check the error code after the comma: with `E0308` written in place of `E0451` the test still passed. So the sentence under milestone 3 that these tests "cannot pass for an unrelated compile error" does not hold, and the rows that cite "the four `compile_fail,E0451` doc tests" for `0003 AC-12` rest on each snippet having no other error. The `ValidatedConfig` snippet at `lib.rs:44` names three of the eight fields: with every field made public it still fails to compile (missing fields), so it no longer guards the private fields. A doc comment change that closes this is reported to the lead for the developer, because the tests sit in production files.

## Full verification: /check verify

Recorded by the checker session `chk-f3-verify` on 2026-11-15 for revision `4c61e3b` (`main`), on macOS 27.0.1, arm64 (workstation, not CI). Docker was running (`docker info` exit 0, server 29.9.2, `arm64`), so the Linux build ran in a container. This run covers the whole feature, `0003 AC-1` to `0003 AC-16`, and closes the scope boxes "Build it" and "Verify it" as far as a workstation can. The checker wrote no code and no test. Every command was run from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. Probes that change a file ran in private clones outside the repository.

**Verdict: PASS.** All sixteen criteria are met on the workstation, and the workspace tests pass in the Linux container. One kind of evidence is not there: the seven checks on a pull request. It is listed under "Not run" with its owner.

**Two revisions.** The run started on `aaad537`, where the checkout had the same tree as `main`. While it ran, `main` moved to `4c61e3b`. `git diff aaad537 4c61e3b` names one file under `apps`, `crates` and `tools`, the test file `crates/pool-control/tests/admission_model.rs` (73 lines added: `open_only_on_full_agreement` now compares the exact list of reasons of every closed answer with a rule of its own). Beside it there are two lines in the tester's part of this file and two files under `docs/scope/`. No production code, manifest, script, baseline or fixture changed. The checker merged `main` and ran these again on its checkout `e7c914d`, whose tree is the tree of `4c61e3b` plus the first version of this section (`git diff 4c61e3b e7c914d` names this file only): `scripts/check.sh all`, `cargo test -p pool-control --locked`, `scripts/check.sh linux-build`, `scripts/prove-checks.sh --with-linux-build` and the 60 mutations. Every other step below ran on `aaad537`, and no file on its path differs in `4c61e3b`.

**What was run again, and what was not.** The checker runs of milestones 1 to 4 above passed at `60fcb08`, `198a1cd` and `d26016c`. A step of those runs is not repeated here unless code on its path changed. Since `d26016c` the production code changed in three files only:

- `crates/pool-config/src/lib.rs` and `crates/pool-config/src/topology.rs` (step 8, `be8bcf1`): how a span becomes a line and a column. This is on the path of every finding with a position, so the fixture steps of `0003 AC-1` to `0003 AC-4` were run again.
- `crates/pool-control/src/admission.rs` (step 9, `8135cca`): the witness check. This is on the path of every admission in `Ready`, so the walks, the small domain and the checker's model of `0003 AC-7` to `0003 AC-12` were run again.

Everything else that `git diff d26016c aaad537 -- apps crates tools` shows is test code. `git diff eaaef95 aaad537 --numstat -- crates apps tools` (the merge of the tester) names eight files with 2585 lines added and none removed: four test files, and four source files that each gained one `#[cfg(test)]` module after their last line (`generation.rs`, `identifier.rs` and `topology.rs` of `pool-config`, and `tools/repo-check/src/workspace.rs`). No production line changed in that merge. `git diff 2648ae1 aaad537` is empty for `Cargo.toml`, `Cargo.lock`, every member manifest, `deny.toml`, `scripts/`, `.github/`, `tools/repo-check/baseline/`, the manifest and lockfile of `tools/repo-check` and `crates/pool-config/fuzz/`. `git diff d26016c aaad537` is empty for `crates/pool-config/tests/fixtures/`.

### Commands on the final tree

- [x] `scripts/check.sh all` → exit 0 on `aaad537` and again on the tree of `4c61e3b`: `ok` for `fmt`, `clippy`, `test`, `deny`, `guards` and `docs`. 448 tests passed, 0 failed, both times. `deny` shows only the known `license-exception-not-encountered` warning → all criteria
- [x] `scripts/prove-checks.sh --with-linux-build` → exit 0, `prove-checks: commit aaad537eaf61e853158bdfe775d5ae9ce376ddb2`, and again with `prove-checks: commit e7c914dec21228c771a3d90c2f52042f78f3b8ab` (the tree of `4c61e3b`): cases 1 to 28 `PASS`, no `FAIL`, `all checks reject their bad change`, both times. Case 17 matched `D3: docs/scope/004-one-real-primary-path.md:1: status disagrees: done in the heading, planned in docs/scope/index.md`. Case 20 ran the Linux build in Docker → `0003 AC-13`
- [x] `scripts/check.sh linux-build` on the good tree → exit 0, `linux build check passed`, on `aaad537` and again on the tree of `4c61e3b`, with the same numbers and the same binary checksum. Image `rust:1.100.0-slim-bookworm@sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37`, `linux/arm64`, host `aarch64-unknown-linux-gnu`, `rustc 1.100.0`. `cargo test --locked --workspace` in the container: 240 passed, 0 failed, with the five `ac15_` tests (`tests/validate.rs` 95 passed in 4.19 s, `tests/check_config.rs` 80 passed in 1.62 s) and the three `ac16_` tests. The 28 fixtures gave their exit status in both formats, with no canary text. The release binary has the SHA-256 `9e23011ed9c9dfa54461f5161af3d5715c538e919b0a37aa18ccb7330f368c82`. This is Linux evidence for the time limit of `0003 AC-15`, which a run on macOS is not → `0003 AC-1` to `0003 AC-16`
- [x] `scripts/check.sh fuzz` → exit 0, `Done 95672 runs in 61 second(s)`, no crash file (see the note in the checker run of milestone 5)
- [x] `cargo metadata --no-deps --format-version 1 --locked` → exit 0, three workspace members: `pool` (`pool-config`, `serde`, `serde_json`), `pool-config` (`serde`, `toml`), `pool-control` (`pool-config`) → `0003 AC-13`
- [x] `Cargo.lock` → 19 entries, the three workspace packages and sixteen others, unchanged since milestone 1 → `0003 AC-13`
- [x] `cargo test -p pool-config -p pool --locked ac15_` → 4 and 1 passed → `0003 AC-15`
- [x] `cargo test -p pool-control --locked ac16_` → 3 passed → `0003 AC-16`
- [x] `cargo test -p pool-control --locked` on the tree of `4c61e3b` → exit 0. `tests/walkthroughs.rs` 26 passed, `tests/admission_model.rs` 3 passed, doc tests 4 passed
- [x] `cargo test -p pool-control --locked walk_` → 4 passed. `cargo test -p pool-control --locked open_only_on_full_agreement` → 1 passed, with the exact count of 130 open answers. `cargo test -p pool-control --locked --doc` → 4 passed, each marked `compile fail` → `0003 AC-7` to `0003 AC-12`
- [x] `grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src` → no output, exit 1 → `0003 AC-11`
- [x] `grep -rnE 'std::fs|File::|std::env|std::net|std::process|\.exists\(|metadata\(|Instant|SystemTime' crates/pool-config/src crates/pool-control/src` → no output, exit 1 → `0003 AC-4`, `0003 AC-11`
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` → exit 0. The same for `valid-topology.toml` → exit 0 → `0003 AC-1`

### Steps run again because their path changed

- [x] The thirteen topology fixtures through the built binary, compared with the table under [Fixtures](index.md#fixtures) → each gives the listed exit status, and every code, key, line and column is as listed, in both formats. `invalid-topology-multiple.toml` gives its five findings in the listed order, and `secret-topology-values.toml` its two → `0003 AC-1`, `0003 AC-2`, `0003 AC-3`
- [x] The three `valid-topology*` fixtures → the same bytes as `valid-minimal.toml` in both formats, which is the valid report of spec 0001 → `0003 AC-1`
- [x] All 28 fixtures (13 of this spec and 15 of spec 0001), binary of `d26016c` against the binary of `aaad537` (the same production code as `4c61e3b`) → 168 comparisons of standard output, standard error and exit status, 0 differences → `0003 AC-1`, `0003 AC-2`, `0003 AC-3`
- [x] The canary → `grep` finds `CANARY`, `9f2c7b1e` and `not-a-real` in none of the 56 output files and in no standard error → `0003 AC-4`
- [x] Positions in large files, against the checker's own count → see the checker run of milestone 5: sixteen shapes at the limit, up to line 1030201 and column 1048402, with CRLF and with scalars of two, three and four bytes → `0003 AC-2`, `0003 AC-3`
- [x] The checker's model of milestone 3, unchanged, on the final tree → the exact reasons in 65536 cases with 474 open answers, the classification table, freshness, two clusters, the bootstrap table and the derives: 6 probe tests passed → `0003 AC-7` to `0003 AC-12`
- [x] The 60 mutations of milestone 3, again, against the tests of the repository only → all 60 still apply to the code. 59 make a test fail. The one left is `A26`, the control that changes nothing. Each of the 13 that no test caught before is caught now, by `ac9_timelines_compare_with_the_journal_not_with_each_other`, `ac9_a_second_cluster_does_not_affect_the_first`, `open_carries_the_epoch_and_revision_of_the_journal`, `ac11_a_tie_hides_evidence_only_at_the_latest_time`, `ac11_the_latest_sample_decides_even_when_it_is_from_the_future`, `the_answer_does_not_depend_on_the_order_or_on_older_samples`, `ac7_ac8_ac10_start_classification_edges`, `ac7_bootstrap_checks_six_to_eight_look_only_at_fitting_nodes` or `ac7_bootstrap_judges_each_cluster_on_its_own_and_ignores_unconfigured_nodes`. This agrees with the record of the tester. The run on `4c61e3b` gives the same 59 and the same one → `0003 AC-7` to `0003 AC-12`
- [x] The restart rules → each line that milestone 3 lists for `0003 AC-14` still holds a comment that starts with `Restart rule:` on the named type: `lib.rs:38`, `topology.rs:53`, `:88`, `:117`, `:166`, `:226`, `generation.rs:54`, `:71`, `:80`, `authority.rs:31`, `:52`, `:65`, `:90`, `:134`, `admission.rs:100`, `:143`, `bootstrap.rs:63`, `:81`, `observation.rs:74` → `0003 AC-14`

**The Cargo warning that is left.** Cargo prints one manifest warning, `warning: unused workspace dependency pool-control`, for `Cargo.toml:14`. It ends in scope 004, when `apps/pool` takes the dependency. No other warning appears in `scripts/check.sh all`, on the workstation or in the container.

### Not run

| Evidence | Status | Reason | Owner |
|---|---|---|---|
| The seven checks (`fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards`, `docs`) on a pull request for scope 003 | `NOT_RUN` | The user holds back the permission to write to GitHub (`F3-1` of the manual file, answered "Belum, lokal dulu" on 2026-11-15). No session may push or open a pull request. The same seven commands pass on the workstation, and `linux-build` passes in the container | user |
| The refusal of an unmapped pair on a real connection (the other half of `PRD-001 AC-1`) | outside this slice | `0003 AC-5` asks for the library half only and gives the connection half to scope 004 | scope 004 |

The manual step, as the lead recorded it: with `gh auth status` signed in as the owner, a clean main checkout and `HEAD` on `main`, run `git push origin main:scope/003-control-and-session-data-model`, then `gh pr create --base main --head scope/003-control-and-session-data-model`, wait for `gh pr checks scope/003-control-and-session-data-model`, then `gh pr merge scope/003-control-and-session-data-model --merge` and `git pull --ff-only origin main`. It passes when the seven checks pass on that pull request and it is merged with a merge commit. Only this step can show that the change passes CI on a runner.

Nothing is `BLOCKED`.

### Readings of the tester that the checker assessed

- **`0003 AC-11`: a latest sample from the future hides an older fresh sample.** The checker accepts it, as in milestone 3. The paragraph "Fresh" says that the functions use the observation with the latest `sampled_at`, and that one that is not fresh is no evidence. For the approved writer this closes (`evidence.writer_missing`). For another node it is silence, which does not close. `ac11_the_latest_sample_decides_even_when_it_is_from_the_future` pins it
- **`0003 AC-2`: `port = -0` is `config.invalid_value`.** The checker accepts it. Observed through the binary: `port = -0`, `port = +0`, `port = 0` and `port = 0x0` each give `config.invalid_value` at `node.port`, line 15, column 8, and `port = -1` gives `config.schema`. In TOML, `-0` and `+0` are the integer 0, so the row "A port of 0" applies, not the row for `port = -1`
- **`0003 AC-2`: a name with U+0080 to U+009F is valid.** The code follows the text: the [name syntax](index.md#keys) lists "U+0000 to U+001F, and U+007F" and nothing else. Observed: a principal with U+001F or U+007F is refused, and one with U+0080, U+009B, U+009F, U+00A0, U+2028 or U+202E is accepted. The checker reads this as a small gap in the text, not as the wrong behaviour of the code. "No control character" in plain words would cover U+0080 to U+009F too, which Unicode also counts as control characters, and the parenthesis narrows it. Today nothing prints a name (`0003 AC-4`), so nothing can go wrong with it in this slice. The checker suggests that the architect either add U+0080 to U+009F to the list, or say in one sentence that the list is complete on purpose and that a later slice which prints a name has to escape it. This is an offer, not a need, and it changes no verdict
- **W4: a table after the last entry counts as part of it.** The checker confirms that the gap is real, with Cargo 1.100.0, in a private clone. A new package `crates/zzz-extra` that is not written in `members`, taken by `pool-control` with a path dependency, plus an unused `[patch.crates-io]` entry that points to a local git repository: `cargo metadata` then lists four workspace members and writes `zzz-extra` as the last `[[package]]` entry, without a `source` line, followed by `[[patch.unused]]` with `name`, `version` and a `source` line. With `zzz-extra` also added to `lock-packages.txt`, `pool-repo-check workspace` prints `ok`. So the addition to W4 does not see this package. Two things have to come together: the package has to sort after every other package by name, and the manifest has to hold an unused patch that is not a path. Rule W1 still reports the package as long as the baseline is not edited in the same change, and a baseline edit is seen in review. The code does what the text of this spec says, so this is not a failed step of `0003 AC-13`. The checker suggests a change of the rule text to the architect: an entry ends at the next line that starts a table (a line that starts with `[`), not only at the next `[[package]]` line. The test `w4_a_table_after_the_last_entry_counts_as_part_of_it` then has to change with it

### Verdict for each criterion

| Criterion | Verdict | Evidence |
|---|---|---|
| `0003 AC-1` | met | Milestone 2 (`198a1cd`). Again in this run: the three `valid-topology*` fixtures give the valid report byte for byte, and all 28 fixtures give the same output as before the fix |
| `0003 AC-2` | met | Milestone 2. Again in this run: the eight single mistake fixtures against the table, positions in large files against the checker's own count, `port = -0` and the control characters through the binary |
| `0003 AC-3` | met | Milestone 2. Again in this run: the five findings of `invalid-topology-multiple.toml` in order. The `compile_fail` doc tests of `pool-config` pass |
| `0003 AC-4` | met | Milestone 2. Again in this run: no canary in any output, and `pool-config` opens no file |
| `0003 AC-5` | library half met | Milestone 2, code unchanged. The tests are in the 448. The connection half belongs to scope 004 |
| `0003 AC-6` | met | Milestone 2, code unchanged. The tests are in the 448 |
| `0003 AC-7` | met | Milestone 3 (`d26016c`). Again in this run: the walk, the classification and bootstrap probes |
| `0003 AC-8` | met | Milestone 3. Again in this run: the walk and the probes |
| `0003 AC-9` | met | Milestone 3. Again in this run: the walk, and the exact reasons in 65536 cases |
| `0003 AC-10` | met | Milestone 3. Again in this run: the walk and the probes |
| `0003 AC-11` | met | Milestone 3. Again in this run: the freshness probe, and both `grep` commands print nothing |
| `0003 AC-12` | met | Milestone 3. Again in this run: 8192 cases with 130 open answers, 65536 cases with 474, and the four `compile_fail` doc tests |
| `0003 AC-13` | met | Milestone 1 (`60fcb08`). Again in this run: `cargo metadata`, the lockfile, `scripts/check.sh all`, `scripts/prove-checks.sh --with-linux-build` and the Linux build. Evidence from CI is `NOT_RUN` |
| `0003 AC-14` | met | Milestones 3 and 4. Again in this run: the restart rules at the listed lines, and `scripts/check.sh docs` |
| `0003 AC-15` | met | The checker run of milestone 5, and the `ac15_` tests on macOS and in the Linux container |
| `0003 AC-16` | met | The checker run of milestone 5, and the `ac16_` tests on macOS and in the Linux container |

Every public type, key, fixture and package that the spec names was found where the spec puts it in milestones 1 to 4, and none of those places changed since. No specced item is missing and none is built but unused, except the workspace dependency that waits for scope 004.

### For the review

- The human report writes one line per call to the operating system, and a file at the limit can give two million findings (the first finding outside the criteria in the checker run of milestone 5).
- The three `ac15_` comments that still say "(proposed)", and the count of 91 where the tree has 95 in the record of the tester.
- The two offers to the architect above: the control characters U+0080 to U+009F in a name, and where a lockfile entry ends for rule W4.
