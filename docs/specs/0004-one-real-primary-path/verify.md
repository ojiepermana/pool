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
