# apps/pool

## Overview

The `pool` daemon binary, with four commands. `check-config` reads one file, passes its bytes to `pool_config::validate`, prints a report and exits, and it opens no socket and starts no runtime. `bootstrap` probes the nodes and writes the first journal, `show-authority` prints the authority ID and the state of the journal, and `run` is the daemon: it composes the listener of `pool-core`, the control loop of `pool-control`, the probes and the log writer ([commands](../../docs/specs/0004-one-real-primary-path/index.md#commands) in spec 0004). The contract of `check-config` is [the check-config command](../../docs/specs/0001-initial-architecture/index.md#the-check-config-command) and [exit status and output](../../docs/specs/0001-initial-architecture/index.md#exit-status-and-output) in spec 0001.

## Key files

| File | Owns |
|---|---|
| `src/main.rs` | Help text, exit status constants, the bounded file read, mapping an outcome to an exit status |
| `src/cli.rs` | Argument parsing with the standard library only, one pass from left to right |
| `src/report.rs` | The human and JSON formats of `check-config`, and the `io.*` findings |
| `src/commands.rs`, `src/output.rs` | `pool bootstrap` and `pool show-authority`, and their reports |
| `src/daemon.rs` | `pool run`: the steps to a serving daemon, the mapping from a topology snapshot to the route view, and the shutdown |
| `src/control.rs` | The control loop: one task owns the `Controller`, takes samples from the probe tasks, marks a silent writer lost and sets the conflict latch |
| `src/probe.rs` | One probe of one node as the monitor login, within `probe.timeout_ms` |
| `src/state.rs` | The check of the state directory and the lock on `pool.lock` |
| `src/log.rs` | The log writer: JSON lines to stderr through a bounded queue |
| `tests/check_config.rs` | Runs the built binary and compares exit status, stdout and stderr |
| `tests/commands.rs` | `bootstrap`, `show-authority` and the refusals of `run`, with no database |
| `tests/integration.rs`, `tests/support/` | The tests against real PostgreSQL 18. Each is ignored unless the integration check runs it |

For credential-only SIGHUP rotation and the daemon task watcher, read [spec 0005](../../docs/specs/0005-clean-session-reuse/index.md#rotation-and-internal-interface). `daemon.rs` reads one complete protected credential snapshot at a time and calls `Server::rotate_credentials`; it supervises control, probe, listener and log tasks through `supervise`. Blocking credential loads lose publication authority during shutdown.

## Commands

Run only this package's tests:

```bash
cargo test -p pool --locked
```

Run the tests against real PostgreSQL (needs Docker; starts two containers from the pinned image and removes them):

```bash
scripts/check.sh integration
```

Run one of those tests again and again. A run with the filter is not a run of the check:

```bash
POOL_IT_FILTER=<test name> scripts/check.sh integration
```

## Conventions

- Exit status is 0, 1, 2 or 3 as the [exit status table](../../docs/specs/0001-initial-architecture/index.md#exit-status-and-output) says. Exit 101 (a panic) is always a defect.
- Write output with `write!` or `writeln!` on the locked `stdout` or `stderr` handle, flush, and turn a failed write into exit 3. Never use `print!`, `println!`, `eprint!` or `eprintln!`.
- All output text comes from constants, schema key paths and numbers. Nothing read from the file, and no parser or operating system message text, reaches stdout or stderr. Each value's source is in [value sourcing](../../docs/specs/0001-initial-architecture/index.md#value-sourcing).
- A finding inside a repeated table prints its key and its place, the fourth form of the finding line in [spec 0003](../../docs/specs/0003-control-and-session-data-model/index.md#findings). `output_version` stays `1`.
- Usage error messages are constant lowercase text (`cli.rs`), and the first usage error ends the run.
- The file read stays bounded: at most `MAX_CONFIG_BYTES` plus one byte, through `Read::take`.
- `pool run` binds the listener only when the journal state is ready, and it never reads a row of the journal in this package: `rusqlite` may be named only in `pool-control` ([one writer](../../docs/specs/0004-one-real-primary-path/index.md#one-writer)).
- Every line that `pool run` writes after it started is one JSON object, and no line holds a password, a verifier, SQL text, a principal, a login, a database name, a host or a file path ([log events](../../docs/specs/0004-one-real-primary-path/index.md#log-events), `0004 AC-24`).
- This package maps between `pool-control` and `pool-core`. Neither of those two may depend on the other.
- Arguments are parsed by hand. Do not add a parser crate or any other dependency the path does not use ([adopted dependencies](../../docs/specs/0001-initial-architecture/index.md#adopted-dependencies)).

## Gotchas

- Tests read the fixtures that `crates/pool-config` owns, through the `FIXTURES` constant built from `CARGO_MANIFEST_DIR`. Add a new fixture there, not here.
- A file a test makes at run time goes under `env!("CARGO_TARGET_TMPDIR")` with a file name of its own, so tests can run at the same time.
- Some cases run only on some platforms: `/dev/zero` and mode `000` under `#[cfg(unix)]`, `/dev/full` under `#[cfg(target_os = "linux")]`. A run on macOS does not run the Linux case.
- The tests in `tests/integration.rs` read addresses from the `POOL_IT_*` variables that `scripts/integration/run.sh` sets. Run without the script, they are skipped as ignored tests. A configuration in a test must pass the cross rules of the probe keys.
- A test that passes the committed CA file to a command copies it first with mode 0644, because the Linux build image gives committed files the mode 0664 and the CA rule refuses it.
- Task supervision uses `supervise` in `daemon.rs`; its tests cover each supervised task ending and the resulting failure events (`0005 AC-9`).
- A test that leaks the fake canary `CANARY-9f2c7b1e` into output is a security regression (`SEC-11`), not a flaky test.

## Related specs

- [Spec 0004, commands](../../docs/specs/0004-one-real-primary-path/index.md#commands), [control loop and topology snapshot](../../docs/specs/0004-one-real-primary-path/index.md#control-loop-and-topology-snapshot), [the integration check](../../docs/specs/0004-one-real-primary-path/index.md#the-integration-check) and [tests and who owns them](../../docs/specs/0004-one-real-primary-path/index.md#tests-and-who-owns-them)
- [Spec 0001, workspace build notes](../../docs/specs/0001-initial-architecture/index.md#workspace-build-notes), and its [tests and who owns them](../../docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them)

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
