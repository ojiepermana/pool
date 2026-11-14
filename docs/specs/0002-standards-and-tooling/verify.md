# 0002. Verification record

This file holds the acceptance evidence for [spec 0002](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

## Milestone 1: Dependency policy and fuzz harness

Build plan steps 1 and 2, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `da17dae` (step 1) and `d9f7a7b` (step 2). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17` (`cargo 1.101.0-nightly`).

### UI / manual

None for this milestone. `scripts/check.sh` does not exist yet (step 3), so the commands under it were run directly.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `deny.toml` compared with the block under [Dependency policy](index.md#dependency-policy) | no difference | no difference | pass |
| `cargo fmt --all --check` | exit 0 | exit 0 | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` (with the new Clippy table) | exit 0, no warning | exit 0, no warning, no source file changed | pass |
| `cargo test --workspace --locked` | exit 0 | exit 0, 134 passed, 0 failed | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, `configuration valid` | pass |
| `git diff --quiet Cargo.lock` against `main` | root lockfile unchanged | unchanged | pass |
| `cargo deny --locked --config deny.toml check` | exit 0, one `license-exception-not-encountered` warning | exit 0, `advisories ok, bans ok, licenses ok, sources ok`, the one expected warning | pass |
| `cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check` | exit 0 | exit 0, `advisories ok, bans ok, licenses ok, sources ok`, no warning | pass |
| `cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml` | exit 0 | exit 0 (see the note below) | pass |
| From `crates/pool-config`, `CARGO_TARGET_DIR=<root>/target/fuzz cargo +nightly-2026-10-17 fuzz run validate fuzz/corpus/validate tests/fixtures -- -max_total_time=10 -max_len=4096` | builds, exit 0, no crash file | exit 0, 217586 runs in 11 seconds, no `fuzz/artifacts/validate/` | pass (short run that creates the lockfile; the 60 second run is step 8) |
| Package names of `crates/pool-config/fuzz/Cargo.lock` compared with `fuzz-lock-packages.txt` in the spec | the same 24 names | the same 24 names, `libfuzzer-sys` 0.4.13 | pass |
| `git status --short --ignored crates/pool-config/fuzz` after the run | only `corpus/` ignored | only `corpus/` ignored | pass |

**Note on `validate.rs`.** The spec text puts the `Ok` arm on one line of 97 characters. `cargo fmt --check` on the fuzz manifest, which the `fmt` check runs, rejects that layout. The committed file is the same code in the layout rustfmt writes (the `assert_eq!` arguments on their own lines). No token changed.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-20` | Clippy table in the root `Cargo.toml`, clippy run above | pass on the workstation (the `scripts/check.sh clippy` run waits for step 3) |
| `0002 AC-4` | Policy half: `deny.toml` content, allowed licenses exactly `Apache-2.0`, `MIT`, `Unicode-3.0` plus the `NCSA` exception for `libfuzzer-sys`, `ignore = []`, both `cargo deny` runs pass | policy half pass; the `scripts/check.sh deny` half waits for milestone 2 (step 3) |
| `0002 AC-15` | Package half: the four files under `crates/pool-config/fuzz/` and the lockfile names | package half pass; the 60 second `scripts/check.sh fuzz` run waits for step 8 |
| Other criteria | Not part of this milestone | not run |

### Checker run: `/check verify`, milestone 1

Recorded by the checker session `chk-f2-verify` on 2026-11-14 at revision `59e4cf5`, on macOS (workstation, not Linux and not CI). The checker did not write the code. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`.

**Verdict: PASS for milestone 1.** It covers `0002 AC-20`, the policy half of `0002 AC-4` and the package half of `0002 AC-15`. The parts of those criteria that need `scripts/check.sh`, the 60 second fuzz run and CI are outside this milestone and were not run.

- [x] `deny.toml` compared with the block under [Dependency policy](index.md#dependency-policy) → `diff` prints nothing, exit 0. `allow` is exactly `Apache-2.0`, `MIT`, `Unicode-3.0`, the one exception is `NCSA` for `libfuzzer-sys`, and `ignore = []` → `0002 AC-4`
- [x] `cargo deny --locked --config deny.toml check` → exit 0, `advisories ok, bans ok, licenses ok, sources ok`, and the one warning `license-exception-not-encountered` at `deny.toml:16` → `0002 AC-4`
- [x] `cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check` → exit 0, `advisories ok, bans ok, licenses ok, sources ok`, no warning → `0002 AC-4`
- [x] `git diff 6de6970 59e4cf5 -- Cargo.toml` → the only change is the table `[workspace.lints.clippy]` with the five lints, the same text as under [Lint rules](index.md#lint-rules) → `0002 AC-20`
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → exit 0, no warning → `0002 AC-20`
- [x] Lint probe (extra): one temporary function added to `apps/pool/src/main.rs`, then to `crates/pool-config/src/lib.rs`, once for each of `println!`, `eprintln!`, `dbg!`, `todo!()` and `unimplemented!()`. All ten clippy runs exit 101 and name the lint: `print_stdout`, `print_stderr`, `dbg_macro`, `todo`, `unimplemented`. Each file was restored with `git checkout`, and clippy passes again on the restored tree → `0002 AC-20`
- [x] `cargo fmt --all --check` and `cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml` → exit 0 for both
- [x] `cargo test --workspace --locked` → exit 0, 134 passed (3 + 66 + 3 + 60 + 2), 0 failed
- [x] `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` → exit 0, `configuration valid`
- [x] `git diff --quiet 6de6970 -- Cargo.lock` → exit 0, the root lockfile is unchanged and still holds 18 packages. `cargo metadata --locked --no-deps --format-version 1` → packages `['pool', 'pool-config']`, 2 workspace members → `0002 AC-15` (the fuzz package stays outside the workspace)
- [x] `git ls-files crates/pool-config/fuzz` → exactly `.gitignore`, `Cargo.lock`, `Cargo.toml` and `fuzz_targets/validate.rs`. `Cargo.toml` equals the spec block (`diff` exit 0). `.gitignore` is the four lines `target`, `corpus`, `artifacts`, `coverage` → `0002 AC-15`
- [x] `validate.rs` compared with the spec block → the same text once spaces and line breaks are removed (`diff` exit 0). The layout differs, see the first reading below → `0002 AC-15`
- [x] Package names of `crates/pool-config/fuzz/Cargo.lock` compared with `fuzz-lock-packages.txt` in the spec → the same 24 names, `libfuzzer-sys` at 0.4.13 → `0002 AC-15`
- [x] From `crates/pool-config`, `CARGO_TARGET_DIR=<root>/target/fuzz cargo +nightly-2026-10-17 fuzz run validate fuzz/corpus/validate tests/fixtures -- -max_total_time=10 -max_len=4096` → builds, exit 0, 179059 runs in 11 seconds, no file under `fuzz/artifacts/validate/`, and the committed fuzz lockfile is unchanged after the build → `0002 AC-15`
- [x] `git status --short --ignored crates/pool-config/fuzz` after the run → only `corpus/` ignored. `git status --short` → empty → `0002 AC-15`

**Readings the checker assessed**

- **Layout of `validate.rs`.** The spec block has one line of 97 characters. As a probe, the spec block was written to the file as it stands: `cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml` then exits 1 and asks for the layout that is committed. So the spec text cannot pass the `fmt` check that the same spec defines. The committed file is the same code, token for token, in the layout rustfmt writes. The checker reads this as meeting the package half of `0002 AC-15`. The code block in the spec should be changed to the committed layout so that both say the same thing. That change belongs to the architect.
- **The warning on the product lockfile.** The spec says under [Required checks](index.md#required-checks) that the product run of `deny` prints one warning, `license-exception-not-encountered`, because the `NCSA` exception is used only by the fuzz lockfile. That is what was observed: one warning on the product lockfile, none on the fuzz lockfile, exit 0 for both. It is the expected result.
- **The artifacts directory.** After the fuzz run in this session the directory `crates/pool-config/fuzz/artifacts/validate/` exists and is empty (`find ... -type f` counts 0). `cargo-fuzz` creates it. The developer's row above says the directory was not there. The rule in the spec is "no file under" that directory, and that holds. A script that tests for the directory instead of for a file in it would report a crash that did not happen.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0002 AC-20` | met on the workstation: the table is in the root `Cargo.toml`, clippy passes, and each of the five lints rejects a probe in both packages. The run through `scripts/check.sh clippy` is outside this milestone |
| `0002 AC-4` | policy half met: file content, the license list, the one exception, no ignored advisory, four checks pass on both lockfiles. The run through `scripts/check.sh deny` is outside this milestone |
| `0002 AC-15` | package half met: four files, the lockfile names, the target builds and runs with no crash file. The 60 second run through `scripts/check.sh fuzz` is outside this milestone |
| All other criteria | outside this milestone, not run |

## Milestone 2: Check entry point and CI files

Build plan steps 3 and 4, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `9747ff0` (step 3), `ba0a046` (step 4) and `7c752a9` (step 3 fix: the `fuzz` crash check looks for a file under `fuzz/artifacts/validate/`, not for the directory, which `cargo-fuzz` creates on every run; a finding of the milestone 1 checker). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, rustup 1.29.2, Docker server 29.9.2.

### UI / manual

None for this milestone. The CI half of `0002 AC-5`, `0002 AC-3`, `0002 AC-12` and the remote half of `0002 AC-13` are Build plan step 11, run by the lead: `NOT_RUN`, `owner: lead`.

### Commands

Run from the repository root unless the row says otherwise. Files are committed with mode `100755` for both scripts.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh` (no name), from `/tmp` | exit 2, usage on stderr, nothing run | exit 2, stdout empty, usage on stderr | pass |
| `scripts/check.sh fmt bogus` | exit 2, nothing run | exit 2, stdout empty, `scripts/check.sh: unknown check: bogus` and usage on stderr | pass |
| `scripts/check.sh fmt clippy test deny` | exit 0, `== ok: <name>` for each, then a summary | exit 0, four `== ok:` lines and the summary; 134 tests passed, 0 failed; both `cargo deny` runs `advisories ok, bans ok, licenses ok, sources ok`, the one expected `license-exception-not-encountered` warning | pass |
| `scripts/check.sh guards` | exit 3, one line naming what is missing (`tools/repo-check` arrives with milestone 3) | exit 3, `== FAILED: guards`, `pool-repo-check is required and tools/repo-check/Cargo.toml does not exist` | pass (green waits for milestone 3) |
| `scripts/check.sh docs` | exit 3, same line | exit 3, same line | pass (green waits for milestone 3) |
| `scripts/check.sh all`, from `/tmp` | runs all six without stopping; exit 3 while `guards` and `docs` cannot run | exit 3; summary `ok` for `fmt`, `clippy`, `test`, `deny`, `FAILED` for `guards`, `docs` | pass (exit 0 waits for milestone 3) |
| `scripts/check.sh tools` with `cargo-deny 0.20.2` already on `PATH` | exit 0, nothing installed | exit 0, `cargo-deny 0.20.2 is already available`, no `target/tools` | pass |
| `scripts/check.sh deny` with a stub `cargo-deny` that prints `cargo-deny 0.19.0` first on `PATH` | exit 3, the required version line | exit 3, `cargo-deny 0.20.2 is required; run scripts/check.sh tools` | pass |
| `scripts/check.sh deny guards` with a stub `cargo-deny` that reports 0.20.2 and fails `check` | exit 1 (a failure and a could not run in one call) | exit 1, both `FAILED`, the check stopped after its first failing command | pass |
| `PATH=/usr/bin:/bin scripts/check.sh linux-build` (no `docker` on `PATH`, daemon still running) | exit 3, one line | exit 3, `linux-build: docker is required and was not found on PATH` | pass |
| `DOCKER_HOST=unix:///nonexistent.sock scripts/linux-build.sh` (daemon unreachable) | exit 3 | exit 3, `runner: Darwin arm64`, `linux-build: the Docker daemon is not running` | pass |
| `scripts/check.sh linux-build` with an untracked file | exit 3 | exit 3, `linux-build: commit or remove local changes first; this check builds only the committed tree` | pass |
| `scripts/check.sh linux-build` on `ba0a046`, clean tree | exit 0, `runner:`, `host: aarch64-unknown-linux-gnu`, last line `linux build check passed` | exit 0, `runner: Darwin arm64`, `host: aarch64-unknown-linux-gnu`, 136 tests passed, 30 fixture blocks with the wanted exit, `sha256sum` `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool`, `pool 0.1.0`, `linux build check passed`, `== ok: linux-build` | pass (workstation reproduction, not the `0002 AC-5` evidence) |
| Text of `scripts/linux-build.sh` after `# --- spec 0001, Linux build ---` compared with the first block under `### Linux build` in spec 0001 on `main` | no difference | no difference (`diff`) | pass (rule W7 itself is milestone 3) |
| The three `.github/` files compared with the blocks in the spec | no difference | written by extracting the spec blocks; JSON parses, YAML parses with the seven jobs `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards`, `docs` | pass |
| Shape of `ci.yml` and `main.json` against rules C1 to C5, read by `grep` and `awk` | C1 to C5 hold | every `uses:` is `owner/name@<40 hex> # v…`; job ids equal the seven `context` values; `run:` values are only `rustup toolchain install` and `scripts/check.sh <name>`, last one per job its own id; every `runs-on:` is `ubuntu-24.04-arm`; one `permissions:` line followed by `  contents: read`; no `pull_request_target`, no `secrets.` | pass (the program check is milestone 3) |
| `gh api repos/actions/checkout/tags`, `gh api repos/actions/cache/tags`, `gh api /apps/github-actions` (read only) | `v7.0.1` = `3d3c42e5…`, `v6.1.0` = `55cc8345…`, id 15368 | same | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, `configuration valid` | pass |
| The crash test of `fuzz` (`find <dir> -type f`) on a scratch directory: empty, with an empty subdirectory, with one file | pass, pass, crash | pass, pass, crash | pass (the full `fuzz` run is step 8) |

Not run in this milestone: `scripts/check.sh fuzz` and `scripts/check.sh sbom` (Build plan step 8; `cargo-cyclonedx` is not installed), the `POOL_CHECK_BASE` selection of `docs` (it is reached only once `tools/repo-check` exists), and `cargo install` in `tools` (the pinned version was already present).

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-1` | Usage, exit status and could not run rows above; `fmt`, `clippy`, `test`, `deny` end with `== ok:` | pass for `fmt`, `clippy`, `test`, `deny` and the status rules; `guards`, `docs` and `all` exit 0 wait for milestone 3 |
| `0002 AC-2` | `ci.yml` copied from the spec; seven jobs on `ubuntu-24.04-arm`; `tools` before `deny`; triggers, read only permission, no secret, actions by full hash | pass (file); the run is `0002 AC-3`, step 11 |
| `0002 AC-4` | `scripts/check.sh deny` checks the version, runs both lockfiles, passes | pass (completes the milestone 1 policy half) |
| `0002 AC-5` | Script half: the copy, the fixed first part and a local reproduction | script half pass; rule W7 is milestone 3; the CI log on `ubuntu-24.04-arm` is step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-13` | `main.json` copied from the spec | file half pass; remote half step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-14` | `pull_request_template.md` copied from the spec | file half pass; `CONTRIBUTING.md` is step 9 |
| Other criteria | Not part of this milestone | not run |

### Checker run: `/check verify`, milestone 2

Recorded by the checker session `chk-f2-verify` on 2026-11-14 at revision `fb4f93d`, on macOS (workstation, not Linux and not CI). The checker did not write the code. Every command was run again from this session's own checkout. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `nightly-2026-10-17`, Docker server 29.9.2 (`linux/aarch64`), `/bin/sh` of macOS and `dash`. Each probe that changed a file was undone with `git checkout` or `rm`, and `git status --short` was empty after each one.

**Verdict: PASS for milestone 2.** It covers the part of `0002 AC-1` that can hold before `tools/repo-check` exists, `0002 AC-2` as a file, the `scripts/check.sh` half of `0002 AC-4`, the script half of `0002 AC-5`, and the file halves of `0002 AC-13` and `0002 AC-14`. A green `guards`, `docs` and `all`, rule W7, the 60 second fuzz run, the inventory run, `CONTRIBUTING.md` and everything on GitHub are outside this milestone and were not run.

The entry point, `0002 AC-1`:

- [x] `git ls-files -s scripts` → mode `100755` for `scripts/check.sh` and `scripts/linux-build.sh`. Line 1 is `#!/bin/sh` and line 5 is `set -eu`. The four pinned versions are written once, as variables on lines 8 to 11 → `0002 AC-1`
- [x] `scripts/check.sh` with no name, called from `/tmp` → exit 2, stdout empty, the usage text on stderr → `0002 AC-1`
- [x] `scripts/check.sh fmt bogus`, `scripts/check.sh bogus fmt`, `scripts/check.sh ""` and `scripts/check.sh ALL` → exit 2 for each, stdout empty (nothing was run), `scripts/check.sh: unknown check: <name>` and the usage text on stderr → `0002 AC-1`
- [x] `scripts/check.sh fmt clippy test deny` with `CARGO_INCREMENTAL` and `CARGO_PROFILE_DEV_DEBUG` unset → exit 0. Each name prints `== check: <name>` and then `== ok: <name>`, followed by `== summary` and the four `== ok:` lines again. 134 tests passed (3 + 66 + 3 + 60 + 2), 0 failed. Both `cargo deny` runs print `advisories ok, bans ok, licenses ok, sources ok`, with the one expected warning `license-exception-not-encountered` → `0002 AC-1`, `0002 AC-4`
- [x] `scripts/check.sh fmt` and `scripts/check.sh deny`, one name each → exit 0, the first line is `== check: <name>`, the last line is `== ok: <name>`, and there is no summary → `0002 AC-1`
- [x] `sh -x scripts/check.sh fmt clippy test deny` → the commands that ran are `cargo fmt --all --check`, `cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `cargo deny --locked --config deny.toml check` and `cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check`. These are the commands under [Required checks](index.md#required-checks), without the three for `tools/repo-check` (see the first reading below) → `0002 AC-1`, `0002 AC-4`
- [x] `scripts/check.sh guards` and `scripts/check.sh docs` → exit 3 for each, `== FAILED: <name>` on stdout, and one line on stderr: `pool-repo-check is required and tools/repo-check/Cargo.toml does not exist`. This is the could not run result that is expected until milestone 3 → `0002 AC-1`
- [x] `scripts/check.sh all`, called from `/tmp` → runs `fmt`, `clippy`, `test`, `deny`, `guards`, `docs` in that order without stopping, exit 3, summary `ok` for the first four and `FAILED` for `guards` and `docs` → `0002 AC-1` (exit 0 waits for milestone 3)
- [x] Failing check probe: one badly formatted function added to `apps/pool/src/main.rs`. `scripts/check.sh fmt` → exit 1, a `Diff in` block, `== FAILED: fmt`. `scripts/check.sh all` → exit 1, `fmt` and `clippy` `FAILED`, `test` and `deny` still ran and are `ok`, `guards` and `docs` `FAILED`. So `all` keeps going after a failure, and status 1 wins over status 3 → `0002 AC-1`
- [x] Failing check probe: the same bad function added only to `crates/pool-config/fuzz/fuzz_targets/validate.rs`. `scripts/check.sh fmt` → exit 1, the `Diff in` block names that file. So the fuzz manifest is checked even while the `tools/repo-check` command is skipped → `0002 AC-1`
- [x] No `cargo` on `PATH` (`PATH=/usr/bin:/bin`), once for each of `fmt`, `clippy`, `test`, `deny`, `guards`, `docs`, `tools`, `fuzz`, `sbom` → exit 3 for all nine, one line on stderr: `<name>: cargo is required and was not found on PATH` → `0002 AC-1`
- [x] `scripts/check.sh deny` with a stub `cargo-deny` that prints `cargo-deny 0.19.0`, first on `PATH` → exit 3, `cargo-deny 0.20.2 is required; run scripts/check.sh tools`. The same result when `cargo-deny` is not there at all (the toolchain `cargo` with an empty `CARGO_HOME`) → `0002 AC-1`, `0002 AC-4`
- [x] `scripts/check.sh deny guards` with a stub `cargo-deny` that reports 0.20.2 and fails `check` → exit 1, both `FAILED`. The stub was called once, for the product lockfile only, so a check stops at its first failing command. `scripts/check.sh guards deny fmt` with the same stub → exit 1, and `fmt` still ran and is `ok`. `scripts/check.sh guards fmt` with the real tools → exit 3 → `0002 AC-1`
- [x] The same stub also wrote down its environment: `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=0` when both were unset, `1` and `2` when they were set to `1` and `2` before the call, and the first entry of `PATH` is `<root>/target/tools/bin` → `0002 AC-1`
- [x] `scripts/check.sh tools` with `cargo-deny 0.20.2` on `PATH` → exit 0, `cargo-deny 0.20.2 is already available`, and `target/tools` does not exist afterward → `0002 AC-4`
- [x] `scripts/check.sh tools` with a stub `cargo` that reports another version and writes down its arguments (nothing is installed) → the script calls `cargo deny --version` and then `cargo install --locked --root target/tools cargo-deny@0.20.2`, exit 0. When the stub fails `install` → exit 3, `tools: cargo install of cargo-deny 0.20.2 failed` → `0002 AC-1`, `0002 AC-4`
- [x] `scripts/check.sh fuzz sbom` with the same stub → exit 3, the two lines `cargo-fuzz 0.13.2 is required; run cargo install --locked --root target/tools cargo-fuzz@0.13.2` and `cargo-cyclonedx 0.5.10 is required; run cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. `scripts/check.sh sbom` with the real tools (`cargo-cyclonedx` is not installed) → exit 3, the same line. `scripts/check.sh fuzz` with a stub `rustup` that lists no nightly → exit 3, `the toolchain nightly-2026-10-17 is required; run rustup toolchain install nightly-2026-10-17` → `0002 AC-1` (the real runs of `fuzz` and `sbom` are Build plan step 8)
- [x] `sh -x scripts/check.sh docs`, five times → with `POOL_CHECK_BASE` unset, empty or forty zeros the call is `repo_check docs --base origin/main`. With `POOL_CHECK_BASE=6de6970…` it is `repo_check docs --base 6de697005c5f23be9d176a7ee2109ff07becd1f2`. In a fresh `git init` copy with no `origin/main` it is `repo_check docs` with no base. Each run then ends with exit 3, because `tools/repo-check` is not there yet → `0002 AC-1`
- [x] `dash -n` on both scripts → no syntax error. `dash scripts/check.sh fmt clippy test deny guards` → exit 3, the same lines as with the `/bin/sh` of macOS. `dash scripts/check.sh nope` → exit 2. `dash` is the `/bin/sh` of the Ubuntu runner → `0002 AC-1`

The Linux build script, `0002 AC-5` (script half):

- [x] The text of `scripts/linux-build.sh` up to and including the marker line, compared with the block under [Linux build script](index.md#linux-build-script) → `cmp` exit 0. The text after the marker line, compared with the first code block under `### Linux build` in spec 0001 (the same file as on `main`) → `cmp` exit 0, 1100 bytes, 28 lines. The marker line occurs once → `0002 AC-5`
- [x] Copy probe: `cargo test --locked --workspace` changed to `cargo test --locked --workspace --no-run` below the marker. The same comparison then prints that one line as different and exits 1. The file was restored → `0002 AC-5` (rule W7, which makes this comparison a check, is milestone 3)
- [x] `PATH=/usr/bin:/bin scripts/check.sh linux-build` (no `docker` on `PATH`, daemon running) → exit 3, one line: `linux-build: docker is required and was not found on PATH`. With `docker` on `PATH` and no `git` → exit 3, `linux-build: git is required and was not found on PATH` → `0002 AC-1`
- [x] `DOCKER_HOST=unix:///nonexistent.sock scripts/check.sh linux-build` (daemon cannot be reached) → exit 3, `runner: Darwin arm64`, then `linux-build: the Docker daemon is not running`. `scripts/linux-build.sh` called directly gives the same two lines and exit 3 → `0002 AC-1`
- [x] `scripts/check.sh linux-build` with one untracked file, and again with one changed tracked file → exit 3 both times, `linux-build: commit or remove local changes first; this check builds only the committed tree` → `0002 AC-1`
- [x] `scripts/check.sh linux-build` on the clean tree at `fb4f93d` → exit 0. `runner: Darwin arm64`, `host: aarch64-unknown-linux-gnu`, `release: 1.100.0`, 136 tests passed (3 + 68 + 3 + 60 + 2), 0 failed, 30 fixture lines (6 with `exit=0`, 24 with `exit=1`, none unexpected, no `canary leaked`), `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool` (the same hash as in the developer's run), `pool 0.1.0`, `linux build check passed`, `== ok: linux-build` → `0002 AC-5` (a workstation reproduction; the evidence the criterion asks for is the CI log of step 11)

The files under `.github/`, `0002 AC-2`, `0002 AC-13`, `0002 AC-14` (file halves):

- [x] `.github/workflows/ci.yml`, `.github/rulesets/main.json` and `.github/pull_request_template.md`, each compared with its code block in the spec → `cmp` exit 0 for all three → `0002 AC-2`, `0002 AC-13`, `0002 AC-14`
- [x] `ci.yml` read with a YAML parser (`ruby -ryaml`) → the jobs are exactly `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards`, `docs`. Every `runs-on` is `ubuntu-24.04-arm`. The triggers are `pull_request`, `push` to `main` and `schedule` with `17 3 * * *`. `permissions` is `contents: read` and nothing else. The job `deny` runs `scripts/check.sh tools` and then `scripts/check.sh deny` → `0002 AC-2`
- [x] Rules C1 to C5, read with `grep` and `awk` → C1: 8 `uses:` lines, all 8 of the form `owner/name@<40 hexadecimal characters> # v…`. C2: the 7 job ids after `jobs:` equal the 7 `"context"` values, in the same order. C3: the `run:` values are `rustup toolchain install` (6 times) and `scripts/check.sh <name>` (8 times), and the last `run:` of each job is `scripts/check.sh <its job id>`. C4: 7 `runs-on:` lines, all `ubuntu-24.04-arm`. C5: one `permissions:` line at the start of a line, followed by `  contents: read`, and no `pull_request_target` and no `secrets.` in `.github/workflows/` → `0002 AC-2` (the program that enforces these rules is milestone 3)
- [x] `gh api repos/actions/checkout/git/ref/tags/v7.0.1` → a commit, `3d3c42e5aac5ba805825da76410c181273ba90b1`. `gh api repos/actions/cache/git/ref/tags/v6.1.0` → a commit, `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`. `gh api /apps/github-actions` → id `15368`. All three are read only calls and match the files → `0002 AC-2`, `0002 AC-13`
- [x] `main.json` read with a JSON parser → `enforcement` is `active`, `bypass_actors` is empty, the condition is `~DEFAULT_BRANCH`, the four rules are `deletion`, `non_fast_forward`, `pull_request` and `required_status_checks`, the seven contexts each carry `integration_id` 15368, the only merge method is `merge`, and the number of required approvals is 0 → `0002 AC-13`
- [x] `rustup toolchain install` with no name, the command six of the CI jobs run, in this checkout → exit 0, `using existing install for 1.100.0-aarch64-apple-darwin`, the toolchain list is unchanged (see the last reading below) → `0002 AC-2`

**Readings the checker assessed**

- **The skipped `tools/repo-check` commands.** While `tools/repo-check/Cargo.toml` does not exist, `fmt`, `clippy` and `test` leave out their command for that package and print `<name>: tools/repo-check does not exist yet; its command was skipped`. `guards` and `docs` exit 3 with one line. The checker reads this as what Build plan step 3 asks for: it says `fmt`, `clippy`, `test` and `deny` pass after steps 1 and 2, and they could not pass if they ran a command on a package that step 5 creates. It is also what `0002 AC-1` allows at this point, since a missing tool is a could not run result. The skip is marked `# provisional: spec 0002 step 5` in the script. It should be taken out when milestone 3 is merged. If it stays, a later change that loses `tools/repo-check/Cargo.toml` would let `fmt`, `clippy` and `test` print `== ok:` without having checked that package. The line marked `# provisional: spec 0002 step 8` in `sbom` is the second one of this kind.
- **Where the lines go, and how failures add up.** The line that says what is missing goes to stderr. The `==` lines go to stdout. Inside one check the commands stop at the first one that fails, and the next name still runs. Over one call, 1 wins over 3. All of this was observed above. It fits the status rule of the spec and the [shared convention](../../engineering-standards.md#shared-conventions) (results on stdout). One effect to know: when the product lockfile fails `deny`, the fuzz lockfile is not looked at in that run.
- **`docker` and `git` looked up first.** `scripts/check.sh` checks that both are on `PATH` before it calls `scripts/linux-build.sh`. The fixed first part of that script cannot tell a missing `docker` from a stopped daemon, and it has no answer for a missing `git`. With the lookup in `check.sh`, each case gives exit 3 and one clear line, as `0002 AC-1` asks.
- **Worked, but looked fragile: `CDPATH`.** Both scripts start with `cd "$(dirname "$0")/.."`. When `CDPATH` is exported and names a directory that has a `scripts` directory in it, and the script is called by a relative path, `cd` goes to that other place and prints its path. Probe: `CDPATH=<dir with scripts/> scripts/check.sh fmt` → exit 1, `cargo` cannot find `Cargo.toml`. Calling the script by its full path is not affected, and few people export `CDPATH`. No criterion covers this. The line in `scripts/linux-build.sh` is fixed by the spec, so a change would start there. It is a note for `/check review`.
- **`rustup toolchain install` updates rustup itself.** The first run of this command in the checker session printed `downloading self-update (new version: 1.29.3)` and moved rustup on this workstation from 1.29.2 to 1.29.3. No toolchain was added or removed. A second run printed `using existing install for 1.100.0-aarch64-apple-darwin` and exit 0. So the form with no name works with rustup 1.29.3 too, which is the version the runner image list in the spec names. The spec note on this command says it was tried with 1.29.2 and is not yet seen on the runner. That is still true.

**Not run by the checker**

- A real `cargo install` in `tools` (nothing may be installed in this session; the call was observed with a stub).
- The real runs of `scripts/check.sh fuzz` and `scripts/check.sh sbom` (Build plan step 8).
- A green `guards`, `docs` and `all`, and rule W7 (milestone 3).
- A missing `rustfmt` or `clippy` component of the toolchain. Only a missing `cargo` was probed.
- Anything under [Remote proof](index.md#remote-proof), and any run on a GitHub runner (Build plan step 11, `owner: lead`).

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0002 AC-1` | met for this milestone: every name runs, `fmt`, `clippy`, `test` and `deny` exit 0 and end with `== ok: <name>`, `all` runs six names in order and keeps going, an unknown name exits 2 with the usage text on stderr, a missing tool, a wrong version, a stopped daemon and a tree with changes exit 3 with one line. `guards`, `docs` and `all` exit 3 until milestone 3, and the provisional skip must be removed then |
| `0002 AC-2` | file met: seven jobs, one runner label, `tools` before `deny`, three triggers, read only permission, no secret, both actions named by the commit hash of their tag. The run is `0002 AC-3`, step 11 |
| `0002 AC-4` | completed: `scripts/check.sh deny` requires `cargo-deny 0.20.2`, runs the four checks on both lockfiles and passes. `tools` does nothing when that version is present and would install only under `target/tools` |
| `0002 AC-5` | script half met: the second part is the spec 0001 command byte for byte, the first part is the spec 0002 text, and the local run passes on `aarch64-unknown-linux-gnu`. Rule W7 is milestone 3. The CI log is step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-13` | file half met: `main.json` equals the spec. The remote half is step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-14` | file half met: the template equals the spec. `CONTRIBUTING.md` is step 9 |
| All other criteria | outside this milestone, not run |

## Milestone 3: Repository guards and document checks

Build plan steps 5 and 6, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `c70cf9e` (step 5) and `fd1b5d4` (step 6), checked on the tree of `59ab36d` (those commits merged with `main` at `fb4f93d`, which holds milestones 1 and 2). Tools: `rustc 1.100.0`, `git`. The package `tools/repo-check` has no dependencies; its `Cargo.lock` holds the one package `pool-repo-check`.

### UI / manual

None for this milestone.

### Commands

Run from the repository root. `pool-repo-check <args>` stands for `cargo run -q --locked --manifest-path tools/repo-check/Cargo.toml --target-dir target/repo-check -- <args>`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo fmt --check --manifest-path tools/repo-check/Cargo.toml` | exit 0 | exit 0 | pass |
| `cargo clippy --manifest-path tools/repo-check/Cargo.toml --all-targets --locked --target-dir target/repo-check -- -D warnings` | exit 0, no warning | exit 0, no warning | pass |
| `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check` | every rule W1 to W7, C1 to C5, D1 to D6 has a passing and a failing case | 71 passed, 0 failed. Passing cases: `good_tree_passes` (workspace), `good_definition_passes` (ci), `good_tree_passes` (docs), `good_tree_passes_with_and_without_base` (ids). Failing cases: tests named after their rule, such as `w1_extra_package_in_lockfile`, `c3_last_run_is_another_check`, `d6_removed_row` | pass |
| `cargo metadata --no-deps --format-version 1` at the root; `git diff main -- Cargo.lock` | packages `pool` and `pool-config` only; root lockfile unchanged | same; no difference | pass |
| `ls tools/repo-check` after the runs above | no `target/` | `Cargo.lock`, `Cargo.toml`, `baseline`, `src` | pass |
| `pool-repo-check workspace` | exit 0, `pool-repo-check workspace: ok` | exit 0, `pool-repo-check workspace: ok` | pass |
| `pool-repo-check ci` | exit 0, `pool-repo-check ci: ok` | exit 0, `pool-repo-check ci: ok` | pass |
| `pool-repo-check docs`, and with `--base 4d03515` (the base before this milestone) and `--base main` | exit 0 | exit 1, one problem in all three runs: `D4: docs/specs/0001-initial-architecture/index.md:712: 0001 AC-2 is defined again; it is first defined at docs/specs/0001-initial-architecture/index.md:25` | fail (the document is wrong, not the rule; the lead is fixing spec 0001 in a separate item) |
| `pool-repo-check docs --base nosuchrev` | exit 3, the `git` failure named | exit 3, `could not run: git ls-tree -r --name-only -z nosuchrev --: fatal: Not a valid object name nosuchrev` | pass |
| `pool-repo-check lint`; `pool-repo-check workspace --base x` | exit 2, usage on stderr | exit 2, `unknown command lint` and `unexpected argument --base`, then usage | pass |
| `scripts/check.sh guards` | exit 0, `== ok: guards` | exit 0, `pool-repo-check workspace: ok`, `pool-repo-check ci: ok`, `== ok: guards` | pass |
| `scripts/check.sh docs`; the same with `POOL_CHECK_BASE=4d03515…` and with forty zeros | exit 0 | exit 1, `== FAILED: docs`, the same single D4 problem in all three runs | fail (same cause as above) |
| Rerun on `4947c5c` (merged with `main` at `ab7e9ef`, which fixes spec 0001 line 712): `pool-repo-check workspace`, `ci`, `docs`, `docs --base 4d03515`, `docs --base main`, `scripts/check.sh guards`, `scripts/check.sh docs` | exit 0 | exit 0 for each, `ok` and no problem | pass |
| `scripts/check.sh fmt clippy test` | each runs the `tools/repo-check` command too, no "was skipped" line | exit 0; `pool-repo-check` checked by clippy and its 71 tests run; no "was skipped" line | pass |
| Cases 8 to 19 and 21 of [Proving the checks](index.md#proving-the-checks), plus one extra case each for W6, C3, C4 and C5, applied one at a time to a `git clone` of `59ab36d` (`docs` with `--base HEAD`) | exit 1 with the rule ID and the named text | all exit 1: `W1 … zmij`, `W2: crates/pool-core`, `W3: apps/pool/src/main.rs:94: forbidden text std::thread`, `W4: apps/pool/Cargo.toml`, `W5 … pool-config -> pool is not allowed`, `W6: x/Cargo.lock`, `W7: scripts/linux-build.sh:42`, `C1 … actions/checkout@v7`, `C2 … job documents`, `C3 … job fmt runs cargo fmt --all --check`, `C4 … runs-on is ubuntu-24.04`, `C5: .github/workflows/ci.yml:10`, `D1 … docs/missing-file.md`, `D2 … no-such-heading`, `D3 … status disagrees`, `D5` for the made up ID of line K, `D6 … SEC-14` | pass (a manual run; `scripts/prove-checks.sh` is step 7) |

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-6` | `workspace` passes on the real tree; each of W1 to W7 has failing tests and was rejected on a copy of the real tree | pass |
| `0002 AC-7` | `ci` passes on the real `ci.yml` and `main.json`; each of C1 to C5 has failing tests and was rejected on a copy | pass |
| `0002 AC-8` | D1 and D2 tests; cases 15 and 16 | pass (green on `4947c5c`) |
| `0002 AC-9` | D3 tests (status, name, number, unknown status and tag, file without a row, repeated number, file name prefix); case 17 | pass |
| `0002 AC-10` | D4, D5, D6 tests, `--base` with a real revision; cases 18 and 19 | pass (green on `4947c5c`) |
| Other criteria | Not part of this milestone | not run |

How the developer read the criteria where the spec leaves room:

- The tests use trees held in memory (unit tests under `src/`). The spec's list of new paths has no `tests/`, and `CARGO_TARGET_TMPDIR` is set only for integration tests. A file that a rule must read and that is missing makes the command exit 3. The other rules still run and their problems are still printed.
- C4 reads only `ci.yml`, because the rule table is introduced as reading that file and `main.json`; only C1 and C5 name every workflow file.
- C5 allows exactly one `permissions:` line in all of `ci.yml`. It must be at the start of a line and be followed by `  contents: read` and nothing more indented under it, so a permission set per job is refused too.
- Heading anchors follow the spec's steps exactly. A closing run of `#` on a heading is not removed, because the spec does not say to.
- A spec or PRD criterion is defined by every list item that starts with `**AC-<n>`, anywhere in the file, as the table says; the definitions are not limited to the Requirements section. That is why the bullet at spec 0001 line 712 counts. A citation must end at a word boundary, so `SEC-1x` is not a citation. Functional requirements and safety contracts are the numbered items that start at the beginning of a line, up to the next heading of level 1 or 2.

### Checker run: `/check verify`, milestone 3

Recorded by the checker session `chk-f2-verify` on 2026-11-14 at revision `ffa14f2` (`main`, which holds steps 5 and 6 and the spec 0001 fix `ab7e9ef`), on macOS (workstation, not Linux and not CI). The checker did not write the code. Every command was run again from this session's own checkout, with `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` and `--target-dir target/repo-check`. The developer's record above is kept as it was. Tools: `rustc 1.100.0`, `git`, `cargo-deny 0.20.2`.

Each probe changed one thing in a scratch copy of the committed tree (`git archive HEAD`, unpacked outside the repository, with its own `git init` and one commit so that `--base HEAD` works). The program was pointed at it with `--root`. The copy was put back with `git checkout` and `git clean` after every probe. The checkout of this session was never changed by a probe.

**Verdict: PASS for milestone 3.** It covers `0002 AC-6` to `0002 AC-10`. All eighteen rules reject a wrong change with exit 1, the rule ID and the text that [What each message must name](index.md#what-each-message-must-name) asks for, and all three commands pass on the committed tree. `scripts/prove-checks.sh` (step 7) and every run on GitHub are outside this milestone and were not run.

The committed tree and the program itself:

- [x] `scripts/check.sh all` → exit 0. `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` each end with `== ok: <name>`, in that order, and the summary repeats the six lines. `clippy` checks `pool-repo-check`, and `test` runs its 71 tests (71 passed, 0 failed). This also closes the part of `0002 AC-1` that milestone 2 left open (a green `guards`, `docs` and `all`) → `0002 AC-6`, `0002 AC-7`, `0002 AC-8`
- [x] `pool-repo-check workspace`, `pool-repo-check ci` and `pool-repo-check docs` → exit 0 for each, one line `pool-repo-check <command>: ok`. `docs --base main`, `docs --base origin/main` and `docs --base 4d03515` (the base before this milestone) → exit 0, `ok`. The one D4 problem in the developer's table is gone, because spec 0001 was fixed in `ab7e9ef` → `0002 AC-6`, `0002 AC-7`, `0002 AC-8`, `0002 AC-10`
- [x] `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check -- --list` → 71 tests. Tests named after their rule, each of which changes a good tree and asserts the rule ID: W1 4, W2 2, W3 2, W4 4, W5 3, W6 1, W7 5, C1 3, C2 1, C3 4, C4 1, C5 4, D1 5, D2 2, D3 6, D4 1, D5 3, D6 3. The passing cases are `good_tree_passes` (workspace, docs), `good_definition_passes` (ci) and `good_tree_passes_with_and_without_base` (ids), plus cases such as `w2_similar_name_is_not_forbidden` and `d6_new_ids_are_fine` → `0002 AC-6`, `0002 AC-7`
- [x] `tools/repo-check/Cargo.toml` compared with the block under [The repository check program](index.md#the-repository-check-program) → `diff` prints nothing. Its `Cargo.lock` holds the one package `pool-repo-check`. The six baseline files hold exactly the lines the spec lists, and each starts with a comment that names spec 0002. `cargo metadata --no-deps` at the root lists `pool` and `pool-config` only, and `ls tools/repo-check` shows no `target/` after the runs → `0002 AC-6`
- [x] Usage and status → `lint`, no command, `--help`, `workspace --base x`, `docs --root` (no value), `docs --base` (no value), `docs extra` and a second `--root` each exit 2 with a reason and the usage text on stderr. `--root /nonexistent` exits 3 for all three commands. `docs --base nosuchrev` exits 3 and names the `git` failure. A run from `/tmp` with no `--root` exits 3 and names each missing file, and the same run with `--root <checkout>` exits 0 → `0002 AC-6`
- [x] `grep` over `tools/repo-check/src` → no `std::net`, no file write or create call, no thread, no `unsafe`. The only process the program starts is `git` (`ls-tree` and `show`), and only for `--base`. This fits "reads files and, for `--base` only, runs `git`" → `0002 AC-6`
- [x] `scripts/check.sh` on the scratch copy → `guards` with `crates/pool-core/` created: exit 1, the W2 line, `== FAILED: guards`. `docs` with the `SEC-14` row removed and `POOL_CHECK_BASE` set to the base commit: the D5 and D6 lines, `== FAILED: docs`. `guards docs` with one dead link: exit 1, `guards` is `ok` and `docs` is `FAILED`. `guards` with `scripts/linux-build.sh` removed: exit 3. So the entry point passes the program's status on → `0002 AC-6`, `0002 AC-8`, `0002 AC-10`

Workspace guards, `0002 AC-6`. Each line is `pool-repo-check workspace` on the scratch copy:

- [x] W1 → a package `leftpad` added to `Cargo.lock`: exit 1, `W1: Cargo.lock: package leftpad is in the lockfile but not in the baseline lock-packages.txt`. The package `zmij` taken out: exit 1, `package zmij is in the baseline lock-packages.txt but not in the lockfile`. One extra package in `crates/pool-config/fuzz/Cargo.lock` and in `tools/repo-check/Cargo.lock`: exit 1, the message names that lockfile and its own baseline file. A second `serde` entry: exit 1 (names are compared as a sorted list). A name in the baseline that the lockfile lacks: exit 1. A `name =` line that is not inside a `[[package]]` entry: exit 0
- [x] W2 → an empty directory `crates/pool-core/`: exit 1, `W2: crates/pool-core: forbidden path crates/pool-core exists`. A plain file `apps/pool-cli`: exit 1. `crates/pool-protocol/` and `crates/pool-control/` together: exit 1, two lines. `crates/pool-core2/`: exit 0
- [x] W3 → `use std::thread;` added to `apps/pool/src/main.rs`: exit 1, `W3: apps/pool/src/main.rs:94: forbidden text std::thread`. `tokio` in a comment in `crates/pool-config/src/lib.rs`: exit 1 (a comment counts). `use std::net::TcpStream;` in a new test file: exit 1, one line for `std::net` and one for `TcpStream`. `UnixListener` in the fuzz target: exit 1. The same text under a `target/` directory, in a `.txt` file, and in `tools/repo-check/src/`: exit 0 for each
- [x] W4 → `[lints]` removed from `apps/pool/Cargo.toml`: exit 1, `W4: apps/pool/Cargo.toml: the manifest has no [lints] table with workspace = true`. `workspace = false`, an empty `[lints]` table, the table commented out, and a member that sets its own `[lints.rust]`: exit 1 for each. Root `unsafe_code = "deny"`, the root table removed, and the key moved under `[workspace.lints.clippy]`: exit 1, `W4: Cargo.toml: [workspace.lints.rust] does not set unsafe_code = "forbid"`. A member `apps/*`: exit 1, the wildcard is named. A new member with no `[lints]`: exit 1, its manifest is named. A `members` array written over several lines: exit 0
- [x] W5 → `pool = { path = … }` added to `crates/pool-config/Cargo.toml`: exit 1, `W5: crates/pool-config/Cargo.toml:10: dependency pool-config -> pool is not allowed by internal-dependencies.txt`. The same dependency under `[dev-dependencies]`, as a table `[build-dependencies.pool]`, under `[target.x86_64-unknown-linux-gnu.dependencies]` and as `pool.path` under `[target.'cfg(unix)'.dev-dependencies]`: exit 1 for each. `pool-config` removed from `apps/pool/Cargo.toml`: exit 1, `dependency pool -> pool-config is listed but missing from the manifests`. A pair added to the baseline that no manifest has: exit 1, missing
- [x] W6 → `x/Cargo.lock` and `apps/pool/Cargo.lock`: exit 1, `W6: x/Cargo.lock: unexpected Cargo.lock`. A `Cargo.lock` under `target/debug/`, under `a/target/` and under `.hidden/`: exit 0
- [x] W7 → `--no-run` added to one line below the marker: exit 1, `W7: scripts/linux-build.sh:25: scripts/linux-build.sh differs from the first code block under ### Linux build in docs/specs/0001-initial-architecture/index.md`. One line added at the end: exit 1, line 44. The last lines removed: exit 1. The marker line removed: exit 1, the marker is named. The block in spec 0001 changed and the script left alone: exit 1, line 25. The heading in spec 0001 renamed: exit 1, no block found. A comment added above the marker: exit 0

CI definition guards, `0002 AC-7`. Each line is `pool-repo-check ci` on the scratch copy:

- [x] C1 → `actions/checkout@v7`: exit 1, `C1: .github/workflows/ci.yml:26: uses actions/checkout@v7 is not <owner>/<name>@<40 hexadecimal characters> followed by a comment that starts with # v`. The full hash with no comment, 39 hexadecimal characters, a comment `# main`, `docker://alpine:3` and a local action path: exit 1 for each. A tag in a second file, `other.yml` and `other.yaml`: exit 1, that file is named
- [x] C2 → the job `docs` renamed to `documents`: exit 1, `C2: .github/workflows/ci.yml:86: job documents is not a required check in .github/rulesets/main.json` and `C2: .github/rulesets/main.json: required check docs is not a job id of .github/workflows/ci.yml`. The context `guards` removed from `main.json`: exit 1, job `guards` named. A context `extra` added: exit 1. A job `extra` added: exit 1
- [x] C3 → `run: cargo fmt --all --check` in the job `fmt`: exit 1, `C3: .github/workflows/ci.yml:30: job fmt runs cargo fmt --all --check, which is neither rustup toolchain install nor scripts/check.sh <name>`. The last `run:` changed to `scripts/check.sh clippy`: exit 1, `the last run of job fmt is scripts/check.sh clippy, not scripts/check.sh fmt`. `scripts/check.sh fmt && curl …`, a `run: |` block, `rustup toolchain install nightly`, the two `run:` lines swapped, and a job with no `run:` at all: exit 1 for each, with the job id
- [x] C4 → `runs-on: ubuntu-24.04`: exit 1, `C4: .github/workflows/ci.yml:23: runs-on is ubuntu-24.04, not ubuntu-24.04-arm`. A list, an expression `${{ matrix.os }}` and the value in quotes: exit 1 for each
- [x] C5 → `permissions:` with `contents: write` under one job: exit 1, `C5: .github/workflows/ci.yml: the workflow has 2 permissions: lines, not one`. `contents: write` at the top, a second permission under the block, `permissions: write-all`, and the block removed: exit 1 for each. `pull_request_target`: exit 1, line 4. `${{ secrets.GITHUB_TOKEN }}`: exit 1, line 19. `secrets.` in `other.yaml`: exit 1, that file is named
- [x] Files the rules must read → `main.json` removed, `ci.yml` removed, `.github/workflows/` removed: exit 3 for each, the missing file named on stderr

Document checks, `0002 AC-8`, `0002 AC-9`, `0002 AC-10`. Each line is `pool-repo-check docs` on the scratch copy, with `--base HEAD` for D6. Most probes use one new file, `docs/probe.md`:

- [x] D1 → `[x](missing-file.md)`: exit 1, `D1: docs/probe.md:3: link target missing-file.md does not exist`. A link with a title, an image, a reference definition `[label]: missing-ref.md`, a target in angle brackets with a space, and a target with `%20`: exit 1 for each, the target as written. `/docs/index.md`: exit 1, absolute path. `../../outside.md`: exit 1, leaves the repository. `https:`, `mailto:`, a directory and an existing file: exit 0
- [x] D1, cases that must not be read → a dead link inside a backtick fence, inside a tilde fence that holds a backtick fence line, and between backticks on one line: exit 0. A three backtick fence inside a four backtick fence: only the link after the outer fence is reported. A line ` ``` not a close ` does not close a fence
- [x] D2 → `[x](index.md#no-such-heading)`: exit 1, `D2: docs/probe.md:3: fragment no-such-heading is not a heading of docs/index.md`. `[y](#not-here)` in the same file: exit 1. Three headings `## Twice`: `#twice`, `#twice-1` and `#twice-2` pass, `#twice-3` and `#Twice` are reported. `## docs/prd` gives `#docsprd`, a link in a heading gives its text, and letters of another script are kept. A heading inside a fence gives no anchor. A fragment on a file that is not `.md`: exit 0. The two examples in the spec, `index.md#where-a-change-belongs` and `index.md#docsprd`: exit 0
- [x] D3 → the row of feature 2 changed to `done`: exit 1, `D3: docs/scope/002-standards-and-tooling.md:1: status disagrees: in-progress in the heading, done in docs/scope/index.md`. The heading of feature 3 changed to `in-progress`, with its tag kept: exit 1, status. A changed name: exit 1, `name disagrees`. A changed number: exit 1, `number disagrees`. The status `blocked` in a row and `started` in a heading: exit 1. A tag `urgent`: exit 1. A new file `018-new-feature.md` with no row, and a row removed while its file stays: exit 1, `the feature file has no row`. A row written twice: exit 1. A file renamed to `018-…` for feature 17: exit 1, number. A row whose file is gone: exit 1. A blank line above the heading: exit 1
- [x] D3, cases that must pass → the committed tree, where fourteen headings carry ` · needs a decision`: exit 0. A heading with two tags, ` · needs a decision · deferred`: exit 0
- [x] D4 → the `SEC-14` row written twice: exit 1, `D4: docs/security/security-requirements.md:21: SEC-14 is defined again; it is first defined at docs/security/security-requirements.md:20`. The same for `R-25`, `F-001` and `T-01`. `**AC-20**` of spec 0002 renamed to `**AC-19**`: exit 1, `0002 AC-19 is defined again`. `- **AC-3**` added at the end of spec 0001, outside its Requirements section: exit 1. `PRD-001 AC-1`, `PRD-001 FR-1` and `0001 SC-1` each written twice: exit 1 for each
- [x] D5 → one made up ID for each of the four table families (a security requirement, a risk, an inventory row and a threat, each with a number above the last one defined): exit 1, one line each, `D5: docs/probe.md:3: <ID> is cited but not defined`. A PRD 001 criterion between backticks, a PRD 001 functional requirement, a spec 0001 safety contract and a spec 0002 criterion, each one number past the last: exit 1 for each. A criterion of PRD 099 and of spec 0009: exit 1, `is cited, but PRD-099 does not exist` and `but spec 0009 does not exist`. A defined ID of each table family written with one more or one fewer leading zero: exit 1 (IDs are compared as text). An undefined ID in `README.md` and in `.github/pull_request_template.md`: exit 1. The made up IDs are not written here, because this file is checked by D5 too: the first draft of this record held them and `pool-repo-check docs` reported all 17, exit 1
- [x] D5, cases that must pass → every defined ID of each family: exit 0. `RUSTSEC-0000-0001`, `XSEC-99`, `PR-77`, `UTF-8`, a bare `AC-99`, `SEC-1x`, `10001 AC-99`: exit 0. An undefined ID inside a fence: exit 0. Files under `.hidden/`, `graphify-out/`, `node_modules/` and `target/`: not read
- [x] D6 → the `SEC-14` row removed, with `--base HEAD`: exit 1, `D6: docs/security/security-requirements.md: SEC-14 is defined at the base revision but no longer defined`, next to the D5 lines for its citations. The same for `F-002`, `R-24`, `T-02`, `0002 AC-20` and `PRD-001 AC-4`. `PRD-001 FR-5` and `0001 SC-10` removed: exit 1, the ID and `the 0001 SC list has 9 items, the base revision has 10`. `0001 SC-3` taken from the middle: exit 1. `SEC-14` given the next free number: exit 1. The text of a row changed and a new row with the next free number added: exit 0. Without `--base`, the removed row shows only as D5

**Readings the checker assessed**

- **Tests on trees held in memory.** The spec allows it by name ("or at trees held in memory"), so this is within the text. One effect to know: no test drives the reader of the real disk or the reader of a Git revision. Both were driven by the runs above (the skipped directories, `--root`, `--base HEAD`, a bad revision), and they behaved as specified. It is a note for `/test`.
- **A missing file is exit 3 and the other rules still run.** Observed for a lockfile, a baseline, the script, `ci.yml`, `main.json` and the scope index. The lines that say what is missing go to stderr, and the last line on stdout is `pool-repo-check <command>: could not run, <n> problems found so far`. The spec gives two forms for the last line and does not name one for status 3, so this third form adds to the text and does not go against it. Status 3 wins over 1, which fits "the check could not run".
- **C4 reads `ci.yml` only, and C5 allows one `permissions:` line.** Both follow the text. The table is introduced as reading `ci.yml` and `main.json`, and only C1 and C5 say "every file under `.github/workflows/`". A permission set for one job was refused, as the reading says. See the first fragile point below for what this leaves open.
- **A closing run of `#` stays in the anchor.** `## Closed heading ##` gives `closed-heading-`, where GitHub gives `closed-heading`. The spec lists the steps and this is not one of them. No heading in the repository is written that way. If one is, a correct link to it is reported, so the check fails in the safe direction.
- **Definitions are read from the whole file.** A `- **AC-3**` item added at the end of spec 0001 was reported by D4. The table says "a list item that starts with `**AC-<digits>`" in the file and does not limit it to one section, so the reading is the text. It is the stricter of the two readings and it found a real repeat in spec 0001.
- **FR and SC come from numbered items at the start of a line, up to the next heading of level 1 or 2.** Observed through D4, D5 and D6 on `PRD-001 FR` and `0001 SC`. The criteria of the PRDs are numbered items too (`1. **AC-1:**`) and are read as criteria, not as functional requirements, because they sit under another heading.

**Worked, but looked fragile** (notes for `/check review`; none of these goes against a criterion):

- A second workflow file is checked by C1 and C5 only. `other.yaml` with `runs-on: ubuntu-latest` and a `run:` of its own gave no C3 and no C4 line. A job in `ci.yml` with no `runs-on:` line also passes C4. The spec text reads this way, so a change would start in the spec.
- C5 looks for the text `secrets.`. `${{ secrets["X"] }}` passes. C3 looks at `run:` only, so a `shell:` key that carries a command passes.
- W5 reads keys. A dependency on a member under another key, `other = { package = "pool", … }`, passes. The spec says only keys count.
- W4 refuses `workspace = true # yes` (a comment after the value) and the two other valid TOML forms, `lints.workspace = true` and `lints = { workspace = true }`. All three fail in the safe direction.
- D1 reads a link inside a double backtick span, ` ``[x](y.md)`` `, as a link. It is a false report, not a missed one.
- The value of `--base` is handed to `git` before `--`. `--base --help` made `git` read it as an option (exit 3, nothing else happened). `scripts/check.sh` passes a commit hash or `origin/main`, so this needs a person to type it.
- For W7, when the block in spec 0001 is missing, the message names the spec file and not `scripts/linux-build.sh`. The message for a differing line names the script and the line, which is the case the table describes.
- Carried over from milestone 2: the skip marked `# provisional: spec 0002 step 5` is still in `scripts/check.sh`. With `tools/repo-check/Cargo.toml` removed from the scratch copy, `scripts/check.sh fmt` printed `fmt: tools/repo-check does not exist yet; its command was skipped` and `== ok: fmt`, exit 0. `guards` and `docs` exit 3 in that state, so `all` does not pass, but `fmt`, `clippy` and `test` alone do.

**Not run by the checker**

- `scripts/prove-checks.sh` and `0002 AC-11` (Build plan step 7, being built).
- Any run on a GitHub runner, and whether `fetch-depth: 0` gives the `docs` job the base revision there (Build plan step 11, `owner: lead`).
- An unreadable file (permission denied) and a file that is not UTF-8. Only missing files were probed for status 3.
- Symbolic links inside the tree.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0002 AC-6` | met: `workspace` passes on the committed tree, each of W1 to W7 rejects a wrong change with the text the spec names, and each has at least one test with a failing input |
| `0002 AC-7` | met: `ci` passes on the committed `ci.yml` and `main.json`, each of C1 to C5 rejects a wrong change and names the file and, for C2 and C3, the job id, and each has a failing test |
| `0002 AC-8` | met: D1 and D2 reject a missing file and a missing heading, and `docs` passes on the committed tree |
| `0002 AC-9` | met: D3 rejects a different number, name or status, a status outside the four, and a feature file with no row. A heading with a tag passes |
| `0002 AC-10` | met: D4 rejects an ID defined twice, D5 a cited ID with no definition, and D6, with `--base`, an ID that the base revision defines and the tree no longer does |
| All other criteria | outside this milestone, not run |

## Milestone 4: Proofs, inventory, documents and workstation evidence

Build plan steps 8 and 9, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `6cfef86` (step 9) and `7fb6052` (step 8: the `sbom` version check). Tools: `rustc 1.100.0`, `cargo-fuzz 0.13.2` on `rustc 1.101.0-nightly (5d8fec066 2026-10-16)` (toolchain `nightly-2026-10-17`), `cargo-cyclonedx 0.5.10` installed with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. Step 7 is commit `6c03124` (`scripts/prove-checks.sh`), run by its developer before the merge at `f6675e3`. Step 10 is recorded under [Step 10: workstation evidence on the final tree](#step-10-workstation-evidence-on-the-final-tree) below.

### UI / manual

None for steps 8 and 9. Item 12 of "Document corrections" waits for Build plan step 11 (`NOT_RUN`, `owner: lead`).

The four remote items of Build plan step 11 are not run by a build session. Each is run by the lead with the steps under [Remote proof](index.md#remote-proof), and the checker records the addresses here afterward.

| Item | Criterion | Remote steps | Result |
|---|---|---|---|
| The seven jobs pass on a real pull request, and the merge | `0002 AC-3` | Remote steps 1 and 6 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The CI log of `linux-build` on `ubuntu-24.04-arm` | `0002 AC-5`, CI half | Remote step 2 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| A pull request with the format error of case 1 is blocked | `0002 AC-12` | Remote step 5 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The branch rule applies to `main` and a direct push is refused | `0002 AC-13`, remote half | Remote steps 3 and 4 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |

### Commands

Run from the repository root.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh fuzz` (60 seconds) | exit 0, no file under `crates/pool-config/fuzz/artifacts/validate/` | exit 0, `Done 655837 runs in 61 second(s)`, `== ok: fuzz`; the directory exists and is empty (`find … -type f` lists nothing); 2616 corpus entries under the ignored `fuzz/corpus/validate`; `git status --short` empty | pass |
| `scripts/check.sh sbom` on `6cfef86`, before the fix | exit 0 | exit 3, `cargo-cyclonedx 0.5.10 is required`, although 0.5.10 was on `PATH`: `cargo cyclonedx --version` prints `cargo-cyclonedx-cyclonedx 0.5.10`, so the exact comparison never matched | fail, fixed in `7fb6052` |
| `scripts/check.sh sbom` on `7fb6052`, `target/sbom` removed first | exit 0, one file `target/sbom/pool.cdx.json`, clean tree | exit 0, `== ok: sbom`, the one file (20258 bytes), `git status --short` empty | pass |
| The command `check_sbom` runs, unchanged from the spec: `cargo cyclonedx --manifest-path apps/pool/Cargo.toml --format json --spec-version 1.5 --all --override-filename pool.cdx`, run alone | the files the tool writes | two untracked files, `apps/pool/pool.cdx.json` and `crates/pool-config/pool.cdx.json`; the script moves the first to `target/sbom/` and removes the second | pass (flags kept, `provisional` mark removed) |
| Content of `target/sbom/pool.cdx.json`, read with `python3 -m json` | CycloneDX 1.5 for `pool`, every third party package with a license | `bomFormat` `CycloneDX`, `specVersion` `1.5`, `metadata.component` `pool` 0.1.0; 17 components, each with a license expression (`pool-config` 0.1.0 `Apache-2.0`, and 16 third party packages); 18 `dependencies` entries | pass |
| Component names plus `metadata.component` compared with `lock-packages.txt` in the spec | the same 18 names, every difference explained | the same 18 names, no difference in either direction. Not a name difference but a placement: `pool` is the document's own subject (`metadata.component`), not an entry in `components`; `pool-config` is first party and appears as a component with `Apache-2.0` | pass |
| `grep -F` for each old sentence of items 1 to 3, 5 to 11 and 13 in the file the item names | none found | none found; the layout block has `deny.toml`, `scripts/`, `tools/repo-check/`, `.github/rulesets/` and `.github/pull_request_template.md`, and no `rustfmt.toml` or `clippy.toml` | pass |
| `grep -c "remote repository review/branch protection settings" CONTRIBUTING.md` (item 12 untouched) | 1 | 1 | pass |
| Every link written in step 9 (`../specs/0001-initial-architecture/verify.md`, `scope/index.md`, `implementation-order.md#handoff-for-later-slices`, `specs/0002-standards-and-tooling/index.md#workspace-guards`, `#adopted-in-scope-001`, `#adopted-in-scope-002`, `../engineering-standards.md#reviewer-roster`, `docs/specs/0002-standards-and-tooling/index.md`) | file and heading exist | all exist | pass (rule D1 itself is milestone 3) |
| `scripts/check.sh fmt clippy test deny` and `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, four `== ok:` lines and the summary; 134 tests passed, 0 failed; both `cargo deny` runs `advisories ok, bans ok, licenses ok, sources ok`; `configuration valid`, exit 0 | pass |
| `scripts/check.sh all` on `78c9dab` (this branch merged with `main`, milestone 3 included) | exit 0 | exit 0, summary `ok` for `fmt`, `clippy`, `test`, `deny`, `guards`, `docs`; 205 tests passed (134 workspace, 71 `pool-repo-check`); `pool-repo-check docs: ok`, so rule D1 accepts every link written in step 9 | pass (step 10 repeats it on the final tree) |

Not run in steps 8 and 9: `scripts/prove-checks.sh` (step 7) and the final tree runs of step 10.

### Step 10: workstation evidence on the final tree

Build plan step 10, on 2026-11-14, on macOS 27.0.1, `arm64` (workstation, not Linux and not CI). Before the runs, commit `f58885f` removed the provisional skip of spec 0002 step 5 from `scripts/check.sh`: the function `repo_check_package_exists` and its three calls are gone, so `fmt`, `clippy` and `test` always run their `tools/repo-check` command. Every run below is on that commit, `f58885f38e1c46ae369ccdf96fe4831a3c282c06` (`git rev-parse HEAD`), with a clean tree. The commit that adds this record changes only documents; `scripts/check.sh docs` was run again on it (last row).

Tools: `rustc 1.100.0 (0ef1bc386 2026-11-09)`; `cargo-deny 0.20.2`; `cargo-fuzz 0.13.2` on `rustc 1.101.0-nightly (5d8fec066 2026-10-16)` (toolchain `nightly-2026-10-17`); `cargo-cyclonedx 0.5.10` (it prints `cargo-cyclonedx-cyclonedx 0.5.10`), installed with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`; Docker server 29.9.2 (`docker info` exit 0); `git 2.54.0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `scripts/check.sh all` | exit 0, `== ok:` for `fmt`, `clippy`, `test`, `deny`, `guards`, `docs`, no line saying a command was skipped | exit 0; summary `ok` for all six; 205 tests passed, 0 failed (134 workspace, 71 `pool-repo-check`); both `cargo deny` runs `advisories ok, bans ok, licenses ok, sources ok`; `pool-repo-check workspace: ok`, `ci: ok`, `docs: ok`; no `was skipped` line | pass |
| `scripts/prove-checks.sh --with-linux-build` | 21 `PASS` lines, last line `all checks reject their bad change`, exit 0, `git status --short` empty | exit 0; `scripts/check.sh all` passed on the unchanged copy first; `PASS 1` to `PASS 21`, no `FAIL` and no `SKIPPED`; last line `all checks reject their bad change`; `git status --short` empty afterward | pass |
| The matched text of each case in that run | the text the case table names | 1 `Diff in`; 2 `ptr_arg`; 3 `print_stdout`; 4 `test result: FAILED`; 5 `license is not explicitly allowed`; 6 `RUSTSEC-0000-0001`; 7 `cannot update the lock file`; 8 `W1 … zmij`; 9 `W2: crates/pool-core`; 10 `W3 … std::thread`; 11 `W4`; 12 `W5 … pool-config -> pool`; 13 `C1 … actions/checkout@v7`; 14 `C2 … job documents`; 15 `D1 … docs/missing-file.md`; 16 `D2 … no-such-heading`; 17 `D3`; 18 `D5` for the made up ID of line K; 19 `D6 … SEC-14`; 20 `test result: FAILED` in the container; 21 `W7` | pass |
| `scripts/check.sh fuzz` (60 seconds) | exit 0, no file under `crates/pool-config/fuzz/artifacts/validate/` | exit 0, `Done 1774761 runs in 61 second(s)`, `== ok: fuzz`; `find crates/pool-config/fuzz/artifacts -type f` lists nothing; `git status --short` empty | pass |
| `scripts/check.sh sbom`, `target/sbom` removed first | exit 0, one file `target/sbom/pool.cdx.json`, clean tree | exit 0, `== ok: sbom`; one file, 20258 bytes; `git status --short` empty. The command it runs is unchanged from step 8: `cargo cyclonedx --manifest-path apps/pool/Cargo.toml --format json --spec-version 1.5 --all --override-filename pool.cdx` | pass |
| Content of `target/sbom/pool.cdx.json`, compared with `tools/repo-check/baseline/lock-packages.txt` | CycloneDX 1.5 for `pool`, a license for every component, the same names as the baseline | `bomFormat` `CycloneDX`, `specVersion` `1.5`, `metadata.component` `pool`; 17 components, none without a license; 18 `dependencies` entries; the 17 component names plus `pool` equal the 18 baseline names. The one difference in placement is the same as in step 8: `pool` is the document's subject, not a component | pass |
| `scripts/check.sh linux-build` | exit 0, `runner:`, `host: aarch64-unknown-linux-gnu`, last line `linux build check passed` | exit 0; `runner: Darwin arm64`, `host: aarch64-unknown-linux-gnu`; 136 tests passed in the container; every fixture with the wanted exit; `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool`; `pool 0.1.0`; `linux build check passed`, `== ok: linux-build` | pass (workstation reproduction, not the `0002 AC-5` evidence) |
| On a `git clone` of `f58885f` in a scratch directory, outside the worktree: delete `tools/repo-check/Cargo.toml`, then run `scripts/check.sh fmt`, `scripts/check.sh clippy` and `scripts/check.sh test` one at a time | each check fails with exit 1 (a command of the check exited with a status other than 0, which [the check entry point](index.md#the-check-entry-point) counts as a failed check), and none says it skipped the command | `fmt` exit 1: `` `cargo metadata` exited with an error: error: manifest path `tools/repo-check/Cargo.toml` does not exist ``, `== FAILED: fmt`. `clippy` exit 1 and `test` exit 1, each after the workspace command passed: `` error: manifest path `tools/repo-check/Cargo.toml` does not exist ``, `== FAILED: clippy`, `== FAILED: test`. No `skipped` line. The scratch copy was removed | pass |
| `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml` | exit 0 | exit 0, `configuration valid` (run inside `scripts/check.sh all` through the tests and by hand) | pass |
| `scripts/check.sh docs` with this record, the two `docs/index.md` rows and the observed versions in place | exit 0 | exit 0, `pool-repo-check docs: ok`, `== ok: docs`; every link and ID written here resolves | pass |

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-15` | The 60 second `scripts/check.sh fuzz` run above, with milestone 1's package half | pass (completes it); step 10 runs it again on the final tree |
| `0002 AC-16` | The `sbom` rows above: one file, CycloneDX 1.5, a license for each package, clean tree, names equal to `lock-packages.txt` | pass |
| `0002 AC-19` | Items 1 to 3, 5 to 11 and 13 applied in `6cfef86`; no old sentence left | pass for those items; item 4 is step 6 (milestone 3), item 12 waits for step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-14` | `CONTRIBUTING.md` names `scripts/check.sh` and the path to `main` through a pull request whose checks passed | pass (completes the milestone 2 file half) |
| `0002 AC-18` | `docs/plan/risk-register.md` links the [reviewer roster](../../engineering-standards.md#reviewer-roster) | link half pass; the roster rows are not part of steps 8 and 9 |
| `0002 AC-11` | Step 7 (`6c03124`), run again on the final tree: `scripts/prove-checks.sh --with-linux-build`, 21 `PASS`, `all checks reject their bad change`, clean tree | pass |
| `0002 AC-17` | The [technology evaluation](../../research/technology-evaluation.md#adopted-in-scope-002) names every adopted tool and now lists the versions observed on `f58885f`; the [testing strategy](../../testing/strategy.md#ci-progression) names the seven checks, the fuzz rule and how PostgreSQL 18 is supplied | pass |

Step 10 evidence on the final tree `f58885f`, by criterion:

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-1` | `scripts/check.sh all` exit 0, six `== ok:` lines; usage and could not run rules were recorded in milestone 2 and are unchanged; a missing `tools/repo-check` now fails `fmt`, `clippy` and `test` instead of being skipped | pass |
| `0002 AC-2` | `guards` passes, so rules C1 to C5 accept `ci.yml`; cases 13 and 14 rejected | pass (file); the run is `0002 AC-3` |
| `0002 AC-3` | Remote steps 1 and 6 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-4` | `deny` in `all` passes on both lockfiles with `cargo-deny 0.20.2`; cases 5, 6 and 7 rejected | pass |
| `0002 AC-5` | Script half: `linux-build` passes on the workstation, W7 passes, case 21 rejected, case 20 rejected in the container. The CI half is remote step 2 | script half pass; CI half `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-6` | `pool-repo-check workspace: ok`; 71 `pool-repo-check` tests pass; cases 8 to 12 and 21 rejected | pass |
| `0002 AC-7` | `pool-repo-check ci: ok`; cases 13 and 14 rejected | pass |
| `0002 AC-8` | `pool-repo-check docs: ok`, also after this record's links were added; cases 15 and 16 rejected | pass |
| `0002 AC-9` | `docs` passes; case 17 rejected | pass |
| `0002 AC-10` | `docs` passes; cases 18 and 19 rejected (case 19 with `--base`) | pass |
| `0002 AC-11` | 21 `PASS` on `f58885f` | pass |
| `0002 AC-12` | Remote step 5 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-13` | File half: `guards` passes on `main.json`. Remote half: remote steps 3 and 4 | file half pass; remote half `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-14` | `.github/pull_request_template.md` unchanged since milestone 2 and now listed in [docs/index.md](../../index.md#project-root); `CONTRIBUTING.md` names `scripts/check.sh` | pass |
| `0002 AC-15` | 60 second `scripts/check.sh fuzz` on `f58885f`, no crash file | pass |
| `0002 AC-16` | `scripts/check.sh sbom` on `f58885f`: one CycloneDX 1.5 file, a license for each component, names equal to the baseline, clean tree | pass |
| `0002 AC-17` | See the row above | pass |
| `0002 AC-18` | The [reviewer roster](../../engineering-standards.md#reviewer-roster) has 12 rows, one for each of the 12 owner roles in the risk register (each review the standards name is held by one of them); each row names its holder (`Not assigned` for all today) and says whether a fresh model may do the review; the risk register links to it | pass |
| `0002 AC-19` | Items 1 to 11 and 13 applied (milestones 3 and 4); `docs` passes | pass for those items; item 12 waits for step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-20` | `clippy` in `all` passes with the denied lints; cases 2 and 3 rejected | pass |

### Checker run: `/check verify`, milestone 4

Recorded by the checker session `chk-f2-verify` on 2026-11-14 at revision `535bd15c4923c53b184326912b733faf9362f720` (`main`, which holds Build plan steps 1 to 10), on macOS 27.0.1, `arm64` (workstation, not Linux and not CI). `git diff --stat f58885f 535bd15` lists only documents, so the code is the code of the developer's final tree. The checker did not write the code. Every command was run again from this session's own checkout, which was clean and had no `target/` directory before the first command. The developer's record above is kept as it was.

Tools: `rustc 1.100.0 (0ef1bc386 2026-11-09)`; `cargo-deny 0.20.2`; `cargo-fuzz 0.13.2` on `rustc 1.101.0-nightly (5d8fec066 2026-10-16)` (toolchain `nightly-2026-10-17`); `cargo-cyclonedx 0.5.10`, installed in this session with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`; Docker server 29.9.2 (`linux/arm64`, `docker info` exit 0); `git 2.54.0`.

Each probe that needed a changed tree was made in a `git clone` of this checkout in a scratch directory outside the repository, and the clone was removed afterward. The one probe inside the checkout (a planted crash file) used an ignored directory and was undone. `git status --short` was empty after every run.

**Verdict: PASS for milestone 4.** It covers `0002 AC-11`, `0002 AC-16`, `0002 AC-17`, `0002 AC-18` and `0002 AC-19` (all items except item 12, which waits for the remote evidence), and it completes `0002 AC-14` and `0002 AC-15`. The four remote items stay `NOT_RUN`, `owner: lead`.

The proof script, `0002 AC-11`:

- [x] `scripts/prove-checks.sh --with-linux-build` → exit 0. It first ran `scripts/check.sh all` on the unchanged copy, then printed `PASS 1` to `PASS 21`, with no `FAIL` and no `SKIPPED` line. The last line is `all checks reject their bad change`. Afterward `git status --short` is empty, `HEAD` is still `535bd15`, and the temporary copy under `$TMPDIR/pool-prove.*` is gone → `0002 AC-11`
- [x] The line the script printed under each `PASS` in that run → exit 1 for every case, and the matched text is the text the case table names: 1 `Diff in`; 2 `ptr_arg`; 3 `print_stdout`; 4 `test result: FAILED. 60 passed; 6 failed`; 5 `rejected: license is not explicitly allowed`; 6 `ID: RUSTSEC-0000-0001`; 7 `cannot update the lock file`; 8 `W1: Cargo.lock: package zmij is in the lockfile but not in the baseline lock-packages.txt`; 9 `W2: crates/pool-core: forbidden path crates/pool-core exists`; 10 `W3: apps/pool/src/main.rs:94: forbidden text std::thread`; 11 `W4: apps/pool/Cargo.toml`; 12 `W5 … dependency pool-config -> pool is not allowed`; 13 `C1 … uses actions/checkout@v7`; 14 `C2 … job documents is not a required check`; 15 `D1: README.md:46: link target docs/missing-file.md does not exist`; 16 `D2 … fragment no-such-heading`; 17 `D3 … status disagrees: done in the heading, planned in docs/scope/index.md`; 18 `D5: README.md:46`, the made up ID of line K; 19 `D6 … SEC-14 is defined at the base revision but no longer defined`; 20 `test result: FAILED. 62 passed; 6 failed`, in the container; 21 `W7: scripts/linux-build.sh:42` → `0002 AC-11`
- [x] The script read against [Proving the checks](index.md#proving-the-checks) → each of the 21 cases names the check, the file and the change that the table gives. Lines A to K are in the script as written in the spec (compared as text, all eleven found). The made up advisory of case 6 equals the spec block, byte for byte. The copy is a `git clone` with `git checkout --detach <commit>`, `POOL_CHECK_BASE` is that commit, `CARGO_TARGET_DIR` is `<root>/target/prove`, `<root>/target/tools/bin` is first on `PATH`, and case 20 commits with the throwaway identity → `0002 AC-11`
- [x] `DOCKER_HOST=unix:///nonexistent.sock scripts/prove-checks.sh --with-linux-build` (the daemon cannot be reached) → exit 0, 20 `PASS` lines, `SKIPPED 20 linux-build`, and the last line `all checks reject their bad change (linux-build not proven)`. So a skipped case stays visible → `0002 AC-11`
- [x] `scripts/prove-checks.sh --nope` → exit 2, `unknown argument: --nope` and the usage line on stderr. `scripts/prove-checks.sh` in a clone with one deleted tracked file → exit 3, `prove-checks: commit or remove local changes first; the proof uses only the committed tree` → `0002 AC-11`
- [x] Probe, does the script notice a check that no longer bites: in a clone, one commit removes the line `print_stdout = "deny"` from the root `Cargo.toml`. `scripts/prove-checks.sh` there → the unchanged copy still passes `all`, then `FAIL 3 clippy exit 1, but the output does not contain: print_stdout`, 19 `PASS` lines, `SKIPPED 20 linux-build`, the line `prove-checks: these cases were not rejected as expected: 3`, exit 1. So a case passes only when the check fails and prints the named text → `0002 AC-11`
- [x] Probe, a tree that is already red: in a clone, one commit appends line A to `apps/pool/src/main.rs`. `scripts/prove-checks.sh` there → the output of `scripts/check.sh all` (`fmt` and `clippy` `FAILED`), then `prove-checks: scripts/check.sh all failed on the unchanged copy (exit 1); a case could not show its own failure`, exit 1, and no case was run → `0002 AC-11`

The fuzz run, `0002 AC-15` (the path changed by `7c752a9`):

- [x] `scripts/check.sh fuzz` → exit 0. The command line it started ends with `-max_total_time=60 -max_len=4096 fuzz/corpus/validate tests/fixtures`. `Done 1690079 runs in 61 second(s)`, `== ok: fuzz`. `find crates/pool-config/fuzz/artifacts -type f` lists nothing (the directory `artifacts/validate/` exists and is empty). `git status --short` is empty, `git status --short --ignored crates/pool-config/fuzz` shows only `corpus/`, and the committed fuzz lockfile is unchanged → `0002 AC-15`
- [x] Crash file probe: one file `crash-probe` put into `crates/pool-config/fuzz/artifacts/validate/`, then `POOL_FUZZ_SECONDS=2 scripts/check.sh fuzz` → `Done 59063 runs in 3 second(s)` (so `POOL_FUZZ_SECONDS` replaces the 60), `fuzz: a crash file exists under crates/pool-config/fuzz/artifacts/validate/; report it with its bytes`, `== FAILED: fuzz`, exit 1. The file was removed. Together with the green run above, where the directory is there and empty, this shows that only a file counts as a crash → `0002 AC-15`

The inventory, `0002 AC-16` (the path changed by `7fb6052`):

- [x] `cargo cyclonedx --version` with `target/tools/bin` on `PATH` → `cargo-cyclonedx-cyclonedx 0.5.10`. That is the text the script compares with
- [x] `scripts/check.sh sbom`, `target/sbom` removed first → exit 0, `== ok: sbom`. One file, `target/sbom/pool.cdx.json`, 20264 bytes. `git status --short` is empty and no `*.cdx.json` file is left outside `target/` → `0002 AC-16`
- [x] The file read with a JSON parser → `bomFormat` `CycloneDX`, `specVersion` `1.5`, `metadata.component` is `pool` 0.1.0 (type `application`, license `Apache-2.0`). 17 components and 18 `dependencies` entries. Every component has a license expression: `pool-config` `Apache-2.0`, and the 16 third party packages (12 `MIT OR Apache-2.0`, `memchr` `Unlicense OR MIT`, `unicode-ident` `(MIT OR Apache-2.0) AND Unicode-3.0`, `winnow` and `zmij` `MIT`). Every component version equals the version in `Cargo.lock` → `0002 AC-16`
- [x] The 17 component names plus `pool`, sorted, compared with `tools/repo-check/baseline/lock-packages.txt` → the same 18 names, nothing on either side only. The one difference is a placement and it is explained: `pool` is the subject of the document (`metadata.component`), not an entry of `components` → `0002 AC-16`
- [x] The command inside `check_sbom`, run alone in a clone: `cargo cyclonedx --manifest-path apps/pool/Cargo.toml --format json --spec-version 1.5 --all --override-filename pool.cdx` → exit 0, two untracked files: `apps/pool/pool.cdx.json` (subject `pool`, 17 components) and `crates/pool-config/pool.cdx.json` (subject `pool-config`, 12 components). The script moves the first by its path and removes the second, so the file it keeps is the one for `pool` → `0002 AC-16`
- [x] `scripts/check.sh sbom` in a clone with no `cargo-cyclonedx`, with a stub that prints `cargo-cyclonedx-cyclonedx 0.5.9`, and with a stub that prints `cargo-cyclonedx 0.5.10` → exit 3 for each, one line: `cargo-cyclonedx 0.5.10 is required; run cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. The tree stays clean → `0002 AC-1`, `0002 AC-16`

The entry point after `f58885f`, `0002 AC-1`:

- [x] In a clone, `tools/repo-check/Cargo.toml` deleted, then `scripts/check.sh fmt`, `scripts/check.sh clippy` and `scripts/check.sh test`, one at a time → exit 1 for each, `== FAILED: <name>`. `fmt` prints `` `cargo metadata` exited with an error: error: manifest path `tools/repo-check/Cargo.toml` does not exist ``. `clippy` and `test` first pass their workspace command (134 tests passed in `test`) and then print the same `manifest path` error. No line says a command was skipped. `scripts/check.sh guards docs` in that state → exit 3. `scripts/check.sh all` → exit 1, only `deny` is `ok`. This closes the fragile point carried from milestones 2 and 3 → `0002 AC-1`

The documents, `0002 AC-19` and `0002 AC-14`. Each item was read in the file it names, and the old and new texts were compared as whole sentences with the spec:

- [x] Items 1, 2, 3, 5, 7, 8, 9, 10 and 11 of [Document corrections](index.md#document-corrections) → in each named file the old sentence is not there and the new text is there, word for word (`docs/testing/strategy.md` line 3, `docs/implementation-order.md` first paragraph, `docs/index.md` line 5, `docs/repository-structure.md` line 3, its "Merge or defer the candidate crates" paragraph and its last paragraph, `docs/research/technology-evaluation.md` "Adoption gate", `docs/plan/risk-register.md` first paragraph, `CONTRIBUTING.md`) → `0002 AC-19`
- [x] Item 4 (step 6) → `docs/index.md` line 20 cites `0001 AC-1` in the example and the old example is not there → `0002 AC-19`
- [x] Item 6 → the layout block of `docs/repository-structure.md` has no `rustfmt.toml` and no `clippy.toml` line. `deny.toml` follows `rust-toolchain.toml`. `scripts/` and `tools/` with `repo-check/` under it follow the `crates/` group. `.github/` has `rulesets/` and `pull_request_template.md` → `0002 AC-19`
- [x] Item 11 → the command block of `CONTRIBUTING.md` holds the one line `scripts/check.sh all` (`git diff 6de6970` shows the three `cargo` lines removed). The paragraph after it says that every pull request runs seven required checks, that `scripts/check.sh X` reproduces a failed check, and that a change reaches `main` only through a pull request whose checks passed → `0002 AC-19`, `0002 AC-14`
- [x] Item 12 → left as it is, as the spec asks: the last paragraph of `CONTRIBUTING.md` still names "remote repository review/branch protection settings", the maintainer team and the private reporting channel. It waits for Build plan step 11 → `0002 AC-19` (`NOT_RUN`, `owner: lead`)
- [x] Item 13 → `README.md`, "Build and check": the block still has its four commands, and the new sentence about `scripts/check.sh all` and `scripts/check.sh linux-build` is there (see the readings below for where it sits) → `0002 AC-19`
- [x] `scripts/check.sh docs` (inside `all`) → `pool-repo-check docs: ok`, so every link and ID in the changed documents resolves → `0002 AC-19`

The roster and the recorded choices, `0002 AC-18` and `0002 AC-17`:

- [x] The [reviewer roster](../../engineering-standards.md#reviewer-roster) against the Owner column of `docs/plan/risk-register.md` → the register names 12 owner roles over `R-01` to `R-25`, and the roster has 12 rows, one for each. Each of the 25 risk IDs appears in exactly one row. The four reviews the standards name (security, protocol, database and infrastructure review) each stand in the row of the role that does them → `0002 AC-18`
- [x] The roster, column by column → every row has a Holder cell, and all 12 say `Not assigned`. Every row has an answer in "May a fresh model do the review?". Those answers agree with the two columns of the table under [Reviewer roster and fresh model reviews](index.md#reviewer-roster-and-fresh-model-reviews): a slice review may be done by a fresh model, and promotion, fencing, recovery and HA designs, drill and lab results, licenses, release sign off, `ignore` entries, security reports, accepting a risk and the branch rule need a person. The text above the table gives the stand in rule for Project lead, Maintainer and Release maintainer. The first paragraph of the risk register links to the roster → `0002 AC-18`
- [x] The section [Adopted in scope 002](../../research/technology-evaluation.md#adopted-in-scope-002), column by column → its table has the columns Version, Purpose, License, Native code, Cost at run time and Alternative, with no empty cell. Its nine rows are the tools the spec adopts: the hosted runner `ubuntu-24.04-arm`, `actions/checkout`, `actions/cache`, cargo-deny 0.20.2, cargo-fuzz 0.13.2, libfuzzer-sys 0.4.13, the toolchain `nightly-2026-10-17`, cargo-cyclonedx 0.5.10 and the PostgreSQL image (rule only, not adopted yet) → `0002 AC-17`
- [x] The table of observed versions in that section against this session → `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, `rustc 1.101.0-nightly (5d8fec066 2026-10-16)`, `cargo-cyclonedx-cyclonedx 0.5.10` and `rustc 1.100.0 (0ef1bc386 2026-11-09)` are what the same commands print here. The ten fuzz only packages have the listed versions in `crates/pool-config/fuzz/Cargo.lock`. The two actions and the runner are marked as not observed on a workstation → `0002 AC-17`
- [x] [CI progression](../../testing/strategy.md#ci-progression) in the testing strategy → it names the seven checks `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards` and `docs`, the fuzz rule (cargo-fuzz, a `fuzz/` directory in the crate under test, one target today run by hand, a CI job with the first slice that parses bytes from a socket), and how PostgreSQL 18 is supplied (the official container image, pinned by digest, started by `scripts/check.sh integration`) → `0002 AC-17`

**Readings the checker assessed**

- **A missing `tools/repo-check` package gives exit 1 for `fmt`, `clippy` and `test`.** The status rule says that exit 3 from `pool-repo-check` or from `scripts/linux-build.sh` is a could not run result, and that any other status that is not 0, from any command of a check, is a failed check. `cargo` ends with an error status on a missing manifest, and that status is not 3, so 1 is what the text gives. It also fits the sense of the rule: the package is part of the committed tree, not a tool of the machine, so a tree without it is a wrong tree. `guards` and `docs` still exit 3 for the same missing file, because they test for the manifest first. Both results are other than 0, so no run can pass by accident.
- **How `scripts/prove-checks.sh` works.** The clone with a detached checkout is the copy the spec describes. The edits are made with `awk` and plain text matching, which the spec leaves open. The extra line under each `PASS` adds evidence and takes nothing away from the `PASS <case> <check> rejected the change` line. A red baseline gives exit 1 and runs no case, as "requires exit 0" asks. With `--with-linux-build` and no daemon, the script prints `SKIPPED 20 linux-build` and the "not proven" ending. The spec says case 20 runs only with the flag and a running daemon, so this is the text.
- **The version line of `cargo-cyclonedx`.** The tool prints `cargo-cyclonedx-cyclonedx 0.5.10`. The spec says `sbom` checks its tool the same way as `deny`, which compares the whole line. Comparing with the line that version 0.5.10 really prints keeps that rule. A build that prints anything else is refused with exit 3.
- **Two files written, one kept.** The spec says the tool may write one file for each workspace member and that the script keeps the one for `pool`. The script takes `apps/pool/pool.cdx.json` by path, and that file has `pool` as its subject. The flags are the ones the spec starts from.
- **`pool` as `metadata.component`.** `0002 AC-16` asks for the third party packages `pool` is built from, with a license for each, and for every difference from `lock-packages.txt` to be explained. All 16 third party packages and `pool-config` are components with a license, and `pool` is the subject. The names are the same 18, and the placement is explained in the record. The checker reads this as met.
- **Twelve roster rows.** `0002 AC-18` asks for one row for each review named in the standards and for each owner role in the risk register. The standards name four reviews, and each is done by a role that the register also names, so the 12 rows cover both lists. The spec assigns no holder by name. It only says the repository owner decides while three roles are open, and the roster text says that too. All rows saying `Not assigned` is a visible gap, which the criterion allows.
- **Action versions and the runner.** The technology evaluation gives the version and commit hash of both actions and the runner label in its first table, and says in the observed table that they run only in CI and belong to Build plan step 11. A workstation cannot observe them, so this is an honest entry and `0002 AC-17` is met.
- **Where item 13 sits in `README.md`.** The spec says to keep the block and "add after it". The new sentence is its own paragraph after the paragraph that explains the last command of the block. It comes after the block, the explanation of the block stays next to it, and the old text is unchanged. The checker reads this as what the item asks.

**Worked, but looked fragile** (notes for `/check review`; none of these goes against a criterion):

- The technology evaluation still says, above its first table, that the observed versions "are added below when the build is done". They are there now. The sentence is out of date in wording only.
- If `cargo cyclonedx` fails after it wrote one of its two files, `check_sbom` returns before it cleans up, and an untracked `pool.cdx.json` can stay beside a manifest. `git status --short` would show it.
- In the lint probe, case 3 still made `clippy` exit 1 without the denied lint, for another reason. The script reported it only because it also asks for the named text. That is the right design, and it shows why both conditions are needed.
- Cases 6 and 7 and the first `deny` run need the network, as the spec says. Without it the proof stops at the baseline.

**Not run by the checker**

- Anything under [Remote proof](index.md#remote-proof): the four items in the table above stay `NOT_RUN`, `owner: lead`.
- A real `cargo install` inside `scripts/check.sh tools` (the pinned `cargo-deny` is already on this machine; `tools` printed `cargo-deny 0.20.2 is already available`, exit 0).
- A run without the network.

**Criteria in this milestone**

| Criterion | Result |
|---|---|
| `0002 AC-11` | met: 21 cases, each rejected with a status other than 0 and the named text, the ending line, a clean tree afterward. The script itself fails when a check stops rejecting its case |
| `0002 AC-14` | completed: the template equals the spec (milestone 2), and `CONTRIBUTING.md` describes the path to `main` and names `scripts/check.sh` |
| `0002 AC-15` | completed: the 60 second run through `scripts/check.sh fuzz` ends with exit 0 and no crash file, and a planted file is reported |
| `0002 AC-16` | met: one CycloneDX 1.5 file for `pool`, a license for each package, a clean tree, and the same 18 names as `lock-packages.txt` with the one placement explained |
| `0002 AC-17` | met: every adopted tool with the six columns, the observed versions, the seven checks, the fuzz rule and the PostgreSQL 18 supply |
| `0002 AC-18` | met: 12 rows for 12 owner roles and the four named reviews, a holder cell and a fresh model answer in every row, and the link from the risk register |
| `0002 AC-19` | met for items 1 to 11 and 13. Item 12 waits for Build plan step 11: `NOT_RUN`, `owner: lead` |
| All other criteria | recorded under the full verify below |

## Full verify: `/check verify standards and tooling`

Recorded by the checker session `chk-f2-verify` on 2026-11-14. The checker did not write the code. This section rolls the whole of spec 0002 into one verdict, criterion by criterion. It uses the runs of this session on the final tree, listed below and under [Checker run: `/check verify`, milestone 4](#checker-run-check-verify-milestone-4), and the checker records of milestones 1, 2 and 3 above for the paths that have not changed since.

**Verdict**: `PASS` for everything that can run on the workstation. Fifteen criteria are met in full. `0002 AC-5` and `0002 AC-13` are met for their workstation half, and `0002 AC-19` for every item except item 12. `0002 AC-3`, `0002 AC-12`, the CI half of `0002 AC-5`, the remote half of `0002 AC-13` and item 12 need the remote steps of Build plan step 11: `NOT_RUN`, `owner: lead`. Nothing failed and nothing is blocked by hardware. The spec is fully met only when step 11 is recorded here.

**Revision**: `535bd15c4923c53b184326912b733faf9362f720` (`main`). `git diff --stat f58885f 535bd15` lists only documents (`docs/index.md`, the technology evaluation, the scope files and this record).

**What changed since the milestone records** (`git log --oneline ffa14f2..535bd15 -- tools scripts .github deny.toml crates apps Cargo.toml Cargo.lock`): two commits, `6c03124` (`scripts/prove-checks.sh`) and `f58885f` (the skip removed from `scripts/check.sh`). `git log fb4f93d..535bd15` shows no commit that touches `deny.toml`, `.github/`, `crates/pool-config/fuzz/`, `scripts/linux-build.sh` or the root `Cargo.toml`. `tools/repo-check/` has no commit after the milestone 3 record.

**Checkout**: `git status --short` was empty and there was no `target/` directory before the first command.

**Workstation**: macOS 27.0.1, `arm64`, with the tools listed in the milestone 4 record. A run here is not Linux evidence and not CI evidence.

### UI / manual

No screen. The four remote items are run by the lead with the steps under [Remote proof](index.md#remote-proof). They are not failures and they are not blocked by missing hardware.

| Item | Criterion | Remote steps | Result |
|---|---|---|---|
| The seven jobs pass on a real pull request, and the merge | `0002 AC-3` | Remote steps 1 and 6 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The CI log of `linux-build` on `ubuntu-24.04-arm` | `0002 AC-5`, CI half | Remote step 2 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| A pull request with the format error of case 1 is blocked | `0002 AC-12` | Remote step 5 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The branch rule applies to `main` and a direct push is refused | `0002 AC-13`, remote half | Remote steps 3 and 4 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| Item 12 of "Document corrections" | `0002 AC-19`, item 12 | After Build plan step 11 has passed | `NOT_RUN`, `owner: lead` |

### Commands

Run from the repository root at `535bd15`.

- [x] `scripts/check.sh all` → exit 0. `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` each end with `== ok: <name>`, in that order, and the summary repeats the six lines. No line says a command was skipped. 205 tests passed, 0 failed (3 + 66 + 3 + 60 + 2 in the workspace, 71 in `pool-repo-check`). Both `cargo deny` runs print `advisories ok, bans ok, licenses ok, sources ok`, with the one expected warning `license-exception-not-encountered` on the product lockfile. `pool-repo-check workspace: ok`, `pool-repo-check ci: ok`, `pool-repo-check docs: ok`. `git status --short` is empty afterward and `tools/repo-check/target` does not exist → `0002 AC-1`, `0002 AC-4`, `0002 AC-6`, `0002 AC-7`, `0002 AC-8`, `0002 AC-9`, `0002 AC-10`, `0002 AC-20`
- [x] `scripts/check.sh` with no name, called from `/tmp` → exit 2, stdout empty, the usage text on stderr. `scripts/check.sh fmt bogus` → exit 2, stdout empty, `scripts/check.sh: unknown check: bogus` and the usage text → `0002 AC-1`
- [x] `PATH=/usr/bin:/bin scripts/check.sh fmt` → exit 3, one line: `fmt: cargo is required and was not found on PATH`. `DOCKER_HOST=unix:///nonexistent.sock scripts/check.sh linux-build` → exit 3, `runner: Darwin arm64`, `linux-build: the Docker daemon is not running`. `scripts/check.sh linux-build` in a clone with one deleted tracked file → exit 3, `linux-build: commit or remove local changes first; this check builds only the committed tree` → `0002 AC-1`
- [x] `scripts/check.sh tools` → exit 0, `cargo-deny 0.20.2 is already available` → `0002 AC-4`
- [x] `git ls-files -s scripts` → mode `100755` for `scripts/check.sh`, `scripts/linux-build.sh` and `scripts/prove-checks.sh` → `0002 AC-1`, `0002 AC-11`
- [x] Files compared with their blocks in the spec as it stands at this revision → no difference for `deny.toml`, `.github/workflows/ci.yml`, `.github/rulesets/main.json`, `.github/pull_request_template.md`, `crates/pool-config/fuzz/Cargo.toml` and `crates/pool-config/fuzz/fuzz_targets/validate.rs` (the spec block now has the committed layout, so the reading from milestone 1 is closed). `crates/pool-config/fuzz/.gitignore` is the four lines `target`, `corpus`, `artifacts`, `coverage` → `0002 AC-2`, `0002 AC-4`, `0002 AC-13`, `0002 AC-14`, `0002 AC-15`
- [x] `scripts/linux-build.sh` → its text up to the marker line equals the block under [Linux build script](index.md#linux-build-script), the marker occurs once, and the text after it equals the first code block under `### Linux build` in spec 0001 (1100 bytes, 28 lines) → `0002 AC-5`
- [x] `scripts/check.sh linux-build` → exit 0. `runner: Darwin arm64`, `host: aarch64-unknown-linux-gnu`, `release: 1.100.0`, 136 tests passed in the container (3 + 68 + 3 + 60 + 2), 30 fixture lines (6 with `exit=0`, 24 with `exit=1`), `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool` (the same hash as in the earlier runs), `pool 0.1.0`, `linux build check passed`, `== ok: linux-build` → `0002 AC-5` (a workstation reproduction; the evidence the criterion asks for is the CI log of step 11)
- [x] `git diff 6de6970 535bd15 -- Cargo.toml` → the only change is the table `[workspace.lints.clippy]` with the five lints. `git diff --quiet 6de6970 -- Cargo.lock` → exit 0, the root lockfile is unchanged and holds 18 packages. The names of both lockfiles equal their baseline files. `cargo metadata --locked --no-deps` lists `pool` and `pool-config` only → `0002 AC-20`, `0002 AC-6`
- [x] The paths `apps/pool-cli`, `crates/pool-protocol`, `crates/pool-core`, `crates/pool-control`, `rustfmt.toml` and `clippy.toml` → none exists. All 21 paths of the list "New paths, and no others" are tracked, and the files added in `6de6970..535bd15` outside `docs/` are those paths and the modules under `tools/repo-check/src/` → the surfaces of the spec
- [x] `scripts/prove-checks.sh --with-linux-build`, `scripts/check.sh fuzz`, `scripts/check.sh sbom` and their probes → see the milestone 4 record above → `0002 AC-11`, `0002 AC-15`, `0002 AC-16`
- [x] The document, roster and recorded choice checks → see the milestone 4 record above → `0002 AC-14`, `0002 AC-17`, `0002 AC-18`, `0002 AC-19`
- [x] `scripts/check.sh docs` with this record in place → exit 0, `pool-repo-check docs: ok`, so every link and ID written here resolves → `0002 AC-8`

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-1` | `all` exits 0 with six `== ok:` lines in order. Usage errors exit 2 with the usage text on stderr. A missing tool, a wrong version, a stopped daemon and a tree with changes exit 3 with one line. `all` keeps going after a failure, and 1 wins over 3 (milestone 2 probes, and the missing manifest probe of this run) | met |
| `0002 AC-2` | `ci.yml` equals the spec block; seven jobs, one runner label, `tools` before `deny`, three triggers, read access only, no secret, actions by full commit hash (milestone 2); `pool-repo-check ci: ok`; cases 13 and 14 rejected | met (file) |
| `0002 AC-3` | Remote steps 1 and 6 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-4` | `deny.toml` equals the spec block; `scripts/check.sh deny` requires `cargo-deny 0.20.2` and passes its four checks on both lockfiles; three licenses and the one `NCSA` exception; `ignore = []`; cases 5, 6 and 7 rejected | met |
| `0002 AC-5` | The script is the spec 0001 command, byte for byte; `pool-repo-check workspace` (rule W7) passes; case 21 rejected; the local run passes on `aarch64-unknown-linux-gnu`; case 20 rejected in the container. The CI log is remote step 2 | workstation half met; CI half `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-6` | `pool-repo-check workspace: ok`; a failing test for each of W1 to W7 (milestone 3, 71 tests pass today); cases 8 to 12 and 21 rejected | met |
| `0002 AC-7` | `pool-repo-check ci: ok`; a failing test for each of C1 to C5 (milestone 3); cases 13 and 14 rejected | met |
| `0002 AC-8` | `pool-repo-check docs: ok` on the committed tree; cases 15 and 16 rejected | met |
| `0002 AC-9` | D3 probes of milestone 3; case 17 rejected | met |
| `0002 AC-10` | D4, D5 and D6 probes of milestone 3; cases 18 and 19 rejected, case 19 with the base revision | met |
| `0002 AC-11` | 21 `PASS`, `all checks reject their bad change`, exit 0, clean tree; the script fails when a check stops rejecting its case | met |
| `0002 AC-12` | Remote step 5 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-13` | `main.json` equals the spec block; rule C2 keeps its seven contexts equal to the job ids. The remote half is remote steps 3 and 4 | file half met; remote half `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-14` | The template equals the spec block; `CONTRIBUTING.md` describes the path to `main` and names `scripts/check.sh` | met |
| `0002 AC-15` | The four package files equal the spec; `scripts/check.sh fuzz` runs `validate` for 60 seconds with `cargo-fuzz 0.13.2` on `nightly-2026-10-17`, exit 0, no crash file; the run is recorded here | met |
| `0002 AC-16` | `scripts/check.sh sbom` writes the one CycloneDX 1.5 file with a license for each package and leaves the tree clean; names compared with `lock-packages.txt`, the one placement explained | met |
| `0002 AC-17` | The technology evaluation and the testing strategy, read column by column | met |
| `0002 AC-18` | The roster, read against the risk register and the standards | met |
| `0002 AC-19` | Items 1 to 11 and 13, each read in its file. Item 12 waits for the remote evidence | met for items 1 to 11 and 13; item 12 `NOT_RUN`, `owner: lead` |
| `0002 AC-20` | The Clippy table in the root `Cargo.toml`; `scripts/check.sh clippy` passes; cases 2 and 3 rejected | met |

Surfaces: every path the spec lists is built and tracked. No surface is specced and missing. The four paths that spec 0001 forbids are absent.

### Readings the checker assessed

The readings for this run are under the milestone 4 record above. The readings of milestones 1, 2 and 3 stand, with two of them now closed: the spec block of `validate.rs` has the committed layout, and the provisional skips in `scripts/check.sh` are gone.

### For `/check review`

The notes under "Worked, but looked fragile" in the records of milestones 2, 3 and 4 are still open, except the provisional skip. The ones most worth a second look: a second workflow file is checked by rules C1 and C5 only, `CDPATH` can send both scripts to another directory, and `check_sbom` does not clean up when the tool fails halfway.

### Not run

- Build plan step 11 and everything under [Remote proof](index.md#remote-proof).
- Any run on Linux outside the container, and any run on a GitHub runner.
- The probes that milestones 1 to 3 already recorded were not repeated where the code on their path has not changed since.

## Verify after the review fixes

Recorded by the checker session `chk-f2-verify` on 2026-11-14, after the fix round that answered the [independent review](../../reviews/2026-11-14-scope-002-standards-and-tooling.md). The checker did not write the code. This section repeats the steps whose path changed in `535bd15..a3258e4`, and it carries the record to the tree that remote step 1 pushes (review finding m3). The records above are kept as they were, with one correction: the remote step numbers in their tables now follow the new order of [Remote proof](index.md#remote-proof) (see [Remote step numbers](#remote-step-numbers) below).

**Verdict**: `PASS` for every step that was repeated. Nothing failed and nothing is blocked. The four remote items and item 12 stay `NOT_RUN`, `owner: lead`.

**Revision**: `a3258e42bd77067bedd6114ee09726ee995f02be` (`git rev-parse HEAD`, equal to `main`). Every run below is on that revision. `git status --short` was empty and there was no `target/` directory before the first command. The commit that adds this section changes only this file, and `scripts/check.sh docs` was run again with it in place (last step).

**Test counts on that revision**: 332 tests passed, 0 failed. 134 in the workspace (3 + 66 + 3 + 60 + 2) and 198 in `pool-repo-check` (90 unit tests and 108 integration tests). In the Linux container 136 tests passed (3 + 68 + 3 + 60 + 2).

**Workstation**: macOS 27.0.1, `arm64`. Not Linux and not CI. Tools: `rustc 1.100.0 (0ef1bc386 2026-11-09)`; `cargo-deny 0.20.2`; `cargo-cyclonedx 0.5.10` (it prints `cargo-cyclonedx-cyclonedx 0.5.10`), installed in this session with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`; Docker server 29.9.2 (`linux/arm64`, `docker info` exit 0); `git 2.54.0`.

**What changed since the full verify** (`git diff --stat 535bd15..a3258e4`, outside `docs/`): `deny.toml` (one line), `scripts/check.sh` (the `cd` line and `check_sbom`), `scripts/linux-build.sh` (the part before the marker), `scripts/prove-checks.sh` (cases 22 to 28, `CDPATH`, `GIT_DIR`, commit hooks), five modules under `tools/repo-check/src/` and the new integration tests under `tools/repo-check/tests/`. Nothing under `.github/`, `apps/` or `crates/` changed, and neither did the root `Cargo.toml` or any of the three lockfiles (`git diff --quiet 535bd15 -- Cargo.lock crates/pool-config/fuzz/Cargo.lock tools/repo-check/Cargo.lock` exits 0). In the spec, "Remote proof" was written again with a new order, rules C3, C4, C5, W4 and W5 got new sentences, the `deny.toml` block gained `unsound = "all"`, the first part of the Linux build script changed, and proof cases 22 to 28 were added.

Each probe that needed a changed tree was made in a `git clone` of this checkout in a scratch directory outside the repository, and the clone was removed afterward. The guard probes ran the binary that `scripts/check.sh all` had just built, with `--root` set to the clone. `git status --short` in this checkout was empty after every run.

### Remote step numbers

The fix round put the remote steps in a new order. The tables of milestone 4 and of the full verify above named the old numbers. Only those numbers were changed there, nothing else.

| Step | Old number | New number |
|---|---|---|
| Push the scope branch, open the pull request, seven checks pass | 2 | 1 |
| Read the Linux build lines from that run | 3 | 2 |
| Apply the branch rule | 1 | 3 |
| A direct push is refused | 4 | 4 |
| A wrong change is blocked | 5 | 5 |
| Merge | 6 | 6 |
| The closing pull request | none | 7 |
| The first daily run | 7 | 8 |

So `0002 AC-3` is now remote steps 1 and 6, the CI half of `0002 AC-5` is remote step 2, `0002 AC-12` stays remote step 5, and the remote half of `0002 AC-13` is remote steps 3 and 4.

### UI / manual

No screen. The remote items are run by the lead with the steps under [Remote proof](index.md#remote-proof). The checker ran none of them.

| Item | Criterion | Remote steps | Result |
|---|---|---|---|
| The seven jobs pass on a real pull request, and the merge | `0002 AC-3` | Remote steps 1 and 6 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The CI log of `linux-build` on `ubuntu-24.04-arm` | `0002 AC-5`, CI half | Remote step 2 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| A pull request with the format error of case 1 is blocked | `0002 AC-12` | Remote step 5 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| The branch rule applies to `main` and a direct push is refused | `0002 AC-13`, remote half | Remote steps 3 and 4 | `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| Item 12 of "Document corrections" and the closing evidence | `0002 AC-19`, item 12 | Remote step 7 | `NOT_RUN`, `owner: lead` |

### Commands

Run from the repository root at `a3258e4`.

The entry point and the committed files, `0002 AC-1`, `0002 AC-4`, `0002 AC-5`:

- [x] `scripts/check.sh all`, run twice, once before the probes and once after them → exit 0 both times. `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` each end with `== ok: <name>`, and the summary repeats the six lines. No line says a command was skipped. 332 tests passed, 0 failed, with the split given above. Both `cargo deny` runs print `advisories ok, bans ok, licenses ok, sources ok`, with the one expected warning `license-exception-not-encountered` on the product lockfile. `pool-repo-check workspace: ok`, `pool-repo-check ci: ok`, `pool-repo-check docs: ok`. `git status --short` is empty afterward and `tools/repo-check/target` does not exist → `0002 AC-1`, `0002 AC-4`, `0002 AC-6`, `0002 AC-7`, `0002 AC-10`
- [x] `deny.toml` compared with the block under [Dependency policy](index.md#dependency-policy) → the same 32 lines, `unsound = "all"` included → `0002 AC-4`
- [x] `scripts/linux-build.sh` compared with its two sources → lines 1 to 23, up to and with the marker line, equal the block under [Linux build script](index.md#linux-build-script). The marker occurs once, on line 23. The 28 lines after it (1100 bytes) equal the first code block under `### Linux build` in spec 0001. `.github/workflows/ci.yml` still equals its spec block. `git ls-files -s scripts` shows mode `100755` for all three scripts → `0002 AC-5`, `0002 AC-1`
- [x] `scripts/check.sh linux-build` → exit 0. `runner: Darwin arm64`, `host: aarch64-unknown-linux-gnu`, `release: 1.100.0`, 136 tests passed in the container, 30 fixture lines (6 with `exit=0`, 24 with `exit=1`, no `unexpected exit`, no `canary leaked`), `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool` (the same hash as in every earlier run), `pool 0.1.0`, `linux build check passed`, `== ok: linux-build`. The tree is clean afterward → `0002 AC-5`, `0002 AC-1` (a workstation reproduction; the evidence the criterion asks for is the CI log of remote step 2)

The first part of `scripts/linux-build.sh`, probed with a stand in `docker` on `PATH` that answers `info` and records every other call. In each case below the stand in recorded no `docker run`, so no archive was sent to a container → `0002 AC-1`, `0002 AC-5`:

- [x] The script copied to a folder that is in no repository → exit 3, stdout `runner: Darwin arm64`. Two lines on stderr: `fatal: not a git repository (or any of the parent directories): .git` from `git`, then `linux-build: git cannot read HEAD here; this check builds only the committed tree`
- [x] The script in a repository made with `git init` and no commit → exit 3. One line on stderr, the `git cannot read HEAD here` line
- [x] A clone whose `.git/index` holds seven bytes of text, so that `git status` fails → exit 3. Two lines on stderr: `fatal: .git/index: index file smaller than expected` from `git`, then `linux-build: git status failed`. The same with a `git` on `PATH` that exits 128 for `status` only → exit 3, its own line and then `linux-build: git status failed`. So a `git status` that fails is not read as a clean tree
- [x] `GIT_TEST_ASSUME_DIFFERENT_OWNER=1 scripts/linux-build.sh` in a clone (an ownership refusal) → exit 3. Five lines on stderr: four from `git` (`fatal: detected dubious ownership in repository at ...` and its advice), then the `git cannot read HEAD here` line
- [x] A clone with one deleted tracked file, and a clone with one untracked file → exit 3 for each, one line: `linux-build: commit or remove local changes first; this check builds only the committed tree`
- [x] The stand in answering `info` with a failure → exit 3, one line: `linux-build: the Docker daemon is not running`
- [x] `scripts/check.sh linux-build` outside a repository, and with the `git` that fails for `status` → exit 3 for each, `== FAILED: linux-build`, the same stderr lines as above
- [x] `CDPATH=<a folder that holds a folder named scripts> sh scripts/linux-build.sh` from the root of a clean clone, and the same through `sh scripts/check.sh linux-build` → the stand in was called from the clone root both times, and the `docker run` it recorded is the command of spec 0001. An exported `CDPATH` no longer sends either script to another directory

The inventory, `0002 AC-16` and `0002 AC-1` (`check_sbom` changed in `edc9fc6`):

- [x] `scripts/check.sh sbom`, `target/sbom` removed first → exit 0, `== ok: sbom`. One file, `target/sbom/pool.cdx.json`, 20264 bytes. `git status --short` is empty and no `*.cdx.json` file is left outside `target/` → `0002 AC-16`
- [x] The file read with a JSON parser → `bomFormat` `CycloneDX`, `specVersion` `1.5`, `metadata.component` `pool` 0.1.0, 17 components, none without a license, 18 `dependencies` entries. The 17 component names plus `pool` equal the 18 names of `tools/repo-check/baseline/lock-packages.txt`. The one placement is the same as before: `pool` is the subject of the document, not a component → `0002 AC-16`
- [x] In a clone, a stand in `cargo-cyclonedx` under `target/tools/bin` that prints the right version line, writes `apps/pool/pool.cdx.json` and `crates/pool-config/pool.cdx.json`, and then exits 1 → `scripts/check.sh sbom` exits 1, `== FAILED: sbom`. `git status --short` is empty, no `pool.cdx.json` is left outside `target/`, and `target/sbom/` was not created. The same when the stand in writes only the `pool-config` file and exits 101, when it fails before it writes, and when it writes one file and exits 3 (exit 1, because only `pool-repo-check` and `scripts/linux-build.sh` may report could not run with 3) → `0002 AC-1`, `0002 AC-16`
- [x] Control: the stand in writes both files and exits 0 → exit 0, `== ok: sbom`, `target/sbom/pool.cdx.json` is there and the tree is clean. The stand in exits 0 but writes no file for `pool` → exit 1 with the `mv` error, and the other file is removed → `0002 AC-16`

The dependency policy, `0002 AC-4` (`unsound = "all"`, `a251c9f`):

- [x] In a clone, `db-path` added under `[advisories]` as in case 6, then `scripts/check.sh deny` → exit 0 on the real database. Then a made up notice `RUSTSEC-0000-0002` for `itoa` with `informational = "unsound"` put into that database. `itoa` reaches `pool` only through `serde_json`. `scripts/check.sh deny` → exit 1, `error[unsound]`, `ID: RUSTSEC-0000-0002`, `== FAILED: deny`. Control: the same notice with the line `unsound = "all"` taken out of `deny.toml` → exit 0. So the new line is what makes an unsound indirect dependency fail. The notice never left the scratch directory → `0002 AC-4`

The proof script, `0002 AC-11`:

- [x] `scripts/prove-checks.sh --with-linux-build` → exit 0. It first ran `scripts/check.sh all` on the unchanged copy, then printed `PASS 1` to `PASS 28`, with no `FAIL` and no `SKIPPED` line. The last line is `all checks reject their bad change`. Afterward `git status --short` is empty, `HEAD` is still `a3258e4` and the temporary copy under `$TMPDIR/pool-prove.*` is gone → `0002 AC-11`
- [x] The line the script printed under each `PASS` → exit 1 for every case. Cases 1 to 20 match the same text as in the milestone 4 record. Case 21 now reports `W7: scripts/linux-build.sh:50`, because the first part of the script grew by eight lines. The new cases: 22 `error[source-not-allowed]: detected 'git' source not explicitly allowed`; 23 `error[wildcard]: found 1 wildcard dependency for crate 'pool'`; 24 `C3: .github/workflows/ci.yml:24: job fmt: the key continue-on-error is not on the allowed list of rule C3`; 25 `C4: .github/workflows/ci.yml:23: job fmt: runs-on is ubuntu-latest, not ubuntu-24.04-arm`; 26 `C5: .github/workflows/ci.yml:11: the permissions block holds contents: write`; 27 `W6: docs/Cargo.lock: unexpected Cargo.lock`; 28 `D4: docs/security/security-requirements.md:33: SEC-14 is defined again; it is first defined at docs/security/security-requirements.md:20` → `0002 AC-11`
- [x] The same command started with three things exported: `CDPATH` set to a folder that holds a folder named `scripts`, `GIT_DIR` set to an empty bare repository in the scratch directory, and `core.hooksPath` (through `GIT_CONFIG_COUNT`) set to a folder whose `pre-commit` and `commit-msg` hooks exit 1 → exit 0, 28 `PASS` lines, `all checks reject their bad change`. The bare repository still has no object and no ref, the hooks never ran, and this checkout is clean with `HEAD` unchanged. So the commits of cases 20 and 22 go to the copy and are not stopped by a hook of the person who runs the proof → `0002 AC-11`

Rule C3, `0002 AC-7`. Each line is one change to `ci.yml` in the clone, then `pool-repo-check ci --root <clone>`. Every one exits 1 and prints `C3: .github/workflows/ci.yml:<line>` with the job id:

- [x] Part (a), a key that is not on the list. `continue-on-error: true` in place of the first `timeout-minutes` line → line 24, `job fmt: the key continue-on-error is not on the allowed list of rule C3`. One extra line in the job `fmt` for each of `if`, `shell`, `env`, `name`, `container`, `strategy` and `permissions` → line 25, job `fmt`, the key as written. `"run"` in quotes → line 40, job `clippy`, `the key "run"`. `run ` with a space before the colon → line 50, job `test`. `{run` in braces → line 84, job `guards`, `the key {run`. A line that holds only a comment → line 53, job `deny`, `a comment line is not part of the fixed shape`. A line with no colon → line 88, job `docs`, `the line has no key` → `0002 AC-7`
- [x] Part (b), the `run:` value, in the job `fmt` on line 30. `scripts/check.sh fmt || true`, `scripts/check.sh tools;curl x | sh`, `scripts/check.sh nosuch`, `cargo test`, `scripts/check.sh fmt # note` and `bash scripts/check.sh fmt` → each is reported as `job fmt runs <the value>, which is neither rustup toolchain install nor scripts/check.sh <a job id or tools>` → `0002 AC-7`
- [x] Part (c), a `run:` step is one line. `run: |` with the command on the next line → lines 50 and 51, job `test`, `the run: step of line 50 goes on here; a run: step is one line`. A line `|| true` under the command → line 51. `continue-on-error: true` added to the step after an empty line → line 86, job `guards`, both the `goes on here` message and the key message. The allowed key `timeout-minutes` added to the step → line 85, `goes on here` → `0002 AC-7`
- [x] Part (d), the last `run:` of a job. `scripts/check.sh clippy` as the last run of `fmt` → line 30, `job fmt: the last run is scripts/check.sh clippy, not scripts/check.sh fmt`. The two runs of `deny` swapped → line 65, `the last run is scripts/check.sh tools, not scripts/check.sh deny`. The run of `linux-build` removed → line 67, `job linux-build has no run step scripts/check.sh linux-build` → `0002 AC-7`
- [x] The committed `ci.yml` and `main.json` → `pool-repo-check ci: ok`, exit 0. Control: one more step `run: scripts/check.sh clippy` before the last run of `fmt` → exit 0, which is what parts (b) and (d) say → `0002 AC-7`

Rules C4 and C5, `0002 AC-7`. Each exits 1 and names the workflow path:

- [x] C4. The first `runs-on` value changed to `ubuntu-latest` → `C4: .github/workflows/ci.yml:23: job fmt: runs-on is ubuntu-latest, not ubuntu-24.04-arm`. The `runs-on` line of `clippy` removed → line 32, `job clippy has no runs-on: line`. A second `runs-on` line in `test` → line 44, `job test has 2 runs-on: lines, not one`. A list as the value in `guards` → line 77, reported with the value as written → `0002 AC-7`
- [x] C5, one workflow file. A second file `.github/workflows/other.yml` (with `permissions: write-all` and a `curl ... | sh` step) → `C5: .github/workflows/other.yml: .github/workflows/ holds a workflow other than .github/workflows/ci.yml`. A file in a folder below, `.github/workflows/sub/deep.yaml` → the same message with that path → `0002 AC-7`
- [x] C5, the closed `permissions` block. `contents: write` → line 11. `permissions: write-all` on one line, and `permissions: # read only` → line 10, `the permissions line must be permissions: at the start of a line with nothing after it`. A second line `pull-requests: write` in the block → line 12. A second line `id-token: write` after an empty line → line 13. No `permissions` line → `the workflow has 0 lines whose key is permissions, not one`. Two top level blocks, and one more `permissions` line inside a job → `the workflow has 2 lines whose key is permissions, not one` → `0002 AC-7`
- [x] C5, the two texts. `${{ secrets['X'] }}` in the `env` block → `C5: .github/workflows/ci.yml:19: the workflow contains the text secrets`, so the index form is seen now. `pull_request_target` → line 4 → `0002 AC-7`

Rules W4 and W5, `0002 AC-6`. Each line is one change to a manifest in the clone, then `pool-repo-check workspace --root <clone>`:

- [x] W4 and comments. `[lints] # inherit` with `workspace = true # from the root` in `apps/pool/Cargo.toml` → exit 0, the false alarm is gone. `unsafe_code = "forbid" # the rule` in the root → exit 0. `workspace = true` only inside a comment, the `[lints]` header inside a comment, and `workspace = false` → exit 1, `W4: <manifest path>: the manifest has no [lints] table with workspace = true`. `unsafe_code = "forbid"` only inside a comment in the root → exit 1, `W4: Cargo.toml`. A `*` in a member string → exit 1, `W4: Cargo.toml: member apps/* uses a wildcard, which the guard cannot follow` → `0002 AC-6`
- [x] W5, the pair `pool-config -> pool` written in the ways the rule now reads. Under a header with a comment (`[dependencies] # runtime`); with the key in double quotes and in single quotes; as a table of its own, `[target.'cfg(unix)'.dependencies.pool]` and `[dependencies.pool]`; as a dotted key under `[target.'cfg(unix)'.dev-dependencies]`; under another key on one line, `daemon = { package = "pool", path = "../../apps/pool" }`; under another key in a table of its own, `[build-dependencies.daemon]` with `package = "pool" # renamed` → exit 1 for each, `W5: crates/pool-config/Cargo.toml:<line>: dependency pool-config -> pool is not allowed by internal-dependencies.txt` → `0002 AC-6`
- [x] W5, the allowed pair `pool -> pool-config` written in other ways. As `cfg = { package = "pool-config", path = "../../crates/pool-config" }`, with a quoted key and a comment, and as the table `[dependencies.pool-config]` → exit 0 for each, so the allowed pair is no longer reported as missing. The pair only inside a comment, and the key `pool-config` standing for another package (`package = "serde_json"`) → exit 1, `dependency pool -> pool-config is listed but missing from the manifests`. A `#` and a member name inside a quoted value → exit 0 → `0002 AC-6`

Rule D6, `0002 AC-10`. `pool-repo-check docs --root <clone> --base <revision>`:

- [x] The committed tree with `--base HEAD` → exit 0, `pool-repo-check docs: ok` → `0002 AC-10`
- [x] A base revision in which `docs/security/security-requirements.md` holds bytes that are not UTF-8, with the file put right again in the next commit → exit 3, `pool-repo-check docs: could not run: cannot read the base revision: git show <revision>:docs/security/security-requirements.md: output is not UTF-8`. The same for a base in which `docs/specs/0001-initial-architecture/index.md` is not UTF-8. This is the case of review finding M5, and it now ends as could not run. Control: a base in which only `README.md`, which owns no IDs, is not UTF-8 → exit 0 → `0002 AC-10`
- [x] A revision that does not exist, `--base --full-tree`, `--base -h` and `--base 'HEAD^{tree}'` → exit 3 for each, `git rev-parse: <revision> is not a commit`. A root that is the folder `docs/` of the clone → exit 3, `the root ... is the folder docs/ of a Git work tree, not its top; --base needs the repository root` → `0002 AC-10`
- [x] The row of `SEC-14` removed, with `--base HEAD` → exit 1, `D6: docs/security/security-requirements.md: SEC-14 is defined at the base revision but no longer defined`, next to the D5 lines for the places that cite it → `0002 AC-10`

The record itself:

- [x] `scripts/check.sh docs` with this section in place → exit 0, `pool-repo-check docs: ok`, so every link and ID written here resolves → `0002 AC-8`

### Acceptance-criteria coverage

Only the criteria whose path changed in `535bd15..a3258e4`. For the rest, the full verify above stands.

| Criterion | Evidence on `a3258e4` | Result |
|---|---|---|
| `0002 AC-1` | `all` exits 0 twice with six `== ok:` lines. `linux-build` exits 0. Every could not run case of the Linux build script exits 3 before `docker run`. `sbom` exits 0, and exits 1 with a clean tree when its tool fails. An exported `CDPATH` does not move either script | met |
| `0002 AC-4` | `deny.toml` equals the spec block, with `unsound = "all"`. `deny` passes on both lockfiles with `cargo-deny 0.20.2`. Cases 5, 6, 7, 22 and 23 rejected. The unsound probe fails with the line and passes without it | met |
| `0002 AC-5` | The first 23 lines equal the spec 0002 block and the 28 lines after the marker equal the spec 0001 block. Rule W7 passes, case 21 rejected. The local run passes on `aarch64-unknown-linux-gnu`, case 20 rejected in the container. The CI log is remote step 2 | workstation half met; CI half `PASS` on 2026-11-14, see [Remote evidence](#remote-evidence) |
| `0002 AC-6` | `pool-repo-check workspace: ok`. The W4 and W5 probes above. Cases 8 to 12, 21 and 27 rejected. 198 `pool-repo-check` tests pass | met |
| `0002 AC-7` | `pool-repo-check ci: ok` on the committed files. A rejected change for each of C3 (a) to (d), C4 and C5, with the rule ID, the path, the line, the job id and the key as written. Cases 13, 14, 24, 25 and 26 rejected | met |
| `0002 AC-10` | `docs` passes with a base. A base that cannot be read, a root that is not the top of the work tree and a revision that is not a commit exit 3. Cases 18, 19 and 28 rejected | met |
| `0002 AC-11` | 28 `PASS` lines, `all checks reject their bad change`, exit 0, clean tree, also with `CDPATH`, `GIT_DIR` and a failing commit hook exported | met |
| `0002 AC-16` | `sbom` writes the one CycloneDX 1.5 file with a license for each component, the names equal the baseline, the tree is clean, and a failing tool leaves no file behind | met |

Surfaces: outside `docs/` and `graphify-out/`, the only paths added in `535bd15..a3258e4` are the test files under `tools/repo-check/tests/`, which belong to the package the spec lists, and no path was removed (`git diff --name-status 535bd15..a3258e4`). `.github/workflows/` holds `ci.yml` only.

### Readings the checker assessed

- **How many lines the Linux build script prints when `git` cannot work.** `0002 AC-1` asks for one line that says what is missing for a missing tool or a stopped Docker daemon, and both give one line. Where `git` itself fails, `git` prints its own message first and the script adds its line after it: two lines outside a repository, one line with no commit, two lines when `git status` fails, five lines on an ownership refusal. The script equals the spec block, and that block sends only the stdout of `git rev-parse` away, so this is what the spec text gives. The line of the script is always the last one and always names the reason. The checker reads the criterion as met and lists the extra lines for the review below.
- **What a C3 message names.** [What each message must name](index.md#what-each-message-must-name) asks for the path with the line number and the job id, and for the key as written when a key is not allowed. Every C3 message has the first two. The key is named in every part (a) message that has a key. A comment line and a line with no colon have no key to name.
- **One more allowed step passes.** A step `run: scripts/check.sh clippy` in front of the last run of `fmt` is accepted. Part (b) allows `scripts/check.sh <name>` for any job id, and part (d) fixes only the last run. That is the text of the rule, and rule C2 still ties the job ids to the required checks.
- **A wildcard member gives two lines.** With `apps/*` in the `members` array, W4 reports the wildcard and W5 also reports the allowed pair as missing, because the guard cannot see the member behind the wildcard. Both lines are true of what the guard can read, and the exit is 1.
- **The tool exit 3 in `sbom`.** A `cargo-cyclonedx` that exits 3 gives exit 1 for the check. The status rule keeps 3 for `pool-repo-check`, for `scripts/linux-build.sh` and for a tool that is missing, so 1 is what the text gives.

### For `/check review`

None of these goes against a criterion.

- `git` writes its own message to stderr before the line of `scripts/linux-build.sh` (one to four extra lines, see the first reading). A reader who expects one line sees more.
- After a good `sbom` run, a later run whose tool fails leaves the older `target/sbom/pool.cdx.json` in place. The exit is 1 and `target/` is ignored by Git, but the file is then not the result of the last run.
- Rule C5 reports the text `secrets` wherever it stands in `ci.yml`, a comment included. That is the rule as written, and it means the word cannot be used in a comment there.
- The three points named at the end of the full verify are closed by this round: a second workflow file is now a problem of C5, an exported `CDPATH` no longer moves the scripts, and `check_sbom` cleans up when its tool fails.

### Not run

- Anything under [Remote proof](index.md#remote-proof). The four remote items and item 12 stay `NOT_RUN`, `owner: lead`.
- `scripts/check.sh fuzz`: `check_fuzz`, the fuzz package and its lockfile did not change in `535bd15..a3258e4`.
- The criteria whose path did not change (`0002 AC-2`, `0002 AC-8`, `0002 AC-9`, `0002 AC-14`, `0002 AC-15`, `0002 AC-17` to `0002 AC-20`) beyond what `scripts/check.sh all` and the proof cover.
- A run without the network, and any run on Linux outside the container or on a GitHub runner.

## Remote evidence

Recorded by the checker session `chk-f2-verify` on 2026-11-15. The checker did not write the code and ran none of the remote steps. The lead ran remote steps 1 to 6 under [Remote proof](index.md#remote-proof) on 2026-11-14 (UTC), with the permission the repository owner gave for this scope. The merge of step 6, at 23:38 UTC, is 06:38 on 2026-11-15 in the local time of the repository owner. The checker then read each result back from GitHub with `gh`, with read requests only, and ran the workstation commands listed below. A value that could not be read back from GitHub is marked **reported by the lead**. Everything else was read by the checker. All times are UTC.

**Verdict**: `PASS` for the four remote items. `0002 AC-12`, the CI half of `0002 AC-5` and the remote half of `0002 AC-13` are met. `0002 AC-3` is met for its runs (remote steps 1 and 6); its last clause, the link from scope 002, is an edit of the lead and reaches `main` with remote step 7. Remote step 7, item 12 of "Document corrections" and remote step 8 are not run yet. Two things went differently from the text of "Remote proof" as it stood at `d463c1d`. Neither goes against a criterion (see [Readings the checker assessed](#readings-the-checker-assessed-remote-evidence)).

**Repository**: `ojiepermana/pool`, public, default branch `main`, no self hosted runner (`gh api repos/ojiepermana/pool/actions/runners` reports 0).

**Revision on `main`**: `d463c1d8a6888e0b1f58a65d42f8449ca7757160`, the merge commit of pull request 1, with the parents `6de697005c5f23be9d176a7ee2109ff07becd1f2` (the earlier `main`) and `5452f9c0e09c3e50dd9647f5a5a2397896189966` (the head of the pull request). `git diff --quiet 5452f9c d463c1d` exits 0, so the tree on `main` is the tree the seven checks passed on. `git rev-parse main origin/main` and `git ls-remote origin refs/heads/main` all give that hash.

The cells that say what the four remote items gave were changed from `NOT_RUN`, `owner: lead` to their result in the tables of milestone 4, of the full verify and of the verify after the review fixes. Nothing else was changed in those records. Their sentences, their "Not run" lists and the milestone 2 record still say what was true on their day; where they name one of the four items as `NOT_RUN`, this section is the later result.

### What happened, in order

Read from the pull request timelines, the run list, the repository activity list and the rule suite list.

| Time (2026-11-14) | Event | Read from |
|---|---|---|
| 14:44:14 | Branch `scope/002-standards-and-tooling` is created at `6cbc33e` | `gh api repos/ojiepermana/pool/activity` |
| 14:44:17 | Pull request 1 is opened. The only run for it is `Copilot`, run 43606433608. No `ci` run starts | `gh pr view 1`, `gh run list` |
| 14:49:50, 14:49:55 | Pull request 1 is closed and opened again | `gh api repos/ojiepermana/pool/issues/1/timeline` |
| 14:49:57 to 14:52:02 | `ci` run 1 (43606905477) on `6cbc33e`, `success`. GitHub lists the workflow `ci` as created at 14:49:57 | `gh run view`, `gh api .../actions/workflows` |
| 14:50:22, 14:50:23 | The automatic Copilot review leaves two review threads on `6cbc33e` | `gh api .../pulls/1/comments` |
| 14:52:43 | Ruleset 27266107 `main` is created, `active` | `gh api .../rulesets` |
| 14:53:39, 14:53:41 | The probe commit is made and its push to `main` is evaluated: `fail` | `git log -1 2a7d9d2`, `gh api .../rulesets/rule-suites` |
| 14:53:56 to 14:56:19 | Branch `prove/fmt-fails` is created at `a9870c6`, pull request 2 is opened, `ci` run 2 (43607256415) ends with `failure`, the pull request is closed and the branch is deleted | activity list, `gh pr view 2`, timeline |
| 15:00:36 | Push to the scope branch, `6cbc33e` to `5452f9c` (the two fixes for the review threads) | activity list |
| 15:00:42 to 15:02:36 | `ci` run 3 (43607823191) on `5452f9c`, `success` | `gh run view` |
| 23:38:00 | Pull request 1 is merged: `main` goes from `6de6970` to `d463c1d`, activity type `pr_merge`, rule suite `pass` | activity list, rule suite list |
| 23:38:04 to 23:39:55 | `ci` run 4 (43654217182) for the push to `main`, `success` | `gh run view` |

The four `ci` runs have the run numbers 1 to 4 with no gap, so no `ci` run was removed. The rule was created 41 seconds after run 1 had ended with seven passes, as "Remote proof" asks.

### Remote step 1: the seven checks on a pull request (`0002 AC-3`)

- [x] `gh pr view 1 --json url,state,baseRefName,headRefName,headRefOid` → https://github.com/ojiepermana/pool/pull/1, base `main`, head branch `scope/002-standards-and-tooling`, head `5452f9c0e09c3e50dd9647f5a5a2397896189966`, not a draft → `0002 AC-3`
- [x] `gh run view 43606905477` and `gh api repos/ojiepermana/pool/actions/runs/43606905477/jobs` → https://github.com/ojiepermana/pool/actions/runs/43606905477, workflow `ci`, event `pull_request`, head `6cbc33ec1e38859f39849dea04cf8e720f004203`, conclusion `success`. Seven jobs, `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards` and `docs`, each with the conclusion `success`, the label `ubuntu-24.04-arm` and the runner group `GitHub Actions` → `0002 AC-3`
- [x] The same two commands for run 43607823191 → https://github.com/ojiepermana/pool/actions/runs/43607823191, workflow `ci`, event `pull_request`, head `5452f9c0e09c3e50dd9647f5a5a2397896189966`, conclusion `success`, the same seven jobs with `success` on `ubuntu-24.04-arm`. This is the run on the head that was merged → `0002 AC-3`
- [x] `gh pr checks 1` → seven lines marked `pass`, one for each of `clippy`, `deny`, `docs`, `fmt`, `guards`, `linux-build` and `test`, all from run 43607823191 → `0002 AC-3`
- [x] The test counts in the `test` job logs → 334 on `6cbc33e` and 339 on `5452f9c`, 0 failed. In both, 136 in the workspace (3 + 68 + 3 + 60 + 2, the Linux count). In `pool-repo-check`, 198 (90 + 108) on `6cbc33e` and 203 (93 + 110) on `5452f9c` → `0002 AC-3`

### Remote step 2: the Linux build lines (`0002 AC-5`, CI half)

- [x] `gh run view 43607823191 --log`, the lines of the job `linux-build`, in this order → `runner: Linux aarch64`, `host: aarch64-unknown-linux-gnu`, `release: 1.100.0`, 136 tests passed (3 + 68 + 3 + 60 + 2), `cb184231b4af3e5c927eb46bb1fd7ad289b5a357a7c8b62a14d45fd6b92d3532  target/release/pool`, `pool 0.1.0`, `linux build check passed`, and then the one line that `scripts/check.sh` adds, `== ok: linux-build` → `0002 AC-5`
- [x] The same lines in the log of run 43606905477 (`6cbc33e`), of run 43654217182 (`d463c1d`, the push to `main`) and of run 43607256415 (the failing pull request, where `linux-build` passed) → the same seven values, and the same `sha256sum` line in all four runs. It is also the hash of every workstation run recorded above → `0002 AC-5`
- [x] The set up lines of that job → runner image `ubuntu-24.04-arm`, version `20261111.182.1`. The container image is pulled by digest, `rust:1.100.0-slim-bookworm@sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37`, and prints `rustc 1.100.0 (0ef1bc386 2026-11-09)`. The runner and the target have the same architecture, so nothing is emulated → `0002 AC-5`

### Remote step 3: the branch rule (`0002 AC-13`, remote half)

- [x] `gh api repos/ojiepermana/pool/rulesets` → one ruleset: `27266107 main active`, target `branch`. `gh api repos/ojiepermana/pool/rulesets/27266107` → `bypass_actors` is empty, `current_user_can_bypass` is `never`, and the condition is `~DEFAULT_BRANCH` → `0002 AC-13`
- [x] `gh api repos/ojiepermana/pool/rules/branches/main --jq '[.[].type] | sort | join(",")'` → `deletion,non_fast_forward,pull_request,required_status_checks`, all four from ruleset 27266107 → `0002 AC-13`
- [x] The same address with the filter of remote step 3 for the contexts → `clippy,deny,docs,fmt,guards,linux-build,test` → `0002 AC-13`
- [x] The live ruleset compared with `.github/rulesets/main.json`, both sorted with `jq -S` → every key of the file is there with the same value. The live rule has three keys more, which GitHub returned and the file does not send: `require_extra_approval_for_unattributed_changes: true` and `required_reviewers: []` under `pull_request`, and `do_not_enforce_on_create: false` under `required_status_checks` → `0002 AC-13`
- [x] `gh api repos/ojiepermana/pool/branches/main/protection` → 404, `Branch not protected`. So there is no older branch protection next to the ruleset, and the ruleset is the only rule on `main`
- [ ] `mergeStateStatus` of pull request 1 right after the rule was applied → **reported by the lead**: `BLOCKED`, not the `CLEAN` that step 3 expects, and `CLEAN` before the merge. A past value of this field cannot be read back. What can be read agrees with it: the live rule has `required_review_thread_resolution: true`, and the two review threads were written at 14:50, before the rule existed at 14:52

The two review threads, read with a GraphQL query on `reviewThreads` and with `gh api repos/ojiepermana/pool/pulls/1/comments`:

| Thread | About | Fix | State now |
|---|---|---|---|
| https://github.com/ojiepermana/pool/pull/1#discussion_r4574073043 on `scripts/check.sh` line 164 | An exported `CDPATH` can send the child shell of `check_fuzz` to another folder | `8cfb720`: the line now starts with `CDPATH= cd --` | resolved by `Copilot`, outdated |
| https://github.com/ojiepermana/pool/pull/1#discussion_r4574073117 on `tools/repo-check/src/text.rs` line 186 | A link inside a code span with two backticks on each side was read as a real link | `3bb3cd2`: runs of backticks of equal length are paired, with new tests | resolved by `ojiepermana`, outdated |

Both fixes are in `6cbc33e..5452f9c` (`8cfb720`, `3bb3cd2`, the merge `6fc1838`, and the log commit `5452f9c`). That the second thread was marked resolved through the lead, after the repository owner allowed it, is **reported by the lead**; GitHub shows only the account `ojiepermana`.

### Remote step 4: a direct push is refused (`0002 AC-13`, remote half)

The output on the terminal of the lead (`GH013`, "Changes must be made through a pull request", "7 of 7 required status checks are expected", and two equal `git ls-remote` lines) is **reported by the lead**. The push did leave a trace on GitHub, and the checker read it:

- [x] `gh api repos/ojiepermana/pool/rulesets/rule-suites` and `.../rule-suites/5118624627` → one evaluation for `refs/heads/main` at 14:53:41, from `6de697005c5f23be9d176a7ee2109ff07becd1f2` to `2a7d9d284f96fdbd981987a3b794e5cdfa9cf944`, result `fail`. `pull_request`: `fail`, "Changes must be made through a pull request." `required_status_checks`: `fail`, "7 of 7 required status checks are expected." `non_fast_forward` and `deletion`: `pass`. All four are from ruleset 27266107 with the enforcement `active` → `0002 AC-13`
- [x] `gh api "repos/ojiepermana/pool/activity?ref=refs/heads/main"` → the last two entries are a `push` to `6de6970` at 12:10:07 and the `pr_merge` to `d463c1d` at 23:38:00. No entry puts `main` on the probe commit, and none is a forced push → `0002 AC-13`
- [x] `git log -1 2a7d9d2` in the shared object store → the probe is a commit with the parent `6de6970` and the same tree as `6de6970`, with the message of remote step 4. `git merge-base --is-ancestor 2a7d9d2 origin/main` exits 1, and `git branch -a --contains 2a7d9d2` prints nothing → `0002 AC-13`
- [x] `gh api repos/ojiepermana/pool/commits/2a7d9d2.../branches-where-head` → `[]`. No pull request and no check run belongs to the commit. GitHub still returns the commit when it is asked for by its hash, because it keeps the object of a refused push; no branch holds it → `0002 AC-13`

### Remote step 5: a wrong change is blocked (`0002 AC-12`)

- [x] `gh pr view 2 --json url,state,headRefName,headRefOid,mergedAt,mergeCommit,mergeStateStatus,statusCheckRollup` → https://github.com/ojiepermana/pool/pull/2, "Do not merge: proves the fmt check", head branch `prove/fmt-fails`, head `a9870c6b895d425992dab3f45b39aee6bc138cba`, state `CLOSED`, `mergedAt` and `mergeCommit` empty, `mergeStateStatus` `BLOCKED`, `fmt=FAILURE`, `clippy=FAILURE` and `SUCCESS` for the other five → `0002 AC-12`
- [x] `gh run view 43607256415` → https://github.com/ojiepermana/pool/actions/runs/43607256415, event `pull_request`, head `a9870c6b895d425992dab3f45b39aee6bc138cba`, conclusion `failure`. `gh pr checks 2` exits 1 and shows `fmt` and `clippy` as `fail` and the other five as `pass` → `0002 AC-12`
- [x] `gh api repos/ojiepermana/pool/commits/a9870c6...` → one parent, `6cbc33e`, and one changed file: `apps/pool/src/main.rs`, one line added, `fn  prove_fmt( ){}`. That is the change of proof case 1. The failed log of `fmt` shows `Diff in /home/runner/work/pool/pool/apps/pool/src/main.rs:91:` and `== FAILED: fmt`. The log of `clippy` shows ``error: function `prove_fmt` is never used``, as step 5 says it would → `0002 AC-12`
- [x] The timeline of pull request 2 → opened at 14:53:58, `closed` at 14:56:18, `head_ref_deleted` at 14:56:19, and no `merged` or `reopened` event. `git ls-remote origin refs/heads/prove/fmt-fails` prints nothing, and `gh api .../branches/prove/fmt-fails` gives 404 → `0002 AC-12`
- [x] The pull request was opened at 14:53:58, after the rule existed (14:52:43), and no entry in the activity list of `main` comes from it → `0002 AC-12`

### Remote step 6: the merge (`0002 AC-3`)

- [x] `gh pr view 1 --json state,mergeCommit,mergedAt,mergedBy` → `MERGED`, merge commit `d463c1d8a6888e0b1f58a65d42f8449ca7757160`, at 23:38:00, by `ojiepermana`. `gh api repos/ojiepermana/pool/pulls/1` → `base.sha` is `6de6970`, so the branch was up to date with `main`, which the strict status check setting asks for → `0002 AC-3`
- [x] `gh api .../rulesets/rule-suites/5123565289` → the update of `refs/heads/main` from `6de6970` to `d463c1d` at 23:38:00 has the result `pass`, with `pass` for `required_status_checks`, `pull_request`, `non_fast_forward` and `deletion`. The activity type is `pr_merge`. So the merge went through the rule and not around it → `0002 AC-3`, `0002 AC-13`
- [x] `gh run view 43654217182` → https://github.com/ojiepermana/pool/actions/runs/43654217182, workflow `ci`, event `push`, branch `main`, head `d463c1d8a6888e0b1f58a65d42f8449ca7757160`, conclusion `success`, seven jobs with `success` on `ubuntu-24.04-arm`. The run had ended when the checker read it. 339 tests passed in its `test` job (136 + 93 + 110) → `0002 AC-3`
- [x] `git ls-remote origin 'refs/heads/*'` → `main` at `d463c1d`, `scope/002-standards-and-tooling` at `5452f9c` (it stays, as step 6 says), and `docs/cross-check-before-development`, which is older than this scope. `gh pr list --state all` → two pull requests, 1 `MERGED` and 2 `CLOSED`
- [ ] `mergeStateStatus` `CLEAN` right before the merge → **reported by the lead**. The rule suite above is the stronger evidence for the same thing

### The revision that was pushed (review finding r2-m1)

"Remote proof" asks that `scripts/check.sh all` and `scripts/prove-checks.sh` pass on exactly the revision that step 1 pushes, and that the checker records that revision and its test counts first. The record before this one names `a3258e4`. Two later revisions were pushed.

- `a3258e4` to `6cbc33e`, the first head. `git diff --stat a3258e4 6cbc33e`, outside `docs/` and `graphify-out/`: `apps/pool/tests/check_config.rs` (the fix for the closed pipe tests, `648db59`), `AGENTS.md`, `CLAUDE.md`, `crates/pool-config/AGENTS.md` and `CHANGELOG.md`. No product source, script, guard or CI file changed.
- `6cbc33e` to `5452f9c`, the merged head. Outside `graphify-out/`: `scripts/check.sh` (one line), `tools/repo-check/src/text.rs`, `tools/repo-check/src/docs.rs`, `tools/repo-check/tests/repo_check/docs.rs` and the coordination log. These are the two fixes for the review threads.
- That `scripts/check.sh all` and `scripts/prove-checks.sh --with-linux-build` (28 `PASS`) passed on `6cbc33e` and on `5452f9c` before each push is **reported by the lead**.

The checker ran both on `d463c1d`, whose tree equals `5452f9c`. Workstation: macOS 27.0.1, `arm64`, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo-deny 0.20.2`, Docker server 29.9.2 (`linux/aarch64`), `git 2.54.0`, `gh 2.96.0`. Not Linux and not CI. `git status --short` was empty and there was no `target/` directory before the first command.

- [x] `scripts/check.sh all` at `d463c1d` → exit 0. `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` each end with `== ok: <name>`, and the summary repeats the six lines. No line says a command was skipped. **337 tests passed, 0 failed**: 134 in the workspace (3 + 66 + 3 + 60 + 2) and 203 in `pool-repo-check` (93 unit tests and 110 integration tests). Both `cargo deny` runs print `advisories ok, bans ok, licenses ok, sources ok`, with the one expected warning `license-exception-not-encountered`. `pool-repo-check workspace: ok`, `pool-repo-check ci: ok`, `pool-repo-check docs: ok`. `git status --short` is empty afterward and `tools/repo-check/target` does not exist → `0002 AC-1`, `0002 AC-4`, `0002 AC-6`, `0002 AC-7`, `0002 AC-8`
- [x] `scripts/prove-checks.sh --with-linux-build` at `d463c1d` → exit 0, `PASS 1` to `PASS 28`, no `FAIL` and no `SKIPPED` line, last line `all checks reject their bad change`. Case 20 was rejected in the container (`test result: FAILED. 62 passed; 6 failed`). Afterward `git status --short` is empty, `HEAD` is still `d463c1d` and the temporary copy is gone → `0002 AC-11`
- [x] The fix `3bb3cd2`, probed in a `git clone` in a scratch directory with the binary that `scripts/check.sh all` had built. A line in `README.md` that holds a link to a missing file inside a code span with two backticks on each side → `pool-repo-check docs: ok`, exit 0. The same link outside a code span → exit 1, `D1: README.md:47: link target docs/nope.md does not exist`. The same link after a run of two backticks that is never closed → exit 1 with the same line, so a run with no partner hides nothing. The clone was removed → `0002 AC-8`
- [x] The fix `8cfb720`, read only: line 164 of `scripts/check.sh` on `main` starts its child shell with `CDPATH= cd -- crates/pool-config`. `scripts/check.sh fuzz` was not run again (see "Not run")
- [x] `scripts/check.sh docs` with this section in place → exit 0, `pool-repo-check docs: ok`, so every link and ID written here resolves → `0002 AC-8`

The 337 tests here and the 339 in CI are the same tests: two of the `pool-config` tests exist on Linux only (66 against 68), as in every earlier record.

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-3` | Pull request https://github.com/ojiepermana/pool/pull/1 to `main`. Seven jobs with `success` on the hosted runner `ubuntu-24.04-arm`, in https://github.com/ojiepermana/pool/actions/runs/43606905477 on `6cbc33ec1e38859f39849dea04cf8e720f004203` and in https://github.com/ojiepermana/pool/actions/runs/43607823191 on `5452f9c0e09c3e50dd9647f5a5a2397896189966`. Merged as `d463c1d8a6888e0b1f58a65d42f8449ca7757160`, and the run for that push, https://github.com/ojiepermana/pool/actions/runs/43654217182, is `success` | `PASS` for the runs and for this record. The link from scope 002 is the edit of the lead in remote step 7 |
| `0002 AC-5`, CI half | The `linux-build` log of run 43607823191 on `ubuntu-24.04-arm`: `runner: Linux aarch64`, then `host: aarch64-unknown-linux-gnu`, and `linux build check passed` as the last line of the script. The same in the three other runs | `PASS`. With the script half above, the criterion is met |
| `0002 AC-12` | Pull request https://github.com/ojiepermana/pool/pull/2 holds the format error of case 1. `fmt` is `FAILURE` in https://github.com/ojiepermana/pool/actions/runs/43607256415, GitHub reports `BLOCKED`, and it is `CLOSED` with no merge | `PASS` |
| `0002 AC-13`, remote half | Ruleset 27266107 `main`, `active`, no bypass. The rules on `main` are the four types with the seven check names. Rule suite 5118624627 shows the direct push refused by two rules, and `main` never held the probe commit | `PASS`. With the file half above, the criterion is met |
| `0002 AC-19`, item 12 | Remote step 7 | `NOT_RUN`, `owner: lead` |

### Readings the checker assessed (remote evidence)

The first two readings compare what happened with the text of "Remote proof" at `d463c1d`, the text the steps were run from. The spec owner has since written both cases into the spec (`818179b`, on the local `main` when this record was committed): a note under remote step 1 for a pull request that starts no run, and the review threads under remote step 3 and under "Branch rule and change path".

- **Pull request 1 was closed and opened again.** Step 1 expected the `ci` run to start when the pull request is opened. It did not: the only run for that event is the `Copilot` review, the workflow `ci` is listed as created at 14:49:57, and run number 1 of `ci` starts two seconds after the `reopened` event. `0002 AC-3` asks for seven jobs with `success` on a real pull request to `main` on a hosted runner. It does not name the event. Runs 1 and 3 are `pull_request` runs of that pull request, so the criterion is met. Why the first event started no run cannot be read from GitHub; the lead suspects that the pull request was opened too soon after the first push that carried the workflow (the branch was created at 14:44:14 and the pull request at 14:44:17). It was not a lasting state: pull request 2 started run 2 on `opened`, and the push at 15:00:36 started run 3. So all three ways a pull request starts the workflow have been seen to work.
- **`BLOCKED` in place of `CLEAN` after step 3.** Step 3 expected `CLEAN` and named only a new run of the checks as the answer to `BLOCKED`. The cause here was another one: two open review threads and `required_review_thread_resolution: true`. That value is in `.github/rulesets/main.json` and in the spec block, so the rule did what the spec says. `0002 AC-13` asks for the four rule types, the seven names and a refused push. It does not ask for `CLEAN`. The criterion is met. Both comments were answered with a commit and not waved away, and the seven checks passed again on the new head.
- **The condition before step 1.** The checker did not record the pushed revision first; the record came after the push, and the head changed once more for the review fixes. Review finding r2-m1 already says the sentence cannot be met word for word. What the sentence is for holds: the tree that reached `main` passed `all` and the 28 proof cases on the workstation of the checker, and the seven checks twice on a GitHub runner (runs 3 and 4).
- **`0002 AC-5`, "ends with".** The last line of `scripts/linux-build.sh` in the log is `linux build check passed`. One line follows it, `== ok: linux-build`, which is the line `0002 AC-1` asks `scripts/check.sh` to print. The checker reads the criterion as met.
- **`0002 AC-12`, what the pull request held.** When pull request 2 was open, `main` was still `6de6970`, so its diff against `main` was the whole scope plus the one line. Its head commit adds the one line only, on top of the head that had just passed seven checks, and only `fmt` and `clippy` failed. So the failure comes from that line.

### For the lead

None of these goes against a criterion.

- The repository runs an automatic Copilot review on every pull request (pull requests 1 and 2 both show `review_requested` and a `Copilot` run). Together with `required_review_thread_resolution: true` it means that any pull request on which that review leaves a thread is `BLOCKED` until the thread is resolved. This will also hold for the closing pull request of remote step 7. The spec at `d463c1d` did not describe it, and its remote steps 1 and 3 did not cover what happened; `818179b` adds both, and leaves the choice about the setting open as a follow-up of the spec.
- The live rule has three keys that `.github/rulesets/main.json` does not send (remote step 3 above). `require_extra_approval_for_unattributed_changes: true` is not a value the spec decided. The checker cannot tell from the API whether it is a default.
- `8cfb720` and `3bb3cd2` changed a script and the guard after both review rounds. They are covered here by `all`, the proof, the probe above and two CI runs, and by the Copilot thread that asked for them. No `/check review` has read them.
- The probe commit `2a7d9d2` can still be fetched from GitHub by its hash. It is in no branch and does no harm.

### Not run

- Remote step 7, the closing pull request that carries this record, the scope 002 status and item 12 of "Document corrections": `NOT_RUN`, `owner: lead`.
- Remote step 8, the first daily run: `NOT_RUN`. `gh run list --workflow ci --event schedule --limit 1 --json conclusion,createdAt,url` gave `[]` at 23:44 on 2026-11-14; the first run is due at 03:17 on 2026-11-15. It does not hold the scope open.
- `scripts/check.sh fuzz` and `scripts/check.sh sbom` on `d463c1d`. `check_sbom` did not change since the last record. `check_fuzz` changed by the one `CDPATH` line, which was read and not run.
- Any command of "Remote proof" that writes to GitHub. The checker sent read requests only.
