# Standards and tooling rationale

## Context

Scope 001 left a small real workspace: the `pool` binary, the `pool-config` library, 134 tests and one offline command. Its rules are checked by hand. A person runs three `cargo` commands, a `grep`, and reads two manifests. Nothing fails by itself when someone adds a dependency, a socket or a crate that the design forbids. The review of scope 001 said so, and spec 0001 left two follow ups for this scope.

The project also has rules that exist only as sentences. The engineering standards ask for a protected `main`, pull requests, required CI and review for safety changes. The security requirements ask for an inventory of dependencies with licenses and advisories (`SEC-13`). The risk register names dependency drift (`R-16`), license conflicts (`R-17`) and a reviewer who is not available at a gate (`R-25`). Several documents still describe the repository as it was before the workspace existed.

Four facts about the place this is built in shaped the answer. The repository is public on GitHub, owned by one personal account, with Actions switched on, no workflow, no pull request and no rule on `main`. The workstation is macOS on Apple silicon, so a Linux build there is emulated. No session may write to GitHub; a person does that. And the coordination flow that builds each scope has, until now, pushed `main` directly when a scope closes.

The cost of not deciding is plain. Every later slice adds sockets, native code, new crates and a database. Each of those needs a check that already exists and is already trusted, or it will be added in a hurry inside a slice whose attention belongs to protocol safety.

## Options considered

### Option 1: One shell entry point, a small Rust check program outside the workspace, GitHub Actions, pull request only `main` (chosen)

Each check is a name. `scripts/check.sh <name>` runs it on a workstation and in CI. Rules that no existing tool covers (lockfile package sets, forbidden paths and source text, lint inheritance, links, anchors, scope status, requirement IDs) live in one Rust program with no dependencies, kept outside the Cargo workspace.

**Pros**:

- A CI failure and its local reproduction are the same command.
- The pinned Rust toolchain is the only thing needed to run the document checks. No second language and no interpreter version to pin.
- The custom rules have unit tests, so "the check catches a wrong change" is itself a test.
- The scope 001 criteria that count workspace members and locked packages stay true, because the program is not a workspace member.

**Cons**:

- The program is code to maintain, with line based readers in place of real parsers.
- A package outside the workspace needs its own format, lint and test commands. The entry point hides that, but it is there.
- Every scope now closes through a pull request that a person has to open.

### Option 2: Off the shelf tools only

Use `cargo-deny` for dependencies, a link checker such as `lychee` for documents, and shell one liners for the rest.

**Pros**:

- Least code written here.
- Each tool is maintained by someone else.

**Cons**:

- No tool checks scope status agreement or this project's requirement IDs, so shell scripts would still be written, and shell is the hardest place to test a rule.
- A link checker is one more tool to install on the runner and on each workstation, with its own large dependency graph to trust.
- The anchor rules would be whatever the tool decides, and would change with its versions.

### Option 3: A Python script for the document and workspace checks

One script with the standard library only.

**Pros**:

- About a third of the code of the Rust program.
- No compile step.

**Cons**:

- A second language in a project whose standards are written for Rust, and an interpreter that is not pinned. The workstation has Python 3.9.6; the runner image has a newer one.
- Reviewers of a proxy written in Rust now review Python too.

### Option 4: Make the check program a workspace member (the usual `xtask` pattern)

**Pros**:

- `cargo fmt`, `cargo clippy` and `cargo test` cover it with no extra command.

**Cons**:

- `0001 AC-1` says the workspace has exactly two members, and `0001 AC-12` says the lockfile holds two workspace packages. Both would stop being true while spec 0001 is still in progress and its Linux evidence is still open. This spec may not edit spec 0001.

## Rationale

The scope asks for "a path to reproduce failures locally". The only design in which that path cannot drift from CI is one where CI has no logic of its own. So the workflow file contains nothing but checkout, toolchain install and `scripts/check.sh <job id>`, and a guard (rule C3) keeps it that way.

The custom rules are the reason for the Rust program. Scope status agreement and stable requirement IDs are particular to this repository's documents, so no tool has them. Once a program must exist for those, links and anchors cost little more, and one program is easier to trust than a tool plus scripts. Rust is chosen over Python because the toolchain is already pinned by file and installed everywhere the checks run, and because the people who review this project read Rust. Keeping the program outside the workspace is the price of leaving the scope 001 criteria untouched.

