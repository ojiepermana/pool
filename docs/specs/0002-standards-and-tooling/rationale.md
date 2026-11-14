# Standards and tooling rationale

## Context

Scope 001 left a small real workspace: the `pool` binary, the `pool-config` library, their tests and one offline command. Its rules are checked by hand. A person runs three `cargo` commands, a `grep`, and reads two manifests. Nothing fails by itself when someone adds a dependency, a socket or a crate that the design forbids. The review of scope 001 said so, and spec 0001 left two follow ups for this scope.

The project also has rules that exist only as sentences. The engineering standards ask for a protected `main`, pull requests, required CI and review for safety changes. The security requirements ask for an inventory of dependencies with licenses and advisories (`SEC-13`). The risk register names dependency drift (`R-16`), license conflicts (`R-17`) and a reviewer who is not available at a gate (`R-25`). Several documents still describe the repository as it was before the workspace existed.

Four facts about the place this is built in shaped the answer. The repository is public on GitHub, owned by one personal account, with Actions switched on, no workflow, no pull request and no rule on `main`. Your proxy and PostgreSQL VMs are ARM, so the Linux target of spec 0001 is 64 bit ARM. The workstation is macOS on Apple silicon and can run that target in a container without emulation; a CI runner has to be ARM to do the same. Build sessions may not write to GitHub. When this spec was drafted nobody but you could; while it was being finished you gave the lead permission to do so for this scope. And the coordination flow that builds each scope has, until now, pushed `main` directly when a scope closes.

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

- `0001 AC-1` says the workspace has exactly two members, and `0001 AC-12` says the lockfile holds two workspace packages. Both would stop being true. Spec 0001 is accepted, and this spec may not edit it.

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
| 2 | Runner image | `ubuntu-24.04-arm`, named exactly, for all seven jobs | `ubuntu-26.04-arm`; or the ARM runner for `linux-build` only and `ubuntu-24.04` for the rest | ordinary | The runner must have the architecture of the Linux target, which is ARM. A moving label such as `ubuntu-latest` changes the build under you, and that label is an Intel or AMD image anyway. One label for every job keeps rule C4 simple and runs the tests on the architecture you deploy |
| 2a | Also build for 64 bit Intel or AMD | No | A second `linux-build` job on `ubuntu-24.04` that is not required | ordinary | The project declares one Linux target. A second one would double the slowest check to prove something nobody deploys. It is one job to add if a contributor on such a machine needs it |
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
| 13a | Unsound packages (added after the review of 2026-11-14, finding m10, decided by the lead) | `unsound = "all"`: fail for direct and indirect dependencies | The default of `cargo-deny` 0.20.2, `workspace`: direct dependencies only | ordinary | A project whose rule is safe first party Rust most wants to hear about a dependency whose safe interface is not safe. Nothing in either lockfile is affected today |
| 14 | Dependency updates | Deliberate pull requests, no bot; a daily run finds new advisories | Dependabot | ordinary | A bot opens pull requests nobody asked for in a project that builds only on request. The daily run gives the signal without the noise |
| 15 | Linux build in CI | The exact container command of spec 0001, copied into a script and compared with the spec by rule W7 | The same steps run directly on the runner, without the container; or a script that restates the platform and image itself | ordinary | The same script serves the workstation and CI. Spec 0001 stays the only place that names the platform and the image, which mattered at once: the target moved from Intel to ARM while this spec was being written |
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
| 27a | Who decides while a role has no holder | The repository owner, for Project lead, Maintainer and Release maintainer only | Every gate that needs a person stays open until a name is given | heavy | With every row unassigned, nobody could add an advisory exception, approve a license or apply the branch rule, and `main` would freeze on the first new advisory. The owner is the one person who certainly exists. The HA and infrastructure roles get no stand in, because those approvals need specialist judgment |
| 28 | What a fresh model may review | The independent review of any slice. Not approvals that need authority over infrastructure, licenses, risk, releases, security reports or the remote | Fresh model for everything; or a person for every security review | heavy | Scope 001 already used a fresh model for `SEC-14`. The right column is where a wrong approval cannot be undone by a later review |
| 29 | Negative proof | A committed script with twenty eight cases, plus one failing pull request on the remote | A list of commands in the verification record | ordinary | A script can be run again by the checker and after every change to a check |

