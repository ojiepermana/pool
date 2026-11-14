# 0001. Verification record

This file holds the acceptance evidence for [spec 0001](index.md), as the [testing strategy](../../testing/strategy.md#acceptance-evidence) asks. Each milestone keeps its own section. A case that was not run, or could not be run, is marked as such.

## Milestone 3: Scaffold from the decision

Build plan steps 1 to 5 of spec 0001 (`/develop stack and workspace`), recorded by the developer session `dev-f1-build` on 2026-11-14.

**Revision**: `d493fca874cc4b78a19bc7d3036cdbe3a13cca9e` (the last code commit, step 3; this file and the steps 4 and 5 commits add no code).

**Workstation**: macOS on Apple silicon. This proves formatting, lints, tests and the fixture commands. It is not the Linux build (AC-2).

```text
$ rustc -vV
rustc 1.100.0 (0ef1bc386 2026-11-09)
binary: rustc
commit-hash: 0ef1bc386059596dada8fb65b50425aab2ea637a
commit-date: 2026-11-09
host: aarch64-apple-darwin
release: 1.100.0
LLVM version: 23.1.5
```

The local default toolchain is `stable` 1.99.1. The 1.100.0 above was selected by `rust-toolchain.toml` (rustup installed it on first use), which shows the file selects the toolchain (AC-1).

**Locked dependencies** (`Cargo.lock`, two workspace packages plus sixteen third party packages, the exact set the spec names):

| Package | Version |
|---|---|
| `pool` | 0.1.0 (workspace) |
| `pool-config` | 0.1.0 (workspace) |
| `itoa` | 1.0.19 |
| `memchr` | 2.8.3 |
| `proc-macro2` | 1.0.107 |
| `quote` | 1.0.47 |
| `serde` | 1.0.229 |
| `serde_core` | 1.0.229 |
| `serde_derive` | 1.0.229 |
| `serde_json` | 1.0.154 |
| `serde_spanned` | 1.1.3 |
| `syn` | 3.0.10 |
| `toml` | 1.2.1+spec-1.1.0 |
| `toml_datetime` | 1.1.3+spec-1.1.0 |
| `toml_parser` | 1.1.5+spec-1.1.0 |
| `unicode-ident` | 1.0.27 |
| `winnow` | 1.0.5 |
| `zmij` | 1.0.23 |

**Fixtures**: the fifteen files in `crates/pool-config/tests/fixtures/`, exactly as listed under [Fixtures](index.md#fixtures). The canary `CANARY-9f2c7b1e` is fake. The not UTF-8, too large and at limit inputs are made at run time by the tests and are not committed.

### UI / manual

- [ ] **Linux build (AC-2)**: `BLOCKED`
  - **reason**: `docker info` exited 1 on 2026-11-14 (the Docker daemon is not running), so the build was not run, as the spec asks. A build on macOS does not meet AC-2.
  - **owner**: user
  - **Prerequisites**: Docker Desktop started, so `docker info` exits 0. Every change committed (`git status --short` empty). Run from the repository root. The image is about 320 MB; the build runs under `linux/amd64` emulation and takes a few minutes.
  - **Command** (exactly as under [Linux build](index.md#linux-build)):

    ```bash
    git archive --format=tar HEAD | docker run --rm -i --platform linux/amd64 \
      -e RUSTUP_TOOLCHAIN=1.100.0 -e CARGO_INCREMENTAL=0 -e CARGO_PROFILE_DEV_DEBUG=0 \
      rust:1.100.0-slim-bookworm@sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37 \
      sh -euc '
    mkdir /work && tar -xf - -C /work && cd /work
    grep -q "channel = \"1.100.0\"" rust-toolchain.toml
    rustc -vV
    rustc -vV | grep -qx "host: x86_64-unknown-linux-gnu"
    cargo build --locked --release -p pool
    cargo test --locked --workspace
    sha256sum target/release/pool
    ./target/release/pool --version
    for f in crates/pool-config/tests/fixtures/*.toml; do
      case "$f" in
        */valid-*) want=0 ;;
        *) want=1 ;;
      esac
      for m in human json; do
        rc=0
        ./target/release/pool check-config --config "$f" --format "$m" >/tmp/out 2>/tmp/err || rc=$?
        echo "== $f $m exit=$rc"
        if grep -q "CANARY-9f2c7b1e" /tmp/out /tmp/err; then echo "canary leaked"; exit 1; fi
        cat /tmp/out /tmp/err
        if [ "$rc" -ne "$want" ]; then echo "unexpected exit $rc, wanted $want"; exit 1; fi
      done
    done
    echo "linux build check passed"
    '
    ```

  - **Expected**: exit status 0; `rustc -vV` output contains `host: x86_64-unknown-linux-gnu`; the test run passes (including `a_failed_write_exits_3_without_stderr`, the `/dev/full` part of AC-11); one `== <fixture> <format> exit=<n>` block per fixture and format, with exit 0 for `valid-*` and 1 for the rest; no `canary leaked` line; last line `linux build check passed`.
  - **Criteria**: `0001 AC-2`, plus the Linux only part of `0001 AC-11`.
  - **Record afterwards**: the image digest (`sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37`), the `sha256sum target/release/pool` line, `git rev-parse HEAD`, and the result.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

| Command | Expected | Observed | Result |
|---|---|---|---|
| `cargo fmt --all --check` | exit 0 | exit 0 | pass |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | exit 0, no warning | exit 0, no warning | pass |
| `cargo test --workspace --locked` | exit 0 | exit 0. `pool` unit 3, `check_config` 12, `pool-config` unit 3, `validate` 13, all passed | pass |
| `cargo build --locked --release -p pool` | exit 0 | exit 0 | pass |
| `cargo metadata --no-deps --format-version 1` (package names) | `pool`, `pool-config` | `['pool', 'pool-config']` | pass |
| `grep -rnE 'std::net\|TcpListener\|TcpStream\|UdpSocket\|UnixListener\|UnixStream\|tokio\|std::thread' apps crates --include='*.rs'` | prints nothing | prints nothing (exit 1) | pass |
| `./target/release/pool check-config --config /dev/zero` | exit 1, `config.too_large` | exit 1, `config.too_large: the file is larger than 1048576 bytes` | pass |
| `ls -d apps/pool-cli crates/pool-protocol crates/pool-core crates/pool-control` | none exists | `No such file or directory` for all four | pass |
| `grep unsafe_code Cargo.toml` and `[lints] workspace = true` in both package manifests | present | present | pass |

Fixture commands from [Linux build](index.md#linux-build), workstation part:

```text
$ cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml; echo "exit=$?"
configuration valid
config_version: 1
result_cache: disabled
automatic_promotion: disabled
exit=0
$ cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/invalid-result-cache-enabled.toml; echo "exit=$?"
configuration invalid
findings: 1
config.unsupported_capability at result_cache.enabled: this capability is not supported by this build; remove the key or set it to false
exit=1
$ cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/invalid-automatic-promotion-enabled.toml --format json; echo "exit=$?"
{"output_version":1,"command":"check-config","result":"invalid","config_version":null,"capabilities":null,"findings":[{"code":"config.unsupported_capability","key":"automatic_promotion.enabled","line":null,"column":null,"message":"this capability is not supported by this build; remove the key or set it to false"}]}
exit=1
$ cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/secret-unknown-key.toml; echo "exit=$?"
configuration invalid
findings: 1
config.schema at line 2, column 1: unknown key, or a value of the wrong type or range
exit=1
```

### Acceptance-criteria coverage

| Criterion | Evidence | Result |
|---|---|---|
| `0001 AC-1` | Release build, `cargo metadata` and `rustc -vV` above. `Cargo.lock` committed in step 1 | pass on the workstation |
| `0001 AC-2` | Linux build under UI / manual (step 5) | blocked (Docker daemon not running), owner: user |
| `0001 AC-3` | `check_config::valid_fixtures_report_valid_in_both_formats` | pass |
| `0001 AC-4` (`SEC-10`) | `validate::valid_fixtures_are_accepted_with_capabilities_disabled`, `check_config::valid_fixtures_report_valid_in_both_formats` | pass |
| `0001 AC-5` (`SEC-10`) | `validate::result_cache_enabled_is_refused`, `validate::enabled_true_is_refused_in_every_table_spelling`, `check_config::reserved_capabilities_enabled_are_refused` | pass |
| `0001 AC-6` (`SEC-10`) | `validate::automatic_promotion_enabled_is_refused`, `check_config::reserved_capabilities_enabled_are_refused` | pass |
| `0001 AC-7` | `validate::other_invalid_fixtures_have_the_listed_finding`, `validate::bad_values_get_the_listed_code`, `validate::crlf_line_endings_give_the_same_lines`, `validate::not_utf8_is_refused`, `validate::size_limit_is_inclusive`, `check_config::other_invalid_fixtures_fail_with_the_listed_finding`, `check_config::made_inputs_check_utf8_and_the_inclusive_size_limit` | pass |
| `0001 AC-8` (`SEC-12`) | `validate::multiple_findings_are_reported_in_order`, `check_config::multiple_findings_are_reported_in_one_run`. `ValidatedConfig` has private fields and no constructor, `Default` or `Deserialize` | pass |
| `0001 AC-9` (`SEC-4`, `SEC-11`) | `validate::secret_fixtures_never_reach_the_rejection_text`, `check_config::secret_fixtures_never_leak_the_canary` | pass |
| `0001 AC-10` | Exact JSON comparisons in `check_config` (valid, invalid, error), field order checked on the raw line | pass |
| `0001 AC-11` | `check_config::usage_errors_exit_2_with_two_lines_on_stderr`, `help_and_version`, `unreadable_paths_exit_3`, `arguments_that_are_not_utf8`, `a_value_after_config_is_always_the_path`. The `/dev/full` case (`a_failed_write_exits_3_without_stderr`) runs only on Linux | pass on the workstation; `/dev/full` part waits for the Linux build |
| `0001 AC-12` | `grep` and `Cargo.lock` set above, `check_config::dev_zero_is_read_only_up_to_the_limit`, `/dev/zero` command above | pass |
| `0001 AC-13` | Stop line `ls`, lints inheritance, fmt, clippy and test above | pass |

`io.permission_denied` has no test, as the spec says.

## Full verify: `/check verify stack and workspace`

Recorded by the checker session `chk-f1-verify` on 2026-11-14. The checker did not write the code. Every command below was run again from this session's own checkout, and each criterion was checked against the real `pool` binary, not only through the tests. The developer's record above is kept as it was.

This run is for revision `5f44d88`. The tester's tests and the review fixes came after it. For the current tree, read [Full verify, second run](#full-verify-second-run-merged-tree-after-the-review-fixes) below, which replaces the results of this section. The text here is kept as the record of what was true at `5f44d88`.

**Verdict**: `BLOCKED`. Eleven criteria are met. `0001 AC-2` and the Linux clause of `0001 AC-11` could not be run because the Docker daemon is off. Nothing failed.

**Revision**: `5f44d88bdc28a6bf958ab02b0ca3d07bee866cd9` (`main` after the scaffold merge). `git diff d493fca 5f44d88` shows no change under `apps/`, `crates/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` or `.gitignore`, so this is the same code the developer recorded.

**Checkout**: `git status --short` was empty and there was no `target/` directory before the first command, so the build started clean.

**Workstation**: macOS on Apple silicon, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, host `aarch64-apple-darwin`. `rustup show active-toolchain` printed `1.100.0-aarch64-apple-darwin (overridden by '<checkout>/rust-toolchain.toml')`. This is not the Linux build.

**Binary under test**: `target/release/pool` from `cargo build --locked --release -p pool` (Mach-O arm64, `sha256` `28b13482479609cf524691f9a894a0fc6beab1256c25a2277b1cd8cf6fe9c5fe`). That checksum is for the macOS build only and is not the AC-2 record.

### UI / manual

- [ ] **Linux build (AC-2, and the `/dev/full` clause of AC-11)**: `BLOCKED`
  - **reason**: `docker info` exited 1 again on 2026-11-14 at 08:32 UTC (`failed to connect to the docker API at unix:///Users/ojiepermana/.docker/run/docker.sock`). Docker 29.9.2 is installed but the daemon is not running, so the command was not run, as the spec asks.
  - **owner**: user
  - The prerequisites, the exact command, the expected result and what to record are in the developer's step above and under [Linux build](index.md#linux-build). They are unchanged. The command text in that step was compared with the spec and is the same, including the image digest.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

- [x] `cargo fmt --all --check` → exit 0 → AC-13
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → exit 0, no warning → AC-13
- [x] `cargo test --workspace --locked` → exit 0. `pool` unit 3, `check_config` 12, `pool-config` unit 3, `validate` 13, all passed → AC-13
- [x] `cargo build --locked --release -p pool` → exit 0 → AC-1
- [x] `cargo metadata --no-deps --format-version 1` → packages `['pool', 'pool-config']`, 2 workspace members → AC-1
- [x] `git ls-files Cargo.lock rust-toolchain.toml` → both tracked. `rust-toolchain.toml` equals the spec text → AC-1
- [x] Package names in `Cargo.lock` → 18 entries: `pool`, `pool-config` and exactly the sixteen named packages, at the versions in the developer's table → AC-12
- [x] `grep -rnE 'std::net|TcpListener|TcpStream|UdpSocket|UnixListener|UnixStream|tokio|std::thread' apps crates --include='*.rs'` → prints nothing, exit 1 → AC-12
- [x] `nm -u target/release/pool` searched for socket, connect, bind, listen, accept, `getaddrinfo` and `pthread_create` → none of them is imported → AC-12 (extra)
- [x] `ls -d apps/pool-cli crates/pool-protocol crates/pool-core crates/pool-control` → `No such file or directory` for all four. `apps/` holds only `pool`, `crates/` holds only `pool-config` → AC-13
- [x] `unsafe_code = "forbid"` under `[workspace.lints.rust]`, and `[lints] workspace = true` in both package manifests → AC-13
- [x] The four `cargo run -q --locked -p pool -- check-config ...` commands under [Linux build](index.md#linux-build), workstation part → same output and exit status as the developer recorded (0, 1, 1, 1) → AC-3, AC-5, AC-6, AC-9
- [x] The fifteen fixture files compared with the text under [Fixtures](index.md#fixtures) → every file is byte for byte the same, and there is no other file → AC-3 to AC-9
- [x] Every fixture through `target/release/pool check-config` in both formats → 30 runs, exit 0 for the three `valid-*` files and 1 for the other twelve, stderr empty every time → AC-3 to AC-9
- [x] Byte for byte probe of the binary (a scratch script, 248 checks, 0 failed; details per criterion below) → AC-3 to AC-12
- [x] Scratch crate that depends on `pool-config` by path (5 programs that must not compile, 1 that runs) → AC-8, AC-9

The scratch script and the scratch crate were kept outside the repository. No file in the repository was changed to run them.

### Acceptance-criteria coverage

| Criterion | What was run against the real build | Result |
|---|---|---|
| `0001 AC-1` | Clean checkout, release build exit 0 with 1.100.0 selected by `rust-toolchain.toml`, `cargo metadata` lists `pool` and `pool-config` only, `Cargo.lock` is tracked | PASS |
| `0001 AC-2` | Not run. `docker info` exited 1 | BLOCKED, owner: user |
| `0001 AC-3` | Each `valid-*` fixture, human and JSON: exit 0, stderr empty, stdout equal to the valid report byte for byte. Also with `--format=human` and `--config=<path>` | PASS |
| `0001 AC-4` (`SEC-10`) | Left out (`valid-minimal.toml`), empty table (`valid-capabilities-empty.toml`), `enabled = false` (`valid-capabilities-disabled.toml`): all exit 0 and show `result_cache: disabled` and `automatic_promotion: disabled`, in both formats. Same for the dotted form, the inline form and a file with only one of the two tables | PASS |
| `0001 AC-5` (`SEC-10`) | `invalid-result-cache-enabled.toml`: exit 1, `findings: 1`, `config.unsupported_capability at result_cache.enabled`, exact text in both formats. The dotted and inline spellings give the same finding | PASS |
| `0001 AC-6` (`SEC-10`) | `invalid-automatic-promotion-enabled.toml`: exit 1, `findings: 1`, `config.unsupported_capability at automatic_promotion.enabled`, exact text in both formats. The dotted and inline spellings give the same finding | PASS |
| `0001 AC-7` | The six other `invalid-*` fixtures give the listed code and line (`invalid-syntax` line 2, `invalid-unknown-key` line 3, `invalid-wrong-type` line 4, the rest at key `config_version`). Three bytes `FF FE FD` give `config.not_utf8`. 1048577 bytes of `#` give `config.too_large`. The made file of exactly 1048576 bytes is valid, and one byte more is `config.too_large`. Every row of the "Which code a bad value gets" table and both CRLF cases give the listed code and line | PASS |
| `0001 AC-8` (`SEC-12`) | `invalid-multiple.toml`: `findings: 3` in the listed order, in one run, in both formats. In the scratch crate, a struct literal of `ValidatedConfig` fails with `E0451` (private fields), and `Default`, `DeserializeOwned` and `Clone` for it fail with `E0277`. `validate` returned `Ok` only for the three `valid-*` fixtures and a `Rejection` that is never empty for the other twelve | PASS |
| `0001 AC-9` (`SEC-4`, `SEC-11`) | The three `secret-*` fixtures in both formats: exit 1, and `CANARY` is in neither stdout nor stderr (6 runs). The `Debug` and `Display` text of the `Rejection` for each was printed by the scratch crate and holds no canary. Extra: the canary as a table name, a key name, a duplicate key value, a wrong type value, inside a bad escape, inside a not UTF-8 file, in a missing path and in a bad option never reached the output | PASS |
| `0001 AC-10` | Every JSON run (valid, rejected, missing file, directory) wrote exactly one line to stdout that parses as one object, with stderr empty. Each line was compared byte for byte with a line built by hand in the spec's field order, so the order of fields is proven on the raw text | PASS |
| `0001 AC-11` | Workstation part: the 13 usage examples from the spec and 24 more exit 2 with empty stdout and the exact two lines on stderr. Missing file exits 3 with `io.not_found`, a directory exits 3 with `io.read_failed`, in both formats. `--help`, `-h`, `--version` and `-V` exit 0 with the exact text, and nothing after them is read. `--config --help` takes `--help` as the path. A path that is not UTF-8 is accepted after `--config`. Linux part (`/dev/full`): not run | PASS on the workstation. The `/dev/full` clause is BLOCKED with AC-2, owner: user |
| `0001 AC-12` | `grep` prints nothing. `Cargo.lock` holds the two workspace packages and the sixteen named ones. `pool check-config --config /dev/zero` exits 1 with `config.too_large` in 0.004 seconds, in both formats. `apps/pool/src/main.rs` reads through `take(MAX_CONFIG_BYTES + 1)` | PASS |
| `0001 AC-13` | The four paths do not exist. Both packages inherit `unsafe_code = "forbid"`. `cargo fmt`, `cargo clippy` and `cargo test` exit 0 | PASS |

Two things were also run that the spec does not ask a test for. As a user who is not root, a file with mode `000` exits 3 with `io.permission_denied` and the exact text in both formats. With stdout or stderr connected to a pipe that has no reader, every command (valid, rejected, help, version, usage error, missing file) exits 3 with no other output and no panic. That second case supports the write failure rule but is not the Linux `/dev/full` case, which stays blocked.

### Readings the checker assessed

1. **`--help=x` and `-h=x` after `check-config` print the help and exit 0.** This agrees with the spec: the "unknown option" row excludes `--help` and `-h` "with or without `=value`", so help is the only reading left. As the first argument, `--help=x`, `-h=x`, `--version=x` and `-V=x` give `unknown command`, which agrees with "the first argument is one of the five". The spec gives no example for either case. One example line there would remove the doubt. Not a defect.
2. **Field order of the JSON is checked on the raw line.** Agreed. This record does the same, byte for byte.
3. **The `/dev/full` test is compiled only on Linux.** This is what the spec asks ("under `#[cfg(target_os = "linux")]`"). The test exists at `apps/pool/tests/check_config.rs` line 525 and has not run anywhere yet.
4. **Milestone 3.** "Scaffold from the decision" is the third box of scope 001, so the number fits.

### For `/check review`

- `ValidatedConfig::config_version()` returns the constant `SUPPORTED_CONFIG_VERSION`, not the number that was parsed (`crates/pool-config/src/lib.rs` line 267). The output is right today because only `1` validates. The [value sourcing](index.md#value-sourcing) table says the value comes from the file's key, so this would go wrong without notice if a second version were ever supported.
- A not UTF-8 argument written as `--config=<bytes>` is `unexpected argument`. The spec allows such a path only as the separate argument after `--config`, so this is as written, but an operator may find it surprising.

## Full verify, second run: merged tree after the review fixes

Recorded by the checker session `chk-f1-verify` on 2026-11-14, after the tester's tests and the review fixes were merged. Every command was run again from a clean checkout, and each rule the review made clearer was checked against the real `pool` binary. This section replaces the results of the run at `5f44d88` above.

**Verdict**: `BLOCKED`, for the same single reason as before. Eleven criteria are met and nothing failed. `0001 AC-2` and the Linux clause of `0001 AC-11` could not be run because the Docker daemon is off.

**Revision**: `b6ce165c09c858c9e072f0b0fcbb115213c705a9` (`main`). The code is the code of `4ebcf93`, the merge of the review fixes; `b6ce165` adds only a scope log entry.

**What changed since `5f44d88`** (`git diff --stat 5f44d88..4ebcf93`): `crates/pool-config/src/lib.rs`, `apps/pool/src/cli.rs`, `apps/pool/src/main.rs`, the two test files, and the spec text (`index.md`, `rationale.md`). `Cargo.toml`, `Cargo.lock`, both package manifests, `rust-toolchain.toml`, `.gitignore` and the fifteen fixtures did not change.

**Checkout**: `git status --short` was empty and there was no `target/` directory before the first command.

**Workstation**: macOS on Apple silicon, `rustc 1.100.0`, selected by `rust-toolchain.toml` (`rustup show active-toolchain` says so). This is not the Linux build.

**Binary under test**: `target/release/pool` (Mach-O arm64, `sha256` `1914331edd730d5356f9ae8e2b2b79a67fb20e7589f9b4b11287d21d15c80c8e`). That checksum is for the macOS build only and is not the AC-2 record.

### Test counts on the merged tree (milestone 5, "Test it")

`cargo test --workspace --locked` exits 0 on macOS with 134 tests passed, 0 failed and 0 ignored.

| Target | Passed on macOS | At `5f44d88` (developer) | Added by the tester (`02bdcbe`) | Added with the review fixes (`4ebcf93`) |
|---|---|---|---|---|
| `pool` unit tests (`apps/pool/src/cli.rs`) | 3 | 3 | 0 | 0 |
| `apps/pool/tests/check_config.rs` | 66 | 12 | 52 | 2 |
| `pool-config` unit tests (`crates/pool-config/src/lib.rs`) | 3 | 3 | 0 | 0 |
| `crates/pool-config/tests/validate.rs` | 60 | 13 | 44 | 3 |
| `pool-config` doc tests (`compile_fail`) | 2 | 0 | 0 | 2 |
| **Total** | **134** | **31** | **96** | **7** |

Two more tests in `check_config.rs` are compiled only on Linux, so they are not in the 134: `a_failed_write_exits_3_without_stderr` (developer) and `ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr` (tester). With the second one the tester added 97 tests in all. Both wait for the Linux build, where the run should show 136 tests. Neither has run anywhere yet.

### UI / manual

- [ ] **Linux build (AC-2, and the `/dev/full` clause of AC-11)**: `BLOCKED`
  - **reason**: `docker info` exited 1 again on 2026-11-14 at 09:17 UTC (`failed to connect to the docker API at unix:///Users/ojiepermana/.docker/run/docker.sock`). The daemon is still not running, so the command was not run, as the spec asks.
  - **owner**: user
  - The prerequisites, the exact command and what to record are in the developer's step under [Milestone 3](#milestone-3-scaffold-from-the-decision) and under [Linux build](index.md#linux-build). The command did not change in the review round. The expected result now includes both Linux only tests named above.

### Commands

Run from the repository root with `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`.

- [x] `cargo fmt --all --check` → exit 0 → AC-13
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → exit 0, no warning → AC-13
- [x] `cargo test --workspace --locked` → exit 0, 134 passed (3 + 66 + 3 + 60 + 2), as in the table above → AC-13
- [x] `cargo build --locked --release -p pool` → exit 0 → AC-1
- [x] `cargo metadata --no-deps --format-version 1` → packages `['pool', 'pool-config']`, 2 workspace members → AC-1
- [x] `git ls-files Cargo.lock rust-toolchain.toml` → both tracked → AC-1
- [x] Package names in `Cargo.lock` → 18 entries, the same names and versions as before. `cargo tree -e features` shows no `unbounded` feature of `toml` → AC-12
- [x] `grep -rnE 'std::net|TcpListener|TcpStream|UdpSocket|UnixListener|UnixStream|tokio|std::thread' apps crates --include='*.rs'` → prints nothing, exit 1 → AC-12
- [x] `nm -u target/release/pool` searched for socket, connect, bind, listen, accept, `getaddrinfo` and `pthread_create` → none of them is imported → AC-12 (extra)
- [x] `ls -d apps/pool-cli crates/pool-protocol crates/pool-core crates/pool-control` → `No such file or directory` for all four → AC-13
- [x] `unsafe_code = "forbid"` under `[workspace.lints.rust]`, and `[lints] workspace = true` in both package manifests → AC-13
- [x] The four `cargo run -q --locked -p pool -- check-config ...` commands under [Linux build](index.md#linux-build), workstation part → exit 0, 1, 1, 1 with the same output as before → AC-3, AC-5, AC-6, AC-9
- [x] The fifteen fixture files compared with the text under [Fixtures](index.md#fixtures) → byte for byte the same, no other file → AC-3 to AC-9
- [x] Every fixture through `target/release/pool check-config` in both formats → 30 runs, exit 0 for the three `valid-*` files and 1 for the other twelve, stderr empty every time. The output is identical to the run at `5f44d88` → AC-3 to AC-9
- [x] Byte for byte probe of the binary (the scratch script from the first run plus the review cases, 398 checks, 0 failed) → AC-3 to AC-12
- [x] Scratch crate that depends on `pool-config` by path (5 programs that must not compile, 2 that run) → AC-7, AC-8, AC-9

### Rules made clearer in the review, each checked on the binary

| Rule | What was run | Observed | Result |
|---|---|---|---|
| An array in place of a reserved table gives exactly one `config.schema` (AC-7, AC-8, `SEC-10`) | For `result_cache` and for `automatic_promotion`: `= [false]`, `= [true]`, `= []`, `= [false, true]`, `= [[true]]`, `= [{ enabled = true }]`, `= [{ enabled = false }]`, an array of tables (`[[result_cache]]`) with `enabled = true` and with `enabled = false`, both formats | Exit 1, `findings: 1`, `config.schema` with a line and a column, every time. Never valid and never `config.unsupported_capability`. `result_cache = [false]`, which the review found was accepted at `5f44d88`, is now `config.schema at line 2, column 16` | PASS |
| A long array that holds the canary (AC-9) | `result_cache = [...]` and `automatic_promotion = [...]` with 2000 copies of the canary string, both formats, and the same input through `validate` in the scratch crate | Exit 1, one `config.schema`. `CANARY` is in neither stdout nor stderr, and not in the `Debug` or `Display` text of the `Rejection` | PASS |
| Other values in place of a table, or in place of `enabled` (AC-7) | A string, a boolean, a float and a date for each reserved table; `enabled = [true]` and `enabled = { enabled = true }`; `config_version = [1]` | Exit 1, one `config.schema` at the listed line | PASS |
| The schema step ends the run (AC-8) | `config_version = 2` together with `result_cache = [true]` | One `config.schema` only. The version finding is not added, because the meaning step is not reached | PASS |
| A dotted key with 81 parts is `config.syntax` with no location (AC-7) | `a.a. ... .a = 1` with 81 parts on line 2, both formats. The same as a table header, and as 80 more parts under `result_cache` | Exit 1. Human: `config.syntax: the file is not valid TOML`. JSON: `line` and `column` are `null` | PASS |
| A dotted key with 80 parts is inside the parser limit (AC-7) | The same with 80 parts, as a key and as a table header | Exit 1, `config.schema at line 2, column 1` (an unknown key), so the parser read it | PASS |
| A file of brackets does not use up the stack (AC-12) | 1 MiB of `[` in a value, 500000 nested arrays, 1 MiB of `{`, 200000 nested inline tables, 1 MiB of `[` at the start, both formats | Exit 1 with one `config.syntax` every time. No crash, no signal, no exit 101 | PASS |
| `--format=<bytes>` and `--config=<bytes>` that are not UTF-8 (AC-11) | `--format=\xff` after and before `--config`, `--config=\xff\xfe.toml` | Exit 2, `error: unexpected argument` and the usage line, stdout empty | PASS |
| A not UTF-8 value as the separate argument after `--format` (AC-11) | `--format \xff --config a` | Exit 2, `error: --format must be human or json` | PASS |
| `--help=x` and `-h=x` where an option is expected (AC-11) | `check-config --help=x`, `check-config -h=x`, `check-config --help=`, `check-config --config a --help=x --bogus` | Exit 0, the exact help text, stderr empty | PASS |
| The first argument is compared whole (AC-11) | `pool --help=x`, `pool -h=x`, `pool --version=x`, `pool -V=x` | Exit 2, `error: unknown command` | PASS |
| `--version` and `-V` are not options of `check-config` (AC-11) | `check-config --version`, `check-config -V`, `check-config --version=x`, `check-config --config a --version` | Exit 2, `error: unknown option` | PASS |
| A file with mode `000` (Findings, `io.permission_denied`) | A file with mode `000`, run as a user who is not root, both formats | Exit 3. Human: `error: io.permission_denied: the configuration file cannot be read: permission denied` on stderr, stdout empty. JSON: the one line error document, stderr empty. The new test `ac11_a_file_that_cannot_be_opened_is_permission_denied` also passed in the test run | PASS |
| The column counts characters, not bytes (AC-7) | A file whose line 2 is `x = "éé" junk`, both formats, and the same text through `validate` | `config.syntax at line 2, column 10`. A count of bytes would give 12 | PASS |
| `config_version()` comes from the parsed number (value sourcing) | `validate` on `config_version = 1` in the scratch crate, and reading `crates/pool-config/src/lib.rs` | Returns 1. The field is now set from the parsed value. Only `1` can validate, so no other value can be shown at run time | PASS, as far as it can be observed |
| The read bound fallback (AC-12) | `--config /dev/zero` in both formats, and reading `apps/pool/src/main.rs` | Exit 1 with `config.too_large`. The fallback of the limit is now `1_048_577`, not `u64::MAX` | PASS |
| Special files are read like any other file (known limit) | `--config /dev/stdin` with a valid file on stdin, and `--config /dev/null` | Valid report and exit 0; `config.missing_key at config_version` and exit 1. A FIFO with no writer was not run, because the spec records that it blocks | PASS for what was run |
| No value can be built outside the crate (AC-8) | The two new `compile_fail` doc tests, and the five scratch programs | The doc tests pass. The struct literal fails with `E0451`; `Default`, `DeserializeOwned` and `Clone` fail with `E0277` | PASS |

### Acceptance-criteria coverage

| Criterion | Touched by the review round | What was run on this revision | Result |
|---|---|---|---|
| `0001 AC-1` | no | Clean checkout, release build exit 0 with 1.100.0 selected by `rust-toolchain.toml`, two workspace members, `Cargo.lock` tracked | PASS |
| `0001 AC-2` | no | Not run. `docker info` exited 1 | BLOCKED, owner: user |
| `0001 AC-3` | no | The three `valid-*` fixtures, both formats, byte for byte | PASS |
| `0001 AC-4` (`SEC-10`) | no | Left out, empty table, `enabled = false`, plus the dotted and inline forms: all valid and `disabled` | PASS |
| `0001 AC-5` (`SEC-10`) | no | `invalid-result-cache-enabled.toml` and the dotted and inline spellings: exit 1, exactly one `config.unsupported_capability` at `result_cache.enabled` | PASS |
| `0001 AC-6` (`SEC-10`) | no | `invalid-automatic-promotion-enabled.toml` and the dotted and inline spellings: exit 1, exactly one finding at `automatic_promotion.enabled` | PASS |
| `0001 AC-7` | yes | All fixtures and made inputs from the first run again, every row of the bad value table with its two new rows (arrays, nesting depth), the exact column case | PASS |
| `0001 AC-8` (`SEC-12`) | yes | `invalid-multiple.toml` gives three findings in order. An array can no longer produce a `ValidatedConfig`. The compile checks above | PASS |
| `0001 AC-9` (`SEC-4`, `SEC-11`) | yes | The three `secret-*` fixtures in both formats, the fifteen extra canary cases from the first run, and the long canary arrays: no canary in stdout, stderr, `Debug` or `Display` | PASS |
| `0001 AC-10` | no | Every JSON run is one line, one object, in the spec's field order, with stderr empty | PASS |
| `0001 AC-11` | yes | 49 usage cases, 11 help cases, 4 version cases, missing file, directory, mode `000`, on the workstation. Linux part (`/dev/full`): not run | PASS on the workstation. The `/dev/full` clause is BLOCKED with AC-2, owner: user |
| `0001 AC-12` | yes | `grep` prints nothing, `Cargo.lock` holds the same 18 packages, `/dev/zero` ends with `config.too_large`, the bracket files end with a finding | PASS |
| `0001 AC-13` | yes | The four paths do not exist, both packages inherit `unsafe_code = "forbid"`, `cargo fmt`, `cargo clippy` and `cargo test` exit 0 with 134 tests | PASS |

With stdout or stderr connected to a pipe that has no reader, every command still exits 3 with no other output and no panic, as in the first run. This supports the write failure rule but is not the Linux `/dev/full` case.

### Notes from this run

- Both notes under "For `/check review`" in the first run are closed. `config_version()` now returns the parsed number, and the spec now says in words that `--config=<bytes>` is an unexpected argument.
- One check in the scratch script was wrong and was corrected, not the product. It expected line 2 for a file with `result_cache = [true]` on line 2 and `automatic_promotion = [true]` on line 3. The binary reports line 3. The spec asks for exactly one `config.schema` there and does not say which of two schema errors is reported, so this is not a failure.
- That case shows a behavior worth knowing. When a file has two schema errors, the one reported is the one whose key comes first in alphabetical order, not the one that comes first in the file (`zzz = 1` on line 2 and `aaa = 1` on line 3 report line 3). It is the same on every run. Whether `5f44d88` behaved the same way was not checked. The consequences in the spec say "only the first such error is reported", which a reader may take as first in the file. One sentence there would remove the doubt. No criterion depends on it.
