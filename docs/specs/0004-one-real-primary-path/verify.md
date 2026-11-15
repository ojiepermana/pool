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

### Checker run

Build plan steps 1 and 2 only, on 2026-11-15 at 17:14 WIB, by the checker session, on the merged tree `d8797a5` (macOS 27.0.1, arm64; a workstation, not Linux and not CI). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, Docker 29.9.2 running (`docker info` exit 0). Every command below was run in this session. The probes used a private clone of `d8797a5` and the built `pool-repo-check`, and changed nothing in the repository.

**Verdict: step 1 PASS, step 2 PASS.** Step 3 was not built and is outside this run.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` | exit 0 | exit 0, 106 unit and 110 integration tests passed, 0 failed; `w4_a_table_after_the_last_entry_ends_it` and the six tests of `source_guard_tests` are among them | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok` | pass |
| `scripts/prove-checks.sh --with-linux-build` on the committed tree `d8797a5` | every case rejects its change | exit 0, `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 ran with Docker. Case 17 matched `D3: docs/scope/099-proof-case.md:1: status disagrees: done in the heading, planned in docs/scope/index.md`. Case 29 matched `W4: Cargo.lock: package pool-prove-ghost has no source line but is not a member written in members of Cargo.toml`. Cases 30 to 35 matched `W3` with `TcpStream`, `UnixStream`, `Instant::now`, `rusqlite`, `StoreProbe` and `writer_admission` at the file each case changes | pass |
| Probe, case 17 with another status of scope 004: in the clone, set the heading and the index row of feature 4 to `planned`, to `in-progress` and to `done`, apply the edit of case 17, run `pool-repo-check docs` | `D3` and `099-proof-case.md` each time | before the edit `pool-repo-check docs: ok` each time; with the edit exit 1 and the same `D3` line for `099-proof-case.md` each time | pass |
| Probe, the lockfile entry end: apply the edit of case 29 in the clone and run `pool-repo-check workspace` built from `d8797a5`, then the one built from `6ad02c0` | the new program reports `W4`, the old one does not | new: exit 1 with the `W4` line for `pool-prove-ghost`; old: `pool-repo-check workspace: ok`, exit 0. So case 29 fails without the change. A local entry that is the last thing in the file is still found (exit 1, `W4`) | pass |
| Probe, the baseline text: the lines of `tools/repo-check/baseline/forbidden-source-patterns.txt` that are not comments, compared with the block under [Source guard](index.md#source-guard) | no difference | the 32 lines are equal | pass |
| Probe, the six groups of `0004 AC-2`: 125 edits, one at a time, each adds a comment line with one text to one `.rs` file in the clone, then `pool-repo-check workspace` | accepted or rejected as the criterion says | 125 of 125 as expected. `tokio`, `TcpStream`, `TcpListener` and `std::net` are accepted in `crates/pool-core/src/` (also a file two folders down), `apps/pool/src/` and `apps/pool/tests/`, and rejected with `W3` in `crates/pool-config`, `crates/pool-control`, `crates/pool-protocol`, `crates/pool-core-x`, `apps/pool-cli` and `crates/pool-config/fuzz`. `std::thread`, `UdpSocket`, `UnixListener` and `UnixStream` are rejected in all five crate folders and in `apps/pool`. `Instant::now`, `SystemTime`, `std::fs` and `std::env` are rejected in three files under `crates/pool-control/src` (also `journal.rs`) and accepted in `crates/pool-control/tests` and `crates/pool-control-x`. `rusqlite` is accepted in `crates/pool-control/src/journal.rs` and rejected in `lib.rs`, `journal/mod.rs`, `journal.rs.rs` and `deep/journal.rs`. The five journal and admission names are rejected in `apps/pool`, `crates/pool-config`, `crates/pool-core`, `crates/pool-protocol` and `crates/pool-control-x`, and accepted in `crates/pool-control/src` and `crates/pool-control/tests` | pass |
| Probe, the accepting half on a real tree: a file `crates/pool-core/src/lib.rs` with all four socket and runtime texts | no `W3` line | the only line is `W2: crates/pool-core: forbidden path crates/pool-core exists`, which goes away with step 3 | pass |
| Probe, each of the cases 30 to 35 without its rule: apply the edit of the case, then delete the one baseline line that forbids it | rejected with the line, accepted without it | six of six: exit 1 with the line, `pool-repo-check workspace: ok` without it | pass |
| Probe, a baseline line of no known form: append `deny crates/pool-config` to the baseline | a `W3` problem that names the baseline file | `W3: tools/repo-check/baseline/forbidden-source-patterns.txt: line deny crates/pool-config is not <text>, deny <path> <text> or allow <path> <text>`, exit 1 | pass |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-4` | met on the workstation. Case 17 no longer depends on a real feature, the entry ends at the next `[` line, case 29 shows `W4` alone, and the script passes on a committed tree |
| `0004 AC-2` | met on the workstation for the guard. Reading the six groups as six refusals (cases 30 to 35) fits the last sentence of the criterion. The accepting half is shown by the unit tests and by the probes above, because `crates/pool-core` and `crates/pool-protocol` do not exist before step 3 |
| `0003 AC-19`, guard half of `0003 AC-18` | met for the guard. The code half comes with Build plan step 6 and was not checked |
| `0004 AC-1`, `0004 AC-3` | outside this run (step 3) |

