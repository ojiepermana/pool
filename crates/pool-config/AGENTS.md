# crates/pool-config

## Overview

The library that decides whether a configuration file is valid. It depends on no other POOL crate and does no input or output: the caller reads the file and passes its bytes to `validate`. A file is accepted or rejected as a whole. The contract is the [public boundary of pool-config](../../docs/specs/0001-initial-architecture/index.md#public-boundary-of-pool-config) in spec 0001.

## Key files

| File | Owns |
|---|---|
| `src/lib.rs` | `validate`, `ValidatedConfig`, `Rejection`, `Finding`, `FindingCode`, `Location`, and the unit tests |
| `tests/validate.rs` | Every fixture and every in memory case through `validate` |
| `tests/fixtures/*.toml` | The shared fixtures, also read by the `pool` tests |
| `fuzz/` | The fuzz package `pool-config-fuzz` with the target `validate`. It is outside the workspace and has its own lockfile ([spec 0002](../../docs/specs/0002-standards-and-tooling/index.md#fuzz-harness)) |

## Commands

Run only this package's tests:

```bash
cargo test -p pool-config --locked
```

Run the fuzz target for 60 seconds (needs `cargo-fuzz` 0.13.2 and the toolchain `nightly-2026-10-17`):

```bash
scripts/check.sh fuzz
```

## Conventions

- `validate` is the only way to get a `ValidatedConfig`. Keep its fields private and add no public constructor, `Default`, `Clone` or `Deserialize`. The `compile_fail` doc tests in `src/lib.rs` guard the struct literal and `Default`.
- A `Rejection` holds only finding codes, static key paths and numbers. Never put text from the file, or `toml` error text, into a finding, a `Debug` or a `Display`.
- New keys follow the [configuration rules](../../docs/specs/0001-initial-architecture/index.md#configuration-rules) and take their names from the [glossary](../../docs/glossary.md). New finding codes follow [findings](../../docs/specs/0001-initial-architecture/index.md#findings) and the [validation order](../../docs/specs/0001-initial-architecture/index.md#validation-order).
- `FindingCode::message` repeats the numbers in `MAX_CONFIG_BYTES` and `SUPPORTED_CONFIG_VERSION`. Change both together.
- Dependencies are `serde` and `toml` only. Keep the `toml` feature `unbounded` off; its nesting limit is what keeps a file of brackets from using up the stack.

## Gotchas

- Fixtures live only in `tests/fixtures/`, and their exact content and expected findings are listed under [fixtures](../../docs/specs/0001-initial-architecture/index.md#fixtures). Adding or changing one changes the `pool` tests too.
- The canary `CANARY-9f2c7b1e` in the `secret-*` fixtures is fake. Inputs that are too large or not UTF-8 are built in memory by the tests and never committed.

## Related specs

- [Spec 0001, workspace build notes](../../docs/specs/0001-initial-architecture/index.md#workspace-build-notes), and its [tests and who owns them](../../docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them)

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
