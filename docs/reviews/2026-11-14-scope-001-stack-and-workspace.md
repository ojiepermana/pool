# Review, main (scope 001, stack and workspace), 2026-11-14

**Reviewed by**: claude-fable-5-2 (author on claude-opus-5-5)
**Scope**: 30 files, the range `a82cf67..02bdcbe` on `main` (26 code, manifest and fixture files, plus `Cargo.lock` for its package set and 3 docs). `graphify-out/` was ignored.
**Verdict**: Changes requested

## Summary

The change adds a two package workspace (`pool`, `pool-config`) and one offline command, `pool check-config`. The code follows spec 0001 closely. Output is built only from constants, static key paths and numbers, the read is bounded, and `enabled = true` is refused for both reserved capabilities in every spelling I tried. I found no blocker. There is one major finding: a reserved capability written as an array (for example `result_cache = [false]`) is accepted as a valid file, where the spec says it is `config.schema`. There are six minor findings: one small deviation from the value sourcing table, and five gaps in tests or evidence. The rest are nits, most of them one sentence fixes in the spec.

The review ran in two passes on the same reviewer model. A first pass read the whole range and wrote the minor findings and nits. A second pass checked each of those against the code and the spec, and found the major one by running the binary.

I ran the checks again on the merged tree (`02bdcbe`, Rust 1.100.0, macOS on Apple silicon): `cargo fmt --all --check` exit 0, `cargo clippy --workspace --all-targets --locked -- -D warnings` exit 0, `cargo test --workspace --locked` exit 0 (3 + 64 + 3 + 57 tests passed, seen in both passes), `cargo build --locked --release -p pool` exit 0. The Linux build (AC-2) and the two Linux only tests were not run. That is a manual step for you and not a finding.

## Major

### 🟠 A reserved capability written as an array is accepted as valid, `crates/pool-config/src/lib.rs:207`

**Violates**: build notes, "Which code a bad value gets" (`docs/specs/0001-initial-architecture/index.md:416`: "`result_cache` or `automatic_promotion` that is not a table" is `config.schema`). Configuration rules 4 and 9 (`index.md:230`, `index.md:235`: no warning mode, the file is judged as a whole). `0001 AC-7` in spirit (invalid configurations fail). The `AGENTS.md` invariant "fail closed".

**Problem**: `RawCapability` (`lib.rs:207` to `211`) derives `Deserialize`. A derived structure accepts two shapes: a table, and a sequence that gives its fields by position. The `toml` crate hands an array to the structure as a sequence. So an array whose first element is a boolean is read as `enabled`, and everything after the first element is dropped without a check. I ran it on the merged tree:

```text
$ printf 'config_version = 1\nresult_cache = [false]\n' > a.toml; pool check-config --config a.toml
configuration valid
config_version: 1
result_cache: disabled
automatic_promotion: disabled
exit=0

$ printf 'config_version = 1\nautomatic_promotion = [false, "CANARY-9f2c7b1e", 3]\n' > b.toml
$ pool check-config --config b.toml --format json
{"output_version":1,"command":"check-config","result":"valid","config_version":1,"capabilities":{"result_cache":"disabled","automatic_promotion":"disabled"},"findings":[]}
exit=0

$ printf 'config_version = 1\nresult_cache = [\n  false,\n  { anything = "goes", here = [1, 2, 3] },\n]\n' > c.toml
$ pool check-config --config c.toml
configuration valid
exit=0

$ printf 'config_version = 1\nresult_cache = [true]\n' > d.toml; pool check-config --config d.toml
configuration invalid
findings: 1
config.unsupported_capability at result_cache.enabled: this capability is not supported by this build; remove the key or set it to false
exit=1
```

The first three must be `config.schema` with exit 1. The fourth is rejected, but with the wrong code, and it names a key (`result_cache.enabled`) that the file does not contain.

The existing test did not catch it by luck of its inputs. `ac7_a_reserved_capability_that_is_not_a_table_is_a_schema_finding` (`crates/pool-config/tests/validate.rs:461`) tries `automatic_promotion = []` and `[[result_cache]]`. The empty array fails because the first field is missing, and the array of tables fails because a table is not a boolean. An array that starts with a boolean is the one shape that passes, and no test has it.

