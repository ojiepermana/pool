# 0004. Verification record

This file holds the acceptance evidence for [spec 0004](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

## Milestone 1: Guards, workspace shape and dependencies

Build plan steps 1 and 2, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commits `41d4709` (step 1) and `8d84f50` (step 2), on top of the accepted spec at `560096e` and the scope change `6ad02c0`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Docker was running (`docker info` exit 0). Step 3 waits for manual step M1 and is not part of this record yet.

### UI / manual

None for steps 1 and 2.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` at `41d4709` | exit 0 | exit 0, 100 unit and 110 integration tests passed, 0 failed; `w4_a_table_after_the_last_entry_ends_it` replaces `w4_a_table_after_the_last_entry_counts_as_part_of_it` | pass |
| `scripts/check.sh all` at `41d4709` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok` | pass |
| `scripts/prove-checks.sh --with-linux-build` on commit `41d4709` | every case rejects its change, with scope 004 `in-progress` | `all checks reject their bad change`, cases 1 to 29 `PASS`. Case 17 matched `D3: docs/scope/099-proof-case.md:1: status disagrees: done in the heading, planned in docs/scope/index.md`. Case 29 matched `W4: Cargo.lock: package pool-prove-ghost has no source line but is not a member written in members of Cargo.toml`. Case 20 ran with Docker | pass |
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` at `8d84f50` | exit 0 | exit 0, 106 unit and 110 integration tests passed, 0 failed; six new tests in the module `source_guard_tests` of `tools/repo-check/src/workspace.rs` | pass |
| `scripts/check.sh all` at `8d84f50` (includes `scripts/check.sh guards`) | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`; the new baseline finds nothing in the current tree | pass |
| `scripts/prove-checks.sh --with-linux-build` on commit `8d84f50` | every case rejects its change | `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 ran with Docker. Cases 30 to 35 matched, in order: `crates/pool-control/src/lib.rs:30: forbidden text TcpStream`, `apps/pool/src/main.rs:94: forbidden text UnixStream`, `crates/pool-control/src/lib.rs:30: forbidden text Instant::now`, `crates/pool-control/src/lib.rs:30: forbidden text rusqlite`, `apps/pool/src/main.rs:94: forbidden text StoreProbe`, `crates/pool-config/src/lib.rs:819: forbidden text writer_admission`, each after `W3:` | pass |

**Note on the six groups of `0004 AC-2`.** The proof cases read the sentence of the criterion as six groups of refusal: (30) a socket or the runtime outside `crates/pool-core` and `apps/pool`, (31) a thread or a Unix or UDP socket anywhere, also in `apps/pool`, (32) a clock, a file call or an environment read in `crates/pool-control/src`, (33) `rusqlite` there outside `journal.rs`, (34) a journal name outside `crates/pool-control` (`0003 AC-19`), and (35) `writer_admission` outside it (the guard half of `0003 AC-18`). `crates/pool-protocol` does not exist before step 3, so case 30 uses `crates/pool-control`. That a socket or the runtime is accepted in `crates/pool-core` and `apps/pool`, and that an `allow` line is compared by whole components, is shown by the unit tests `an_allow_line_lifts_the_text_only_at_or_under_its_path` and `source_rule_reads_the_three_forms`, since a proof case can only show a refusal.

**Note on the cargo warning.** Cargo still prints `warning: unused workspace dependency pool-control`. It goes away when a member takes the dependency, in a later step (`0004 AC-1`).

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-4` | `local_packages` in `tools/repo-check/src/workspace.rs` ends an entry at the next line that starts with `[`; test `w4_a_table_after_the_last_entry_ends_it`; case 17 with its own feature 99 and case 29 in `scripts/prove-checks.sh`; `scripts/prove-checks.sh` passes on the committed trees `41d4709` and `8d84f50` with scope 004 `in-progress` | pass on the workstation; CI evidence waits for the pull request |
| `0004 AC-2` | `SourceRule`, `source_rule`, `covers` and `forbidden_in` in `tools/repo-check/src/workspace.rs`; the baseline `tools/repo-check/baseline/forbidden-source-patterns.txt` holds the file of [Source guard](index.md#source-guard); the tests of `source_guard_tests`, one passing and one failing case per kind of line; cases 30 to 35 | pass on the workstation; CI evidence waits for the pull request |
| `0003 AC-19`, guard half of `0003 AC-18` | cases 34 and 35, and the `allow crates/pool-control` lines of the baseline | pass for the guard; the library half of `0003 AC-18` is Build plan step 6 |
| `0004 AC-1` (shape), `0004 AC-3` | Build plan step 3, after manual step M1 | not run |
| `0004 AC-5` to `0004 AC-28` | Build plan steps 4 to 14, other milestones | not run |

## Milestone 2: Configuration and wire format

This section covers Build plan step 5 only, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commit `31f41fd` (step 5), merged with `main` at `03f54a9` in `586edfd`. Step 4 (the wire format) is built by another session and gets its own part of this section. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2` with `nightly-2026-10-17`.

### UI / manual

None for step 5.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-config --locked` | exit 0 | exit 0: 32 unit, 122 in `tests/validate.rs`, 1 doc test plus 5 doc tests (`compile_fail` included) passed, 0 failed | pass |
| `cargo test -p pool --locked` | exit 0 | exit 0: 3 unit and 84 in `tests/check_config.rs` passed, 0 failed | pass |
| `scripts/check.sh all` on `586edfd` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards`, `docs` all `ok` | pass |
| `scripts/check.sh fuzz` on `31f41fd` before the merge | exit 0 | exit 0: target `validate`, 788203 runs in 61 seconds, no crash; the harness compiled unchanged | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0, the valid report | exit 0, `configuration valid` and the three lines of spec 0001 | pass |
| `pool check-config` on each of the twelve new fixtures, human format | the result of the table under [Fixtures](index.md#fixtures) | the four valid ones print exactly the valid report; each invalid one prints its one finding: `listener.port`, `listener.tls_private_key_file` (missing), `limits.session_max_count`, `probe.observation_expiry_ms`, `principal.name` at line 66 column 8 (`config.duplicate_principal`), `config.schema` at line 14 column 1, `identity.backend_login` at line 22 column 17, `identity.database` at line 20 column 12 | pass |
| `pool check-config` on `invalid-unknown-key.toml` after its table became `[no_table]` | `config.schema` at line 3, column 2, as before | the same line in both formats | pass |

**Note on the cargo warning.** Cargo still prints `warning: unused workspace dependency pool-control`, as in milestone 1. It is not a failure and goes away when a member takes the dependency.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-5` | `src/serving.rs` (keys, defaults, ranges, the expiry cross rule, principals), the new keys in `src/topology.rs`, `RawConfig` in `src/lib.rs`; tests `ac5_*` in `tests/validate.rs` (every fixture, every number key at both edges and one past, wrong types, missing listener keys, entry positions, the order of the new checks, principal repeats) and `spec_0004_ac5_*` in `tests/check_config.rs` (exact output in both formats); the tests of spec 0001 and spec 0003 pass unchanged except the three listed below | pass on the workstation |
| `0004 AC-6` | `has_control` in `src/topology.rs` holds the closed list; tests `ac6_every_name_and_path_key_refuses_each_listed_character` (14 keys times the 79 listed characters, canary absent), `ac6_the_neighbours_of_each_listed_range_stay_allowed`, `ac5_ac6_every_new_string_value_keeps_the_canary_out_of_every_text`, the unit test `ac2_name_syntax_counts_bytes_and_refuses_only_the_listed_characters`, and `spec_0004_ac6_the_canary_never_reaches_the_output` | pass on the workstation |
| `0004 AC-7` | `MAX_FINDINGS`, `Findings` and `Rejection::omitted` in `src/lib.rs`; the omitted line and `findings_omitted` in `apps/pool/src/report.rs`; the `BufWriter` in `apps/pool/src/main.rs`; tests `ac7_validate_keeps_the_first_twenty_thousand_and_counts_the_rest` (made input 5), `ac7_at_twenty_thousand_nothing_is_omitted_and_one_more_is_counted`, `spec_0004_ac7_the_binary_writes_twenty_thousand_findings_and_the_omitted_count` (20000 finding lines, under 4 MiB in each format), `spec_0004_ac7_a_report_of_twenty_thousand_has_no_omitted_line_or_field`; `ac15_the_binary_reports_every_finding_of_input_3_within_ten_seconds` still sees 17268 findings | pass on the workstation |
| `0004 AC-8` | `src/views.rs` (`probing`, `serving`, the two views); tests `ac8_views_need_what_each_command_needs`, `ac8_one_missing_monitor_key_or_principal_gives_one_finding`, `ac8_validate_never_gives_incomplete`, and the probe only fixture through the binary (exit 0, valid report) | pass on the workstation |
| `0001 AC-3` to `AC-11`, `0003 AC-1` to `AC-6`, `AC-15` | the existing tests of both test files, unchanged except: `ac2_name_syntax_edges` (U+0085 moved from valid to invalid by the rule of AC-6), `ac4_validated_config_debug_holds_only_states` (the `Debug` text now lists the new fields), and the fixture `invalid-unknown-key.toml` with the two cases of `0001 AC-7` that now name `[no_table]` (architect decision `c789c15`) | pass on the workstation |
| `0004 AC-28` | Build plan step 4, the other part of this milestone | not run here |

**Two readings.** First, the spec puts the monitor keys "after `expected_writer`" and the TLS keys "after `port`" inside the sentence that adds every new check after item 5. The build reads it as one new pass after item 5: listener, limits, probe, then the monitor keys of each cluster, the TLS keys of each node, then each principal. The test `ac5_new_checks_come_after_item_5_and_before_the_cross_checks` locks that order. Second, the first version of the key table let `probe.period_ms` go up to 60000, and above 59900 no expiry could meet the cross rule. That reading went to the architect, and decision 49 lowered the upper bound to 30000, like the timeout. The follow up commit on this branch applies it: 30000 is valid, 30001 to 60000 give `config.invalid_value` at `probe.period_ms`, and a value the type cannot hold is still `config.schema` (tests `ac5_number_keys_accept_their_range_and_refuse_what_is_outside` and `ac5_a_period_above_thirty_thousand_is_refused_at_the_period`). No fixture holds a period above 5000.
