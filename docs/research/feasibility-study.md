# Feasibility study

## Finding

POOL is technically feasible as a PostgreSQL focused proxy. A correct session proxy and manual operations layer are reasonable early goals. Complete Pgpool parity is a long program with several independent research risks. Rust can help contain memory errors but cannot prove transaction semantics, fencing, or a throughput advantage.

| Area | Confidence | Evidence or validation needed |
|---|---|---|
| Bounded asynchronous proxy | High feasibility | Mature runtime and TLS ecosystem; fragmented frame and cancellation tests still needed |
| Session pooling | High feasibility, high correctness risk | Identity partitioning and reset quarantine tests; limited benefit for permanently connected clients |
| SQL syntax classification | High feasibility | PostgreSQL grammar bindings available; native build and fuzzing required |
| Transparent arbitrary read splitting | Low confidence without restrictions | Catalog dependent semantics, session state, prepared statements, and causal consistency require a narrower contract |
| Read only session selection | Medium to high feasibility | Explicit application profile, health filtering, lag budget, and server enforcement |
| Manual promotion | Feasible with external dependencies | Verified fencing, durable operations, timeline checks, RPO disclosure, and restart reconciliation |
| Automatic promotion | Not ready for initial release | Independent safety case, distributed authority, fencing and partition testing |
| Query result caching | High uncertainty | External writers, permissions, MVCC snapshots and invalidation cannot be solved by TTL alone |
| Native or snapshot middleware replication | High uncertainty | Determinism, partial commit, sequences, functions, and divergence need a separate go or no go study |

## Proposed stopping rules

If parser coverage cannot meet the declared PostgreSQL 18 subset, route that subset to primary or disable routing; do not substitute prefix checks. If a session cannot be reset confidently, close the backend. If fencing cannot be independently verified, do not promote through POOL. If benchmark overhead is unacceptable, profile and fix the measured bottleneck before adding policies.

## Operational ownership

PostgreSQL owns WAL, durability, replication, transaction isolation, backups and promotion primitives. Operators own backup validation and recovery objectives. systemd owns process supervision. Proxmox owns VM lifecycle and host availability. External fencing owns exclusion of the old primary. POOL observes, controls admission, and orchestrates explicitly authorized operations. Running every VM on one physical host does not create host high availability.

## Commercial and maintenance assumptions

This plan assumes an open source project without hosted account management or billing. Two experienced engineers and periodic protocol, security, and SRE review are the estimate baseline. Establish at least two release maintainers, a private security channel, a dependency update cadence, and a documented support matrix before production releases. These organizational requirements cost time and remain unstaffed assumptions.

## Go or no go milestones

You can proceed from research to the first proxy slice after the initial decision is ratified. Proceed to shared backend reuse only after authentication and cleanup evidence. Proceed to replica profiles only after routing invariants pass. Proceed to V1 only after manual failover drills and driver compatibility pass on the actual lab. Advanced features do not block the core MVP.
