# 0002. Standards and tooling

**Date**: 2026-11-14
**Status**: In Progress

## Summary

This spec gives the project its required checks: seven named checks that every change must pass before it reaches `main`. Each check is one command you can run on your own machine, and the same command runs in CI (the service that runs the checks for every pull request). The spec also fixes how a change reaches `main`, which tools watch dependencies, licenses and security advisories, and who may review what.

Nothing here adds proxy behavior. It makes the rules that are already written true for the workspace that already exists, and it makes a broken rule fail by itself.

## Requirements

These criteria cover [scope 002](../../scope/002-standards-and-tooling.md). Cite them as `0002 AC-n`. Exact file content, commands and rules are in [Checks and tooling](#checks-and-tooling).

**User stories**:

- As a contributor, I want one command per check, so that I can reproduce a failed CI check on my own machine.
- As a maintainer, I want `main` to accept only changes whose checks passed, so that a broken build, a new license or a known advisory cannot arrive unseen.
- As a reviewer, I want each check proven against a deliberately wrong change, so that I can rely on a green result.
- As the project lead, I want every review role to have a named holder or a visible gap, so that a gate is never approved by the person who made the change.

**Acceptance criteria**:

- **AC-1** (one entry point): `scripts/check.sh <name>` runs each check named under [The check entry point](#the-check-entry-point). On the committed tree, each of `fmt`, `clippy`, `test`, `deny`, `guards` and `docs` exits 0 and ends with the line `== ok: <name>`. `scripts/check.sh all` runs those six in that order, keeps going after a failure, and exits 0 only when all six passed. An unknown name exits 2 with the usage text on stderr. A missing tool or a stopped Docker daemon exits 3 with one line that says what is missing.
- **AC-2** (CI definition): `.github/workflows/ci.yml` has exactly the seven jobs `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards` and `docs`. Every job runs on `ubuntu-24.04-arm` and its project command is `scripts/check.sh <job id>`; the job `deny` runs `scripts/check.sh tools` before it. The workflow runs on a pull request, on a push to `main` and once a day. It asks for read access to the repository content only, uses no secret, and names each action by a full commit hash.
- **AC-3** (the checks run in CI): On a real pull request to `main`, all seven jobs finish with the result `success` on a GitHub hosted runner. The run address and the commit hash are recorded in `verify.md` and linked from scope 002.
- **AC-4** (dependencies, licenses and advisories, `SEC-13`, `R-16`, `R-17`): `deny.toml` has the content under [Dependency policy](#dependency-policy). `scripts/check.sh deny` requires `cargo-deny` 0.20.2 and runs its four checks (advisories, bans, licenses, sources) on the product lockfile and on the fuzz lockfile. Both pass on the committed tree. The allowed licenses are exactly `Apache-2.0`, `MIT` and `Unicode-3.0`, plus one exception, `NCSA` for `libfuzzer-sys`. No advisory is ignored.
- **AC-5** (Linux build without emulation, spec 0001 Follow-up 9): `scripts/linux-build.sh` runs the [Linux build](../0001-initial-architecture/index.md#linux-build) command of spec 0001, copied without a change, and rule W7 fails when the two differ. The CI job `linux-build` runs on a hosted runner that has the architecture of the Linux target spec 0001 names. Its log shows `runner: Linux aarch64`, then the `host:` line that `0001 AC-2` asks for, and ends with `linux build check passed`. A run on a workstation is how you reproduce the job; it is not this evidence.
- **AC-6** (workspace guards, spec 0001 Follow-up 11, `R-16`): `pool-repo-check workspace` enforces the rules W1 to W7 under [Workspace guards](#workspace-guards): the package set of each lockfile, the paths that must not exist, the source patterns that must not appear, lint inheritance, the allowed dependencies between workspace packages, and the copy of the Linux build command. It passes on the committed tree. Each rule has at least one test that feeds it a failing input.
- **AC-7** (CI definition guard): `pool-repo-check ci` enforces the rules C1 to C5 under [CI definition guards](#ci-definition-guards). It passes on the committed tree, and each rule has a failing test.
- **AC-8** (links and anchors): `pool-repo-check docs` fails when a local link in a Markdown file points to a file that does not exist, or to a heading that the target file does not have. It passes on the committed tree.
- **AC-9** (scope status agreement): `pool-repo-check docs` fails when a row of the scope index and the first line of its feature file disagree on number, name or status, when a status is not one of `planned`, `in-progress`, `done` or `dropped`, or when a feature file has no row.
- **AC-10** (stable requirement IDs): `pool-repo-check docs` fails when an ID is defined twice, when a cited ID has no definition, and, given `--base <revision>`, when an ID that the base revision defines is gone.
- **AC-11** (each check catches a wrong change, locally): `scripts/prove-checks.sh` applies every case under [Proving the checks](#proving-the-checks) to a private copy of the committed tree. For each case the named check exits with a status other than 0 and prints the named text. The script ends with `all checks reject their bad change` and leaves `git status --short` empty.
- **AC-12** (a failed check blocks a merge, on the remote): A pull request that holds the deliberate format error of case 1 shows the check `fmt` as failed, and GitHub reports the pull request as blocked. It is closed without a merge and its address is recorded in `verify.md`.
- **AC-13** (branch rule): `.github/rulesets/main.json` has the content under [Branch rule and change path](#branch-rule-and-change-path). On the remote, the rules that apply to `main` are `deletion`, `non_fast_forward`, `pull_request` and `required_status_checks` with the seven check names, and a direct push to `main` is refused.
- **AC-14** (pull request path): `.github/pull_request_template.md` has the content under [Pull request template](#pull-request-template). `CONTRIBUTING.md` describes the path a change takes to `main` and names `scripts/check.sh`.
- **AC-15** (fuzz harness): The package `crates/pool-config/fuzz/` has the content under [Fuzz harness](#fuzz-harness). `scripts/check.sh fuzz` runs the target `validate` for 60 seconds with `cargo-fuzz` 0.13.2 on the toolchain `nightly-2026-10-17` and ends with exit 0 and no crash file. The run is recorded in `verify.md`.
- **AC-16** (inventory, `SEC-13`): `scripts/check.sh sbom` writes `target/sbom/pool.cdx.json`, a CycloneDX document that names the third party packages `pool` is built from, with a license for each, and leaves `git status --short` empty. One run is recorded in `verify.md`, with its component names compared to `lock-packages.txt` and every difference explained.
- **AC-17** (choices recorded): The [technology evaluation](../../research/technology-evaluation.md#adopted-in-scope-002) names every tool this spec adopts, with version, purpose, license, native code, cost and alternative. After the build it holds the locked versions that the developer observed. The [testing strategy](../../testing/strategy.md#ci-progression) names the seven checks, the fuzz rule and how PostgreSQL 18 is supplied to integration tests.
- **AC-18** (reviewer roster, `R-25`): The [reviewer roster](../../engineering-standards.md#reviewer-roster) has one row for each review named in the standards and for each owner role in the risk register. Each row names its holder, or says `Not assigned`, and says whether a fresh model may do that review. The risk register links to it.
- **AC-19** (documents match the workspace): Each sentence listed under [Document corrections](#document-corrections) is replaced by the text given there. The file each item names no longer contains the old sentence. Item 12 is the one exception: it waits for the remote evidence.
- **AC-20** (lint rules that were only written down): The root `Cargo.toml` denies the Clippy lints `dbg_macro`, `print_stderr`, `print_stdout`, `todo` and `unimplemented` for the whole workspace, and `scripts/check.sh clippy` passes.

| Criterion | Source it answers |
|---|---|
| AC-1 | Scope 002: "a path to reproduce failures locally" |
| AC-2, AC-3 | Scope 002 "Done when": format, lint, test and dependency checks run in CI. "The maintainer uses the real manifest to select required CI checks". The CI service choice |
| AC-4 | The dependency, license and advisory checker choice. `SEC-13`, `R-16`, `R-17` |
| AC-5 | Spec 0001 Follow-up 9 |
| AC-6 | Spec 0001 Follow-up 11. `R-16` (a change to the locked packages is always visible). The repository structure rule on dependencies between crates |
| AC-7, AC-13, AC-14 | "branch review rules". Engineering standards, Git and releases |
| AC-8, AC-9, AC-10 | "documentation checks for local links/anchors, scope status agreement and stable requirement IDs" |
| AC-11, AC-12 | "Verify the checks detect a deliberately failing change before relying on them" |
| AC-15 | The fuzz harness choice |
| AC-16 | The SBOM generator choice. `SEC-13` |
| AC-17 | "Record the choices" and "Add each adopted tool to the technology evaluation". How PostgreSQL 18 is supplied |
| AC-18 | "record the reviewer roster". `R-25` |
| AC-19, AC-20 | "Done when": documented conventions match the workspace. "Reconcile the planning rules with the actual scaffold" |

AC-3, AC-12, the remote half of AC-13 and the CI half of AC-5 need a real pull request on GitHub. No build session writes to GitHub. Those four are collected in the last Build plan step, which the lead runs with the permission you gave for this scope on 2026-11-14, using the exact commands under [Remote proof](#remote-proof). Everything else is proven on the workstation.

The scope box "Capture conventions: `/audit`" belongs to the audit session and is already ticked on `main`. It has no criterion here.

## Decision

**Chosen option**: One shell entry point, `scripts/check.sh`, that both people and CI call, with seven required checks on GitHub Actions and a branch rule that lets a change reach `main` only through a pull request whose checks passed. (basis: scope 002, engineering standards "Git and releases", spec 0001 "Change path")

The parts of that decision:

- **CI service**: GitHub Actions on the hosted runner `ubuntu-24.04-arm`. It is a 64 bit ARM machine, the same architecture as the Linux target of spec 0001 (your VMs are ARM), so the Linux build runs without emulation. All seven checks run there. No check builds for 64 bit Intel or AMD. (basis: the repository is on GitHub with Actions on; spec 0001 Follow-up 9)
- **Dependency, license and advisory checker**: `cargo-deny` 0.20.2, one tool for all three. (basis: `SEC-13`, `R-16`, `R-17`)
- **Repository checks**: a small Rust program, `pool-repo-check`, in `tools/repo-check/`. It has no dependencies, sits outside the Cargo workspace, and holds the workspace guards, the CI definition guards and the document checks. (basis: spec 0001 Follow-up 11; the scope 001 criteria on workspace members and locked packages stay true)
- **SBOM generator** (an SBOM is a machine readable list of every component in a build): `cargo-cyclonedx` 0.5.10, CycloneDX JSON. It is a command you run, not a required check, until the release work of scope 011. (basis: `SEC-13`)
- **Fuzz harness** (fuzzing feeds a program random input to find crashes): `cargo-fuzz` 0.13.2 with `libfuzzer-sys` 0.4.13 on the toolchain `nightly-2026-10-17`, one target today for `pool_config::validate`. It is not a required check yet. (basis: `R-12`, testing strategy "Fuzzing")
- **PostgreSQL 18 for integration tests**: the official `postgres` container image, Debian variant, pinned by digest, started by the same script on a workstation and in CI. Nothing is built for it now, because no code talks to a database yet. (basis: testing strategy "Database integration")
- **Change path**: from this scope on, nothing is pushed to `main` directly. The coordination flow pushes its merged result as a branch and opens one pull request per scope. (basis: spec 0001 "Change path")
- **Reviews**: a fresh model (a model session that did not write the change and starts without the author's conversation) may do the independent review of a slice. Decisions that need authority over real infrastructure, licenses, risk or the remote stay with a named person.

## Checks and tooling

Where this section names a value, use it as written. Where it leaves an internal detail open (function names, module layout, the exact wording of a message that no criterion quotes), you may choose.

### What this build is and where it stops

You add checks, their configuration and their documents. You add one small Rust program outside the workspace and one fuzz package beside `pool-config`.

Not in this build: any change to what `pool` does, any new workspace member, any new dependency of `pool` or `pool-config`, `rustfmt.toml`, `clippy.toml`, a database, a fuzz job in CI, an SBOM job in CI, a release, a dependency update bot. The four paths that spec 0001 forbids stay absent.

New paths, and no others:

```text
deny.toml
scripts/check.sh
scripts/linux-build.sh
scripts/prove-checks.sh
tools/repo-check/Cargo.toml
tools/repo-check/Cargo.lock
tools/repo-check/src/main.rs        (plus modules of your choice under tools/repo-check/src/)
tools/repo-check/baseline/lock-packages.txt
tools/repo-check/baseline/fuzz-lock-packages.txt
tools/repo-check/baseline/tool-lock-packages.txt
tools/repo-check/baseline/forbidden-paths.txt
tools/repo-check/baseline/forbidden-source-patterns.txt
tools/repo-check/baseline/internal-dependencies.txt
crates/pool-config/fuzz/Cargo.toml
crates/pool-config/fuzz/Cargo.lock
crates/pool-config/fuzz/.gitignore
crates/pool-config/fuzz/fuzz_targets/validate.rs
.github/workflows/ci.yml
.github/rulesets/main.json
.github/pull_request_template.md
docs/specs/0002-standards-and-tooling/verify.md
```

The three scripts under `scripts/` are committed with the executable bit set.

### Required checks

A required check is a check that must pass on a pull request before it can be merged. The name in the first column is the name everywhere: the argument of `scripts/check.sh`, the job id in the workflow and the check name in the branch rule.

| Check | What it runs | What it proves |
|---|---|---|
| `fmt` | `cargo fmt --all --check`, then `cargo fmt --check --manifest-path tools/repo-check/Cargo.toml`, then `cargo fmt --check --manifest-path crates/pool-config/fuzz/Cargo.toml` | Formatting, with the defaults of the pinned toolchain |
| `clippy` | `cargo clippy --workspace --all-targets --locked -- -D warnings`, then `cargo clippy --manifest-path tools/repo-check/Cargo.toml --all-targets --locked -- -D warnings` | Lints, with warnings as errors |
| `test` | `cargo test --workspace --locked`, then `cargo test --manifest-path tools/repo-check/Cargo.toml --locked` | The test suites of the product and of the repository checks |
| `deny` | `cargo deny --locked --config deny.toml check`, then `cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check` | Advisories, licenses, banned and duplicate packages, package sources |
| `linux-build` | `scripts/linux-build.sh` | The Linux build check of spec 0001 |
| `guards` | `pool-repo-check workspace`, then `pool-repo-check ci` | The workspace guards and the CI definition guards |
| `docs` | `pool-repo-check docs`, with `--base` when a base revision is known | Links, anchors, scope status and requirement IDs |

`pool-repo-check <args>` above stands for `cargo run -q --locked --manifest-path tools/repo-check/Cargo.toml --target-dir target/repo-check -- <args>`.

Every `cargo clippy`, `cargo test` and `cargo run` on `tools/repo-check/Cargo.toml` passes `--target-dir target/repo-check`. The package is its own workspace root, so without that option it would build into `tools/repo-check/target/`, which `.gitignore` does not cover, and the tree would no longer be clean.

The product run of `deny` prints one warning, `license-exception-not-encountered`, because the `NCSA` exception is used only by the fuzz lockfile. That warning is expected and does not fail the check.

The fuzz package is formatted and its dependencies are checked, but no required check compiles it. That is a known gap until the first slice that adds a fuzz job (see [Rules for later slices](#rules-for-later-slices)).

### The check entry point

`scripts/check.sh` is a POSIX shell script (`#!/bin/sh`, `set -eu`). It may be called from any directory; it first changes to the repository root, which is the parent of its own directory.

```text
scripts/check.sh <name> [<name>...]
```

| Name | Meaning |
|---|---|
| `fmt`, `clippy`, `test`, `deny`, `linux-build`, `guards`, `docs` | The required checks above |
| `all` | `fmt`, `clippy`, `test`, `deny`, `guards`, `docs`, in that order. It does not stop at a failure. `linux-build` is not part of `all`, because it needs Docker and a clean tree |
| `tools` | Makes `cargo-deny` 0.20.2 available (see below) |
| `fuzz` | The fuzz run under [Fuzz harness](#fuzz-harness). Not a required check |
| `sbom` | The inventory under [Software inventory](#software-inventory). Not a required check |

Rules:

- For each name the script prints `== check: <name>` first. After the commands of that check it prints `== ok: <name>` or `== FAILED: <name>`. With more than one name, or with `all`, it runs every name and then prints one line per name again as a summary.
- Exit status follows the [shared convention](../../engineering-standards.md#shared-conventions): 0 when every named check passed, 1 when a check failed, 2 for a usage error (no name, or a name not in the table; usage text on stderr, nothing run), 3 when a check could not run (a tool is missing or has the wrong version, the Docker daemon is not running, the tree has uncommitted changes where a clean tree is required). If both 1 and 3 occur in one call, the status is 1.
- How the status of a command maps to that: exit 3 from `pool-repo-check` or from `scripts/linux-build.sh` counts as could not run. A failed `cargo install` in `tools` counts as could not run. Any other status that is not 0, from any command of a check, counts as a failed check.
- The script sets `CARGO_INCREMENTAL=0` and `CARGO_PROFILE_DEV_DEBUG=0` unless they are already set, and puts `target/tools/bin` at the front of `PATH`.
- Pinned tool versions are written once, as variables at the top of this script: `cargo-deny` 0.20.2, `cargo-fuzz` 0.13.2, `cargo-cyclonedx` 0.5.10 and the fuzz toolchain `nightly-2026-10-17`.
- `deny` first compares the output of `cargo deny --version` with `cargo-deny 0.20.2`. If the tool is missing or the version differs, it prints `cargo-deny 0.20.2 is required; run scripts/check.sh tools` and counts as could not run.
- `tools` does nothing when `cargo deny --version` already prints `cargo-deny 0.20.2`. Otherwise it runs `cargo install --locked --root target/tools cargo-deny@0.20.2`. It never installs anything outside `target/tools`.
- `docs` picks the base revision this way: the value of `POOL_CHECK_BASE` if it is set, not empty and not forty zeros (GitHub sends forty zeros when a push has no earlier commit); otherwise `origin/main` if `git rev-parse --verify -q origin/main` succeeds; otherwise none. With a base it passes `--base <revision>`.
- `fuzz` and `sbom` check their tool the same way as `deny` and name the install command in their message: `cargo install --locked --root target/tools cargo-fuzz@0.13.2` and `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. A missing toolchain `nightly-2026-10-17` is reported with the command `rustup toolchain install nightly-2026-10-17`; the script does not install toolchains.

To reproduce a failed CI job named `X`, check out the same commit and run `scripts/check.sh X`. That is the whole rule.

### Linux build script

`scripts/linux-build.sh` is the executable copy of the command under [Linux build](../0001-initial-architecture/index.md#linux-build) in spec 0001. Spec 0001 owns that command: its platform, its image digest and everything the inner script checks. This spec does not repeat any of them.

The script has two parts. The first part is fixed here:

```sh
#!/bin/sh
# Linux build check of spec 0001 (0001 AC-2). Needs a running Docker daemon.
# The command below the marker line is copied from spec 0001, "Linux build". Do not edit it here.
set -eu
CDPATH= cd -- "$(dirname -- "$0")/.."
echo "runner: $(uname -sm)"
if ! docker info >/dev/null 2>&1; then
  echo "linux-build: the Docker daemon is not running" >&2
  exit 3
fi
if ! git rev-parse --verify -q HEAD >/dev/null; then
  echo "linux-build: git cannot read HEAD here; this check builds only the committed tree" >&2
  exit 3
fi
if ! changes=$(git status --porcelain); then
  echo "linux-build: git status failed" >&2
  exit 3
fi
if [ -n "$changes" ]; then
  echo "linux-build: commit or remove local changes first; this check builds only the committed tree" >&2
  exit 3
fi
# --- spec 0001, Linux build ---
```

Three things in this part are there on purpose. `CDPATH=` in front of `cd` keeps an exported `CDPATH` from sending the script to another directory. `git rev-parse --verify -q HEAD` is a statement of its own, so a `git` that cannot work here (no repository, no commit, an ownership refusal) ends the check with exit 3 before the copied command pipes an empty archive into the container. The status of `git status` is tested apart from its output, so a failing `git status` is not read as a clean tree.

The second part is everything after the marker line `# --- spec 0001, Linux build ---`. It is the content of the first code block under the heading "Linux build" in `docs/specs/0001-initial-architecture/index.md` (the block that starts with `git archive`), byte for byte, and nothing else. Copy it from spec 0001 as it stands on `main` when you build this step. Rule W7 compares the two on every pull request, so a later change to that command in spec 0001 must change this script in the same pull request.

The line `runner:` shows where the check ran: `Linux aarch64` on the CI runner, `Darwin arm64` on the workstation of this project. On that workstation Docker runs ARM Linux containers without emulation, so the local run takes about as long as the CI run.

The runner and the Linux target must have the same architecture. If spec 0001 ever names a different Linux target, the `runs-on` label of this spec changes with it, in this spec.

### Dependency policy

`deny.toml` at the repository root has exactly this content:

```toml
[graph]
all-features = true

[advisories]
db-urls = ["https://github.com/rustsec/advisory-db"]
yanked = "deny"
unmaintained = "all"
unsound = "all"
ignore = []

[licenses]
allow = ["Apache-2.0", "MIT", "Unicode-3.0"]
confidence-threshold = 0.8
unused-allowed-license = "deny"

[[licenses.exceptions]]
crate = "libfuzzer-sys"
allow = ["NCSA"]

[licenses.private]
ignore = false

[bans]
multiple-versions = "deny"
wildcards = "deny"
allow-wildcard-paths = true

[sources]
unknown-registry = "deny"
unknown-git = "deny"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
allow-git = []
```

What it means, in plain words:

- A known vulnerability, a withdrawn (yanked) version, a package reported as unmaintained or a package reported as unsound (its safe interface can cause memory errors) fails the check, for direct and indirect dependencies alike. `unsound = "all"` is written out because the default of `cargo-deny` 0.20.2 looks at direct dependencies only.
- Every package must offer at least one license from the list. The two workspace packages are checked too. A license in the list that no package needs also fails, so the list cannot grow quietly.
- Two versions of one package in a lockfile fail. A dependency without a version requirement fails, except a path dependency inside this repository.
- Packages come from crates.io only. A dependency taken from a Git address or another registry fails.

Rules for changing it:

- Adding a license, a license exception or an entry to `ignore` is a decision of the Release maintainer (licenses) or the Maintainer (advisories), who must be a person (see the [reviewer roster](../../engineering-standards.md#reviewer-roster)). The pull request that makes the change records the reason in the technology evaluation. An `ignore` entry also records the advisory ID and a date by which it is looked at again.
- A dependency update is its own pull request. It changes `Cargo.toml` or `Cargo.lock`, the matching baseline file if the set of package names changed, and the technology evaluation if a direct dependency changed. No update bot is configured. New advisories are found by the daily run of the workflow.

### Lint rules

Add this table to the root `Cargo.toml`, after `[workspace.lints.rust]`. Change nothing else in that file. Spec 0001 lists the content of that file as the contract of the first build; this table is the one addition since, and no criterion of spec 0001 depends on its absence.

```toml
[workspace.lints.clippy]
dbg_macro = "deny"
print_stderr = "deny"
print_stdout = "deny"
todo = "deny"
unimplemented = "deny"
```

`print_stdout` and `print_stderr` turn the output rule of spec 0001 (never use `print!`, `println!`, `eprint!` or `eprintln!`) into a lint. The other three keep unfinished code out. The current code passes all five in every target, tests included, so no source file changes.

`unwrap()`, `expect()` and `panic!` are not denied by a lint. The standards ask for a documented invariant and a review where they appear in a production hot path, and the tests use `expect` on purpose. They stay a review item.

The project has no `rustfmt.toml` and no `clippy.toml`. Formatting uses the defaults of the pinned toolchain. Lint levels live in the lint tables of the root `Cargo.toml`.

### The repository check program

`tools/repo-check/Cargo.toml`:

```toml
[package]
name = "pool-repo-check"
version = "0.0.0"
edition = "2024"
rust-version = "1.100.0"
license = "Apache-2.0"
publish = false

[workspace]

[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
dbg_macro = "deny"
todo = "deny"
unimplemented = "deny"
```

The empty `[workspace]` table makes the package its own workspace root, so `cargo metadata` at the repository root still lists exactly `pool` and `pool-config`, and the root `Cargo.lock` does not change. The package has no dependencies and no build script. Its `Cargo.lock` is committed and holds one package.

```text
pool-repo-check workspace [--root <DIR>]
pool-repo-check ci        [--root <DIR>]
pool-repo-check docs      [--root <DIR>] [--base <REVISION>]
```

- `--root` is the repository root to check. The default is the current directory. Tests use it to point at small trees they build under `env!("CARGO_TARGET_TMPDIR")` or at trees held in memory.
- Each problem is one line on stdout: `<rule>: <path>[:<line>]: <message>`, where `<rule>` is the rule ID from the tables below (`W1`, `C2`, `D3` and so on). The last line is `pool-repo-check <command>: ok` or `pool-repo-check <command>: <n> problems`.
- Exit status: 0 no problem, 1 at least one problem, 2 usage error (message on stderr), 3 the check could not run (a file it must read is missing or unreadable, `git` failed for `--base`).
- The program reads files and, for `--base` only, runs `git`. It writes nothing and opens no socket.
- This is a developer tool that reads the repository's own files, so its messages may quote a path, a link target or an ID. The rule of spec 0001 that output never repeats input is for `pool`, which reads operator files.
- Tests use the built in `cargo test` harness. Write at least one passing and one failing case for every rule ID.

#### Workspace guards

| Rule | What must hold | Baseline file |
|---|---|---|
| W1 | For each of the three lockfiles, the names of its `[[package]]` entries are exactly the lines of its baseline file | `lock-packages.txt` for `Cargo.lock`, `fuzz-lock-packages.txt` for `crates/pool-config/fuzz/Cargo.lock`, `tool-lock-packages.txt` for `tools/repo-check/Cargo.lock` |
| W2 | No path listed in the baseline exists | `forbidden-paths.txt` |
| W3 | No `.rs` file under `apps/` or `crates/` contains a listed text. Directories named `target` are skipped | `forbidden-source-patterns.txt` |
| W4 | The root `Cargo.toml` has `unsafe_code = "forbid"` in `[workspace.lints.rust]`, and the manifest of every workspace member has a `[lints]` table with `workspace = true` | none |
| W5 | The dependencies between workspace members are exactly the listed pairs | `internal-dependencies.txt` |
| W6 | Every file named `Cargo.lock` in the repository is one of the three lockfiles of W1. Directories named `target` and directories whose name starts with a dot are skipped | none |
| W7 | The text of `scripts/linux-build.sh` after its marker line equals the first code block under the heading `### Linux build` in `docs/specs/0001-initial-architecture/index.md` | none |

Baseline files live in `tools/repo-check/baseline/`. A line that is empty or starts with `#` is ignored. Each file starts with a comment line that names this spec.

```text
--- lock-packages.txt
itoa
memchr
pool
pool-config
proc-macro2
quote
serde
serde_core
serde_derive
serde_json
serde_spanned
syn
toml
toml_datetime
toml_parser
unicode-ident
winnow
zmij

--- fuzz-lock-packages.txt
arbitrary
cc
cfg-if
find-msvc-tools
getrandom
jobserver
libc
libfuzzer-sys
pool-config
pool-config-fuzz
proc-macro2
quote
r-efi
serde
serde_core
serde_derive
serde_spanned
shlex
syn
toml
toml_datetime
toml_parser
unicode-ident
winnow

--- tool-lock-packages.txt
pool-repo-check

--- forbidden-paths.txt
apps/pool-cli
crates/pool-protocol
crates/pool-core
crates/pool-control

--- forbidden-source-patterns.txt
std::net
TcpListener
TcpStream
UdpSocket
UnixListener
UnixStream
tokio
std::thread

--- internal-dependencies.txt
pool -> pool-config
```

Details:

- W1 reads a lockfile line by line: a `name = "<x>"` line that follows a `[[package]]` line is a package name. Names are compared as a sorted list, so a package that appears twice must be listed twice. The message names each missing and each extra package. If Cargo resolves a different set for the fuzz lockfile than the one above, stop and report it to the lead; do not edit the baseline to fit.
- W3 compares plain text, not patterns. The eight lines are the alternatives of the `grep` in `0001 AC-12`. A match in a comment counts, as it does for that `grep`.
- W4 and W5 read manifests line by line, without a TOML parser. A `#` comment that is not inside quotes is cut from a line before the line is read, so `[dependencies] # runtime` is still a table header and `workspace = true # inherit` is still that setting. Workspace members are the quoted strings in the `members` array of the root `Cargo.toml`; a `*` in a member string is a problem of rule W4, because the guard cannot follow it. A member's package name is the `name` in its `[package]` table. A dependency is a key in a table whose name is `dependencies`, `dev-dependencies` or `build-dependencies`, or ends with a dot and one of those names, or a table whose name ends with `<one of those>.<key>`, with or without a prefix in front (for example `[target.'cfg(unix)'.dependencies.pool]`). The key is the text before the first dot, space or equals sign, with quotes around it removed. When the line or the table of a dependency holds `package = "<name>"`, that name is the package it stands for, not the key. Only dependencies that stand for a member package count for W5.
- W7 finds the heading line `### Linux build` in spec 0001, takes the lines between the first fence that follows it and that fence's closing line, and compares them with the lines of the script after the line `# --- spec 0001, Linux build ---`. Both sides are compared line by line, ignoring one trailing newline at the end of the file. A missing marker line or a missing block is a problem.
- A baseline file changes only in the pull request that carries the spec or Build plan step that calls for the change, and the pull request names that spec. The first slice that opens a socket removes lines from `forbidden-source-patterns.txt`; the slice that creates `crates/pool-protocol` removes that line from `forbidden-paths.txt` and adds its pairs to `internal-dependencies.txt`.

#### CI definition guards

These rules read `.github/workflows/ci.yml` and `.github/rulesets/main.json` line by line. Both files are written in the fixed shape given in this spec, so no YAML or JSON parser is needed.

| Rule | What must hold |
|---|---|
| C1 | In every file under `.github/workflows/`, each `uses:` value is `<owner>/<name>@<40 hexadecimal characters>` followed by a comment that starts with `# v` |
| C2 | The job ids of `ci.yml` (lines of the form two spaces, an id, a colon, anywhere after the `jobs:` line) are exactly the `"context"` values of the `required_status_checks` rule in `main.json` |
| C3 | After the `jobs:` line, `ci.yml` holds the fixed shape of this spec and nothing else. (a) Every line that is not empty is a job id line, or a line whose key is on the allowed list below. (b) Every `run:` value is exactly `rustup toolchain install` or `scripts/check.sh <name>`, where `<name>` is a job id of the file or `tools`. (c) A `run:` step is one line: the next line that is not empty starts a new step or a new job. (d) The last `run:` of each job is `scripts/check.sh <its job id>`. This is what keeps CI free of logic of its own |
| C4 | Every job has exactly one `runs-on:` line, and its value is `ubuntu-24.04-arm` |
| C5 | `.github/workflows/` holds `ci.yml` and no other file. `ci.yml` has exactly one line whose key is `permissions`. That line is `permissions:` with nothing after it, at the start of a line, and the block under it is the one line `  contents: read`: every line from there to the next line that starts in the first column is that line or is empty. No workflow file contains the text `pull_request_target` or `secrets` |

The allowed keys of rule C3, taken from the workflow in this spec:

```text
runs-on
timeout-minutes
steps
uses
with
persist-credentials
fetch-depth
path
key
run
```

Details:

- The key of a line is found this way: drop the leading spaces, drop one leading `- ` if there is one, and take the text before the first colon. It is compared as written. So `"run"` in quotes, `{run` in braces and `run ` with a space before the colon are not the key `run`, and each is a problem of C3. A line with no colon has no key and is a problem. A line that holds only a comment is a problem too, because the fixed shape has none after `jobs:`.
- A job id line is two spaces, an id made of letters, digits, hyphens and underscores, a colon, and nothing else. It is the same line that rule C2 reads.
- Part (a) is what reports `continue-on-error:`, `if:`, `shell:`, `env:`, `container:`, `name:`, `strategy:`, `permissions:` and every other key that can change whether or how a step runs. None of them is on the list.
- For part (b), the value is the text after `run:` with the spaces around it removed. It must equal one of the allowed texts in full, so `scripts/check.sh fmt || true` and `scripts/check.sh tools;<anything>` are problems.
- For part (c), a new step is a line with the same indent as the `run:` line that begins with `- `. A new job is a job id line. Anything else after a `run:` line is a problem, also when an empty line stands between them. That covers a line that YAML would join to the command, and a key added to the step.
- A later slice that needs another key, or a second workflow file, adds it to this rule in its own spec, in the same pull request.

What these rules still do not read is listed under [Consequences](#consequences).

#### Document checks

The document checks read every `*.md` file under the root. They skip directories whose name starts with a dot (except `.github`), and directories named `target`, `graphify-out` or `node_modules`. They read the working tree, so a Markdown file that is not committed yet is checked too.

Lines inside a fenced code block are never read as links, headings or IDs. A fence opens with a line that starts with three or more backticks, or three or more tildes, after optional spaces. It closes with a line of the same character that is at least as long and has nothing else on it. A shorter fence inside a longer one is content.

| Rule | What must hold |
|---|---|
| D1 | The path of every local link exists |
| D2 | The fragment of every local link to a Markdown file is a heading anchor of that file |
| D3 | The scope index and the feature files agree |
| D4 | No ID is defined twice |
| D5 | Every cited ID is defined |
| D6 | With `--base`: every ID that the base revision defines is still defined |

**Links (D1, D2).** A link is `[text](target)` or `![text](target)`, with an optional title in double quotes after the target, or a reference definition line `[label]: target`. Text between backticks on one line is removed before links are looked for. A run of one or more backticks opens such a span, and the next run of the same length on that line closes it. So a span written with two backticks on each side is removed whole, with any single backtick inside it. A run that has no partner of the same length stays as plain text. A target that starts with a scheme (a letter, then letters, digits, `+`, `.` or `-`, then a colon), such as `https:` or `mailto:`, is not checked; the checks never use the network. Angle brackets around a target are dropped and `%20` is read as a space. The target is split at the first `#` into path and fragment. An empty path means the same file. A path that starts with `/` is a problem (use a relative path). Otherwise the path is resolved from the directory of the file that holds the link and must exist, as a file or a directory, inside the root.

A fragment is checked only when the target is a `.md` file. The anchors of a file come from its headings, lines of one to six `#` characters and a space, outside code blocks. Take the heading text, replace each link `[text](target)` by its text, make it lowercase, drop every character that is not a letter, a digit, a space, a hyphen or an underscore (letters and digits of any script count), and turn each space into a hyphen. A second heading with the same result gets `-1` added, a third `-2`. For example, `## Where a change belongs` gives `where-a-change-belongs` and `## docs/prd` gives `docsprd`.

**Scope status (D3).** In `docs/scope/index.md`, a feature row is a table row of the form `| <n> | [<name>](<file>) | <phase> | <status> |`. For each row: `<file>` exists in `docs/scope/`, its name starts with `<n>` written with three digits and a hyphen, its first line is `# <n>. <name> · <status>`, optionally followed by one or more tags, each written as ` · <tag>`, and `<status>` is `planned`, `in-progress`, `done` or `dropped`. The tags are `needs a decision` and `deferred`. A tag is not a status and the index row does not carry it. Fourteen of the seventeen feature files carry `needs a decision` today, so write a passing test for a heading with a tag. A feature file is a direct child of `docs/scope/` whose name is three digits, a hyphen, then lowercase letters, digits and hyphens, then `.md`. Every feature file has exactly one row, and no number appears twice.

**Requirement IDs (D4, D5, D6).** These are the citation forms the [documentation index](../../index.md#where-a-change-belongs) names.

| Family | Citation | Where it is defined |
|---|---|---|
| Security requirement | `SEC-<digits>` | A table row of `docs/security/security-requirements.md` whose first cell is the ID |
| Risk | `R-<digits>` | The same, in `docs/plan/risk-register.md` |
| Inventory row | `F-<digits>` | The same, in `docs/feature-matrix.md` |
| Threat | `T-<digits>` | The same, in `docs/security/threat-model.md` |
| PRD criterion | `PRD-<3 digits> AC-<digits>` | A list item that starts with `**AC-<digits>` in `docs/prd/<3 digits>-*.md` |
| PRD functional requirement | `PRD-<3 digits> FR-<digits>` | Item number `<digits>` of the numbered list under `## Functional requirements` in that file |
| Spec criterion | `<4 digits> AC-<digits>` | A list item that starts with `**AC-<digits>` in `docs/specs/<4 digits>-*/index.md`, or in `docs/specs/<4 digits>-*.md` for a spec that is one file |
| Spec safety contract | `<4 digits> SC-<digits>` | Item number `<digits>` of the numbered list under `## Safety contracts` in that file |

- A citation is any place the citation form appears in a checked file outside a fenced code block, text between backticks included, because that is how the documents cite. The form must start at a word boundary, so `RUSTSEC-0000-0001` is not a citation of a security requirement. IDs are compared as text, so the same number written with and without a leading zero is two different IDs.
- D4: within its owner (the file for the first four families, the PRD or spec number for the others) an ID is defined once.
- D5: every citation has a definition. A citation of a PRD or spec that does not exist is a problem. A bare `AC-3` with no document number in front is not a citation and is not checked, because a reader of that file knows which document it means.
- D6: the program reads the same owner files at the base revision with `git ls-tree -r --name-only <revision>` and `git show <revision>:<path>`. Every ID defined there must be defined now, in the same family and under the same PRD or spec number. For the two positional families an item's number is the number written at the start of the list item, and the list must not be shorter than it was. A requirement that is withdrawn keeps its row or item and says so in its text; its ID is never removed or used again.

#### What each message must name

The wording of a message is yours, but a message must contain the rule ID and the items below, written exactly as shown, because `scripts/prove-checks.sh` looks for them.

| Rule | The message contains |
|---|---|
| W1 | The lockfile path and the package name that is missing from the baseline or from the lockfile |
| W2 | The forbidden path as written in the baseline, for example `crates/pool-core` |
| W3 | The file, the line number and the forbidden text as written in the baseline, for example `std::thread` |
| W4 | The manifest path |
| W5 | The pair as `<from> -> <to>`, for example `pool-config -> pool`, and whether it is not allowed or is missing |
| W6 | The path of the unexpected `Cargo.lock` |
| W7 | `scripts/linux-build.sh` and the first line number that differs |
| C1, C2, C4, C5 | The workflow or ruleset path and, for C2, the job id. For a second workflow file (C5), the path of that file |
| C3 | The workflow path with the line number, and the job id. For a key that is not allowed, also the key as written, for example `continue-on-error` |
| D1 | The file, the line number and the link target as written, for example `docs/missing-file.md` |
| D2 | The file, the line number and the fragment as written, for example `no-such-heading` |
| D3 | The feature file, and which of number, name or status disagrees |
| D4, D5, D6 | The ID as cited or defined, for example `SEC-14`, and for D4 and D5 the file and line number |

### Fuzz harness

One fuzz target exists today, for the only code that parses bytes it does not trust. Fuzz targets live inside the package they test, as the [repository structure](../../repository-structure.md#merge-or-defer-the-candidate-crates) asks.

`crates/pool-config/fuzz/Cargo.toml`:

```toml
[package]
name = "pool-config-fuzz"
version = "0.0.0"
edition = "2024"
license = "Apache-2.0"
publish = false

[package.metadata]
cargo-fuzz = true

[workspace]

[dependencies]
libfuzzer-sys = "=0.4.13"
pool-config = { path = ".." }

[[bin]]
name = "validate"
path = "fuzz_targets/validate.rs"
test = false
doc = false
bench = false
```

`crates/pool-config/fuzz/fuzz_targets/validate.rs`:

```rust
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    match pool_config::validate(data) {
        Ok(config) => assert_eq!(
            config.config_version(),
            pool_config::SUPPORTED_CONFIG_VERSION
        ),
        Err(rejection) => assert!(!rejection.findings().is_empty()),
    }
});
```

`crates/pool-config/fuzz/.gitignore` has the four lines `target`, `corpus`, `artifacts` and `coverage`.

Write the manifest by hand. Do not use `cargo fuzz init`: on this workspace it writes a wrong package name and leaves out the `[workspace]` table. The package has no lint table, because the `fuzz_target!` macro needs an unsafe attribute. That is the one place in the repository where the `unsafe_code` rule does not apply, and the package is never a dependency of anything.

`scripts/check.sh fuzz` runs, from `crates/pool-config`, with `CARGO_TARGET_DIR` set to `<root>/target/fuzz`:

```bash
cargo +nightly-2026-10-17 fuzz run validate fuzz/corpus/validate tests/fixtures -- -max_total_time=60 -max_len=4096
```

The script changes to that directory inside a child shell, and writes `CDPATH=` in front of that `cd` as it does in front of its first one. An exported `CDPATH` can then not send the fuzz run to another tree while the crash file test looks at this one.

It creates `fuzz/corpus/validate` first if it is missing. The fixture directory is a second, read only seed corpus. `POOL_FUZZ_SECONDS` replaces the 60. Exit 0 with no file under `fuzz/artifacts/validate/` is a pass. A crash file is a finding to report with its bytes, not to delete.

### Software inventory

`scripts/check.sh sbom` writes one file, `target/sbom/pool.cdx.json`: a CycloneDX JSON document, specification version 1.5, for the package `pool` with all of its dependencies, direct and indirect.

`cargo-cyclonedx` writes its output next to the manifest it describes, and may write one file for each workspace member. The script moves the file for `pool` into `target/sbom/`, removes any other file the tool wrote, and leaves the tree clean. Start from this command, and adjust the flags if version 0.5.10 behaves differently; the result above is the contract, the flags are not:

```bash
cargo cyclonedx --manifest-path apps/pool/Cargo.toml --format json --spec-version 1.5 --all --override-filename pool.cdx
```

Record the exact command you ended with in `verify.md`. If the tool cannot produce one clean file, stop and report it; the runner up, `cargo-sbom`, writes to stdout.

This inventory is evidence for `SEC-13` at the size the project has today. The release inventory, with license texts and notices for what is shipped, belongs to scope 011.

### CI definition

`.github/workflows/ci.yml`:

```yaml
name: ci

on:
  pull_request:
  push:
    branches: [main]
  schedule:
    - cron: "17 3 * * *"

permissions:
  contents: read

concurrency:
  group: ci-${{ github.event_name }}-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

env:
  CARGO_TERM_COLOR: never
  POOL_CHECK_BASE: ${{ github.event.pull_request.base.sha || github.event.before || '' }}

jobs:
  fmt:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh fmt

  clippy:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh clippy

  test:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh test

  deny:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - uses: actions/cache@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0
        with:
          path: target/tools
          key: tools-${{ runner.os }}-${{ runner.arch }}-${{ hashFiles('scripts/check.sh') }}
      - run: scripts/check.sh tools
      - run: scripts/check.sh deny

  linux-build:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: scripts/check.sh linux-build

  guards:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh guards

  docs:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
          fetch-depth: 0
      - run: rustup toolchain install
      - run: scripts/check.sh docs
```

Notes:

- `ubuntu-24.04-arm` is a standard GitHub hosted runner, free for a public repository, with 64 bit ARM processors. Its image list of 2026-11-11 names Docker 28.0.4 and rustup 1.29.3.
- `rustup toolchain install` with no name installs the toolchain that `rust-toolchain.toml` selects, with its components. The version is written in that one file only. This form was tried with rustup 1.29.2 on the workstation and is not yet seen on the runner. If the runner refuses it, report it; do not write the version into the workflow as a quick fix.
- No job has a `name:` key, so the check name GitHub reports is the job id.
- The workflow needs no secret and gets a read only token, so a pull request from a fork runs the same checks with nothing to steal.
- `POOL_CHECK_BASE` is a commit hash supplied by GitHub (the base of the pull request, or the commit before a push). On the daily run it is empty and `docs` falls back to `origin/main`. The `docs` job fetches the full history so that the base commit is present.
- The cache holds only `target/tools`, which is the `cargo-deny` binary. Its key changes when `scripts/check.sh` changes, which is where the version is pinned.
- The daily run at 03:17 UTC is what finds an advisory published against a package that nobody touched. GitHub turns a schedule off after 60 days without activity in a public repository; a person turns it on again from the Actions page.
- The two action hashes were read from the GitHub API on 2026-11-14: `actions/checkout` tag `v7.0.1` and `actions/cache` tag `v6.1.0`. Changing an action version is a dependency update like any other.

### Branch rule and change path

`.github/rulesets/main.json` is the rule for `main`, kept in the repository so that the remote setting can be applied and compared from a file:

```json
{
  "name": "main",
  "target": "branch",
  "enforcement": "active",
  "bypass_actors": [],
  "conditions": {
    "ref_name": {
      "include": ["~DEFAULT_BRANCH"],
      "exclude": []
    }
  },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" },
    {
      "type": "pull_request",
      "parameters": {
        "allowed_merge_methods": ["merge"],
        "dismiss_stale_reviews_on_push": true,
        "require_code_owner_review": false,
        "require_last_push_approval": false,
        "required_approving_review_count": 0,
        "required_review_thread_resolution": true
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "strict_required_status_checks_policy": true,
        "required_status_checks": [
          { "context": "fmt", "integration_id": 15368 },
          { "context": "clippy", "integration_id": 15368 },
          { "context": "test", "integration_id": 15368 },
          { "context": "deny", "integration_id": 15368 },
          { "context": "linux-build", "integration_id": 15368 },
          { "context": "guards", "integration_id": 15368 },
          { "context": "docs", "integration_id": 15368 }
        ]
      }
    }
  ]
}
```

`integration_id` 15368 is the GitHub Actions app (read with `gh api /apps/github-actions` on 2026-11-14). With it, only a check reported by Actions can satisfy a required check; a status with the same name from somewhere else cannot.

What it does: `main` cannot be deleted or rewritten. A change reaches it only through a pull request. The pull request must be up to date with `main` and all seven checks must have passed on it. Only a merge commit is allowed, so the commits of each Build plan step stay visible. Nobody is on the bypass list, the repository owner included.

One limit of "merge commit only": it binds the merge button and the merge API. It may not bind a direct `git push` of the head commit of a pull request that is open, up to date and green, as long as the number of required approvals is 0. GitHub can take such a push as that pull request being merged, and `main` then moves forward without a merge commit. The content would still be content that passed all seven checks. Nobody should push that way, and the [remote proof](#remote-proof) is written so that it never tries.

The number of required approvals is 0, because GitHub does not let the author of a pull request approve it and the project has one GitHub account today. The independent review that `SEC-14` asks for is done as a written review under `docs/reviews/`, linked from the pull request. When a second maintainer account exists, raise `required_approving_review_count` to 1 in this file and on the remote in one pull request.

`required_review_thread_resolution` is `true`. An open review thread (a comment on a line of the change, with its replies) therefore holds the merge, also when all seven checks have passed. The first real case came on 2026-11-14. The account that owns the repository has the automatic Copilot code review switched on, and it reviews every pull request without being asked. It left two threads on the scope pull request, and the merge state stayed `BLOCKED` until both were marked as resolved. Three things are worth knowing:

- A thread still holds the merge after the line it points at was fixed. GitHub then shows it as outdated, which is not the same as resolved.
- The automatic reviewer marked one of its two threads as resolved by itself after the fix. The other one had to be marked by a person.
- Marking a thread as resolved changes public content on GitHub. It needs the repository owner's permission, like every other write there.

Both findings were real defects, and both were fixed before the merge. Whether the setting stays is Follow-up 12.

**How a change reaches `main` from now on:**

1. Work happens on a branch. In the coordination flow each session still commits on its own branch `claude/<name>-<hash>`, and the lead still merges those branches locally with `--no-ff` after the checks pass on the merged tree.
2. The lead no longer pushes `main`. The merged result is pushed as a branch named `scope/<number>-<slug>`, for example with `git push origin main:scope/003-control-and-session-data-model`, and a pull request to `main` is opened from it with `gh pr create`. A pull request for a smaller change uses any branch name. Writing to GitHub needs your permission; the lead asks for it per scope, and without it you run these two commands yourself.
3. The seven checks run. A failed check is reproduced with `scripts/check.sh <name>`, fixed on the branch and pushed again.
4. Review threads are settled. If the automatic reviewer or a person left a thread on the pull request, each finding is judged first: is it a real defect or not. A real one is fixed by a developer session on its own branch, merged locally and pushed to the pull request branch like the fix of a failed check, and the seven checks run again. Then each thread is marked as resolved, by the repository owner, or by the lead with the owner's permission. A finding that is judged not to be a defect is marked the same way; the lead says why when asking for that permission. The command that lists the threads is under remote step 3 of the [remote proof](#remote-proof).
5. The pull request is merged with a merge commit, and the local `main` is moved forward with `git pull --ff-only`.
6. Whatever a scope can record only after its pull request ran (run addresses, the status `done`, the closing log entry) goes into a second pull request from a branch named `scope/<number>-closing`, in the same way. Remote step 7 of the [remote proof](#remote-proof) is the first one. This takes the place of the push of `main` that used to close a scope.

If the rule itself must change, or must be switched off in an emergency, the repository owner does that in the repository settings. The change is then made in `main.json` in the next pull request, and the reason is written in the changelog. The ruleset history on GitHub shows every such change.

### Pull request template

`.github/pull_request_template.md`:

```markdown
## What and why

<!-- The problem, and the scope feature and spec this belongs to. -->

## Acceptance criteria

<!-- The criteria this change meets, by ID, for example 0002 AC-4. -->

## Evidence

<!-- Commands you ran and their results, or a link to the verification record. Say what was not run. -->

- [ ] `scripts/check.sh all` passes on this branch

## Safety and review

<!-- Does this touch protocol, parser, authentication, pooling, cancellation, failover or a dependency? -->
<!-- Name the reviewer and link the review record under docs/reviews/, or say why none is needed. -->

## Baseline and policy changes

<!-- List any change to deny.toml or tools/repo-check/baseline/ and the spec that calls for it. Write "none" otherwise. -->
```

### Proving the checks

`scripts/prove-checks.sh` shows that each check fails when it should. It refuses to run with uncommitted changes (exit 3). It makes one private copy of the committed tree: `git clone` of the repository into a temporary directory, then `git checkout --detach <commit>`, where `<commit>` is the `HEAD` it was started from. It applies one case, runs the check inside the copy, restores the copy (`git reset --hard <commit>` and `git clean -fdq`), and goes on to the next case. It sets `POOL_CHECK_BASE=<commit>` for every run in the copy, so the base revision never depends on what the clone calls `origin/main`. It shares one build directory, `<root>/target/prove`, through `CARGO_TARGET_DIR`, and puts `<root>/target/tools/bin` on `PATH` so that the copy finds the same tools. It never changes the working tree it was started from. Where a case needs a commit in the copy, it commits with a throwaway identity given on the command line (`git -c user.name=prove -c user.email=prove@example.invalid commit`); the restore step removes that commit again.

For each case the check must exit with a status other than 0 and its output must contain the text in the last column. The script prints `PASS <case> <check> rejected the change` or `FAIL <case> ...`, and ends with `all checks reject their bad change` (exit 0) or with the list of failed cases (exit 1). Before the cases it runs `scripts/check.sh all` in the copy and requires exit 0, so that a failure in a case can only come from the change.

| Case | Check | Change made in the copy | Output must contain |
|---|---|---|---|
| 1 | `fmt` | Append line A to `apps/pool/src/main.rs` | `Diff in` |
| 2 | `clippy` | Append line B to `crates/pool-config/src/lib.rs` | `ptr_arg` |
| 3 | `clippy` | Append line C to `crates/pool-config/src/lib.rs` | `print_stdout` |
| 4 | `test` | Replace the content of `crates/pool-config/tests/fixtures/valid-minimal.toml` with line D | `test result: FAILED` |
| 5 | `deny` | In the root `Cargo.toml`, replace `license = "Apache-2.0"` with `license = "GPL-3.0-only"` | `license is not explicitly allowed` |
| 6 | `deny` | Add a made up advisory for `itoa` to the advisory database (see below) | `RUSTSEC-0000-0001` |
| 7 | `deny` | Add line E under `[dependencies]` in `apps/pool/Cargo.toml` and line F under `[workspace.dependencies]` in the root `Cargo.toml`, without updating `Cargo.lock` | `cannot update the lock file` |
| 8 | `guards` | Delete the line `zmij` from `tools/repo-check/baseline/lock-packages.txt` | `W1` and `zmij` |
| 9 | `guards` | Create the file `crates/pool-core/.keep` | `W2` and `crates/pool-core` |
| 10 | `guards` | Append line G to `apps/pool/src/main.rs` | `W3` and `std::thread` |
| 11 | `guards` | Delete the `[lints]` table from `apps/pool/Cargo.toml` | `W4` |
| 12 | `guards` | Add line H under `[dependencies]` in `crates/pool-config/Cargo.toml` | `W5` and `pool-config -> pool` |
| 13 | `guards` | In `.github/workflows/ci.yml`, replace the first hash after `actions/checkout@` with `v7` | `C1` |
| 14 | `guards` | In `.github/workflows/ci.yml`, rename the job id `docs:` to `documents:` | `C2` |
| 15 | `docs` | Append line I to `README.md` | `D1` and `docs/missing-file.md` |
| 16 | `docs` | Append line J to `README.md` | `D2` and `no-such-heading` |
| 17 | `docs` | In the first line of `docs/scope/003-control-and-session-data-model.md`, replace `planned` with `done` | `D3` |
| 18 | `docs` | Append line K to `README.md` | `D5` |
| 19 | `docs` | Delete the table row of `SEC-14` from `docs/security/security-requirements.md` | `D6` and `SEC-14` |
| 20 | `linux-build` | Apply case 4 and commit it in the copy | `unexpected exit` or `test result: FAILED` |
| 21 | `guards` | In `scripts/linux-build.sh`, replace the text `linux build check passed` with `linux build check skipped` | `W7` |
| 22 | `deny` | Add a dependency that comes from a Git address (see below) | `source-not-allowed` |
| 23 | `deny` | In the root `Cargo.toml`, replace the line that starts with `serde_json = ` with line N | `error[wildcard]` |
| 24 | `guards` | In `.github/workflows/ci.yml`, replace the first line `    timeout-minutes: 15` with `    continue-on-error: true` | `C3` and `continue-on-error` |
| 25 | `guards` | In `.github/workflows/ci.yml`, replace the first `ubuntu-24.04-arm` with `ubuntu-latest` | `C4` |
| 26 | `guards` | In `.github/workflows/ci.yml`, replace the line `  contents: read` with `  contents: write` | `C5` |
| 27 | `guards` | Create the empty file `docs/Cargo.lock` | `W6` and `docs/Cargo.lock` |
| 28 | `docs` | Append line L to `docs/security/security-requirements.md` | `D4` and `SEC-14` |

```text
A  fn  prove_fmt( ){}
B  pub fn prove_clippy(v: &Vec<u8>) -> usize { v.len() }
C  pub fn prove_print() { println!("x"); }
D  config_version = 2
E  itoa.workspace = true
F  itoa = "1"
G  // std::thread
H  pool = { path = "../../apps/pool" }
I  [broken](docs/missing-file.md)
J  [broken](docs/glossary.md#no-such-heading)
K  `SEC-99`
L  | SEC-14 | Fixture row used to prove the duplicate rule |
M  pool-prove-source = { git = "file://<temporary directory>/prove-source" }
N  serde_json = "*"
```

Case 6, step by step: in the copy, add the line `db-path = "<temporary directory>/advisory-db"` under `[advisories]` in `deny.toml`. Run `scripts/check.sh deny` once; it downloads the real database into that directory and must pass. Then create the file `crates/itoa/RUSTSEC-0000-0001.md` inside the one directory that the download created there (its name starts with `advisory-db-`), with the content below, and run `scripts/check.sh deny` again. The made up advisory is never committed and never leaves the temporary directory.

````text
```toml
[advisory]
id = "RUSTSEC-0000-0001"
package = "itoa"
date = "2026-11-14"
url = "https://example.invalid/pool-fixture"
categories = ["memory-corruption"]

[versions]
patched = []
```

# Fixture advisory used to prove the advisory check

This advisory is not real. It exists only so the check can be shown to fail.
````

Case 22, step by step. It shows that the policy as committed refuses a package that does not come from crates.io.

1. In the temporary directory, create the folder `prove-source` with an empty `src/lib.rs` and this `Cargo.toml`, run `git init` there and commit both files with the throwaway identity:

   ```toml
   [package]
   name = "pool-prove-source"
   version = "0.0.0"
   edition = "2024"
   license = "Apache-2.0"
   ```

2. In the copy, add line M under `[dependencies]` in `crates/pool-config/Cargo.toml`, with the real path of the temporary directory.
3. In the copy, run `cargo metadata --format-version 1` once, without `--locked`, and drop its output. Cargo clones the folder and adds the package to `Cargo.lock`, so the lockfile is current and the check does not stop at the stale lockfile of case 7.
4. Run `scripts/check.sh deny`. The sources check fails with `source-not-allowed`.

Case 23 works because a `*` requirement still matches the locked version, so the lockfile stays current and only the bans check fails.

Case 20 runs only when the script is started with `--with-linux-build` and the Docker daemon is running. Without it the script prints `SKIPPED 20 linux-build` and ends with `all checks reject their bad change (linux-build not proven)`. That wording keeps a skipped case visible.

Cases 6 and 7, and the first `deny` run, need the network. Everything else runs offline once the crates are downloaded. Cases 22 and 23 contact no host: case 22 clones from a folder on the same machine.

Four parts of the dependency policy still have no case: two versions of one package, another registry, a withdrawn version and an unmaintained package. Each would need a lockfile entry written by hand or one more made up advisory, and was left out to keep the proof short. They are listed under [Consequences](#consequences).

What a local proof cannot show is that GitHub runs the checks and refuses a merge. That is the [remote proof](#remote-proof), the last Build plan step.

### Remote proof

These steps are the only ones that write to GitHub. The lead runs them from the main checkout, in this order, after every branch of this scope is merged into the local `main`, and after `scripts/check.sh all` and `scripts/prove-checks.sh` have both passed on exactly the revision that step 1 pushes. The checker records that revision and its test counts in `verify.md` first. No build, check or test session runs these steps. `gh` must be signed in as the repository owner. Each step names the criterion it proves and what its output must show; the checker copies the addresses into `verify.md`.

The order is part of the design. The workflow runs on a GitHub runner for the first time in step 1, while `main` has no rule yet. A surprise on the new runner is then fixed on the scope branch, with nothing locked. The rule is applied in step 3, only after seven green checks, and every step after it runs with the rule active.

How the blocks are written:

- Each block stands on its own. It sets every variable it uses, so you can run it in a fresh shell (`sh`, `bash` or `zsh`).
- A variable in front of a colon is written `${NAME}`, because zsh reads `$NAME:r` as something else.
- A block that waits for a run can take longer than ten minutes. Run it in the background if your shell has a time limit.
- The wait loop asks GitHub every five seconds, for at most three minutes, whether a run for the commit exists. It is there because `gh pr checks --watch` gives up with "no checks reported" when it is asked before GitHub has registered the run. If the loop ends with no run, follow the note under remote step 1.
- Remote steps 1, 5 and 7 push or commit from the main checkout. Before each of those blocks, `git status --short` must print nothing and `git rev-parse --abbrev-ref HEAD` must print `main`. The blocks do not test this themselves. A changed tracked file would go into the commit of step 5, because `git commit -am` takes every changed tracked file, and a `HEAD` that is not `main` means the tree you checked is not the one you push. Step 6 pulls into the current branch, so it needs `HEAD` on `main` too.

**Remote step 1: push the scope branch and open the pull request** (**AC-3**).

```bash
git push origin main:scope/002-standards-and-tooling
gh pr create --base main --head scope/002-standards-and-tooling \
  --title "Scope 002: standards and tooling" \
  --body "Spec 0002. Required checks, branch rule and tooling. Evidence: docs/specs/0002-standards-and-tooling/verify.md"
SHA=$(git rev-parse main)
RUN=; n=0
until [ -n "$RUN" ] || [ "$n" -ge 36 ]; do
  sleep 5; n=$((n + 1))
  RUN=$(gh run list --commit "$SHA" --event pull_request --limit 1 --json databaseId --jq '.[0].databaseId // empty')
done
gh run watch "${RUN:?no run was registered within three minutes}" --exit-status
gh pr checks scope/002-standards-and-tooling
gh pr view scope/002-standards-and-tooling --json url,headRefOid --jq '"\(.url) \(.headRefOid)"'
gh run view "$RUN" --json url,conclusion,headSha --jq '"\(.url) \(.conclusion) \(.headSha)"'
```

Expected: `gh run watch` ends with exit 0. `gh pr checks` prints seven lines marked `pass`, one for each check name. The last line holds the run address, the word `success` and the hash that `git rev-parse main` prints.

If a check fails, this is a finding for a developer session, not something to fix on GitHub: read it with `gh run view <id> --log-failed`, reproduce it with `scripts/check.sh <name>`, fix it on a session branch and merge locally. Then run the block again without the `gh pr create` command. Do not go on to step 3 before this step shows seven passes.

If the block ends with `no run was registered within three minutes`, the pull request exists but GitHub started no `ci` run for it. This happened on 2026-11-14. `gh pr create` ran about one second after the first push that brought `.github/workflows/ci.yml` to GitHub, and the `opened` event started only the automatic Copilot review, not `ci`. The likely cause is that the pull request was opened before GitHub had taken in the new workflow file. That is a guess, not a proven fact. The pull request of step 5, opened ten minutes later, got its run at once.

Look first, with commands that only read:

```bash
R=$(gh repo view --json nameWithOwner -q .nameWithOwner)
gh run list --commit "$(git rev-parse main)" --json databaseId,event,workflowName,status --jq '.[] | "\(.databaseId) \(.event) \(.workflowName) \(.status)"'
gh api "repos/$R/contents/.github/workflows/ci.yml?ref=scope/002-standards-and-tooling" --jq .path
gh pr view scope/002-standards-and-tooling --json state,mergeable,headRefOid --jq '"\(.state) \(.mergeable) \(.headRefOid)"'
```

What the three commands tell you:

1. Each line is one run for the commit: its number, the event, the workflow name and the state. A line with the event `dynamic` is the automatic review and does not count. If a line with `pull_request ci` is there, the run was only late: run the step 1 block again without `gh pr create`.
2. `.github/workflows/ci.yml`. If the file is not on the branch, the push did not bring it, and that is the thing to fix.
3. `OPEN MERGEABLE` and the hash that `git rev-parse main` prints. `UNKNOWN` in the middle means GitHub has not worked it out yet; ask again after a few seconds. With `CONFLICTING`, GitHub starts no pull request run at all, and the branch must first be brought up to date with `main`.

If no `ci` run exists, the file is there and the pull request is open and mergeable, close the pull request and open it again:

```bash
PR=$(gh pr view scope/002-standards-and-tooling --json number --jq .number)
gh pr close "$PR"
gh pr reopen "$PR"
```

The `reopened` event starts the workflow; on 2026-11-14 the run was registered two seconds later. Then run the step 1 block again without `gh pr create`. Closing and opening again is a write to GitHub of the same kind as step 1. It keeps the head commit, so the run is for exactly the revision that was checked on the workstation. An extra commit would also start a run, but for a revision that nobody checked.

**Remote step 2: read the Linux build lines** (**AC-5**).

```bash
RUN=$(gh run list --commit "$(git rev-parse main)" --event pull_request --limit 1 --json databaseId --jq '.[0].databaseId')
gh run view "$RUN" --log | grep -E 'runner: |host: |target/release/pool|linux build check passed'
```

Expected: the line `runner: Linux aarch64`, the `host:` line that `0001 AC-2` asks for, the `sha256sum` line of `target/release/pool`, and `linux build check passed`.

**Remote step 3: apply the branch rule** (**AC-13**). Run it only after step 1 showed seven passes.

```bash
R=$(gh repo view --json nameWithOwner -q .nameWithOwner)
gh api "repos/$R/rulesets" --jq 'length'
[ "$(gh api "repos/$R/rulesets" --jq 'length')" = 0 ] &&
  gh api -X POST "repos/$R/rulesets" --input .github/rulesets/main.json --jq '"\(.id) \(.name) \(.enforcement)"'
gh api "repos/$R/rules/branches/main" --jq '[.[].type] | sort | join(",")'
gh api "repos/$R/rules/branches/main" --jq '[.[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context] | sort | join(",")'
gh pr view scope/002-standards-and-tooling --json mergeStateStatus --jq .mergeStateStatus
```

Expected, line by line:

1. `0`. No ruleset exists yet. With any other number the rule is not sent; stop and look at what is there.
2. A number, then `main active`.
3. `deletion,non_fast_forward,pull_request,required_status_checks`.
4. `clippy,deny,docs,fmt,guards,linux-build,test`.
5. `CLEAN`, when the pull request has no open review thread. The seven checks that passed before the rule existed count for it, because GitHub looks at the results on the head commit. `UNKNOWN` means GitHub has not worked it out yet; ask again after a few seconds. If it stays `BLOCKED`, look for open review threads first (see below). Only when no thread is open, run the checks once more with `gh run rerun <run id>` and wait for them. That is a write to GitHub of the same kind as step 1.

On 2026-11-14 the fifth line was `BLOCKED`, with all seven checks at `SUCCESS`. The cause was not the checks. The rule also asks that every review thread is resolved, and the automatic Copilot review had left two open threads (see [Branch rule and change path](#branch-rule-and-change-path)). This lists the threads of the pull request. It is a query and changes nothing:

```bash
R=$(gh repo view --json nameWithOwner -q .nameWithOwner)
PR=$(gh pr view scope/002-standards-and-tooling --json number --jq .number)
gh api graphql -F owner="${R%/*}" -F name="${R#*/}" -F number="$PR" \
  -f query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){pullRequest(number:$number){reviewThreads(first:100){nodes{isResolved isOutdated path comments(first:1){nodes{author{login}}}}}}}}' \
  --jq '.data.repository.pullRequest.reviewThreads.nodes[] | "\(if .isResolved then "resolved" else "open" end) outdated=\(.isOutdated) \(.path) \(.comments.nodes[0].author.login)"'
```

Each line is one thread: `open` or `resolved`, whether GitHub calls it outdated, the file, and who started it. A line that starts with `open` holds the merge, also when it says `outdated=true`. Settle the threads as step 4 of "How a change reaches `main`" says. A `BLOCKED` state for this reason does not hold up steps 4 and 5 of this proof, which do not read the merge state. Step 6 does: it starts only when the state is `CLEAN`.

**Remote step 4: a direct push is refused** (**AC-13**). The block has its own gate: it pushes only if GitHub reports exactly the four rule types and the seven names of step 3.

```bash
R=$(gh repo view --json nameWithOwner -q .nameWithOwner)
TYPES=$(gh api "repos/$R/rules/branches/main" --jq '[.[].type] | sort | join(",")')
NAMES=$(gh api "repos/$R/rules/branches/main" --jq '[.[] | select(.type=="required_status_checks") | .parameters.required_status_checks[].context] | sort | join(",")')
if [ "$TYPES" = "deletion,non_fast_forward,pull_request,required_status_checks" ] &&
   [ "$NAMES" = "clippy,deny,docs,fmt,guards,linux-build,test" ]; then
  git fetch origin main
  PROBE=$(git commit-tree 'origin/main^{tree}' -p origin/main \
    -m "probe: a direct push that the branch rule must refuse (spec 0002 AC-13)")
  echo "probe ${PROBE} on top of $(git rev-parse origin/main)"
  git ls-remote origin refs/heads/main
  git push origin "${PROBE}:refs/heads/main" || echo "push refused, exit $?"
  git ls-remote origin refs/heads/main
else
  echo "the rule on main is not the one remote step 3 applies; the probe was not run"
fi
```

What is pushed is a new commit with no change in it, made on top of the remote `main`. It belongs to no pull request and no check ever ran on it, so an active rule has two reasons to refuse it. It is not the head of the scope pull request. That head is the one commit GitHub may accept on a direct push (see [Branch rule and change path](#branch-rule-and-change-path)), which is why the probe does not use it. The local `main` and the working tree are not touched; the probe commit stays in the local repository as an object that nothing points to.

Expected: the push is rejected with a message that contains `GH013` or `rule violations`, the block prints `push refused`, and both `git ls-remote` lines show the same hash.

If the second `git ls-remote` line shows the probe hash, the push went through and **AC-13** has failed. Record the output. What landed is one empty commit. To take it off again, with the hash from the `probe` line:

```bash
PROBE=<the hash from the probe line>
BEFORE=$(git rev-parse "${PROBE}^")
git push --force-with-lease="refs/heads/main:${PROBE}" origin "${BEFORE}:refs/heads/main"
git ls-remote origin refs/heads/main
```

This is a forced push to `main`, so ask the repository owner before you run it. The lease makes it safe: GitHub moves `main` back only if `main` is still the probe commit. If GitHub refuses this push too, leave the empty commit where it is, bring it into the local `main` with `git fetch origin main` and `git merge --no-edit origin/main`, and run the step 1 block again without `gh pr create`, so that the scope branch is up to date. In both cases stop here and report to the repository owner before step 5, because the rule does not do what this spec says.

**Remote step 5: a wrong change is blocked** (**AC-12**). Run it before step 6.

```bash
git switch -c prove/fmt-fails main
printf 'fn  prove_fmt( ){}\n' >> apps/pool/src/main.rs
git commit -am "test: deliberate format error, do not merge"
git push origin prove/fmt-fails
gh pr create --base main --head prove/fmt-fails \
  --title "Do not merge: proves the fmt check" \
  --body "Deliberate format error for spec 0002 AC-12. This pull request is closed without a merge."
SHA=$(git rev-parse prove/fmt-fails)
RUN=; n=0
until [ -n "$RUN" ] || [ "$n" -ge 36 ]; do
  sleep 5; n=$((n + 1))
  RUN=$(gh run list --commit "$SHA" --event pull_request --limit 1 --json databaseId --jq '.[0].databaseId // empty')
done
gh run watch "${RUN:?no run was registered within three minutes}" --exit-status || echo "run failed, exit $?"
gh pr checks prove/fmt-fails || echo "checks failed, exit $?"
gh pr view prove/fmt-fails --json url,mergeStateStatus,statusCheckRollup \
  --jq '"\(.url) \(.mergeStateStatus) fmt=\([.statusCheckRollup[] | select(.name=="fmt") | .conclusion] | join(","))"'
git switch main
gh pr close prove/fmt-fails --delete-branch
git branch --list prove/fmt-fails
git ls-remote origin refs/heads/prove/fmt-fails
git status -sb | head -1
```

Expected: the block prints `run failed` and `checks failed`, which is the point. `gh pr checks` shows `fmt` as `fail`. It shows `clippy` as `fail` too, because the added function is never used; that does no harm. `gh pr view` prints the address, then `BLOCKED fmt=FAILURE`. The two commands after `gh pr close` print nothing: the branch is gone locally and on the remote. If the first of them still prints the branch, remove it with `git branch -D prove/fmt-fails`. Afterward `git status --short` is empty and the local `main` is unchanged.

The commit of this step is made without the graph update that `AGENTS.md` asks for before a commit. It is exempt because it is never merged.

**Remote step 6: merge** (**AC-3**).

```bash
gh pr view scope/002-standards-and-tooling --json mergeStateStatus --jq .mergeStateStatus
gh pr merge scope/002-standards-and-tooling --merge
git pull --ff-only origin main
git status -sb | head -1
SHA=$(git rev-parse origin/main)
RUN=; n=0
until [ -n "$RUN" ] || [ "$n" -ge 36 ]; do
  sleep 5; n=$((n + 1))
  RUN=$(gh run list --commit "$SHA" --event push --limit 1 --json databaseId --jq '.[0].databaseId // empty')
done
gh run watch "${RUN:?no run was registered within three minutes}" --exit-status
gh pr view scope/002-standards-and-tooling --json state,mergeCommit --jq '"\(.state) \(.mergeCommit.oid)"'
gh run view "$RUN" --json event,conclusion,url,headSha --jq '"\(.event) \(.conclusion) \(.url) \(.headSha)"'
```

Expected: the first line is `CLEAN`. After the merge, `git status -sb` shows `## main...origin/main` with nothing ahead or behind. The run for the push to `main` is watched to its end before it is read. The last two lines are `MERGED` with the hash of the merge commit, and `push success` with the run address and the same hash.

The branch `scope/002-standards-and-tooling` stays on the remote, because the repository does not delete a branch when its pull request is merged. Nothing depends on it. You may delete it on GitHub at any time.

**Remote step 7: the closing pull request** (**AC-3**, **AC-19**). The addresses and hashes of steps 1 to 6 exist only now, and with the rule active they can reach `main` only through a second pull request. First the closing commits are brought onto the local `main`, which step 6 left equal to `origin/main`. Together they hold:

- the pull request addresses, run addresses and commit hashes of steps 1 to 6 in `verify.md`, written by the checker, and the four remote items there no longer `NOT_RUN`
- the status of scope 002 in its index row and in the first line of its feature file, with the links to that evidence, and the status line of this spec (`Accepted`)
- item 12 of [Document corrections](#document-corrections)
- the closing entry of the coordination log, and the sync step's edits to `AGENTS.md` if they are not on `main` yet
- the graph update in `graphify-out/`

Then:

```bash
git status -sb | head -1
git push origin main:scope/002-closing
gh pr create --base main --head scope/002-closing \
  --title "Scope 002: closing evidence" \
  --body "Remote evidence for spec 0002 (AC-3, AC-5, AC-12, AC-13), scope 002 status, and item 12 of Document corrections."
SHA=$(git rev-parse main)
RUN=; n=0
until [ -n "$RUN" ] || [ "$n" -ge 36 ]; do
  sleep 5; n=$((n + 1))
  RUN=$(gh run list --commit "$SHA" --event pull_request --limit 1 --json databaseId --jq '.[0].databaseId // empty')
done
gh run watch "${RUN:?no run was registered within three minutes}" --exit-status
gh pr checks scope/002-closing
gh pr merge scope/002-closing --merge
git pull --ff-only origin main
git status -sb | head -1
```

Expected: the first line shows the local `main` ahead of `origin/main` by the closing commits and not behind. `gh pr checks` prints seven lines marked `pass`. After the merge, `git status -sb` shows `## main...origin/main` with nothing ahead or behind.

Two notes from steps 1 and 3 apply here too. If the block ends with `no run was registered within three minutes`, follow the note under remote step 1 with the branch name `scope/002-closing`. It is less likely here, because GitHub knows the workflow by now, but the cause in step 1 was never proven. And the automatic review may leave a thread on this pull request as well. `gh pr merge` then refuses. List the threads with the block under remote step 3 (with this branch name), settle them, and run the last three commands of the block again.

The branch name follows the pattern `scope/<number>-closing`.

For this scope and for every later one, this step takes the place of step 9 of the coordination skill ("Push `main`"). The closing commits of a scope reach `main` through a pull request and never through a push to `main`. A later scope whose evidence does not depend on its own pull request can put its closing commits into the scope pull request itself and then needs no second one.

**Remote step 8: the first daily run.** A day later, `gh run list --workflow ci --event schedule --limit 1 --json conclusion,createdAt,url` shows one run with `success`. This does not hold the scope open.

To undo the rule:

```bash
R=$(gh repo view --json nameWithOwner -q .nameWithOwner)
gh api "repos/$R/rulesets" --jq '.[] | select(.name=="main") | .id'
gh api -X DELETE "repos/$R/rulesets/<number>"
```

The second command prints the number of the rule and the third removes it. Removing the rule does not take back anything else. These steps also leave on the remote: the merge commits of steps 6 and 7 on `main`, which stay; the branches `scope/002-standards-and-tooling` and `scope/002-closing`; and three pull requests, two merged and one closed. The branch `prove/fmt-fails` is gone after step 5.

### Rules for later slices

These are decisions with nothing to build today. The slice named in each rule builds it and cites this section.

**PostgreSQL 18 for integration tests** (first used by [scope 004](../../scope/004-one-real-primary-path.md)):

- The database is the official `postgres` container image, Debian variant, with a tag of the form `18.<minor>-bookworm`, pinned by its index digest. The tag and digest are written in one script and recorded in the technology evaluation when the slice adopts them. On 2026-11-14 the newest such tag was `18.7-bookworm`.
- A new check name, `integration`, is added to `scripts/check.sh`. It starts the containers it needs with `docker run`, runs the tests, and removes the containers, also after a failure. CI gets a job with the same name that runs `scripts/check.sh integration` on the same runner label as the other jobs, where Docker is already installed. The official image has an ARM variant. The workflow does not use a `services:` block, because that path cannot be run on a workstation.
- Tests reach only a database that this script started. They read its address from the environment the script sets. They never contact another database and use fixture credentials only.
- Without a running Docker daemon the check exits 3. It is never reported as passed or silently skipped.

**Fuzzing** (first extended by the slice that first parses bytes from a socket, [scope 004](../../scope/004-one-real-primary-path.md) or [scope 006](../../scope/006-protocol-breadth-and-cancellation.md)):

- A new target goes in the `fuzz/` directory of the crate it tests, with a manifest of the same shape as `crates/pool-config/fuzz/Cargo.toml`, its own committed lockfile, a baseline file for rule W1 and a `deny` run on that lockfile.
- That slice adds a CI job `fuzz` that builds every target and runs it on its committed regression inputs for a bounded time, and adds the job to the branch rule. From then on the fuzz packages are compiled on every pull request.
- An input that made a target crash is committed as a regression input together with the fix.

**New workspace members**: a slice that adds a crate adds `[lints] workspace = true` to its manifest (rule W4), its package to `lock-packages.txt` (rule W1) and its allowed dependencies to `internal-dependencies.txt` (rule W5). `pool-core` and `pool-control` never appear as a pair in that file.

**Notices**: the first slice that bundles native code, or the first release, whichever comes first, creates the `NOTICE` file. Until then the license check and the inventory are the record.

### Reviewer roster and fresh model reviews

The roster itself is a living table in the [engineering standards](../../engineering-standards.md#reviewer-roster), because holders change and a spec should not. This spec decides its shape and the rule below.

A fresh model is a model session that did not write or design the change, starts without the author's conversation, and works from the spec, the diff and the running build. Its findings go to `docs/reviews/`.

| A fresh model may do this instead of a person | A named person is required |
|---|---|
| The independent review of a slice that `SEC-14` asks for, for any change, including protocol, parser, authentication, pooling and cancellation changes | Approving a promotion, fencing, recovery or HA design, and signing off a drill or a lab result |
| The cross check of a drafted spec | Accepting a risk, or closing a gate while a criterion is failed, blocked or not run |
| A first pass over a dependency update: advisories, maintenance signals, the license inventory | Adding a license, a license exception or an advisory `ignore` entry |
| Checking that a verification record matches its spec | Release sign off and notices |
| | Handling a security report |
| | Changing the branch rule or the required checks on the remote |

Limits:

- A review is never done by the session or person that wrote the change (`R-25`).
- While Project lead, Maintainer or Release maintainer has no named holder, the repository owner decides in that role, and the record of the decision says so. Without this, a new advisory or the branch rule itself could not be acted on by anyone. For this scope it means: accepting this spec approves the first license list, and the permission you gave the lead covers applying the branch rule.
- The other roles in the right column have no such stand in. If a promotion, fencing, recovery or HA design, or a drill result, has no named holder to approve it, its gate stays open and the feature does not become `done`.
- A fresh model review carries a slice. It does not by itself qualify a release. At a [milestone gate](../../plan/milestones.md), a role that still has no named person for authentication, cancellation, routing or promotion is recorded as open exposure, with the release claim it blocks, as the risk register already asks.

### Document corrections

Each item is a sentence that is false for the workspace as it is, with the text that replaces it. Three were changed together with this spec: line 3 and line 9 of `docs/engineering-standards.md`, and the second paragraph of "CI progression" in `docs/testing/strategy.md`. Line 9 was changed once more after the review of 2026-11-14, to name the fuzz package as the one place outside the `unsafe_code` rule. The rest are Build plan steps 6 and 9.

The replacement texts are in code blocks because their links are written for the file they go into, not for this one.

1. `docs/testing/strategy.md`, line 3.

   ```text
   old: All runtime evidence is pending.
   new: Evidence exists only where a verification record says so. The first one is the [record for spec 0001](../specs/0001-initial-architecture/verify.md), for the offline configuration check. No proxy behavior has been run.
   ```

2. `docs/implementation-order.md`, first paragraph, its first three sentences.

   ```text
   old: The next queued unit of work is [001. Stack and workspace](scope/001-stack-and-workspace.md). The design in [spec 0001](specs/0001-initial-architecture/index.md) is ratified; the workspace does not exist yet. Implementation starts only on explicit request.
   new: The [scope index](scope/index.md) shows which unit of work is next. The first handoff below was carried out as [001. Stack and workspace](scope/001-stack-and-workspace.md): the workspace exists and [spec 0001](specs/0001-initial-architecture/index.md) holds its build notes. Later work starts only on explicit request.
   ```

3. `docs/index.md`, line 5, its first sentence.

   ```text
   old: For the next development task, start with the [workspace handoff](implementation-order.md#first-handoff-the-workspace).
   new: For the next development task, start with the [scope index](scope/index.md) and the [handoff for later slices](implementation-order.md#handoff-for-later-slices).
   ```

4. `docs/index.md`, the citation examples (Build plan step 6). The old example cites a spec that does not exist, which rule D5 would report.

   ```text
   old: `0004 AC-1` for a criterion that exists only in a feature spec
   new: `0001 AC-1` for a criterion that exists only in a feature spec
   ```

5. `docs/repository-structure.md`, line 3, its second sentence.

   ```text
   old: Only documentation and the chosen project license are created during planning.
   new: The paths that exist today are the scope 001 workspace (`apps/pool`, `crates/pool-config`) and the tooling of [spec 0002](specs/0002-standards-and-tooling/index.md).
   ```

6. `docs/repository-structure.md`, the layout block. Remove the lines `rustfmt.toml` and `clippy.toml`. Add `deny.toml` after `rust-toolchain.toml`. After the `crates/` group add `scripts/` and `tools/` with `repo-check/` under it. Under `.github/` add `rulesets/` and `pull_request_template.md`.

7. `docs/repository-structure.md`, "Merge or defer the candidate crates".

   ```text
   old: Acyclic internal dependencies are checked in CI from Cargo metadata once a workspace exists.
   new: The allowed dependencies between workspace packages are listed in a baseline file and checked on every pull request (rule W5 of [spec 0002](specs/0002-standards-and-tooling/index.md#workspace-guards)).
   ```

8. `docs/repository-structure.md`, last paragraph.

   ```text
   old: `CHANGELOG.md` starts with the first tagged release.
   new: `CHANGELOG.md` records changes under `Unreleased` until the first tagged release.
   ```

9. `docs/research/technology-evaluation.md`, "Adoption gate", last sentence.

   ```text
   old: No application dependencies were installed during this phase.
   new: The dependencies adopted so far are listed under [Adopted in scope 001](#adopted-in-scope-001) and [Adopted in scope 002](#adopted-in-scope-002).
   ```

10. `docs/plan/risk-register.md`, first paragraph, last sentence.

    ```text
    old: Each owner is a role to staff, not a named person already assigned.
    new: Each owner is a role. The [reviewer roster](../engineering-standards.md#reviewer-roster) names who holds it, or shows that nobody does yet.
    ```

11. `CONTRIBUTING.md`. In the command block, replace the three `cargo` lines with the one line `scripts/check.sh all`. Then:

    ```text
    old: There is no CI yet; it arrives with [scope 002](docs/scope/002-standards-and-tooling.md).
    new: Every pull request runs seven required checks. To reproduce a failed check named `X`, run `scripts/check.sh X`. A change reaches `main` only through a pull request whose checks passed. The checks and that path are described in [spec 0002](docs/specs/0002-standards-and-tooling/index.md).
    ```

12. `CONTRIBUTING.md`, last paragraph. The list of what the project still needs names "remote repository review/branch protection settings". Leave it as it is in step 9. It is removed in the closing pull request of remote step 7, once remote steps 1 to 6 have passed. The maintainer team and the private reporting channel stay in the list.

13. `README.md`, "Build and check". Keep the four command block. Add after it:

    ```text
    new: `scripts/check.sh all` runs the checks that CI runs, except the Linux container build, which is `scripts/check.sh linux-build`.
    ```

`AGENTS.md` and `CLAUDE.md` are not in this list. The audit session and the lead's sync step own them.

### Value sourcing

| Value | Source |
|---|---|
| Check names | The table under [Required checks](#required-checks). The workflow job ids and the branch rule contexts repeat them, and rule C2 keeps them equal |
| Rust toolchain in CI | `rust-toolchain.toml`, through `rustup toolchain install` |
| Tool versions | Variables at the top of `scripts/check.sh` |
| Linux platform, builder image and inner script | Spec 0001, "Linux build", copied into `scripts/linux-build.sh` and guarded by rule W7 |
| Runner label | This spec: `ubuntu-24.04-arm`, the hosted runner with the architecture of spec 0001's Linux target |
| Action commit hashes | GitHub API, read on 2026-11-14, written in `ci.yml` |
| Base revision for D6 | `POOL_CHECK_BASE`, set by the workflow from the event, else `origin/main` |
| Lockfile package names | The scratch runs recorded in the [rationale](rationale.md#evidence-gathered) |
| Allowed licenses | The licenses the two lockfiles need today, checked with `cargo-deny` |
| Owner and repository name for the manual steps | `gh repo view --json nameWithOwner`, run by the person who does the step |
| Holders in the roster | The repository owner, through the lead. No session guesses a name. Until a name is given, the row says `Not assigned` |

### Evidence

The developer writes `docs/specs/0002-standards-and-tooling/verify.md` with one section per milestone, as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks, and adds it to the spec's entry in `docs/index.md`. A step that needs GitHub is recorded there as `NOT_RUN` with `owner: lead` until the lead has run it, and then with the pull request address, the run address and the commit hash. Those addresses reach `main` in the closing pull request of remote step 7.

## Build plan

The order is a tracer bullet (one thin real path first): first the policy and the entry point, so that one check runs from one command; then the files CI and the remote need; then the program that guards them; then the proof that every check bites; then the documents and the evidence. Each step ends with a commit named `<type>(<scope>): spec 0002 step <n>, ...`.

1. **Lint rules and dependency policy.** Add the Clippy table to the root `Cargo.toml` ([Lint rules](#lint-rules)) and create `deny.toml` ([Dependency policy](#dependency-policy)). Set this spec's `**Status**` line to `In Progress`. Files: `Cargo.toml`, `deny.toml`, this file's status line. Satisfies **AC-20**, and the policy half of **AC-4**.
2. **Fuzz harness.** Create the four files under `crates/pool-config/fuzz/` ([Fuzz harness](#fuzz-harness)). Build once with the pinned nightly to create `Cargo.lock`, compare its package names with `fuzz-lock-packages.txt` in this spec, and commit it. Files: `crates/pool-config/fuzz/`. Satisfies the package half of **AC-15**.
3. **Check entry point.** Write `scripts/check.sh` with every name in [The check entry point](#the-check-entry-point), and `scripts/linux-build.sh` ([Linux build script](#linux-build-script)). This step needs the update of spec 0001 that moves its Linux target to ARM to be on `main` first, because the script copies that command. `fmt`, `clippy`, `test` and `deny` pass after steps 1 and 2; `guards` and `docs` pass after steps 5 and 6. Files: `scripts/check.sh`, `scripts/linux-build.sh`. Satisfies **AC-1**, completes **AC-4**, and the script half of **AC-5**.
4. **CI definition, branch rule and pull request template.** Create the three files under `.github/` exactly as given in [CI definition](#ci-definition), [Branch rule and change path](#branch-rule-and-change-path) and [Pull request template](#pull-request-template). Files: `.github/workflows/ci.yml`, `.github/rulesets/main.json`, `.github/pull_request_template.md`. Satisfies **AC-2**, and the file halves of **AC-13** and **AC-14**.
5. **Repository check program, workspace and CI rules.** Create the package `tools/repo-check/` with the commands `workspace` and `ci`, the six baseline files and tests for rules W1 to W7 and C1 to C5. Files: `tools/repo-check/`. Satisfies **AC-6**, **AC-7**.
6. **Repository check program, document rules.** Add the command `docs` with rules D1 to D6 and their tests. Change the citation example in `docs/index.md` ([Document corrections](#document-corrections)). `scripts/check.sh all` passes from here on. Files: `tools/repo-check/src/`, `docs/index.md` (the example only). Satisfies **AC-8**, **AC-9**, **AC-10**.
7. **Proof that the checks bite.** Write `scripts/prove-checks.sh` with the twenty eight cases ([Proving the checks](#proving-the-checks)) and run it. Files: `scripts/prove-checks.sh`. Satisfies **AC-11**.
8. **Fuzz run and inventory run.** Run `scripts/check.sh fuzz` and `scripts/check.sh sbom` once each. `cargo-cyclonedx` 0.5.10 is installed for this with `cargo install --locked --root target/tools cargo-cyclonedx@0.5.10`. No committed file changes, except a fix to `scripts/check.sh` if the inventory flags had to change. Completes **AC-15**, satisfies **AC-16**.
9. **Document corrections.** Apply items 1 to 3, 5 to 11 and 13 of [Document corrections](#document-corrections). The texts about CI and pull requests describe `main` as it is once the pull request of step 11 is merged; they reach `main` only with that pull request. Files: `CONTRIBUTING.md`, `README.md`, `docs/implementation-order.md`, `docs/index.md` (line 5), `docs/repository-structure.md`, `docs/testing/strategy.md` (line 3), `docs/plan/risk-register.md`, `docs/research/technology-evaluation.md` (item 9 only). Satisfies **AC-19**, completes **AC-14**, **AC-18**.
10. **Workstation evidence.** Run `scripts/check.sh all`, `scripts/prove-checks.sh` (with `--with-linux-build` if `docker info` succeeds) and the fuzz and inventory commands on the final tree. Record commands, results, the revision and tool versions in `verify.md`. Fill the locked versions you observed into the "Adopted in scope 002" section of the technology evaluation. Add two rows to `docs/index.md`: `verify.md` under this spec, and `.github/pull_request_template.md` under "Project root". Record the four remote items as `NOT_RUN`, `owner: lead`. Files: `docs/specs/0002-standards-and-tooling/verify.md`, `docs/research/technology-evaluation.md` (observed versions), `docs/index.md` (two rows). Completes **AC-17**. Evidence for **AC-1**, **AC-2**, **AC-4**, **AC-6** to **AC-11** and **AC-14** to **AC-20**.
11. **Remote evidence (run by the lead, not by a build session).** With everything merged into the local `main`, and `scripts/check.sh all` and `scripts/prove-checks.sh` green on that revision, run remote steps 1 to 7 under [Remote proof](#remote-proof) in order: push the branch and open the pull request, read the Linux build lines from its run, apply the branch rule, try the direct push, open and close the failing pull request, merge, and then open and merge the closing pull request. The rule is applied only after the seven checks have passed once on a GitHub runner. The checker records the addresses in `verify.md`, the lead links them from scope 002, and both reach `main` in the closing pull request of remote step 7, together with item 12 of [Document corrections](#document-corrections). Satisfies **AC-3**, **AC-12**, completes **AC-5**, **AC-13** and **AC-19**.

**Parallel sessions.** Steps 1, 2, 3, 4 and 9 touch files that do not overlap and can run at the same time (step 3 once the spec 0001 update it names is on `main`); the developer of step 3 reads the commands from this spec and does not wait for steps 1 and 2. Steps 5 and 6 share one package and run one after the other in one session; they can start at once, because their tests use their own inputs, but their run on the real tree is green only after steps 1 to 4 are merged. Step 7 comes after steps 1 to 6. Step 8 needs steps 2 and 3. Step 10 comes after everything else, and step 11 after step 10 is merged.

## Consequences

**Positive**:

- A failed CI check is one command away on your own machine, and it is the same command.
- A new package, a new license, a known advisory, a socket in the source or a forbidden crate fails a check by itself. The parts of `0001 AC-12` and `0001 AC-13` that were checked by hand no longer are.
- The Linux build runs on every pull request, on hardware of the architecture you deploy to, with the very command spec 0001 names. A guard keeps the script and the spec equal.
- Every check has a recorded case that it refuses, so a green result means something.
- The product still has sixteen third party packages. The tooling added none to it.
- A review finding cannot be merged past unseen. On the first scope pull request the rule held the merge for two findings of the automatic Copilot review. Both were real defects (an exported `CDPATH` could move the fuzz run to another tree, and a link inside a code span with two backticks was read as a link), and both were fixed before the merge.

**Negative / tradeoffs**:

- The coordination flow can no longer push `main`. Every scope now closes through a pull request that a person opens or allows, and waits for seven checks. A scope whose evidence exists only after that pull request ran needs a second, closing pull request, and waits for the seven checks twice. This is the main cost of the decision.
- An open review thread holds a merge the way a failed check does, and this was not weighed when the rule was written. While the automatic Copilot review is on for the owner's account, any pull request can end up waiting for a person: the threads must be marked as resolved by the repository owner, or by the lead with the owner's permission, also after the fix is in and all seven checks are green. A scope that closes with two pull requests can wait twice. Follow-up 12 holds the choice.
- The branch rule requires no approval, because one account cannot approve its own pull request. Until a second maintainer account exists, the independent review is a written record that GitHub does not enforce.
- The repository owner can still change or switch off the rule in the settings. The rule makes a bypass visible; it does not make it impossible.
- "Merge commit only" holds for the merge button and the merge API. While 0 approvals are required, GitHub may accept a direct push of the head of an open, up to date, green pull request, and `main` then moves without a merge commit. The content has still passed all seven checks.
- Three line based readers (lockfiles, manifests, the workflow and ruleset files) stand in for real parsers. They work because this spec fixes the shape of those files. An unusual but valid way to write a manifest can produce a false problem, which is then fixed in the tool, not worked around.
- The anchor rule follows GitHub's heading anchors as observed on this project's documents. It does not know headings written with underlines, HTML anchors or emphasis written with underscores.
- Rule D6 can tell that an ID is gone. It cannot tell that the text behind an ID changed its meaning, or that two IDs swapped their text. That stays a review matter.
- The fuzz package is not compiled by any required check, and the fuzz run and the inventory are commands, not checks. Each can stop working without a red check until the slice that makes it one.
- The fuzz harness brings ten third party packages and compiled C++ (libFuzzer) into a test only package, on a nightly toolchain. None of it can reach the product, and rule W1 on the product lockfile shows that.
- The daily run can fail on a day when nobody changed anything, because a new advisory was published. That is its purpose, and it is also an interruption.
- Nothing is built or tested for 64 bit Intel or AMD Linux. A contributor on such a machine can meet a failure that CI never saw.
- `scripts/linux-build.sh` holds a second copy of the command in spec 0001. Rule W7 makes a difference fail, at the price of one more thing to update when spec 0001 changes.

**Known limits of the guards**, found by the review of 2026-11-14 and left as they are on purpose. Each one is small or needs intent, and a pull request can change the guard itself anyway: the guards catch honest drift and help the reviewer, they do not stop a person who means to get around them.

- The CI definition guards do not read the `on:` block of `ci.yml`, so removing the daily `schedule:` passes. They do not read the values under `with:`, so `persist-credentials: true` passes. In `main.json` they read only the check names, so a changed `enforcement` or a bypass actor in the file passes. The file is applied by hand in remote step 3, whose output shows what GitHub really holds. The runner up that would close these is Follow-up 11.
- Rules W4 and W5 know only the members written in the `members` array. A package that Cargo treats as a member because of a path dependency is not checked for lint inheritance. Rule W1 still makes its name visible in the lockfile baseline (Follow-up 10).
- Rule W3 is a tripwire, not a wall. It compares plain text, as the `grep` of `0001 AC-12` does, so a grouped import such as `use std::{io, thread};` passes, and a folder named `target` is skipped at any depth. Review is the guard for what it misses.
- Rule W7 guards only the lines after the marker of `scripts/linux-build.sh`. The lines before it are fixed by this spec and by review.
- A symbolic link under a walked folder is not followed and not reported, so a linked `.rs`, `.md` or `Cargo.lock` file is not checked.
- The Markdown reader does not know every form GitHub accepts. Beyond the heading forms named above, it reports a target with a query part as missing, takes a relative target such as `README.md:12` for a scheme and skips it, and does not see a link that is split over two lines.
- Four parts of the dependency policy have no proving case: two versions of one package, another registry, a withdrawn version and an unmaintained package. A later edit that weakens one of those keys in `deny.toml` would not be caught by `scripts/prove-checks.sh`, only by review of the pull request that touches `deny.toml`, which the pull request template asks to be listed.

**Neutral**:

- The seven check names, `scripts/check.sh` and the baseline files are contracts from now on. Renaming a check means changing the workflow and the branch rule together, which rule C2 enforces.
- Later slices change baseline files on purpose. That is the design: the diff of a baseline file is where a reviewer sees the change.
- `pool-repo-check` and `pool-config-fuzz` are packages that are never published and never workspace members.

## Follow-up

1. Name the holder of each role in the [reviewer roster](../../engineering-standards.md#reviewer-roster), or confirm that it stays `Not assigned`. The rows that need a person block the gates named there.
2. When a second maintainer has a GitHub account, raise the required approvals to 1 in `main.json` and on the remote.
3. Spec 0001 Follow-up 9 and Follow-up 11 are answered by **AC-5** and **AC-6** of this spec. Applied in spec 0001 (2026-11-14): its Follow-up 9 and Follow-up 11 are marked as answered, its "Linux build" section points at `scripts/linux-build.sh` and rule W7, and its "Workspace layout and manifests" section notes that the root `Cargo.toml` gained a Clippy lint table.
4. `0001 AC-2` passed in the container on the workstation on 2026-11-14, and spec 0001 is accepted. From remote step 1 on, the CI job `linux-build` runs the same command on a native runner on every pull request. Nothing in spec 0001 needs to change for that. Done: remote steps 1 and 2 ran on 2026-11-14, and the run on the native runner printed `runner: Linux aarch64` and `linux build check passed`. The evidence is in [verify.md](verify.md).
5. The private channel for security reports and the maintainer team in `SECURITY.md` and `CONTRIBUTING.md` are still open. GitHub private vulnerability reporting is off on the repository. This spec does not decide them.
6. [Scope 004](../../scope/004-one-real-primary-path.md) builds the `integration` check and the first protocol fuzz target under [Rules for later slices](#rules-for-later-slices).
7. [Scope 011](../../scope/011-release-qualification-harness.md) turns the inventory into a release inventory with notices, and decides whether the inventory becomes a required check.
8. After the first scheduled run, confirm that the daily run happened (remote step 8).
9. The text of the coordination skill (`.claude/skills/coordinate/SKILL.md`: its step 9, "Push `main`", and the limit "Lead hanya push `main` ke `origin`") no longer fits [Branch rule and change path](#branch-rule-and-change-path) once the branch rule is active. The repository owner updates it, because no session edits `.claude/`. Until then the lead follows this spec: remote step 7 of the [remote proof](#remote-proof), a closing pull request from `scope/<number>-closing`, stands in for step 9 of the skill, for this scope and for later ones. The rule is active since remote step 3 ran on 2026-11-14 (the evidence is in [verify.md](verify.md)), so this applies now. The skill text also has no word yet for step 4 of "How a change reaches `main`", the review threads.
10. Before the slice that adds a third workspace crate, decide how rules W4 and W5 learn the packages and their dependencies. The review of 2026-11-14 (finding M3) named a fuller option that was not taken now: read the edges from the root `Cargo.lock`, which names the real package however the manifest spells it. The same decision covers a package that Cargo treats as a member without it being written in the `members` array (finding m14). Today's rule reads the manifests, with the four spellings of M3 added.
11. The runner up for the CI definition guards is a rule in the style of W7 that compares `ci.yml` and `main.json` with their code blocks in this spec. It would also cover what the rules C1 to C5 still do not read (the `on:` block, the values under `with:`, and every field of `main.json` except the check names). Take it up if one of those gaps is ever met in practice.
12. Decide whether `required_review_thread_resolution` stays `true` in `main.json` and on the remote. Remote step 3 showed what it costs (see [Branch rule and change path](#branch-rule-and-change-path)), and the choice has not been made. Until it is, the rule stays as it is. The options:
    - **Keep it** (recommended, and what the lead recommends). Step 4 of "How a change reaches `main`" is the way through. Every finding is judged before a merge. The price is a wait for the repository owner on each pull request that gets a thread.
    - **Set it to `false`**, in the file and on the remote in one pull request. A thread then informs and no longer holds, so a finding can be merged past before anyone judged it. This changes the rule payload and needs the repository owner.
    - **Keep it and switch off the automatic Copilot review** in the owner's GitHub settings. Nothing holds a pull request then, but the reviewer that found two real defects in the first pull request it read is gone.
13. Remote steps 1 to 6 of the [remote proof](#remote-proof) have run: steps 1 to 5 on 2026-11-14 and step 6, the merge, on 2026-11-15 (local time, seven hours ahead of UTC). The evidence is in [verify.md](verify.md). Step 7, the closing pull request, and step 8, the first daily run (Follow-up 8), are still to run. The notes that those runs added to this spec are listed in the [rationale](rationale.md#update-after-the-remote-steps-of-2026-11-14-and-2026-11-15).

## Rationale

Reasoning, alternatives, and source evidence: [rationale.md](rationale.md).
