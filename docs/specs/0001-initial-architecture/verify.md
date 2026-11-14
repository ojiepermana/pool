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
