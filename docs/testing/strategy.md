# Testing strategy

This strategy defines how to qualify the declared support profile. All runtime evidence is pending.

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

Run the same fixture through direct PostgreSQL 18 and through POOL with the mapped login, verified TLS and one dedicated backend. Compare the committed INSERT/SELECT result and a deliberately generated server SQLSTATE. Then disconnect during a second, uncommitted insert. Observe the backend exit, the permit return and the absence of that row from a fresh connection. With a configured connection or frame limit exceeded, verify bounded rejection and a subsequent successful login. This covers the first slice of the [core proxy PRD](../prd/001-core-proxy.md#acceptance-criteria).

## Acceptance evidence

Before implementation, use the owning feature spec to identify the acceptance criteria in this slice. Qualify each PRD reference with its document number (for example, `PRD-001 AC-1`). For each case identify the fixture, supported profile, expected observation and applicable negative or fault case. Link relevant inventory IDs (`F-*`) and security requirements (`SEC-*`) where they apply. A foundation slice can use its scope or spec criteria directly.

Keep the verification record with the feature spec, as `verify.md` beside its `index.md`. Keep independent review findings under `docs/reviews/`, named by date and branch. Record the revision and artifact digest where applicable, a reproducible command or CI run, fixture and dependency versions, redacted configuration, expected and observed behavior, and the result for each criterion. Mark a failed, blocked or unrun case explicitly. Tests and logs containing credentials, cancel keys or sensitive SQL must use sanitized fixtures or redacted artifacts.

For a design or feasibility slice, record the reviewed decision, evidence considered and its limits. A no-go decision can conclude the study; it does not qualify a runtime capability.

Close an implementation slice after its criteria pass in the declared environment, critical regression cases are committed, an independent reviewer has examined the applicable safety changes, and operator or configuration instructions reflect the behavior. Link the verification record and the review findings from the owning scope feature. A release report can cite the same record for the same artifact and profile. If the candidate or profile changes, identify affected claims and rerun their cases.

## CI progression

On each implementation PR, format, lint, compile supported targets, run unit/property and targeted integration tests. Protocol/auth/pooling changes add driver and fuzz regression suites. Promotion changes add operation crash and fencing tests. Nightly or scheduled project CI can run broader chaos, load and soak suites once the project exists. Expensive performance runs use controlled hosts rather than noisy shared runners for release decisions.

For later Rust code, proposed checks include `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, dependency/license checks and documented Linux builds. These are future commands; this workspace currently has no Cargo project.

## Release gates

Apply the [milestone gates](../plan/milestones.md) to a pinned release candidate. Record results for the applicable [driver profiles](compatibility-matrix.md), soak duration above, failure cases, and [benchmark method](benchmark-plan.md). For operational V1, retain the four VM topology, sanitized operation history and independent fence evidence from the drill. Keep blocked or unrun cases visible. An unresolved critical safety defect or a failed required case blocks the corresponding release claim.
