# Review, main (scope 003, control and session data model), 2026-11-15

**Reviewed by**: `claude-fable-5-2` for the code, the manifests, the guards and the tests (author on `claude-opus-5-5`), with a second independent read of the production code by `claude-sonnet-5-5`, and `claude-opus-5-5` for spec 0003, the ADR and the other documents (author on `claude-fable-5-2`). Each part was read by a model that did not write it. The main reviewer is a session that had not seen the implementation before this review.
**Scope**: the range `ca9a897..be06393` on `main`, read with `git diff ca9a897..be06393 -- <path>`. 48 files without `graphify-out/` and `docs/scope/`, which were left out on purpose.
**Verdict**: Approve with nits. No blocker and no major finding. 15 minor findings and 18 nits. The test reviewer and the document reviewer each rated one finding major. I checked both against the code and the spec and rate them minor here; the reasons are with t1 and d1.

## Summary

The change adds the keys for `[control]`, `[[cluster]]`, `[[node]]` and `[[identity]]` to `pool-config`, the identifier and generation types, and a third crate, `pool-control`, that holds the authority rules as plain functions. Nothing listens, nothing opens a file or a database, and the report of `pool check-config` for a valid file is byte for byte what spec 0001 says.

The code is careful and it does what spec 0003 says. I found no way to get an `Open` answer from the inputs you pass in unless the configuration, the journal value and a fresh sample of the approved writer agree. Every other state I could build ends closed. No text from the file reaches a finding, a `Debug` text, a `Display` text or a panic message in any probe I ran.

What is left are limits of the design that the next slice has to close, not defects of this one. The most important is that a `Ready` state is a copy of the journal from the moment of the start, and nothing in `pool-control` lets the daemon tell it about a conflict latch that was set later (m1). Read m1 to m3 before scope 004 calls `writer_admission`.

## How this review was run

- I read every production file in the range in full: the five files of `crates/pool-control/src/`, the four of `crates/pool-config/src/`, `apps/pool/src/report.rs`, the manifests, the lockfile, the change to `tools/repo-check/src/workspace.rs`, the three baseline files and `scripts/prove-checks.sh`. I read spec 0003 `index.md` in full before the code.
- Three reviewers ran in parallel and did not see each other's results: `claude-sonnet-5-5` on the production code, `claude-fable-5-2` on the tests, `claude-opus-5-5` on the documents. I checked each of their findings against the code before it went into this file.
- I built `pool` from the tree at `be06393` (debug build, macOS, Rust 1.100.0) and ran `pool check-config` on inputs I made: CRLF line ends, tab indents, multi byte text before a value, a value at the end of a file with no line feed, a byte order mark, an empty file, and the canary `CANARY-9f2c7b1e` in a key, a table name, a value of the wrong type, an array entry, an unterminated string and every string key of the new tables.
- I ran a temporary test against `pool-control` with two clusters, edge times and a journal that changed after the start, read its output, and removed the file again. The tree was clean afterwards.
- I changed the code on purpose five times, ran the tests, and put the code back each time (a mutation probe: if the tests still pass, nothing pins that line). The five changes are named in t1, t2 and t3.
- I ran `cargo test --workspace --locked` on the tree: 238 tests passed, 0 failed (78 in `apps/pool`, 3 in its unit tests, 25 and 95 in `pool-config`, 3 and 26 in `pool-control`, 8 documentation tests). I did not run `scripts/check.sh all`, `scripts/prove-checks.sh`, the Linux build or the fuzz target. The lead ran those on `main`. A run on macOS is not Linux evidence.

## The five questions of the brief

**1. Can anything get `Open` without full agreement?** Not from honest inputs. `writer_admission` returns `Open` only at `crates/pool-control/src/admission.rs:305`, after the witness check (`:195`), the cluster lookup (`:232`) and all nine other rules gave no reason. `Authority` is made only at `crates/pool-control/src/authority.rs:183`, and only when the journal is consistent, the witness is equal and the hold is `none`. Every other class of start maps to one closed reason (`admission.rs:210`). Probes: age equal to the expiry is closed and one nanosecond less is open; an expiry of zero is closed; a `now` before the sample is closed; a conflict in cluster `b` closes `b` and leaves `a` open; a cluster that only the journal knows is `authority.cluster_unknown`; a node that the journal lists in two clusters closes the one the configuration does not put it in. There is no arithmetic that can overflow: the age is a `checked_sub` and the generations only compare. Three limits remain, and all three are about what the caller passes in: m1, m2 and m3.