| 30 | What the direct push probe sends (review finding B1) | A new empty commit on top of the remote `main`, made with `git commit-tree`, that belongs to no pull request and has no check run | The first commit after `origin/main` on the way to the local `main` | ordinary | If the rule were not active, the empty commit is the smallest thing that can land, and it can be taken off again while nothing protects `main`. The runner up would move `main` by one real commit of this scope |
| 31 | When the branch rule is applied (M1) | After the seven checks have passed once on a GitHub runner | Before the first run, as first written | ordinary | A first run on a new runner often fails for a reason outside the code. With this order that happens while nothing is locked. It costs nothing: the scope branch stays up to date with `main` |
| 32 | How the evidence of a scope reaches `main` after the merge (M2) | A second pull request from `scope/<number>-closing` | Commit the evidence on the scope branch before the merge, and accept that it names the run of the commit before it | heavy | The addresses of the merge and of the failing pull request exist only after the merge. It is marked heavy because it replaces step 9 of the coordination skill for every later scope. The lead asked for this shape |
| 33 | How rule C3 keeps a check from being switched off (M4, decided by the lead) | A short list of allowed keys after `jobs:`, one line per `run:` step, and a known `<name>` | A rule in the style of W7 that compares `ci.yml` and `main.json` with their blocks in the spec | ordinary | The list is a few lines in the existing reader and names the bad key in its message. The comparison is fuller and is kept as Follow-up 11 |
| 34 | Which `<name>` a `run:` may pass to `scripts/check.sh` | A job id of the file, or `tools` | The eleven names of the entry point, written into the program | ordinary | A later slice that adds a job gets its name allowed by adding the job, and rule C2 ties job ids to the branch rule. `all`, `fuzz` and `sbom` have no place in the workflow today |
| 35 | Where the "one workflow file" rule and the stricter `permissions:` rule live (m16, m17) | Inside rule C5 | A new rule C6 | ordinary | `0002 AC-7` names the rules C1 to C5. A sixth rule would change the text of a criterion for no gain |
| 36 | How rule W5 reads a dependency (M3) | Keep reading the manifests, and add the four spellings the review found | Take the edges from the root `Cargo.lock` | ordinary | The same small helper also removes the false alarms of finding m13, and the tests stay as they are. The lockfile option is sounder in the long run and is Follow-up 10, to decide before a third crate exists |
| 37 | How the sources half of the policy is proven (m11) | A real dependency on a Git repository in a temporary folder, with the policy as committed | Empty the `allow-registry` list in the copy of `deny.toml` | ordinary | The reviewer's worry is a later edit that weakens `unknown-git`. Only a case that leaves `deny.toml` alone can catch that |
| 38 | What to do with the gaps the new rules still leave (rest of m17, m14, m15, m18, m19) | Name them as known limits in the spec | Close each one now | ordinary | Each is small or needs intent, and a pull request can change the guard itself. Closing them all is the comparison of decision 33 or a real parser |

Decisions 3, 4, 5, 12, 27a, 28 and 32 are the ones to look at before you accept.

## Evidence gathered

All of this was done on 2026-11-14, in a scratch directory outside the repository that was removed afterward, on macOS (Apple silicon) with Rust 1.100.0.

