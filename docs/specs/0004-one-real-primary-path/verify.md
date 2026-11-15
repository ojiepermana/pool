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