**Why it matters**: Nothing unsafe happens today. The capability stays `disabled`, `[true]` is still refused, and no file text reaches the output. That is why this is not a blocker. It is more than minor for three reasons. The one job of this command is to say which files are valid, and it says "valid" for a file that the spec lists by name as invalid. The file can carry any content after the first element (keys, strings, nested tables) and none of it is judged, which is the "unknown keys are rejected at every level" rule failing open. And the cause is a property of every derived structure, so each table that later slices add (`[[backend]]`, listeners, identity maps) inherits it unless the rule is fixed here, in the slice that sets the configuration contract.

**Suggested fix**: Make the structures that are read from the file accept a table only. The smallest change is a hand written `Deserialize` for `RawCapability` whose visitor implements `visit_map` and nothing else, so an array becomes a type error from the `toml` crate, with its span. Step 4 then reports `config.schema` with a line and a column and no other code changes. Add cases to the test at `validate.rs:461` for both capabilities: `= [false]`, `= [true]` (exactly one finding, `config.schema`, not `config.unsupported_capability`), and an array with more elements. Add one case through the binary in `apps/pool/tests/check_config.rs` (exit 1). Add one sentence to "Validation order" step 4 in the spec: a structure read from the file accepts a table only, never an array of positional values. That keeps later slices from repeating it.

**How confirmed**: Ran the binary (output above). `cargo test --workspace --locked` passes with the defect in place.

## Minor

### 🟡 `config_version()` returns a constant, not the number from the file, `crates/pool-config/src/lib.rs:267`

**Violates**: build notes, "Value sourcing" (`docs/specs/0001-initial-architecture/index.md:492`).

**Problem**: `validate` checks the parsed number at `lib.rs:246`, then drops it (`Some(_) => {}` at `lib.rs:250`). At `lib.rs:267` it stores `SUPPORTED_CONFIG_VERSION`. So `ValidatedConfig::config_version()` (`lib.rs:36`) never returns a value that came from the file. The value sourcing table says it "comes from the file's `config_version` key".

**Why it matters**: The output is right today, because only `1` validates. The risk is later. If a build ever accepts a second version and nobody touches line 267, the report prints `config_version: 1` for a version 2 file and no test fails. This slice is the contract that later slices extend, so the source of the value should be the one the spec names.

**Suggested fix**: Bind the parsed number in the `match` at `lib.rs:241` and store that number in `ValidatedConfig`. Keep the comparison with the constant as the check.

**How confirmed**: Read. No test can tell the two apart today.

### 🟡 The parser depth limit and the "no location" branch have no test, `crates/pool-config/src/lib.rs:291`

**Violates**: review guide, test adequacy (branch not covered). Engineering standards, "Bound large payloads and auth/parser work explicitly". Build notes, "Findings" (`index.md:398` lists `config.syntax` with a line and column).

**Problem**: A file can reach the `None => Location::None` arm of `error_location`. A dotted key or a table header nested deeper than 80 levels makes the `toml` parser stop with an error that has no span. The report is then `config.syntax: the file is not valid TOML`, with no line and no column. No test reaches this arm, and no test feeds deep nesting of any kind.

I confirmed it by running. The file is `a` followed by `.a` 80 times and ` = 1` (81 levels):

```text
$ pool check-config --config d.toml      (81 levels)
configuration invalid
findings: 1
config.syntax: the file is not valid TOML
$ pool check-config --config d.toml      (80 levels)
config.schema at line 1, column 1: unknown key, or a value of the wrong type or range
```

I also fed 1 MiB files of deep nesting. None crashed and all took under 0.7 seconds in a debug build: 500000 `[` then 500000 `]` gives `config.syntax at line 2, column 85`; `{a=` 200000 times gives `config.syntax at line 1, column 245`; a dotted key and a table header 500001 levels deep both give `config.syntax` with no location.

