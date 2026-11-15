# crates/pool-config

## Overview

The library that decides whether a configuration file is valid. It depends on no other POOL crate and does no input or output: the caller reads the file and passes its bytes to `validate`. A file is accepted or rejected as a whole. The contract is the [public boundary of pool-config](../../docs/specs/0001-initial-architecture/index.md#public-boundary-of-pool-config) in spec 0001.

## Key files

| File | Owns |
|---|---|
| `src/lib.rs` | `validate`, `ValidatedConfig`, `Rejection`, `Finding`, `FindingCode`, `Location`, the line index that finds a line and column, and the unit tests |
| `src/topology.rs` | The `[control]`, `[[cluster]]`, `[[node]]` and `[[identity]]` keys, their checks and the identity map index |
| `src/identifier.rs`, `src/generation.rs` | One type for each kind of ID and for each durable counter |
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
- The cluster, node and identity keys, their finding codes and the order of their checks are in [spec 0003](../../docs/specs/0003-control-and-session-data-model/index.md#configuration). An ID or a counter of one kind never compares with another kind, and `resolve_identity` matches the exact pair only.
- Validation time must stay near linear in the file size up to the 1 MiB limit, for a valid file and for an invalid one (`0003 AC-15`). Look a position up through the line index, never by counting from the start of the file.
- `FindingCode::message` repeats the numbers in `MAX_CONFIG_BYTES` and `SUPPORTED_CONFIG_VERSION`. Change both together.
- Dependencies are `serde` and `toml` only. Keep the `toml` feature `unbounded` off; its nesting limit is what keeps a file of brackets from using up the stack.

## Gotchas

- Fixtures live only in `tests/fixtures/`, and their exact content and expected findings are listed under [fixtures](../../docs/specs/0001-initial-architecture/index.md#fixtures) in spec 0001 and under [fixtures](../../docs/specs/0003-control-and-session-data-model/index.md#fixtures) in spec 0003 for the `*topology*` files. Adding or changing one changes the `pool` tests too.
- The canary `CANARY-9f2c7b1e` in the `secret-*` fixtures is fake. Inputs that are too large or not UTF-8 are built in memory by the tests and never committed.

## Related specs

- [Spec 0001, workspace build notes](../../docs/specs/0001-initial-architecture/index.md#workspace-build-notes), and its [tests and who owns them](../../docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them)
- [Spec 0003, configuration](../../docs/specs/0003-control-and-session-data-model/index.md#configuration), its [identifier and generation types](../../docs/specs/0003-control-and-session-data-model/index.md#identifier-and-generation-types) and its [validation time](../../docs/specs/0003-control-and-session-data-model/index.md#validation-time)

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
