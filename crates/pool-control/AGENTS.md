# crates/pool-control

## Overview

The library that holds the authority rules of the control plane: which state a start is in, when a first bootstrap may happen, and when writes to a cluster are admitted. Every rule is a plain function over values. Since scope 004 the crate also holds the journal on disk (one SQLite file), the topology snapshot and the text of the probe query, and the daemon in `apps/pool` calls it. It still reads no clock and no socket: the caller passes the time and the probe samples in as values, and only `journal.rs` touches a file ([the journal on disk](../../docs/specs/0004-one-real-primary-path/index.md#the-journal-on-disk) in spec 0004). The contract of the rules is [the authority model in pool-control](../../docs/specs/0003-control-and-session-data-model/index.md#the-authority-model-in-pool-control) in spec 0003.

## Key files

| File | Owns |
|---|---|
| `src/lib.rs` | The crate documentation and the public names |
| `src/authority.rs` | `StoreProbe`, `StoredAuthority`, `AuthorityState`, `Authority` and `classify_startup` |
| `src/bootstrap.rs` | `BootstrapPlan`, `BootstrapRefusal` and `plan_bootstrap` |
| `src/admission.rs` | `WriterAdmission`, `EligiblePrimary`, `Closed`, `ClosedReason` and `writer_admission` |
| `src/observation.rs` | `Observation`, `NodeFacts`, `TimelineId`, `MonotonicTime` and the freshness rule |
| `src/journal.rs` | `Journal`, `JournalRead`, `JournalError` and `LatchReceipt`: the journal on disk. The only file that names `rusqlite` |
| `src/snapshot.rs` | `build_snapshot`, the only caller of `writer_admission`, `TopologySnapshot` and `Controller` |
| `src/probe.rs` | `PROBE_QUERY` and `observation_from_row` |
| `tests/walkthroughs.rs` | The four walks of spec 0003, row by row, and the `ac16_` tests |
| `tests/journal.rs` | The journal on real files under `CARGO_TARGET_TMPDIR`, with damaged files |
| `tests/controller.rs` | The board, `build_snapshot`, a round of the `Controller`, the retry of the latch and the probe row |
| `tests/admission_model.rs` | The model test over 8192 cases, with a rule of its own for the answer and its reasons |

## Commands

Run only this package's tests:

```bash
cargo test -p pool-control --locked
```

That no clock, file or environment access came in (`0003 AC-11`) is held by rule W3 since spec 0004, not by a search run by hand:

```bash
scripts/check.sh guards
```

## Conventions

- Authority is never derived from an observation. `writer_admission` opens only when the configuration (`expected_writer`), the journal (the approved writer) and a fresh observation name the same node, and every other state ends closed (`0001 SC-8`, `R-19`).
- Code outside this crate cannot build an `Authority`, an `EligiblePrimary`, a `Closed` or a `BootstrapPlan`. Keep their fields private. Each has a `compile_fail` doc test with a twin that compiles.
- A `Ready` state belongs to the configuration it was classified with. `writer_admission` compares `control.authority_id` first and closes with one reason when it is missing or different (`0003 AC-16`).
- The caller passes the time as a value. The freshness rule uses the latest sample of a node only. A sample dated after now, a stale one, or a tie at the latest time counts as no evidence.
- The dependencies are `pool-config` and `rusqlite`, and `rusqlite` may be named only in `src/journal.rs` and in the tests. A `Journal` owns one connection, and its methods that write take `&mut self` ([one writer](../../docs/specs/0004-one-real-primary-path/index.md#one-writer)). This crate never depends on `pool-core`. It has no `dev-dependencies` and no build script.
- Every public record type states its restart rule in its documentation comment (`0003 AC-14`).
- The reason names, such as `authority.uninitialized` and `authority.conflict_latched`, are a contract. Changing one is a decision in spec 0003.

## Gotchas

- A `Ready` state is a copy of the journal at the start. It does not learn of a conflict latch that is set later, so the `Controller` carries it and writes it to the journal ([control loop and topology snapshot](../../docs/specs/0004-one-real-primary-path/index.md#control-loop-and-topology-snapshot) of spec 0004, [follow up 4](../../docs/specs/0003-control-and-session-data-model/index.md#follow-up) of spec 0003).
- `StoreProbe` and `StoredAuthority` can be built by any crate. The guarantee of `0003 AC-12` holds only while the journal reader is the one code that builds them.
- The tests get a `ValidatedConfig` by calling `pool_config::validate` on TOML text written in the test. There are no fixture files here. The journal tests make their files at run time.
- A unit test in `journal.rs` expects `rusqlite::version()` to be `3.54.1`, so a build that took another SQLite fails ([native code](../../docs/specs/0004-one-real-primary-path/index.md#native-code)).

## Related specs

- [Spec 0004, the journal on disk](../../docs/specs/0004-one-real-primary-path/index.md#the-journal-on-disk), [the probe](../../docs/specs/0004-one-real-primary-path/index.md#the-probe) and [control loop and topology snapshot](../../docs/specs/0004-one-real-primary-path/index.md#control-loop-and-topology-snapshot)
- [Spec 0003, the authority model in pool-control](../../docs/specs/0003-control-and-session-data-model/index.md#the-authority-model-in-pool-control), its [four walks](../../docs/specs/0003-control-and-session-data-model/index.md#the-four-walks), and its [tests and who owns them](../../docs/specs/0003-control-and-session-data-model/index.md#tests-and-who-owns-them)

_Drafted by /sync from the introducing change, worth a quick human pass._