- **Repository settings, read only.** `gh` reports: public, owned by a user account, default branch `main`, one collaborator, merge commits allowed. `main` is not protected and there are no rulesets. Actions are enabled with `allowed_actions: all`, and the default workflow token is read only. Private vulnerability reporting is off. The local `main` was one commit ahead of the remote.
- **Documents.** A throwaway scan of 75 Markdown files found 519 local links and none broken, using the anchor rule written in the spec. The ID survey found 14 `SEC`, 25 `R`, 165 `F` and 14 `T` definitions and no citation without one, except the example in `docs/index.md` that cites a spec which does not exist.
- **cargo-deny 0.20.2.** With the `deny.toml` in the spec, `cargo deny --locked check` on a copy of the workspace printed `advisories ok, bans ok, licenses ok, sources ok`. Changing the workspace license to `GPL-3.0-only` failed with `license is not explicitly allowed`. Adding an unused license to the list failed with `license-not-encountered`. Adding a dependency without updating the lockfile failed with `cannot update the lock file`. A made up advisory for `itoa`, added as a file to the downloaded advisory database, failed with `RUSTSEC-0000-0001`, and the file survived a second run that fetched again. A `file:` address for the database is refused by the tool, which is why case 6 works on the downloaded copy. `--config` is an option of `cargo deny` itself and goes before `check`; after `check` the tool rejects it. The two command lines in the spec are the ones that were run.
- **Fuzz harness.** `cargo fuzz init` wrote a wrong package name and no `[workspace]` table, so the manifest in the spec is written by hand. With it, `cargo +nightly-2026-10-17 fuzz run validate` built in 17 seconds and did 599095 runs in 21 seconds with no crash, with `CARGO_TARGET_DIR` pointing outside the package, as the spec asks. Its lockfile held the 24 package names listed in the spec. `cargo deny` on that lockfile failed only on the `NCSA` part of the `libfuzzer-sys` license, and passed with the one exception. The same configuration on the product lockfile then prints one warning and passes.
- **Lints.** With the five Clippy lints denied, `cargo clippy --workspace --all-targets --locked -- -D warnings` passes with no source change. `expect_used` would report 73 places and `panic` 7, all in tests; none in library or binary code.
- **Bad changes.** Lines A, B, C and D of the spec were applied one at a time. `cargo fmt --all --check` printed `Diff in`. Clippy printed `ptr_arg` for B and `print_stdout` for C. The test run printed `test result: FAILED` for D.
- **Versions read from registries.** crates.io: `cargo-deny` 0.20.2 (MIT OR Apache-2.0), `cargo-fuzz` 0.13.2 (MIT OR Apache-2.0), `libfuzzer-sys` 0.4.13 ((MIT OR Apache-2.0) AND NCSA), `cargo-cyclonedx` 0.5.10 (Apache-2.0), `cargo-sbom` 0.10.0. GitHub API: `actions/checkout` `v7.0.1` at `3d3c42e5aac5ba805825da76410c181273ba90b1`, `actions/cache` `v6.1.0` at `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`, both MIT. Docker Hub: `postgres:18.7-bookworm`, pushed 2026-11-12, digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, not pulled.
- **Documentation read.** The GitHub REST reference for repository rulesets (rule types, the `pull_request` and `required_status_checks` parameters, `~DEFAULT_BRANCH`). The `cargo-cyclonedx` and `cargo-sbom` readme files (options and where output goes). `rustup toolchain install --help` on rustup 1.29.2 says it installs the active toolchain when no name is given.
- **Runner.** The GitHub documentation page for hosted runners lists `ubuntu-24.04-arm`, `ubuntu-22.04-arm` and `ubuntu-26.04-arm` as standard arm64 runners for public repositories (4 processors, 16 GB), and says standard runners are free and unlimited there. The image list for Ubuntu 24.04 Arm64 (image version 20261111.182.1) names Docker 28.0.4, rustup 1.29.3, Rust 1.99.1 and Git 2.56.1. `docker info` on the workstation reports `aarch64 linux`.
- **Docker.** `docker info` succeeded during this work. The lead ran the Linux build of spec 0001 in the container the same day (`0001 AC-2` passed), so checks that need a container run locally after a `docker info` test. The lead also reports a passing trial of the same command as an ARM container from `main`, with the same image digest, which is an index that holds an ARM variant.
- **GitHub Actions app.** `gh api /apps/github-actions` reports id 15368, the value used as `integration_id` in the branch rule.
- **Scope headings.** Fourteen of the seventeen feature files end their first line with the tag `needs a decision`. The cross check found this; the status rule allows tags because of it.
- **Not verified.** Nothing was run on a GitHub runner. The workflow file and the ruleset file are specified, not observed, and the runner facts above were read, not run. `cargo-cyclonedx` was not installed or run; its flags come from its readme. The `pool-repo-check` program, `scripts/check.sh` and `scripts/prove-checks.sh` do not exist yet.

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