**Why it matters**: Two things. First, a reachable output shape (a syntax code with no location) is never checked. Second, the fact that 1 MiB of brackets cannot overflow the stack rests on a limit inside the `toml` crate (`LIMIT = 80` in `src/de/parser/mod.rs:71` of `toml` 1.2.1). That limit goes away if anyone turns on the crate's `unbounded` feature. Neither the spec nor a test pins it. A stack overflow would end the command with a signal, which is the same kind of defect as exit status 101.

**Suggested fix**: Add in memory cases to `crates/pool-config/tests/validate.rs`: a dotted key 81 levels deep gives exactly one `Syntax` finding with `Location::None`; arrays and inline tables nested far past the limit give exactly one finding. Add one sentence under "Findings" in the spec that nesting deeper than the parser allows is `config.syntax` and may carry no location. Add one line under "Adopted dependencies" that the `unbounded` feature of `toml` stays off.

### 🟡 The `io.permission_denied` path has no test, `apps/pool/src/report.rs:30`

**Violates**: review guide, test adequacy (error handling branch not covered). `0001 AC-11` names the two other `io.*` codes only, and the spec waives this test at `index.md:420` and `index.md:617`.

**Problem**: The `PermissionDenied` arm of `IoFinding::from_kind` (`report.rs:30`), its code (`report.rs:38`) and its message (`report.rs:46`) are never run by `cargo test`. The spec says there is no test because the Linux build runs as root. That reason holds for the container. It does not hold for the workstation, where the suite also runs.

The behavior is right. I ran it by hand as a user who is not root (uid 502):

```text
$ chmod 000 noperm.toml; pool check-config --config noperm.toml        -> exit 3
error: io.permission_denied: the configuration file cannot be read: permission denied
$ pool check-config --config noperm.toml --format json                 -> exit 3
{"output_version":1,"command":"check-config","result":"error","config_version":null,"capabilities":null,"findings":[{"code":"io.permission_denied","key":null,"line":null,"column":null,"message":"the configuration file cannot be read: permission denied"}]}
```

**Why it matters**: The code and message are public contract text. A typo or a swapped arm would pass every test. I keep this at minor and not major because the spec chose to leave it out, the arm is one line of constants, and two sessions have now checked it by hand.

**Suggested fix**: Add a `#[cfg(unix)]` test that writes a file with mode `000`, tries to open it from the test itself, and returns early when that open succeeds (the root case). Otherwise it compares stdout and stderr byte for byte in both formats. Change the two spec sentences to match.

### 🟡 The column test cannot fail for a byte count, `crates/pool-config/tests/validate.rs:657`

**Violates**: build notes, "Validation order" (`index.md:433`, the column counts Unicode scalar values). Review guide, tests that assert nothing meaningful.

**Problem**: `ac7_column_counts_characters_not_bytes_before_the_error` uses the text `"é" = 1` on line 2 and accepts any column from 1 to 7. The parser points at the start of the unknown key, so the column is 1. A byte count and a character count both give 1. The comment says the unknown key starts after a multibyte key, but the multibyte key is the unknown key.

```text
$ printf 'config_version = 1\n"\xc3\xa9" = 1\n' > e.toml; pool check-config --config e.toml
config.schema at line 2, column 1: unknown key, or a value of the wrong type or range
```

**Why it matters**: The test name promises a rule that the test does not check. The rule itself is correct in the code and is checked by the unit test `position_counts_lines_and_scalars` (`lib.rs:331`), but only on the private function. Through `validate` nothing enforces it.

**Suggested fix**: Use an input where a multibyte character sits before the reported position on the same line, and assert the exact column. For example a line `x = "éé" junk` on line 2 reports `config.syntax at line 2, column 10` today (I ran it); a byte count would give 12.

### 🟡 Three criteria have no automated guard, `crates/pool-config/src/lib.rs:27`

**Violates**: review guide, test adequacy. `0001 AC-8` (second sentence), `0001 AC-12` (package set and `grep`), `0001 AC-13` (stop line and lint inheritance).

