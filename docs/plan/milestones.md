# Milestones

The first column identifies where the gate is assessed. Passing an earlier gate does not close later PRD cases. Detailed fixtures and run records follow the [testing strategy](../testing/strategy.md#acceptance-evidence).

| Milestone / scope | Demonstrable result | Release gate |
|---|---|---|
| Walking slice / [004](../scope/004-one-real-primary-path.md) | One authenticated TLS client, one primary, one INSERT/SELECT cycle, bounded admission and teardown | Reproducible evidence, no shared reuse yet; internal experiment |
| Core MVP / [006](../scope/006-protocol-breadth-and-cancellation.md) | Correct declared protocol profile, session pooling, reset isolation, TLS/SCRAM, cancellation, limits and basic diagnostics | MVP rows in the [driver matrix](../testing/compatibility-matrix.md), malformed protocol tests and 24 hour soak pass |
| Routing preview / [007](../scope/007-restricted-replica-sessions.md) | Read only profiles select healthy replicas automatically; primary profile preserves mixed workload semantics | Explicit application consistency contract and routing corpus pass |
| V1 / [011](../scope/011-release-qualification-harness.md) | Your initial four VM scenario, including manual fenced failover and operator diagnostics | V1 driver matrix, independent safety review, crash/partition/fencing drills, 72 hour soak and benchmark report on the declared profile |
| V1.5 / [012](../scope/012-multiple-proxy-authority.md), [013](../scope/013-checked-online-recovery.md) | Multiple proxy instances and controlled recovery with external authority | Minority, stale leader, cancellation ownership and fencing tests pass |
| V2 | Independently approved advanced features | Each feature passes its own consistency and operations gates |
| Eventual parity | Applicable baseline behaviors accounted for with test evidence | Census reconciled, exclusions disclosed, compatibility results published; no date committed |

## Release practices

Use pre 1.0 releases while configuration and protocol support contracts are evolving. Publish supported PostgreSQL versions, protocol profiles, driver versions, known limitations and upgrade steps with every release. A version number is not a safety certification. Promote a release only from tagged artifacts with checksums, SBOM, license notices and reproducible test evidence.

Before publishing V1, the release maintainer links the tested operator procedures for install, drain, upgrade and restore. Rehearse a compatible rollback or a documented forward recovery on a disposable deployment with existing state. Reverting binaries or TOML cannot undo a promotion or silently restore stale writer authority. Identify the supported upgrade path (or first install only), approver, artifact and any unresolved blockers. Follow the [security reporting policy](../../SECURITY.md) for a public release.

You can keep V1 on manual failover indefinitely. Automatic promotion is an optional product decision, not a condition for calling the core proxy useful.