Pull request only `main` follows from two sentences that were already ratified: the standards ask for a protected `main` with required CI, and spec 0001 says that from scope 002 on changes go through pull requests. A rule that lets the owner push past it would make the checks advice. The cost lands on the coordination flow, which is why that decision is marked heavy.

The tool choices are the boring ones. `cargo-deny` covers advisories, licenses, bans and sources in one configuration file and was already on the workstation. `cargo-fuzz` is the standard libFuzzer front end for Rust. `cargo-cyclonedx` is the generator maintained by the CycloneDX project itself. The official `postgres` image is what PostgreSQL publishes.

What is built now is limited by what exists to check. There is a parser of untrusted bytes (`pool_config::validate`), so one fuzz target is built and run. There is a lockfile, so one inventory is generated. There is no database path, so the PostgreSQL supply is a rule and nothing more.

## Decisions and runner ups

Each line is a decision taken on your behalf under the `auto` decision mode. Tag `heavy` marks a decision that changes how later scopes work or that needs a person's authority; `ordinary` marks one that is cheap to reverse.

| # | Question | Taken | Runner up | Tag | Why |
|---|---|---|---|---|---|
| 1 | CI service | GitHub Actions, hosted runner | A self hosted runner on your Proxmox hosts | ordinary | The repository is on GitHub and Actions is on. A self hosted runner on a public repository runs code from any pull request on your own machine |
| 2 | Runner image | `ubuntu-24.04`, named exactly | `ubuntu-latest`, or `ubuntu-26.04` | ordinary | A moving label changes the build under you. 24.04 is the older of the two current images and has Docker and `rustup` |
| 3 | How `main` is protected | A repository ruleset with nobody on the bypass list | Classic branch protection with the owner allowed to bypass | heavy | A bypass for the owner means the one account that does all the work is never checked. It changes how every later scope closes |
| 4 | How the coordination flow closes a scope | Push the merged result as `scope/<number>-<slug>` and open a pull request | Keep pushing `main` through an owner bypass, with CI running after the fact | heavy | Checks that run after the push cannot stop a bad push. Spec 0001 already announced pull requests from this scope on |
| 5 | Required approvals | 0 for now, 1 when a second maintainer account exists | 1 now | heavy | GitHub does not let an author approve their own pull request and there is one account. Requiring 1 would stop every merge |
| 6 | Merge method | Merge commit only | Squash | ordinary | The commits of each Build plan step are the record the conventions ask for. A squash would erase them |
| 7 | Entry point | A POSIX shell script | A cargo alias to the Rust program, which then runs `cargo` itself | ordinary | The Linux build is already a shell command. A shell script has nothing to compile before the first check can run |
| 8 | Custom checks | A Rust program with no dependencies | A Python script | ordinary | See the rationale above |
| 9 | Where that program lives | `tools/repo-check/`, its own workspace root | A workspace member | ordinary | Keeps `0001 AC-1` and `0001 AC-12` true |
| 10 | How the workspace guards know what is allowed | Baseline files that a spec changes on purpose | Values written into the program | ordinary | Later slices must add sockets and crates. A text file diff is where a reviewer sees that |
| 11 | Dependency, license and advisory checker | `cargo-deny` 0.20.2 | `cargo-audit` for advisories plus `cargo-about` for licenses | ordinary | One tool and one file for four checks |
| 12 | Allowed licenses | `Apache-2.0`, `MIT`, `Unicode-3.0`, and `NCSA` for `libfuzzer-sys` only | A wider list written in advance (BSD, ISC, Zlib and so on) | heavy | The list is exactly what the two lockfiles need today. A license nobody needs yet is a decision nobody has made yet. Marked heavy because a license list is a Release maintainer decision (`R-17`) |
| 13 | Unmaintained packages | Fail for direct and indirect dependencies | Fail for direct dependencies only | ordinary | `R-16` is about abandoned crates anywhere in the graph. With sixteen packages the stricter rule costs nothing today |
| 14 | Dependency updates | Deliberate pull requests, no bot; a daily run finds new advisories | Dependabot | ordinary | A bot opens pull requests nobody asked for in a project that builds only on request. The daily run gives the signal without the noise |
| 15 | Linux build in CI | The exact container command of spec 0001, on a native runner | The same steps run directly on the runner, without the container | ordinary | The same script then serves the workstation and CI, and its green run is evidence for `0001 AC-2` as written |
| 16 | Installing `cargo-deny` on the runner | `cargo install --locked` into `target/tools`, cached | A third party install action, or a downloaded release binary | ordinary | The same command works on a workstation, and no action outside GitHub's own is trusted |
| 17 | Actions used | `actions/checkout` and `actions/cache`, each named by commit hash | Also a Rust toolchain action | ordinary | `rustup` is on the runner and `rust-toolchain.toml` already says which version |
| 18 | SBOM generator | `cargo-cyclonedx` 0.5.10, CycloneDX JSON 1.5 | `cargo-sbom` 0.10.0 | ordinary | Maintained by the CycloneDX project. `cargo-sbom` is simpler to drive, since it writes to stdout, and is the fallback named in the spec |
| 19 | Is the SBOM a required check | No. A command, run once for evidence | A job on every pull request | ordinary | `SEC-13` asks for it before a release. Nothing is released yet. Scope 011 decides again |
| 20 | Fuzz harness | `cargo-fuzz` 0.13.2, `libfuzzer-sys` 0.4.13, toolchain `nightly-2026-10-17` | `cargo-afl`; or property tests only | ordinary | libFuzzer through `cargo-fuzz` is the common choice and both were already installed and work on this machine |
| 21 | Build a fuzz target now | Yes, one, for `pool_config::validate` | Record the choice only | ordinary | It is the one place that parses bytes it does not trust. Building it proves the harness on this workspace before a protocol slice depends on it |
| 22 | Fuzzing in CI | Not yet. The first socket parsing slice adds the job | A build only job now | ordinary | It would add a nightly toolchain and a C++ build to every pull request for a target that guards an offline command |
| 23 | PostgreSQL 18 for tests | Official image by digest, started by `scripts/check.sh integration`, same locally and in CI | Distribution packages on the runner and Homebrew locally; or the workflow's `services:` block | ordinary | Two installation paths mean two PostgreSQL builds. A `services:` block cannot be run on a workstation |
| 24 | Clippy lints added | Deny `dbg_macro`, `print_stderr`, `print_stdout`, `todo`, `unimplemented` | Also deny `unwrap_used`, `expect_used`, `panic` | ordinary | The five are clean in every target today and three of them are rules spec 0001 already wrote. The other three would need `allow` attributes across 80 test lines, and the standards ask for review there, not a ban |
| 25 | `rustfmt.toml` and `clippy.toml` | None | Add both with explicit settings | ordinary | Defaults of a pinned toolchain are already fixed. A file of defaults is one more thing to keep true |
| 26 | Requirement ID stability | Compare with the base revision in `git` | A committed list of all IDs | ordinary | A second list is a second owner of the same fact. `git` already knows what the IDs were |
| 27 | Where the reviewer roster lives | The engineering standards | This spec, or a new file | ordinary | Holders change. A spec that is accepted should not be edited for a new name |
| 28 | What a fresh model may review | The independent review of any slice. Not approvals that need authority over infrastructure, licenses, risk, releases, security reports or the remote | Fresh model for everything; or a person for every security review | heavy | Scope 001 already used a fresh model for `SEC-14`. The right column is where a wrong approval cannot be undone by a later review |
| 29 | Negative proof | A committed script with twenty cases, plus one failing pull request on the remote | A list of commands in the verification record | ordinary | A script can be run again by the checker and after every change to a check |