**Problem**: These parts conform today, and I checked each one again. But they were checked by hand or by a scratch crate that is not in the repository. Nothing in `cargo test` fails if someone adds `Default` or `Deserialize` to `ValidatedConfig`, adds a package to `Cargo.lock`, or starts a thread.

**Why it matters**: "`validate` is the only way to obtain a `ValidatedConfig`" is the `SEC-12` guarantee of this slice. Until scope 002 adds CI, a regression in any of the three passes the only automated check you have.

**Suggested fix**: For AC-8, add `compile_fail` documentation tests on `ValidatedConfig` (a struct literal, `Default::default()`). They run in the built in harness and add no dependency. For AC-12 and AC-13, write the `grep`, the package set check and the stop line check into scope 002 as required CI steps, so they are not lost.

### 🟡 The verification record does not cover the merged tree, `docs/specs/0001-initial-architecture/verify.md:181`

**Violates**: build notes, "Change path" (`index.md:675`, checks pass on the merged tree). Testing strategy, "Acceptance evidence" (record the revision and the result).

**Problem**: `verify.md` records revision `5f44d88` and the counts "`check_config` 12, `validate` 13". The merge `02bdcbe` brought in 98 more tests (`5ad687d`, `fae4bc6`). No section records a run of the merged tree, and the "Test it" milestone has no section of its own.

**Why it matters**: The scope box "Test it" is ticked, but the record you link from the scope does not show those tests passing at any revision.

**Suggested fix**: Add a short section to `verify.md` with the merged revision and the counts. I observed 3 + 64 + 3 + 57 passed on macOS at `02bdcbe`, with `fmt` and `clippy` at exit 0. Two more tests exist for Linux only and have never run.

## Nits

- ⚪ `apps/pool/src/cli.rs:61`, a not UTF-8 argument written with an equals sign (`--format=<bytes>`, `--config=<bytes>`) is `unexpected argument`. The code matches the literal spec text at `index.md:265`. Add one sentence to the spec that says so, and add `--format=<bytes>` to the list at `apps/pool/tests/check_config.rs:1629`, which today pins only `--config=<bytes>`. See "Known open items" below.
- ⚪ `apps/pool/src/cli.rs:64`, `--help=x` and `-h=x` after `check-config` print the help and exit 0 (pinned by `check_config.rs:792`). As the first argument `--help=x` is `unknown command`. The spec row at `index.md:274` can be read either way. Add one example line to the spec.
- ⚪ `apps/pool/src/main.rs:88`, the fallback of the conversion is `u64::MAX`, which would remove the read limit. It cannot happen on a 64 bit target, but a fallback should keep the bound and not drop it.
- ⚪ `apps/pool/src/main.rs:90`, a FIFO with no writer blocks the command with no time limit. I ran it: `mkfifo fifo; pool check-config --config fifo` was still blocked after 2 seconds. The spec is silent. Reading special files is also what makes `/dev/zero` (AC-12) and `/dev/stdin` work, so I would keep the behavior and add one sentence to the spec that says special files are read like any file and may block.
- ⚪ `apps/pool/src/cli.rs:74`, the block that takes a value is written twice (lines 74 to 83 and 90 to 99). A small private helper would remove the copy.
- ⚪ `crates/pool-config/src/lib.rs:273`, `capability_state` returns `Disabled` even right after it records an "enabled" finding. This is safe only because of the check at `lib.rs:263`. Add a comment that says so.
- ⚪ `crates/pool-config/src/lib.rs:170`, the messages repeat `1048576` and `1` as text, apart from the constants at `lib.rs:14` and `lib.rs:17`. A test pins the text, so this is safe today. A short comment that links them would help the next editor.
- ⚪ `crates/pool-config/tests/validate.rs:828`, `assert_eq!(findings(&bytes), findings(&bytes))` on a pure function proves very little.
- ⚪ `crates/pool-config/tests/validate.rs:896`, the name says "holds only states" but the test only checks that the text contains `Disabled`.
- ⚪ `apps/pool/tests/check_config.rs:1478`, the field order loop uses `>=` and always finds the first finding with a given code, so it does not check a second finding with the same code. The byte for byte tests already cover this, so the loop adds little.
- ⚪ `docs/specs/0001-initial-architecture/index.md:52`, the sentence "The lifecycle status remains Proposed" is stale now that the status is `In Progress`. The lead fixes it at closing. The lines in `CLAUDE.md` ("documentation only", "stays Proposed") and `AGENTS.md` ("There is no Cargo workspace yet") are stale in the same way and belong to the "Document it" step.

