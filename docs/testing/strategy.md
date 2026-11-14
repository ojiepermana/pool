# Testing strategy

Tests establish the declared support profile; they do not prove every PostgreSQL behavior. Map cases to PRD acceptance IDs and feature matrix IDs. Save binary hashes, configs, fixture version, environment and test logs with release evidence. No runtime test has been executed during this documentation phase.

| Layer | Test method | Required evidence |
|---|---|---|
| Protocol units | Golden frames, split at every byte boundary, checked lengths | Correct parse/state/error and bounded allocations |
| Protocol properties | proptest state and framing models | Encode/decode, order, synchronization and limit invariants |
| Fuzzing | Startup, message codec, routing and native parser adapter | Minimized regressions, no memory errors/panics/unbounded work within corpus budgets |
| Native boundaries | Sanitizers and targeted harnesses where tooling permits | Parser, crypto and SQLite integration failures investigated |
| Database integration | Real PostgreSQL 18 direct and proxy paths | Matching query results and transaction/session behavior |
| Authentication | SCRAM/PLUS/mTLS/rotation/denial cases | No downgrade, cross role access or secret export |
| Pool isolation | Alternating users with GUC/temp/lock/portal/LISTEN state | Zero observed cross owner leakage; uncertain cleanup closes |
| Routing | Differential syntax plus semantic adversarial corpus | Zero observed write to replica misroutes; conservative unknowns |
| Replication | Pause receive/replay, role switch, timeline divergence | Unknown/stale data excluded; no false freshness claim |
| Operations | Journal crash and disk faults at every phase | Idempotency and reconciliation without repeated irreversible action |
| Partition/chaos | Firewall, delay, process pause, VM stop, stale controller | No unfenced promotion or conflicting writer |
| Stability | 24 hour MVP and 72 hour V1 mixed workload soak | No unexplained resource growth, leakage or safety violation |
| Performance | Repeated equivalent workload comparisons | Distributions, resource use and correctness counters |

## First walking slice acceptance

Use one primary and one real client, verified TLS, actual authentication, bounded framing and admission, a successful INSERT and SELECT, disconnect and observed rollback of any incomplete transaction. Record the direct PostgreSQL baseline. Shared connection reuse, replica routing and failover are later slices, so this success is not reported as a complete MVP.

## CI progression

On each implementation PR, format, lint, compile supported targets, run unit/property and targeted integration tests. Protocol/auth/pooling changes add driver and fuzz regression suites. Promotion changes add operation crash and fencing tests. Nightly or scheduled project CI can run broader chaos, load and soak suites once the project exists. Expensive performance runs use controlled hosts rather than noisy shared runners for release decisions.

For later Rust code, proposed checks include `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, dependency/license checks and documented Linux builds. These are future commands; this workspace currently has no Cargo project.

## Release gates

No unresolved critical safety defect. Every claimed matrix capability has test evidence and known limits. Driver releases and PostgreSQL package versions are pinned. A fresh reviewer examines protocol identity and failover assumptions. Benchmark changes report methodology and confidence, not only a winning throughput number. Failed gates keep the feature planned or in progress; documentation alone never changes it to Implemented/Tested.
