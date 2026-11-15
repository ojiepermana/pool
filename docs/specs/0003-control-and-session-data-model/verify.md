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