**2. Can file text leak, and is `LineIndex` right?** No leak found. Each record that holds text from the file has a hand written `Debug` that prints `[REDACTED]` (`crates/pool-config/src/topology.rs:74`, `:148`, `:210`, `:239`, `:274`), a `Finding` holds a code, a constant key and two numbers, and a parser error gives only its span (`crates/pool-config/src/lib.rs:538`). The canary appeared in no output of the twelve canary runs. `LineIndex` (`lib.rs:579`) gave the right line and column for every edge I tried, and a lookup is a binary search plus a scan of fewer than 512 bytes, so there is no quadratic path. I found no `unwrap`, `expect`, index or slice in code outside tests that input can reach. Findings come out in the order of the spec, and the skips that keep one mistake from giving two findings are where the spec puts them (`topology.rs:704` to `:766`). `resolve_identity` is an exact lookup in a map (`topology.rs:269`). See m5 and m8.

**3. Is the public contract of spec 0001 unchanged?** Yes. `apps/pool/src/report.rs` changed by one line (the arm for `Location::KeyAt`, `:106`). `OUTPUT_VERSION`, `SUPPORTED_CONFIG_VERSION`, the valid report, the seven old codes with their messages and the exit status are as they were. The valid report of `valid-topology.toml` and the JSON report of `valid-minimal.toml` are the bytes of spec 0001.

**4. Are the guards sound?** The baseline changes are exactly the three lines the spec names. The new lockfile check of W4 works for an entry at the start, in the middle and at the end of the file. It has one known way around it, which a test pins as intended (m4).

**5. Does spec 0003 match the code, and do the notes change accepted text?** The spec matches the code closely. Every reason name and its order, every bootstrap refusal, every finding code and message, every key path and every constant that the spec names is in the code as written. Each record has an owner, a source, a place and a restart rule, the walks are traced, and the three questions that scope 004 needs are answered. The code has seven public accessors that the interface block of the spec does not list (n12). The dated notes in spec 0001 and spec 0002 add text and rewrite no sentence. They do change what a reader has to take as current: `0001 AC-1`, `0001 AC-12` and `0001 AC-13` now describe the first build only, the first secret reference key arrives one scope earlier than spec 0001 says, three baseline files differ from the block in spec 0002, W4 has one more check, and case 17 edits another file. Each note says so in plain words and names `0003 AC-13` as the current statement, which is an honest way to do it. W3 and the forbidden source patterns did not change. Two notes say a little more than is true: "closes finding m14" (m4), and that the rows for W4 and case 17 "keep their text", which is right about the text and leaves rows that no longer describe the code (n13).

## Minor

### 🟡 m1. A `Ready` state cannot learn of a conflict latch that was set after the start, `crates/pool-control/src/admission.rs:182`, `crates/pool-control/src/authority.rs:137`

**Problem**: `AuthorityState::Ready` holds a private copy of the journal as it was when `classify_startup` ran, and `writer_admission` reads only that copy. Spec 0003 says the daemon classifies once and keeps the state for the whole run (`docs/specs/0003-control-and-session-data-model/index.md:784`), and that after a second primary was seen the daemon sets the latch in the journal and "also remembers it in memory for the rest of the run" (`:781`). `pool-control` has no way to take that memory in. Code outside the crate also cannot build a `Closed` with `authority.conflict_latched`. Probe: `a2` reports that it is a primary, the answer is `Closed` with `evidence.second_primary` and `latches_conflict()` is true. Then `a2` drops out of sight and the same state answers `Open`, `a1`. That is row a of walk 3, where the spec says "the answer is row o, not row a" (`:862`). The walk tests reach row o only with a journal value that a test built by hand with the latch already set.
**Why it matters**: This is the case `R-19` and `PRD-006 AC-2` are about: the other primary is still there, it just cannot be seen any more, and writes open again. It is the same kind of gap as the one `0003 AC-16` closed, where the function trusted its caller to keep two things together. Nothing calls the function yet, so nothing is wrong today. It will be wrong on the first day of scope 004 unless that slice adds the missing piece inside `pool-control`. The same holds for the authority revision in an `Open` answer (`admission.rs:309`), which is the one read at the start.
**Suggested fix**: Decide it in spec 0003 before scope 004 dispatches SQL, for example as a new item of Follow-up 4 or as a criterion next to `AC-16`. Two shapes would work: a function in `pool-control` that takes a `Ready` state and a `Closed` that asks for the latch and gives back the state with the latch set, or a rule that the daemon reads and classifies the journal again after every write to it, plus the memory for the case where that write failed. Then add a walk that goes from row d to row o through the real functions, with the second primary gone in the second call. Owner: the architect for the spec, then the developer.

