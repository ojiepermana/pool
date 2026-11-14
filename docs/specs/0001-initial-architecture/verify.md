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

- [ ] **Linux build (AC-2)**: see the step 5 entry below.

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
| `0001 AC-2` | Linux build, step 5 below | blocked (Docker daemon not running) |
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