## Independent cross check

A different model (Claude Opus) read the drafted spec and the related document edits without changing files, fetching links or running builds. It tried the document rules by hand on the real files. Its verdict: the design is sound and most of it can be built as written, and five things would have stopped it from going green. Under the `auto` decision mode its recommended fixes were applied, except where noted.

| Finding | What was done |
|---|---|
| The scope status rule rejected 14 of 17 real feature files, whose first line ends with the tag `needs a decision` | The rule now allows tags after the status and asks for a test with one |
| Building `tools/repo-check` would write into `tools/repo-check/target/`, which is not ignored, and leave the tree dirty | Every cargo command on that package passes `--target-dir target/repo-check` |
| `scripts/prove-checks.sh` could not find `cargo-deny` inside its copy | It puts the original `target/tools/bin` on `PATH` |
| `--config` might belong after `check` | Not changed. The order in the spec is the one that was run; the other order is rejected by the tool. Recorded under evidence |
| No rule for how a command's exit status becomes the status of `scripts/check.sh` | Added |
| The cases of the proof look for text that the messages were only allowed, not required, to contain | A table now says what each message must name |
| The remote commands were left to a file that did not hold them | They are in the spec, under "Remote proof" (added before the cross check returned, when you gave the lead permission) |
| With every roster row unassigned, the branch rule, an advisory exception and the license list could be decided by nobody | The repository owner decides for three roles until they are named (decision 27a) |
| Step 9 had to record versions that exist only after later steps | Moved to step 10 |
| The contributing text about CI is applied before CI exists on `main` | Kept in step 9, with the reason: that text reaches `main` only inside the pull request that makes it true. The one sentence about remote settings still waits for the remote evidence |
| Clone details of the proof script: detached commit, restore, identity, base revision | All fixed in the spec |
| A push event can carry forty zeros as its earlier commit | Treated as no base |
| "A search for each old sentence finds nothing" was false, since this spec quotes them | Now "the file each item names no longer contains it" |
| The pull request template is a new Markdown file and the index lists every one | Step 10 adds its row |
| `CARGO_TARGET_DIR` with cargo-fuzz was thought untested | It was the way the scratch run was done. Recorded under evidence |
| The Clippy table departs from the manifest text in spec 0001 | One sentence under "Lint rules", and a pointer for the lead |
| The duplicated Linux build command had no guard | Rule W7 (added before the cross check returned, when the target moved to ARM) |
| The CI guard required one matching step, not the absence of other logic | Rule C3 now covers every `run:` value |
| A status with the same name from another source could satisfy a required check | `integration_id` 15368 on each check |
| A scheduled run and a push run shared one queue | The concurrency group now includes the event name |
| Recommendation to record the fuzz harness only and build it with the first socket parsing slice | Not taken. Decision 21 stands: one target is built now. It is listed for you with its runner up |
| Doc edits that stated things as true too early: "every package inherits it", "are now checked by a tool", present tense in the testing strategy, "branch protection" | Each reworded |

The cross check read the spec before the Linux target moved to ARM, so the runner label, rule W7 and the "Remote proof" section were not part of what it reviewed.