## Strengths

- No text from the file can reach any output. The only use of a `toml` error is `error.span()` at `crates/pool-config/src/lib.rs:289`. Its `Display` and `message()` are never called, never stored, and `Rejection` has no `source()`. `Rejection` and `Finding` hold only an enum, a `&'static str` and two `u32`, so the `Debug` derives are safe by type and not by care.
- The read is truly bounded (`take` then `read_to_end` at `apps/pool/src/main.rs:90`), and `validate` checks the length again on its own, so the library does not trust its caller.
- Every write goes through a locked handle and returns its error. A closed pipe on stdout or stderr ends with exit 3 and no panic. I saw `pool --help | true` exit 3.
- The tests drive the real binary and compare stdout and stderr byte for byte, in both formats, including field order on the raw JSON line.
- The manifests, the toolchain file and all fifteen fixtures equal the spec text exactly.

## Test coverage

Covered well: every fixture through `validate` and through the binary; all three spellings of a table for both capabilities, on and off; every row of the "Which code a bad value gets" table; CRLF; the size limit on both sides; UTF-8; the canary in values, keys, table names, comments, paths and typed arguments; every row of the usage error table with many variants; help and version; missing file, directory, path through a file, symlink; `/dev/zero` and `/dev/null`; closed pipes on stdout and stderr.

Not covered by any test:

- A reserved capability given as an array that starts with a boolean (major finding above). The suite passes with the defect in place.
- The `io.permission_denied` arm (minor finding above).
- The "no span" arm of `error_location` and any deep nesting (minor finding above).
- The column rule through `validate` (minor finding above).
- The constructor guarantee of AC-8, the package set and `grep` of AC-12, the stop line of AC-13 (minor finding above).
- `--format=<not UTF-8 bytes>` as one argument (nit above).

Never run anywhere yet: the two `#[cfg(target_os = "linux")]` tests (`check_config.rs:525` and `check_config.rs:1768`). I read both. They set stdout to `/dev/full`, leave stderr captured, and assert exit 3 and empty stderr. They look correct. Also `ac11_a_path_that_is_not_utf8_is_used_as_given` (`check_config.rs:1669`) takes its fallback branch on APFS, so the real claim (a path that is not UTF-8 is read and accepted) will first run on Linux.

## Conformance by criterion

| Criterion | Verdict | Where in code | Enforcing test |
|---|---|---|---|
| AC-1 | conforms | `Cargo.toml`, `rust-toolchain.toml`, `Cargo.lock` | none in the suite; by command (release build exit 0, `cargo metadata` lists `pool` and `pool-config`) |
| AC-2 | not judged | none | not run, Docker is off; manual step for you |
| AC-3 | conforms | `apps/pool/src/report.rs:124`, `report.rs:158` | `check_config.rs:183` |
| AC-4 | conforms | `crates/pool-config/src/lib.rs:273` | `validate.rs:64`, `validate.rs:337` to `372`; `check_config.rs:915` to `1010` |
| AC-5 | conforms | `lib.rs:252`, `lib.rs:278` | `validate.rs:75`, `validate.rs:375` to `396`; `check_config.rs:202`, `check_config.rs:1013` |
| AC-6 | conforms | `lib.rs:257`, `lib.rs:278` | `validate.rs:86`, `validate.rs:399` to `420`; `check_config.rs:202`, `check_config.rs:1035` |
| AC-7 | conforms for every listed fixture and made input; deviates for one row of "Which code a bad value gets" (a reserved capability that is an array, major finding) | `lib.rs:218` to `238`; `lib.rs:207` | `validate.rs:135`, `168`, `223`, `231`; `check_config.rs:253`, `289`, `1100`. No test for the array row beyond `validate.rs:461`, which misses it |
| AC-8 | conforms | `lib.rs:240` to `270`; private fields at `lib.rs:28` | order: `validate.rs:114`, `749`, `768`; `check_config.rs:218`, `1246`. Constructor part: by reading only |
| AC-9 | conforms | `lib.rs:288` (span only); all of `report.rs` | `validate.rs:245`, `837`, `859`, `879`; `check_config.rs:314`, `1299`, `1330`, `1369` |
| AC-10 | conforms | `report.rs:75` to `98`, `report.rs:156` | `check_config.rs:148` (raw line), `1395`, `1499` |
| AC-11 | conforms on macOS; the `/dev/full` clause is not judged | `apps/pool/src/cli.rs`, `apps/pool/src/main.rs:44` | `check_config.rs:345`, `363`, `447`, `571` to `831`, `1730`, `1753`; Linux: `525`, `1768` (never run) |
| AC-12 | conforms | `main.rs:87` | `check_config.rs:478`, `1586`; `grep` and package set by command only |
| AC-13 | conforms | `Cargo.toml:18`, `[lints]` in both package manifests | by command (`fmt`, `clippy`, `test` all exit 0 on the merged tree) |