### 🟡 m2. Any crate can build a journal value, so the type rule of `AC-12` holds only by convention, `crates/pool-control/src/authority.rs:18`, `:35`

**Problem**: `StoreProbe`, `StoredAuthority`, `ApprovedTopology` and `Quarantine` are public with public fields. Code in any crate can write a `StoredAuthority` with the writer it likes, pass it to `classify_startup` with a configuration that carries the same witness, and get `Ready` and then `Open`. The spec says so (`index.md:727`) and names the condition under which its safety claim holds (`:1037`, "as long as the journal reader stays the only code that builds a `StoreProbe`"). No check enforces that condition.
**Why it matters**: `AC-12` shows that `Authority`, `EligiblePrimary`, `Closed` and `BootstrapPlan` cannot be built outside the crate. With a free `StoreProbe` that proof does not carry far: the one thing that turns into authority is the thing anyone can make. Today only tests do.
**Suggested fix**: When scope 004 puts the journal reader inside `pool-control`, make the fields private and give the tests one constructor that runs the same checks as the reader, or add a line to rule W3 or W5 that keeps `StoreProbe::Present` out of every other crate. Record the choice in Follow-up 4. Owner: the architect.

### 🟡 m3. Two inputs turn a seen second primary into no evidence, `crates/pool-control/src/observation.rs:140`

**Problem**: `facts` uses the latest sample of a node, and it gives no evidence when that sample is dated after `now` (`:157`) or when two samples share the latest time (`:146`). Both are what the spec says (`index.md:729`). For the approved writer that closes. For any other node it hides what the node said. Probes: a fresh sample of `a2` as a primary plus a later sample of `a2` at 101 seconds with `now` at 100 gives `Open`. The same sample of `a2` passed twice gives `Open`.
**Why it matters**: Both can happen in scope 004 without anyone doing something strange. If the daemon reads `now` and a probe result lands a moment later, that sample is in the future of `now`. If two probe paths hand in the same sample, they tie. In both cases one admission can open while a second primary is in the list.
**Suggested fix**: A spec decision for scope 004. The smallest one is a duty of the caller, written into the documentation comment of `writer_admission` and into Follow-up 4: read `now` after the list of observations is fixed, and pass at most one sample per node. A stricter one is in the rule itself: for a node other than the writer, a latest sample that cannot be used closes with a reason of its own. Owner: the architect.

### 🟡 t1. No test fails when the check for a repeated cluster `id` is removed, `crates/pool-config/src/topology.rs:688`

**Problem**: `config.duplicate_id` is tested for `node.id` and `identity.id` (`crates/pool-config/tests/validate.rs:1991` to `:2000`) and never for `cluster.id`. I took the call for clusters out of `cross_check`, and all 238 tests of the workspace still passed. The code itself is right: a file with two clusters named `main` gives `config.duplicate_id at cluster.id, line 9, column 6` from the built binary.
**Why it matters**: `writer_admission` and `plan_bootstrap` take the first cluster with a given `id`. A file with the same `id` twice would have a second `expected_writer` and a second system identifier that nothing reads. The test reviewer rated this major. I rate it minor, because an open answer would still need the first entry, the journal and fresh evidence to agree, so losing the check would confuse an operator and would not open a write. It is still the first test I would add.
**Suggested fix**: One test with two `[[cluster]]` entries of the same `id`, and one that has a repeated cluster, node and identity `id` in one file to pin the order clusters, nodes, identities of step 6.1. Owner: the tester.

### 🟡 t2. Three skip rules of the cross checks are not pinned, `crates/pool-config/src/topology.rs:728`, `:731`, `:772`

**Problem**: Three more changes to the code left every test green. First, skipping the `expected_writer` check for all clusters when one cluster has a bad `id`, where the spec says the check "still runs for every other cluster" (`index.md:313`). Second, running that check although a node has a bad `cluster`, where the spec says it is skipped as a whole. Third, letting the endpoint check read a node whose port failed, which `AC-15` relies on for made input 3 ("There is no other finding").
**Why it matters**: These are the rules that make one mistake give one finding (`AC-2`). They were reworded after the verification of milestone 2, so they are the part of the validation order most likely to be touched again.
**Suggested fix**: Three small cases in `crates/pool-config/tests/validate.rs`: two clusters where one has a bad `id` and the other a wrong writer; a writer whose node has a bad `cluster`; two nodes with the same host and `port = 0`. Owner: the tester.