Notes that are not failures:

- No committed test compares the baseline file with the block in the spec, and no committed test runs the real baseline against the paths of `0004 AC-2`. The unit tests use baselines of their own. The probes above did both by hand.
- CI evidence does not exist yet. It waits for the pull request.

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

**Two readings.** First, the spec puts the monitor keys "after `expected_writer`" and the TLS keys "after `port`" inside the sentence that adds every new check after item 5. The build reads it as one new pass after item 5: listener, limits, probe, then the monitor keys of each cluster, the TLS keys of each node, then each principal. The test `ac5_new_checks_come_after_item_5_and_before_the_cross_checks` locks that order. Second, `probe.period_ms = 60000` passes its own range, but no expiry can cover it, because the timeout is at least 100 and the expiry at most 60000. Only the cross rule gives a finding, and the range test says so.

### Checker run

Build plan step 5 only, on 2026-11-15 at 17:14 WIB, by the checker session, on the merged tree `d8797a5` (macOS 27.0.1, arm64; a workstation, not Linux and not CI). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2` with `nightly-2026-10-17`. "The old binary" is `pool` built from `6ad02c0`, the tree before step 5, in a folder of its own. "The new binary" is `pool` built from `d8797a5`. Every command below was run in this session. The probes wrote their inputs outside the repository.

**Verdict: step 5 PASS.** Step 4 was not built and is outside this run.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-config --locked` | exit 0 | exit 0: 32 unit, 122 in `tests/validate.rs`, 1 and 5 doc tests passed, 0 failed | pass |
| `cargo test -p pool --locked` | exit 0 | exit 0: 3 unit and 84 in `tests/check_config.rs` passed, 0 failed | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards`, `docs` all `ok` | pass |
| `scripts/check.sh fuzz` | exit 0 | exit 0: target `validate`, 513768 runs in 61 seconds, no crash file | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0, the valid report | exit 0, `configuration valid` and the three lines of spec 0001 | pass |
| Probe, every old fixture: each of the 28 fixtures of `6ad02c0`, old binary on the old file and new binary on the file of the same name, human and JSON, with LF and with CRLF line endings; compare stdout, stderr, exit status and the two streams sent to one file | no difference | 112 pairs of runs, 0 differences. The only fixture whose text changed is `invalid-unknown-key.toml` (line 3, `[listener]` to `[no_table]`) | pass |
| Probe, `invalid-unknown-key.toml` | `config.schema` at line 3, column 2, as before | old binary on the old text, old binary on the new text and new binary on the new text all print `config.schema at line 3, column 2`, exit 1. The new binary on the old text prints three `config.missing_key` findings at `listener.*`, which is why the text had to change | pass |
| Probe, the twelve new fixtures through the new binary, both formats | the table under [Fixtures](index.md#fixtures) | the four valid ones (also `secret-serving-values.toml`) give the same bytes as `valid-minimal.toml`, exit 0, canary absent. The eight invalid ones exit 1 with one finding each: `listener.port`, `listener.tls_private_key_file` (`config.missing_key`), `limits.session_max_count`, `probe.observation_expiry_ms`, `config.duplicate_principal` at `principal.name` line 66 column 8, `config.schema` at line 14 column 1, `identity.backend_login` at line 22 column 17, `identity.database` at line 20 column 12 | pass |
| Probe, ranges and types: 135 files made from `valid-serving.toml`, one change each. Every `limits` key at its lowest and highest value, one above, one below where the range does not start at 0, and five values of the wrong type; the three `probe` keys at both edges and the cross rule at and one below `period_ms` plus `timeout_ms`; `listener.port`; `principal.session_max_count`; each missing listener and principal key; a wrong host and server name; repeated principal `id` and `name`; an unknown key in each new table; a file with none of the new tables | valid, `config.invalid_value` at the key, `config.missing_key`, `config.schema`, `config.duplicate_id` or `config.duplicate_principal`, as [Keys](index.md#keys) says | 135 of 135 as expected. `probe.period_ms = 60000` gives one finding, `config.invalid_value` at `probe.observation_expiry_ms`, also with the expiry at 60000 | pass |
| Probe, the order of the new checks: one file with fourteen mistakes | item 5 of spec 0003, then `listener`, `limits`, `probe`, the monitor keys, the node TLS keys, the principals, then the cross checks of spec 0003, then `config.duplicate_id` at `principal.id`, then `config.duplicate_principal` | `identity.routing_profile`, `listener.host`, `listener.port`, `limits.session_max_count`, `limits.frontend_message_max_bytes`, `probe.timeout_ms`, `cluster.monitor_login`, `cluster.monitor_credential_file`, `node.tls_ca_file`, `principal.verifier_file`, `principal.session_max_count`, `config.duplicate_endpoint` at `node.host`, `config.duplicate_id` at `principal.id`, `config.duplicate_principal` at `principal.name` | pass |
| Probe, defaults through the public boundary: a small program outside the repository calls `validate` on `config_version = 1` | the defaults of the table under [Keys](index.md#keys) | limits `1000 64 100 100 5s 5s 30s 8192 16777216`, probe `2s 1s 6s`, no listener, no principal; `principal.session_max_count` follows `limits.session_max_count` (1000, and 1 in `valid-serving-limits.toml`); both new generation types start at 1; `MAX_FINDINGS` is 20000 | pass |
| Probe, names and paths: each of the 14 keys with name or path syntax, with a value that holds `CANARY-9f2c7b1e` and one of the 79 listed characters, written as a TOML escape and also raw where TOML allows it; then 11 neighbours of the listed ranges (U+0020, U+007E, U+00A0, U+061B, U+061D, U+200D, U+2010, U+2027, U+202F, U+2065, U+206A) | exit 1 with exactly one finding, `config.invalid_value` at the key, canary in no output of either format; the neighbours valid | 1918 runs, 1918 as expected. No run gave a second finding | pass |
| Probe, made input 3 of `0003 AC-15`, built by the rule of spec 0003 (17268 entries, 1048559 bytes), old and new binary, both formats | the same bytes, `findings: 17268` | equal stdout, stderr and exit status in both formats; `findings: 17268` and 17268 finding lines | pass |
| Probe, the edge of the limit: files with 19999, 20000, 20001 and 20002 nodes whose port is 0, old and new binary | unchanged up to 20000; above it `findings: 20000`, the first 20000 findings of the old report, then the omitted count | 19999 and 20000: the same bytes as the old binary in both formats, no omitted line or field. 20001 and 20002: `findings: 20000`, the 20000 finding lines equal the first 20000 of the old binary, last line `findings omitted: 1` and `findings omitted: 2`; JSON has `,"findings_omitted":<n>,"findings":[` and `output_version` 1 | pass |
| Probe, made input 5, built by the rule under [Number of findings](index.md#number-of-findings) (349514 entries, 1048574 bytes) | 20000 finding lines, less than 4 MiB in each format, omitted `6 x 349514 - 20000 = 2077084` | human: exit 1, 1777935 bytes, `findings: 20000`, 20000 finding lines, last line `findings omitted: 2077084`. JSON: exit 1, 2358018 bytes, 20000 findings, `findings_omitted` 2077084 directly before `findings`. The first 20000 lines equal those of the old binary, which writes 2097084 findings in 190709698 bytes | pass |
| Probe, a failed write: the report of made input 5, and the report of `valid-minimal.toml`, sent into a pipe whose reader is gone | exit 3, as before | exit 3 for the old and the new binary in both formats | pass |
| Probe, the two views through the public boundary, on the fixtures and on seven files made from `valid-serving.toml` with one or more pieces removed | a view, or one `config.incomplete` per missing piece at the key of [What a command needs](index.md#what-a-command-needs) | `valid-serving.toml`, `valid-serving-limits.toml` and `secret-serving-values.toml`: both views. `valid-serving-probe-only.toml`: `probing()` gives a view, `serving()` gives `listener.host` and `identity.principal` at lines 35 and 43. `valid-minimal.toml`: `control.state_directory`, `cluster.id`, and for `serving()` also `listener.host` and `identity.id`. One missing `monitor_login`, `monitor_credential_file`, `tls_ca_file`, principal or the identities each give one finding at its key and entry. Every finding of every rejection has the code `config.incomplete` | pass |
| Probe, `pool check-config` on files a view refuses (`valid-serving-probe-only.toml`, `valid-minimal.toml`, `valid-topology.toml`) | exit 0, the valid report, no `config.incomplete` | exit 0 and the valid report for each | pass |
| Probe, `Debug` text of the configuration, the two views, the listener, every principal, the limits and the probe for `secret-serving-values.toml` | the canary in none | the canary is in the file and in no `Debug` text. The views print counts only (`ServingView { clusters: 1, nodes: 3, identities: 2, principals: 2 }`), the listener prints `[REDACTED]` for each field, a principal prints its `id` and its session limit | pass |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-5` | met on the workstation. Old fixtures unchanged byte for byte, new fixtures as listed, defaults as listed, and one new pass after item 5 and before the cross checks of spec 0003, which fits the sentence of the spec |
| `0004 AC-6` | met on the workstation, for all 14 keys with name or path syntax, old and new |
| `0004 AC-7` | met on the workstation, with made input 5 through the built binary and the edge at 20000 |
| `0004 AC-8` | met on the workstation. `pool check-config` never reports `config.incomplete` |
| `0001 AC-3` to `AC-11`, `0003 AC-1` to `AC-6`, `0003 AC-15` | unchanged: the outputs of the old fixtures and of made input 3 are the same bytes as from the binary of `6ad02c0`, and the tests of both specs pass |
| `0004 AC-28` | outside this run (step 4) |

Notes that are not failures:

- `probe.period_ms = 60000` is inside its own range, and no file with that value can be valid, because the expiry is at most 60000 and the timeout at least 100. The largest period a valid file can hold is 59900. The spec text may want to say so; the code follows the text as written.
- The old binary and the new one exit 3 with nothing on stderr when the reader of stdout is gone. That is the behavior from before this step.
- CI evidence does not exist yet. It waits for the pull request.
