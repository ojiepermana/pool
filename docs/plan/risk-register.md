# Risk register

Probability is an initial engineering judgment, not a measured frequency. Critical impact means data loss, cross identity exposure or split brain. Each owner is a role to staff, not a named person already assigned.

| Risk | Probability | Impact | Mitigation | Validation | Owner |
|---|---|---|---|---|---|
| Protocol incompatibility | High | High | Explicit state machine and declared profile | Driver and golden frame corpus | Protocol engineer |
| Incorrect SQL classification | High | Critical | Primary default, restricted read profiles | Differential and adversarial routing corpus | Database architect |
| Session state leakage | High | Critical | Identity keys, pinned leases, close uncertain reset | Alternating tenant state corpus | Protocol engineer |
| Prepared statement incompatibility | High | High | Pin Parse/Bind/Execute ownership | Named/unnamed and pipeline tests | Protocol engineer |
| Incorrect reuse or late cancel | Medium | Critical | Lease generations and reset quarantine | Race and model tests | Systems engineer |
| Transaction consistency change | Medium | Critical | No live migration or transparent replay | Isolation and history checks | Database architect |
| Replica lag mistaken for freshness | High | High | Fresh samples, comparable LSNs, explicit staleness | Paused replay and clock skew drills | SRE |
| Split brain | Medium | Critical | Independent fencing and authority | Partition and stale leader drills | HA engineer |
| Failover loses acknowledged commits | Medium | Critical | Proven flush target or explicit unknown RPO | Synchronous/asynchronous failure histories | Database architect |
| Ambiguous COMMIT replay | High | Critical | No automatic transaction retries | Drop response after server commit | Protocol engineer |
| Authentication or role confusion | Medium | Critical | Separate credentials, no privileged shared role | SCRAM, mapping and rotation tests | Security engineer |
| Malformed frame vulnerability | High | Critical | Bounds and safe parsing | Fuzzing, sanitizers at native boundaries | Security engineer |
| Memory exhaustion | High | High | Global bytes/tasks/queue budgets | Slow readers and many pool keys | Systems engineer |
| Runtime blocking/backpressure failure | Medium | High | Dedicated blocking lanes and fair admission | Saturation and fault profiles | Systems engineer |
| Performance regression | Medium | High | Equivalent baseline methodology | Repeatable benchmarks and profiles | Performance engineer |
| Dependency drift or abandoned crate | Medium | High | Lockfile, SBOM, update policy and adapters | Advisory and maintenance review | Maintainer |
| License conflict or missing notices | Medium | High | Per file and transitive review before reuse | Release notice manifest audit | Release maintainer |
| Journal disk loss or replay | Medium | Critical | FULL durability, idempotency, reconciliation | Crash/power and disk full tests | Control engineer |
| Restart adopts wrong writable node | Medium | Critical | Durable approved writer/epoch/quarantine plus fresh matching observations | Promotion/restart with stale config and conflicting roles | Control engineer |
| Fencing provider unavailable | Medium | Critical | Refuse promotion; operator runbook | Expired receipt and partial power action tests | Infrastructure operator |
| Proxmox host correlated failure | Medium | High | Separate hosts before HA claim | Host loss drill | Infrastructure operator |
| Scope expansion | High | High | MVP/V1 boundary, advanced go or no go gates | Review roadmap at each release | Project lead |
| Single maintainer dependency | High | High | Two release maintainers and documented operations | Independent release/restore exercise | Project lead |

The highest initial priorities are protocol identity and state, unsafe routing, cancellation reuse races, uncertain outcomes, and fencing. No throughput optimization takes precedence over these invariants.
