# Risk register

Probability is an initial engineering judgment, not a measured frequency. Critical impact means data loss, cross identity exposure or split brain. Each owner is a role to staff, not a named person already assigned.

| ID | Risk | Probability | Impact | Mitigation | Validation | Owner |
|---|---|---|---|---|---|---|
| R-01 | Protocol incompatibility | High | High | Explicit state machine and declared profile | Driver and golden frame corpus | Protocol engineer |
| R-02 | Incorrect SQL classification | High | Critical | Primary default, restricted read profiles | Differential and adversarial routing corpus | Database architect |
| R-03 | Session state leakage | High | Critical | Identity keys, pinned leases, close uncertain reset | Alternating tenant state corpus | Protocol engineer |
| R-04 | Prepared statement incompatibility | High | High | Pin Parse/Bind/Execute ownership | Named/unnamed and pipeline tests | Protocol engineer |
| R-05 | Incorrect reuse or late cancel | Medium | Critical | Lease generations and reset quarantine | Race and model tests | Systems engineer |
| R-06 | Transaction consistency change | Medium | Critical | No live migration or transparent replay | Isolation and history checks | Database architect |
| R-07 | Replica lag mistaken for freshness | High | High | Fresh samples, comparable LSNs, explicit staleness | Paused replay and clock skew drills | SRE |
| R-08 | Split brain | Medium | Critical | Independent fencing and authority | Partition and stale leader drills | HA engineer |
| R-09 | Failover loses acknowledged commits | Medium | Critical | Proven flush target or explicit unknown RPO | Synchronous/asynchronous failure histories | Database architect |
| R-10 | Ambiguous COMMIT replay | High | Critical | No automatic transaction retries | Drop response after server commit | Protocol engineer |
| R-11 | Authentication or role confusion | Medium | Critical | Separate credentials, no privileged shared role | SCRAM, mapping and rotation tests | Security engineer |
| R-12 | Malformed frame vulnerability | High | Critical | Bounds and safe parsing | Fuzzing, sanitizers at native boundaries | Security engineer |
| R-13 | Memory exhaustion | High | High | Global bytes/tasks/queue budgets | Slow readers and many pool keys | Systems engineer |
| R-14 | Runtime blocking/backpressure failure | Medium | High | Dedicated blocking lanes and fair admission | Saturation and fault profiles | Systems engineer |
| R-15 | Performance regression | Medium | High | Equivalent baseline methodology | Repeatable benchmarks and profiles | Performance engineer |
| R-16 | Dependency drift or abandoned crate | Medium | High | Lockfile, SBOM, update policy and adapters | Advisory and maintenance review | Maintainer |
| R-17 | License conflict or missing notices | Medium | High | Per file and transitive review before reuse | Release notice manifest audit | Release maintainer |
| R-18 | Journal disk loss or replay | Medium | Critical | FULL durability, idempotency, reconciliation | Crash/power and disk full tests | Control engineer |
| R-19 | Restart adopts wrong writable node | Medium | Critical | Durable approved writer/epoch/quarantine plus fresh matching observations | Promotion/restart with stale config and conflicting roles | Control engineer |
| R-20 | Fencing provider unavailable | Medium | Critical | Refuse promotion; operator runbook | Expired receipt and partial power action tests | Infrastructure operator |
| R-21 | Proxmox host correlated failure | Medium | High | Separate hosts before HA claim | Host loss drill | Infrastructure operator |
| R-22 | Scope expansion | High | High | MVP/V1 boundary, advanced go or no go gates | Review roadmap at each release | Project lead |
| R-23 | Single maintainer dependency | High | High | Two release maintainers and documented operations | Independent release/restore exercise | Project lead |
| R-24 | Walking slice exceeds its estimate | High | Medium | The 004 spec orders checkpoints that each leave a runnable binary; breadth stays in later slices | Compare actual effort with the [timeline](timeline.md) at the first re-estimate | Project lead |
| R-25 | Reviewer or lab unavailable at a gate | High | High | Name the reviewer and lab owner at slice entry; the gate stays open and is never self approved | Verification record names the reviewer and environment | Project lead |

Cite a risk by its ID. At slice entry, the implementer assigns a person to each applicable risk with the reviewer. Link the mitigating decision and the negative test or drill in the spec. At the milestone review, identify any remaining exposure, its owner and the decision it blocks. An open fencing or identity safety failure cannot be waived by reporting a good benchmark. See [handoffs](dependencies.md) and [release gates](milestones.md).

The highest initial priorities are protocol identity and state, unsafe routing, cancellation reuse races, uncertain outcomes, and fencing. No throughput optimization takes precedence over these invariants.