### 🟡 m4. The lockfile check of W4 can be passed, and nothing proves it rejects, `tools/repo-check/src/workspace.rs:236`

**Problem**: An entry ends at the next `[[package]]` line or at the end of the file, so a table after the last package counts as part of it. A `[[patch.unused]]` table with a `source` line after a local package that sorts last hides that package. The test at `:1210` pins this as intended, because the spec text defines an entry that way (`index.md:981`). `scripts/prove-checks.sh` has a case for W4 (`:265`), but none for the lockfile check, so the rule of spec 0002 that every check is shown to reject a wrong change is not met for it. The note in spec 0002 says the check "closes finding m14" (`docs/specs/0002-standards-and-tooling/index.md:1374`), which is more than it does.
**Why it matters**: The check exists for one case, a package that Cargo treats as a member without being listed. That case still needs a change to the W1 baseline, which a reviewer sees, so the risk is small. A guard with a known way around it and a test that calls the way around correct will be trusted more than it should be.
**Suggested fix**: End an entry at any line that starts with `[`. A lockfile entry never holds a table of its own. Change the sentence in spec 0003 and the pinned test with it, add one case to `scripts/prove-checks.sh` that adds a local package to the lockfile and expects W4, and soften "closes" in the spec 0002 note until then. Owner: the architect for the two sentences, the developer for the rest.

### 🟡 m5. One file at the size limit gives two million findings and half a gigabyte of memory, `crates/pool-config/src/topology.rs:399`, `apps/pool/src/report.rs:139`

**Problem**: An entry with no key gives one finding per required key. A file of 1048574 bytes that holds `identity = [{},{},...]` has 349514 entries and gives 2097084 findings. Measured with the debug build: the human format wrote 190709698 bytes in 2.5 seconds with a peak of 405 MB of memory, and the JSON format wrote 251525214 bytes in 8.5 seconds with a peak of 539 MB. The time is linear. The human format writes through the line buffered `stdout` lock (`apps/pool/src/main.rs:77`), so each finding is one system call.
**Why it matters**: The file belongs to the operator, so this is not an attack. It is the same function the daemon will call at a start and at a reload, and "bounded resources" in the root guide then means about 500 times the input. `AC-15` bounds the time for the made inputs and says nothing about the number of findings.
**Suggested fix**: Wrap the locked `stdout` in a `BufWriter` in `check_config`, which is a small change with no change to the output. Then decide in the spec whether the number of findings gets a limit, for example one `config.missing_key` per entry with no key at all, or a fixed top count with a last line that says more were left out. That second part changes `AC-3`, so it is the architect's call. Owner: the developer for the buffer, the architect for the limit.

### 🟡 t3. The `compile_fail` tests do not check the error they name, and the record says they do, `crates/pool-control/src/admission.rs:89`, `crates/pool-config/src/lib.rs:44`

**Problem**: The four documentation tests of `pool-control` say `compile_fail,E0451` (`admission.rs:89`, `:137`, `crates/pool-control/src/authority.rs:128`, `crates/pool-control/src/bootstrap.rs:57`). The stable compiler does not check the code after the comma. I changed one to `E0308`, an unrelated error, and the test still passed. `docs/specs/0003-control-and-session-data-model/verify.md:206` says the tests "cannot pass for an unrelated compile error", which is not so on Rust 1.100.0. The older test for `ValidatedConfig` (`lib.rs:44`) got weaker with this change: its struct literal names three of what are now eight fields, so it fails for the missing fields and would keep failing if the fields became public.
**Why it matters**: These tests are the proof of the second half of `AC-12` and of the rule in `AC-3` that `validate` is the only way to a `ValidatedConfig`. Each still catches the real mistake as long as its snippet has no other error. A renamed field or type turns it into a test that passes for the wrong reason, without a sign.
**Suggested fix**: Correct the sentence in `verify.md`. For `ValidatedConfig`, use the update form, `ValidatedConfig { config_version: 2, ..valid }` with a value from `validate`, whose only possible error is the private field. For each `compile_fail` test add a passing documentation test beside it that uses the same paths through the public functions. Owner: the tester, and the checker for the sentence.