Two deviations from the build notes remain: a reserved capability written as an array is accepted (major finding), and the value sourcing of `config_version` (first minor finding).

## The eight items you asked about

1. **Conformance to AC-1 to AC-13 and the build notes.** See the table. The argument grammar, exit codes, output text, finding codes, validation order and public boundary all match. I compared `pool --help` with the spec block byte for byte, and the three JSON examples in the spec with the binary output; all equal. Two deviations: a reserved capability written as an array is accepted as valid (major finding), and the value sourcing of `config_version`.
2. **`SEC-10` and `SEC-11`.** Conforms on the safety property: no input turns a capability on, and the array form `[true]` is refused too (with the wrong code, see the major finding). Left out, empty table and `enabled = false` all validate and report `disabled`. `enabled = true` is refused for each capability with its own fixture (`invalid-result-cache-enabled.toml`, `invalid-automatic-promotion-enabled.toml`) and in all three spellings. I also tried quoted keys, an escaped key name (`"enabled"`), a trailing comma in an inline table and a trailing comment; all were refused. `Enabled` with a capital is an unknown key. `SEC-11` speaks of logs, metrics and traces; this build writes none, and its command output holds no file content.
3. **No file content and no secret in any output or `Debug`.** Conforms. Every `Debug` derive: `ValidatedConfig`, `CapabilityState`, `Rejection`, `Finding`, `FindingCode`, `Location` hold no file text. `Format` and `IoFinding` hold none. `Invocation` holds the path you typed, and it is never printed. The two raw structures that do hold parsed values (`RawConfig`, `RawCapability`) derive only `Deserialize`. Third party error text cannot reach stdout, stderr, JSON or `Debug`: the `toml` error is borrowed for its span and then dropped. No `unwrap`, `expect`, `panic!` or print macro exists in `apps/*/src` or `crates/*/src`. I ran all fifteen fixtures in both formats through the release binary and searched for `CANARY`, `9f2c7b1e`, `backend_password` and `not-a-real`: no hit.
4. **Read limit and no socket, thread or runtime.** Conforms. `File::open(path)?.take(1048577).read_to_end(..)`. `/dev/zero` ends in 0.005 seconds with `config.too_large`. A directory gives `io.read_failed`. Symlinks are followed and a dangling one is `io.not_found` (the spec is silent, a test pins it). A FIFO blocks (the spec is silent, nit above). The release binary imports no socket call and no `pthread_create`.
5. **`unsafe_code`, package set, toolchain.** Conforms. `unsafe_code = "forbid"` is at `Cargo.toml:19` and both packages have `[lints] workspace = true`. `Cargo.lock` holds 18 names: `pool`, `pool-config` and exactly the sixteen the spec lists. `rust-toolchain.toml` equals the spec text.
6. **Crate boundaries.** Conforms. `apps/` holds only `pool`, `crates/` holds only `pool-config`. There is no `pool-core`, `pool-control`, `pool-protocol`, `pool-cli` or `common`. `pool-config` depends on `serde` and `toml` only. No package has `dev-dependencies` or a build script.
7. **Test quality.** Good overall. The tests check exit status, exact bytes and the absence of secrets on the real binary. One test claims a rule it does not fully check: `validate.rs:461` ("not a table") passes while an array that starts with a boolean is accepted. Weak or tautological tests: `validate.rs:657`, `validate.rs:828`, `validate.rs:896`, `check_config.rs:1478`. Gaps are listed under "Test coverage".
8. **General rubric.** Argument order, repeats, `--`, `-`, empty values, the first equals sign, help after an error, exit code mapping, JSON validity and field order, stdout against stderr, broken pipe, and the fixed order of findings are all correct and all tested. Error handling is complete: every `io::Result` is returned to `main`, which maps a failed write to exit 3. The code is small and clear.

