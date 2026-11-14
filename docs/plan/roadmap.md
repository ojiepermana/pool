# Roadmap

You can treat each phase as a gate, not a promise to complete every later phase. Estimates and staffing assumptions live in [timeline](timeline.md). All implementation is still planned.

| Phase | Goal and scope | Depends on | Deliverable and acceptance | Main risk and exit condition |
|---|---|---|---|---|
| 0 Research and architecture | Verify baseline, inventory behavior, choose boundaries and safety policy | Your brief and accessible primary sources | Proposed architecture, matrix, PRDs, threat model, delivery plan | Unknown semantics; exit after you ratify initial direction and record remaining gates |
| 1 Foundation | Minimal workspace, stable tools, CI, config, logs, test lab and protocol fixtures | Ratified foundation | Reproducible Linux build, versioned config validation, direct PostgreSQL reference test | Premature scaffolding; exit with only the first slice's crates and dependencies |
| 2 Core proxy | Real TLS/auth path, forwarding, session reuse, extended protocol, COPY, cancel, bounded resources | Foundation, auth and protocol specs | Core MVP with driver evidence, reset isolation and overload behavior | Protocol/session leakage; exit only when declared profile and supported driver matrix pass |
| 3 SQL routing | Grammar adapter, read profiles, role/lag eligibility, weighted replica selection, primary fallback policy | Core session contract and topology model | Controlled read distribution with session preservation and explicit consistency | Semantic misclassification; exit after routing decision corpus and stale replica tests pass |
| 4 Production reliability | Health hardening, topology reconciliation, CLI/API, manual fenced promotion, drain, security and soak | Core, routing, durable operation model and real fencing provider | Operational V1 covers your four VM scenario | Split brain and unknown outcomes; exit after crash/partition/fencing drills and compatibility evidence |
| 5 High availability | Multiple proxies, external authority, cancellation ownership, endpoint HA, checked recovery | Proven V1 and separate HA decision | V1.5 with controller isolation and recovery drills | Stale leader and fence assumptions; automatic failover remains a separately gated option |
| 6 Advanced features | Selected cache, dashboard, routing and mode proposals | Measured need and separate safety cases | V2 features individually qualified | Scope and consistency; full parity is evaluated, never assumed |

## Delivery slices

Start with one real client connecting through the daemon to one primary, performing an INSERT and SELECT, then disconnecting without leaked work. Include framing, real authentication, TLS, a bounded connection permit, one configuration path, an observable connection event, and a reproducible test. The slice has no shared backend reuse yet. It is a narrow proof inside Phase 2, not an MVP or production readiness claim.

Thicken the same path with reset and reuse, extended operations and cancellation, then replica profile selection and health, then operator control and failover. Security, diagnostics and backpressure are built into each slice rather than waiting until Phase 4.
