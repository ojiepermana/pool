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

Build plan steps 3 and 4, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `9747ff0` (step 3) and `ba0a046` (step 4). Tools: `rustc 1.100.0`, `cargo-deny 0.20.2`, `cargo-fuzz 0.13.2`, rustup 1.29.2, Docker server 29.9.2.

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
