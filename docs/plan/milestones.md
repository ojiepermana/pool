# Milestones

| Milestone | Demonstrable result | Release gate |
|---|---|---|
| Walking slice | One authenticated TLS client, one primary, one INSERT/SELECT cycle, bounded admission and teardown | Reproducible evidence, no shared reuse yet; internal experiment |
| Core MVP | Correct declared protocol profile, session pooling, reset isolation, TLS/SCRAM, cancellation, limits and basic diagnostics | Selected driver matrix and malformed protocol tests pass; no replica or failover marketing claim |
| Routing preview | Read only profiles select healthy replicas automatically; primary profile preserves mixed workload semantics | Explicit application consistency contract and routing corpus pass |
| V1 | Your initial four VM scenario, including manual fenced failover and operator diagnostics | Production safety review, crash/partition drills, 72 hour soak and benchmark report |
| V1.5 | Multiple proxy instances and controlled recovery with external authority | Minority, stale leader, cancellation ownership and fencing tests pass |
| V2 | Independently approved advanced features | Each feature passes its own consistency and operations gates |
| Eventual parity | Applicable baseline behaviors accounted for with test evidence | Census reconciled, exclusions disclosed, compatibility results published; no date committed |

## Release practices

Use pre 1.0 releases while configuration and protocol support contracts are evolving. Publish supported PostgreSQL versions, protocol profiles, driver versions, known limitations and upgrade steps with every release. A version number is not a safety certification. Promote a release only from tagged artifacts with checksums, SBOM, license notices and reproducible test evidence.

You can keep V1 on manual failover indefinitely. Automatic promotion is an optional product decision, not a condition for calling the core proxy useful.
