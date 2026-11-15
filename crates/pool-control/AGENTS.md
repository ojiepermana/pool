# crates/pool-control

## Overview

The library that holds the authority rules of the control plane: which state a start is in, when a first bootstrap may happen, and when writes to a cluster are admitted. Every rule is a plain function over values. The crate reads no file, no clock and no socket, and nothing calls it yet. Scope 004 connects the journal, the probe and the clock to it. The contract is [the authority model in pool-control](../../docs/specs/0003-control-and-session-data-model/index.md#the-authority-model-in-pool-control) in spec 0003.

## Key files

| File | Owns |
|---|---|
| `src/lib.rs` | The crate documentation and the public names |
| `src/authority.rs` | `StoreProbe`, `StoredAuthority`, `AuthorityState`, `Authority` and `classify_startup` |
| `src/bootstrap.rs` | `BootstrapPlan`, `BootstrapRefusal` and `plan_bootstrap` |
| `src/admission.rs` | `WriterAdmission`, `EligiblePrimary`, `Closed`, `ClosedReason` and `writer_admission` |
| `src/observation.rs` | `Observation`, `NodeFacts`, `TimelineId`, `MonotonicTime` and the freshness rule |
| `tests/walkthroughs.rs` | The four walks of spec 0003, row by row, and the `ac16_` tests |
| `tests/admission_model.rs` | The model test over 8192 cases, with a rule of its own for the answer and its reasons |

## Commands

Run only this package's tests:

```bash
cargo test -p pool-control --locked
```

Check that no clock, file or environment access came in (`0003 AC-11`). It must print nothing:

```bash
grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src
```

## Conventions

- Authority is never derived from an observation. `writer_admission` opens only when the configuration (`expected_writer`), the journal (the approved writer) and a fresh observation name the same node, and every other state ends closed (`0001 SC-8`, `R-19`).
- Code outside this crate cannot build an `Authority`, an `EligiblePrimary`, a `Closed` or a `BootstrapPlan`. Keep their fields private. Each has a `compile_fail` doc test with a twin that compiles.
- A `Ready` state belongs to the configuration it was classified with. `writer_admission` compares `control.authority_id` first and closes with one reason when it is missing or different (`0003 AC-16`).
- The caller passes the time as a value. The freshness rule uses the latest sample of a node only. A sample dated after now, a stale one, or a tie at the latest time counts as no evidence.
- The only dependency is `pool-config`. This crate never depends on `pool-core`. It has no `dev-dependencies` and no build script.
- Every public record type states its restart rule in its documentation comment (`0003 AC-14`).
- The reason names, such as `authority.uninitialized` and `authority.conflict_latched`, are a contract. Changing one is a decision in spec 0003.

## Gotchas

- A `Ready` state is a copy of the journal at the start. It does not learn of a conflict latch that is set later, so the daemon of scope 004 must carry that ([follow up 4](../../docs/specs/0003-control-and-session-data-model/index.md#follow-up) of spec 0003).
- `StoreProbe` and `StoredAuthority` can be built by any crate. The guarantee of `0003 AC-12` holds only while the journal reader is the one code that builds them.
- The tests get a `ValidatedConfig` by calling `pool_config::validate` on TOML text written in the test. There are no fixture files here.

## Related specs

- [Spec 0003, the authority model in pool-control](../../docs/specs/0003-control-and-session-data-model/index.md#the-authority-model-in-pool-control), its [four walks](../../docs/specs/0003-control-and-session-data-model/index.md#the-four-walks), and its [tests and who owns them](../../docs/specs/0003-control-and-session-data-model/index.md#tests-and-who-owns-them)

_Drafted by /sync from the introducing change, worth a quick human pass._