Decisions 3, 4, 5, 12 and 28 are the ones to look at before you accept.

## Evidence gathered

All of this was done on 2026-11-14, in a scratch directory outside the repository that was removed afterward, on macOS (Apple silicon) with Rust 1.100.0.

- **Repository settings, read only.** `gh` reports: public, owned by a user account, default branch `main`, one collaborator, merge commits allowed. `main` is not protected and there are no rulesets. Actions are enabled with `allowed_actions: all`, and the default workflow token is read only. Private vulnerability reporting is off. The local `main` was one commit ahead of the remote.
- **Documents.** A throwaway scan of 75 Markdown files found 519 local links and none broken, using the anchor rule written in the spec. The ID survey found 14 `SEC`, 25 `R`, 165 `F` and 14 `T` definitions and no citation without one, except the example in `docs/index.md` that cites a spec which does not exist.
- **cargo-deny 0.20.2.** With the `deny.toml` in the spec, `cargo deny --locked check` on a copy of the workspace printed `advisories ok, bans ok, licenses ok, sources ok`. Changing the workspace license to `GPL-3.0-only` failed with `license is not explicitly allowed`. Adding an unused license to the list failed with `license-not-encountered`. Adding a dependency without updating the lockfile failed with `cannot update the lock file`. A made up advisory for `itoa`, added as a file to the downloaded advisory database, failed with `RUSTSEC-0000-0001`, and the file survived a second run that fetched again. A `file:` address for the database is refused by the tool, which is why case 6 works on the downloaded copy.
- **Fuzz harness.** `cargo fuzz init` wrote a wrong package name and no `[workspace]` table, so the manifest in the spec is written by hand. With it, `cargo +nightly-2026-10-17 fuzz run validate` built in 17 seconds and did 599095 runs in 21 seconds with no crash. Its lockfile held the 24 package names listed in the spec. `cargo deny` on that lockfile failed only on the `NCSA` part of the `libfuzzer-sys` license, and passed with the one exception. The same configuration on the product lockfile then prints one warning and passes.
- **Lints.** With the five Clippy lints denied, `cargo clippy --workspace --all-targets --locked -- -D warnings` passes with no source change. `expect_used` would report 73 places and `panic` 7, all in tests; none in library or binary code.
- **Bad changes.** Lines A, B, C and D of the spec were applied one at a time. `cargo fmt --all --check` printed `Diff in`. Clippy printed `ptr_arg` for B and `print_stdout` for C. The test run printed `test result: FAILED` for D.
- **Versions read from registries.** crates.io: `cargo-deny` 0.20.2 (MIT OR Apache-2.0), `cargo-fuzz` 0.13.2 (MIT OR Apache-2.0), `libfuzzer-sys` 0.4.13 ((MIT OR Apache-2.0) AND NCSA), `cargo-cyclonedx` 0.5.10 (Apache-2.0), `cargo-sbom` 0.10.0. GitHub API: `actions/checkout` `v7.0.1` at `3d3c42e5aac5ba805825da76410c181273ba90b1`, `actions/cache` `v6.1.0` at `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`, both MIT. Docker Hub: `postgres:18.7-bookworm`, pushed 2026-11-12, digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, not pulled.
- **Documentation read.** The GitHub REST reference for repository rulesets (rule types, the `pull_request` and `required_status_checks` parameters, `~DEFAULT_BRANCH`). The `cargo-cyclonedx` and `cargo-sbom` readme files (options and where output goes). `rustup toolchain install --help` on rustup 1.29.2 says it installs the active toolchain when no name is given.
- **Docker.** `docker info` succeeded during this work, although the brief recorded the daemon as stopped.
- **Not verified.** Nothing was run on a GitHub runner. The workflow file, the ruleset file and the `rustup` version on the runner are specified, not observed. `cargo-cyclonedx` was not installed or run; its flags come from its readme. The `pool-repo-check` program, `scripts/check.sh` and `scripts/prove-checks.sh` do not exist yet.

## References

**Project sources**:

- [Scope 002](../../scope/002-standards-and-tooling.md), its paragraph and "Done when"
- [Spec 0001](../0001-initial-architecture/index.md): "Linux build", "Change path", `0001 AC-1`, `0001 AC-2`, `0001 AC-12`, `0001 AC-13`, Follow-up 9 and 11
- [Engineering standards](../../engineering-standards.md): "Dependency and configuration changes", "Security and reviews", "Git and releases"
- [Security requirements](../../security/security-requirements.md): `SEC-13`, `SEC-14`
- [Risk register](../../plan/risk-register.md): `R-12`, `R-16`, `R-17`, `R-25`
- [Testing strategy](../../testing/strategy.md): the layer table, "Acceptance evidence", "CI progression"
- [Repository structure](../../repository-structure.md): fuzz targets live inside a package
- [Technology evaluation](../../research/technology-evaluation.md): the adoption gate

**Practices and standards**:

- CI runs the same command a person runs
- Actions named by commit hash, a read only token and no secret for pull request workflows
- A license allow list that holds only what is needed
- A check is trusted only after it has been seen to fail
- CycloneDX as the inventory format