### 🟡 d1. Row k of walk 3 goes against the letter of `PRD-006 AC-2`, and the PRD does not say so, `docs/specs/0003-control-and-session-data-model/index.md:780`, `:855`

**Problem**: `PRD-006 AC-2` reads "Two observed primaries close write admission until explicit reconciliation" (`docs/prd/006-cluster-management.md:60`). Row k keeps writes open when the second primary is a quarantined node, and rules 9 and 10 leave quarantined nodes out. `AC-9` of spec 0003 cites `PRD-006 AC-2` and includes row k. The spec gives its reason in the text (`index.md:780`) and as decision 17 of the rationale, marked heavy, with the runner up named.
**Why it matters**: The document reviewer rated this major. I rate it minor. The decision is written down, reasoned, and I think it is right: the quarantine is the reviewed answer to exactly that node, closing would let one returned old primary stop a cluster after a correct promotion, and POOL sends nothing to the quarantined node either way. What is missing is one sentence where the requirement lives, so that a reader of the PRD does not take row k for a defect, and so that scope 010 knows the exception also covers a quarantined node on a later timeline.
**Suggested fix**: Add the exception to `PRD-006 AC-2`, or a dated note under it that points to spec 0003, and say in the spec that rule 10 leaves quarantined nodes out on purpose. `docs/prd/` is not a file a sub session may change, so this is for the lead. Owner: the lead and the architect.

### 🟡 m6. The cost of one admission grows with nodes times observations, `crates/pool-control/src/admission.rs:238`, `:288`, `crates/pool-control/src/observation.rs:143`

**Problem**: Each call walks every node of the file, builds two lists, and for every node of the cluster walks every observation. The member check at `admission.rs:257` is members times nodes. Spec 0003 has scope 004 call the function at every session admission and again after the backend is acquired (`index.md:784`).
**Why it matters**: With three nodes it is nothing. The configuration allows about 13000 nodes, and the root guide asks for bounded work on the data path. The validation time rule of this spec covers `validate` only.
**Suggested fix**: When scope 004 wires it, either answer admission from the topology snapshot and call this function only when the configuration, the journal or an observation changes, or index the observations by node once. Say which in the spec of scope 004. Owner: the architect of scope 004.

### 🟡 m7. A workspace dependency that nothing uses, against a written rule, `Cargo.toml:14`

**Problem**: `pool-control = { path = "crates/pool-control" }` sits under `[workspace.dependencies]` and no member takes it. Cargo 1.100.0 prints `warning: unused workspace dependency pool-control` on every command. `AGENTS.md:31` says a dependency is declared there "only when the current path uses it". Spec 0003 asks for the line now (`index.md:978`), and no note in spec 0001 or in `AGENTS.md` records the exception.
**Why it matters**: The warning fails no check, so it will sit in the output of every check until scope 004. A warning that is always there teaches people to stop reading warnings. And a spec that orders what the root guide forbids leaves the next agent to guess which one wins.
**Suggested fix**: Take the line out now and let scope 004 add it in the commit that adds the first use. Rule W5 reads the manifests of the members and does not need it. That means one sentence less in spec 0003. If you would rather keep it, add a dated note where the rule lives. Owner: the architect to choose, the developer to apply.

### 🟡 m8. Names accept control characters above U+007F, `crates/pool-config/src/topology.rs:456`

**Problem**: The check refuses U+0000 to U+001F and U+007F, as the spec lists them (`index.md:251`). The control characters U+0080 to U+009F pass, and so do the characters that change the direction of text, such as U+202E. Probe: a principal that holds U+009B and U+202E, and a `credential_file` that holds U+0085, are valid. The test at `topology.rs:894` pins this.
**Why it matters**: Nothing prints a name today, and the telemetry table keeps names out of default output. Scope 004 sends the backend login and database to PostgreSQL, and scope 009 will show identity routes to an administrator. U+009B is an escape lead on some terminals, and a direction override makes two entries look alike on a screen.
**Suggested fix**: Decide in the spec before a name reaches any output: either refuse the whole Unicode control class and the direction characters, or say in one sentence that the list is complete on purpose and that every later output escapes names. Owner: the architect.

### 🟡 t4. The canary tests put the secret text in values only, `crates/pool-config/tests/validate.rs:1446`, `apps/pool/tests/check_config.rs:2284`

