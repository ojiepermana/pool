# pool

High Performance PostgreSQL Proxy, Connection Pooler and Cluster Manager.

POOL is a planning stage open source Rust project for PostgreSQL 18 on Linux. You can start with one modular proxy daemon, a separate CLI, session pooling, explicit read only replica profiles, health/lag diagnostics and manual failover protected by verified fencing. PostgreSQL continues to own storage, WAL and replication.

**Current state:** Planning package with a ratified initial foundation. There is no proxy binary, Cargo workspace, benchmark result or supported production release. No application dependencies have been installed and no infrastructure has been deployed. This work stops at planning as requested.

The confirmed foundation uses Tokio, rustls, PostgreSQL grammar through pg_query, Axum administration, Prometheus metrics, tracing logs and a local SQLite operation journal. Frontend and backend authentication are separate and preserve the actual database login identity. Session multiplexing, automatic promotion, result caching and middleware replication are deferred.

## Start here

1. [Complete documentation index](docs/index.md), every generated document and its path.
2. [Ratified initial foundation](docs/specs/0001-initial-architecture/index.md), with [alternatives and rationale](docs/specs/0001-initial-architecture/rationale.md).
3. [Scope index](docs/scope/index.md), managing 17 individual feature files with 6 deferred.
4. [Feature matrix](docs/feature-matrix.md), 165 behavior/command entries, plus [207 sample spellings and 186 canonical runtime names](docs/research/pgpool-configuration-inventory.md).
5. [Roadmap](docs/plan/roadmap.md), [timeline](docs/plan/timeline.md), [milestones](docs/plan/milestones.md), and [starting point](docs/implementation-order.md).
6. [Threat model](docs/security/threat-model.md), [test strategy](docs/testing/strategy.md), and [benchmark plan](docs/testing/benchmark-plan.md).

## Delivery boundary

The core MVP is a correct primary proxy with session reuse and declared protocol/driver support. Operational V1 completes your four VM scenario with restricted replica routing, health, lag, diagnostics and fenced manual promotion. For two experienced engineers, the provisional realistic ranges are 16 to 24 elapsed weeks to the core MVP and 34 to 53 weeks to V1. These estimates include subsequent engineering work and qualification; no precise delivery dates or throughput promises are made.

The selected functional baseline is Pgpool-II 4.7.3, verified on 2026-11-14. The inventory distinguishes facts, proposals and explicit exclusions. It does not claim 100 percent parity or a full source/runtime audit. PostgreSQL 18 protocol 3.2 cancellation behavior is included in the planned compatibility contract.

## Contribution and license

You can read [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [engineering standards](docs/engineering-standards.md), and [SECURITY.md](SECURITY.md) before implementation. POOL is licensed under [Apache 2.0](LICENSE). Third party source reuse and native dependency notices need their own review.

The next queued step is `/develop stack and workspace`, followed by one real authenticated TLS client to one PostgreSQL primary with bounded framing, INSERT/SELECT and clean teardown. You have accepted the foundation; a build starts only when you explicitly request implementation. The first proxy experiment sits inside the MVP rather than scaffolding every future crate.
