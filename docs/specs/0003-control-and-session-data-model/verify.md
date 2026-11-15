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
