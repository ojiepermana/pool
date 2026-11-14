# pool

High Performance PostgreSQL Proxy, Connection Pooler and Cluster Manager.

POOL is a planning stage open source Rust project for PostgreSQL 18 on Linux. You can start with one modular proxy daemon, a separate CLI, session pooling, explicit read only replica profiles, health/lag diagnostics and manual failover protected by verified fencing. PostgreSQL continues to own storage, WAL and replication.

**Current state:** Planning package with a ratified initial foundation, plus a first workspace slice: the `pool` binary with one offline command, `pool check-config`, that accepts or rejects a configuration file. There is no proxy path, listener, benchmark result or supported production release, and no infrastructure has been deployed. The Linux container build of this slice is still open; see the [verification record](docs/specs/0001-initial-architecture/verify.md). Work beyond this slice waits for an explicit request.

The confirmed foundation uses Tokio, rustls, PostgreSQL grammar through pg_query, Axum administration, Prometheus metrics, tracing logs and a local SQLite operation journal. Frontend and backend authentication are separate and preserve the actual database login identity. Session multiplexing, automatic promotion, result caching and middleware replication are deferred.

## Start here

1. [Complete documentation index](docs/index.md), every generated document and its path.
2. [Ratified initial foundation](docs/specs/0001-initial-architecture/index.md), with [alternatives and rationale](docs/specs/0001-initial-architecture/rationale.md).
3. [Scope index](docs/scope/index.md), managing 17 individual feature files with 6 deferred.
4. [Feature matrix](docs/feature-matrix.md), 165 behavior/command entries, plus [207 sample spellings and 186 canonical runtime names](docs/research/pgpool-configuration-inventory.md).
5. [Roadmap](docs/plan/roadmap.md), [timeline](docs/plan/timeline.md), [milestones](docs/plan/milestones.md), and [starting point](docs/implementation-order.md).
6. [Threat model](docs/security/threat-model.md), [test strategy](docs/testing/strategy.md), and [benchmark plan](docs/testing/benchmark-plan.md).

## Delivery boundary

The core MVP is a correct primary proxy with session reuse and declared protocol/driver support. Operational V1 completes your four VM scenario with restricted replica routing, health, lag, diagnostics and fenced manual promotion. The [estimation model](docs/plan/timeline.md) gives effort ranges and staffing assumptions; it does not promise a delivery date or throughput.

The selected functional baseline is Pgpool-II 4.7.3, verified on 2026-11-14. The inventory distinguishes facts, proposals and explicit exclusions. It does not claim 100 percent parity or a full source/runtime audit. PostgreSQL 18 protocol 3.2 cancellation behavior is included in the planned compatibility contract.

## Contribution and license

You can read [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [engineering standards](docs/engineering-standards.md), and [SECURITY.md](SECURITY.md) before implementation. POOL is licensed under [Apache 2.0](LICENSE). Third party source reuse and native dependency notices need their own review.

The first build followed the [workspace handoff](docs/implementation-order.md#first-handoff-the-workspace). The [scope index](docs/scope/index.md) shows what comes next, and [CHANGELOG.md](CHANGELOG.md) lists what has changed.

## Build and check

Rust 1.100.0 is pinned by `rust-toolchain.toml`, so `rustup` selects it for you. You can run these from the repository root:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml
```

The last command prints `configuration valid` and exits 0. Exit status 1 means the file was rejected, 2 a usage error and 3 that the command could not finish. The Linux container build is described under [Linux build](docs/specs/0001-initial-architecture/index.md#linux-build) in spec 0001.

`scripts/check.sh all` runs the checks that CI runs, except the Linux container build, which is `scripts/check.sh linux-build`.
