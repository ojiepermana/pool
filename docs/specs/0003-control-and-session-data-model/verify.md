# 0003. Verification record

This file holds the acceptance evidence for [spec 0003](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

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