## Update after the review of 2026-11-14

The [review of scope 002](../../reviews/2026-11-14-scope-002-standards-and-tooling.md) blocked the scope on one finding and raised five major ones. This update answers the findings that belong to the spec: B1, M1 and M2 on the remote proof, M4 on rule C3, the spec half of M3, and the minor findings m1 to m5, m7, m9 to m11, m16 and part of m17. Decisions 13a and 30 to 38 above are the choices it made. No criterion ID changed, and no criterion text had to change: the number of proof cases and the remote step numbers are named only in the design sections and the Build plan.

No second cross check by another model was run for this update. The Opus reviewer had already read these sections for the review, and each remote command that changed was reasoned again against the GitHub pages and run wherever a run was possible without writing to GitHub.

What was checked on 2026-11-14 for this update, in a scratch folder outside the repository unless it says otherwise:

- **The push probe.** Against a bare repository on the same machine, the block of remote step 4 was run under `zsh` and `sh`. With a hook that refuses the push, it printed the refusal and `push refused`, both `git ls-remote` lines were equal, and the local `main` did not move. With no hook, the empty commit landed, and the lease push took it off again. A first try with `"$PROBE:refs/heads/main"` failed under `zsh`, which reads `:r` after a variable name as a modifier; the blocks now write `${PROBE}`.
- **Read only calls to GitHub.** The repository has no ruleset (`length` prints `0`), no pull request and no run. `gh run list --workflow ci` fails with "could not find any workflows named ci" while the workflow is not on the default branch, so the blocks find a run by commit and event. The wait loop and every `--jq` text that needs no pull request were run as written. `delete_branch_on_merge` is false. The `gh` version is 2.96.0.
- **Not run.** No step that writes to GitHub was run. Whether GitHub accepts a direct push of a green pull request head with 0 required approvals was not tried, and the GitHub pages read for this do not say so in plain words for rulesets. The page on required status checks says that a pull request that is up to date and passing can be merged locally and pushed to a protected branch, which is the case the first probe would have hit. The same pages say that a branch under the pull request rule takes changes only through a pull request and that a commit must first pass its checks on another ref, which is why the empty commit must be refused. Whether checks that passed before the rule was created count for it is reasoned from "required checks must pass on the latest commit", not observed; remote step 3 reads the merge state and names what to do if they do not.
- **`unsound = "all"`.** `cargo-deny` 0.20.2 accepts the key, and both lockfiles still pass with it. A wrong value is refused with the list `all`, `workspace`, `transitive`, `none`.
- **The new proof cases.** Case 22 (a dependency from a Git repository in a temporary folder) made `scripts/check.sh deny` fail with `error[source-not-allowed]`, and the `cargo metadata` step worked with every HTTP request blocked. Case 23 (`serde_json = "*"`) failed with `error[wildcard]`. Cases 25 to 28 were run against the check program as it is on `main` and gave `C4`, `C5`, `W6` with `docs/Cargo.lock`, and `D4` with `SEC-14`. Case 24 passes on that program today, which is finding M4; it must fail once rule C3 is rebuilt.
- **The reworded rules C3 to C5.** A throwaway script that follows the new sentences passed on the committed `ci.yml` and reported every input of findings M4, m16 and m17 that the rules now claim: `continue-on-error`, `if:`, a continuation line, `shell:` after an empty line, a command after `tools;`, a quoted key, `name:` on a job, a second permission after a comment, a job level `permissions:`, a job without `runs-on:`, and `secrets['X']`.
- **The opening lines of `scripts/linux-build.sh`.** With a stub `docker`, under `sh`, `dash` and `bash --posix`: no repository and a repository with no commit gave exit 3 at the `git rev-parse` line, a broken index gave exit 3 at the `git status` line, a dirty tree gave exit 3, and a clean tree reached the marker, also with an exported `CDPATH` that pointed at a decoy folder and when called from another directory.
