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

## Milestone 4: Proofs, inventory, documents and workstation evidence

Build plan steps 8 and 9, on 2026-11-14, on macOS (workstation, not Linux and not CI). Commits `6cfef86` (step 9) and `7fb6052` (step 8: the `sbom` version check). Tools: `rustc 1.100.0`, `cargo-fuzz 0.13.2` on `rustc 1.101.0-nightly (5d8fec066 2026-10-16)` (toolchain `nightly-2026-10-17`), `cargo-cyclonedx 0.5.10` installed with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. Steps 7 and 10 add to this section later.

### UI / manual

None for steps 8 and 9. Item 12 of "Document corrections" waits for Build plan step 11 (`NOT_RUN`, `owner: lead`).

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

Not run in these steps: `scripts/prove-checks.sh` (step 7), `scripts/check.sh all` with `guards` and `docs` green and the final tree runs (step 10, after milestone 3 is on `main`).

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0002 AC-15` | The 60 second `scripts/check.sh fuzz` run above, with milestone 1's package half | pass (completes it); step 10 runs it again on the final tree |
| `0002 AC-16` | The `sbom` rows above: one file, CycloneDX 1.5, a license for each package, clean tree, names equal to `lock-packages.txt` | pass |
| `0002 AC-19` | Items 1 to 3, 5 to 11 and 13 applied in `6cfef86`; no old sentence left | pass for those items; item 4 is step 6 (milestone 3), item 12 waits for step 11, `NOT_RUN`, `owner: lead` |
| `0002 AC-14` | `CONTRIBUTING.md` names `scripts/check.sh` and the path to `main` through a pull request whose checks passed | pass (completes the milestone 2 file half) |
| `0002 AC-18` | `docs/plan/risk-register.md` links the [reviewer roster](../../engineering-standards.md#reviewer-roster) | link half pass; the roster rows are not part of steps 8 and 9 |
| `0002 AC-11`, `0002 AC-17` | Steps 7 and 10 | not run |
| Other criteria | Not part of steps 8 and 9 | not run |