## Known open items

1. **`--format=<not UTF-8 bytes>` gives `unexpected argument`.** Severity: nit. The code is right and the spec needs one sentence. The rule at `index.md:265` speaks of "an argument" that is not valid UTF-8 "as the value of `--format`". In the equals form the argument is not the value; it holds the option name too, and the parser cannot read the name without decoding the argument. So "anywhere else it is an unexpected argument" applies. Changing the code would need byte level splitting for no gain, and would make `--format=<bytes>` differ from `--config=<bytes>`, which the spec already rejects. Both forms exit 2 with constant text, so nothing unsafe happens either way. I ran it: `pool check-config --config a $'--format=\xff'` gives exit 2 and `error: unexpected argument`. Recommendation: add the sentence "A not UTF-8 argument that carries an equals sign is an unexpected argument" to the spec, and add the case to the test at `check_config.rs:1629`.
2. **`config_version()` returns the constant; `--config=<not UTF-8 bytes>` is `unexpected argument`.** The first part is a minor finding (first one above); fix the code, it is a three line change. The second part conforms to the spec as written ("accepted only as the separate argument that follows `--config`"), is pinned by `check_config.rs:1632`, and needs no code change. It is covered by the same spec sentence as item 1.
3. **Linux build and the two Linux tests never ran.** Not a finding. I did not run Docker. I read both tests and they look correct. AC-2 and the `/dev/full` clause of AC-11 stay not judged.
4. **Stale sentence in `## Decision`.** Nit, listed above. The lead fixes it at closing.

## What I ran

All on the merged tree `02bdcbe`, with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Scratch inputs were kept outside the repository and removed. `target/` was removed at the end.

- `cargo test --workspace --locked`: exit 0; `pool` unit 3, `check_config` 64, `pool-config` unit 3, `validate` 57.
- `cargo fmt --all --check`: exit 0. `cargo clippy --workspace --all-targets --locked -- -D warnings`: exit 0.
- `cargo build --locked --release -p pool`: exit 0. `cargo metadata --no-deps --format-version 1 --locked`: `pool`, `pool-config`, 2 members.
- `grep -rnE 'std::net|TcpListener|TcpStream|UdpSocket|UnixListener|UnixStream|tokio|std::thread' apps crates --include='*.rs'`: prints nothing.
- `nm -u target/release/pool` searched for socket, connect, listen, accept, `getaddrinfo`, `pthread_create`: none imported.
- Second pass, debug build of `pool` at the same tree: twelve made files that give a reserved capability or `config_version` a value that is not a table or not an integer. Arrays that start with a boolean gave the results quoted in the major finding. An empty array, an array of tables, a nested array, a boolean, a string, a date, `config_version = [1]` and `enabled = [false]` all gave `config.schema` with exit 1, as the spec asks.
- The probes quoted in the findings above, plus a byte order mark at the start of a file (accepted; an error on line 1 reports column 2 because the mark counts as one character, which matches the column rule as written) and stdout closed with `>&-` (exit 0, because the Rust runtime reopens a closed standard stream on `/dev/null`).
