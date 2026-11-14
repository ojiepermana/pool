# Roadmap

You can treat each phase as a gate, not a promise to complete every later phase. Estimates and staffing assumptions live in [timeline](timeline.md). All implementation is still planned.

| Phase | Goal and scope | Depends on | Deliverable and acceptance | Main risk and exit condition |
|---|---|---|---|---|
| 0 Research and architecture | Verify baseline, inventory behavior, choose boundaries and safety policy | Your brief and accessible primary sources | Ratified initial architecture, proposed feature contracts and inventoried gaps | Initial direction ratified; feature decisions remain with their slices |
| 1 Foundation | Minimal workspace, stable tools, CI, offline config validation and the control/session model | Ratified foundation | Reproducible Linux build, checked config fixtures and a reviewed writer/identity contract for the first live connection | Premature scaffolding; exit with only the first slice's crates and dependencies |
| 2 Core proxy | Real TLS/auth path, forwarding, session reuse, extended protocol, COPY, cancel, bounded resources | Foundation, direct PostgreSQL fixture and the auth/protocol contract for the slice being opened | Core MVP with driver evidence, reset isolation and overload behavior | Protocol/session leakage; exit only when declared profile and supported driver matrix pass |
| 3 SQL routing | Grammar adapter, read profiles, role/lag eligibility, weighted selection among eligible replicas; refuse if none qualifies by the deadline | Core session contract and topology model | Controlled read distribution with session preservation and explicit consistency | Semantic misclassification; exit after routing decision corpus and stale replica tests pass |
| 4 Production reliability | Health hardening, topology reconciliation, CLI/API, manual fenced promotion, drain, security and soak | Core, routing and durable operation model; a verified fencing provider is required for promotion | Operational V1 covers your four VM scenario | Split brain and unknown outcomes; exit after crash/partition/fencing drills and compatibility evidence |
| 5 High availability | Multiple proxies, external authority, cancellation ownership, endpoint HA, checked recovery | Proven V1 and separate HA decision | V1.5 with controller isolation and recovery drills | Stale leader and fence assumptions; automatic failover remains a separately gated option |
| 6 Advanced features | Selected cache, dashboard, routing and mode proposals | Measured need and separate safety cases | V2 features individually qualified | Scope and consistency; full parity is evaluated, never assumed |

## Using the plan

Use the [scope queue](../scope/index.md) for numbered work and status, the [dependency plan](dependencies.md) for handoffs, and the [milestone table](milestones.md) for release decisions. Phases group work for estimation. They do not move security, backpressure or evidence collection out of the slice that first exposes the behavior.
