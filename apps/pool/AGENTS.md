# apps/pool

## Overview

The `pool` daemon binary. Today it has one working command, `check-config`, which reads one file, passes its bytes to `pool_config::validate`, prints a report and exits. It opens no socket and starts no thread or async runtime. The contract is [the check-config command](../../docs/specs/0001-initial-architecture/index.md#the-check-config-command) and [exit status and output](../../docs/specs/0001-initial-architecture/index.md#exit-status-and-output) in spec 0001.

## Key files

| File | Owns |
|---|---|
| `src/main.rs` | Help text, exit status constants, the bounded file read, mapping an outcome to an exit status |
| `src/cli.rs` | Argument parsing with the standard library only, one pass from left to right |
| `src/report.rs` | The human and JSON formats, and the `io.*` findings |
| `tests/check_config.rs` | Runs the built binary and compares exit status, stdout and stderr |

## Commands

Run only this package's tests:

```bash
cargo test -p pool --locked
```

## Conventions

- Exit status is 0, 1, 2 or 3 as the [exit status table](../../docs/specs/0001-initial-architecture/index.md#exit-status-and-output) says. Exit 101 (a panic) is always a defect.
- Write output with `write!` or `writeln!` on the locked `stdout` or `stderr` handle, flush, and turn a failed write into exit 3. Never use `print!`, `println!`, `eprint!` or `eprintln!`.
- All output text comes from constants, schema key paths and numbers. Nothing read from the file, and no parser or operating system message text, reaches stdout or stderr. Each value's source is in [value sourcing](../../docs/specs/0001-initial-architecture/index.md#value-sourcing).
- Usage error messages are constant lowercase text (`cli.rs`), and the first usage error ends the run.
- The file read stays bounded: at most `MAX_CONFIG_BYTES` plus one byte, through `Read::take`.
- Arguments are parsed by hand. Do not add a parser crate or any other dependency the path does not use ([adopted dependencies](../../docs/specs/0001-initial-architecture/index.md#adopted-dependencies)).

## Gotchas

- Tests read the fixtures that `crates/pool-config` owns, through the `FIXTURES` constant built from `CARGO_MANIFEST_DIR`. Add a new fixture there, not here.
- A file a test makes at run time goes under `env!("CARGO_TARGET_TMPDIR")` with a file name of its own, so tests can run at the same time.
- Some cases run only on some platforms: `/dev/zero` and mode `000` under `#[cfg(unix)]`, `/dev/full` under `#[cfg(target_os = "linux")]`. A run on macOS does not run the Linux case.
- A test that leaks the fake canary `CANARY-9f2c7b1e` into output is a security regression (`SEC-11`), not a flaky test.

## Related specs

- [Spec 0001, workspace build notes](../../docs/specs/0001-initial-architecture/index.md#workspace-build-notes), and its [tests and who owns them](../../docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them)

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