**Problem**: For the new tables, the tests of `AC-4` place `CANARY-9f2c7b1e` in string values. No test places it in a key that is not in the schema inside an entry or inside `[control]`, in a table name, in an array entry or in a value of the wrong type.
**Why it matters**: I ran those positions by hand through the built binary, in both formats, and the canary came out in none. So the code is fine today. The positions go through the parser error path, which holds the text of the key, and only a test keeps a later change from printing it.
**Suggested fix**: One table driven test in each of the two files with the positions above. Owner: the tester.

### 🟡 t5. `LineIndex` has no test of its own, `crates/pool-config/src/lib.rs:579`

**Problem**: The index is tested only through `validate`. That covers CRLF, multi byte text and offsets near 256 and 512. It does not cover a stride boundary that falls inside a multi byte character, an offset at or past the end of the text, or a tab. The same rule also lives in `line_column` (`lib.rs:556`), and nothing compares the two.
**Why it matters**: My probes gave the right positions for all of these. The two functions have to agree for every offset, or a syntax finding and a value finding in the same file would count columns in two ways.
**Suggested fix**: One unit test that walks every offset of a few short texts (with CRLF, a tab, two, three and four byte characters, and a length that is a multiple of 256) and compares `LineIndex::line_column` with `line_column`. Owner: the tester.

### 🟡 t6. Nothing automatic keeps a clock or a file out of `pool-control`, `tools/repo-check/baseline/forbidden-source-patterns.txt`

**Problem**: The second half of `AC-11` is a `grep` that someone runs by hand. It prints nothing today. Adding `Instant::now()` to `crates/pool-control/src/` would fail no test and no guard.
**Why it matters**: That `pool-control` reads no clock and no file is what makes the four walks run as tests and what scope 004 builds on. Scope 004 puts the journal reader into this crate, which is exactly when a file read or a clock read is most likely to slip into the rule functions.
**Suggested fix**: When scope 004 decides where the reader lives, make the `grep` a guard, for example a list of patterns for the rule modules only. That is a change to rule W3, so it needs a sentence in the spec. Owner: the architect of scope 004.

## Nits

