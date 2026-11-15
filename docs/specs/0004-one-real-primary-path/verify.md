# 0004. Verification record

This file holds the acceptance evidence for [spec 0004](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

## Milestone 1: Guards, workspace shape and dependencies

Build plan steps 1 and 2, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commits `41d4709` (step 1) and `8d84f50` (step 2), on top of the accepted spec at `560096e` and the scope change `6ad02c0`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Docker was running (`docker info` exit 0). Build plan step 3 followed on the same day, after manual step M1 was answered (`ISC` and `BSD-3-Clause` allowed for `aws-lc-rs` 1.19.0, `aws-lc-sys` 0.47.0, `rustls-webpki` 0.103.17, `untrusted` 0.9.0 and `subtle` 2.6.1 only): commit `9df18f6`, on top of `main` at `87db7b0`, which holds step 5.

### UI / manual

None for steps 1 to 3. Manual step M1, the license decision, was answered by the user before step 3.

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
| `cargo build --workspace` at step 3 (before the commit) | `Cargo.lock` gains the packages | exit 0 in 21 s; `aws-lc-sys` and `libsqlite3-sys` compiled on macOS; Cargo prints the `unused dependency` warnings of the known exception of [Adopted dependencies](index.md#adopted-dependencies) | pass |
| `cargo metadata --no-deps --format-version 1` at `9df18f6` | five members and the six pairs of [Workspace](index.md#workspace); `pool-protocol` with no dependency | `pool` (`pool-config`, `pool-control`, `pool-core`, `serde`, `serde_json`, `tokio`, `tracing`), `pool-config` (`serde`, `toml`), `pool-control` (`pool-config`, `rusqlite`), `pool-core` (`aws-lc-rs`, `pool-config`, `pool-protocol`, `rustls`, `tokio`, `tokio-rustls`, `tracing`), `pool-protocol` (none) | pass |
| Names and versions of `Cargo.lock` compared with [locked packages](rationale.md#locked-packages) | the 70 entries of the rationale and the five members | 75 entries: the five members at 0.1.0 and 70 third party entries whose names and versions equal the rationale list line for line (`diff` empty) | pass |
| `scripts/check.sh all` at `9df18f6` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`. `deny` printed `advisories ok, bans ok, licenses ok, sources ok`, and warns `license-exception-not-encountered` for `untrusted` and `subtle` on a lockfile that does not hold them, as the spec expects | pass |
| `scripts/check.sh linux-build` at `9df18f6` | exit 0 | exit 1 after 121 s. In the image, `aws-lc-sys v0.47.0` and `libsqlite3-sys v0.40.0` compiled with the image's C compiler and no other tool, the release build finished in 1 min 13 s, and every `cargo test --locked --workspace` result was `ok`. The fixture loop then stopped at `crates/pool-config/tests/fixtures/secret-serving-values.toml human exit=0` with `unexpected exit 0, wanted 1`: the loop wants exit 0 only for names that start with `valid-`, and this fixture of step 5 is valid by the fixture table of the spec. See the note below | fail |
| `scripts/prove-checks.sh` on commit `9df18f6` | every case rejects its change | exit 1: `FAIL 9 guards accepted the change (exit 0)`; every other case `PASS`, case 20 skipped. Case 9 makes `crates/pool-core`, which is no longer a forbidden path. See the note below | fail |
| `scripts/check.sh all` at `41b25c6` (step 3 with proof case 9 on `apps/pool-cli` in `c29070a`, the fixture of step 5 renamed to `valid-serving-secret-values.toml` in `a696745`, and `main` at `44f637d` merged) | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok` | pass |
| `scripts/check.sh linux-build` at `41b25c6` | exit 0 | exit 0 in 120 s: host `aarch64-unknown-linux-gnu`, `aws-lc-sys` and `libsqlite3-sys` compiled with the image's C compiler and no other tool, every test `ok`, every fixture with its expected exit, `pool 0.1.0`, binary `sha256 59244a7f729c5ca954897632d002b11c7720795189130c8f7bf90fad38bee8e1` | pass |
| `scripts/check.sh fuzz` at `41b25c6` | exit 0 | exit 0: target `validate`, 708964 runs in 61 s, no crash | pass |
| `scripts/prove-checks.sh --with-linux-build` on commit `41b25c6` | every case rejects its change | `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 ran with Docker. Case 9 matched `W2: apps/pool-cli: forbidden path apps/pool-cli exists` | pass |

**Note on the six groups of `0004 AC-2`.** The proof cases read the sentence of the criterion as six groups of refusal: (30) a socket or the runtime outside `crates/pool-core` and `apps/pool`, (31) a thread or a Unix or UDP socket anywhere, also in `apps/pool`, (32) a clock, a file call or an environment read in `crates/pool-control/src`, (33) `rusqlite` there outside `journal.rs`, (34) a journal name outside `crates/pool-control` (`0003 AC-19`), and (35) `writer_admission` outside it (the guard half of `0003 AC-18`). `crates/pool-protocol` does not exist before step 3, so case 30 uses `crates/pool-control`. That a socket or the runtime is accepted in `crates/pool-core` and `apps/pool`, and that an `allow` line is compared by whole components, is shown by the unit tests `an_allow_line_lifts_the_text_only_at_or_under_its_path` and `source_rule_reads_the_three_forms`, since a proof case can only show a refusal.

**Note on the cargo warning.** Cargo still prints `warning: unused workspace dependency pool-control`. It goes away when a member takes the dependency, in a later step (`0004 AC-1`).

**Note on the two failures at `9df18f6`.** Both were in files that the first commit of step 3 did not own, and neither was in the native build. `scripts/linux-build.sh`, the copy of the Linux build block of spec 0001 (rule W7), expects exit 0 only for a fixture whose name starts with `valid-`, and the valid fixture `secret-serving-values.toml` of step 5 did not. Following the architect's decision, the fixture is now `valid-serving-secret-values.toml` with the same content (`a696745`). Spec 0004 "Fixtures" now says the start of a fixture name gives its result. Case 9 of `scripts/prove-checks.sh` made `crates/pool-core`, which step 3 turned into a member. It now makes `apps/pool-cli`, the one forbidden path left (`c29070a`). Both are resolved in the rows above.

**Note on the cargo warning since step 3.** Cargo prints `unused workspace dependency` and `unused dependency` warnings for the new dependencies and members that no code uses yet (twelve lines in the `check.sh all` log). No required check fails because of them. They end with Build plan step 10 (`0004 AC-1`).

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-4` | `local_packages` in `tools/repo-check/src/workspace.rs` ends an entry at the next line that starts with `[`; test `w4_a_table_after_the_last_entry_ends_it`; case 17 with its own feature 99 and case 29 in `scripts/prove-checks.sh`; `scripts/prove-checks.sh` passes on the committed trees `41d4709` and `8d84f50` with scope 004 `in-progress` | pass on the workstation; CI evidence waits for the pull request |
| `0004 AC-2` | `SourceRule`, `source_rule`, `covers` and `forbidden_in` in `tools/repo-check/src/workspace.rs`; the baseline `tools/repo-check/baseline/forbidden-source-patterns.txt` holds the file of [Source guard](index.md#source-guard); the tests of `source_guard_tests`, one passing and one failing case per kind of line; cases 30 to 35 | pass on the workstation; CI evidence waits for the pull request |
| `0003 AC-19`, guard half of `0003 AC-18` | cases 34 and 35, and the `allow crates/pool-control` lines of the baseline | pass for the guard; the library half of `0003 AC-18` is Build plan step 6 |
| `0004 AC-1` (shape) | `cargo metadata` and `Cargo.lock` rows above; baselines `lock-packages.txt` (75 names), `forbidden-paths.txt` (only `apps/pool-cli`), `internal-dependencies.txt` (the six pairs); `scripts/check.sh guards` passes. `cargo build --workspace --locked` without the dependency warning waits for Build plan step 10 | pass for the shape on the workstation; the warning half and CI evidence are not run |
| `0004 AC-3` | `deny.toml` with the five exceptions by crate and the unchanged general list; `scripts/check.sh deny` passes on the product lockfile and both fuzz lockfiles; `NOTICE` names SQLite 3.54.1 and AWS-LC 0.47.0 with their terms, copied from the locked `libsqlite3-sys` 0.40.0 and `aws-lc-sys` 0.47.0; `scripts/check.sh linux-build` at `41b25c6` passes with the native code compiled by the C compiler of the pinned image and no other tool | pass on the workstation; CI evidence waits for the pull request |
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

### Checker run of the whole milestone

Build plan steps 1 to 3, on 2026-11-15 at 17:35 WIB, by the checker session, on the merged tree `486b90e` (macOS 27.0.1, arm64, with Docker 29.9.2 running; the Linux build ran in the pinned image, the rest on the workstation, and none of it is CI). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. Every command below was run in this session. The probes used a private clone of `486b90e` and changed nothing in the repository. This run repeats the checks of steps 1 and 2 on the new tree, because the baselines and the proof script changed again with step 3.

**Verdict: milestone 1 PASS** on the workstation and in the pinned Linux image. Two halves of its criteria belong to later steps and were not checked: the dependency warning of `0004 AC-1` (step 10) and the second fuzz lockfile of `0004 AC-3` (step 12).

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo metadata --no-deps --format-version 1 --locked` | five members, the six pairs of [Workspace](index.md#workspace), `pool-protocol` with no dependency, no `dev-dependencies`, no build script | members `pool`, `pool-config`, `pool-control`, `pool-core`, `pool-protocol`; pairs `pool -> pool-config`, `pool -> pool-control`, `pool -> pool-core`, `pool-control -> pool-config`, `pool-core -> pool-config`, `pool-core -> pool-protocol`; `pool-protocol` has no dependency; every dependency of every member is a normal one; no member has a build script; each member has the five inherited fields. The third party dependencies of each member equal the table | pass |
| Probe, `Cargo.lock` against [locked packages](rationale.md#locked-packages): name and version of every entry | the 70 entries of the rationale and the five members | 75 entries. The 70 with a `source` line equal the rationale list by name and version. The five without one are the members at 0.1.0. No entry of `d8797a5` was removed or changed | pass |
| Probe, the baselines against the spec text | W1 the lockfile names, W2 only `apps/pool-cli`, W5 the six lines | `lock-packages.txt` holds the 73 distinct names of the lockfile; `forbidden-paths.txt` holds `apps/pool-cli` alone; `internal-dependencies.txt` holds the six lines of the spec | pass |
| Probe, the adopted dependencies in the root `Cargo.toml` against [Adopted dependencies](index.md#adopted-dependencies) | the six versions and feature lists | `tokio` 1.54.1, `rustls` 0.23.46, `tokio-rustls` 0.26.6, `aws-lc-rs` 1.19.0, `rusqlite` 0.42.0 and `tracing` 0.1.44, each without default features and with the listed features | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok`. `deny` printed `advisories ok, bans ok, licenses ok, sources ok` for the product lockfile and for `crates/pool-config/fuzz/Cargo.lock` | pass |
| Probe, which packages need the new licenses: `cargo deny --locked --config deny.toml list --layout license` | only the five packages of manual step M1 | `ISC`: `aws-lc-rs` 1.19.0, `aws-lc-sys` 0.47.0, `rustls-webpki` 0.103.17, `untrusted` 0.9.0, and `rustls` 0.23.46, which also offers `Apache-2.0` and `MIT` and needs no exception. `BSD-3-Clause`: `aws-lc-sys` 0.47.0 and `subtle` 2.6.1. On the product lockfile the only exception that is not met is the old one for `libfuzzer-sys`, so all five new ones are used. On the fuzz lockfile the five are not met, which is a warning, as the spec says | pass |
| Probe, each of the five exceptions removed in turn in the clone, then `cargo deny --locked --config deny.toml check licenses` | the check fails each time | five of five: exit 4, `error[rejected]: failed to satisfy license requirements`, `licenses FAILED`. With the file as committed: `licenses ok`. The general list is `Apache-2.0`, `MIT`, `Unicode-3.0`, as before, and the five blocks equal the block under [Dependency policy](index.md#dependency-policy) | pass |
| Probe, `NOTICE` against the locked sources | SQLite and AWS-LC with version and terms, copied from the locked packages | `NOTICE` names SQLite 3.54.1 and AWS-LC 0.47.0. `sqlite3.h` of the locked `libsqlite3-sys` 0.40.0 defines `SQLITE_VERSION "3.54.1"` and opens with the dedication that `NOTICE` quotes. The ISC, MIT, BSD 3-Clause and MIT-0 texts of `NOTICE` equal lines 247 to 316 of the `LICENSE` file of the locked `aws-lc-sys` 0.47.0 (`diff` empty), and the list of parts and their terms follows that file | pass |
| `scripts/check.sh linux-build` on the committed tree | exit 0 | exit 0: `host: aarch64-unknown-linux-gnu`, `Compiling aws-lc-sys v0.47.0` and `Compiling libsqlite3-sys v0.40.0` in the release build (51.62 s) and in the test build, 15 test results `ok` and none failed, 80 fixture runs each with the exit status its name asks for, `pool 0.1.0`, binary `sha256 a31960798a172e78c26f315a61998bef8b43e0d6e27f2b28af29221bc753123c`, `linux build check passed` | pass |
| Probe, the tools of the pinned image: `docker run` of `rust:1.100.0-slim-bookworm@sha256:7c7d4eb4...` with `command -v` for fifteen tools | a C compiler and no other build tool | `cc` and `gcc` (Debian 12.2.0) and `perl` exist. `g++`, `cmake`, `make`, `ninja`, `nasm`, `yasm`, `clang`, `go`, `python3`, `bindgen` and `pkg-config` do not. So the native code was compiled by the C compiler of the image alone | pass |
| `scripts/prove-checks.sh --with-linux-build` on the committed tree `486b90e` | every case rejects its change | exit 0, `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 ran with Docker. Case 9 matched `W2: apps/pool-cli: forbidden path apps/pool-cli exists` | pass |
| Probe, case 9 without its baseline line: make `apps/pool-cli/.keep` in the clone, then delete `apps/pool-cli` from `forbidden-paths.txt` | rejected with the line, accepted without it | exit 1 with the `W2` line; `pool-repo-check workspace: ok` without the line | pass |
| Probe, the 125 edits of the first checker run, repeated on `486b90e` | as `0004 AC-2` says | 125 of 125 as expected. The 35 edits that must be accepted now give `pool-repo-check workspace: ok` and exit 0, also in `crates/pool-core`, which exists now | pass |
| Probe, pairs that rule W5 must refuse: `pool-core -> pool-control`, `pool-control -> pool-core`, `pool-config -> pool-protocol`, `pool-protocol -> pool-config`, each added to a manifest in the clone | `W5` each time | four of four, for example `W5: crates/pool-core/Cargo.toml:10: dependency pool-core -> pool-control is not allowed by internal-dependencies.txt` | pass |
| Probe, rules W4 and W1 on the new members: `[lints]` removed from `crates/pool-core/Cargo.toml`; `aws-lc-sys` removed from `lock-packages.txt` | `W4`, `W1` | `W4: crates/pool-core/Cargo.toml: the manifest has no [lints] table with workspace = true`; `W1: Cargo.lock: package aws-lc-sys is in the lockfile but not in the baseline lock-packages.txt` | pass |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-1`, the shape | met. Five members, six pairs, `pool-protocol` without a dependency, the lockfile of the rationale, no `apps/pool-cli`, no `dev-dependencies`, no build script. The last sentence (no `unused workspace dependency` line) waits for Build plan step 10 and was not checked |
| `0004 AC-2` | met again on `486b90e` |
| `0004 AC-3` | met for what exists. `deny` passes on the product lockfile and on the one fuzz lockfile there is; the second one comes with Build plan step 12. Five exceptions by crate, the general list unchanged, and no other package needs them. `NOTICE` names both libraries with version and terms. The Linux build passes with the C compiler of the pinned image and no other tool |
| `0004 AC-4` | met again on `486b90e`, with case 9 on `apps/pool-cli` |
| `0003 AC-19`, guard half of `0003 AC-18` | met for the guard, as in the first run |

Notes that are not failures:

- Cargo prints seven `unused dependency` warnings: `pool-control` and `pool-core` in `apps/pool`, `rusqlite` in `pool-control`, and `pool-config`, `pool-protocol`, `tokio-rustls` and `tracing` in `pool-core`. Each is a dependency or a member that step 3 added, so each is covered by the exception of [Adopted dependencies](index.md#adopted-dependencies), which ends with step 10. No other warning was printed, and the older `unused workspace dependency pool-control` line is gone.
- No first party code names `tokio`, `rustls`, `aws-lc-rs` or `tracing` yet, and Cargo prints no warning for `tokio`, `rustls` and `aws-lc-rs` in `pool-core`, or for `tokio` and `tracing` in `apps/pool`. So a build without the warning is not proof that every dependency is used. The check of `0004 AC-1` at step 10 may want a second look at that.
- The five exceptions name a crate and no version, as the block in the spec does. Manual step M1 named versions. A later version of one of the five would pass under the same exception, and rule W1 compares names only. Whether to pin the version in `deny.toml` is a question for the architect and the Release maintainer.
- `NOTICE` has no copyright line for the project. `LICENSE` holds the Apache text with no copyright notice, and the spec asks for the name of the project and its license, which `NOTICE` gives. Adding a copyright holder is a decision of a person.
- CI evidence does not exist yet. It waits for the pull request.

### Checker run of the exact version exceptions

The change to `deny.toml` that followed the checker run above (decision 50), on 2026-11-15 at 18:07 WIB, by the checker session, on the merged tree `59b7e2c`. Tool: `cargo-deny 0.20.2`. Every command below was run in this session. The probes changed a private clone and nothing in the repository.

**Verdict: PASS.** `0004 AC-3` still holds, and the third note of the checker run above (exceptions without a version) is settled.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| Probe, the five exception blocks of `deny.toml` against the block under [Dependency policy](index.md#dependency-policy) | no difference | equal: `aws-lc-rs@=1.19.0`, `aws-lc-sys@=0.47.0`, `rustls-webpki@=0.103.17`, `untrusted@=0.9.0`, `subtle@=2.6.1`, each with the licenses of manual step M1. The general list is still `Apache-2.0`, `MIT`, `Unicode-3.0` | pass |
| `scripts/check.sh all` (its `deny` part) | exit 0 on the product lockfile and the fuzz lockfile | exit 0, `advisories ok, bans ok, licenses ok, sources ok` for both. On the product lockfile the only exception that is not met is the old one for `libfuzzer-sys`, so all five pinned ones still match their package | pass |
| Probe, one version changed at a time in the clone (`1.19.1`, `0.47.1`, `0.103.18`, `0.9.1`, `2.6.0`), then `cargo deny --locked --config deny.toml check licenses` | the check fails each time | five of five: exit 4, `error[rejected]: failed to satisfy license requirements`, `licenses FAILED`. With the file as committed: `licenses ok` | pass |

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

### Step 4: the wire format

Build plan step 4, on 2026-11-15, on macOS (workstation, not Linux and not CI). Commit `48775c2`, on top of `main` at `486b90e`. Tools: `rustc 1.100.0`. No manual step, no container, no network: every test is hermetic. Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-protocol --locked` | exit 0 | exit 0: 21 unit tests beside the code and 9 in `tests/golden.rs` passed, 0 failed | pass |
| `cargo clippy -p pool-protocol --all-targets --locked -- -D warnings` | exit 0 | exit 0 | pass |
| `scripts/check.sh all` at `48775c2` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`; rule W3 finds no socket or runtime text in `crates/pool-protocol` | pass |

What the tests hold:

- `first_packets_split_at_every_byte_give_the_same_result` and `messages_split_at_every_byte_give_the_same_result`: 15 golden first packets (`SSLRequest`, `GSSENCRequest`, `CancelRequest`, the startup packet of `psql`, a startup packet for 3.2 with a `_pq_.` option, and the refused ones) and 29 golden messages (`Query`, `Terminate`, every authentication and SASL message, `NegotiateProtocolVersion`, `ReadyForQuery`, `ParameterStatus`, `BackendKeyData`, an error from PostgreSQL and the twelve errors of POOL), each fed whole, in two pieces split at every position, and one byte at a time, with the same result each time. `a_stream_of_all_golden_messages_splits_anywhere` does the same for all messages in one stream, and `headers_split_at_every_byte_classify_the_same` for the five byte header reader.
- `a_length_field_is_checked_before_anything_is_reserved`: a first packet and messages that claim 4 GiB are refused after their header bytes, the body of a `CancelRequest` is counted and not stored, the authentication limit is 1024 bytes of body, and counts inside a body are compared and never reserved from. In the code, `FirstPacketReader` and `MessageReader` call `Vec::with_capacity` only after `decode_first_header` or the limit check passed.
- `first_packets_give_their_rows`, `header_rows`, `handshake_order` and `parameter_rows`: every row of the tables under [Startup](index.md#startup) that needs no socket: `28000` for a startup packet before TLS, `N` then a new packet after `GSSENCRequest`, `0A000` for major 2 and 4, `replication`, `options` and every unlisted name, `08P01` for a short length, a wrong order, a duplicate or empty name, a missing `user` and bytes after the end, `54000` above `limits.startup_packet_max_bytes`, and `NegotiateProtocolVersion` for 3.0 only for a minor version above 0 or a `_pq_.` name.
- `frontend_rows` and `backend_rows`: every row of the tables under [Messages after login](index.md#messages-after-login).
- `every_pool_error_has_its_exact_bytes`: the bytes of the twelve errors, fields `S`, `V`, `C`, `M` only, against the table under [SQLSTATEs that POOL generates](index.md#sqlstates-that-pool-generates).
- `rfc_7677_exchange_parses` and the refusal tests of `scram.rs`, the RFC 4648 vectors and the strict form tests of `base64.rs`, and the length checks of `SASLInitialResponse` in `auth.rs`.

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-28` | the split tests and the length test above | pass on the workstation; CI evidence waits for the pull request |
| `0004 AC-15`, byte level half | the startup rows above; the TLS handshake, the node side and `psql` are steps 7, 9 and 10 | pass for the byte level half; the rest not run |
| `0004 AC-16`, byte level half | `AuthenticationSASL` offers `SCRAM-SHA-256` only in the golden message; the client-first parser refuses `p=`, an authorization identity and extensions; the 1024 byte limit; the SCRAM crypto and the full exchange are step 7 | pass for the byte level half; the rest not run |
| `0004 AC-23`, byte level half | `classify_frontend` and `classify_backend` with every tag of the tables; `BackendKeyData` prints `[REDACTED]`; the session that uses them is step 9 | pass for the byte level half; the rest not run |

**Changes of 2026-11-15 (decisions 52 to 56 of the spec).** Commit `87a068a`. `classify_frontend` refuses `X` with a length field other than 4 with `08P01`, and `FrontendAction::Terminate` has no body (unit test `frontend_rows`, golden test `terminate_has_no_body`). `parse_client_first` reads the user name up to the next comma and ignores it, whatever it holds; a message that is not UTF-8 is still `Malformed` (`client_first_refusals`). `FirstPacketReader` keeps the `Handshake` of its connection and decides on the eight header bytes: a startup packet without TLS gives `28000` after 8 bytes with its parameters unread (`first_packets_give_their_rows`, `one_reader_follows_the_connection_and_never_reads_a_body_before_tls`); the golden split tests feed the packets that come over TLS after an `SSLRequest` on the same reader, and still give the same result for every split. `cargo test -p pool-protocol --locked`: 22 unit and 10 golden tests passed. `scripts/check.sh all` at `87a068a`: exit 0.

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

### Checker run after decisions 47 and 49

The two changes to step 5 that followed the first checker run, on 2026-11-15 at 17:35 WIB, by the checker session, on the merged tree `486b90e`: the fixture `secret-serving-values.toml` is now `valid-serving-secret-values.toml` (decision 47), and `probe.period_ms` stops at 30000 (decision 49). Where the first checker run above names `secret-serving-values.toml` or a period of 60000, it describes the tree `d8797a5`. Every command below was run in this session, with the `pool` binary built from `486b90e`.

**Verdict: both changes PASS**, and `0004 AC-5` still holds.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 0. `cargo test`: 32 unit and 123 in `tests/validate.rs` for `pool-config`, 3 unit and 84 in `tests/check_config.rs` for `pool`, 0 failed | pass |
| Probe, the renamed fixture: the Git blob of `secret-serving-values.toml` at `d8797a5` and of `valid-serving-secret-values.toml` now | the same content | both are blob `9a58de9d8b5cf4ee8fc56d9ee09e62b3bdde87bc`; the old name no longer exists in the fixture folder | pass |
| Probe, the start of every fixture name against its result: all 40 fixtures through the binary, both formats | `valid-*` exit 0, `invalid-*` and `secret-*` exit 1, the canary in no output | 80 runs, 80 as expected: 10 `valid-`, 26 `invalid-`, 4 `secret-`, and no other start. The four `secret-*` fixtures exit 1 (`0001 AC-9`). The canary is in eight fixtures and in no output | pass |
| The same loop inside `scripts/check.sh linux-build` | every fixture with the exit status its name asks for | `linux build check passed`; `valid-serving-secret-values.toml` exits 0 in both formats | pass |
| Probe, the twelve fixtures of spec 0004 through the binary, both formats | the table under [Fixtures](index.md#fixtures) | as in the first run; `valid-serving-secret-values.toml` gives the same bytes as the valid report, with the canary in the file and in no output | pass |
| Probe, `probe.period_ms` through the binary, on files made from `valid-serving.toml` | 30000 valid, above it `config.invalid_value` at `probe.period_ms` | 30000 with the timeout at 30000 and the expiry at 60000: valid. 30000 with the default timeout and the expiry at 31000: valid. 30001, 59900, 60000 and 60001: one finding each, `config.invalid_value` at `probe.period_ms`. 99: the same finding. 100: valid | pass |
| Probe, the other ranges, types, missing keys and the order of the new checks, repeated | as [Keys](index.md#keys) says | 138 of 138 as expected, and the order of the fourteen findings is the one of the first run | pass |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-5` | still met. The renamed fixture has the same content and the same result, and the new upper bound of `probe.period_ms` is enforced at its own key |
| `0001 AC-9` | holds for every `secret-*` fixture: each exits 1 and none leaks the canary |

Note that is not a failure: with decision 49 the first note of the first checker run above (a period of 60000 that no expiry can cover) is settled. Every value in the range of the period can now be used with some timeout and expiry.

### Checker run of milestone 2

Build plan step 4, and a second look at step 5, on 2026-11-15 at 17:48 WIB, by the checker session, on the merged tree `8332451` (macOS 27.0.1, arm64, with Docker 29.9.2 running; the Linux build ran in the pinned image, the rest on the workstation, and none of it is CI). Tools: `rustc 1.100.0`. Every command below was run in this session. The probes are a small program outside the repository that calls the public functions of `pool-protocol`. Its packets and messages are written out as bytes by hand and its expected results are written from the tables of the spec, so it does not lean on the encoders or the tests of the crate.

**Verdict: milestone 2 PASS** on the workstation and in the pinned Linux image. Steps 4 and 5 are both verified. The session halves of `0004 AC-15`, `AC-16` and `AC-23` belong to Build plan steps 7, 9, 10 and 11 and were not checked.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-protocol --locked` | exit 0 | exit 0: 21 unit tests and 9 in `tests/golden.rs` passed, 0 failed | pass |
| `cargo clippy -p pool-protocol --all-targets --locked -- -D warnings` | exit 0 | exit 0 | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok` | pass |
| `scripts/check.sh linux-build` on the committed tree | exit 0 | exit 0: `host: aarch64-unknown-linux-gnu`, 16 test results `ok` and none failed, among them the 21 unit tests and the 9 golden tests of `pool-protocol`, `linux build check passed` | pass |
| Probe, first packets split at every position: 29 packets written as bytes (`SSLRequest`, `GSSENCRequest`, a `CancelRequest` of 8, 16 and 44 bytes, the startup packet of `psql`, one for 3.2, one with a `_pq_.` name, one exactly at the limit, and eighteen that must be refused), each fed whole, in two pieces at every position, in three pieces at every pair of positions, one byte at a time, and with other bytes after it | the result of the table under [Startup](index.md#startup) each time, and no byte taken after the packet | 19599 splits, 19715 checks, 0 mismatches | pass |
| Probe, messages split at every position: 26 messages written as bytes, from both sides (`Query`, `Terminate`, the two SASL answers, seven `Authentication` messages, `ParameterStatus`, `BackendKeyData`, `ReadyForQuery` with each status, row description, data row, command complete, empty query, an error and a notice of PostgreSQL, a notification, `NegotiateProtocolVersion`, a COPY response), each fed whole, in two pieces, in three pieces and one byte at a time, then all as one stream split at every position, and the five header bytes split at every position | the same tag and body each time | 20877 splits, 21216 checks, 0 mismatches | pass |
| Probe, random and changed inputs: 400000 inputs, a quarter random bytes and the rest golden bytes with up to three changes, through the thirteen parsers and through both readers whole and split at a random position | no panic, and the same result whole and split | no panic; 800000 comparisons, 0 mismatches | pass |
| Probe, reservations, with an allocator that counts every reservation during the call | nothing reserved before the length passed its limit | a first packet that claims 4294967295, 2147483648 or one byte above its limit: 0 bytes reserved. A `CancelRequest` of 65536 bytes: 0 bytes for its header and 0 for its 65528 body bytes. A startup packet at its limit: one reservation of exactly its body (504 and 65528 bytes). A message that claims 4294967295 bytes, one byte above the limit, or a length of 3: 0 bytes. A message at the limit of 1024: 1024 bytes. A `NegotiateProtocolVersion` that claims 4294967295 names: 0 bytes. A `SASLInitialResponse` that claims 2147483647 bytes of data: 13 bytes, the mechanism name. Base64 of 1024 characters: 768 bytes | pass |
| Probe, reading the code for the same rule | `Vec::with_capacity` only after the check | `FirstPacketReader::feed` reserves after `decode_first_header` accepted the length, and only for a `StartupMessage`. `MessageReader::feed` reserves after `body_len() > limit` was tested. No parser reserves from a count or a length inside a body | pass |
| Probe, the order of first packets: every order of one to five packets of the four kinds (1364 orders), against a model written from the sentences under [Startup](index.md#startup) | the same step or SQLSTATE at every position | 12744 checks, 0 mismatches. A `StartupMessage` before TLS is `28000`, also after `GSSENCRequest`. After an error or a last packet every further packet is `08P01` | pass |
| Probe, the first eight bytes: 95 cases | the rows of the table | a length of 0 to 7 is `08P01` for every code; a length above the limit is `54000` for every code; `SSLRequest` and `GSSENCRequest` of another length than 8 are `08P01`; a `CancelRequest` of 8 bytes up to the limit is accepted; major 3 with any minor version is a `StartupMessage`; major 0, 1, 2, 4 and others are `0A000`; an unknown code under major 1234 is `08P01` | pass |
| Probe, the two tables under [Messages after login](index.md#messages-after-login): every tag byte from 0 to 255 with eleven lengths, from the client and from the backend | the action of the table | 5632 checks, 0 mismatches | pass |
| Probe, the table under [SQLSTATEs that POOL generates](index.md#sqlstates-that-pool-generates), read from `index.md` by the program | twelve rows, the same SQLSTATE and message in the same order, and the bytes `S`, `V`, `C`, `M` and nothing else | 12 rows; every SQLSTATE, every message and every encoded message equal the spec, byte for byte, and each reads back through the message reader | pass |
| Probe, SCRAM text, message bytes and `Debug`: 63 cases | as [Frontend authentication](index.md#frontend-authentication) and [Backend connection](index.md#backend-connection) say | the exchange of RFC 7677 parses. A client-first with `p=`, an authorization identity, `m=` or an extension is refused; `n,,` and `y,,` pass. A client-final must repeat the header (`biws` for `n,,`, `eSws` for `y,,`, and not the other way round), the whole nonce and a proof of 32 bytes. POOL as a client writes `n,,n=,r=<nonce>` and `c=biws,r=<nonce>`. A server-first needs the client nonce at its start and a count of 1 to 600000. `AuthenticationSASL` holds `SCRAM-SHA-256` alone. `BackendKeyData` takes a key of 4 to 256 bytes and prints `BackendKeyData([REDACTED])`. `Debug` of a startup request prints counts only | pass |
| Probe, the crate itself | no dependency, no socket, runtime or thread | `cargo metadata` lists no dependency for `pool-protocol`; its manifest has no `dev-dependencies`; `scripts/check.sh guards` passes; a search of the crate for the texts of rule W3, for `std::fs`, `std::env`, a clock and `unsafe` finds nothing. Outside its tests the code has no `unwrap`, and four `expect` calls, each in an encoder with a `# Panics` section | pass |
| Probe, step 5 on this tree | `0004 AC-5` to `AC-8` as before | no file of `crates/pool-config` or `apps/pool` changed since `486b90e`. `cargo test`: 32 unit and 123 in `tests/validate.rs`, 3 unit and 84 in `tests/check_config.rs`, 0 failed. Through the binary: 80 fixture runs with the exit status of their name, 138 range and order cases, 1918 name and path cases, made input 3 with 17268 findings, the edge at 20000, and made input 5 with 20000 finding lines, `findings omitted: 2077084`, 1777935 and 2358018 bytes. All as in the earlier runs | pass |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-28` | met. Every golden packet and message gives the same result whole and split at every position, also with packets and messages that the crate did not build itself, and no reader reserves from a length field before the limit check |
| `0004 AC-15`, byte level half | met: `28000` before TLS, `0A000` for major 2 and 4, for `replication` and for a name outside the list, `NegotiateProtocolVersion` for 3.0, a `CancelRequest` that is read and dropped. `N` for `GSSENCRequest`, TLS and the node side are written by later steps |
| `0004 AC-16`, byte level half | met: `SCRAM-SHA-256` alone, the refusals of the client-first message, and the limit of 1024 bytes. The exchange itself is Build plan step 7 |
| `0004 AC-23`, byte level half | met: `Query` and `Terminate` are accepted, the eleven listed tags are `0A000`, every other tag and a length below 4 are `08P01`, a COPY response is told apart, and the real `BackendKeyData` prints nothing. The session is Build plan step 9 |
| `0004 AC-5` to `AC-8` | still met on this tree |

How the readings of the build were judged:

- **Order of the startup checks** (structure, then `user`, then the names). The spec gives no order. Putting `08P01` before `0A000` is the safer reading, and it is accepted.
- **`CancelRequest`** of any length from 8 to the limit, with the body dropped. That is what the row of the table says.
- **Codes.** An unknown code under major 1234 is `08P01` and every other major but 3 is `0A000`. That fits the last two rows of the table.
- **The 1024 bytes** of an authentication message are counted as the body, without the tag and the length field. The spec says "at most 1024 bytes" and does not say which bytes. The fixture test of Build plan step 11 should use the same edge.

Notes that are not failures:

- **`Terminate` is told from its tag alone.** A `Terminate` whose length field is 4294967295 is classified as `Terminate` with a body of 4294967291 bytes, whatever `limits.frontend_message_max_bytes` is. The table under "Messages after login" checks the size only for `Query`, and `0004 AC-20` names only `Query`. The table under [Limits](index.md#limits) counts "the length field of a message from the client". The two texts differ, and the session of step 9 will have to choose what it does with such a body. A question for the architect before step 9.
- **The name in the client-first message.** The parser refuses a name that holds `=` in another form than `=2C` or `=3D`. The spec says the name is ignored, as PostgreSQL ignores it, and does not list this among the refusals. A client that sends such a name would pass PostgreSQL and be refused here with `08P01`. `psql` sends an empty name and is not touched.
- **The duplicate check of the parameter parser** compares every name with every earlier one. At the default limit of 8192 bytes a packet with 1633 different names takes about 5 ms and reserves 128 KiB. At the largest limit a file may set, 65536 bytes, a packet with 13104 names takes about 330 ms and reserves 1 MiB, in the optimized build of the probe. It is bounded and happens after TLS and before authentication. Worth a look in the review.
- **`FirstPacketReader`** reads and parses the body of a `StartupMessage` without asking `Handshake`. On the path before TLS the caller has to use `decode_first_header` and `Handshake`, as the module text says, or a body would be read where the spec sends `28000` at once. That is for the check of Build plan steps 7 and 9.
- The encoders that call `encode_message` (`encode_query`, the authentication encoders, `PoolError::encode` and others) can panic on a body above 4 GiB like it does, and do not repeat its `# Panics` section. No caller builds such a body.
- CI evidence does not exist yet. It waits for the pull request.

### Checker run of the changes to step 4 (decisions 52, 53, 54 and 71)

On 2026-11-15, by the checker session, on the tree `3b58997` (the files of `crates/pool-protocol/` are the same on `ecd0ee5`, the tip after the last merge). macOS 27.0.1, arm64, `rustc 1.100.0`, with Docker 29.9.2 running. The commits looked at are `87a068a` (decisions 52, 53 and 54), `cc77c34` (the session reads its first packets with one `FirstPacketReader`), `8f64f5a` (`Debug` written by hand) and `795775f` (decision 71). Every command below was run in this session, after the pause of the usage limit. The probe is the checker's own program outside the repository. It calls the public functions of `pool-protocol` with packets written out as bytes by the probe itself, and it repeats the probes of the first run of milestone 2 that showed the problems. The bytes of the real server were recorded in this session from the PostgreSQL 18.7 of the fixture of `scripts/check.sh integration`, with a raw socket of the probe over TLS, and from no other database.

**Verdict: the changes to step 4 PASS.** The byte level halves of `0004 AC-15`, `AC-16` and `AC-23` hold after decisions 52, 53, 54 and 71, and no type of `pool-protocol` prints what a log may not show (`0004 AC-24`).

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` on `ecd0ee5` | exit 0 | exit 0, all six `ok`, 24 test results `ok`, none failed | pass |
| `scripts/check.sh linux-build` on `3b58997` | exit 0 | exit 0, 22 test results `ok`, `linux build check passed` | pass |
| `Terminate` with a length field of 4, then of 5, 6, 100, 1024, 1025, 65536 and 4294967295 (decision 52) | 4 ends the session, every other length is `08P01` | `FrontendAction::Terminate` for 4, `08P01` for the seven others, also for the ones above the size limit, so no `54000` | pass |
| Every tag byte (256) with seven lengths against the table of the spec, with the new row for `Terminate` | the action of the table | all 1792 cases as the table says | pass |
| Client first messages with a user name that PostgreSQL takes and the old code refused (decision 53): `n=a=b`, `n==`, `n=user=name=`, `n=a=2Cb`, an empty name, and the header `y,,` | accepted | accepted, all six | pass |
| Client first messages that must stay refused: channel binding `p=`, an authorization identity, an extension after the nonce, `m=`, a comma inside the name, an empty nonce | refused | refused, all six | pass |
| One reader for one connection (decision 54): a `StartupMessage` before TLS, 73 bytes long | `28000` after the 8 header bytes, and the body is never taken | `28000`, and the reader took 8 of 73 bytes | pass |
| Ten orders of first packets (`SSLRequest`, `GSSENCRequest`, `CancelRequest`, `StartupMessage`, and the orders that are not allowed) through one reader, each whole, split at every position (454 splits) and byte by byte | the same result in every split | the same result in every split: the allowed orders pass, the others end with `28000` or `08P01` | pass |
| `NegotiateProtocolVersion` against a real server (decision 71). PostgreSQL 18.7, asked for 3.0 with the option `_pq_.zq9`, sent the body `00030000000000015f70715f2e7a713900` (first number 196608, one name) | POOL sends the same bytes for the same request | `encode_negotiate_protocol_version` gives the same 17 bytes | pass |
| The same for a request for 3.9. PostgreSQL 18.7 sent `0003000200000000` (196610, its own 3.2, no name), and for 3.9 with two options `00030002000000025f70715f2e7a7139005f70715f2e7a713800` | POOL sends the same shape with the code of 3.0 | `0003000000000000` (196608, no name) | pass |
| The parser on the recorded messages of the real server, and on a first number that is a bare minor 0, and on a code with major 4 | it reads both recorded messages and refuses the two others | minor 2 with no name and minor 0 with one name are read; the bare 0 and the major 4 are refused | pass |
| `Debug`, in both forms, of each type of `pool-protocol` that holds content, filled with a canary text: `StartupRequest`, `Forwarded`, `FirstPacket`, `FirstPacketReader` and `MessageReader` with half a packet inside, `Message`, `Authentication` (three kinds), `SaslInitialResponse`, `NegotiateProtocolVersion`, `ParameterStatus`, `BackendKeyData`, `ClientFirst`, `ServerFirst`, `ClientFinal` | no canary in any output | no canary; the outputs show counts and lengths, for example `Message { tag: 'Q', body_len: 20 }` and `BackendKeyData([REDACTED])` | pass |
| `scripts/check.sh integration` on `ecd0ee5`, which holds the test of step 11 where `psql` 18.7 asks for protocol 3.2 | exit 0 | exit 0, 21 of 21 | pass |

| Criterion | What was checked | Verdict |
|---|---|---|
| `0004 AC-15`, byte level half | The first packets through one reader, in every split; the answer to a newer minor version compared with the bytes of a real server | met |
| `0004 AC-16`, byte level half | The SCRAM client first message with any user name, and the forms that stay refused | met |
| `0004 AC-23`, byte level half | The table of frontend tags with the row for `Terminate`, every tag with seven lengths | met |
| `0004 AC-24`, in `pool-protocol` | `Debug` of fourteen types with a canary | met |

**Notes that are not failures.**

- The first run of milestone 2 took the bare minor number 0 from the text of the spec and passed it. A real server sends the whole protocol code, and `psql` 18.7 refused the old bytes. This run compared with bytes that a real PostgreSQL 18.7 sent, as the rule "A golden message is not proof of a wire format" asks.
- POOL answers 3.0 where PostgreSQL 18.7 answers 3.2, so the first number differs from the real server by design. The shape, the count and the names are the same.
- `Debug` of `SaslInitialResponse` prints the mechanism as `[REDACTED]`. The mechanism is not a secret, so this hides a little more than it has to. It is not a fault.

**For the tester.** No mutation run was done for these changes. The record of the developer names a unit or golden test for each change, and the test of step 11 with `psql` covers decision 71 against a real client. I did not try whether those tests catch a wrong change. One gap: no test in the repository compares the bytes of `encode_negotiate_protocol_version` with bytes recorded from a real server. The recorded bodies above can serve as a golden value.

## Milestone 3: Authority on disk and the first contact with PostgreSQL

### Build plan step 6: authority on disk and the control loop

On 2026-11-15, on macOS (workstation, not Linux and not CI). Commit `fc5a5fe` (step 6), merged with `main` at `8332451` in `dff749e`. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, the SQLite of `rusqlite 0.42.0` with the feature `bundled`. The step is hermetic: no container and no PostgreSQL. Every journal is a real file in a folder of the test's own under `CARGO_TARGET_TMPDIR`. Steps 7 and 8 belong to other sessions and get their own parts of this section.

#### UI / manual

None for step 6.

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-control --locked` on `dff749e` | exit 0 | exit 0: 5 unit, 3 in `tests/admission_model.rs`, 11 in `tests/controller.rs`, 15 in `tests/journal.rs`, 28 in `tests/walkthroughs.rs`, 6 and 6 doc tests (`compile_fail` included), 0 failed | pass |
| `scripts/check.sh all` on `dff749e` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards`, `docs` all `ok` | pass |
| `grep -rnE 'Instant::now\|SystemTime\|std::fs\|std::env' crates/pool-control/src` | no line | no line (exit 1) | pass |
| `grep -rln rusqlite crates/pool-control/src` | `journal.rs` only | `crates/pool-control/src/journal.rs` | pass |

#### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0003 AC-17` | `AuthorityState::latch_conflict` and `latch_recorded` in `src/authority.rs`; `Controller::round` and `latch_recorded` in `src/snapshot.rs`; `ac17_walk_from_row_d_to_row_o_through_the_real_functions` in `tests/walkthroughs.rs` (once with a receipt of a real journal, once with the write refused as busy between the two calls; row o has no journal value built by hand), `ac17_latch_conflict_changes_nothing_unless_a_ready_state_is_asked_to`, and the `ac17_*` tests of `tests/controller.rs` (latch in memory before the write, the receipt, a failed write that stays listed and is tried again, a latch the journal already holds, a receipt for a cluster not latched, a state that is not `Ready`) | pass on the workstation |
| `0003 AC-18` | `ObservationBoard` in `src/observation.rs`; `build_snapshot` in `src/snapshot.rs`, the one caller of `writer_admission`; `ac18_the_board_keeps_one_sample_per_node` and `ac18_a_now_earlier_than_the_latest_sample_gives_the_same_snapshot`; the text `writer_admission` outside `crates/pool-control` is refused by rule W3 (milestone 1) | pass on the workstation |
| `0003 AC-19`, code half | `JournalRead` in `src/journal.rs`, with private fields and a `compile_fail` doc test; `ac19_journal_read_plans_bootstrap_with_the_board_and_the_configuration`; the guard half is milestone 1 | pass on the workstation |
| `0004 AC-10`, library half | `Journal::read`, `create`, `open_ready`, `set_conflict_latch` in `src/journal.rs`; the 15 tests of `tests/journal.rs`: marks, WAL mode, exactly the four tables and the first rows; `Absent` for no file and an empty file; `Unreadable` for a wrong application ID, another schema version, a missing `store` row, a writer without member row, a value out of range, an ID of the wrong syntax or length, a missing table, damaged pages, a file of another kind and a folder; reading leaves the file bytes as they were; `open_ready` advances the start sequence once per start, refuses a journal that changed and stops at the top; busy with no wait; the latch written once, advancing the revision | pass on the workstation; the half through `pool bootstrap` and `pool run` is steps 8 and 10 |
| `0004 AC-12`, library half | `PROBE_QUERY` and `observation_from_row` in `src/probe.rs` with three unit tests (a good row, sixteen other answers and no row give `Failed`); `ac12_build_snapshot_answers_every_cluster_with_the_expiry_of_the_configuration` and `ac12_a_failed_probe_row_takes_the_evidence_away_at_once` | pass on the workstation; the login, TLS and the query on PostgreSQL are steps 7 and 8 |
| `0003 AC-7` to `AC-12`, `AC-14`, `AC-16` | `tests/walkthroughs.rs` (four walks, `ac9_*`, `ac11_*`, `ac16_*`) and `tests/admission_model.rs` (8192 cases) run unchanged; no rule of `classify_startup`, `plan_bootstrap` or `writer_admission` changed | pass on the workstation |

**One reading.** When the snapshot generation cannot advance, `Controller::round` returns every cluster closed, as the spec says, but the spec names no reason for that case. The round builds its snapshot from the state `Unreadable`, so each cluster is closed with the existing `authority.unreadable` and no new reason name is added. The generation stays at 9223372036854775807. At one round per millisecond, it takes about 292 million years to get there. The unit test `a_round_whose_generation_cannot_advance_closes_every_cluster` locks it.

#### Follow up of decisions 57, 59 and 61

On 2026-11-15, by the developer of step 6, during step 8. Commits `4d7f0d6` (decision 57: `JournalRead::plan_bootstrap` renamed `bootstrap_plan`, so no method that other crates call holds a guarded text), `a2a6a6d` (decision 59: one rule for an empty database in the reader and in `Journal::create`, and `PRAGMA wal_checkpoint(TRUNCATE)` after the commit of `create`) and `6a9f23d` (decision 61: `AuthorityState::latch_recorded` takes a revision only when it is higher). Run on the tree `ae729d5` on macOS (workstation, not Linux and not CI).

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-control --locked` | exit 0 | exit 0: 5 unit, 3 in `tests/admission_model.rs`, 12 in `tests/controller.rs`, 19 in `tests/journal.rs`, 28 in `tests/walkthroughs.rs`, 6 and 6 doc tests, 0 failed | pass |
| New tests of decision 59 in `tests/journal.rs` | a file left after only `journal_mode = WAL` is `Absent` and `create` succeeds on it; a journal cut to one or two pages, one with its marks set to zero and one with its first 100 bytes set to zero are `Unreadable`; after `create` the main file alone reads as the journal | `ac10_a_file_left_after_only_the_wal_mode_is_absent_and_can_be_created`, `ac10_a_journal_cut_to_one_or_two_pages_is_unreadable`, `ac10_a_journal_without_its_marks_or_its_header_is_unreadable`, `ac10_after_create_the_main_file_alone_is_the_journal`: ok | pass |
| New test of decision 61 in `tests/controller.rs` | two receipts (revisions 2 and 3) handed over in reverse leave the state at 3 | `ac17_receipts_handed_over_in_reverse_keep_the_higher_revision`: ok | pass |
| `scripts/check.sh all` | exit 0, `guards` included | exit 0, all six `ok` | pass |

#### Checker run

Build plan step 6 only, on 2026-11-15 at 18:07 WIB, by the checker session, on the merged tree `59b7e2c` (macOS 27.0.1, arm64, with Docker 29.9.2 running; the Linux build ran in the pinned image, the rest on the workstation, and none of it is CI). Tools: `rustc 1.100.0`, SQLite 3.54.1 through `rusqlite 0.42.0`. Every command below was run in this session. The probes are a small program outside the repository that calls the public functions of `pool-control` on real journal files in a folder of its own, and reads those files back with its own SQLite connection. Its expected values are written from the text of spec 0003 and spec 0004.

**Verdict: step 6 PASS** on the workstation and in the pinned Linux image. Steps 7 and 8 were not built and are outside this run, so the milestone has no verdict yet.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-control --locked` | exit 0 | exit 0: 5 unit, 3 in `tests/admission_model.rs`, 11 in `tests/controller.rs`, 15 in `tests/journal.rs`, 28 in `tests/walkthroughs.rs`, 6 and 6 doc tests, 0 failed | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok` | pass |
| `scripts/check.sh linux-build` on the committed tree | exit 0 | exit 0: `host: aarch64-unknown-linux-gnu`, 18 test results `ok` and none failed, among them the 15 journal tests, the 11 controller tests and the 28 walk tests, `linux build check passed` | pass |
| `grep -rnE 'Instant::now\|SystemTime\|std::fs\|std::env' crates/pool-control/src`, and a search for `rusqlite`, a thread, a socket, the runtime and `unsafe` | no line; `rusqlite` in `journal.rs` only | no line; `rusqlite` is named in `crates/pool-control/src/journal.rs` alone; no thread, socket, runtime or `unsafe` text | pass |
| Probe, the old tests: `git diff 8332451..59b7e2c` for `tests/walkthroughs.rs` and `tests/admission_model.rs` | nothing removed or changed | `walkthroughs.rs` has 145 added lines and no removed line; `admission_model.rs` is unchanged; the four walks, the seven `ac11_*` and `ac16_*` tests and the model test pass | pass |
| Probe, the journal that a bootstrap writes: `Journal::read` on a missing path, `plan_bootstrap` with a board of fresh samples, an empty file, `Journal::create`, then the file read with another connection | the marks, WAL, exactly the four tables, the first rows | `application_id` 1347374924, `user_version` 1, journal mode `wal`, the tables `approved_topology`, `member`, `quarantine`, `store` and no other object, and the four `CREATE TABLE` statements equal the block of spec 0003 word for word. The `store` row holds the given authority ID, revision 1, start sequence 0, hold `none`. One approved topology per cluster with the writer, timeline 1, epoch 1, no latch, and one member row per node | pass |
| Probe, what the reader returns | `Absent` for no file and an empty file; the state of each witness | no file and a file of 0 bytes: no authority ID, state `uninitialized` without a witness and `missing` with one. A written journal: the authority ID, `unconfirmed` without a witness, `mismatch` with another one, `ready` with its own. A second `plan_bootstrap` is refused with `bootstrap.store_present`. `journal.sqlite3` is the same bytes after three reads | pass |
| Probe, every `Unreadable` case of spec 0003, each on a journal of its own | state `unreadable` | thirteen of thirteen: another application ID, a newer schema version, no `store` row, a writer without a member row, a timeline and a system identifier outside their type, a node ID of the wrong syntax, a hold that is no hold, an authority ID of two bytes, a missing table, a file that is no database, a damaged page, a folder in the place of the file. A hold of `restore_pending` reads as `restore_pending` and a quarantine row of a member as `ready` | pass |
| Probe, `PRAGMA quick_check`: a journal whose rows all read, with the count of free pages in its header set to 5 | state `unreadable` | another connection reads every row and reports `Freelist: size is 0 but should be 5` from `quick_check`; the reader gives `unreadable` | pass |
| Probe, `open_ready` | the start sequence advances once per start, `journal.changed`, `journal.busy`, `journal.open`, `journal.write` | first start: 1 in the answer and in the file. An authority classified before that start: `journal.changed`, nothing written. While another connection holds `BEGIN IMMEDIATE`: `journal.busy`. Second start: 2. No file: `journal.open`. At 9223372036854775807: `journal.write`, the file unchanged. `Debug` of an error prints the variant only | pass |
| Probe, the walk from row d to row o with a journal that the real functions wrote (`0003 AC-17`) | closed in memory first, the write second, no journal value built by hand | healthy samples: `main` open on `pg1`, epoch 1, revision 1, `fresh_until` 98 s plus 6 s. `pg2` reports that it is a primary: the round returns `authority.conflict_latched` and `evidence.second_primary`, `main` is on the list of latches not yet recorded, and the journal file is the same bytes as before. `pg2` in recovery again: `authority.conflict_latched` alone, which is row o. The write while another connection holds the lock: `journal.busy`; the next round still closes `main`, keeps it on the list and opens the other cluster at revision 1. The retry: a receipt for `main` at revision 2, and the file holds latch 1 and revision 2. After `latch_recorded`: the list is empty and the other cluster opens at revision 2 | pass |
| Probe, the same journal at the next start | closed from the journal, nothing written again | `authority.conflict_latched`, an empty list, the file the same bytes, the other cluster at revision 2. A second primary on the latched cluster asks for no second write. Setting the latch again returns revision 2 and changes no row | pass |
| Probe, the edges of the latch write | `journal.write` at the largest revision | at 9223372036854775807: `journal.write` and no latch in the file. A cluster the journal does not hold: `journal.write`. A journal whose revision another connection changed: `journal.changed`. An `unconfirmed` state with a second primary closes with `authority.unconfirmed` alone and latches nothing | pass |
| Probe, the board and the snapshot (`0003 AC-18`) | one sample per node, the later one, the held one on a tie, and no sample later than now | an older sample does not replace a newer one; a sample of the same time keeps the one held; a later failed sample replaces a good one; 200 samples of four nodes leave four. `build_snapshot` with a now one second before the latest sample equals the one with a now equal to it, and a second primary sampled after now closes with `evidence.second_primary`. Samples as old as the expiry: `evidence.writer_missing` and no `fresh_until`. One answer per configured cluster, in the order of the file. In `crates/pool-control/src` the one call of `writer_admission` is in `build_snapshot` | pass |
| Probe, building a journal value from outside the crate (`0003 AC-19`): fifteen small programs, one that uses the public functions and fourteen that try another way | the first compiles, the others do not | the first compiles. A struct literal of `JournalRead`, `LatchReceipt`, `Journal`, `TopologySnapshot`, `ClusterAdmission` and `SnapshotGeneration` fails with `E0451` or `E0423`; reading the field of a `JournalRead` or of a `Controller` fails with `E0616`; `Default`, `new` and `From<StoreProbe>` do not exist for `JournalRead`; the module `journal` is private; a `Controller` gives no way to change its state | pass |
| Probe, the probe row (`0004 AC-12`) | the query of the spec, and `Failed` for every answer the spec lists | `PROBE_QUERY` equals the block under [The probe](index.md#the-probe) byte for byte. A good row gives its three facts, also with timeline 4294967295. No row, two, four and no columns, a null in each column, a role of `t` or `TRUE`, a timeline of 0, `-1`, `+1`, ` 1`, `1.0`, 4294967296 or nothing, a system identifier of text, 0 or above its type, and an answer above 4096 bytes: `Failed`, 20 of 20 | pass |
| Probe, 51 changes to the code, one at a time in a private clone, each followed by `cargo test -p pool-control --locked` | every change to a rule is caught by a test | 42 caught and 9 not. Caught: all 20 changes to the latch, the board, the clamp, the snapshot and the round, among them a latch that is never set or always set, a snapshot that is not built again, a latch already held that is listed again, a receipt that is not taken, a tie that replaces, and a now that is not clamped. The 9 that no test caught are under the table | pass for the rules of the latch and the board |

| Criterion | Checker verdict |
|---|---|
| `0003 AC-17` | met. The walk runs through `writer_admission`, `latch_conflict`, `latch_recorded` and a real journal, once with a receipt and once with the write failing in between. The closed snapshot comes from memory before any write, and a latch the journal already holds is not written again |
| `0003 AC-18` | met, code half and guard half |
| `0003 AC-19` | met, code half and guard half. See the first note below for what it means for step 8 |
| `0004 AC-10`, library half | met: the marks, WAL, the four tables, `Absent`, every `Unreadable` case, a reader that does not change the file. The start of `pool run` and the commands are Build plan step 8 |
| `0004 AC-12`, library half | met: the query text and the reading of the row. The login, TLS and PostgreSQL are steps 7 and 8 |
| `0003 AC-7` to `AC-12`, `AC-14`, `AC-16` | unchanged: the old tests run as they were, and each new public record states its restart rule |

How the additions of the build were judged:

- **`Controller::state()` and `Controller::board()`** are not in the block of the spec. Both only read. Nothing in the spec forbids them.
- **`authority.unreadable` when the generation cannot advance.** The spec asks for every cluster closed and names no reason. An existing reason is used, which fits. The name says something about the journal that is not true in that case; it cannot be reached in practice.
- **`journal.changed` and `journal.write` from `set_conflict_latch`** for a journal whose revision moved and for a cluster without a row are not in the sentence of the spec about that function. Both fail closed, the latch stays in memory, and the error names are the ones the spec defines. Not against the text.
- **The probe row** refuses a sign and a space in the timeline, which the spec covers with "text that does not parse".

The changes that no test caught, for the tester. Each rule was still seen to hold, by a probe or by reading:

- The reader without `PRAGMA quick_check`. The probe above shows that it runs. The committed tests damage files in ways that fail before it.
- `synchronous = OFF`, `foreign_keys = OFF`, `wal_autocheckpoint = 10`, `journal_size_limit = 1024`. The four settings of every open that writes are set in `set_write_pragmas` (read in the code), and no test or probe can see them from outside the connection.
- A `busy_timeout` of 300 ms in place of 0, and a reader that opens the file for writing. The code sets 0 and the read only flag.
- An answer above 4096 bytes that still parses, and another text for `PROBE_QUERY`. The probe above covers both.

Notes that are not failures:

- **`JournalRead::plan_bootstrap` cannot be called from `apps/pool` as the guard stands.** Rule W3 forbids the text `plan_bootstrap` in every `.rs` file outside `crates/pool-control` (`0004 AC-2`, `0003 AC-19`), and the method has that name. Step 5 of [pool bootstrap](index.md#pool-bootstrap) calls it from the binary. The probe of milestone 1 showed that a line with `plan_bootstrap` in `apps/pool` is refused. The method needs another name, or the guard another text, before Build plan step 8. A question for the architect.
- **A bootstrap that stops after its first write blocks the next one.** `Journal::create` sets `journal_mode = WAL` before its transaction. That alone turns the empty file into a database of one page (4096 bytes). If the command stops there, the reader finds `page_count` 1 and no application ID and returns `Unreadable`, the next `plan_bootstrap` is refused with `bootstrap.store_present`, and `Journal::create` on that file returns `journal.open`. The probe made such a file with one `PRAGMA journal_mode = WAL` and saw all three. The spec says a stopped bootstrap "must not block the next bootstrap", and its rule covers only the file of 0 pages. It fails closed, and removing the file by hand is the way out. A question for the architect and the developer of step 8.
- `Debug` of a `JournalRead` prints the whole journal value: the authority ID, the system identifier, cluster and node IDs. The spec says the type "shows none of it". No function gives the value out, so nothing can be built from it, and spec 0003 allows each of these in telemetry. A custom `Debug` would match the sentence.
- `AuthorityState::latch_recorded` takes the revision of any receipt for a latched cluster, also a lower one. The journal hands out receipts in rising order, so it cannot go back today.
- CI evidence does not exist yet. It waits for the pull request.

### Build plan step 7: secrets, TLS and the backend connection

Build plan step 7, on 2026-11-15, on macOS (workstation) and in the Linux builder image. Commit `24d1c8f`, on top of `main` at `8332451`. Tools: `rustc 1.100.0`. Every test is hermetic: the node is a scripted peer inside the test that speaks TLS over an in memory stream (`tokio::io::duplex`), plus one loopback TCP connection. No container and no PostgreSQL; the first real contact is step 8. The certificates and the key under `crates/pool-core/tests/fixtures/` were made with `openssl` for these tests only (a CA, a node certificate for `node.test` valid until 2126, and a second CA).

#### UI / manual

None for step 7.

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-core --locked` | exit 0 | exit 0: 25 tests passed, 0 failed | pass |
| `cargo clippy -p pool-core --all-targets --locked -- -D warnings` | exit 0 | exit 0; Cargo prints `unused dependency tracing`, the known exception that ends with Build plan step 10 | pass |
| `scripts/check.sh all` before the commit | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok` | pass |
| `scripts/check.sh linux-build` at `24d1c8f` | exit 0 | exit 0 in 85 s; the `pool-core` tests ran in the image with AWS-LC compiled by its C compiler, 25 passed | pass |

What the tests hold:

- `secret.rs`: a credential, a verifier and a private key are read only from a regular file (after links) with mode `0600` or `0400` and the owner that the caller passes; each group or other bit, a directory, another owner and a link to a loose file are `secret.permissions`; a missing file and a dangling link are `secret.not_found`; a file over its limit (4096 bytes for a credential or verifier, 1048576 for a certificate file) is `secret.invalid`. The password rule (one trailing line end dropped, 1 to 1024 printable ASCII bytes), the verifier form of PostgreSQL with its ranges, exactly one private key, and at least one certificate. `Debug` of `Password`, `Verifier`, `PrivateKey`, `MockKey`, `ScramKeys`, `ExpectedSignature` and `Credential` prints `[REDACTED]`.
- `scram.rs`: the RFC 7677 vector on both sides (proof `dHzb...`, signature `6rri...`), a client and the server of this crate agreeing, a wrong password failing, a name that is no principal getting a stable salt of 16 bytes and the first principal's count and always failing after the full exchange, and the refusals of `p=` and of a malformed final message.
- `tls.rs`: the server configuration needs the key of the leaf and offers no ALPN; a node name must be a DNS name or an address; `Debug` of the node setup names no host.
- `backend/tests.rs`: a session opens with `n,,n=,r=<18 random bytes in base64>`, the startup packet holds exactly the login, the database and the forwarded parameter, parameter status and the key are kept, and the one row query returns the text columns; `SCRAM-SHA-256` is chosen beside `SCRAM-SHA-256-PLUS`; the keys are derived once for three logins with one salt and count; no startup packet is sent after `N`, a wrong CA or a wrong name; clear text, MD5, GSS, no method at all and a list without `SCRAM-SHA-256` end the attempt; a wrong password ends it with the node's `28P01`; a foreign nonce, a count of 0 or 600001, `e=` and a wrong server signature end it; a session needs `in_hot_standby` `off` (the probe does not) and the status `I`; more than 64 parameters end it; a silent node runs into the deadline; TCP to a closed port is `connect`.

#### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-9`, secret half | the `secret.rs` tests above; the state directory checks and the lock are step 8 | pass for the secret half on the workstation and in the Linux image; the rest not run |
| `0004 AC-15`, library half | the TLS and backend tests above: no startup packet unless the node answered `S`, the chain leads to the configured CA and the name matches | pass for the library half; the listener side and `psql` are steps 9 and 10 |
| `0004 AC-16`, library half | the SCRAM tests above: the right password passes, a wrong one and an unknown name fail after the same exchange; the backend password and the frontend verifier are separate types | pass for the library half; the message sequence over the listener is step 9 |

#### Changes after the verification of step 7 (decisions 62 to 69)

On 2026-11-15, one commit per decision, on top of `main` at `8458624`:

| Commit | Decision | Change | Test |
|---|---|---|---|
| `da0931f` | 66 | The last report of `in_hot_standby` before `ReadyForQuery` decides (`0004 AC-14`) | `the_last_report_of_in_hot_standby_decides`: `off` then `on` is `NotPrimary`, `on` then `off` passes |
| `e9bcacd` | 69 | `Debug` by hand on `ServerExchange`, `ClientExchange`, `ServerCredential`, `CertificateChain` and `TrustRoots` (`0004 AC-24`) | `the_exchange_types_print_no_nonce_and_no_client_message`; the counts of `CertificateChain` and `TrustRoots` in `files_are_read_through_their_limit` |
| `ca17e89` | 64 | One derivation at a time per credential: the cache is behind an async lock held across PBKDF2 | `eight_logins_at_a_cold_start_derive_the_keys_once` (one derivation), `a_waiter_goes_on_when_the_login_that_derives_is_cancelled` |
| `614eb24` | 68 | `derive_keys` and `verifier_text` take a `NonZeroU32`; the backend turns its parsed count (1 to 600000) into one | every SCRAM test |
| `251d840` | 67 | Protected files: the path is looked at only to refuse what is not a regular file, then type, mode and owner are checked on the open file and the content is read from it | `a_named_pipe_is_refused_without_waiting` (a pipe and a link to it: `secret.permissions` for a secret, `secret.invalid` for a certificate or CA file, within 5 s); the link to a good file in `secret_files_need_owner_only_mode_a_regular_file_and_the_owner` still passes |
| `def82a0` | 62 | The first line of `tests/fixtures/node.key` is the text `test key for node.test, used only by the tests of pool-core; it protects nothing`; the PEM reader skips it | every test that loads the key |

**The trace of `#[derive(Debug)]` in `pool-core` against the definition under "Log events".** Changed by hand, because they hold a nonce, a client message, a salt made from a name, or certificates from a file: the five types above. Judged safe, because they hold only IDs, generations, counts, durations or constant names: `Route`, `RouteState`, `RouteView` (`route.rs`: node and cluster IDs, epoch, revision, a duration); `LeaseGeneration`, `SessionId`, `StartupProfile` (`records.rs`); `Refusal`, `EndReason`, `Counters` (`report.rs`: reasons, node IDs, constant classes and SQLSTATEs, counts); `Phase`, `ServerError`, `Tracker` (`session.rs`); `WireError` (`wire.rs`); `BackendError`, `Purpose`, `TcpConnector`, `SecretError`, `ScramFault`, `RandomError`, `TlsPolicy`. Already by hand before: `Password`, `Verifier`, `PrivateKey`, `ScramKeys`, `MockKey`, `ExpectedSignature`, `Credential`, `Node`, `Login`, `BackendConnection`, `NodeTls`, `VerifiedPrincipal`, `PoolKey`, `Lease`, `Server`, `ServerParts`. Outside `pool-core`, several types of `pool-protocol` still derive `Debug` over content (`Message`, `ParameterStatus`, `SaslInitialResponse`, `ClientFirst`, `ServerFirst`, `ClientFinal`, `Forwarded`, `NegotiateProtocolVersion`); they are not part of this item.

Results on the final tree: `cargo test -p pool-core --locked` 37 unit and 27 session tests passed; `cargo test -p pool --locked` passed (7, 84, 10; the 6 integration tests run under `integration`); `scripts/check.sh all`, `scripts/check.sh linux-build` and `scripts/check.sh integration` (6 of 6, with Docker) exit 0.

#### Checker run

Build plan step 7 only, on 2026-11-15 at 18:20 WIB, by the checker session, on the merged tree `77ca71f` (macOS 27.0.1, arm64, with Docker 29.9.2 running; the Linux build ran in the pinned image, the rest on the workstation, and none of it is CI). Tools: `rustc 1.100.0`, OpenSSL 4.0.3 for the certificates of the probes, Python 3.9 for one cross check. Every command below was run in this session. The probes are a small program outside the repository that calls the public functions of `pool-core`. Its node is a scripted peer of its own over an in memory stream. Its certificates are a CA and three node certificates (valid, expired, not yet valid) made for this run in a folder outside the repository, so they are not the ones of `crates/pool-core/tests/fixtures/`.

**Verdict: step 7 PASS** on the workstation and in the pinned Linux image. Step 8 is not built, so the milestone has no verdict yet. Two things were known to differ from the spec text before this run and are being changed in Build plan step 9, so they were noted and not judged: a message of a node during startup is read up to 65536 bytes where the spec now says 16384 (decision 55), and `scram.rs` takes random bytes through a private call and not through `pool_core::random::fill_random` (decision 58).

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-core --locked` | exit 0 | exit 0: 25 tests passed, 0 failed | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok` | pass |
| `scripts/check.sh linux-build` on the committed tree | exit 0 | exit 0: `host: aarch64-unknown-linux-gnu`, 18 test results `ok` and none failed, among them the 25 tests of `pool-core`, `linux build check passed` | pass |
| Probe, the texts of rule W3 in `crates/pool-core/src`, and `unsafe`, `unwrap`, `expect` and `panic!` outside the tests | only texts the baseline allows; no `unsafe`, no `unwrap` | `tokio`, `TcpStream` and `TcpListener` only, each allowed for `crates/pool-core`. No `unsafe`, no `unwrap()`. One `expect`, in `derive_keys`, for an iteration count of 0, with a `# Panics` section | pass |
| Probe, the mode of a secret file: a credential file with each of the 256 modes that give the owner the read bit | accepted exactly when no group and no other bit is set | 256 of 256. `0600` and `0400` pass, and so does every mode that adds only owner bits. Each mode with a group or other bit is `secret.permissions` | pass |
| Probe, the other rules for a secret file | `secret.permissions` or `secret.not_found` | mode `0200`, which the owner cannot read: `secret.permissions`. Another owner: `secret.permissions`. A directory with mode `0700`: `secret.permissions`. A named pipe with mode `0600`: `secret.permissions`, at once and without waiting on the pipe. No file: `secret.not_found`. A link to a good file is followed, and a link to a file with a group bit is `secret.permissions` | pass |
| Probe, what each file may hold | the rules under [State directory and protected files](index.md#state-directory-and-protected-files) | password: 1 and 1024 bytes, a space and a tilde, and one trailing LF or CR LF pass; nothing, only a line feed, two line feeds, a tab, U+001F, U+007F, a zero byte, a byte above ASCII, a lone carriage return and 1025 bytes are `secret.invalid`. Verifier: a count of 1 and 600000, a salt of 1 and 64 bytes pass; a count of 0, 600001, `-1` or none, a salt of 0 or 65 bytes, a key of 31 or 33 bytes, an MD5 hash and a clear password are refused. Private key: PKCS 8, SEC 1 and PKCS 1 are read; two keys, a certificate alone and an empty file are `secret.invalid` | pass |
| Probe, the limits | a longer file is `secret.invalid` | a valid private key padded to 65536 bytes is read, to 65537 it is `secret.invalid`. A valid CA file padded to 1048576 bytes is read, to 1048577 it is `secret.invalid`. The limit of 4096 for a credential cannot be seen from outside, because no valid credential is that long; it was read in the code | pass |
| Probe, certificate and CA files | no mode rule; `secret.not_found` or `secret.invalid` | a file with mode `0644` is read. No file: `secret.not_found`. No certificate in it, and a directory: `secret.invalid`. A key that does not belong to the leaf: `secret.invalid`. The server configuration offers no ALPN protocol. See the first note below for a file that cannot be read | pass |
| Probe, what is printed | `[REDACTED]`, and no secret, path or peer text in any error | `Debug` of a password, a verifier, a private key, a credential, derived keys and the mock key is `[REDACTED]`. `Debug` of a login prints a count only. With the canary in the password, the login, the database, the file name and the error text of the node: the canary is in no `Debug` and no `Display` of a value or an error. `Display` of a backend error is `backend attempt failed:` and a constant word | pass |
| Probe, the verifier against another implementation: `verifier_text` for `pencil`, the salt of RFC 7677 and 4096 rounds, and the same computed with `hashlib` and `hmac` of Python | the same text | equal: `SCRAM-SHA-256$4096:W22ZaJ0SNY7soEsUEjb6gQ==$WG5d8oPm...:wfPLwcE6...` | pass |
| Probe, SCRAM as a server | the right password passes; a wrong one and an unknown name fail the same way | the right password passes and the client accepts the server signature. A wrong password and a name that is no principal both end with the same fault after the whole exchange. The server-first message has the same shape for all three: three parts, a nonce of 48 characters, a salt of 24 characters, the same count. One unknown name gets the same salt twice, another name another salt, another mock key another salt. `n,,` and `y,,` are accepted with any user name; `p=`, an authorization identity, an extension, `m=`, an empty nonce and a nonce with a space are protocol faults. One changed bit in the proof fails; another header or a shorter nonce in the final message is a protocol fault | pass |
| Probe, the time of the last step of the server, 20000 runs each, taken in turn | no difference that tells the two apart | wrong password 4.63 µs, unknown name 4.72 µs on average, in the unoptimized build. Both do the same HMAC, SHA-256 and comparison in constant time (read in the code) | pass |
| Probe, the backend connection, the good path | steps 2 to 7 of [Backend connection](index.md#backend-connection) | the node sees `SSLRequest` first, then over TLS a startup packet with exactly `user`, `database`, `application_name`, `client_encoding`, then the mechanism `SCRAM-SHA-256` and `n,,n=,r=` with 24 characters. The connection keeps the parameters in order, the key and the status idle. `Debug` of it names no login, database, password or key | pass |
| Probe, no fallback and a checked certificate | the attempt ends, and nothing more is sent | the node answers `SSLRequest` with `N`, with `E`, or closes: `tls_refused`, and the node receives no further byte. A chain that does not lead to the configured CA, a certificate for another name, an address as server name, an expired certificate and one that is not yet valid: `tls` each time, and the node sees no startup packet. The leaf followed by its CA is accepted. The scripted node asks for no client certificate and the handshake passes | pass |
| Probe, the authentication of a node | only `SCRAM-SHA-256`, with every check of step 5 | a clear text password, MD5, GSS, `AuthenticationOk` at once and `SCRAM-SHA-256-PLUS` alone: `authentication_method`. PLUS beside plain: POOL chooses plain with the header `n`. A nonce that does not start with ours, a count of 0 or 600001, and no `AuthenticationOk` after the final message: `protocol`. `e=` and a wrong server signature: `authentication`. A wrong backend password: the error of the node with SQLSTATE `28P01`, and none of its text | pass |
| Probe, the messages up to `ReadyForQuery` | step 6 and step 7 | `BackendKeyData` twice: `protocol`. A notice: dropped. An error: `server_error`. Status `T`: `not_idle`. `in_hot_standby` `on` or missing, for a session: `not_primary`; `on` for the probe: accepted. 64 parameters: accepted; 65, or more than 16384 bytes together: `protocol`. A body of 65537 bytes, an unknown tag and a row description: `protocol`. The stream closed early: `closed`. A node that says nothing: `timeout` after 303 ms with a deadline of 300 ms | pass |
| Probe, the one row query | the columns of one row, nothing for 0 or 2 rows, the error after `ReadyForQuery` | one row with a null and an empty text: its three columns. No row, two rows and an empty query answer: nothing. An error and then `ReadyForQuery`: the SQLSTATE of the node. A COPY response, and a row that claims two columns and holds one: `protocol` | pass |
| Probe, the kept keys (decision 43) | one derivation for the same salt and count | three logins with one salt and count: 1 derivation. A login with another salt: a second one. See the notes below for what one entry and a cold start mean | pass |
| Probe, the committed test files | used by tests only | `ca.pem`, `node.pem` (for `node.test`, signed by that CA, valid until 2126) and `other-ca.pem` are certificates; `node.key` is the P-256 key of `node.pem`. The keys of the two CAs are not in the repository. Every use is inside `#[cfg(test)]` code of `pool-core` | see the notes |
| Probe, 56 changes to the code, one at a time in a private clone, each followed by `cargo test -p pool-core -p pool-protocol --locked` | every change to a check is caught by a test | 42 caught and 14 not. Caught: the group bits, the other bits, the owner and the link rule of a secret file; a proof that is not compared; a server signature or an `e=` that is accepted; all six changes to the nonce, binding and count checks; any answer to `SSLRequest`; any mechanism; a signature that is not checked; the standby and the idle check. The 14 that no test caught are under the table | pass for the permission bits, the nonce checks and the proof |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-9`, secret half | met: `secret.permissions` for any group or other bit, for a file that is not regular and for another owner; `secret.not_found`; the limits and the content rules. The state directory, the lock and the exit status are Build plan step 8 |
| `0004 AC-15`, library half | met: toward a node, no startup packet unless the node answered `S`, the chain leads to the configured CA and the name matches; a certificate outside its dates is refused too. The server side offers no ALPN and asks for no client certificate. TLS 1.3 and 1.2 as the only versions were read in the code, not seen on the wire. The listener and `psql` are steps 9 and 10 |
| `0004 AC-16`, library half | met: SCRAM on both sides, the verifier form of PostgreSQL, a made up exchange for an unknown name that has the same messages, the same shape and the same cost as a wrong password. The exchange over the listener and the limit of 1024 bytes on the wire are step 9 |

How the readings of the build were judged:

- **A file that cannot be opened for lack of permission is `secret.permissions`, other errors of the operating system are `secret.invalid`.** For a secret file that fits `0004 AC-9`. For a certificate or CA file see the first note.
- **A private key file needs exactly one key**, and other parts of the file are skipped. That is the row of the spec.
- **A second `BackendKeyData` is a protocol error, and a notice during startup is dropped.** The spec says "kept privately" and "a notice is dropped". Refusing a second key is the strict reading, and it is accepted.
- **The one row query reports an error only after `ReadyForQuery`.** That leaves the connection at a message boundary, and it is accepted.

The changes that no test caught, for the tester. Where a probe above covers the rule, it says so:

- A secret file that need not be a regular file. The tests use a directory with group and other bits, which fails on the mode. The probe used a directory with mode `0700` and a named pipe.
- The four limits on file size (a file one byte above its limit, 8192 for a credential, 131072 for a key, twice the limit for a certificate). The tests use files whose content is invalid anyway. The probe padded valid files to the limit and one byte more, for the key and the CA file.
- A file that may not be opened reported as `secret.invalid`. The probe covers it with mode `0200`.
- A made up salt that does not depend on the mock key. The probe covers it.
- A second `BackendKeyData` that replaces the first, parameter bytes without a limit, an error inside the one row query that is dropped, and kept keys used for another salt. The probe covers all four.
- Any `Authentication` message after the final one counted as `AuthenticationOk`. No test and no probe sends another `Authentication` code there. The code refuses it (read).
- `Display` of a backend error that prints its `Debug` form, and an unknown name that is not forced to fail. The second cannot be told from the code as it is, because the made up stored key cannot be met.
- No committed test uses a certificate that is expired or not yet valid. The probe did.

Notes that are not failures:

- **A certificate or CA file that exists and cannot be read gives `secret.permissions`.** The row of the spec for such a file names `secret.not_found` or `secret.invalid`, and the message of `secret.permissions` speaks of a secret file. The probe saw it with a CA file of mode `0000`. A question for the architect: the row or the code.
- **`crates/pool-core/tests/fixtures/node.key` is a committed private key.** The spec says under [Tests and who owns them](index.md#tests-and-who-owns-them) that hermetic session tests put a pass through in the place of TLS, "so no private key is committed". The key belongs to a certificate for the name `node.test` from a CA whose key is not in the repository, nothing outside test code reads it, and it guards nothing. `SEC-4` speaks of the secrets of a deployment. Without it the TLS checks of `0004 AC-15` could be tested only on the fixture. It still goes against that sentence, and a scanner for secrets will report it. A question for the architect: change the sentence and name the file as a test key, or make the key at test time.
- **Keys are derived more than once at a cold start.** Eight first logins at the same time with one new credential made eight derivations. The spec says "one derivation per credential and run" and that a burst of logins cannot keep the probe from a blocking thread. That holds once the first login has filled the entry, and not for the logins that start before it has. At a count of 600000 each derivation takes a noticeable part of a second on one of the four blocking threads.
- **The credential keeps one entry.** A login with another salt replaces it, so going back to the first salt derives again (three derivations for the salts A, B, A). PostgreSQL keeps one salt per role, so one cluster does not alternate.
- **`in_hot_standby` reported as `off` and later as `on` passes for a session**, because any report of `off` is enough. PostgreSQL reports each parameter once during startup.
- The check of a secret file and its reading are two lookups of the same path: `metadata` first, then `open`. Someone who can replace the file between the two already has the rights of the directory. Checking the opened file would close it.
- A certificate or CA path is opened before its type is checked, so a named pipe at that path would make the start wait (read in the code, not tried). A secret file is checked before it is opened.
- `derive_keys` is public and panics for a count of 0. Every caller in the crate passes a count that a parser has checked.
- `Debug` of a `ServerExchange` prints the text of the client-first message, which holds whatever user name the client wrote there. Nothing logs it today.
- CI evidence does not exist yet. It waits for the pull request.
#### Checker run of the additions to step 7

The three changes to step 7 that came with Build plan step 9 (commit `ca122c2`, decisions 55 and 58), on 2026-11-15 at 19:03 WIB, by the checker session, on the merged tree `8783337`. Every command below was run in this session, with a scripted node of the checker's own.

**Verdict: PASS.** The two differences that the checker run above noted and did not judge are gone.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| Probe, the limit of a message of a node during startup: a notice with a body of 16384 bytes, then of 16385 bytes, before `ReadyForQuery` | accepted, then `protocol` | 16384 bytes: the connection opens. 16385 bytes: `protocol` | pass |
| Probe, the limit of a probe answer: the one row query answered with a data row, a command complete and `ReadyForQuery` whose bodies are 4096 bytes together, then 4097 | the row, then `protocol` | 4096 bytes: one row with its column of 4086 bytes. 4097 bytes: `protocol`. No single message of the second answer is above 4096 bytes, so the bodies are counted together | pass |
| Probe, one source of random bytes: a search of every first party crate for a call that asks for random bytes | `pool_core::random::fill_random` alone | `aws_lc_rs::rand::fill` is called in `crates/pool-core/src/random.rs` and nowhere else. `scram.rs` gets its nonce and the mock key through `fill_random` | pass |
| Probe, `random_failed` | a login that cannot get random bytes is closed without a message and counted | read in the code, not seen: a failure of the generator cannot be made from outside. `ScramFault::Random` at the server nonce and `BackendError::Random` at the client nonce both end the login with no message and count `random_failed` | read |

### Build plan step 8: bootstrap, the authority command and the first run against PostgreSQL

On 2026-11-15, on macOS (workstation, Docker Desktop 29.9.2, not Linux and not CI). Commits `0e93678` (step 8) and `7231266` (the finding `random.failed` with the random bytes of step 9), on the tree `ae729d5`. Image `postgres:18.7-bookworm`, index digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, `arm64` variant. Observed in it: `postgres --version` and `psql --version` print 18.7 (Debian 18.7-1.pgdg12+2), and `openssl version` prints OpenSSL 3.0.22.

#### UI / manual

- [ ] The CI job `integration` on the hosted runner → `NOT_RUN`, owner: user. It enters the workflow with manual step M2 (after Build plan step 12), and its Linux evidence waits for a pull request.
- [ ] A Docker daemon on the machine that runs the check (manual step M4) → met on this workstation; elsewhere the check exits 3 without it.

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool --locked` | exit 0 | exit 0: 7 unit, 84 in `tests/check_config.rs`, 10 in `tests/commands.rs`, 6 ignored in `tests/integration.rs`, 0 failed | pass |
| `scripts/check.sh integration` | exit 0, two containers started and removed | exit 0: 6 tests of `tests/integration.rs` passed against PostgreSQL 18.7; `docker ps -a`, `docker volume ls` and `docker network ls` the same before and after, no temporary directory left | pass |
| `scripts/check.sh integration` with one assertion changed to fail | exit 1, the containers removed | exit 1, `5 passed; 1 failed`; no `pool-it-` container, no new volume, no temporary directory left; the test file restored afterwards | pass |
| `scripts/integration/run.sh` stopped with SIGTERM once both containers ran | exit 1, the containers removed | exit 1; no `pool-it-` container and no temporary directory left | pass |
| `DOCKER_HOST=unix:///nonexistent/docker.sock scripts/check.sh integration` | exit 3 | exit 3, `integration: the Docker daemon is not running; start it and run scripts/check.sh integration again` | pass |
| `scripts/check.sh all` | exit 0 | exit 0, all six `ok` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0, `linux build check passed` | pass |
| `scripts/prove-checks.sh` | every case rejects its change | exit 0, `all checks reject their bad change (linux-build not proven)` | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, the valid report | pass |

#### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-9` | `apps/pool/src/state.rs`; in `tests/commands.rs`: `ac9_a_state_directory_that_does_not_exist_is_state_not_found`, `ac9_a_state_directory_of_the_wrong_type_or_mode_is_state_permissions` (seven modes, a regular file, a link), `ac9_a_journal_file_of_the_wrong_type_or_mode_is_state_permissions` (each of the four files with five modes, as a folder and as a link; other entries not looked at), `ac9_a_second_command_while_the_lock_is_held_is_state_locked_and_changes_nothing`, `ac9_a_secret_or_ca_file_that_cannot_be_used_ends_bootstrap_with_its_key`; `pool run` comes with step 10 | pass on the workstation for `bootstrap` and `show-authority` |
| `0004 AC-10`, completed | `it_bootstrap_writes_the_journal_and_show_authority_reads_it_back`: the file of a real bootstrap has mode 0600, the SQLite header, WAL mode, `user_version` 1 and `application_id` 1347374924, and the reader gives revision 1 and start sequence 0; `show-authority` leaves its bytes as they were; the advance at `pool run` comes with step 10 | pass on the workstation |
| `0004 AC-11` | `it_bootstrap_writes_the_journal_and_show_authority_reads_it_back` (exit 0 with a new ID, a second bootstrap refused with `bootstrap.store_present`, `show-authority` prints the same ID as `unconfirmed` and, with the witness, `ready`), `it_bootstrap_json_prints_the_new_id`, `it_a_node_that_cannot_be_reached_refuses_bootstrap_and_writes_no_file`; `show_authority_reports_each_state_it_can_reach_without_a_journal` and `ac11_bootstrap_against_a_node_that_does_not_answer_creates_no_journal` in `tests/commands.rs`; every output is searched for the paths, `pz7_`, `pz7-`, `127.0.0.1` and `pg.pool.test` | pass on the workstation |
| `0004 AC-12` | `it_the_probe_logs_in_as_the_monitor_login_over_tls_and_asks_one_question` (`pz7_monitor` has LOGIN and nothing else and no membership; the log shows `connection authorized: user=pz7_monitor database=postgres application_name=pool-probe SSL enabled` and the query), `it_a_wrong_password_ca_or_server_name_is_a_failed_probe`, `it_the_second_server_is_another_identity` (`bootstrap.identity_mismatch`), and the closed port and the silent listener of `tests/commands.rs` within the timeout | pass on the workstation |
| `0004 AC-26` | `scripts/check.sh integration` with `scripts/integration/run.sh` and `scripts/integration/initdb/10-pool-fixture.sh`: the rows above for a pass, a failing test, a stop part way and no daemon | pass on the workstation; the CI job is manual step M2 |

**Follow up of 2026-11-15** (from the verification of milestone 3): a bootstrap that stopped after setting the WAL mode left a journal file of one page with no mark and no table, which the reader counts as absent, and the next `pool bootstrap` refused it with `state.permissions` because `create_empty_journal` took only a file of 0 bytes. Fixed in commit `b33f155`, `fix(pool): spec 0004 step 8, a stopped bootstrap's journal file no longer blocks the next`: a file that exists is handed to `Journal::create`, which applies the rule for an empty database. The test `it_a_bootstrap_that_stopped_after_the_wal_mode_does_not_block_the_next` writes the exact page that SQLite 3.54 leaves (compared byte for byte with a file SQLite made), failed on the old code with `state.permissions`, and passes now; `scripts/check.sh integration` 21 of 21.

**Follow up of 2026-11-15, a test fix** (found by `dbg-f4-fix` after one failure in `linux-build` on `cc58ae3`): `ac9_a_second_command_while_the_lock_is_held_is_state_locked_and_changes_nothing` in `tests/commands.rs` failed on Linux because a child that another test of the same binary was starting held a copy of the descriptor of `pool.lock` between fork and exec, so the `flock` outlived `drop(lock)` (89 of 300 runs of the suite in the pinned image failed, 0 of 500 with the fix, by `dbg-f4-fix`). Commit `2590ba6`: every child of `tests/commands.rs` now starts under one mutex that is released after `spawn`, the pattern `tests/check_config.rs` already has. `tests/check_config.rs` needs nothing more, and `tests/integration.rs` with `tests/support/mod.rs` neither: no test there holds a lock in the test process, the check runs them with `--test-threads=1`, and every child starts from the one test thread. The product does not start child processes; nothing under `apps/pool/src/` changed.

**Follow up of 2026-11-15, the lock file race** (the second finding of the verification of milestone 3): when `pool.lock` did not exist and several commands started together, the losers of `create_new` got `AlreadyExists`, which became `state.permissions` instead of `state.locked`. Commit `fd62cf4`: the file that the other command made is opened and `try_lock` decides; the opened file is checked on its descriptor (a regular file, mode `0600`, the owner of the directory, and the same inode as the path read without following a link), and every other failure stays `state.permissions`. The test `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked` in `tests/commands.rs` releases eight `show-authority` together through named pipes that carry their configurations, in 25 rounds, and accepts only exit 0 or `state.locked`. In a container of the pinned image of `linux-build` (`rust:1.100.0-slim-bookworm`, the test run 30 times each): 0 of 30 failed with the fix, 30 of 30 failed on the old `state.rs`, each with `state.permissions`. On macOS: 10 of 10 failed on the old code; with the fix it passed, and 24 to 57 of the 200 processes of a run got `state.locked`.

**Follow up of 2026-11-15, the lock race test could hang** (found by `chk-f4-verify` on tree `d4fdb62`): once on macOS, on a very busy machine, one `show-authority` of `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked` stayed in `read()` on the named pipe that carried its configuration, with no writer left, and the test waited for it without end. The cause is not proven: a loop of 3000 rounds of `cat` reading a named pipe whose writer opened, wrote and closed at once (about 2250 of them with 20 busy loops) never lost the end of the pipe. The test no longer depends on it. The configuration is a regular file again, and each child waits at a gate of its own: `/bin/sh` reads one line from a named pipe and then becomes `pool show-authority`. The line ends that read, so nothing waits for the end of a pipe, and the test keeps the gates open until every child has finished. Each stage has a limit of 60 seconds per child: the gate must be opened by its child (the test opens it without waiting and asks again), and the child must then finish. Past the limit, every child of the round is killed and the test fails with the round, the child and the stage (checked by making the gate never open, and the child never finish: both failed within the limit with that message, and no child was left). The assertions are unchanged. Runs with the fix: on macOS, 30 of 30 runs of the test alone and 40 of 40 runs of the whole `commands` binary, each with 20 busy loops on 10 cores, passed. In a container of the pinned image of `linux-build`, as a user that is not root: 30 of 30 runs alone and 30 of 30 runs of the whole binary with 16 busy loops passed. 3 to 12 of the 200 processes of a run still got `state.locked`, so the commands still meet at the lock file.

**Readings.** The JSON of `bootstrap` when it is refused uses the shape of spec 0001 (`config_version` and `capabilities` `null`, `result` `refused`), since the spec gives its fields only for exit 0. The state directory and the four journal files are checked without following links, so a link in their place is `state.permissions`. A bootstrap whose runtime could not start has no sample and is refused with `bootstrap.evidence_missing`, writing nothing.

### Checker run of milestone 3

Build plan step 8, the follow up of step 6 (decisions 57, 59 and 61) and the changes to step 7 (decisions 62 to 69), on 2026-11-15, by the checker session. macOS 27.0.1, arm64, `rustc 1.100.0`, Docker 29.9.2 running, PostgreSQL 18.7 from `postgres:18.7-bookworm` by the digest of `scripts/check.sh`. The Linux runs used the pinned build image. None of it is CI. Every command below was run in this session. The probes are the checker's own programs and scripts outside the repository: one drives the built `pool` binary with no database, one takes the place of the test command inside `scripts/integration/run.sh` and drives `pool` against the fixture, and one calls the public functions of `pool-core` and `pool-control`. The session contacted only the two PostgreSQL containers that `scripts/integration/run.sh` started for it, with the fixture credentials.

**The run was cut in two by a usage limit.** The table says which part ran when. Before the pause the tree was `08091ec`. After it the tree was `3b58997`, which differs from `08091ec` in documents only, so the results before the pause hold for it. The first finding was then fixed on `main` (`b33f155`), and the last runs are on `ecd0ee5`, which differs from `3b58997` in `apps/pool/src/state.rs` (the fix) and in the tests of `apps/pool/tests/`.

**Verdict: milestone 3 FAIL, for one small open finding against `0004 AC-9`.** On a new state directory, a command that loses the race to create `pool.lock` ends with `state.permissions`, where `state.locked` is right (finding 2). It fails closed, and the lock itself never let two commands in. Everything else is met: `0004 AC-10`, `AC-11`, `AC-12` and `AC-26`, `0003 AC-17` and `AC-19` after decisions 57, 59 and 61, and the changes to step 7. Finding 1 (a stopped bootstrap blocked the next one) failed on `3b58997`, was fixed in `b33f155` and passes on `ecd0ee5`.

| Command or probe | Tree and time | Expected | Observed | Result |
|---|---|---|---|---|
| `scripts/check.sh all` | `08091ec`, before the pause, and again on `ecd0ee5` | exit 0 | exit 0 both times, all six `ok`; on `ecd0ee5` 24 test results `ok`, none failed | pass |
| `scripts/check.sh integration` | `08091ec`, before the pause | exit 0, nothing left behind | exit 0, 8 of 8 tests against PostgreSQL 18.7, about 11 seconds in all; no `pool-it-` container of this run, no new volume, no temporary directory | pass |
| `scripts/check.sh integration` | `ecd0ee5`, after | exit 0, nothing left behind | exit 0, 21 of 21 in 45 seconds, with `it_a_bootstrap_that_stopped_after_the_wal_mode_does_not_block_the_next`; 89 volumes before and after, no container of this run, no temporary directory | pass |
| `scripts/check.sh linux-build` | `3b58997`, after | exit 0 | exit 0, 22 test results `ok`, none failed, `linux build check passed` | pass |
| The command probe with no database, 100 checks | `08091ec`, before the pause | every check as the spec says | 100 checks, 0 mismatches (details below) | pass |
| The fixture probe, 34 checks | first run on `08091ec` before the pause, second run on `3b58997` after | every check as the spec says | second run: 33 as expected, 1 mismatch, which is finding 1. The first run had five more mismatches that were mistakes of the probe (it damaged a copy of the wrong file), corrected for the second run | **fail** (finding 1) |
| The fixture probe again | `ecd0ee5`, after | 34 as expected | 34 checks, 0 mismatches | pass |
| `scripts/integration/run.sh` with a test command that fails | `3b58997`, after | exit 1, nothing left | exit 1; its two containers and its temporary directory are gone | pass |
| `scripts/integration/run.sh` stopped with SIGTERM while the test command ran | `3b58997`, after | exit 1, nothing left | exit 1; its two containers and its temporary directory are gone, but only when the test command had ended by itself (see the notes) | pass |
| `DOCKER_HOST=tcp://127.0.0.1:1 scripts/check.sh integration` | `3b58997`, after | exit 3 | exit 3, `integration: the Docker daemon is not running; start it and run scripts/check.sh integration again`, `== FAILED: integration` | pass |
| The lock probe on Linux: a release `pool` in the pinned image, processes only, with a listener that accepts and never answers as the node | `3b58997`, after | one holder at a time, `state.locked` for the others | see "The lock" below; 38 commands in 70 rounds ended with `state.permissions` | **fail** (finding 2) |
| `tests/commands.rs` 200 times in the pinned image | `ecd0ee5`, after | 200 runs pass | 200 runs, 0 failed, each `12 passed` | pass |
| The probe of the changes to step 7 and of decision 61, 61 checks with the ones of step 4 | `3b58997`, after | every check as the spec says | 0 mismatches | pass |
| The guard probe, 125 changed lines, each in a private copy | `3b58997`, after | each refused or accepted as rule W3 says | 125 of 125 | pass |
| The whole session probe of step 9, again | `3b58997`, after | as in the run of step 9 | 101 checks, 99 as expected; the 2 others are known mistakes of the probe (it counts the closed backends of two sessions as one, and compares a decimal number with a hexadecimal one) | pass |

#### The state directory and the lock (`0004 AC-9`)

Before the pause, on macOS, with no database:

- A state directory that does not exist gives `state.not_found`. A directory with mode 0755, 0750, 0701, 0600, 0500, 0770 or 01700, a regular file in its place and a link to a good directory all give `state.permissions`, exit 3.
- Each of `journal.sqlite3`, `journal.sqlite3-wal`, `journal.sqlite3-shm` and `pool.lock` with mode 0644, 0400 or 0660, as a directory and as a link, gives `state.permissions`, and the directory is the same afterwards. Other entries in the directory are not looked at.
- `pool.lock` is made with mode 0600, also under a umask of 0.
- Ten problems with the monitor credential file (missing, mode 0644, mode 0640, empty, a tab inside, a named pipe) and the CA file (missing, no certificate inside, a named pipe, not readable) each end bootstrap at once with exit 3 and `error: <code> at <key>: <message>`, with the key `cluster.monitor_credential_file` or `node.tls_ca_file`. A named pipe as the credential is `secret.permissions` and as the CA is `secret.invalid`, and neither blocks.

The lock, as the product takes it (`File::try_lock` on `pool.lock`):

| Probe | Runs | Two holders | No holder | `state.locked` where it was not expected | Another result |
|---|---|---|---|---|---|
| macOS, before the pause: while one bootstrap held the lock inside its probe, 118 other commands | 118 | 0 | 0 | 0 (all 118 got `state.locked`, as expected) | 0 |
| macOS, before the pause: eight bootstraps started together, each holding the lock for 0.8 seconds, then `show-authority` at once | 30 rounds | 0 | 0 | 0 of 30 after the round | 0 |
| Linux: a bootstrap holds the lock for 0.15 seconds, one command during it and one right after the holder ended | 300 cycles | 0 | 0 | 0 of 300 after the holder ended (300 of 300 during it, as expected) | 0 |
| Linux: eight bootstraps started together on a state directory where `pool.lock` exists | 40 rounds, 320 commands | 0 | 0 | 0 | 0 |
| Linux: the same on a new state directory with no `pool.lock`, first run | 30 rounds, 240 commands | 0 | 0 | 0 | 11 commands with `state.permissions` |
| Linux: the same, second run | 40 rounds, 320 commands | 0 | 0 | 0 | 27 commands with `state.permissions` |

So the lock is exclusive in every run, and a command never got `state.locked` once the holder had ended.

**Finding 2 (open, sent to the developer of step 8).** On a new state directory, commands that start together race to create `pool.lock`. `StateDirectory::open` creates the file with `create_new`, and `create_protected` turns every error into `StateError::Permissions`, also the one that says the file exists by now. The loser then prints `error: state.permissions: the state directory or a file in it has the wrong type, mode or owner`, which is not true, and `0004 AC-9` gives `state.locked` for a second command. Nothing is written and never two commands pass, so this is a wrong code and not a safety fault. It was not seen on macOS in 30 rounds. `apps/pool/src/state.rs` of `ecd0ee5` has the same lines.

**The unstable test.** `ac9_a_second_command_while_the_lock_is_held_is_state_locked_and_changes_nothing` failed once in `linux-build` on `cc58ae3`. In this session it passed in the one `linux-build` run on `3b58997`. My reading before I saw the fix: the test takes the lock inside the test process, which runs other tests on other threads, and a child that another thread starts keeps a copy of the lock descriptor until its exec. That is the cause `dbg-f4-fix` found, and commit `2590ba6` lets one child start at a time. With it the test binary passed 200 of 200 runs in the pinned image. The product lock is not the cause: the table above has no late `state.locked` in 330 runs.

#### Bootstrap, `show-authority` and the probe on the fixture (`0004 AC-10`, `AC-11`, `AC-12`)

On PostgreSQL 18.7 from the pinned digest, second run of the fixture probe:

- The monitor login has LOGIN and no other attribute, is a member of no role and has no member (`t|f|f|f|f|f|0|0` for login, superuser, replication, create role, create database, bypass of row security, roles it is a member of, and members it has). The frontend principal is no role of PostgreSQL. The two servers have different system identifiers.
- `pool bootstrap` exits 0 with its four lines and an authority ID of 32 hexadecimal digits. Every file in the state directory has mode 0600. The journal has `application_id` 1347374924, `user_version` 1, WAL mode, four tables, revision 1, start sequence 0, no hold, and the topology row with the system identifier of the server and timeline 1.
- `show-authority` says `unconfirmed` with the same ID in both formats and changes no byte. With the ID as `control.authority_id` it says `ready`, and with another ID `mismatch`.
- A second bootstrap exits 1 with `bootstrap.store_present`, and with the witness set also `bootstrap.witness_present`. The JSON of a bootstrap is one line with the keys `output_version`, `command`, `result`, `authority_id`, `clusters` and `findings`.
- The server log shows each probe connection as the monitor login over TLS with `application_name=pool-probe`, and one statement with `pg_control_system` per connection. No other login of POOL appears.
- A node that cannot be reached, a wrong CA, a wrong server name and a wrong monitor password each give `bootstrap.evidence_missing`, exit 1, and no journal. The second server, and a wrong configured system identifier, give `bootstrap.identity_mismatch`.
- Without a database (before the pause): `show-authority` says `uninitialized`, `missing` or `unreadable` with exit 0 and `authority_id` `null` in JSON; bootstrap against a closed port is `bootstrap.evidence_missing` with no journal; a configured witness adds `bootstrap.witness_present`; a store that cannot be read gives `bootstrap.store_present`.

#### A bootstrap that stopped (decision 59)

| Case | `show-authority` | `pool bootstrap` on `3b58997` | `pool bootstrap` on `ecd0ee5` |
|---|---|---|---|
| A journal file of one page that holds only the WAL setting (4096 bytes), which is what a bootstrap leaves when it stops after setting the journal mode | `uninitialized` | **exit 3, `state.permissions`** (finding 1) | exit 0, a new authority ID, the journal complete |
| A copy taken while a writer was inside its first transaction, with `-wal` and `-shm` | `uninitialized` | **exit 3, `state.permissions`** | exit 0 |
| The main file alone of a journal that was created | `unconfirmed` | exit 1, `bootstrap.store_present` | the same |
| A created journal cut to one page, cut to two pages, with its header overwritten, with its marks set to zero | `unreadable` | exit 1, `bootstrap.store_present`, no byte changed | the same |

**Finding 1 (fixed).** `create_empty_journal` took only a file of 0 bytes, so the one page file was refused as `state.permissions` before `Journal::create` could apply the rule of decision 59. The library met the decision and the command did not. Commit `b33f155` hands a file that exists to `Journal::create`. The rows for a damaged journal show that this did not open a way to write over a journal that holds something: those cases are refused by `bootstrap_plan` before any file is touched.

#### The follow up of step 6 (decisions 57 and 61)

- `apps/pool/src/commands.rs` calls `read.bootstrap_plan(...)`. A line with `read.plan_bootstrap(` in that file is refused by rule W3 (`forbidden text plan_bootstrap`), and the same line with `bootstrap_plan` is accepted. `StoreProbe`, `StoredAuthority`, `classify_startup` and `writer_admission` are still refused in every crate but `pool-control`. No `.rs` file outside `crates/pool-control` holds the text `plan_bootstrap`.
- Two latch receipts with revisions 2 and 3, handed to the controller in reverse order: the state holds 3 after the first and still 3 after the second, and both clusters leave the list of latches the journal does not hold.

#### The changes to step 7 (decisions 62 to 69)

| Probe | Expected | Observed | Result |
|---|---|---|---|
| `in_hot_standby` reported `off` and then `on` to a session connection (decision 66) | refused as not a primary | `not_primary`; `on` and then `off` passes; the probe connection accepts both, as before | pass |
| Eight first logins at the same time with one new credential (decision 64) | one derivation of the keys | 8 backends opened, 1 derivation | pass |
| A named pipe as the credential file, the CA file and the certificate file (decision 67) | refused without reading it | `Permissions`, `Invalid`, `Invalid`, all three in under a millisecond | pass |
| `derive_keys` and `verifier_text` with a count (decision 68) | the count is a `NonZeroU32`, so 0 cannot be passed | the probe had to change its calls to compile; a count of 0 no longer compiles | pass |
| `Debug`, in both forms, of `ServerExchange`, `ClientExchange`, `ServerCredential` (known and unknown), `CertificateChain`, `TrustRoots`, `PoolKey`, `Node`, `Login`, `Credential` and an open `BackendConnection`, each filled with a canary (decision 69) | no canary, no nonce, no certificate text | none; for example `ServerExchange([REDACTED])` and `CertificateChain { certificates: 1 }` | pass |
| The test key `crates/pool-core/tests/fixtures/node.key` (decision 62) | its first line says what it is | `test key for node.test, used only by the tests of pool-core; it protects nothing` | pass |
| A CA file that exists and cannot be read (decision 63) | `secret.permissions` at its key | `error: secret.permissions at node.tls_ca_file: ...`, exit 3 | pass |

#### Output of the two commands

In 226 outputs of the command probe and 29 of the fixture probe, human and JSON, there is no password, no path, no host, no server name, no login and no database name. The key names of the configuration appear, which the shared conventions allow. Usage errors and `--help` exit 2 and 0 as for `check-config`, and a configuration with findings is reported with the command name before any state is touched.

#### Criteria

| Criterion | What was checked | Verdict |
|---|---|---|
| `0004 AC-9` | The directory and file checks, the secret files, the lock on macOS and Linux | **not met in one case**: the wrong code in the race to create `pool.lock` (finding 2). The rest is met |
| `0004 AC-10` | The journal of a real bootstrap, read with SQLite from outside | met |
| `0004 AC-11` | Bootstrap, second bootstrap, `show-authority` in each state, refusals that write nothing, a stopped bootstrap | met on `ecd0ee5` (finding 1 fixed) |
| `0004 AC-12` | The attributes of the monitor login, the server log of the probe, wrong CA, name and password, the second server | met |
| `0004 AC-14`, library half | The last report of `in_hot_standby` decides | met |
| `0004 AC-24`, in `pool-core` | `Debug` of ten types with a canary | met |
| `0004 AC-26` | The check passes, fails, is stopped and has no Docker; the image is taken by digest | met; the CI job is manual step M2 |
| `0003 AC-17` and `AC-19`, after decisions 57, 59 and 61 | The method name against the guard, the receipts in reverse, the stopped bootstrap in the reader | met |

#### Notes that are not failures

- **Read in the code only.** `random.failed` cannot be made to happen from outside. In `commands.rs` the authority ID comes from `pool_core::random::fill_random` before `create_empty_journal` is called, so a failure leaves no journal file. `pool.lock` exists by then, because the lock is taken first. The check of a secret file on the open file (decision 67) was also read, and the named pipe probe supports it.
- **SIGTERM to `run.sh` alone.** The shell runs its trap only after the test command returns. With a stand in test command that slept 60 seconds, the containers were removed 60 seconds after the signal. A signal to the whole process group, as from a terminal or a cancelled CI job, ends the test command too.
- **Other runs on this machine.** While this session ran, other sessions ran the same check, and their `pool-it-` containers were visible. This session judged "nothing left" by the process ID in the container names of its own runs, and touched no other container.
- **A wrong CA or name toward the node** shows at the command as `bootstrap.evidence_missing`. The class of the probe failure is not in the output of `bootstrap`, by the rule that the output names no deployment detail.

#### For the tester

No mutation run was done for the code of step 8 in this item. Test gaps seen:

- Before `b33f155` no test put a one page journal file in front of `pool bootstrap`. `it_a_bootstrap_that_stopped_after_the_wal_mode_does_not_block_the_next` now does. A hermetic twin in `tests/commands.rs` is not possible while bootstrap needs a node that answers.
- No test starts several commands together on a state directory that has no `pool.lock` yet (finding 2).
- No test puts a journal that was cut or whose marks were changed in front of `pool bootstrap`. The tests use a file of plain text for the state `unreadable`, and only with `show-authority` and `pool run`. The probe compared the files before and after.
- The lock test holds the lock inside the test process. A helper process that holds it would test the product the way two daemons meet it.

### Checker run of the fix for finding 2 (the race to create `pool.lock`)

On 2026-11-15 at 21:50 WIB, by the checker session, after the developer of step 8 reported the fix. Commit `fd62cf4` (`fix(pool): spec 0004 step 8, a lock file made by another process at the same moment is state.locked`). The Linux probe ran on `fd62cf4`, the workstation runs on `8242a1a` in a private clone, and the two checks at the end on `aa10007`, the tip of `main`, which holds the same `apps/pool/src/state.rs` and `apps/pool/tests/commands.rs`. The probe is a new program of the checker, outside the repository, with the standard library only. It starts eight commands from eight threads that wait on one barrier, and it has its own listener that accepts and never answers, so a bootstrap holds the lock for 0.6 seconds. In the pinned build image it ran as a user that is not root, against a release `pool` built there.

**Verdict: the fix PASSES, and with it milestone 3 PASSES.** `0004 AC-9` is met: a command that loses the race gets `state.locked`. No other finding of milestone 3 is open.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| Linux, `fd62cf4`: 40 rounds of eight bootstraps on a new state directory with no `pool.lock`, 320 commands | one holder in each round, `state.locked` for the seven others | 40 holders, 280 times `state.locked`, 0 rounds with two holders, 0 with none, 0 other results; `show-authority` right after each round exits 0 in 40 of 40 | pass |
| The same on a state directory where `pool.lock` exists | the same | 40 holders, 280 times `state.locked`, 0 other results | pass |
| Linux, `fd62cf4`: 200 rounds of eight `show-authority` on a new state directory, 1600 commands | exit 0 or `state.locked`, nothing else | 1533 times exit 0, 67 times `state.locked`, 0 other results; after each round the directory holds `pool.lock` with mode 0600 and nothing else | pass |
| Control, the same probe on `ecd0ee5` (before the fix) | the probe shows the fault | 34 of 320 bootstraps and 126 of 1600 `show-authority` ended with `state.permissions`; with `pool.lock` there, 0 | the fault, as found |
| What must stay `state.permissions`: `pool.lock` with mode 0644, 0400 or 0660, as a directory, as a link to a good file, as a link to nothing, as a named pipe, each with `bootstrap` and `show-authority` (14 runs on Linux, 14 on macOS) | exit 3 with `state.permissions`, no hang, nothing changed, the target of the link not made | all 28 as expected | pass |
| macOS, `8242a1a`: the same probe with 30 rounds | as on Linux | 30 holders and 210 times `state.locked` in both forms; 964 times exit 0 and 236 times `state.locked` of 1200 `show-authority`; 0 other results | pass |
| macOS, `8242a1a`: the command probe of the first run, 100 checks | 0 mismatches | 0 mismatches | pass |
| `scripts/check.sh all` on `8242a1a` | exit 0 | exit 0, all six `ok`, with `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked ... ok` | pass |
| `scripts/check.sh integration` on `aa10007` | exit 0, nothing left behind | exit 0, 21 of 21; 89 volumes before and after, no container of this run, no temporary directory | pass |
| `scripts/check.sh linux-build` on `aa10007` | exit 0 | exit 0, 22 test results `ok`, none failed, with the new test and the lock test `ok`, `linux build check passed` | pass |

**What I read in the fix.** When `create_new` says the file exists, the command opens the file the other command made and lets `try_lock` decide. The file is then checked on the open descriptor: a regular file, mode 0600, the owner of the directory, and the same device and inode as the path read without following a link. A link at the path is still refused: the path is read without following a link before the open and again after it, and a link to nothing ends in `state.permissions` because `create_new` does not follow it. `create_protected` now sets the mode on the open file and not on the path.

**For the tester.** The new test accepts exit 0 or `state.locked` for eight `show-authority`. It has no bootstrap case, where one command holds the lock for a while, and it does not look at the files afterwards. The probe did both.

### Checker run of the lock file fix on `main`, with links, owners and the umask

On 2026-11-15 at 21:53 WIB, by the checker session, on the tree `d4fdb62` (the tip of `main`; `apps/pool/src/state.rs` is the one of `fd62cf4`). All of it ran in the pinned build image, against a release `pool` built there, with the probe of the run above and three new parts. Parts A to E ran as a user that is not root, part F as root.

**Verdict: `0004 AC-9` is met, and milestone 3 PASSES.**

| Probe | Expected | Observed | Result |
|---|---|---|---|
| A. 60 rounds of eight bootstraps on a new state directory with no `pool.lock`, 480 commands | one holder in each round, `state.locked` for the others, no `state.permissions` | 60 holders, 420 times `state.locked`, 0 rounds with two holders, 0 with none, 0 other results; the same with `pool.lock` there | pass |
| B. 300 rounds of eight `show-authority` on a new state directory, 2400 commands | exit 0 or `state.locked` | 2162 times exit 0, 238 times `state.locked`, 0 other results | pass |
| C. `pool.lock` with mode 0644, 0400 or 0660, as a directory, as a link to a good file, as a link to nothing, as a named pipe, with both commands | `state.permissions`, nothing changed | 14 of 14 | pass |
| D. A link at `pool.lock` that points at a file on which the probe holds an exclusive lock. First still, then 3000 `show-authority`, one at a time, while a thread of the probe swapped `pool.lock` 360822 times between a good file and that link (with `rename`, so the swap falls before the open, between the open and the check, and after it) | `state.permissions` or exit 0, and never `state.locked`, which would mean that a command followed the link as far as the lock | still: `state.permissions`. In the swap: 735 times exit 0, 2265 times `state.permissions`, 0 times `state.locked`, 0 other results | pass |
| E. The umask: 60 rounds of eight `show-authority` on a new state directory under each of the umasks 0000, 0022, 0077 and 0177, 1920 commands | exit 0 or `state.locked`; `pool.lock` has mode 0600 afterwards | 0 times `state.permissions`; mode 0600 in 240 of 240 directories | pass |
| E. The same under a umask that takes a permission away from the owner: 0200, 0400, 0600 and 0777, 1920 commands | see the reading below | 43 commands got `state.permissions` (10, 4, 16 and 13); mode 0600 in 240 of 240 directories afterwards; one command alone exits 0 in 240 of 240 | a note, see below |
| F. As root, with a state directory of root: `pool.lock` with mode 0600 owned by another user, with both commands; then each of the three journal files owned by another user | `state.permissions` | `state.permissions` in all five runs; exit 0 before and after, when every file is owned by root | pass |

**The edge with the umask, and why it is a note.** The file is created with mode 0600 less the umask, and the mode is set to 0600 on the open file right after. A command that looks at the file between those two steps sees the mode before the second step. With a umask that leaves the owner bits alone, which is every umask a service runs with, that mode is already 0600, and the probe shows no wrong answer in 1920 commands. Only a umask that takes read or write away from the owner makes the file exist for a moment with another mode, and then a command that starts in that moment says `state.permissions`. I read this as within `0004 AC-9` as it is written: the criterion gives `state.permissions` for a journal file whose mode is not 0600, and in that moment the mode is not 0600. It fails closed, nothing is written, the file ends with mode 0600, and the next start passes. A sentence in the spec that names this case would be enough. The developer named this edge in the report of the fix.

**Links.** The path is read without following a link before the open (`check_protected`) and again after it (`check_opened`, which also compares device and inode with the open file). Part D could not get a command past both.

## Milestone 4: The session path

Build plan steps 9 to 11. Each step keeps its own part of this section.

### Build plan step 9: the session

Build plan step 9, on 2026-11-15, on macOS (workstation) and in the Linux builder image. Commit `05c76fa`, on top of `main` at `34e9054`. Tools: `rustc 1.100.0`. Every test is hermetic: `crates/pool-core/tests/session.rs` runs the whole session over `tokio::io::duplex` streams, with a scripted client (the encoders of `pool-protocol` and the SCRAM client of `pool-core`) and a scripted node behind a test `Connector`. No socket, no container, no PostgreSQL; the real fixture is steps 10 and 11.

#### UI / manual

None for step 9.

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-core --locked` | exit 0 | exit 0: 29 unit tests and 27 in `tests/session.rs` passed, 0 failed, in 0.4 s | pass |
| `cargo clippy -p pool-core --all-targets --locked -- -D warnings` | exit 0 | exit 0 | pass |
| `scripts/check.sh all` before the commit | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`; rule W3 accepts the runtime in `crates/pool-core` and finds no thread | pass |
| `scripts/check.sh linux-build` at `05c76fa` | exit 0 | exit 0; the 29 unit and 27 session tests also passed in the image | pass |

#### Acceptance-criteria coverage

| Criterion | Tests in `tests/session.rs` | Result |
|---|---|---|
| `0004 AC-14`, hermetic half | `a_closed_or_stale_route_refuses_with_57p03_and_opens_no_backend` (closed and stale: `57P03`, no backend opened), `a_route_that_changed_while_the_backend_opened_refuses_at_step_9` (the epoch moved during step 8: `57P03`, the backend closed, no query reached it), `a_node_in_recovery_gives_08006` (`in_hot_standby` `on`), `a_positive_conflict_ends_running_sessions_and_silence_does_not` (closed without a conflict: the running session goes on; with a conflict: `57P01` idle, `08007` with a query in flight) | pass for the hermetic half; the fixture half is steps 10 and 11 |
| `0004 AC-15`, hermetic half | `startup_without_tls_is_refused_with_28000_and_no_authentication_request`, `gssenc_is_answered_with_n_and_a_version_3_2_gets_negotiation`, `refused_versions_and_parameters` (major 2 and 4, `replication`, `options`: `0A000`), `a_cancel_request_is_read_and_dropped` (no answer, no backend), `a_wrong_node_certificate_gives_08006` (no startup packet toward the node) | pass for the hermetic half |
| `0004 AC-16`, hermetic half | the frontend offers exactly `SCRAM-SHA-256` (checked in every login); `a_wrong_password_and_an_unknown_name_fail_after_the_same_sequence` (both `R`, `R`, `E` with `28P01`); `an_authentication_message_over_1024_bytes_is_refused` (a body of 1024 bytes is read, 1025 gives `08P01`) | pass for the hermetic half |
| `0004 AC-17`, hermetic half | `a_database_without_an_identity_entry_gets_28000_and_no_backend`; `a_mapped_principal_gets_a_session_on_its_backend_login` (the node saw the backend login and database of the entry, never the principal) | pass for the hermetic half |
| `0004 AC-19`, hermetic half | `a_client_that_leaves_inside_a_transaction_closes_its_backend_and_returns_its_permit` (with `session_max_count = 1` the next login succeeds); the clean `Terminate` in `a_mapped_principal_gets_a_session_on_its_backend_login` | pass for the hermetic half |
| `0004 AC-20`, hermetic half | `a_query_over_the_message_limit_gets_54000_and_never_reaches_the_node`, `a_startup_packet_over_its_limit_is_refused`, `the_session_limit_gives_53300`, `waiters_and_the_acquire_deadline` (one waiter more: `53300` at once; a waiter: `53300` after 300 ms, within 2 s), `a_client_that_sends_nothing_is_closed_at_the_startup_deadline`, `the_handshake_limit_closes_a_new_socket_at_once`; `a_large_row_streams_through_the_relay_buffer` (a 200000 byte row through the 65536 byte buffer) | pass for the hermetic half |
| `0004 AC-22`, hermetic half | `a_lost_commit_answer_gives_08007_and_no_second_attempt` (one backend opened, the COMMIT sent once) | pass for the hermetic half |
| `0004 AC-23`, hermetic half | `unsupported_and_broken_frontend_messages_end_the_session` (`P`, `F`, `d`: `0A000`; an unknown tag and a length below 4: `08P01`; nothing reaches the node, the backend is closed), `a_copy_response_and_a_broken_backend_message_end_the_session`, the greeting `S S S Z` without `BackendKeyData`; unit tests `the_pool_key_has_eight_parts_and_hides_the_database_and_the_login` and `lease_generations_start_at_1_and_never_wrap` | pass for the hermetic half |
| `0004 AC-25`, hermetic half | `shutdown_ends_idle_sessions_with_57p01_and_waits_for_the_others` (idle: `57P01`; in a transaction: goes on until idle, then `57P01`; no accept after), `at_the_shutdown_deadline_a_query_in_flight_gets_08007_never_57p01`, `shutdown_closes_connections_before_their_session_started` | pass for the hermetic half; signals and the exit status are step 10 |

**Readings.**
- Since `cc77c34` the session reads the first packets with one `FirstPacketReader` before and after TLS. The reader decides on the eight header bytes, so a startup packet without TLS gets `28000` before its parameters are read, and a body is read only for `CancelRequest` and, over TLS, for `StartupMessage`. `scripts/check.sh all` and `scripts/check.sh linux-build` at `cc77c34`: exit 0.
- Each message of the node from SCRAM up to `ReadyForQuery` has a body of at most 16384 bytes (`BACKEND_STARTUP_MESSAGE_MAX_BYTES`); the one row query of the probe keeps 65536.
- `Terminate` with a length other than 4 ends the session with `08P01` (`unsupported_and_broken_frontend_messages_end_the_session`), from the change in `pool-protocol` of `87a068a`.
- Refusals and backend failures are counted in `Counters` and written by `Counters::flush`, which the daemon calls once a second (step 10).

**Also in this item, from the spec text of 2026-11-15.** Commit `ca122c2` adds `crates/pool-core/src/random.rs` with `fill_random` and `RandomError` (unit test `two_fills_differ_and_the_error_prints_its_name`), makes it the one source of random bytes of the crate (`MockKey::generate` returns `RandomError`, for `random.failed`), ends a login without a message under `random_failed` when no nonce can be made, and stops the one row query of the probe at 4096 bytes of answer (`a_probe_answer_over_4096_bytes_is_not_read_on`). The limit of 16384 bytes per node message during startup has its test too (`a_startup_message_of_the_node_over_16384_bytes_ends_the_attempt`). `cargo test -p pool-core --locked`: 32 unit and 27 session tests passed. `scripts/check.sh all` and `scripts/check.sh linux-build` at `ca122c2`: exit 0.

#### Checker run

Build plan step 9 only, on 2026-11-15 at 19:03 WIB, by the checker session, on the merged tree `8783337` (macOS 27.0.1, arm64, with Docker 29.9.2 running; the Linux build ran in the pinned image, the rest on the workstation, and none of it is CI). Tools: `rustc 1.100.0`. Every command below was run in this session. The probes are a small program outside the repository that builds a `Server` through the public functions of `pool-core`, with a scripted client, a scripted node behind a connector of its own, in memory streams and a `tracing` subscriber that keeps every event. The canary is in the principal name, its password, the requested database, the backend login, the backend database, the backend password, the host, every path and the SQL text.

**Verdict: step 9 FAIL, for one finding.** A session whose client has stopped reading is ended neither by the shutdown deadline nor by a positive conflict, and `Server::shutdown` then never returns. Everything else that was probed is as the spec says. Three changes to `crates/pool-protocol` are known to be still in work and were not judged: `Terminate` with a length other than 4 (decision 52), the user name of a SCRAM first message (decision 53) and `FirstPacketReader` with its `Handshake` (decision 54).

**The finding.** Sent to the developer of step 9 at the moment it was found.

- **How to see it.** A client logs in, sends one `Query` that the node answers with 8 MB of data rows, and reads nothing more. After 500 ms the relay waits to write to that client. Then either `Server::shutdown` is called with `limits.shutdown_timeout_ms = 300`, or a route view with `Closed { conflict: true }` is published.
- **Observed.** Shutdown: `shutdown` had not returned after 4 seconds; the phase was `Ending`, one connection and one session were left, and the backend was still open (1 opened, 0 closed). Conflict: 1.5 seconds later the session was still there and the backend still open. The same in each of seven runs.
- **Expected.** [Shutdown](index.md#shutdown), step 4: when the deadline has passed, every session that is left is ended, with no message when it is not at a message boundary. [When a running session is ended](index.md#when-a-running-session-is-ended): a positive conflict ends every running session of the cluster and closes its backend. `0004 AC-25` (the daemon exits) and the ending rule of `0004 AC-14`.
- **Where.** `copy_exact` in `crates/pool-core/src/session.rs` looks for a conflict or the end of shutdown between two pieces, and while it waits to read. Its write to the destination waits alone. Once the send buffer of the TLS connection and the stream to the client are full, the task stays in that write. The other writes of the relay toward the client (a message header, the flush, an error of POOL) wait the same way.
- **What it touches.** A client that reads slowly is still ended at the next piece. The case of `0004 AC-21`, sessions that asked for many rows and then stopped reading, is exactly this one, so a daemon in that state does not stop on SIGTERM.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-core --locked` | exit 0 | exit 0: 32 unit tests and 27 in `tests/session.rs` passed, 0 failed | pass |
| `scripts/check.sh all` | exit 0 | exit 0: `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` all `ok` | pass |
| `scripts/check.sh linux-build` on the committed tree | exit 0 | exit 0: `host: aarch64-unknown-linux-gnu`, 19 test results `ok` and none failed, among them the 32 unit tests and the 27 session tests of `pool-core`, `linux build check passed` | pass |
| Probe, the texts of rule W3, `unsafe`, `unwrap`, `expect`, `panic!` and a thread in `crates/pool-core/src` outside the tests | none that the baseline forbids | only `tokio`, `TcpStream` and `TcpListener`. No `unsafe`, no `unwrap()`, no `std::thread`. One `expect`, in `derive_keys`, as before. PBKDF2 runs in `spawn_blocking` | pass |
| Probe, a startup packet without TLS: only its eight header bytes are sent, claiming a body of 92 bytes that never comes | `28000` at once, no authentication request | `28000`, `TLS is required`, within 300 ms, then the stream is closed. So the body is not read. Counted as `plaintext` | pass |
| Probe, the other first packets | the rows under [Startup](index.md#startup) | `GSSENCRequest`: `N`, then `SSLRequest`: `S`. Over TLS: protocol 3.2 with a `_pq_.` name gets `NegotiateProtocolVersion` first, then `SCRAM-SHA-256` alone; major 2, major 4, `replication` and `options`: `0A000`; no `user`, and a second `SSLRequest`: `08P01`; a packet of 513 bytes with a limit of 512: `54000`; none of them gets an authentication request. A `CancelRequest` over TLS: no answer and no effect on the node | pass |
| Probe, the startup deadline, `limits.startup_timeout_ms = 400` | closed at the deadline, with no message | a client that sends nothing: closed after 403 ms with no byte. A client that stops after the SASL request: closed 402 ms after the accept, not after the request, with no message. Both counted as `startup_timeout` | pass |
| Probe, authentication | the same messages for a wrong password and an unknown name | the right password: SASL, continue, final, `AuthenticationOk`, two parameters, `ReadyForQuery`. A wrong password and a name that is no principal: the same three messages and the same error, `28P01`. A SASL message of 1025 bytes of body: `08P01`. The mechanism names `SCRAM-SHA-256-PLUS`, `PLAIN` and `scram-sha-256`, each with a well formed first message: `08P01` and no server-first message | pass |
| Probe, the identity map | `28000` and no backend; the backend login of the entry | a database without an entry: `28000`, `no identity entry for this user and database`, and the node saw no connection. A mapped login: the node is logged in to as the backend login and backend database of the entry, with `application_name` and `client_encoding`, and never sees the name or the database the client sent | pass |
| Probe, the first route check | `57P03` and no backend | a route that is closed, closed with a conflict, open and stale, or missing from the view: `57P03`, no backend opened, no `AuthenticationOk` | pass |
| Probe, the second route check: the view is replaced at the moment the backend has opened | `57P03`, the backend closed, no byte of SQL | the route closes, names another node, has another topology epoch, has another authority revision, or is stale: `57P03` with no `AuthenticationOk`; the backend that was opened is closed again and got no byte after its login. The same route published again, and one that only has a later `fresh_until`: the session opens | pass |
| Probe, no byte before a query | nothing reaches the node after its login until the client sends a `Query` | 0 bytes after the login of the node, then the `Query` | pass |
| Probe, a backend that cannot be opened | `08006` | no connection: `08006`, `the backend connection could not be established`. A node that reports `in_hot_standby` as `on`: `08006` and no session. A backend that does not open before the acquire deadline: `08006` | pass |
| Probe, the limits | [Limits](index.md#limits) | one session more than `limits.session_max_count = 1`: `53300` and no second backend. The second session of a principal with `session_max_count = 1`: `53300`, while another principal logs in. One handshake more than `limits.handshake_max_count = 1`: the stream is dropped at once. With one backend per node and one waiter: a third login gets `53300` at once, the waiter gets `53300` after 621 ms with a deadline of 600 ms, and a waiter gets the permit when the first session ends. A normal login succeeds after each case | pass |
| Probe, the message limit, `limits.frontend_message_max_bytes = 1024` | `54000` and no byte of the query at the node | a `Query` whose length field is 1025: `54000`; the node got 5 bytes, a `Terminate` of POOL, and no query. A length field of exactly 1024 is relayed | pass |
| Probe, the permits come back | the next login succeeds | a client that leaves inside a transaction: the backend gets `Terminate` and is closed, no session is left, and with `limits.session_max_count = 1` the next login succeeds. The same after a `Terminate` of the client | pass |
| Probe, the greeting and the relay | [Messages after login](index.md#messages-after-login) | the greeting is `AuthenticationOk`, the parameters of the node and `ReadyForQuery`; the `BackendKeyData` of the node is in no message a client received in the whole run. `Parse`, `Sync`, `CopyData` and a function call: `0A000`; an unknown tag and a length of 3: `08P01`; a COPY response of the node: `0A000`. Each time the backend is closed and `session.ended` names the reason. An answer of 3 MB in 375 rows passes through | pass |
| Probe, backend loss | `08007` with a query in flight, `08006` without, nothing in the middle of a message, never a second attempt | the node closes after it took the query (a lost COMMIT answer): `08007`, the query reached the node once, no new backend was opened. A `ReadyForQuery` of length 6: `08007`. The node closes in the middle of a data row: no error, the connection is closed. The node closes while no query is in flight: `08006`, `the backend connection was lost`. `session.ended` says `in_flight` for the first two | pass |
| Probe, a positive conflict and silence, with a client that reads | [When a running session is ended](index.md#when-a-running-session-is-ended) | the cluster closes without a conflict: the running session goes on and a new login gets `57P03`. A conflict with no query in flight: `57P01`, and both sessions that were running on the cluster are ended and their backends closed. A conflict with a query in flight: `08007`, never `57P01`. A conflict while a message of the node is half written to the client: the connection is closed with no error message | pass |
| Probe, shutdown with clients that read, `limits.shutdown_timeout_ms = 500` | [Shutdown](index.md#shutdown), steps 1 to 4 | an idle session: `57P01` at once. A connection that had not started its session: closed with no byte. A new connection: not accepted. A session inside a transaction: `57P01` at the deadline. A session with a query in flight: `08007` at the deadline, never `57P01`. The end of shutdown while a message is half written: no error message. `shutdown` returns after 504 ms with no connection left. With a deadline of 5 seconds, a session that commits during the drain gets its answer, then `57P01`, and `shutdown` returns long before the deadline | pass |
| Probe, shutdown and a conflict with a client that does not read | the session is ended | not ended: see the finding above | **fail** |
| Probe, the log events of the whole run, 85 lines | only the fields of [Log events](index.md#log-events), and none of the canary | `session.started` carries its seven fields and `session.ended` its four; every line is one of `session.started`, `session.ended`, `session.refused`, `backend.failed`, with no other field. A session ID is 32 hex characters that start with the start sequence. Within one server the lease generation rises by one with each lease (1 to 16). `flush` writes `session.refused` per reason and `backend.failed` per node and class. No line holds the principal name, a login, a database name, a host, a path, a password or SQL | pass |
| Probe, 47 changes to the code, one at a time in a private clone, each followed by `cargo test -p pool-core -p pool-protocol --locked` | every change to a rule is caught by a test | 31 caught and 16 not. Caught: a second check that never refuses, the swap of `08007` and `57P01`, a lost backend that always gives `08006`, the in flight flag in both directions, session permits that are not given back, each limit raised by one except the one of a principal, the message, startup and authentication limits, and the deadlines. The 16 that no test caught are under the table | see below |

| Criterion | Checker verdict |
|---|---|
| `0004 AC-14`, hermetic half | **not met** for one case: a running session whose client does not read is not ended when its cluster closes with a conflict. Met for the rest: both route checks, no connection for a closed or stale route, a running session ended by a conflict and kept through silence |
| `0004 AC-15`, hermetic half | met on the listener side |
| `0004 AC-16`, hermetic half | met |
| `0004 AC-17`, hermetic half | met |
| `0004 AC-19`, hermetic half | met: the backend is closed and the permits come back after a disconnect inside a transaction and after `Terminate` |
| `0004 AC-20`, hermetic half | met |
| `0004 AC-22`, hermetic half | met: one `08007`, no second attempt, no claim of a rollback |
| `0004 AC-23`, hermetic half | met, with the reading of `Terminate` that decision 52 changes |
| `0004 AC-25`, hermetic half | **not met** for one case: with a client that does not read, the sessions are not all ended at `limits.shutdown_timeout_ms` and `shutdown` does not return. Met for clients that read |

The changes that no test caught, for the tester. A probe above covers each rule except where it says so:

- **The second route check has one test, for a route that closes.** Leaving out the comparison of the node, of the topology epoch or of the authority revision, or the freshness of the second look, passes every test. The probe covers all four. Leaving out the pool key changes nothing that can be seen, because the node and the epoch are compared beside it.
- **Endings in the middle of a message.** A conflict that does not cut a message, and an error sent after a cut or after a backend lost in the middle of a message, pass every test. The probe covers all three.
- **A lease that is never dropped** (the backend stays open) passes, because the tests count closed backends only where the node closes first or gets a `Terminate`. The probe counts opened and closed backends after a conflict and after shutdown.
- **The limit of a principal** raised by one, **any SASL mechanism name** accepted, and **the same lease generation for every lease** pass. The probe covers them.
- **`session.started` with the name of the principal** in the place of its ID passes: no test reads a log event. The probe does.
- **Connections before their session at shutdown** are closed by the startup deadline in the test, so a shutdown that does not close them passes. The probe sees them closed within 400 ms, with a startup deadline of 5 seconds.
- **The limit of 16384 bytes** for a message of a node during startup, and **a probe answer that is not counted across messages**, pass. The probe of the additions to step 7 covers both.

Notes that are not failures:

- `Terminate` is sent on with whatever body its length field claims. Decision 52 changes that in `pool-protocol`, and the session follows what `classify_frontend` gives.
- `random_failed` was read in the code and not seen.
- The lease generation is taken before the greeting is written, so a greeting that fails uses up a generation. Nothing depends on a gap.
- Once in five runs the probe read the count of closed backends in the same instant in which `shutdown` returned and saw one fewer than opened. The node of the probe notices the close a moment later, and the read did not wait. In the other runs, and wherever the probe waits 80 ms first, the counts are equal.
- CI evidence does not exist yet. It waits for the pull request.

#### Fix: a client that stops reading (from the verification of step 9)

The verification of step 9 found that a relay waiting on a write toward a client that no longer read was never ended by the shutdown deadline or by a positive conflict (`0004 AC-25`, `0004 AC-14`). Commit `da51f43` races every write of the relay, toward the client and toward the backend, against the cut: a write that fits still completes, a write that waits is abandoned when the cut comes, and mid message the connection is closed without a message. The close of the client stream at the end of a session is bounded by one second (`CLOSE_TIMEOUT`).

| Test in `tests/session.rs` | What it holds | Old code | Now |
|---|---|---|---|
| `the_shutdown_deadline_ends_a_session_whose_client_stopped_reading` | A client asks for 8 MB of rows and reads nothing more; with `shutdown_timeout_ms = 300`, `shutdown()` returns within 5 s, after the deadline, with no session and no connection left, and the backend closed | failed | pass |
| `a_positive_conflict_ends_a_session_whose_client_stopped_reading` | The same client; a positive conflict ends the session within 3 s and closes the backend | failed | pass |

`cargo test -p pool-core --locked`: 37 unit and 29 session tests passed. `scripts/check.sh all`, `scripts/check.sh linux-build` and `scripts/check.sh integration` (6 of 6) at `da51f43`: exit 0.

#### Checker run of the fix

The fix `da51f43` for the finding of the checker run above, on 2026-11-15 at 19:09 WIB, by the checker session, on the merged tree `5b11ca3`, which holds it. The same probe program was built against this tree and run again, whole. No mutation was run again.

**Verdict: the finding is closed.** With it, the probe finds nothing against the hermetic halves of `0004 AC-14` and `0004 AC-25`.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| Probe, shutdown with `limits.shutdown_timeout_ms = 300` and one client that stopped reading in the middle of 8 MB of rows | `shutdown` returns after the deadline, nothing is left | `shutdown` returns after 1.31 s, which is the deadline and the one second the close of the client stream may take; no connection, no session, the backend closed (1 opened, 1 closed) | pass |
| Probe, a positive conflict with the same client | the session is ended and the backend closed | 1.5 s later: no session, the backend closed | pass |
| Probe, the rest of the program on this tree | as in the checker run above | 101 checks. The two that do not match are mistakes of the probe, the same as before: one expects one closed backend where two sessions were running on the cluster and both were ended, and one compares a lease generation in decimal with a session counter in hex. The rows of the table above state what was seen | pass |

#### Fix: a node that stops in the middle of a message (review finding c1)

The independent review of scope 004 (`docs/reviews/2026-11-16-scope-004-one-real-primary-path.md`, finding c1) showed that the read of the status byte of `ReadyForQuery` was the one wait of the relay that was not raced against the cut: a node that sent the five header bytes and stopped kept its session through a positive conflict (`0004 AC-14`), and `Server::shutdown` did not return (`0004 AC-25`). The fix closes the class. Every wait of a session after login was traced, in `crates/pool-core/src/session.rs`:

| Wait | Before | Now |
|---|---|---|
| The greeting toward the client (`AuthenticationOk`, the parameters, `ReadyForQuery`) | not raced | raced against the cut; on a cut the greeting may be partial, so nothing more is sent |
| The relay loop: a header from the client or from the backend, and a change of the route view or the phase | raced | unchanged |
| The body of a message, read and written through the relay buffer (`copy_exact`) | raced | unchanged |
| A header toward the client, a flush, a write toward the backend (`write_client`, `write_backend`) | raced since `da51f43` | unchanged |
| The status byte of `ReadyForQuery` | **not raced** | raced; the header is not forwarded yet, so a cut there is at a message boundary and sends `08007` with a query in flight, `57P01` without |
| A generated error and its close (`send`) | raced | unchanged |
| The close of the client stream at the end | bounded by `CLOSE_TIMEOUT` (1 s) | unchanged |
| The wait of `shutdown` after `Phase::Ending` | no limit | bounded by `END_TIMEOUT` (2 s); after it every connection task left is dropped, which drops its lease and closes its backend. With the rows above no session reaches it; it is the last limit against a defect |

The waits before login (the first packets, TLS, SCRAM, step 7, step 8 and a refusal) are not raced against a conflict, because no session runs yet; they end when shutdown starts (`connection` races `admit` against the phase). They are not changed. Out of this fix and in `apps/pool`: a signal handler that cannot be installed (finding c1, `apps/pool/src/daemon.rs:234`). A peer that is gone without closing (M1) is decided separately.

| Test in `tests/session.rs` | What it holds | Old code | Now |
|---|---|---|---|
| `c1_a_positive_conflict_ends_a_session_whose_node_stopped_before_the_status_byte` | The node sends `CommandComplete` and the five header bytes of `ReadyForQuery`, then nothing. After a positive conflict the client gets the `CommandComplete`, then `08007`, then the end; no session, the backend closed, one backend opened | failed: no error, the session stayed | pass |
| `c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_before_the_status_byte` | The same node, `shutdown_timeout_ms = 200`: `shutdown()` returns within 5 s, then the same end | failed: no error | pass |
| `c1_a_positive_conflict_ends_a_session_whose_node_stopped_in_a_header` | The node stops after three header bytes of a `DataRow`: the client gets the `RowDescription`, then `08007` | pass (the header read was raced) | pass |
| `c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_in_a_header` | The same through the shutdown deadline | pass | pass |
| `c1_a_positive_conflict_ends_a_session_whose_node_stopped_in_a_body` | The node stops in the body of a `DataRow`: the client gets exactly the bytes of the node and no error, the backend is closed | pass (the body read was raced) | pass |
| `c1_the_shutdown_deadline_ends_a_session_whose_node_stopped_in_a_body` | The same through the shutdown deadline | pass | pass |

"Old code" is the fixed tree with only the status byte read put back to an unguarded `read_exact`. Every one of the six tests runs inside its own limit of 15 s, and each wait inside it has a shorter one, so a regression fails with a test name instead of hanging the run (finding t4). The drop after `END_TIMEOUT` has no test: no wait of a session reaches it any more.

`cargo test -p pool-core --locked`: 50 unit and 53 session tests passed. `scripts/check.sh all`, `scripts/check.sh integration` (26 of 26) and `scripts/check.sh linux-build` (on the committed tree of this fix): exit 0, each on its first run.

### Build plan step 10: the daemon

On 2026-11-15, on macOS (workstation, Docker Desktop 29.9.2, not Linux and not CI). Commit `fef8023` (step 10), merged with `main` at `b8818f9` in `77c1817`, which holds the protocol fixes of step 9. Image `postgres:18.7-bookworm` by the digest of step 8; PostgreSQL 18.7 and `psql` 18.7 (Debian 18.7-1.pgdg12+2).

#### UI / manual

- [ ] The CI job `integration` on the hosted runner → `NOT_RUN`, owner: user (manual step M2).

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool --locked` on `77c1817` | exit 0 | exit 0: 13 unit, 84 in `tests/check_config.rs`, 12 in `tests/commands.rs`, 8 ignored in `tests/integration.rs`, 0 failed | pass |
| `scripts/check.sh integration` on `77c1817` | exit 0, nothing left behind | exit 0, 8 of 8, among them `it_the_walking_fixture_gives_the_same_answers_through_pool`; no `pool-it-` container, no new volume or network, no temporary directory | pass |
| `cargo build --workspace --locked` | no dependency warning | exit 0, no warning at all | pass |
| The use check of [Adopted dependencies](index.md#adopted-dependencies) | no line | no line: `tracing` of `apps/pool` is named by `tracing::subscriber`, `tracing::field` and the event macros of `src/log.rs`, `src/probe.rs`, `src/control.rs` and `src/daemon.rs`, all in code | pass |
| `scripts/check.sh all` | exit 0 | exit 0, all six `ok` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0, `linux build check passed` | pass |

The log of the walking fixture, as the daemon wrote it on that run (the authority ID and the times are of that run):

```text
{"ts":"2026-11-15T11:39:20.296Z","level":"info","event":"daemon.started","start_sequence":1,"configuration_generation":1,"authority_id":"9497d54313baf4069d186bf77cbcd122"}
{"ts":"2026-11-15T11:39:20.299Z","level":"info","event":"admission.changed","cluster_id":"main","state":"closed","reasons":"evidence.writer_missing","snapshot_generation":1}
{"ts":"2026-11-15T11:39:20.381Z","level":"info","event":"admission.changed","cluster_id":"main","state":"open","node_id":"pg1","snapshot_generation":2}
{"ts":"2026-11-15T11:39:20.834Z","level":"info","event":"session.started","session":"00000000000000010000000000000001","principal_id":"it-principal","identity_id":"it-identity","cluster_id":"main","node_id":"pg1","lease_generation":1,"topology_epoch":1}
{"ts":"2026-11-15T11:39:20.849Z","level":"info","event":"session.ended","session":"00000000000000010000000000000001","reason":"client_terminate","status":"idle","duration_ms":15}
{"ts":"2026-11-15T11:39:21.034Z","level":"info","event":"daemon.stopping","sessions":0}
{"ts":"2026-11-15T11:39:21.034Z","level":"info","event":"daemon.stopped"}
```

#### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-13` | `prepare` in `apps/pool/src/daemon.rs` classifies once and returns before reading a file or binding; `ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds` (uninitialized, an empty journal file, missing, unreadable; the port is free after each) and `ac13_run_reports_the_configuration_the_view_and_the_state_directory_first` in `tests/commands.rs`; `it_run_refuses_a_journal_without_or_with_another_witness_and_never_binds` (unconfirmed, mismatch; the journal bytes unchanged) in `tests/integration.rs` | pass on the workstation |
| `0004 AC-18` | `it_the_walking_fixture_gives_the_same_answers_through_pool`: the same SQL file (TRUNCATE, a transaction of three inserts, a SELECT in key order, a division by zero) through `psql -X` of the image, once straight to PostgreSQL as `pz7_rw` on `pz7_appdb` and once through POOL as `pz7_principal` on `pz7_db`; exit status, stdout and stderr are equal byte for byte | pass on the workstation |
| `0004 AC-19` | the same test: the PostgreSQL log shows `connection authorized: user=pz7_rw database=pz7_appdb` for the session through POOL, no role `pz7_principal` exists, and `pg_stat_activity` has no `pz7_rw` session within 2 seconds after the client left (asked every 50 ms); the log has `session.ended` with `client_terminate` | pass on the workstation |
| `0004 AC-24` | the same test reads the whole log of the run: every line is one JSON object with `ts`, `level` and `event` first, no `log.unnamed`, and none of the paths of the test and of the fixture, `pz7`, the three passwords, `127.0.0.1`, `pg.pool.test`, `host.docker.internal`, `TRUNCATE`, `SELECT`, `INSERT` or a verifier; stdout is empty while serving; `src/log.rs` unit tests for the order, the escaping, `log.unnamed` and `log.dropped` | pass on the workstation |
| `0004 AC-25` | the same test: SIGTERM gives exit 0, `daemon.stopping` and `daemon.stopped` as the last line; the start advanced the start sequence to 1 (`daemon.started` and the journal read afterwards) | pass on the workstation for an idle stop; the stop with sessions in flight is step 11 |
| `0004 AC-14`, completed | the walking fixture admits only after `admission.changed` reports `open` for `pg1`; the first round is closed with `evidence.writer_missing` until a sample arrives; the wrong identity on the fixture is step 11 | pass on the workstation for the open path |
| `0004 AC-1`, completed | the build and the use check above | pass on the workstation |

**Readings.** A runtime that cannot be built is `listener.failed`, since nothing can listen without it and the spec has no code for it. A `Server` that cannot be built names the key whose file is missing (`principal.verifier_file`, `identity.credential_file` or `node.tls_ca_file`); the serving view and step 4 make it unreachable. The two tests of spec 0001 that used `run` as the example of an unknown command now use `serve`, since `run` is a command. The walking fixture writes `principal.password` (mode 0600) for the client; POOL never reads it.

### Build plan step 11: limits, loss and denial on the fixture

On 2026-11-15, on macOS (workstation, Docker Desktop 29.9.2, not Linux and not CI). Commits `f59c7aa` and `38c4ffa` (the tests of step 11), merged with `main` at `08091ec` in `0e99342`, which holds the fix of `NegotiateProtocolVersion` (`795775f`). PostgreSQL 18.7 and `psql` 18.7 of `postgres:18.7-bookworm` by the digest of step 8. Every case starts its own daemon on a journal of its own. No file under `apps/pool/src/` changed in this step.

#### UI / manual

- [ ] The CI job `integration` on the hosted runner → `NOT_RUN`, owner: user (manual step M2).

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh integration` on `0e99342` | exit 0 | exit 0, 20 of 20; no `pool-it-` container, no new volume or network, no temporary directory | pass |
| `scripts/check.sh integration` three times on `38c4ffa`, before the fix of `NegotiateProtocolVersion` | the protocol 3.2 case fails, every other case passes | each run: 19 passed, 1 failed (`it_a_request_for_protocol_3_2_goes_on_with_3_0`: `received invalid protocol negotiation message: server requested downgrade to pre-3.0 protocol version`); nothing left behind | the defect, found here and fixed in `pool-protocol` |
| `cargo test -p pool --locked` on `0e99342` | exit 0 | exit 0: 13 unit, 84 in `tests/check_config.rs`, 12 in `tests/commands.rs`, 20 ignored in `tests/integration.rs` | pass |
| `scripts/check.sh all` | exit 0 | exit 0, all six `ok` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0 | pass |

Observed numbers:

| What | Value |
|---|---|
| Highest resident memory of `pool` with 32 sessions that each asked for 256 MiB and stopped reading (`ps -o rss=`, 20 samples over 1 second) | 13376, 13952, 15968, 19120, 15200, 18624 and 18512 KiB in seven runs; the budget is 96 MiB (98304 KiB) |
| One row of 64 MiB (`repeat('ab', 33554432)`) | 67108865 bytes on stdout of `psql -At` (with its line feed), the same `md5sum` straight and through POOL |
| A `Query` whose length field is 1024, and 1025, with `frontend_message_max_bytes = 1024` | 1024 passes; 1025 gets `54000`; with a text of more than 1024 bytes the marker of the statement is not in the PostgreSQL log |
| A startup packet of more than 512 bytes (`application_name` of 600 bytes) | `54000` |
| A client that sends nothing, `startup_timeout_ms = 500` | closed with no byte after at least 450 ms and less than 2.5 s |
| The waiter with `backend_acquire_timeout_ms = 3000` | `53300` after at least 3 s and at most 5.5 s from the start of its `psql`; one more waiter `53300` in less than 2 s |
| Shutdown with `shutdown_timeout_ms = 1000` | an idle session `57P01`; a session in `pg_sleep(10)` `08007` after at least 0.9 s; exit 0 |

The SQLSTATEs of the psql cases are read from the message of the `FATAL` line, since `psql` does not print a SQLSTATE for a connection error, and each message is one row of [SQLSTATEs that POOL generates](index.md#sqlstates-that-pool-generates). The raw client (the backend code of `pool-core`, logged in to POOL as a client) reads the field `C` itself.

#### Acceptance-criteria coverage

| Criterion | Evidence (tests in `apps/pool/tests/integration.rs`) | Result |
|---|---|---|
| `0004 AC-14`, fixture half | `it_the_wrong_identity_closes_admission_before_any_login`: the node points at container 2 (same certificate, roles and passwords); `admission.changed` names `evidence.writer_identity`, the login gets `57P03`, and the log of container 2 shows no connection of `pz7_rw` | pass on the workstation |
| `0004 AC-15`, completed | `it_startup_parameters_versions_and_authentication_on_the_fixture`: `sslmode=disable` gets `28000` (`TLS is required`), `replication=database` and `options` get `0A000`; `it_a_request_for_protocol_3_2_goes_on_with_3_0`; `it_a_node_that_refuses_tls_gives_08006_and_gets_no_startup_packet` (the relay answers `N` to the `SSLRequest` of the session connection: `08006`); `it_a_wrong_ca_or_name_toward_the_node_closes_admission` (see the reading below) | pass on the workstation |
| `0004 AC-16`, completed | the same startup test: a wrong password and an unknown user both get `28P01` (`authentication failed`); the verifier logs in as nothing; the principal is no role of PostgreSQL | pass on the workstation |
| `0004 AC-17`, completed | the same test: an unmapped database gets `28000` (`no identity entry for this user and database`) and the PostgreSQL log shows no connection; `current_user`, `current_database()` are `pz7_rw` and `pz7_appdb` | pass on the workstation |
| `0004 AC-18`, completed | the same test: through POOL `pg_is_in_recovery()` is `f` and the system identifier is the one of container 1 | pass on the workstation |
| `0004 AC-19`, with the limit | `it_the_session_limit_and_teardown_give_the_permit_back`: with `session_max_count = 1` a client that leaves inside an open transaction leaves no row, its backend is gone within 2 seconds, and the next login succeeds | pass on the workstation |
| `0004 AC-20`, completed | `it_message_and_startup_limits_refuse_and_a_normal_login_follows`, `it_the_session_limit_and_teardown_give_the_permit_back`, `it_waiters_are_limited_and_time_out_after_the_acquire_deadline` (with `wait_limit` and `acquire_timeout` in the log); a normal login after each case | pass on the workstation |
| `0004 AC-21`, completed | `it_the_memory_budget_holds_with_32_sessions_that_stopped_reading`, numbers above | pass on the workstation |
| `0004 AC-22`, completed | `it_a_lost_commit_answer_gives_08007_and_is_never_replayed`: the relay passes the COMMIT, drops the answer of PostgreSQL and closes; the client gets one `FATAL` (`08007`), the COMMIT marker is in the PostgreSQL log once, no new connection of `pz7_rw` follows, the row is there afterwards, and the log has `backend_lost` | pass on the workstation |
| `0004 AC-23`, real server half | `it_messages_of_a_real_server_pass_through_or_end_as_the_spec_says`: a `NOTICE` from a `DO` block, `LISTEN` with `NOTIFY`, several statements in one query string, an error inside a transaction, all equal straight and through POOL; an empty query string gives `I` then `Z` from both; `COPY ... TO STDOUT` and `COPY ... FROM STDIN` work straight and end with `0A000` through POOL, and the backend is gone within 2 seconds | pass on the workstation |
| `0004 AC-25`, with sessions | `it_shutdown_ends_idle_sessions_with_57p01_and_busy_ones_with_08007` | pass on the workstation |

**Hermetic only**, as the rule "A golden message is not proof of a wire format" asks, because no real client of the fixture can be made to send them: a `GSSENCRequest`, a `CancelRequest`, a startup packet with another major version, a `_pq_.` parameter, another SCRAM header, a COPY of the tag `W`, and an authentication message of 1025 bytes. They are covered by the tests of `pool-protocol` and `pool-core`. An earlier version of this step sent them from the test with raw bytes and with `openssl s_client`; that was removed in `38c4ffa`.

**Readings.**

- A wrong CA or a wrong server name toward the node closes the cluster through the probe before any session tries: the probe and the sessions share the TLS setup of the node. On the fixture the client therefore gets `57P03`, not `08006`, and `probe.failed` names the class `tls`. `08006` toward the node is shown on the fixture with the relay that refuses TLS on the session connection only, and a wrong certificate gives `08006` in the hermetic test `a_wrong_node_certificate_gives_08006` of `pool-core`.
- The notification line of `psql` names the process ID of the backend, which differs straight and through POOL. The test sets that number aside before it compares.
- The fixture writes no new file for this step. The relay is a task of the test, in `apps/pool/tests/support/mod.rs`, and runs on a runtime of the test, since the source guard refuses threads of the standard library under `apps/`.

### Checker run of steps 10 and 11, and of milestone 4

Build plan steps 10 and 11, on 2026-11-15 from 22:00 to 22:55 WIB, by the checker session, on the tree `7d3e7fd` (the branch of the checker on top of `main` at `d4fdb62`; the code is the one of `d4fdb62`). macOS 27.0.1, arm64, `rustc 1.100.0`, Docker 29.9.2 running, PostgreSQL 18.7 and `psql` 18.7 of `postgres:18.7-bookworm` by the digest of `scripts/check.sh`. Nothing here is Linux or CI. Step 9 was verified before (see "Checker run" and "Checker run of the fix" under step 9). Its mutation runs were not repeated: the commits since then (`da51f43`, `cc77c34`) were verified in those two runs and in the run of milestone 3.

The tests that the tester added (on `main` from `e9389cc`, see [Test it](#test-it)) came after this run. The wrong changes and the test gaps below are measured against the tests of `d4fdb62`, and some of them may be closed by now.

Every command below was run in this session. The probe is the checker's own script. It takes the place of the test command inside `scripts/integration/run.sh`, so it runs against the two PostgreSQL containers that script started for it and against nothing else. It starts the built `pool run` with configurations of its own (23 runs of the daemon in one pass), and it drives them with `psql` of the image and with a client and a relay written for the probe in plain Python: TLS, `SCRAM-SHA-256` and the simple query protocol from the standard library, with no code of the repository. So the client side of every check is independent of `pool-core` and `pool-protocol`. The last pass of the whole probe, with nothing else running on the machine, had 131 checks and no mismatch, and its numbers are the ones below. The pass before it, without the section for a node that refuses TLS, had 129 checks and no mismatch. Earlier passes had mismatches that were mistakes of the probe, named under the notes.

**Verdict: step 10 PASS, step 11 PASS, milestone 4 PASS** on the workstation. The CI job `integration` is manual step M2 and has not run.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo build --workspace --locked`, after touching every crate root, and again with `--release` | no warning | 0 warnings in both | pass |
| The use check of [Adopted dependencies](index.md#adopted-dependencies), copied from the spec | no line | no line | pass |
| `scripts/check.sh all` on `ecd0ee5` (earlier in this session; the files of steps 10 and 11 are the same) | exit 0 | exit 0, all six `ok` | pass |
| `scripts/check.sh integration` on `aa10007` | exit 0, nothing left | exit 0, 21 of 21 in 76 s; no container of this run, no new volume, no temporary directory | pass |
| `scripts/check.sh linux-build` on `aa10007` | exit 0 | exit 0, 22 test results `ok` | pass |
| The daemon probe, all sections | every check as the spec says | 131 checks, 0 mismatches | pass |

#### `pool run` without a ready authority (`0004 AC-13`)

While `pool run` ran, a thread of the probe tried to connect to the listener port all the time. Each case was also tried with the port free afterwards.

| State of the journal | How the probe made it | Observed |
|---|---|---|
| Uninitialized | no journal, no witness | exit 1, `run refused`, `findings: 1`, `authority.uninitialized`; 0 connections reached the port |
| Unconfirmed | a real bootstrap, no witness in the configuration | exit 1, `authority.unconfirmed`; 0 connections |
| Missing | a witness, no journal | exit 1, `authority.missing`; 0 connections |
| Mismatch | a real bootstrap, another witness | exit 1, `authority.mismatch`; 0 connections |
| Unreadable | a real journal with its header overwritten, and one cut to one page | exit 1, `authority.unreadable`; 0 connections |
| Held | a real journal whose `hold` the probe set to `restore_pending` with SQLite | exit 1, `authority.restore_pending`; `show-authority` says `restore_pending`; 0 connections |

In every case stdout holds the three lines and stderr is empty, `journal.sqlite3` has the same bytes afterwards, and the port can be bound afterwards. The first read of a journal makes the two side files of SQLite (an empty `-wal` and a `-shm` of 32768 bytes), which `0004 AC-10` allows. `pool run --format json` is a usage error, exit 2, as the spec says.

When the state is ready and the start cannot go on: with the listener port taken by the probe, exit 3 `listener.failed`, and the start sequence in the journal is 1 (the journal is opened in step 5, the bind is step 7). With no listener key, no verifier or no credential file, exit 3 with `secret.not_found at` the key, and the start sequence is still 0.

#### The walking result with another SQL file (`0004 AC-18`, `AC-17`)

The SQL file of the probe is not the one of the developer. It has 27 statements and `psql` commands: `\set VERBOSITY verbose`, a transaction that inserts 2500 rows with text outside ASCII and updates five, an aggregate with `md5` over all rows, a row with NULL, an empty text, escapes, a large numeric and a `bytea`, a `DO` block with a NOTICE, three statements on one line, a transaction that fails on a duplicate key and is rolled back, a division by zero, a table that does not exist, a NULL into a column that forbids it, an empty statement, and `SHOW` before and after `SET`.

- Straight to PostgreSQL as the backend login and through POOL as the principal: exit 0 and 0, stdout 1196 and 1196 bytes, stderr 1067 and 1067 bytes, each pair equal byte for byte. The output through POOL holds the SQLSTATEs 22012, 42P01, 23505, 23502 and 25P02 with the fields PostgreSQL sent.
- Through POOL, `current_user` is `pz7_rw`, `current_database()` is `pz7_appdb`, `pg_is_in_recovery()` is `f`, and the system identifier is the configured one, the one of container 1.
- The table holds the same 2500 rows after each run. The PostgreSQL log shows the session as `user=pz7_rw database=pz7_appdb` and never the principal or the requested database.
- The raw client is offered `SCRAM-SHA-256` and nothing else, gets no `BackendKeyData` (`RRRR`, fifteen `S`, `Z`), and the server signature POOL sends is the one the password gives. `application_name` and `client_encoding` of the client reach PostgreSQL. An error of PostgreSQL arrives with all its fields and its own text.
- The first view is closed with `evidence.writer_missing` at snapshot generation 1, and admission opens at generation 2, after the first sample. `daemon.started` carries start sequence 1, configuration generation 1 and the authority ID of the bootstrap. A second start has start sequence 2. SIGTERM and SIGINT with no session: exit 0 at once, `daemon.stopping` and `daemon.stopped` last, nothing on stdout.

#### Startup, authentication and the identity map (`0004 AC-15`, `AC-16`, `AC-17`)

All of these were sent to the real daemon on the fixture by the raw client:

| What the client does | Observed |
|---|---|
| A startup packet without TLS | one error `28000` `TLS is required` with the fields `S`, `V`, `C`, `M`, no authentication request, closed |
| `GSSENCRequest` | `N` |
| `CancelRequest` | no byte, closed |
| A wrong password; a user that is no principal; the backend login with its own password; the principal with the password of the backend login | `28P01` `authentication failed`, each |
| A wrong password and two unknown users, message by message | the same three messages with the same lengths (`R` 19, `R` 88, `E` 45 bytes of body) |
| A database with no identity entry; the backend database as the requested one; no `database` parameter | `28000` `no identity entry for this user and database`, each |
| `replication`, `options`, another parameter (`DateStyle`) | `0A000`, each |
| Protocol major 2 and major 4 | `0A000`, each |
| A request for 3.2 with the option `_pq_.zq9` | `NegotiateProtocolVersion` first, body `00030000000000015f70715f2e7a713900`, then the login goes on and a query works |
| A SASL message of more than 1024 bytes | `08P01` |

Over all of it the PostgreSQL log shows three connections of the backend login, the three logins that passed. No refused login reached PostgreSQL. The principal is no role of PostgreSQL, and `psql` straight to PostgreSQL as the principal with its password is refused. The log of POOL counts the refusals as `plaintext` 1, `authentication` 7, `unmapped` 3, `unsupported` 5, `cancel_ignored` 1 and `protocol` 2.

#### The node (`0004 AC-14`, `AC-15`)

- **Another identity.** The journal is made against container 1 and the node then points at container 2. `admission.changed` is closed with `evidence.writer_identity`, three logins get `57P03`, the cluster was never open, and the log of container 2 shows the monitor login and no connection of the backend login.
- **A CA that did not sign the node certificate, and a server name the certificate does not hold.** `probe.failed` with the class `tls`, a login gets `57P03`, PostgreSQL authorized no connection. This is the reading of decision 72.
- **A node that refuses TLS on the session connection only.** The relay of the probe answers `N` to the `SSLRequest`. The client gets `08006` `the backend connection could not be established`. POOL sent the 8 bytes of the `SSLRequest` and not one byte after the `N`. The log has `backend.failed` with the class `tls`.
- **A wrong password of the backend login.** The client gets `08006` with the constant message, and the log has `backend.failed` with `class` `login` and `sqlstate` `28P01`. No text of PostgreSQL reaches the client or the log.

#### The control loop and the route view (`0004 AC-14`, decision 12)

| Case | Observed |
|---|---|
| **Silence.** `docker pause` on the PostgreSQL container in the middle of a run, with the expiry at 2 s | closed with `evidence.writer_missing` after 1.1 s; `probe.failed` with the class `timeout`; a new login gets `57P03`; the session that was running is not ended. After `docker unpause`: open again, `probe.recovered`, the old session answers a query, a new login works |
| **A positive conflict.** The node is a relay of the probe. With three sessions running (idle, idle in a transaction, a statement in flight), the relay cuts the connection the probe keeps and leads new ones to container 2 | closed with `evidence.writer_identity`; after 0.2 s (0.4 s in the pass before) the idle session and the one in a transaction get `57P01` and the one in flight gets `08007`; three `session.ended` lines with the reason `conflict`; a new login gets `57P03`; the backends are gone from container 1, the open transaction left no row, container 2 saw no connection of the backend login; no latch is set |
| **A second primary.** A second node of the same cluster that leads to the same server | closed with `authority.conflict_latched` and `evidence.second_primary`; the log has that closed view first, then `authority.conflict_latched` with `recorded` false, then with `recorded` true; a login gets `57P03`; the journal afterwards has `conflict_latched` 1 and authority revision 2 |
| **The same while the journal cannot be written.** The second node is not reachable at first, so the cluster opens on the first node and a session runs. The probe then takes the write lock of the journal with SQLite and lets the second node answer | closed 0.18 s after the switch (0.14 and 0.30 s in the passes before), the running session is ended with `57P01` at the same moment, a login gets `57P03`; `journal.write_failed` with the class `journal.busy`; the latch is logged with `recorded` false only. When the probe lets go, the next round writes it (`recorded` true), and the journal has revision 2 |
| **The latch at the next start.** Back to the one good node | `show-authority` says `ready`, the cluster stays closed with `authority.conflict_latched`, a login gets `57P03`, the latch is not written again |

So the cluster closes while the journal cannot be written, and the latch reaches the journal later. That the view is sent before the write, and not after it, is read in `apps/pool/src/control.rs`: the round sends the view first and writes the latch after it, on a blocking thread. From outside the two orders look the same, because the journal answers busy at once.

#### Teardown, a lost COMMIT answer, limits and memory (`0004 AC-19` to `AC-22`)

- **Teardown**, with `limits.session_max_count = 1`. After an INSERT in an open transaction the client closes its socket, or sends `Terminate`: the backend is gone from `pg_stat_activity` after 0.07 and 0.06 s, the row is not there, a second login meanwhile got `53300`, and the next login works. A client that leaves while a statement runs is noticed when the statement ends, as the spec says: the backend was still there after 0.5 s, and gone 0.2 s after the probe cancelled the statement.
- **Lost COMMIT.** The relay of the probe passes the COMMIT on, drops the answer of PostgreSQL and closes. The client gets one message, the error `08007` with severity `FATAL`, and then the end of the connection. The PostgreSQL log holds that COMMIT once (by a marker in a comment), no new connection of the backend login follows, the row is there, and the log of POOL says `session.ended` with `backend_lost` and `in_flight`.
- **Limits**, with `frontend_message_max_bytes = 2048`, `startup_packet_max_bytes = 512`, `startup_timeout_ms = 700`, `handshake_max_count = 4`, `node_backend_max_count = 1`, `backend_wait_max_count = 1` and `backend_acquire_timeout_ms = 2500`:

| Case | Observed |
|---|---|
| A `Query` whose length field is exactly 2048 | passes |
| A `Query` of 3031 bytes | `54000`, closed, and its text is not in the PostgreSQL log |
| The 5 header bytes of a `Query` with the length 2049 and nothing more | `54000` at once |
| A startup packet of more than 512 bytes, and one of about 450 | `54000`, and a login |
| A client that sends nothing | closed with no byte after 0.70 s |
| A client that stops after the authentication request | closed with no message after 0.70 s |
| Four connections that say nothing, then a fifth | the fifth is closed at once with no byte |
| The one backend of the node is taken: a first waiter, then a second | the first gets `53300` after 2.51 s, the second gets `53300` after 0.00 s |
| A waiter, and the holder then leaves | the waiter is served |
| `session_max_count = 2`, a third login; then a principal limit of 1 under it | `53300` (a wrong password still gets `28P01`); `53300`, counted as `principal_limit` |

  A normal login worked after each case. The log counts `wait_limit`, `acquire_timeout`, `message_limit`, `startup_timeout` and `handshake_limit`.
- **Memory.** 32 raw clients each asked for 4096 rows of 65536 bytes (256 MiB) and never read. PostgreSQL showed 32 active statements, all 32 waiting to write to the client. Resident memory of `pool` (`ps -o rss=`): 9920 KiB before, highest of 40 samples over 4 seconds 18192 KiB, lowest 17824 KiB. The two passes before gave 18048 and 18192 KiB. The budget is 98304 KiB. One of the clients then read its whole result: 4096 rows, and the md5 over all of them is the one PostgreSQL computes.
- **One row of 64 MiB**, with content that does not repeat (`string_agg` of 2097152 `md5` texts): 67108865 bytes on stdout of `psql -At` straight and through POOL, with the same `md5sum`. Resident memory of `pool` afterwards 18240 KiB.

#### Shutdown with sessions (`0004 AC-25`)

With `shutdown_timeout_ms = 1500`, five sessions and one connection still in its startup, once with SIGTERM and once with SIGINT, and a second signal 0.3 s after the first:

| Session | Observed (SIGTERM, SIGINT) |
|---|---|
| Idle | `57P01` after 0.00 and 0.00 s |
| Idle in a transaction | `57P01` after 1.51 and 1.50 s, which is the deadline, as step 2 of the spec takes only the status `I` at once |
| A statement that runs for 30 s | `08007` after 1.51 and 1.50 s, and never `57P01` |
| A statement that ends 0.7 s after the signal | its whole answer, then `57P01`, after 0.68 and 0.69 s |
| A client that asked for 256 MiB and stopped reading | whole messages and then the end of the connection, with no error message |
| The connection in its startup | closed with no byte |

The daemon exits 0 after 2.56 and 2.52 s. The log has one `daemon.stopping` with `sessions` 5, five `session.ended` with the reason `shutdown`, and `daemon.stopped` last. A connect after the signal is refused. No backend of the five is left on PostgreSQL and the open transaction left no row.

#### Messages after login on a real server (`0004 AC-23`)

Several statements in one `Query` with an error in the middle, `LISTEN` with `NOTIFY`, and a `ParameterStatus` after `SET` pass through. `COPY ... TO STDOUT` and `COPY ... FROM STDIN` end with `0A000`, the client gets no COPY message, and the backend is gone. Each of the tags `P`, `B`, `D`, `E`, `C`, `H`, `S`, `F`, `d`, `c`, `f` after login gets `0A000`. An unknown tag (`Z`, `q`, `!`, `R`), a `Query` with the length 3 and a `Terminate` with a body get `08P01`. Every backend of those sessions is gone afterwards and a normal login works.

#### The log (`0004 AC-24`)

Over the whole pass: 375 lines of 23 runs of the daemon, with canary texts in the application name, a user name, a database name, a password, SQL text, an error raised in PostgreSQL and an oversized message.

- Every line is one JSON object. `ts`, `level` and `event` come first, in that order, and `ts` has the form of the spec.
- Only events of the table appear, each with only its fields, in the order of the table. Every value is a string, a whole number or a boolean.
- No line holds a password, the verifier, a canary, SQL text, error text of PostgreSQL, a name with `pz7`, a host name, `127.0.0.1`, a path, a system identifier or a port of the fixture.
- stdout is empty in all 23 runs.
- Events seen: `daemon.started`, `daemon.stopping`, `daemon.stopped`, `admission.changed`, `authority.conflict_latched`, `journal.write_failed`, `probe.failed`, `probe.recovered`, `session.started`, `session.ended`, `session.refused` and `backend.failed`. Only `log.dropped` was not produced.

#### Criteria

| Criterion | Verdict |
|---|---|
| `0004 AC-1`, last sentence | met: the use check prints nothing and the build has no warning |
| `0004 AC-13` | met, for all six states |
| `0004 AC-14` | met: the open path, the wrong identity on the fixture, and on the fixture also what the criterion asks only of a hermetic test (a running session is ended for a positive conflict and not for silence) |
| `0004 AC-15` | met, with the reading of decision 72 for a wrong certificate |
| `0004 AC-16`, `AC-17` | met |
| `0004 AC-18` | met, with another SQL file than the tests use |
| `0004 AC-19` | met |
| `0004 AC-20` | met |
| `0004 AC-21` | met, about 18 MiB against a budget of 96 MiB |
| `0004 AC-22` | met |
| `0004 AC-23`, real server half | met |
| `0004 AC-24` | met over 375 lines |
| `0004 AC-25` | met, with sessions in every state and a client that stopped reading |

#### The readings of the developer

- **A runtime that cannot be built is `listener.failed`.** Accepted. The spec has no code for step 6, nothing can listen without a runtime, and the case cannot be made to happen from outside. It was read in `daemon.rs` only.
- **`serve` in the tests of spec 0001.** `run` is a command now, so the two tests of an unknown command use `serve`. Seen in `apps/pool/tests/check_config.rs`.
- **The numbers of the fixture** (an acquire deadline of 3000 ms, a probe of 200, 5000 and 8000 ms) are written in the spec since decision 73. The probe used other numbers (2500 ms; 200, 1000 and 2000 ms) and the bounds held: 2.51 s for the deadline, 1.1 s to close on silence.
- **The client of the memory case.** The spec now says a client that does not read. The probe used its own, with no code of `pool-core`, and got the same range as the developer (13 to 19 MiB).
- **Hermetic only.** The record of step 11 lists the messages that only hermetic tests cover and says so, as the rule "A golden message is not proof of a wire format" asks. Its reason, that no real client of the fixture can be made to send them, holds for `psql`. The raw client of the probe did send five of them to the real daemon: `GSSENCRequest`, `CancelRequest`, another major version, a `_pq_.` parameter and a SASL message of more than 1024 bytes, each with the result of the spec. Another SCRAM header and a COPY of the tag `W` stay hermetic.

#### Notes that are not failures

- **`journal.write_failed` is written in every round while the write fails.** With two nodes and a period of 200 ms that was 16 lines in about a second, at the level `error`. The spec says the write is tried again after the next round and logged, so this is as written. The refusals are grouped per second for the reason that a flood must not flood the log, and a disk that stays broken does the same here. For the architect.
- **Two errors when PostgreSQL ends an idle backend.** With `pg_terminate_backend` on the backend of an idle session, the client gets the `FATAL` `57P01` of PostgreSQL, passed on unchanged, and then `08006` of POOL. Both rules of the spec apply (a message of the backend is passed on; a lost backend gives one error of POOL), so a client sees two.
- **A client that stopped reading costs one second more at shutdown.** The daemon exits about 1.1 s after the deadline, which is the `CLOSE_TIMEOUT` of the fix `da51f43`.
- **A statement that produces nothing keeps its PostgreSQL backend** after POOL closed the connection, until the statement ends: PostgreSQL notices a closed client only when it writes. A first version of the probe used `pg_sleep(30)` and found that backend 5 seconds after the shutdown. With a statement that sends a notice every 0.2 s the backend is gone at once. This is PostgreSQL, not POOL, and `0004 AC-25` does not ask for more.
- **A failed bind uses up a start sequence.** The journal is opened and advanced in step 5 and the listener is bound in step 7, as the spec orders it.
- **No command shows a latch.** After the second primary, `show-authority` says `ready` while every login gets `57P03`. The reason is in the log of the daemon (`admission.changed`). A later scope may want it in a command.
- **Mistakes of the probe in earlier passes**, corrected before the two clean passes: it gave `--format` to `pool run`; it compared the side files of SQLite; it expected the probe of the daemon to open a new connection each period, where the spec says it keeps one, so its relay had to cut that connection; it armed the relay on the connection of the probe too; and it counted the backends of an earlier section.

#### For the tester: wrong changes that no test caught

Twelve wrong changes to `apps/pool/src/`, one at a time, in a private copy, each against `cargo test -p pool --locked` and then `scripts/check.sh integration`:

| Wrong change | Result |
|---|---|
| M1. `route_view` never sets the conflict flag, so a positive conflict ends no running session | **not caught** |
| M2. The route view is published after the latch writes, not before (against decision 12) | **not caught** |
| M3. A conflict latch is never written to the journal | **not caught** |
| M4. An open route stays fresh for one hour more than its sample allows | **not caught** |
| M5. The state `mismatch` is reported as `authority.unconfirmed` | caught by `it_run_refuses_a_journal_without_or_with_another_witness_and_never_binds` |
| M6. The state `restore_pending` is reported as `authority.unreadable` | **not caught** |
| M7. Shutdown does not end the sessions | caught by `it_shutdown_ends_idle_sessions_with_57p01_and_busy_ones_with_08007` |
| M8. The probe keeps its connection after a failure | **not caught** |
| M9. `probe.failed` is written for every failed sample, not only when the class changes | **not caught** |
| M10. `admission.changed` is written only when open or closed changes, not when the reasons change | caught by `it_the_wrong_identity_closes_admission_before_any_login` |
| M11. The counters of refusals are never flushed while the daemon serves | caught by `it_waiters_are_limited_and_time_out_after_the_acquire_deadline` |
| M12. The start sequence is advanced before the protected files are read, so a missing key file uses one up | **not caught** |

Eight of twelve were not caught. The probe of this run would have caught M1, M3, M6 and M12, which shows what a test needs. It would not have caught M2: the journal answers busy at once, so a view that is published after the write is late by nothing one can measure from outside. That order is read in the code only. M6, M10 and M11 were run a second time with nothing else on the machine, with the same result.

Test gaps seen beside those:

- No test on the fixture has a second node, so `evidence.second_primary`, the latch, its two log lines and the journal write are tested in `pool-control` only, and the path through `apps/pool/src/control.rs` is not.
- No test ends a running session through the daemon for a positive conflict, and none shows that silence does not end one. `pool-core` tests both with a route view made by the test.
- No test makes the journal write fail while the daemon runs.
- No test starts `pool run` on a journal with the hold `restore_pending`.
- The redaction test searches for `pz7`, the passwords, the hosts and three SQL words. It does not check that only the fields of the table appear, and it has no canary in the error text of PostgreSQL.

#### One hang of a test, sent to the developer of step 8

During the mutation runs, one run of `cargo test -p pool --locked` on macOS did not end. The test `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked` of `tests/commands.rs` waited in `wait_with_output` for a child, and that child, `pool show-authority` with a named pipe as its configuration, was blocked in `read` on the pipe. No other process had the pipe open, so there was no writer left, and the read still got no end of file. It stayed so for 12 minutes, until the probe ended the child. The machine was very busy at that moment: the fuzz check and two builds ran beside it. The wrong change under test at that time (M6) cannot reach this path, and the product is not involved: the child did what it should with a file that never ended.

It did not come back: 20 runs of that test alone, 30 runs of the whole test binary, and 80 runs of the whole binary with 24 busy loops on the 10 cores all passed. So it was seen once in 137 runs on macOS, and the cause is not established. A test that can wait without a bound stops a whole check, so it was sent to the developer with what was seen. It does not change a verdict of this run.

## Milestone 5: Fuzzing, documents and evidence

Build plan steps 12 to 14. Each step keeps its own part of this section.

### Build plan step 12: fuzzing and the CI rule

Build plan step 12, on 2026-11-15, on macOS (workstation, Docker Desktop running) and in the Linux builder image. Commit `87be008`, on top of `main` at `60086ae`, after commit `8f64f5a` (`Debug` by hand on the `pool-protocol` types that hold content). Tools: `rustc 1.100.0`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`, `cargo-deny 0.20.2`.

#### UI / manual

- **M2. CI jobs and the branch rule file** (`owner: user`, `BLOCKED` until you make it; `.github/` is not a file of any session). After Build plan step 12. Prerequisite: this commit on the scope branch. Add the two jobs `integration` and `fuzz` of spec 0004 "CI definition and manual steps" to `.github/workflows/ci.yml`, after the job `docs`, exactly as the spec writes them (`runs-on: ubuntu-24.04-arm`, `timeout-minutes: 30`, the pinned `actions/checkout` and `actions/cache`, the `run:` lines `rustup toolchain install`, `scripts/check.sh integration`, `scripts/check.sh fuzz-tools` and `scripts/check.sh fuzz`), and the two contexts `integration` and `fuzz` to `required_status_checks` in `.github/rulesets/main.json`, in one commit. Command to check it before the commit: `scripts/check.sh guards`. Expected: exit 0, `pool-repo-check ci: ok` (rule C3 now accepts `scripts/check.sh fuzz-tools`; rule C2 holds when the two contexts and the two jobs come together). Criteria: `0004 AC-27` on CI, and the CI half of the integration criteria.
- **M3. Push, pull request and the rule on the remote** (`owner: user`, `BLOCKED`). Push the scope branch, open the pull request, and after the nine checks have run on it, apply `main.json` to the ruleset on the remote, as the remote proof of spec 0002 did for the first seven. Expected: the jobs `integration` and `fuzz` pass on the runner, and a merge needs all nine. Until then the two jobs run and do not block a merge.

#### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo test -p pool-protocol --locked` at `8f64f5a` | exit 0 | exit 0: 22 unit, 10 golden and 4 in the new `tests/redaction.rs` passed | pass |
| `cargo test -p pool-core --locked` at `8f64f5a` | exit 0 | exit 0: 37 unit and 29 session tests passed; nothing depended on the old `Debug` text | pass |
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` | exit 0 | exit 0: 108 unit and 110 integration tests passed, among them `w1_and_w6_know_the_lockfile_of_the_protocol_fuzz_package` and `c3_accepts_fuzz_tools_and_nothing_like_it` | pass |
| `scripts/check.sh fuzz-tools` | exit 0 | exit 0: `cargo-fuzz 0.13.2 is already available`, `the toolchain nightly-2026-10-17 is already installed` | pass |
| `scripts/check.sh fuzz` at `87be008` | every target of both packages runs on its committed inputs for the bounded time, no crash file | exit 0 in 130 s: `validate` 273996 runs, `startup` 2733111, `frontend_message` 660274, `scram` 3546499, each 31 s, no file under any `fuzz/artifacts/` | pass |
| `scripts/check.sh all` at `87be008` | exit 0 | exit 0: fmt (now also the new fuzz manifest), clippy, test, deny (now also the new lockfile), guards, docs all `ok` | pass |
| `scripts/check.sh linux-build` at `87be008` | exit 0 | exit 0 | pass |
| `scripts/check.sh integration` at `87be008` | exit 0 | exit 0: 8 of 8 against PostgreSQL 18 in Docker | pass |

**The final tree.** After commits `8e71ae5` (the flag on both fuzz lockfile commands) and `795775f` (`NegotiateProtocolVersion` carries the whole protocol code, `0004 AC-15`), and `main` merged in `d556e98`:

| Command at `d556e98` | Observed | Result |
|---|---|---|
| `cargo test -p pool-protocol --locked` | 22 unit, 11 golden (with `negotiation_carries_the_whole_protocol_code_as_postgresql_18_7_sends_it`) and 4 redaction tests passed | pass |
| `cargo test -p pool-core --locked` | 37 unit and 29 session tests passed | pass |
| `scripts/check.sh all` | exit 0 | pass |
| `scripts/check.sh fuzz` | exit 0: `validate` 92047 runs, `startup` 1362502, `frontend_message` 427525, `scram` 3685731, each 31 s, no crash file | pass |
| `scripts/check.sh integration` | exit 0, 8 of 8 | pass |
| `scripts/check.sh linux-build` | first run exit 1: `ac9_a_second_command_while_the_lock_is_held_is_state_locked_and_changes_nothing` in `apps/pool/tests/commands.rs:403` got exit 3 `state.locked` after the test dropped its lock; second run exit 0. The test is not of this step and passed at `87be008` | flaky, reported to the lead |
| `scripts/prove-checks.sh --with-linux-build` | exit 0: `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 run with Docker | pass |

**What the targets hold.** `startup` feeds the first packet reader before or after TLS (the first input byte says which) and asks the same results for the whole input, three pieces cut where the next two bytes say, and single bytes; it also runs the parameter parser on the rest. `frontend_message` does the same for the whole message reader (limit 1024) and for the header reader with the classification of client and backend headers. `scram` runs the four SCRAM parsers and checks that base64 decodes what it encodes and encodes back what it decodes. That closes the gaps the checker named: three pieces and random input on the first packets and the messages, from raw bytes. The regression inputs under `crates/pool-protocol/fuzz/regressions/<target>/` are the golden packets and messages of the tests (33 files); `crates/pool-config/fuzz/regressions/validate/` starts with the minimal fixture.

**The `deny` command of the fuzz lockfiles.** `cargo deny check` on `crates/pool-protocol/fuzz/Cargo.lock` with the `deny.toml` of the repository failed only with `license-not-encountered` for `Unicode-3.0`: the lockfile holds the same ten third party packages as `crates/pool-config/fuzz/Cargo.lock`, at the same versions, and none of them is under that license. On the decision of the architect (spec 0004 "Dependency policy", commit `7b93379` of the spec branch), both fuzz lockfile commands of `scripts/check.sh deny` now run with `-A license-not-encountered` after `check`; the command of the product lockfile is unchanged, and so is `deny.toml`. With the flag both fuzz lockfiles give `advisories ok, bans ok, licenses ok, sources ok`.

#### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0004 AC-27` | `crates/pool-protocol/fuzz/` with the targets `startup`, `frontend_message` and `scram`, its lockfile and committed regression inputs; `scripts/check.sh fuzz` builds every target of both fuzz packages and runs each on its committed inputs for 30 s, and ends without a crash file | pass on the workstation; the CI job is manual step M2 |
| `0004 AC-24`, the `pool-protocol` types | `tests/redaction.rs` puts `CANARY-9f2c7b1e` into every content field of `Message`, `MessageReader`, `Authentication`, `SaslInitialResponse`, `NegotiateProtocolVersion`, `ParameterStatus`, `ClientFirst`, `ServerFirst`, `ClientFinal`, `ServerFinal`, `Forwarded`, `FirstPacket` and `FirstPacketReader` and finds it in no `Debug` text | pass |

### Build plan step 13: document corrections

On 2026-11-15, on top of `main` with step 12 merged. Items 1 and 2 of [Document corrections](index.md#document-corrections), and nothing else in the two files.

| Item | File | Change | Result |
|---|---|---|---|
| 1 | `docs/engineering-standards.md` | The row "SQLSTATEs and messages that POOL itself generates" of the shared conventions table now holds the rule (`FATAL`, the fields `S`, `V`, `C` and `M` only, constant text, one SQLSTATE per situation from a fixed table, a server error during a query passed on unchanged) and links to [SQLSTATEs that POOL generates](index.md#sqlstates-that-pool-generates). The first paragraph names the five workspace members | done |
| 2 | `docs/repository-structure.md` | The first paragraph adds `crates/pool-protocol` and `crates/pool-core` to the paths that exist | done |

`scripts/check.sh docs`: exit 0 (links and anchors D1 and D2, scope status D3, IDs D4 to D6).

Two sentences of the same files are no longer exact and are left to the "Document it" box of scope 004, as item 5 of the corrections says: the paragraph on the fuzz package in "Rust and runtime" of `docs/engineering-standards.md` names only `crates/pool-config/fuzz/`, and `crates/pool-protocol/fuzz/` now exists too; the row of `apps/pool` in the crate table of `docs/repository-structure.md` says it depends on all four internal crates, and today it depends on `pool-config`, `pool-control` and `pool-core`, not on `pool-protocol`.

### Checker run of steps 12 and 13

Build plan step 12 (`0004 AC-27`) and step 13, on 2026-11-15, by the checker session, on the tree `7d3e7fd` (the code of `main` at `d4fdb62`). macOS 27.0.1, arm64, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`, `cargo-deny 0.20.2`. Nothing here is Linux or CI. Step 14 is not part of this run, and neither are the tests the tester added afterwards. Every command below was run in this session.

**Verdict: step 12 PASS, step 13 PASS.** The CI job `fuzz` is manual step M2 and has not run.

| Command or probe | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh fuzz-tools` | exit 0 | exit 0: `cargo-fuzz 0.13.2 is already available`, `the toolchain nightly-2026-10-17 is already installed` | pass |
| `scripts/check.sh fuzz` | every target of both packages is built and run on its committed inputs for a bounded time, no crash file | exit 0: `validate` 256923 runs, `startup` 1021575, `frontend_message` 874753, `scram` 2434550, each 31 s; no file under any `fuzz/artifacts/`; `git status` clean afterwards | pass |
| What each target started from, read in the output | its committed inputs | `validate`: 1 regression input and the 39 fixtures; `startup`: 11 inputs; `frontend_message`: 13; `scram`: 9 (33 for the three new targets) | pass |
| Rule W1, in a private copy: a package added to `crates/pool-protocol/fuzz/Cargo.lock`; the last line taken from `protocol-fuzz-lock-packages.txt` | a W1 problem | `W1: crates/pool-protocol/fuzz/Cargo.lock: package zq9-extra is in the lockfile but not in the baseline`; `W1: ... package shlex is in the lockfile but not in the baseline` | pass |
| Rule W6: a fifth `Cargo.lock` under `crates/pool-core/fuzz/` | a W6 problem | `W6: crates/pool-core/fuzz/Cargo.lock: unexpected Cargo.lock`; the unchanged copy with its four lockfiles is `ok` | pass |
| Rule C3: a job `fuzz` in a copy of the workflow with the line `scripts/check.sh fuzz-tools`; then with `scripts/check.sh fuzz-toolsx`, `scripts/check.sh sbom` and `cargo install cargo-fuzz` in its place | the first is taken by C3, the three others are refused | no C3 problem for `fuzz-tools`; a C3 problem for each of the three others. (Each copy also has the C2 problem that the job is no required check yet, which is manual step M2) | pass |
| The three `deny` commands of `scripts/check.sh deny` | `ok` | `advisories ok, bans ok, licenses ok, sources ok` for the product lockfile and for both fuzz lockfiles | pass |
| Decision 70: the two fuzz commands with a copy of `deny.toml` that has no exception for the license of the fuzz runtime (NCSA) | a license that is not allowed still fails | exit 4 and `licenses FAILED` with one rejected package, for both fuzz lockfiles | pass |
| The same with `MIT` taken from the allowed list | fails where a package needs it | `crates/pool-config/fuzz`: exit 4, `licenses FAILED`. `crates/pool-protocol/fuzz`: exit 0, because no package of that lockfile needs MIT alone | pass |
| The fuzz commands without the flag `-A license-not-encountered` | the reason for the flag | `crates/pool-protocol/fuzz`: `licenses FAILED`, only for an allowed license that no package uses. `crates/pool-config/fuzz`: `ok` | as the decision says |
| The product lockfile, which runs without the flag | `ok`, so the list of the product cannot grow unseen | `ok` | pass |
| Step 13: `docs/engineering-standards.md` and `docs/repository-structure.md` against items 1 and 2 of [Document corrections](index.md#document-corrections) | the two changes and nothing else | the row for SQLSTATEs holds the rule and the link, the first paragraph names five members; the first paragraph of the other file adds `crates/pool-protocol` and `crates/pool-core`. The commit `d6bec25` changes those three lines and no other line of the two files | pass |

#### Does a fuzz target catch a wrong change?

Five wrong changes to `crates/pool-protocol/src/`, one at a time, in a private copy. Each was run against `cargo test -p pool-protocol --locked`, then against the committed inputs of the target alone (`-runs=0`), then for 25 seconds of fuzzing from those inputs.

| Wrong change | `cargo test -p pool-protocol` | The fuzz target |
|---|---|---|
| `Terminate` with any length is taken as `Terminate` (the fault behind decision 52) | caught by `message::tests::frontend_rows` | **not caught** by `frontend_message`: its 13 committed inputs pass, also `terminate-with-body`, and 25 s of fuzzing find nothing |
| `NegotiateProtocolVersion` carries the bare minor number (the fault behind decision 71) | caught by `auth::tests::negotiation_round_trips_and_checks_its_count` and `startup::tests::negotiation_only_when_needed` | **not caught**: no target calls the encoder |
| The message reader does not ask its limit before it reserves the body | caught by `message::tests::a_reader_reserves_nothing_for_a_body_over_its_limit` | caught by a committed input of `frontend_message`: `ERROR: libFuzzer: out-of-memory (malloc(4294967291))` |
| The first packet reader asks the size limit only when the 8 header bytes came in one piece | caught by `first_packets_split_at_every_byte_give_the_same_result` | caught by a committed input of `startup`: the pieces and the whole input give different results |
| base64 takes padding in the middle of the text | caught by `base64::tests::strict_form_only` | no committed input fails; `scram` finds it within 25 s of fuzzing (what it decodes does not encode back to the same text) |

What this shows: a target fails for a panic, for a large reservation, and for an answer that depends on how the bytes were cut. It does not know which answer is right. So the two faults that the verifications found by hand (decisions 52 and 71) are caught by the unit and golden tests that were added with their fixes, and not by a fuzz target. That is what the spec asks of the targets ("Must not panic, or reserve memory from a length field"), so it is a note for the tester and not a failure.

#### Notes that are not failures

- **`fuzz/corpus/` is not committed**, so a new checkout starts each of the three new targets from its regression inputs alone. That is the design of the spec. On this run `startup` started from its 11 inputs, `frontend_message` from 13 and `scram` from 9, and each ran 31 seconds with no crash.
- **A lockfile that is missing** makes `pool-repo-check workspace` say `could not run`, not a W problem. That is how the tool treats every lockfile of W1.
- **Spec 0002 still says three lockfiles** in the row of rule W6 of its table. Spec 0004 and the tool say four. A dated note in spec 0002 would close it. For the architect.
- **The CI workflow has no `fuzz` and no `integration` job yet.** That is manual step M2, and `.github/` is no file of a session.

### Build plan step 14: evidence

On 2026-11-15, on macOS (workstation, Docker Desktop running; not Linux and not CI) and in the Linux builder image. The final tree is revision `ecd0ee5` of `main`: every Build plan step from 1 to 13, the fix of the lock test (`2590ba6`) and of the bootstrap that stopped (`b33f155`). Its code is that of `3ab5045`; `ecd0ee5` adds two lines to this file only. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`. Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

#### UI / manual

Manual steps M2 and M3 are under [Build plan step 12](#build-plan-step-12-fuzzing-and-the-ci-rule), and both are `BLOCKED`, `owner: user`. Until they are done the CI jobs `integration` and `fuzz` do not exist in `.github/` and their runs are `NOT_RUN`.

#### Commands

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo metadata --no-deps --format-version 1` | five members, the six pairs of [Workspace](index.md#workspace), `pool-protocol` without dependencies | `pool` → `pool-config`, `pool-control`, `pool-core`; `pool-control` → `pool-config`; `pool-core` → `pool-config`, `pool-protocol`; `pool-config` and `pool-protocol` take no workspace package | pass |
| `Cargo.lock` | the five members and 70 third party entries | 75 entries, 70 of them from the registry | pass |
| `cargo build --workspace --locked` | exit 0 and no warning about a dependency | exit 0, no warning line at all | pass |
| The use check under [Adopted dependencies](index.md#adopted-dependencies) | no output | no output | pass |
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards, docs all `ok`. Tests: `pool` 13 unit, 84 `check_config`, 12 `commands` (21 integration ignored here); `pool-config` 32 unit, 123 `validate` and its doc tests; `pool-control` 5 unit, 3 `admission_model`, 12 `controller`, 19 `journal`, 28 `walkthroughs`; `pool-core` 37 unit, 29 `session`; `pool-protocol` 22 unit, 11 golden, 4 redaction; `pool-repo-check` 108 unit, 110 integration | pass |
| `scripts/check.sh integration` | exit 0 | exit 0 in 63 s: 21 of 21 against the pinned image | pass |
| `DOCKER_HOST=unix:///nonexistent/docker.sock scripts/check.sh integration` | exit 3 without a Docker daemon (`0004 AC-26`) | exit 3, `integration: the Docker daemon is not running; start it and run scripts/check.sh integration again` | pass |
| `scripts/check.sh fuzz` | every target of both fuzz packages on its committed inputs, no crash file | exit 0: `validate` 325335 runs, `startup` 3285074, `frontend_message` 491711, `scram` 2501940, 31 s each; no file under any `fuzz/artifacts/` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0 in 101 s, host `aarch64-unknown-linux-gnu`, `pool 0.1.0`, binary `sha256 35ead2faee9d225e9696ee3f507e824a2235f935e5932ff21f2abd41356ddbbc`. It failed once at an earlier tree of step 12, see [Build plan step 12](#build-plan-step-12-fuzzing-and-the-ci-rule); here the first run passed | pass |
| `scripts/prove-checks.sh --with-linux-build` | every case rejects its change | exit 0 on `ecd0ee5`: `all checks reject their bad change`, cases 1 to 35 `PASS`, case 20 run with Docker | pass |
| CI run of `integration` and of `fuzz` | the jobs pass on the hosted runner | the jobs do not exist yet (manual step M2) | `NOT_RUN`, `owner: user` |

**The tree moved once more.** While this record was written, `main` gained `fd62cf4` (a lock file made by another process at the same moment is `state.locked`, in `apps/pool/src/state.rs` and `apps/pool/tests/commands.rs`). On `22182fb`, which holds it, the checks that this change can reach ran again: `cargo build --workspace --locked` exit 0 with no warning, the use check printed nothing, `scripts/check.sh all` exit 0 (`commands.rs` now 13 tests), `scripts/check.sh integration` exit 0 (21 of 21), `scripts/check.sh linux-build` exit 0 (binary `sha256 7ccc2e2ba0ba2b3df8f4acef60e937f2a0207c54caa80cd1e21c481072a9ca7a`), and `scripts/prove-checks.sh --with-linux-build` exit 0 (35 of 35, case 20 with Docker). `scripts/check.sh fuzz` was not run again: the change touches no code that a fuzz target reaches. The rows of the table below hold for both revisions.

**The declared environment.** The image is `postgres:18.7-bookworm@sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, as written once in `scripts/check.sh`. In a one-off container of that image, removed afterwards, `postgres --version` printed `postgres (PostgreSQL) 18.7 (Debian 18.7-1.pgdg12+2)` and `psql --version` printed `psql (PostgreSQL) 18.7 (Debian 18.7-1.pgdg12+2)`. Locked versions of the six adopted dependencies and the two native libraries: `tokio` 1.54.1, `rustls` 0.23.46, `tokio-rustls` 0.26.6, `aws-lc-rs` 1.19.0 (`aws-lc-sys` 0.47.0), `rusqlite` 0.42.0 (`libsqlite3-sys` 0.40.0), `tracing` 0.1.44. This macOS run is evidence for the behavior against real PostgreSQL 18, not Linux evidence; the Linux evidence of the fixture is the CI job `integration`, which is `NOT_RUN`.

#### Evidence per criterion

Each row names what proves the criterion on the final tree and the part of this record that holds the details. The checkers' verdicts are theirs and are not repeated here.

| Criterion | What proves it at `ecd0ee5` | Details | Result |
|---|---|---|---|
| `0004 AC-1` | `cargo metadata`, `Cargo.lock`, `cargo build --workspace --locked`, the use check, `scripts/check.sh guards` (rows above) | [Milestone 1](#milestone-1-guards-workspace-shape-and-dependencies), [step 10](#build-plan-step-10-the-daemon) | pass on the workstation |
| `0004 AC-2`, `0004 AC-4` | the `pool-repo-check` tests and `scripts/prove-checks.sh` | [Milestone 1](#milestone-1-guards-workspace-shape-and-dependencies) | pass on the workstation |
| `0004 AC-3` | `scripts/check.sh deny` (inside `all`) and `scripts/check.sh linux-build` | [Milestone 1](#milestone-1-guards-workspace-shape-and-dependencies), [step 12](#build-plan-step-12-fuzzing-and-the-ci-rule) | pass on the workstation and in the Linux image |
| `0004 AC-5` to `0004 AC-8` | `pool-config` tests and `check_config.rs` (inside `all`) | [Milestone 2](#milestone-2-configuration-and-wire-format) | pass |
| `0004 AC-9`, `0004 AC-13` | `commands.rs` (inside `all`) and the secret tests of `pool-core`; for `0004 AC-13` the states `unconfirmed`, `mismatch` and `restore_pending`, which need a real journal, in `integration.rs` (inside `integration`; corrected on 2026-11-16, from the full verification) | [step 7](#build-plan-step-7-secrets-tls-and-the-backend-connection), [step 8](#build-plan-step-8-bootstrap-the-authority-command-and-the-first-run-against-postgresql), [step 10](#build-plan-step-10-the-daemon) | pass |
| `0004 AC-10` | `pool-control` `journal.rs` (inside `all`) and `integration` | [step 6](#build-plan-step-6-authority-on-disk-and-the-control-loop), [step 8](#build-plan-step-8-bootstrap-the-authority-command-and-the-first-run-against-postgresql) | pass |
| `0004 AC-11`, `0004 AC-12` | `integration`, with `pool-control` `controller.rs` for the library half | [step 6](#build-plan-step-6-authority-on-disk-and-the-control-loop), [step 8](#build-plan-step-8-bootstrap-the-authority-command-and-the-first-run-against-postgresql) | pass on PostgreSQL 18.7 in the container |
| `0004 AC-14` to `0004 AC-17`, `0004 AC-19`, `0004 AC-20`, `0004 AC-22`, `0004 AC-23`, `0004 AC-25` | `pool-core` tests for the hermetic half (inside `all`), `integration` for the half on the fixture | [step 9](#build-plan-step-9-the-session), [step 10](#build-plan-step-10-the-daemon), [step 11](#build-plan-step-11-limits-loss-and-denial-on-the-fixture) | pass in both halves; the CI half `NOT_RUN` |
| `0004 AC-18`, `0004 AC-21`, `0004 AC-24` | `integration` | [step 10](#build-plan-step-10-the-daemon), [step 11](#build-plan-step-11-limits-loss-and-denial-on-the-fixture) | pass on PostgreSQL 18.7 in the container; the CI half `NOT_RUN` |
| `0004 AC-26` | `integration` with Docker (exit 0) and without a daemon (exit 3) | rows above, [step 8](#build-plan-step-8-bootstrap-the-authority-command-and-the-first-run-against-postgresql) | pass |
| `0004 AC-27` | `scripts/check.sh fuzz` | [step 12](#build-plan-step-12-fuzzing-and-the-ci-rule) | pass on the workstation; the CI job `fuzz` `NOT_RUN` (M2) |
| `0004 AC-28` | `pool-protocol` tests (inside `all`) | [step 4](#step-4-the-wire-format) | pass |
| `0003 AC-17`, `0003 AC-18` | `pool-control` tests (inside `all`); the guard half of `0003 AC-18` in `scripts/check.sh guards` | [step 6](#build-plan-step-6-authority-on-disk-and-the-control-loop), [Milestone 1](#milestone-1-guards-workspace-shape-and-dependencies) | pass |
| `0003 AC-19` | `scripts/check.sh guards` and its case in `scripts/prove-checks.sh` | [Milestone 1](#milestone-1-guards-workspace-shape-and-dependencies) | pass |

**Messages proved only hermetically.** Under the rule "A golden message is not proof of a wire format" of [The integration check](index.md#the-integration-check), these stay hermetic because no real client of the fixture can be made to send them: a `GSSENCRequest`, a `CancelRequest` and a startup packet with another major version (`gssenc_is_answered_with_n_and_a_version_3_2_gets_negotiation`, `a_cancel_request_is_read_and_dropped`, `refused_versions_and_parameters` in `pool-core/tests/session.rs`, and the golden tests of `pool-protocol`), a `_pq_.` parameter, whose `NegotiateProtocolVersion` answer is compared with bytes captured from PostgreSQL 18.7 (`negotiation_carries_the_whole_protocol_code_as_postgresql_18_7_sends_it`), another SCRAM header, a COPY response of the tag `W`, and an authentication message of 1025 bytes. These seven are the list of [step 11](#build-plan-step-11-limits-loss-and-denial-on-the-fixture). Every other message that POOL sends to a client or expects from a node has had a real `psql` 18.7 or PostgreSQL 18.7 on the other end in `integration`. (Corrected on 2026-11-16, from the full verification: the first version of this paragraph named four of the seven.)

## Test it

The `Test it` box of feature 4 (`/test one real primary path`), on 2026-11-15 from 20:00 to 22:45 WIB, by the tester session, on `main` at `ec85719` and the six test commits of its branch (`7cc36d2`, `e6c6c36`, `028eee1`, `0f97302`, `306e5c6`, and ``7008b55`` for the `covers` lines of `0004 AC-28`). macOS 27.0.1, arm64, Docker 29.9.2 running; a workstation, not Linux and not CI, except where a row says the pinned Linux image. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`. No line of production code, script, manifest or baseline changed: the branch touches test files only, plus tests appended at the end of the existing `#[cfg(test)]` modules of `crates/pool-core/src/secret.rs` and `crates/pool-core/src/scram.rs`.

### What was added

The gaps that the checker runs of this record named, in their order. Each new test is named after the criterion it guards (`ac<n>_` or a `covers: 0004 AC-n` line).

| Gap | Tests | File |
|---|---|---|
| 1. The four comparisons of the second route check | `ac14_a_second_look_that_names_another_node_refuses_with_57p03`, `..._with_another_topology_epoch_...`, `..._with_another_authority_revision_...`, `..._at_a_route_that_went_stale_...`, `..._at_a_closed_route_...`, and the positive control `ac14_the_same_route_published_again_or_only_fresher_lets_the_session_open`. Each refusal also checks that the opened backend is closed again and got no SQL | `crates/pool-core/tests/session.rs` |
| 2. Log events | A `tracing::Subscriber` of the test keeps every event. `ac24_session_events_carry_exactly_the_fields_of_the_spec_and_ids_in_place_of_names` (the seven fields of `session.started` and the four of `session.ended` in order, the principal by its ID, the lease generation 1, 2, 3, the session ID with the start sequence in its high half), `ac24_the_end_reason_and_status_name_how_a_session_ended`, `ac24_refusals_are_written_once_per_reason_with_a_count_and_no_name`, `ac24_no_event_of_a_whole_run_holds_a_name_a_password_a_host_a_path_or_sql` (a canary search over every field of every event) | `crates/pool-core/tests/session.rs` |
| 3. Endings, leases, limits, mechanisms, shutdown | `ac23_a_node_that_closes_in_the_middle_of_a_message_closes_the_client_with_no_error`, `ac14_a_positive_conflict_in_the_middle_of_a_message_...`, `ac25_the_shutdown_deadline_in_the_middle_of_a_message_...` (the client gets exactly the bytes of the node and then the end: no error after a cut message), `ac14_a_positive_conflict_closes_the_backend_of_every_session_it_ends`, `ac25_shutdown_closes_the_backend_of_every_session_it_ends`, `ac20_the_limit_of_a_principal_admits_exactly_its_count`, `ac16_a_mechanism_other_than_scram_sha_256_gets_08p01_and_no_server_first_message` (five names), `ac25_shutdown_closes_connections_before_their_session_without_waiting_for_the_startup_deadline` (startup deadline 60 s) | `crates/pool-core/tests/session.rs` |
| 4. `pool-core` step 7 | `a_directory_with_mode_0700_is_not_a_secret_file`, `a_secret_file_that_its_owner_may_not_read_is_secret_permissions` (mode `0200`; skipped for root), `a_password_of_1024_characters_passes_and_1025_do_not`, `the_key_and_certificate_files_are_read_up_to_their_limit_and_not_one_byte_more` (65536 and 1048576 bytes, then one more), `the_salt_of_an_unknown_name_depends_on_the_mock_key_and_the_name_and_is_stable`; in the backend tests: a startup message of exactly 16384 bytes and 16385, the parameter bytes at 16384 and 16385, a probe answer of exactly 4096 bytes of bodies across messages and 4097, a second `BackendKeyData`, four `Authentication` codes in place of `AuthenticationOk`, an error inside the one row query, kept keys per salt and count, `Display` of every `BackendError` | `crates/pool-core/src/secret.rs`, `scram.rs` (appended to `mod tests`), `crates/pool-core/src/backend/tests.rs` |
| 5. `pool-control` step 6 | `ac10_a_header_with_a_wrong_free_page_count_is_unreadable` (damage that only `quick_check` sees; the rows can still be read), `ac10_open_ready_does_not_wait_for_a_writer_that_holds_the_lock` and `ac10_the_reader_does_not_wait_for_a_connection_that_holds_the_file_exclusively` (both under 200 ms), `ac10_a_journal_file_that_cannot_be_written_is_still_read`, `ac12_a_probe_row_of_exactly_4096_bytes_of_text_is_read_and_one_byte_more_is_not`, `ac12_the_probe_query_is_the_text_of_the_spec` (`PROBE_QUERY` against the block under [The probe](index.md#the-probe)); in `commands.rs`, `ac10_a_journal_that_was_cut_or_whose_marks_changed_blocks_bootstrap_and_is_left_as_it_is` (a real journal made with the library, cut to one page, with another application ID, with a newer version; bootstrap refused with `bootstrap.store_present`, `show-authority` says `unreadable`, the bytes unchanged) | `crates/pool-control/tests/journal.rs`, `controller.rs`, `apps/pool/tests/commands.rs` |
| 6. `pool-protocol` | `ac23_every_tag_from_the_client_gets_the_action_of_its_row` and `ac23_every_tag_from_the_backend_gets_the_action_of_its_row` (all 256 tags, several lengths, the rows written out again from the spec), `ac23_terminate_with_any_length_but_4_is_08p01`, `ac15_every_order_of_up_to_four_first_packets_follows_the_rule_of_the_spec` (2930 steps against a model written from the text of [Startup](index.md#startup)), `ac15_negotiation_bytes_for_a_request_with_a_pq_option_match_a_real_server` (the body PostgreSQL 18.7 sent for `_pq_.zq9`, from the checker run of the changes to step 4). `covers: 0004 AC-28` on the three split and reservation tests that had no ID | `crates/pool-protocol/tests/golden.rs` |
| 7. Guard and fixtures | `ac2_the_committed_w3_baseline_is_the_block_of_spec_0004`, `ac2_the_committed_w3_baseline_allows_and_refuses_each_text_where_the_spec_says` (33 files and texts on a tree with the committed baseline, through the real binary, `apps/pool-cli` and `crates/pool-core-x` included); `ac5_every_fixture_gives_the_result_its_name_prefix_promises` (every file of the fixture folder, both formats), `ac5_each_new_number_key_takes_its_range_edges_and_refuses_one_past_them` (twelve keys, the expiry cross rule, `listener.port`, the principal limit), `ac6_raw_c1_direction_and_separator_characters_in_names_and_paths_are_refused_without_the_canary` (twelve raw characters in four keys) | `tools/repo-check/tests/repo_check/workspace.rs`, `apps/pool/tests/check_config.rs` |
| 8. The lock | `ac9_a_lock_held_by_another_pool_process_gives_state_locked_and_changes_nothing` (a `pool bootstrap` holds the lock; its connection to the silent node of the test is the signal that it does), `ac9_bootstraps_started_together_on_a_new_state_directory_have_one_holder_and_leave_only_the_lock` (three rounds of eight, released together through named pipes: one refused for missing evidence, seven `state.locked`, one connection to the node, only `pool.lock` with mode `0600` left) | `apps/pool/tests/commands.rs` |

### Test count

Test attributes (`#[test]` and `#[tokio::test]`) under `apps/`, `crates/` and `tools/repo-check/`, fuzz targets and doc tests not counted.

| File | Before (`ec85719`) | After |
|---|---|---|
| `crates/pool-core/tests/session.rs` | 29 | 47 |
| `crates/pool-core/src/backend/tests.rs` | 16 | 24 |
| `crates/pool-core/src/secret.rs` | 7 | 11 |
| `crates/pool-core/src/scram.rs` | 7 | 8 |
| `crates/pool-control/tests/journal.rs` | 19 | 23 |
| `crates/pool-control/tests/controller.rs` | 12 | 14 |
| `crates/pool-protocol/tests/golden.rs` | 11 | 16 |
| `apps/pool/tests/check_config.rs` | 86 | 89 |
| `apps/pool/tests/commands.rs` | 13 | 16 |
| `tools/repo-check/tests/repo_check/workspace.rs` | 28 | 30 |
| All | 475 | 520 |

### Mutations

Each change was made in a copy of the tree outside the repository, one at a time, and the named tests were run against it twice: with the test files of `ec85719` ("before") and with the test files of this branch ("after"). A file that holds both code and tests was mutated from its committed text. "Caught" means the run failed; the test that failed is named.

| # | Change to the code | Tests run | Before | After, caught by |
|---|---|---|---|---|
| C1 | Second route check: the node is not compared (with the pool key, which holds the node) | `pool-core --test session` | survived | `ac14_a_second_look_that_names_another_node_refuses_with_57p03` |
| C2 | The same for the topology epoch | the same | caught | `ac14_a_second_look_with_another_topology_epoch_...`, `a_route_that_changed_while_the_backend_opened_refuses_at_step_9` |
| C3 | The authority revision is not compared | the same | survived | `ac14_a_second_look_with_another_authority_revision_...` |
| C4 | The second look does not ask for freshness | the same | survived | `ac14_a_second_look_at_a_route_that_went_stale_...` |
| C5 | The pool key comparison alone left out | the same | survived | survived: an equivalent change, because the node and the epoch, the only parts of the key a route can change, are compared beside it |
| C6 | `session.started` carries the principal name in place of its ID | the same | survived | `ac24_session_events_carry_exactly_...`, `ac24_no_event_of_a_whole_run_holds_...` |
| C7 | The same lease generation for every lease | the same | survived | `ac24_session_events_carry_exactly_...` |
| C8 | The limit of a principal raised by one | the same | survived | `ac20_the_limit_of_a_principal_admits_exactly_its_count` |
| C9 | Any SASL mechanism name accepted | the same | survived | `ac16_a_mechanism_other_than_scram_sha_256_...` |
| C10 | The lease is never dropped (`mem::forget`), so the backend stays open | the same | caught | six tests, among them `ac14_a_positive_conflict_closes_the_backend_of_every_session_it_ends` and `ac25_shutdown_closes_the_backend_of_every_session_it_ends` |
| C11 | An error is sent after a backend lost in the middle of a message | the same | survived | `ac23_a_node_that_closes_in_the_middle_of_a_message_...` |
| C12 | An error is sent after a cut in the middle of a message | the same | survived | `ac14_a_positive_conflict_in_the_middle_of_a_message_...`, `ac25_the_shutdown_deadline_in_the_middle_of_a_message_...` |
| C13 | Shutdown does not close connections before their session | the same | survived | `ac25_shutdown_closes_connections_before_their_session_...` |
| C14 | `session.ended` reports `idle` in place of `in_transaction` | the same | survived | `ac24_the_end_reason_and_status_name_how_a_session_ended` |
| S1 | A directory passes both type checks of a secret file | `pool-core --lib` | survived | `a_directory_with_mode_0700_is_not_a_secret_file` |
| S2 | A secret file that cannot be opened is `secret.invalid` | the same | survived | `a_secret_file_that_its_owner_may_not_read_is_secret_permissions` |
| S3 | The key file limit raised by one | the same | survived | `the_key_and_certificate_files_are_read_up_to_their_limit_...` |
| S4 | Every file limit lowered by one (`>=`) | the same | survived | the same |
| S5 | The password limit raised by one | the same | caught | `a_password_of_1024_characters_passes_and_1025_do_not`, `passwords` |
| S6 | The made up salt does not depend on the mock key | the same | survived | `the_salt_of_an_unknown_name_depends_on_the_mock_key_...` |
| B1 | A second `BackendKeyData` replaces the first | the same | survived | `a_second_backend_key_data_ends_the_attempt` |
| B2 | No limit on the parameter bytes | the same | survived | `the_parameters_of_the_node_stop_at_16384_bytes_...` |
| B3 | An error inside the one row query is dropped | the same | survived | `an_error_inside_the_one_row_query_is_a_server_error_with_its_sqlstate` |
| B4 | Kept keys are used for another salt | the same | survived | `kept_keys_are_used_only_for_their_own_salt_and_count` |
| B5 | Any `Authentication` message after the final one counts as `AuthenticationOk` | the same | survived | `an_authentication_message_after_the_final_one_that_is_not_ok_...` |
| B6 | `Display` of a backend error prints its `Debug` form | the same | survived | `display_of_a_backend_error_is_its_constant_kind_and_no_debug_form` |
| B7 | The limit of a startup message of the node raised by one | the same | survived | `a_startup_message_of_the_node_of_exactly_16384_bytes_...` |
| B8 | The probe answer is not counted across messages | the same | survived | `a_probe_answer_of_exactly_4096_bytes_of_bodies_...` |
| P1 | The reader without `PRAGMA quick_check` | `pool-control` | survived | `ac10_a_header_with_a_wrong_free_page_count_is_unreadable` |
| P2 | `busy_timeout` 300 ms on the open that writes | the same | survived | `ac10_open_ready_does_not_wait_for_a_writer_that_holds_the_lock` |
| P3 | `busy_timeout` 300 ms on the reader | the same | survived | `ac10_the_reader_does_not_wait_for_a_connection_that_holds_the_file_exclusively` |
| P4 | The reader opens the file for writing | the same | survived | survived: SQLite falls back to read only by itself when a file opened for writing is write protected, so the flag cannot be seen from outside |
| P5 | The probe answer limit raised by one | the same | survived | `ac12_a_probe_row_of_exactly_4096_bytes_...` |
| P6 | Another text for `PROBE_QUERY` | the same | survived | `ac12_the_probe_query_is_the_text_of_the_spec` |
| P7 | `synchronous = OFF` | the same | survived | survived: see the open gaps |
| P8 | `wal_autocheckpoint = 10` | the same | survived | survived: see the open gaps |
| R1 | `Terminate` with a length of 8 accepted | `pool-protocol --test golden` / whole package | survived / caught | `ac23_terminate_with_any_length_but_4_is_08p01` / and `message::tests::frontend_rows` |
| R2 | `f` (`CopyFail`) is not refused as unsupported | the same | survived / caught | `ac23_every_tag_from_the_client_...` / and `frontend_rows` |
| R3 | A notification (`A`) is not forwarded | the same | survived / caught | `ac23_every_tag_from_the_backend_...` / and `backend_rows` |
| R4 | A `ReadyForQuery` of any length from 5 up | the same | survived / caught | the same / and `backend_rows` |
| R5 | A `CancelRequest` keeps the state of the connection | the same | survived / caught | `ac15_every_order_of_up_to_four_first_packets_...` / and `startup::tests::handshake_order` |
| R6 | A COPY response `W` is backend loss | the same | survived / caught | `ac23_every_tag_from_the_backend_...` / and `backend_rows` |
| G1 | The baseline allows `std::thread` in `crates/pool-core` | `repo-check --test repo_check` | survived | both `ac2_the_committed_w3_baseline_...` tests |
| G2 | The baseline allows `tokio` in `crates/pool-control` | the same | survived | both |
| G3 | The baseline drops `deny crates/pool-control/src rusqlite` | the same | survived | both |
| G5 | `covers` compares by prefix, not by whole components | the same | survived | `ac2_the_committed_w3_baseline_allows_and_refuses_...` (`crates/pool-core-x`); the unit tests of `src/workspace.rs` were not part of this run |
| K1 | `limits.backend_wait_max_count` up to 100001 | `pool --test check_config` | survived | `ac5_each_new_number_key_takes_its_range_edges_...` |
| K2 | `limits.backend_acquire_timeout_ms` from 99 | the same | survived | the same |
| K3 | `limits.shutdown_timeout_ms` up to 599999 | the same | survived | the same |
| K5 | The control characters end at U+009E | the same | survived | `ac6_raw_c1_direction_and_separator_characters_...` |
| L1 | A held lock is not refused | `pool --test commands` | caught | four tests, among them `ac9_a_lock_held_by_another_pool_process_...` and `ac9_bootstraps_started_together_...` |
| L2 | The command that loses the race to create `pool.lock` gets `state.permissions` | the same | caught | `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked`, `ac9_bootstraps_started_together_...` |

For R1 to R6 the unit tests beside the code already caught every change; the new golden tests are a second net that reads the rows of the spec again, and catch each change on their own.

### Stability

`crates/pool-core/tests/session.rs` ran five times in a row after it was written: 47 of 47 each time. `crates/pool-control` ran ten times after the fix below: no failure. `apps/pool/tests/commands.rs` ran four times: 15 of 15 (16 with the journal test, which came after). One fault of my own was found by repetition and fixed before the commit: a new journal test used the folder name `read-only` of an existing test, so the two raced; in 5 of 8 runs one of them failed. It now has a name of its own.

### Gates on the final tree

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `7008b55`, between 22:24 and 22:39 WIB. The merge of `main` at `b8d5099` that followed (`d972c9b`) brought two documents only, the checker's record of the lock file fix and the coordination log, so these results hold for it. `scripts/check.sh docs` ran again on the record: `ok`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`. Tests: `pool` 13 unit, 87 `check_config`, 16 `commands` (21 integration ignored here); `pool-config` 32 unit, 123 `validate`; `pool-control` 5 unit, 3 `admission_model`, 14 `controller`, 23 `journal`, 28 `walkthroughs`; `pool-core` 50 unit, 47 `session`; `pool-protocol` 22 unit, 16 golden, 4 redaction; `pool-repo-check` 108 unit, 112 integration. The first run of this command, on `306e5c6`, failed in `guards` and in `cli::every_command_passes_on_the_committed_tree`, both for one comment of mine in `commands.rs` that named the text `std::thread`; `7008b55` changed the comment | pass |
| `scripts/check.sh integration` | exit 0, nothing left behind | exit 0: 21 of 21 against the pinned image in 39 s; no container of this run left afterwards | pass |
| `scripts/check.sh fuzz` | no crash file | exit 0: `validate` 618497 runs, `startup` 3045689, `frontend_message` 939021, `scram` 4124179, 31 s each; no file under any `fuzz/artifacts/` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0: host `aarch64-unknown-linux-gnu`, 22 test results `ok` and none failed, among them `session` 47, `commands` 16 and `golden` 16; every fixture with its expected exit; `pool 0.1.0`, `linux build check passed` | pass |
| `scripts/prove-checks.sh` | every case rejects its change | exit 0: `all checks reject their bad change (linux-build not proven)`, 34 cases `PASS`, case 20 skipped because the flag `--with-linux-build` was not given | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, the valid report of spec 0001 | pass |
| `git diff --stat main...HEAD` | test files and this record only | the ten test files of the table above and this record; 0 lines of production code, script, manifest or baseline. The 4 removed lines are harness lines of `backend/tests.rs` and `session.rs` | pass |

### Gaps that stay open

- **The four settings of every open that writes** (`synchronous`, `foreign_keys`, `wal_autocheckpoint`, `journal_size_limit`, P7 and P8) and **the read only flag of the reader** (P4). They live on the connection of the journal module, which no caller can reach, and SQLite reports them to that connection only. A test needs a way to ask the open connection, which is production code. A follow up for the owner of `crates/pool-control`, if the spec wants them locked by a test.
- **A certificate that has expired or is not valid yet.** It needs a certificate signed by one of the test CAs, whose keys are in no repository by the rule under [Tests and who owns them](index.md#tests-and-who-owns-them), or a second CA and key made at test time, which needs `openssl` or a dependency in the test. The checker's probe covered it.
- **Case 17 of `scripts/prove-checks.sh` on every status of scope 004.** The case is in the script, which this item may not change, and `scripts/prove-checks.sh` runs it on the status the tree has. A test in `tools/repo-check/` can only reach rule D3, not the script.
- **The pool key comparison alone (C5)** is an equivalent change, as the table says.
- **`0004 AC-26` and `0004 AC-27`** are proven by `scripts/check.sh integration` (also without a Docker daemon) and `scripts/check.sh fuzz`, not by a test that names them.
- **No new integration test.** The gaps above were all reachable without a container. `scripts/check.sh integration` ran unchanged, as a gate.

### Round two

On 2026-11-15 from 23:00 and 2026-11-16 until 00:15 WIB, by the tester session, on `main` at `b116bd2` and the commits `525c0e8`, `ac57b9a` and ``aaede74`` of its branch, with `main` merged again at ``1650321``. macOS 27.0.1, arm64, Docker 29.9.2 running; a workstation, not Linux and not CI, except the `linux-build` row. The source is the list "For the tester: wrong changes that no test caught" and the test gaps of [the checker run of steps 10 and 11, and of milestone 4](#checker-run-of-steps-10-and-11-and-of-milestone-4). No line of production code changed: the unit tests are appended at the end of the existing `mod tests` of `apps/pool/src/control.rs`, and `apps/pool/tests/support/mod.rs` gained two methods of the relay (`retarget` and `accept_tls_again`).

#### What was added

| Gap of the checker | Tests | File |
|---|---|---|
| M1, M4: the route view | `ac14_route_view_marks_a_positive_conflict_and_not_silence` (a second primary gives `Closed { conflict: true }`, a writer with no sample `Closed { conflict: false }`), `ac14_an_open_route_is_fresh_exactly_until_its_snapshot_says` | `apps/pool/src/control.rs` |
| M2: published before the disk (decision 12) | `ac14_the_conflict_view_is_published_before_the_latch_is_written`: the round is polled exactly once with a waker that does nothing; the latch write is still on its blocking thread, and the view with the conflict is already out. (A first version used `tokio::time::timeout` with a zero duration; the timer rounds up to the next millisecond, the write was done by then, and M2 survived. The single poll catches it.) | `apps/pool/src/control.rs` |
| M3 and "the journal write through `control.rs`" | `ac14_a_second_primary_latches_the_cluster_in_the_journal_through_the_control_loop`: a real journal under the temporary folder, two nodes, `pg2` reports a second primary; after one round the latch is on disk (a new `Controller` from the file closes with `authority.conflict_latched` although both nodes look healthy), and `authority.conflict_latched` is logged with `recorded` false, then true | `apps/pool/src/control.rs` |
| "A journal write that fails while the daemon runs" | `ac14_a_latch_write_that_fails_is_logged_and_the_latch_holds_in_memory`: a second holder of the journal moves the authority revision on, so each write of the loop fails with `journal.changed`; two rounds log `journal.write_failed` twice and the unrecorded line once, and the cluster stays closed with the conflict flag | `apps/pool/src/control.rs` |
| M12 | `ac10_a_run_that_stops_at_a_missing_protected_file_does_not_use_up_a_start_sequence`: a ready journal made with the library, the listener certificate missing; two runs exit 3 with `secret.not_found`, and the journal file is the same byte for byte | `apps/pool/tests/commands.rs` |
| M8, M9 | `it_ac12_a_probe_that_lost_its_connection_opens_a_new_one_and_logs_each_class_once`: the connection of the monitor login is ended with `pg_terminate_backend`; the probe logs `probe.failed`, then `probe.recovered`. Then the relay refuses TLS to new connections for two seconds (about ten rounds): one `probe.failed` line with the class `tls`, three `probe.failed` lines in all, two `probe.recovered` | `apps/pool/tests/integration.rs` |
| "Silence does not end a running session" | `it_ac14_a_writer_that_falls_silent_keeps_running_sessions_and_refuses_new_logins`: a session runs, the probe loses the node; a new `admission.changed` closed with `evidence.writer_missing` alone; the running session answers `SELECT 2`, a new login gets `57P03`, and no `session.ended` is written | `apps/pool/tests/integration.rs` |
| M1 through the daemon, "a running session ended for a positive conflict" | `it_ac14_a_writer_that_changes_its_identity_ends_running_sessions_through_the_daemon`: the relay sends the probe's next connection to container 2 (another system identifier) while the session stays on container 1; `evidence.writer_identity`, then the idle session gets `57P01` within 10 seconds and `session.ended` names `conflict` and `idle`. A second primary with the same system identifier, and so the latch, cannot be made with the two containers of the fixture; the unit tests above cover it | `apps/pool/tests/integration.rs`, `support/mod.rs` |
| M6, "`pool run` on the hold `restore_pending`" | `it_ac13_run_refuses_a_journal_on_hold_for_a_restore_and_never_binds`: no command of this slice sets the hold, so the test sets it with the `sqlite3` tool of the host; `show-authority` says `restore_pending`, `pool run` exits 1 with `authority.restore_pending`, never binds, and the file does not change | `apps/pool/tests/integration.rs` |
| "Only the fields of the table, a canary in the error text of PostgreSQL" | `it_ac24_the_log_holds_only_the_fields_of_the_table_and_no_text_from_postgresql`: `CANARY-9f2c7b1e` in a `RAISE EXCEPTION`, in a cast error and as a wrong password; the client sees the error text, the log never does; every line is an event of the table under [Log events](index.md#log-events) with only its fields, each a string, a whole number or a boolean | `apps/pool/tests/integration.rs` |

Test count: `apps/pool/src/control.rs` 1 to 6, `apps/pool/tests/commands.rs` 16 to 17, `apps/pool/tests/integration.rs` 21 to 26; 11 more in all.

#### Mutations M1 to M12

Each change was made in a copy of the tree outside the repository, one at a time, and run against `cargo test -p pool --locked` and then `scripts/check.sh integration` (the second only when the first passed). "Before" is the result of the checker against the tests of `d4fdb62`; `apps/pool/tests/integration.rs`, `tests/support/` and `apps/pool/src/` did not change between `d4fdb62` and `b116bd2`. The tests of round one added nothing that reaches these changes: at `b116bd2` all twelve passed `cargo test -p pool --locked`.

| # | Wrong change | Before | After, caught by |
|---|---|---|---|
| M1 | `route_view` never sets the conflict flag | not caught | the four unit tests of `control.rs` that see a conflict, among them `ac14_route_view_marks_a_positive_conflict_and_not_silence` |
| M2 | The view is published after the latch writes | not caught | `ac14_the_conflict_view_is_published_before_the_latch_is_written` |
| M3 | A latch is never written | not caught | `ac14_a_second_primary_latches_the_cluster_in_the_journal_through_the_control_loop`, `ac14_a_latch_write_that_fails_...`, `ac14_the_conflict_view_is_published_...` |
| M4 | An open route stays fresh one hour more | not caught | `ac14_an_open_route_is_fresh_exactly_until_its_snapshot_says` |
| M5 | `mismatch` reported as `authority.unconfirmed` | caught | `it_run_refuses_a_journal_without_or_with_another_witness_and_never_binds` (as before) |
| M6 | `restore_pending` reported as `authority.unreadable` | not caught | `it_ac13_run_refuses_a_journal_on_hold_for_a_restore_and_never_binds` |
| M7 | Shutdown does not end the sessions | caught | the four new fixture tests, each within the 20 second limit of its stop (run with the new tests alone). The existing `it_shutdown_ends_idle_sessions_with_57p01_and_busy_ones_with_08007` does not fail but hangs, because `Daemon::stop` waits without a limit; the first run with the whole file was ended by the 900 second limit of the mutation runner |
| M8 | The probe keeps its connection after a failure | not caught | `it_ac12_a_probe_that_lost_its_connection_opens_a_new_one_...`, `it_ac14_a_writer_that_changes_its_identity_...` |
| M9 | `probe.failed` for every failed sample | not caught | `it_ac12_a_probe_that_lost_its_connection_..._logs_each_class_once` |
| M10 | `admission.changed` only when open or closed changes | caught | `it_ac14_a_writer_that_changes_its_identity_...`, and `it_the_wrong_identity_closes_admission_before_any_login` as before |
| M11 | The refusal counters are never flushed while serving | caught | `it_ac24_the_log_holds_only_the_fields_...`, and `it_waiters_are_limited_and_time_out_after_the_acquire_deadline` as before |
| M12 | The start sequence is advanced before the protected files are read | not caught | `ac10_a_run_that_stops_at_a_missing_protected_file_does_not_use_up_a_start_sequence` |

All twelve are caught. The order that M2 breaks, which the checker read in the code only, is now held by a test.

#### Faults found in the tests, and stability

- **My silence test, fixed in `ac57b9a`.** The daemon writes one `admission.changed` closed with `evidence.writer_missing` before its first sample, and the first version waited for any such line, so it found that one, and a login could still meet the open route and get `08006`. Run alone, it failed in 2 of 3 runs, and in the first mutation run it failed beside each change. It now counts the lines first and waits for a new one. Then: 5 of 5 runs alone, and the mutation runs of M5, M6, M9, M10 and M11 were done again.
- **A named pipe that never ended, in my bootstrap race test of round one, rewritten in ``aaede74``.** On macOS, during a plain `cargo test -p pool --locked --test commands` while the machine was busy, one `pool bootstrap` of the eight had read its whole configuration from its named pipe (707 of 707 bytes, seen with `lsof`) and waited in `read` for an end of file, with no writer left open. The test then waited for it with no limit, for 632 seconds until I ended the child. This is the hang that the checker reported once in 137 runs for `ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked`; the product only reads its file. The test now starts the eight one after another, which still meets a held lock, because the first holds it for the probe timeout of 3 seconds, and waits for each child, and for the holder of the other lock test, at most 20 seconds. After the change: 5 of 5 and, after the merge with the time limit that `dev-f4-m2` gave to the original test, 3 of 3. The original test, which still uses pipes, catches L2 of round one; the rewritten one no longer races the creation of `pool.lock`.
- **One failure that did not come back.** In one mutation run of M11, `ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds` of `commands.rs` failed, which M11 cannot reach. The output was not kept. It passed in the next 2 runs with M11 and in 8 plain runs. Seen once, cause not established; it has a limit of 5 seconds on the time of `pool run`, and the machine was busy.
- The unit tests of `control.rs` ran 5 times (6 of 6 each), the new fixture tests ran in two whole runs of `scripts/check.sh integration` and in the mutation runs.

#### Gates on the final tree

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `1650321` (the branch with `main` at `19eea61` merged), on 2026-11-16 from 00:00 to 00:09 WIB.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok`; `pool` 18 unit tests (6 of them in `control.rs`), 87 `check_config`, 17 `commands`, 26 integration tests ignored here | pass |
| `scripts/check.sh integration` | exit 0, nothing left behind | exit 0: 26 of 26 in 42 s against the pinned image; no `pool-it-` container afterwards | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0: host `aarch64-unknown-linux-gnu`, 22 test results `ok`, none failed, `linux build check passed` | pass |
| `git diff --stat main...HEAD` | test files and this record only | `apps/pool/src/control.rs` (lines added at the end of `mod tests` only), `apps/pool/tests/commands.rs`, `apps/pool/tests/integration.rs`, `apps/pool/tests/support/mod.rs` and this record; 0 lines of production code | pass |

During the mutation runs one run with M7 was ended by the limit of the runner while `scripts/integration/run.sh` waited for its test; its two containers (`pool-it-1-74706`, `pool-it-2-74706`, both of my run) stayed until I ended the mutated `pool run` with SIGKILL, after which the script removed them itself.

#### Gaps that stay open after round two

- **A second primary and the latch on the fixture.** It needs a second PostgreSQL with the same system identifier (a copy of container 1 made with `pg_basebackup` and promoted), which `scripts/integration/` does not make. The path through `control.rs` is held by the unit tests above, with a real journal.
- **`Daemon::stop` of `integration.rs` waits without a limit.** M7 shows it: the existing shutdown test hangs instead of failing. The new tests use `stop_within(20)`. `Daemon::stop` is called 13 times and `stop_after_signal` twice in the existing tests; giving them a limit is a small change of the helper, left as a follow up for the lead to assign.
- **The hold `restore_pending` needs the `sqlite3` tool on the host** of `scripts/check.sh integration`. It is on this workstation (`sqlite3` 3.54.0). Whether the hosted runner of the CI job `integration` has it was not checked, because that job does not exist yet (manual step M2). Without it the test fails with a message that names the tool.

### Round three

On 2026-11-16 from 00:20 to 01:35 WIB, by the tester session, on `main` at `b2aec30` and the commit `331d64e` of its branch, with `main` merged at `0503bba`. macOS 27.0.1, arm64, Docker 29.9.2 running. Three small items from the full verification of [the tests the tester added](#the-tests-the-tester-added) and [the two tests that were once unstable](#the-two-tests-that-were-once-unstable). Test files only: `apps/pool/tests/integration.rs`, `apps/pool/tests/commands.rs` and `apps/pool/tests/support/mod.rs`. The lock race test of `9765845` is unchanged.

#### 1. Every log of the daemon is checked

`Daemon::stop`, `Daemon::stop_after_signal` and `Daemon::stop_within` of `integration.rs` now end in one function, `finish`. After the daemon exited, it checks the whole log the way `it_ac24_the_log_holds_only_the_fields_of_the_table_and_no_text_from_postgresql` did for one run: every line is an event of the table under [Log events](index.md#log-events) with only its fields, each a string, a whole number or a boolean (`assert_log_fields`), and no line names the folder of the test, the fixture folder, `pz7`, `127.0.0.1`, `pg.pool.test`, `host.docker.internal`, three SQL words, the start of a verifier or a password of the fixture (`assert_log_text_is_clean`, the old `assert_log_is_clean` for a folder). Each of the 17 tests that run the daemon ends it through one of the three, so each of the 18 runs of the daemon in `scripts/check.sh integration` is checked, the runs that make the probe fail among them. A check in `Drop` was not used: it would have to know whether the test is already failing, and the only way to ask that, `std::thread::panicking`, is a text that rule W3 refuses in the tests too.

| Wrong change | Tests | Before (`b2aec30`) | After |
|---|---|---|---|
| T6. `probe.failed` carries a field `host` with the address of the node (`host = "127.0.0.1"`) | `scripts/check.sh integration` | not caught | caught by `it_a_wrong_ca_or_name_toward_the_node_closes_admission`, `it_ac12_a_probe_that_lost_its_connection_...`, `it_ac14_a_writer_that_changes_its_identity_...`, `it_ac14_a_writer_that_falls_silent_...` |

#### 2. No wait on the daemon without a limit

- `Daemon::stop` and `Daemon::stop_after_signal` (13 and 2 calls in the tests from before round two) wait at most 45 seconds, which is more than the default `limits.shutdown_timeout_ms` of 30 seconds. A daemon that is still running then is killed, and the test fails with `the daemon did not exit within 45 s after SIGTERM from Daemon::stop; it was killed. Its log:` and the log (or `after the signal that the test sent` for the second). `stop_within(20)` of round two uses the same function.
- That alone did not stop M7 from hanging: the shutdown test sends SIGTERM itself and then reads the error of each session with `support::next_error`, which waited for ever on a session that is never ended. `support::next_error` now waits at most 30 seconds and fails with `an error message or the end of the stream within 30 s`.

| Wrong change | Tests | Before | After |
|---|---|---|---|
| M7. Shutdown does not end the sessions (`server.shutdown()` replaced by a future that never ends) | `it_shutdown_ends_idle_sessions_with_57p01_and_busy_ones_with_08007` alone, through `scripts/check.sh integration` | hung: ended by the limit of the mutation runner after 1500 s; its containers stayed until I ended the mutated daemon and the test, after which `scripts/integration/run.sh` removed them | caught: the test failed after 38 s |

#### 3. `ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds`

The reading of the checker, tested:

- **Can another test be handed the port?** On macOS the kernel gives the ports of `bind` to port 0 in order, for the whole machine: after a port was released, it came back only after 16350 to 16352 more binds, in 200 of 200 trials (a Python probe, `bind` and `close` in a loop). Within a run of `commands`, which makes a few dozen such binds, the port of `ac13` cannot come back to another test of the same binary. In the pinned PostgreSQL image (Linux, Debian bookworm, with `perl`), a released port was handed to one of the next 20 binds, held at the same time, in 5 of 3000 rounds. So on Linux the race is real, and on macOS, where both failures were seen, it is not the likely cause.
- **What it looks like.** With the old test, a listener that another test opens on that port right after `closed_port()` (made on purpose in a copy) makes the test fail at the check that the port is still free, although `pool run` did nothing wrong (R1 below).
- **The change.** The test now opens its own listener on the port of the configuration and holds it through the four runs of `pool run`. A `pool run` that tried to bind would fail with `listener.failed` and exit 3, so the check is stronger, not weaker (B1 below). The run is waited for at most 30 seconds. Every assertion of a run prints the whole output and the time the run took: `exit <code>, after <duration>`, then stdout and stderr. The limit of 5 seconds on a run stays, and its message now shows the time.
- **The other candidate**, the limit of 5 seconds on a busy machine, is not shown either. If it is the cause, the next failure prints the time.

| Change | Tests | Before | After |
|---|---|---|---|
| R1. Another test is handed the port that `ac13` released (a listener opened by the test on that port, in a copy of the old test) | `cargo test -p pool --locked --test commands ac13_run_refuses_every_state` | the test fails, though the product is right | does not apply: the port is held by the test for the whole run |
| B1. `pool run` binds the listener before it reads the journal (in `daemon.rs`, before step 3) | `cargo test -p pool --locked --test commands` | not caught | caught by `ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds` |

#### Repeated runs of `commands`

The whole test binary of `apps/pool/tests/commands.rs` at `331d64e`, run again and again; the output of every failed run was kept.

| Where | Runs | Passed | Failed |
|---|---|---|---|
| macOS 27.0.1, arm64, with the Linux runs below going on in Docker beside it | 50 | 50 | 0 |
| The pinned image `rust:1.100.0-slim-bookworm` of `scripts/linux-build.sh` (Linux, arm64), as root, built from `git archive` of `331d64e` | 50 | 50 | 0 |

No run failed, so no output had to be kept. Each run holds 17 tests and takes about 10 seconds. `ac13` was not seen failing in these 100 runs or before them in this round; if it fails again, its message now holds the output and the time.

#### Gates

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `0503bba` (`331d64e` with `main` at `4285f8b` merged), on 2026-11-16 from 01:26 to 01:29 WIB.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 0: fmt, clippy, test, deny, guards and docs all `ok` | pass |
| `scripts/check.sh integration` | exit 0, nothing left behind | exit 0: 26 of 26 in 43 s, each of the 18 runs of the daemon checked by `finish`; no `pool-it-` container afterwards | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0: 22 test results `ok`, none failed, `linux build check passed` | pass |
| `git diff --stat main...HEAD` | test files and this record only | `apps/pool/tests/commands.rs`, `apps/pool/tests/integration.rs`, `apps/pool/tests/support/mod.rs` and this record; 0 lines of production code | pass |

**For the developer of `scripts/integration/` (finding g4 of the review).** The two runs of this round and of round two that hung, M7 both times, were ended from outside, and their containers stayed until the daemon and the test were killed, because `run.sh` removes them only when the test command returns. With the limits of this round a run of the tests can no longer hang on a daemon that does not stop, so that path is now reached only by a signal to the script itself.

### Round four

On 2026-11-16 from 01:40 to 01:55 WIB, by the tester session: the test findings of [the review of scope 004](../../reviews/2026-11-16-scope-004-one-real-primary-path.md) (M3, t1 to t10, g4), on `main` at `4285f8b` and later, with the commits `fd84e4d` (M3) and `486db56` (the minor findings) of its branch, and `main` merged at `0c0424c`. macOS 27.0.1, arm64, Docker 29.9.2 running. Test files only, plus tests appended at the end of the existing test modules of `apps/pool/src/control.rs`, `crates/pool-core/src/records.rs` and `crates/pool-core/src/backend/tests.rs`. `crates/pool-core/tests/session.rs` was not touched: the developer of finding c1 works in it.

#### M3: a second primary beside a silent writer

| Test | What it holds |
|---|---|
| `ac14_a_second_primary_beside_a_silent_writer_is_a_conflict_although_silence_is_not` (`control.rs`) | `pg1` with a failed sample, or with none, beside `pg2` that reports as a primary: the reasons hold `evidence.writer_missing` and `evidence.second_primary` (and the latch that the second primary sets in memory in the same round), and the route view is `Closed { conflict: true }` |
| `ac14_a_latched_journal_with_an_empty_board_is_a_conflict` (`control.rs`) | a journal with the latch on disk and no sample yet: `authority.conflict_latched` among the reasons, and the conflict flag |
| `it_ac14_a_second_primary_beside_a_silent_writer_ends_sessions_and_latches_the_journal` (`integration.rs`) | two node entries of one cluster reach container 1 through two relays, so both are primaries with one system identifier and no base backup is needed. `pg2` is silent at first; the daemon opens on `pg1` and a session runs. Then `pg1` falls silent and `pg2` answers: `admission.changed` closed with both reasons, the running session gets `57P01`, `authority.conflict_latched` with `recorded: true`. A new start with a healthy writer and a silent `pg2` stays closed with `"reasons":"authority.conflict_latched"`, and a login gets `57P03`. 5 of 5 runs alone, about 1.2 s each |

| Wrong change | Tests run | Before | After, caught by |
|---|---|---|---|
| The reviewer's: `any` becomes `all` in `route_view` (`control.rs:54`) | `cargo test -p pool --locked` | not caught (all passed) | both new unit tests; and the new fixture test, run alone |
| M3 of round two: a latch is never written | `scripts/check.sh integration`, the new test alone | not caught on the fixture | the new fixture test |

This also closes the gap "a second primary and the latch on the fixture" that round two left open.

#### The minor findings

| Finding | What was done | Wrong change, before → after |
|---|---|---|
| t1, t3 | Not done: both are in `crates/pool-core/tests/session.rs`, which another session holds for finding c1. Left until the lead hands the file over | not run |
| t2 | `assert_hidden` of `crates/pool-protocol/tests/redaction.rs` now also refuses the canary written as a list of numbers, which is how a derived `Debug` prints bytes. The exact text of `StartupRequest` and `Forwarded`, the alternate form of `ParameterStatus` and the two readers in the middle of a body. The exact `Debug` of `VerifiedPrincipal` (`records.rs`) and of `Credential` (`backend/tests.rs`). Every log of the daemon is checked since round three, and the listen host (`POOL_IT_LISTEN_HOST`) is now in the forbidden list. Not done: `Lease`, `ServerParts` and `Server` need a real TLS connection or live in `session.rs` | `#[derive(Debug)]` on `StartupRequest` (D1): not caught → caught by `the_startup_types_print_exactly_...` and `startup_values_and_a_reader_...`. On `ParameterStatus` (D2): not caught → caught by two tests. On `VerifiedPrincipal` (V1): not caught → caught by `a_verified_principal_prints_its_id_and_never_its_name` |
| t4 | For the files of this item: since round three the stop helpers and `support::next_error` have limits. Now also `pool` (60 s), `psql` as a client, in the background and as the superuser (60 s), and the two `pool run` of the refusal tests (30 s), each failing with a message that names the step, and `connect_timeout=10` in both connection strings. The scripted client of `session.rs` is the other half, not done for the reason above. A limit on the whole test command in `scripts/integration/run.sh` is a script change (see g4) | M7 of round two: hung → failed in 38 s (round three) |
| t5 | Done in round three for `commands.rs`. Now also the two checks of `integration.rs`: `it_run_refuses_a_journal_without_or_with_another_witness_and_never_binds` and `it_ac13_run_refuses_a_journal_on_hold_...` hold a listener on the port for the whole run | B1, `pool run` binds before it reads the journal: not caught by either → caught by both |
| t6 | `ac10_a_journal_on_hold_for_a_restore_classifies_as_restore_pending` in `crates/pool-control/tests/journal.rs` sets the hold with the `damage` helper; with no witness the state is `unconfirmed`, which spec 0003 puts first. `ac13_run_refuses_a_real_journal_without_or_with_another_witness_and_never_binds` in `commands.rs` gives `unconfirmed` and `mismatch` through `pool run` without Docker. A unit test of `not_ready` is not possible: `daemon.rs` has no test module, and its functions are private to the binary. So `restore_pending` through `pool run` stays on the fixture, with `sqlite3` | H1, the hold read as none: not caught → caught. M5 of round two: caught on the fixture only → now also by the new `commands.rs` test |
| t7 | `it_ac9_ac25_a_serving_daemon_holds_its_state_directory_and_stops_on_sigint`: `pool show-authority` and a second `pool run` get `state.locked` while the daemon serves, then a SIGINT stops it with exit 0, `daemon.stopping` and `daemon.stopped`. `ac10_open_ready_refuses_a_journal_whose_authority_id_changed` and `ac10_a_latch_write_refuses_a_journal_whose_authority_id_changed`. The lock race test now asserts that at least one child met the lock; 20 runs of it on macOS gave 11 to 66 of 200 | L3, the lock dropped when the daemon serves (`drop(_state)` after the destructuring; a first version with `_state: _` changed nothing, because the field then lives until the end of the function): not caught → caught. S1, SIGINT not handled: not caught → caught. I1 and I2, the ID not compared in `open_ready` and in `set_conflict_latch`: not caught → caught |
| t8 | The narrow windows of `session.rs` are not done, for the reason above. The waiter test of `integration.rs` still uses five pauses: no outside sign shows that a login waits for a backend, so there is no state to wait on. A client written for that is more than this item. The single poll of `ac14_the_conflict_view_is_published_before_the_latch_is_written` asserts that the round is still pending, so the test fails loudly, never passes wrongly, if the blocking write ever ends within that one poll | not run |
| t9 | `it_ac19_a_client_that_vanishes_inside_a_transaction_leaves_no_row_and_gives_the_permit_back`: a client of the test drops its socket inside a transaction, with no `Terminate`; the backend goes, the row is gone, and with one session allowed the next login works. The psql case now also checks `INSERT 0 1`. The SQL file of `0004 AC-18` starts with `\set VERBOSITY verbose`, so the deliberate error is compared with its SQLSTATE (`22012`) through POOL and directly. The second primary: M3 above. The refusals of POOL are still compared by message text on the fixture; their codes are held by the hermetic tests. A standby, a node that restarts and a sample that expires with nothing to replace it are left for the scope that brings replicas (scope 007) | not run |
| t10 | `ac28_the_short_messages_have_the_literal_bytes_of_the_postgresql_documentation` in `golden.rs`: `SSLRequest`, the codes of `GSSENCRequest` and `CancelRequest`, `AuthenticationOk`, `AuthenticationSASL`, `ReadyForQuery` in its three states, `Terminate`, `Query`, `BackendKeyData` and `ParameterStatus`, written byte by byte. Not done: the list of fuzz targets is the architect's first (the review says so); and the second sentence of `0004 AC-28` (nothing reserved before the limit check) needs an allocator that counts, which needs `unsafe` code, and `unsafe_code = "forbid"` holds for the test targets too, so that is a decision for the architect | L1, the code of `SSLRequest` changed in the one constant that the encoder and the parser share: not caught by any test of `pool-protocol` → caught |
| g4 | Not mine (`scripts/`). The proposal, for the developer: fail the check unless the test output holds a result line with more than 0 tests passed, give the test command a limit of its own so that the cleanup always runs, and remove the containers by the IDs that `docker run -d` printed | not run |

#### Gates

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, on `0c0424c` (`486db56` with `main` at `6916184` merged, which holds Build plan step 15 of `apps/pool/src/` and the review decisions of spec 0004), on 2026-11-16 from 01:49 to 01:52 WIB, and again on `2734540` (the last merge of `main`, with more of step 15 in `apps/pool/src/`) from 01:53 to 01:56 WIB with the same results.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 1: fmt, clippy, deny, guards and docs `ok`; `test` failed in one test, `workspace::ac2_the_committed_w3_baseline_is_the_block_of_spec_0004` of round one. The block under [Source guard](index.md#source-guard) gained, with `14e6903`, a bare `rusqlite`, `allow crates/pool-control/tests rusqlite`, `aws_lc_rs` and `allow crates/pool-core aws_lc_rs`, and `tools/repo-check/baseline/forbidden-source-patterns.txt` does not hold them yet. The test reports exactly that difference, which the coordination log names as known (`9502b5c`). The baseline belongs to the developer; this item may not change it. Every other test of the workspace passed | **fail**, for a cause outside this item |
| `scripts/check.sh integration` | exit 0, nothing left behind | exit 0: 29 of 29 in 56 s and in 51 s, each run of the daemon checked by `finish`; no container of this run left | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0, no test failed, `linux build check passed` | pass |
| `git diff --stat main...HEAD` | test files and this record only | the files named at the top of this round and this record; 0 lines of production code | pass |

### Round five

On 2026-11-16 from 02:20 to 02:55 WIB, by the tester session: the findings of [the review](../../reviews/2026-11-16-scope-004-one-real-primary-path.md) that round four left for `crates/pool-core/tests/session.rs` (t1, t3, and the parts of t4 and t8 in that file), and decision 94 of [Fuzzing](index.md#fuzzing). On `main` at `ce67e27`, commit `9818739` of the branch. macOS 27.0.1, arm64.

#### The tests

| Finding | Tests in `crates/pool-core/tests/session.rs` | What they hold |
|---|---|---|
| t1 | `ac15_a_request_for_3_2_gets_negotiate_protocol_version_first_with_its_exact_body` | A new helper, `login_recorded`, keeps every message of a login in order with its authentication code and checks the server signature when it comes. For 3.2: `v`, then `R` 10, 11, 12 and 0, three `S`, `Z`; the body of `v` is `00 03 00 00 00 00 00 00` (3.0, no option). With `_pq_.zq9`: the body PostgreSQL 18.7 sent for that request |
| t1 | `ac16_a_login_for_3_0_gets_the_server_signature_and_then_authentication_ok_and_no_negotiation` | `R` 10, 11, 12, 0, three `S`, `Z`, and no `v` |
| t1 | `ac14_no_authentication_ok_comes_before_a_refusal_after_the_proof` | After a right proof, a closed route (`57P03`), a route that changed at the second look (`57P03`), a database without an identity entry (`28000`) and a node in recovery (`08006`) each give exactly `R` 10, 11, 12 and the error, and no `AuthenticationOk` |
| t3 | `ac20_no_byte_of_an_oversized_query_reaches_the_node` | The scripted node keeps every byte it gets after its `ReadyForQuery` (`after_ready`, read with `read`, so also the bytes of a message that never ends). After a `Query` of 1100 bytes with a limit of 1024, `54000`, and the node got only the `Terminate` of POOL |
| t4 | every test of the file | `read_msg` and the new `read_msg_kept` wait at most 30 seconds and then fail with `a message or the end of the stream within 30 s`; the two bare `shutdown()` calls wait at most 10 seconds |
| t8 | `a_positive_conflict_ends_running_sessions_and_silence_does_not`, `at_the_shutdown_deadline_a_query_in_flight_gets_08007_never_57p01`, `waiters_and_the_acquire_deadline` | The first two wait until the node has the `Query` (then it is in flight) in place of a fixed 50 ms. The waiter test has a deadline of 1 second in place of 300 ms: "at once" is under 700 ms, the waiter ends at 1 to 3 seconds |

The other fixed waits of the file stay. They come before an assertion that something did not happen (no query reached the node, no second backend), where the wait is the window in which a wrong byte would have arrived.

| Wrong change (`crates/pool-core/src/session.rs`) | Before (`ce67e27`) | After, caught by |
|---|---|---|
| N1. `NegotiateProtocolVersion` is never sent | not caught (review) | `ac15_a_request_for_3_2_gets_negotiate_protocol_version_first_with_its_exact_body` |
| N2. The server signature is never sent to the client | not caught (review) | `ac16_a_login_for_3_0_gets_the_server_signature_...`, `ac15_...`, `ac14_no_authentication_ok_...` |
| N3. `AuthenticationOk` is sent right after the proof, before the route checks and the backend | not caught (review) | `ac14_no_authentication_ok_comes_before_a_refusal_after_the_proof` |
| N4. The header of an oversized `Query` is written to the node before the refusal (for `MessageTooLarge` only, as the review made it) | not caught | `ac20_no_byte_of_an_oversized_query_reaches_the_node`. (A first version wrote the header for every refused message; the old `unsupported_and_broken_frontend_messages_end_the_session` caught that one through the other tags) |
| N5. The cut check at the top of the relay loop is removed (t4) | the test binary hung (review: ten minutes) | 12 tests fail, among them `a_positive_conflict_ends_running_sessions_and_silence_does_not` and `at_the_shutdown_deadline_a_query_in_flight_gets_08007_never_57p01`; the whole run of the five mutations took under two minutes |

"Before" was run again here, with the tests of `ce67e27` in a copy of the tree: N1, N2, N3 and N4 passed every test of `pool-core`, as the review found; N5 made the session tests hang until the limit of 600 seconds of the mutation runner ended the run.

`cargo test -p pool-core --locked --test session`: 71 of 71, five runs in a row.

#### Fuzzing (decision 94)

- `crates/pool-protocol/fuzz/fuzz_targets/auth.rs`, a new target, with its `[[bin]]` entry in `crates/pool-protocol/fuzz/Cargo.toml`. It feeds the whole input to `parse_sasl_initial_response` first, then `parse_authentication`, `parse_negotiate_protocol_version`, `parse_ready_for_query`, `parse_parameter_status`, `parse_backend_key_data` and `error_sqlstate`. Nothing may panic, and what a parser accepts, its encoder writes back to the same bytes. 13 regression inputs under `fuzz/regressions/auth/`, made from the golden messages. `cargo metadata --locked` on the fuzz manifest passes: the lockfile of the package does not change. `max_len` for the line of `scripts/check.sh`: **4096** (a SASL message of a client is at most 1024 bytes; 4096 also covers the node messages these parsers read).
- `data_row`, which decision 94 names too, is a private function of `pool-core` and is not reachable from this package: `pool-core` is not a dependency of the fuzz package, and adding it would change its lockfile. Not done; a question for the architect.
- `scram` now takes the length of a nonce from its first byte (0 to 63) and the nonce from the next bytes; the rest is the message. Shown on a copy: with a `panic!` put right after the decoding of the proof in `parse_client_final`, an input that holds a nonce and a client-final message with that nonce reaches it (`REACHED: the proof of a client-final message was decoded`, `deadly signal`), and the old target with the same input does not. The committed regression inputs of `scram` were not changed; their first byte is now read as the length of a nonce.
- The literal bytes of the short messages are in `golden.rs` since round four. The verifier of RFC 7677 as a literal is not added: `golden.rs` was not a file of this round.

| Command, from `crates/pool-protocol` | Observed | Result |
|---|---|---|
| `CARGO_TARGET_DIR=<root>/target/fuzz cargo +nightly-2026-10-17 fuzz run auth fuzz/corpus/auth fuzz/regressions/auth -- -max_total_time=30 -max_len=4096` | exit 0, 3057871 runs in 31 s, no file under `fuzz/artifacts/` | pass |
| the same for `scram` with `-max_len=4096` | exit 0, 3223829 runs in 31 s, no file under `fuzz/artifacts/` | pass |

#### Gates

| Command | Observed | Result |
|---|---|---|
| `scripts/check.sh all > <file> 2>&1; echo $?` on the tree of `9818739` before its commit | exit status 0: fmt, clippy, test, deny, guards and docs `ok` | pass |
| `scripts/check.sh linux-build` on `9818739` | exit status 1: in the pinned image, 4 of the 18 tests of `apps/pool/tests/commands.rs` failed, each because `pool bootstrap` stops at `secret.permissions at node.tls_ca_file` before it reaches the node of the test (`ac11_bootstrap_against_a_node_that_does_not_answer_creates_no_journal`, `ac10_a_journal_that_was_cut_or_whose_marks_changed_...`, `ac9_a_lock_held_by_another_pool_process_...`, `ac9_bootstraps_started_together_...`). These are the four that are red on `main` there, whose fix the developer of `apps/pool` is making; this round changed nothing in `apps/pool`. `cargo test` stops at the first test binary that fails, so the other packages did not run in that check | fail, the known four only |
| `cargo test --locked --no-fail-fast -p pool-core -p pool-protocol -p pool-control -p pool-config` in the same pinned image, from `git archive` of `9818739` | exit 0, every test result `ok`, among them `session` 71 of 71 and `golden` 17 | pass |

### Round six

On 2026-11-16 until 03:05 WIB, by the tester session: the rest of finding t10 of the review and of decision 94, on `main` at `cd18109`, commit `9923a48` of the branch. macOS 27.0.1, arm64.

- **The exchange and the verifier of RFC 7677.** `ac28_the_scram_messages_of_rfc_7677_parse_and_encode_to_their_literal_bytes` in `crates/pool-protocol/tests/golden.rs` holds the four messages of section 3 of the RFC (user `user`, password `pencil`) and the verifier of that password in the form PostgreSQL stores, `SCRAM-SHA-256$4096:W22ZaJ0SNY7soEsUEjb6gQ==$WG5d8oPm3OtcPnkdi4Uo7BkeZkBFzpcXkuLmtbsT4qY=:wfPLwcE6nTWhTAmQ7tl2KeoiWGPlZqQxSrmfPwDl2dU=`. The two keys were derived with PBKDF2-HMAC-SHA-256 of the Python standard library, outside this crate, and that derivation reproduces the client proof and the server signature that the RFC prints. The salt, the proof, the signature and the keys are written in the test as bytes, so the parsers and encoders of `scram.rs` and the base64 of the crate are held to values that they did not make. Wrong change: `+` and `/` swapped in the alphabet and in the decoder of `base64.rs` together, so that every round trip still holds: not caught by the tests of `pool-protocol` at `cd18109`, caught by the new test.
- **The regression inputs of `scram`.** In the input form of decision 94 the first byte is the length of a nonce. The nine old inputs now start with a byte 0, an empty nonce, and the rest is unchanged, so each still feeds the message its name says to the parsers that need no nonce (client-first, server-final, the base64 cases). Five new inputs carry a nonce of their own: `nonce-client-final-proof` and `nonce-client-final-proof-y` (the two headers), `nonce-client-final-short-proof`, `nonce-server-first` and `nonce-server-first-count-0`. Shown on a copy with a `panic!` right after the decoding of the proof in `parse_client_final`: both `nonce-client-final-proof` inputs reach it, and the old `client-final`, with its empty nonce, does not.

| Command | Observed | Result |
|---|---|---|
| `scripts/check.sh all > <file> 2>&1; echo $?` before the commit | exit status 0: fmt, clippy, test, deny, guards and docs `ok` | pass |
| From `crates/pool-protocol`: `CARGO_TARGET_DIR=<root>/target/fuzz cargo +nightly-2026-10-17 fuzz run scram fuzz/corpus/scram fuzz/regressions/scram -- -max_total_time=30 -max_len=4096` | exit 0, 3943376 runs in 31 s, no file under `fuzz/artifacts/` | pass |
| `scripts/check.sh linux-build` on `9923a48` | exit status 0, `linux build check passed` | pass |

## Full verification

The `Verify it` box of feature 4 (`/check verify one real primary path`), on 2026-11-16 from 00:14 to 00:45 WIB, by the checker session, on `main` at `af23fa4`. macOS 27.0.1, arm64, `rustc 1.100.0`, Docker 29.9.2 running, PostgreSQL 18.7 of `postgres:18.7-bookworm` by the digest of `scripts/check.sh`, and the pinned build image for Linux. The workstation and the Linux image are not CI. Every command below was run in this session.

This run does not repeat the checker runs above. It asks three things: did anything change since each of them, do all checks pass on the final tree, and is there a verdict for every criterion.

**Verdict: feature 4 PASS** on the workstation and in the pinned Linux image. Every criterion from `0004 AC-1` to `AC-28` and `0003 AC-17` to `AC-19` is met. What is left is manual and is listed at the end with its reason and its owner. Three notes go to their owners and do not change the verdict.

### The final tree against the trees of the earlier runs

- **No line of production code changed since `fd62cf4`**, the fix of the lock file race. `git diff --numstat d4fdb62..af23fa4 -- crates apps tools scripts` names thirteen files: ten test files, and three source files whose only change sits inside their `#[cfg(test)]` module (`apps/pool/src/control.rs` 342 lines added after line 260, the module starts at line 222; `crates/pool-core/src/scram.rs` 18; `crates/pool-core/src/secret.rs` 119; no line removed in any of the three). `scripts/`, `tools/repo-check/src/`, `deny.toml`, `Cargo.toml`, `Cargo.lock` and both fuzz packages are the same as on `d4fdb62`.
- **The Linux release binary is the same file.** `scripts/check.sh linux-build` on `af23fa4` prints `sha256 7ccc2e2ba0ba2b3df8f4acef60e937f2a0207c54caa80cd1e21c481072a9ca7a` for `target/release/pool`, the hash the developer recorded on `22182fb`, the first tree with the fix.
- So every earlier verdict of the checker still holds for the product. What had to be looked at again is the tests, the checks and the documents.

### Every check on the final tree

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0 | exit 0, all six `ok`, 24 test results `ok` and none failed. `pool` 18 unit, 87 `check_config`, 17 `commands`; `pool-config` 32 and 123; `pool-control` 5, 3, 14, 23 and 28; `pool-core` 50 and 47; `pool-protocol` 22, 16 and 4; `pool-repo-check` 108 and 112 | pass |
| `scripts/check.sh integration` | exit 0, 26 tests, nothing left | exit 0, 26 of 26 in 42 s; no container of this run, no temporary directory | pass |
| `scripts/check.sh fuzz` | no crash file | exit 0: `validate` 654436 runs, `startup` 2921491, `frontend_message` 1082817, `scram` 4834686, each 31 s; no file under any `fuzz/artifacts/` | pass |
| `scripts/check.sh linux-build` | exit 0 | exit 0, 22 test results `ok`, `pool 0.1.0`, `linux build check passed` | pass |
| `scripts/prove-checks.sh --with-linux-build` | every case rejects its change | exit 0 in 2 min 19 s: cases 1 to 35 `PASS`, case 20 with Docker, `all checks reject their bad change`; the tree clean afterwards | pass |
| `cargo build --workspace --locked`, after touching every crate root | no warning | 0 warnings | pass |
| The use check of [Adopted dependencies](index.md#adopted-dependencies) | no line | no line | pass |
| `cargo metadata --no-deps --format-version 1` | five members, six pairs, `pool-protocol` with no dependency | `pool`, `pool-config`, `pool-control`, `pool-core`, `pool-protocol`; the six pairs of the spec; `pool-protocol` has none; no package has a dev or build dependency | pass |

The probes of the checker, again on `af23fa4` (they are described in the runs above):

| Probe | Checks | Mismatches |
|---|---|---|
| The command probe with no database (state directory, files, lock, secret files, output) | 100 | 0 |
| The fixture probe (bootstrap, `show-authority`, the probe, a stopped bootstrap, the bytes of a real server) | 34 | 0 |
| The daemon probe (23 runs of `pool run`: not ready, walking result, startup, node, control loop, teardown, lost COMMIT, limits, memory, shutdown, messages, the whole log of 375 lines) | 131 | 0 |

### A verdict for every criterion

"Obtained on" is the tree of the checker run that holds the detail. "On `af23fa4`" is what was run again in this run. The product code is the same on all of them from the tree named.

| Criterion | Verdict | Obtained on | On `af23fa4` |
|---|---|---|---|
| `0004 AC-1` | met | the shape on `486b90e`; the last sentence (the use check, no warning) on `d4fdb62` | `cargo metadata`, the build and the use check, above |
| `0004 AC-2` | met | `d8797a5`, `486b90e`; rule W3 again with 125 changed lines on `3b58997` | `guards` inside `all`; the six cases in `prove-checks.sh` |
| `0004 AC-3` | met | `486b90e`; the exact versions on `59b7e2c`; both fuzz lockfiles and decision 70 on `d4fdb62` | `deny` inside `all`; `linux-build`; `NOTICE` read against the lockfile, below |
| `0004 AC-4` | met | `d8797a5`, `486b90e` | `prove-checks.sh`, 35 of 35 |
| `0004 AC-5` to `AC-8` | met | `d8797a5`, `486b90e`, `8332451` | the tests of `pool-config` and `check_config.rs` inside `all`; `crates/pool-config/src` has not changed since |
| `0004 AC-9` | met | the secret files on `77ca71f`; the state directory and the lock on `ecd0ee5`; the lock file race on `fd62cf4` and `d4fdb62` | the command probe; `commands.rs` inside `all` and in the repetition below |
| `0004 AC-10` | met | the library half on `59b7e2c`; the journal of a real bootstrap on `ecd0ee5` | the fixture probe |
| `0004 AC-11` | met | `3b58997` (it failed there for a stopped bootstrap), fixed and met on `ecd0ee5` | the fixture probe |
| `0004 AC-12` | met | the library half on `59b7e2c`; the monitor login and the server log on `3b58997` | the fixture probe |
| `0004 AC-13` | met | `d4fdb62`, all six states | the daemon probe |
| `0004 AC-14` | met | the hermetic half on `8783337` (not met there for a client that does not read), fixed and met on `5b11ca3`; the last report of `in_hot_standby` on `3b58997`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-15` | met | byte level on `8332451` and `3b58997`; the library half on `77ca71f`; hermetic on `8783337`; the fixture on `d4fdb62`, with the reading of decision 72 | the daemon probe |
| `0004 AC-16` | met | byte level on `8332451` and `3b58997`; the library half on `77ca71f`; hermetic on `8783337`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-17` | met | hermetic on `8783337`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-18` | met | `d4fdb62`, with another SQL file than the tests use | the daemon probe |
| `0004 AC-19` | met | hermetic on `8783337`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-20` | met | hermetic on `8783337`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-21` | met | `d4fdb62` | the daemon probe: about 18 MiB against 96 MiB |
| `0004 AC-22` | met | hermetic on `8783337`; the fixture on `d4fdb62` | the daemon probe |
| `0004 AC-23` | met | byte level on `8332451` and `3b58997`; hermetic on `8783337`; a real server on `d4fdb62` | the daemon probe |
| `0004 AC-24` | met | `Debug` of the types of `pool-protocol` and `pool-core` on `3b58997`; the log on `d4fdb62` | the daemon probe, 375 lines |
| `0004 AC-25` | met | hermetic on `8783337` (not met there for a client that does not read), fixed and met on `5b11ca3`; the daemon with sessions on `d4fdb62` | the daemon probe |
| `0004 AC-26` | met | `3b58997` (a pass, a failing test, SIGTERM, no Docker, the digest) | `integration`, 26 of 26, nothing left; the CI job is manual |
| `0004 AC-27` | met | `d4fdb62` | `fuzz`; the CI job is manual |
| `0004 AC-28` | met | `8332451`, and again with one reader per connection on `3b58997` | the tests of `pool-protocol` inside `all` |
| `0003 AC-17` | met | `59b7e2c`; a receipt out of order on `3b58997` | the tests of `pool-control` inside `all`; the daemon probe sets and records a latch |
| `0003 AC-18` | met | the guard half on `d8797a5` and `486b90e`; the code half on `59b7e2c` | `guards` and the tests of `pool-control` inside `all` |
| `0003 AC-19` | met | the guard half on `d8797a5`; the code half on `59b7e2c`; the method name of decision 57 on `3b58997` | `guards` inside `all` and its case in `prove-checks.sh` |

No criterion is left between two runs. The two that were once not met (`0004 AC-14` and `AC-25`, for a client that does not read) and the two findings of milestone 3 (`AC-11` and `AC-9`) each have a fix and a run of the checker after it.

### Step 14: the evidence table

The table [Evidence per criterion](#evidence-per-criterion) names, for each criterion, a check or a test file. Each of them exists and passed on `af23fa4` in the commands above. The four tests it names for the hermetic messages exist where it says. The rows that are `NOT_RUN` or `BLOCKED` (the CI jobs, M2, M3) have a reason and `owner: user`. `docs/index.md` has a row for this file.

Two things in it are not exact, and both go to its author:

- **Its list of messages that are proved only hermetically has four entries, and the record of step 11 has seven.** It leaves out another SCRAM header, a COPY response of the tag `W` and an authentication message of 1025 bytes, and then says that every other message has had a real `psql` or PostgreSQL on the other end. For those three that is not so. The record as a whole does name them, under step 11, so the rule "A golden message is not proof of a wire format" is kept; the summary of step 14 is too short by three.
- **The row for `0004 AC-13` names `commands.rs` alone.** The states `unconfirmed`, `mismatch` and `restore_pending` need a real journal and are tested in `integration.rs`.

The table says "at `ecd0ee5`", with a paragraph for `22182fb`. That is the date of that record. This section is the one for `af23fa4`.

### The tests the tester added

Eight wrong changes, one at a time, in a private copy of `af23fa4`, each against `cargo test --locked` of its package and then `scripts/check.sh integration`:

| Wrong change | Result |
|---|---|
| T1. `route_view` never sets the conflict flag (M1 of the run of milestone 4) | caught by four unit tests of `control.rs`, among them `ac14_route_view_marks_a_positive_conflict_and_not_silence` |
| T2. The route view is published after the latch writes (M2) | caught by `ac14_the_conflict_view_is_published_before_the_latch_is_written` |
| T3. A latch is never written to the journal (M3) | caught by `ac14_a_second_primary_latches_the_cluster_in_the_journal_through_the_control_loop` and two more |
| T4. An open route stays fresh one hour more (M4) | caught by `ac14_an_open_route_is_fresh_exactly_until_its_snapshot_says` |
| T5. `daemon.started` carries a field the table does not have | caught by `it_ac24_the_log_holds_only_the_fields_of_the_table_and_no_text_from_postgresql` |
| T6. `probe.failed` carries a field `host` with the address of the node | **not caught** |
| T7. `session.started` swaps the principal ID and the identity ID | caught by `it_the_walking_fixture_gives_the_same_answers_through_pool` |
| T8. `session.started` loses the field `topology_epoch` | caught by `ac24_session_events_carry_exactly_the_fields_of_the_spec_and_ids_in_place_of_names` |

So the tests for M1 to M4 guard what they say, and the order of decision 12 is now held by a test. T6 shows the one gap: the field check and the search for names run on the log of three tests (`assert_log_is_clean` is called three times), and none of those three runs writes `probe.failed`. The tests that do make the probe fail do not search their log. `0004 AC-24` speaks of a whole run of the integration check. The daemon probe of the checker searches every log of its 23 runs, so the product is clean today; the gap is in what the tests would notice tomorrow. For the tester.

**The `sqlite3` tool of the host.** `it_ac13_run_refuses_a_journal_on_hold_for_a_restore_and_never_binds` sets the hold with `sqlite3`. Run with a `PATH` that has no `sqlite3` (the other tools the test needs were on it), the test fails: `panicked at apps/pool/tests/integration.rs:2031:10: the sqlite3 tool of the host (needed by this test): Os { code: 2, kind: NotFound ... }`, `test result: FAILED. 0 passed; 1 failed`. It is not skipped. Two notes: the spec says of the integration check that the host needs no tool, and this test now needs one; and whether the hosted runner has it is not known until manual step M2.

### The two tests that were once unstable

`apps/pool/tests/commands.rs` on `af23fa4`: the whole test binary, run again and again. The output of every run that fails or hangs is kept.

| Where | Runs | Passed | Failed | Hung |
|---|---|---|---|---|
| macOS, nothing else of this session running on the host (the Linux runs went on in Docker beside it) | 50 | 50 | 0 | 0 |
| macOS, with 20 busy loops on the 10 cores | 40 | 40 | 0 | 0 |
| The pinned Linux image, as a user that is not root | 50 | 50 | 0 | 0 |
| The pinned Linux image, with 16 busy loops in the container | 40 | 40 | 0 | 0 |

180 runs of 17 tests each, no failure and no hang, so no output had to be kept. A run takes about 10 seconds on both, also under load, because most of it is waiting for probe timeouts.

- **The lock race test** (`ac9_commands_that_make_the_lock_file_at_the_same_moment_get_state_locked`, after `9765845`). Each child now waits at a gate of its own and the test waits at most 60 seconds for each, so the hang of the run of milestone 4 can no longer stop a check: it would be a failure with a message. It did not fail here in 180 runs.
- **`ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds`.** It failed once for the debugger and once for the tester, with no output kept. It did not fail here in 180 runs, so there is still no output to send to its developer. What can be said from reading it: the test takes a port by binding port 0 and letting it go, runs `pool run` four times, and after each run binds that port again to show the listener never took it. Other tests of the same binary do the same in other threads at the same time, and some of them keep a listener. If one of them is handed the port this test let go, the last assert fails though `pool run` did nothing wrong. The other candidate is the limit of 5 seconds on a run of `pool run` on a busy machine. Neither is shown. For the developer of step 8: let the test hold its listener on the port while `pool run` runs, which also proves more (a `pool run` that tried to bind would fail in another way), and keep the output of a failure.

### The documents the developers touched

| Document | Compared with | Result |
|---|---|---|
| `docs/research/technology-evaluation.md`, "Adopted in scope 004" | `Cargo.lock`, the root `Cargo.toml`, `deny.toml`, `scripts/check.sh`, the header of the bundled SQLite | the six versions (`tokio` 1.54.1, `rustls` 0.23.46, `tokio-rustls` 0.26.6, `aws-lc-rs` 1.19.0, `rusqlite` 0.42.0, `tracing` 0.1.44) and their features are the ones locked and declared; `aws-lc-sys` 0.47.0, `libsqlite3-sys` 0.40.0 and SQLite 3.54.1; 75 lockfile entries, 70 from the registry; the five license exceptions by crate with their exact versions; the image tag and digest. No difference |
| `NOTICE` | the same | names SQLite 3.54.1 through `libsqlite3-sys` 0.40.0 and `rusqlite` 0.42.0 with its public domain dedication, and AWS-LC 0.47.0 through `aws-lc-sys` and `aws-lc-rs` 1.19.0 with the terms of its parts and the license texts. No difference |
| `docs/engineering-standards.md` | items 1 of [Document corrections](index.md#document-corrections) | the row for SQLSTATEs holds the rule and the link; the first paragraph names the five members |
| `docs/repository-structure.md` | item 2 | the first paragraph names the five paths |
| `docs/index.md` | the files of this spec | rows for `index.md`, `rationale.md` and `verify.md`, and for ADR 0004 |
| Spec 0002, rules W1 and W6 | the tool | a dated note now says the count is four lockfiles, which closes the note of the run of step 12 |

Two sentences that the developer of step 13 already named are still as they were, and belong to the "Document it" box: the paragraph on the fuzz package in `docs/engineering-standards.md` names only `crates/pool-config/fuzz/`, and the row of `apps/pool` in the crate table of `docs/repository-structure.md` says it depends on all four internal crates, where it depends on three.

### What is left, and who owns it

| Step | State | Reason | Owner |
|---|---|---|---|
| M2 (`F4-3`): the jobs `integration` and `fuzz` in `.github/workflows/ci.yml` and the two contexts in `.github/rulesets/main.json` | `NOT_RUN` | `.github/` is no file of a session. Until it is done there is no Linux evidence of the fixture from a hosted runner, and no run of the fuzz job | user |
| M3 (`F4-1`): push, pull request, the nine checks on the remote, the rule on the remote | `NOT_RUN` | writing to GitHub needs the user's permission each time | user |
| `F4-5`: a report of a secret scanner on `crates/pool-core/tests/fixtures/node.key` | `NOT_RUN` | it can only come once the pull request exists. The answer is decision 62: one test key is committed on purpose, and its first line says what it is | user |
| M4 (`F4-4`): a running Docker daemon | met on this workstation | every check that needs it ran here | user, on any other machine |
| M1 (`F4-2`): the license decision | done on 2026-11-15 | the five exceptions are in `deny.toml` with exact versions | done |

Every step that a session can run has run and passed on `af23fa4`. Evidence from CI is the one kind that is missing, and no claim in this record rests on it.

### Notes for the owners

- **For the tester:** the gap of T6 above. `Daemon::stop` of `integration.rs` still waits without a limit in the older tests, as the tester's own list says.
- **For the developer of step 14:** the two inexact places in the evidence table.
- **For the architect:** a test of the integration check now needs a tool of the host (`sqlite3`), against the sentence that the host needs no tool. Either the sentence or the test should give.
- **For the developer of step 8:** the reading of the `ac13` test above.

## Build plan step 15

### `apps/pool`, the daemon and its tasks (developer B, stage 1)

Built on 2026-11-16 on top of `main` at `6ba9e36`, as decisions 86 and 96 and findings c1, c2, c9 and d3 of the independent review ask. One commit for each part:

- `9c42a72`, **`runtime.failed`** (c1, d3). A runtime that cannot be built is `runtime.failed` with exit 3 in `run` (which reported it as `listener.failed`) and in `bootstrap` (which went on with no sample and was refused with `bootstrap.evidence_missing`). The handlers for SIGTERM and SIGINT are installed before step 9, and a handler that cannot be installed is `runtime.failed` too. The daemon used to wait for ever on a handler that was missing. No test makes the operating system refuse a runtime or a handler, so these two paths are held by review only.
- `52ca154`, **the panic hook** (d3). `log::install` sets a hook that writes `daemon.panicked` through the queue and nothing else. Commands other than `run` keep the default hook.
- `0ac3996`, **the daemon watches its tasks** (c2). Step 10 waits on the two signals, the control loop, every probe task (a `JoinSet`), the listener task and the log writer. The first task that ends with no shutdown having asked for it gives `daemon.failed` with `task` `control`, `probe`, `listener` or `log`, then shutdown, `daemon.stopped` and exit 3. For `log` the line is written straight to stderr as one JSON object, since the writer is gone. A join handle that returned is never awaited again. **Open until stage 2:** the spec ends the sessions as if the deadline had passed (step 4 of Shutdown). `pool-core` has no call for that yet, so the failed path runs the whole `Server::shutdown()`, with a comment in `daemon.rs`. `dev-f4-m1` adds the call, and stage 2 uses it. No test ends a task of the daemon on purpose, so the watch is held by review only.
- `77e57f8`, **a last try for each latch** (c9). When the loop stops, every latch on `unrecorded_latches()` is written once more before the journal is closed, and one that still fails gets `authority.conflict_latched` with `recorded` false. Test: `control::tests::c9_a_latch_still_open_at_shutdown_is_tried_once_more_and_said_so`, which fails without the last try: one line and one failed write instead of two.

Gates on this tree, on macOS: `cargo test -p pool --locked` and `scripts/check.sh integration` passed (26 of 26). In `scripts/check.sh all`, every check but `test` passed, and `test` failed in one place only, the test `ac2_the_committed_w3_baseline_is_the_block_of_spec_0004` of `tools/repo-check`. The spec on `main` already has the lines `rusqlite` and `aws_lc_rs` of decision 93 (finding g2), and the committed `forbidden-source-patterns.txt` does not have them yet. That file belongs to another part of step 15, so this is not something stage 1 changed, and stage 1 does not touch `tools/`.

### `apps/pool`, node loss, the failed path and the fixture (developer B, stage 2)

Built on 2026-11-16 on top of `main` at `653d864`, which has the crate part of `dev-f4-m1` (the accessors of `EligiblePrimary`, the two fields of `Route`, the flag `lost`, `Server::with_identity_query`, `Server::shutdown_now`, `probe.node_loss_timeout_ms`). One commit for each part:

- `f9df6c3`, **a silent writer is marked lost** (M1, decision 84). The probe task hands the class of each failure to the control loop with the sample. The loop keeps, per node, the time of the first failure of the class `connect` or `timeout` since the last other outcome; `tls`, `login` and `answer` start the time again. When the writer of a cluster has been silent for `probe.node_loss_timeout_ms`, the loop marks it, writes `node.lost` once, and the route view carries `lost` on that cluster. The first good sample removes the mark. The writer is the `expected_writer` of the configuration: a cluster opens only while it names the approved writer of the journal, and `apps/pool` cannot read the journal rows (rule W3), so a running session is only ever on that node. Unit tests in `control.rs`: `ac30_connect_and_timeout_for_the_whole_time_mark_the_writer_lost_once`, `ac30_a_failure_of_another_class_starts_the_time_again`, `ac30_a_good_sample_starts_the_time_again_and_removes_the_mark`, `ac30_only_the_writer_counts_and_the_view_carries_the_mark`.
- `192604c`, **keepalive on the listener** (M1). Step 7 builds the listener with `TcpSocket`, keepalive and reuseaddr on, and listens with a backlog of 1024. Linux and macOS give an accepted socket the keepalive of its listener. That an accepted socket has it on is the check with `ss -tno` that the spec gives to the checker. This stage did not run it.
- `1382928`, **`shutdown_now` on the failed path** (c2). A task that ended without being asked to now ends the sessions with `Server::shutdown_now`, from step 4 of Shutdown on. A signal still drains with `Server::shutdown`. This closes the gap that stage 1 left open.
- `9b5f44e`, **the fixture tests**. `it_ac29_a_login_to_an_address_that_moved_to_another_server_gets_08006`: the probe stays on container 1 through the relay while new connections reach container 2. The login gets `08006` with the message of a backend that could not be established, `backend.failed` has the class `identity`, and the log of container 2 since the move has no statement but the identity question (counted by its first line, since PostgreSQL writes a statement of several lines with a tab after each line feed), and not the SQL of the client. `it_ac30_a_paused_writer_is_marked_lost_and_its_sessions_end`: container 1 is paused with `probe.node_loss_timeout_ms = 2000`. `node.lost` is written once, the session with `pg_sleep(60)` in flight gets `08007`, the idle one gets `08006`, both end with `node_lost`, and a login works again after the unpause. A guard unpauses the container and ends the `pg_sleep` backend also when the test fails. The first run without that end of the backend showed why it is needed: four later tests failed while it still ran. The log check of the tests now knows `node.lost`, `daemon.failed` and `daemon.panicked`.
- `ddbb371`, **the integration script** (g4, and the two items of n13 for `run.sh`). Containers are removed by the ID that `docker run -d` printed. The trap is set before `mktemp`, and an empty listener address fails the run. The test command runs in a process group of its own with a watchdog of 1800 seconds, and a run in which no test passed exits 1. Each was checked once with a temporary change: a filter that matches no test gave exit 1 with `no test ran`, and a limit of 25 seconds ended the tests after 30 seconds with exit 1. In both, no container or test process was left. SIGTERM to the script alone, while the tests ran, ended it within one second, with nothing left. That is more than the paragraph "A signal to the script alone" of the spec says now (the trap ran only after the test command had returned), and that paragraph is for the architect to bring up to date.

Gates on this tree, on macOS: `scripts/check.sh all` (exit 0), `cargo test -p pool --locked` and `scripts/check.sh integration` (31 of 31, exit 0, no container left) passed.

After `git merge main` at `7a67c93`: `d41d25c`, **a copy of the test CA in the command tests**. `scripts/check.sh linux-build` had failed in four tests of `commands.rs` with `secret.permissions at node.tls_ca_file`, as the record of developer A above says: `git archive` gives the committed `ca.pem` mode 664, and the rule of decision 88 refuses it, as it must. Each test folder now holds a copy with mode 0644. Then, on that tree: `scripts/check.sh linux-build` exit 0 and `scripts/check.sh all` exit 0. `scripts/check.sh integration` was run twice. The first run exited 1: `it_ac14_a_second_primary_beside_a_silent_writer_ends_sessions_and_latches_the_journal` (round four of the tester, `fd84e4d`) found 2 monitor connections where it asserts 1. The test calls `second.accept_tls_again()` and then counts the monitor connections of container 1, so the probe of `pg2` can log in between the two. That reads as a race in the test, and stage 2 did not change that path. The second run exited 0, 31 of 31. On my trees that test failed once in three runs. The count of Docker volumes and networks was the same before and after each run, and no `pool-it-` container was left.

**Two more items from the lead.**

- `2504324`, **the CA of the fixture tests, and a CA that others can write**. `integration.rs` also named the committed `other-ca.pem` in its two cases of a wrong CA. With mode 0664 those cases would be refused by the rule of decision 88 before any TLS, and would no longer test the TLS check, so they use a copy with mode 0644. `check_config.rs` reads no CA file, `support/` parses the CA from bytes, and `scripts/integration/run.sh` makes its certificates with mode 0644. The new test `c6_a_ca_file_that_the_group_or_others_can_write_ends_bootstrap_with_its_key` in `commands.rs` holds the rule through the binary: a CA file of mode 0664, 0646 or 0666 ends `bootstrap` with `secret.permissions` at `node.tls_ca_file` and writes no journal, and with mode 0644 the bootstrap goes on to the probe. A CA owned by another account needs root to set up, so that case stays with the tests of `pool-core`. On `2504324`: `scripts/check.sh linux-build` exit 0, `scripts/check.sh all` exit 0, `scripts/check.sh integration` exit 0 with 31 of 31.
- `be80af4` and `ae8d25a`, **no host tool** (decision 101). The unit test `daemon::tests::each_state_that_is_not_ready_gives_its_own_finding` maps the six states that are not `Ready` to their six `authority.*` codes and checks that no two share one. With the arms for `missing` and `mismatch` swapped, it failed (`left: "authority.mismatch"`, `right: "authority.missing"`). With them restored, it passed. Then `it_ac13_run_refuses_a_journal_on_hold_for_a_restore_and_never_binds`, which set the hold with the host tool `sqlite3`, was removed. The library half stays in `crates/pool-control/tests/journal.rs`. The fixture tests were 29 before stage 2, 31 with its two new tests, and are 30 now. On `ae8d25a`: `scripts/check.sh all` exit 0, `scripts/check.sh integration` exit 0 with 30 of 30 and no container left.
- `f11f4f3`, **the race in the second primary test** (the failure recorded above, one run in three). The writer now falls silent, and its monitor connection is ended and counted while pg2 is still refused. pg2 answers only after that, so its own monitor login can no longer be counted. What the test proves is unchanged: a second primary beside a silent writer, `57P01` for the session, the latch with `recorded` true, and closed after a restart. `2f960cd` adds `POOL_IT_FILTER` to `scripts/integration/run.sh`, a filter of test names. With it, `POOL_IT_FILTER=it_ac14_a_second_primary_beside_a_silent_writer scripts/check.sh integration` was run 10 times in a row: 10 of 10 exit 0, each with 1 passed and 29 filtered out. Then on `2f960cd`: `scripts/check.sh all` exit 0, and the full `scripts/check.sh integration` exit 0 with 30 of 30 and no container left.

### Crates, tooling and build (developer A)

Built on 2026-11-16 on macOS (workstation, Docker running, not Linux and not CI), on top of `main` at `adeb0a0` and then `b92a300` (merged in `58461ce`, which keeps the tests of both sides in `crates/pool-core/src/backend/tests.rs`). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2` on `nightly-2026-10-17`. One commit for each part, each named for this step and its finding:

| Commit | Finding | What changed |
|---|---|---|
| `195a07a` | g2, decision 93 | `rusqlite` and `aws_lc_rs` are bare lines of the source guard, allowed in `crates/pool-control/src/journal.rs`, `crates/pool-control/tests` and `crates/pool-core`; the baseline is the block of [Source guard](index.md#source-guard) line for line. No code had to change. Case 36 of `scripts/prove-checks.sh` names `rusqlite` in `apps/pool/src/main.rs` |
| `4703bb5` | c3 | `parse_startup` keeps the names it has seen in a `HashSet`, so a repeated name costs one lookup. No answer changes; the golden and split tests of `pool-protocol` pass unchanged |
| `b3d9d5b` | c4, decision 87 | `.cargo/config.toml` with the two lines of [Native code](index.md#native-code); `journal::tests::the_linked_sqlite_is_the_bundled_3_54_1` |
| `a5de823` | g5, n5, n13 | `NOTICE`: "AWS-LC 5.12.0, as bundled in aws-lc-sys 0.47.0". The two comments name `JournalRead::bootstrap_plan`. `scripts/check.sh fuzz` refuses `POOL_FUZZ_SECONDS` of `0`, `abc` and `-5`, and checks each fuzz lockfile with `cargo metadata --locked` before the run |
| `d5211fe` | the full verification | The two corrections of the step 14 evidence: seven hermetic only messages, and `integration.rs` for `0004 AC-13` |
| `d91f619` | M2, decision 85 | `EligiblePrimary::system_identifier()` and `writer_timeline()`; the twin doc test reads all six accessors |
| `77a89a8` | M1, decision 84 | `probe.node_loss_timeout_ms`: default 30000, 1000 to 3600000, no cross rule, `ProbeConfig::node_loss_timeout()`. `valid-serving-limits.toml` holds 1000; the range table, the key list and the `Debug` text of `tests/validate.rs` name it |
| `4affb99` | M1 | Step 1 of the backend connection resolves the name and tries each address on a `tokio::net::TcpSocket` with `set_keepalive(true)` before it connects. `ac30_a_socket_to_a_node_has_keepalive_on_before_it_connects` (IPv4 and IPv6) |
| `6d7ad51` | M2 | `BackendConnection::verify_identity` and `BackendError::Identity` (class `identity`); `ac29_the_node_must_name_...` with the writer and eleven refused answers |
| `68d9427` | M2, M1 | `Route` carries `system_identifier` and `timeline`; step 8 runs for every session before `AuthenticationOk`. `RouteState::Closed` carries `lost`, `RouteView::is_lost`, `EndReason::NodeLost` (`node_lost`); a lost node is cut like a conflict and told as backend loss. In `apps/pool/src/control.rs`, as the lead allowed for the build only: the two fields, `lost: false`, and `lost: false` in the literals of its test module |
| `fb4e724` | decision 86 | `Server::shutdown_now()`: shutdown from step 4 on, with the same last limit of 2 s |
| `27f9495` | n1 | The lease is dropped before the bounded close of the client |
| `bb4df12`, `e19fe7e` | c6, decision 88 | A CA file or a certificate chain with a write bit for the group or others, or owned by another account than the service account or root, is `secret.permissions`. `read_certificate_chain`, `read_trust_roots` and `load::node` take the owner; `, owner` at their three callers in `apps/pool` |
| `94d2940` | M2 | The one line of `apps/pool/src/daemon.rs` that builds the server calls `Server::with_identity_query(server_parts, pool_control::PROBE_QUERY)` |

#### Tests and what they catch

| Test | Holds | Mutation | Result |
|---|---|---|---|
| `ac29_another_system_identifier_...`, `..._another_timeline_...`, `..._a_node_in_recovery_...`, `..._an_error_...`, `..._two_rows_...`, `..._a_null_...`, `..._no_row_...` in `tests/session.rs` | `08006` before `AuthenticationOk`, one backend, the question asked once, no `Query` of the client at the node, the backend closed, `backend.failed` with the class `identity` | the answer of step 8 not checked | all seven fail |
| `ac29_the_answer_of_the_approved_writer_lets_the_session_open`, `ac29_a_server_built_without_the_question_refuses_every_login` | the writer passes; `Server::new` fails closed | | pass |
| `ac30_a_query_in_flight_on_a_lost_node_gets_08007`, `ac30_an_idle_session_on_a_lost_node_gets_08006` | the mark ends the session with `08007` or `08006`, the backend closed, `session.ended` with `node_lost` | the mark ignored | both fail at their own limit of 15 s, no hang |
| `ac30_silence_without_the_mark_ends_no_session` | a cluster closed without a conflict or the mark keeps its session; a conflict over the mark gives `57P01` | | pass |
| `shutdown_now_ends_every_session_at_once_and_returns_within_its_limit` | with a deadline of 600 s: `57P01` idle and in a transaction, `08007` with a query in flight, back within 3 s, no accept after | `shutdown_now` as `shutdown` | fails at its limit of 5 s |
| `step_12_closes_the_backend_before_the_client_that_does_not_read` | the backend closed within 600 ms of a conflict while the client that does not read still holds the session | the old order | fails: 0 closed |
| `c6_a_ca_file_or_chain_that_others_can_write_or_that_another_account_owns_is_refused` in `secret.rs` | 644, 444, 600, 400, 640, 604 pass; 664, 646, 666, 620, 602, 660 and another owner refused; root allowed by the rule. The case of another owner is skipped when the test runs as root (`e19fe7e`): in the Linux image the file of the test belongs to root, which the rule allows | group write allowed | fails at 664 |

**One test that failed and then passed.** `ac30_a_query_in_flight_on_a_lost_node_gets_08007` failed 10 of 10 times when the three `ac30_` tests ran together (`assert!(eventually(|| !named(&events, "session.ended").is_empty()).await)` at line 2224), and passed when it ran alone, where the event was there with `reason` `node_lost`. The cause is in the test, not the code: `tracing` caches the interest of a callsite, and the other two tests ended sessions on threads with no subscriber. With a subscriber in all three, as the `ac24_` tests do, the whole `tests/session.rs` binary passed 10 of 10 runs, and later every run.

#### What the build switch does, run both ways (c4)

| Run | Linked SQLite | Result |
|---|---|---|
| With `.cargo/config.toml`, no variable | 3.54.1 | the version test passes |
| With the file and `LIBSQLITE3_SYS_USE_PKG_CONFIG=1` | 3.54.1 | passes: `force` wins |
| Without the file and `LIBSQLITE3_SYS_USE_PKG_CONFIG=1` | 3.54.0, the SQLite of the host from Homebrew `pkg-config` | **fails**: `left: "3.54.0"` |
| Without the file, no variable | 3.54.1 | passes |

For AWS-LC, with the file and `AWS_LC_SYS_USE_SYSTEM=1` the build is from source. **With the file and `AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin=1` the build script of `aws-lc-sys` 0.47.0 still looks for an installed AWS-LC**, because it reads the name with the target before the plain one (`optional_env_crate_target`), and `force` covers only the plain name. On this host it found none and the build failed; on a host that has one it would link it. This is a gap in the two lines of [Native code](index.md#native-code), for the architect. `libsqlite3-sys` 0.40.0 reads only the one name and then builds the bundled source, so SQLite is covered.

**The names with the target, after decision 99.** `.cargo/config.toml` now holds the block of [Native code](index.md#native-code): the plain line and one line for each of `aarch64_unknown_linux_gnu`, `x86_64_unknown_linux_gnu`, `aarch64_apple_darwin` and `x86_64_apple_darwin`. Run again on this host (`aarch64-apple-darwin`), with `cargo build -vv -p pool-core --locked`:

| `.cargo/config.toml` | Shell | Exit | What the build script of `aws-lc-sys` 0.47.0 saw |
|---|---|---|---|
| The block of decision 99 | `AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin=1` | 0 | `AWS_LC_SYS_USE_SYSTEM` `0` and `AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin` `0`; "Building with: CC" and `rustc-link-lib=static=aws_lc_0_47_0_crypto` from its own build folder, so the library was built from the source in the crate |
| The two lines of decision 87, for comparison | the same | 101 | `AWS_LC_SYS_USE_SYSTEM` `0` and `AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin` `1`: "AWS_LC_SYS_USE_SYSTEM=1 requires a system-installed AWS-LC, but none was found" |

The other three names with a target were not run: this host builds only for `aarch64-apple-darwin`. `scripts/check.sh all` with the block: exit 0.

**`cargo fuzz run` has no `--locked`** in `cargo-fuzz` 0.13.2. The check runs `cargo metadata --locked --manifest-path <crate>/fuzz/Cargo.toml` first, which refuses a lockfile that would have to change.

#### Checks on the final tree

On macOS unless a row says otherwise; Docker running. `0004 AC-29` and `0004 AC-30` are proven here in their hermetic halves only: the fixture cases and the control loop belong to `dev-f4-m2`.

| Command | Tree | Exit | Result |
|---|---|---|---|
| `scripts/check.sh all` | every commit of the table above, and `e19fe7e` | 0 | fmt, clippy, test, deny, guards, docs |
| `cargo test -p pool-core --locked` | `e19fe7e` | 0 | 56 unit and 67 session tests |
| `scripts/check.sh integration` | `94d2940` | 0 | 29 of 29 against PostgreSQL 18.7 of the pinned image. The commits from `68d9427` to `bb4df12` refuse every login there with `08006` and the class `identity`, because the daemon still built its server with `Server::new`; `94d2940` hands it the probe query. The branch reached `main` as one merge (`653d864`) |
| `scripts/check.sh fuzz` | `94d2940` | 0 | four targets, 31 s each, no crash file |
| `scripts/prove-checks.sh` | `6d7ad51` | 0 | 36 of 36, with case 36; `linux-build` not proven in that run |
| `cargo test --locked -p pool-core` in `rust:1.100.0-slim-bookworm` by its pinned digest, from `git archive HEAD`, as root | `bb4df12` | not recorded (the output went through `grep`) | `test result: FAILED. 55 passed; 1 failed`: the c6 test, at the case of another owner (the file belongs to root) |
| The same | `e19fe7e` | 0 | 56 unit and 67 session tests, `c6_`, `ac29_` and `ac30_` among them |
| `scripts/check.sh linux-build` | `94d2940` | 1 | **failed** in four tests of `apps/pool/tests/commands.rs` (two `ac9_` lock cases, `ac10_`, `ac11_`), each with `secret.permissions at node.tls_ca_file`. `git archive` writes the tree with `tar.umask` 002, so in the image `crates/pool-core/tests/fixtures/ca.pem` is mode 664, owned by root, and `commands.rs` names that committed file. The rule of decision 88 refuses the group write bit, as it must. The fix is on the test side, a copy of the CA with mode 0644 in the folder of the test, given to `dev-f4-m2`. Not run again yet |
| `scripts/prove-checks.sh --with-linux-build` | | not run | waits for that fix on `main` |

**After the fix of the CA test on `main`** (`6efe44c`, merged in `0f13d31`), on `bfc4a00`, which holds the four names of decision 99 (`770589a`) and the fuzz target `auth` of the tester in `check_fuzz` (`fuzz_target crates/pool-protocol auth 4096`):

| Command | Tree | Exit | Result |
|---|---|---|---|
| `scripts/check.sh all` | the tree of `bfc4a00` | 0 | fmt, clippy, test, deny, guards, docs |
| `scripts/check.sh fuzz` | the tree of `bfc4a00` | 0 | five targets, 31 s each (`validate`, `startup`, `frontend_message`, `scram`, `auth`), no crash file |
| `scripts/check.sh linux-build` | `bfc4a00` | 0 | the four tests of `commands.rs` pass in the image |
| `scripts/prove-checks.sh --with-linux-build` | `bfc4a00` | 0 | 36 of 36 cases rejected their bad change, `linux-build` proven |