- ⚪ n1. `crates/pool-control/src/bootstrap.rs:160`: a cluster whose writer has no fitting facts is left out of the plan without a reason of its own. Validation makes that impossible today. A last check that the plan holds as many clusters as the configuration would make it impossible by itself.
- ⚪ n2. `crates/pool-control/src/admission.rs:188`: `expiry` is taken as given, so `Duration::MAX` makes every sample fresh. Probe confirmed. Scope 004 adds the key and should give it a range.
- ⚪ n3. `crates/pool-control/src/bootstrap.rs:84`: `BootstrapRefused` has private fields and no `compile_fail` test, unlike its four neighbours. The spec does not ask for one.
- ⚪ n4. `scripts/prove-checks.sh:288`: case 17 now edits the first line of the scope 004 file. It will fail, loudly, the day scope 004 leaves `planned`, and need a new target for every scope after that. A copy of a scope file made by the script would end that.
- ⚪ n5. `crates/pool-config/tests/validate.rs:2098`, `:2099`, `:2169`, `:2184`, `:2197`, `:2233`, `apps/pool/tests/check_config.rs:2351`, `:2373`: eight comment lines still call `0003 AC-15` proposed or not yet accepted. The criterion is accepted. The verification record counts three (`verify.md:539`, `:716`).
- ⚪ n6. `docs/specs/0003-control-and-session-data-model/verify.md:582`, `:590`: the count table of the tester says 91 tests for `tests/validate.rs` and 257 in all. The tree has 95 there, so the total is 261.
- ⚪ n7. `crates/pool-config/src/topology.rs:490`: the index is built and every value is looked up also for a valid file, which needs no position. It is linear and `AC-15` passes, so this only costs a little time.
- ⚪ n8. `crates/pool-config/src/lib.rs:206`, `:285`: `FindingCode` and `Location` gained variants and are not `#[non_exhaustive]`. Only `apps/pool` matches on them, so nothing breaks. Mark them before a crate outside the workspace can.
- ⚪ n9. `crates/pool-control/src/admission.rs:218`: the arm that maps `Ready` to `authority.unreadable` cannot be reached and stays closed. A `debug_assert!` there would show a later mistake in a test.
- ⚪ n10. `crates/pool-control/src/admission.rs:101`: an `EligiblePrimary` can be cloned and kept. Its documentation comment could say that it is the answer of one moment and that the caller asks again, as the spec does.
- ⚪ n11. `docs/specs/0003-control-and-session-data-model/verify.md:243`, `:705`: the record cites the checker's own model of 65536 cases as evidence. That model is not in the tree. The committed test has 8192 cases and never varies the journal timeline, a quarantined writer or the configured system identifier together with the rest. Commit the larger model, or say in the record that it was a probe.
- ⚪ n12. `crates/pool-control/src/observation.rs:43`, `:98`, `:103`, `:108`, `crates/pool-control/src/authority.rs:143`, `:148`, `:153`: seven public accessors are not in the interface block of the spec (`since_origin`, three on `Observation`, three on `Authority`). And three rows of the Records table (`index.md:104` to `:106`) name no type, so `AC-14` cannot be checked for them as written. The types are `StoredAuthority`, `ApprovedTopology` and `Quarantine`.
- ⚪ n13. `docs/specs/0002-standards-and-tooling/index.md:327`, `:501`, `:831`: the row for W4, its message text and the row for case 17 describe the first build, and the dated notes that correct them sit further down. A reader of only the row is misled. A short pointer in each row would fix it. The same holds for `docs/specs/0001-initial-architecture/index.md:462`, which says configuration file content never appears in a log, while spec 0003 lets identifiers from the file into log fields (`index.md:137`). The next bullet of spec 0001 allows that when a spec permits the field, so the two can stand together, but only a note says which one wins.
- ⚪ n14. `docs/glossary.md`: "approved topology", "authority witness", "membership decision" and "reconcile" are used in other rows and have no row of their own. "Fresh" (`:37`) does not carry the two rules of spec 0003 for a sample from the future and for a tie.
- ⚪ n15. `docs/specs/0003-control-and-session-data-model/index.md:729`: the text says the functions look at "no other node" than the configured ones. `crates/pool-control/src/admission.rs:273` also reads the sample of an approved writer that the configuration no longer lists. The answer is closed either way (`authority.config_conflict` is already there); only the list of reasons can differ.
- ⚪ n16. `AGENTS.md:8`, `:32` and `CLAUDE.md`: both still say the workspace holds two packages and that `crates/pool-control` does not exist. The spec gives this to the "Document it" box (item 6 of its document corrections). It is listed here so that it is not lost. The same box owns the risk register: the case that spec 0003 names as still open for `R-19`, a raw file copy of the journal or a rollback of the whole machine (`index.md:1049`), is not in the register yet.
- ⚪ n17. Small things in tests: `crates/pool-control/tests/walkthroughs.rs:741` asserts only that a reason name starts with `bootstrap.`; `crates/pool-config/tests/validate.rs:2325` has a `continue` that would pass with an empty loop; `crates/pool-config/tests/validate.rs:1484` cannot see a read of a file that exists; `crates/pool-config/src/topology.rs:920` repeats the rule of the code it tests.
- ⚪ n18. `docs/specs/0003-control-and-session-data-model/verify.md:445`, `:447`: the developer's rows for `AC-15` and `AC-16` say "pass" from probes, before the `ac15_` and `ac16_` tests existed. The rows say so themselves, and the checker ran the tests later. A reader who stops at the table could take it for a test result.

## The items the brief listed as known

| Item | State | Where it is in this file |
|---|---|---|
| a. Unused workspace dependency | Minor | m7 |
| b. W4 lockfile entry runs to the end of the file | Minor | m4 |
| c. U+0080 to U+009F in names | Minor | m8 |
| d. Two million findings from one file | Minor | m5 |
| e. `AC-15` is a wall clock limit | Fine as it is | The limit is far from both sides: all 95 tests of `tests/validate.rs` took 1.7 seconds here, the old code took minutes. The spec names the cost under Consequences. No finding |
| f. The `provisional: F3` marker, the count in the record, the `(proposed)` comments | Marker gone, the rest nits | `crates/pool-control/src/lib.rs` no longer holds the marker. n5 for the comments, which are eight lines and not three. n6 for the count: 91 should be 95 and the total 261 |
| g. Milestone 3 built before milestones 1 and 2 passed, tests started before `Build it` | Nothing left open on the final tree | The last commit that changed code or tests is `4c61e3b`. The full verification ran on `4c61e3b`. After it only `verify.md` and `docs/scope/` changed (`git diff --stat 4c61e3b..be06393`). So every line of the final tree was built and tested together at least once after the order was mixed. What is still not run is CI on GitHub and nothing else |

## Strengths

- The rule that matters most is small and readable. `admit_ready` is ninety lines, follows the order of the spec table one check at a time, and has one place that returns `Open`.
- Closed is the default everywhere: a journal that is not consistent, a witness that is absent or different, a hold, an unknown cluster, a tie, a sample from the future for the writer, an age equal to the expiry.
- The test of the small domain computes its answer with a rule of its own, in another shape than the code, and pins the count of open answers at 130 of 8192. A change that opens one more case fails it.
- No text from the file can reach a finding by construction, not by care: the types that carry findings have no field that could hold it.
- The position lookup is linear to build and a binary search to use, and it gave the right answer for every odd input I tried.
- The change to spec 0001 behaviour is one line of code, and the old fixtures keep their results.

## Test coverage

The test signal is `configured`: the built in `cargo test` harness, with no dependency for tests. The suite is strong where it counts most. Every criterion that a test can prove has a real test, and the old tests of spec 0001 were not weakened: above the new part, `apps/pool/tests/check_config.rs` changed only in its header comment, and `crates/pool-config/tests/validate.rs` changed only an import and one exact `Debug` text that grew with the new fields.

| Criterion | Proving tests | Verdict |
|---|---|---|
| `0003 AC-1` | `validate.rs:1050`, `:1062`, `:1124`; `check_config.rs:1953`, `:2310` | Real |
| `0003 AC-2` | `validate.rs:1134`, `:1218`, `:1372`, `:1607` to `:1947`; `check_config.rs:1970`, `:2192` | Real for the fixtures and the code table. Gaps t1 and t2 |
| `0003 AC-3` | `validate.rs:1183`, `:1950`, `:2232`; `check_config.rs:2068` | Real for the findings. Weak for "the only way to obtain a `ValidatedConfig`" (t3) |
| `0003 AC-4` | `validate.rs:1446`; `topology.rs:929`; `check_config.rs:2107`, `:2284` | Real for values. Other positions by hand only (t4) |
| `0003 AC-5` | `validate.rs:1495`, `:1529`, `:2030`, `:2083` | Real |
| `0003 AC-6` | `identifier.rs:171` to `:365`; `generation.rs:88` to `:196`; documentation tests | Real. The "does not compile" half is weak (t3) |
| `0003 AC-7` | `walkthroughs.rs:213`, `:705`, `:1480`, `:1541`, `:1588` | Real |
| `0003 AC-8` | `walkthroughs.rs:292`, `:1401`, `:1480` | Real |
| `0003 AC-9` | `walkthroughs.rs:345`, `:1047` to `:1398` | Real. Row o is reached only with a journal built by hand (m1) |
| `0003 AC-10` | `walkthroughs.rs:530`, `:1401` | Real |
| `0003 AC-11` | `walkthroughs.rs:605`, `:850`, `:928`, `:974`, `:993`; `admission_model.rs:320` | Real for freshness, to the nanosecond. The `grep` half has no guard (t6) |
| `0003 AC-12` | `admission_model.rs:204`, `:367`; four documentation tests | Real. The model is its own rule and not a copy of `admit_ready` |
| `0003 AC-13` | `tools/repo-check/src/workspace.rs:761`, `:776`, `:1114` to `:1275` | Real as unit tests. No proof case, one known hole (m4) |
| `0003 AC-14` | None. The spec says it is checked by reading | Not a test matter |
| `0003 AC-15` | `validate.rs:2168`, `:2183`, `:2196`, `:2232`; `check_config.rs:2372` | Real. One wall clock sample: a slowdown of up to about 25 times passes, a real one runs for minutes first |
| `0003 AC-16` | `walkthroughs.rs:1665`, `:1674`, `:1689` | Real |

File paths in the table are short for `crates/pool-config/tests/validate.rs`, `apps/pool/tests/check_config.rs`, `crates/pool-config/src/`, `crates/pool-control/tests/`.

What no test covers, in the order I would add it: the walk from row d to row o through the real functions (m1), a repeated cluster `id` (t1), the three skip rules (t2), a lockfile proof case (m4), the canary in keys and names (t4), `LineIndex` against `line_column` (t5).

## Not run in this review

- `scripts/check.sh all`, `scripts/prove-checks.sh`, the Linux build and the fuzz target. The lead ran the first two on `main`.
- CI on GitHub. The record marks it `NOT_RUN` with the user as owner, which is honest. The Linux evidence in the record is a Docker container on this machine, and it is labelled as that.
- The test reviewer and the document reviewer ran no `cargo` command. Where this file says a changed line survived the tests, I ran it.
