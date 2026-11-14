# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

@AGENTS.md

AGENTS.md above is the canonical agent guide (workflow, invariants, documentation ownership). This file only adds what it does not cover. Update shared guidance there, not here.

## Current state and commands

The repository holds the planning package plus the scope 001 workspace: `apps/pool` (the `pool` binary) and `crates/pool-config`. The only command is the offline `pool check-config`. There is no listener, proxy path or CI yet. Rust 1.100.0 is pinned by `rust-toolchain.toml`.

These checks exist today and are run by hand (CI arrives with scope 002):

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test -p <crate> <test_name>   # single test
cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-minimal.toml
```

The Linux container build is the command under "Linux build" in [spec 0001](docs/specs/0001-initial-architecture/index.md#linux-build). It needs a running Docker daemon, and a build on macOS is not Linux evidence. Its latest result is in the [verification record](docs/specs/0001-initial-architecture/verify.md). When CI appears, replace this section with what it defines.

## Planned architecture

[Spec 0001](docs/specs/0001-initial-architecture/index.md) is the owning decision. Its content was ratified on 2026-11-14. Its lifecycle status mirrors scope feature 1: "In Progress" while the foundation is being built and proven, "Accepted" when the feature is done. That is intentional, not an inconsistency to fix.

- **Shape:** one Rust daemon (`apps/pool`) plus a separate admin CLI (`apps/pool-cli`), and four internal crates. `pool-protocol` (framing, protocol state) and `pool-config` (validated config) depend on no other POOL crate. `pool-core` (auth integration, leases, sessions, routing, backend I/O) depends on protocol and config. `pool-control` (probes, topology, SQLite operation journal, admin operations) depends on config only.
- **Core and control never depend on each other.** Each owns its boundary types; the daemon maps control topology snapshots into core's route view. The CLI talks only to the versioned admin API. There is no `common` crate. Auth, pooling, router, balancer, health, failover and similar candidates start as modules inside core or control and are extracted only with a documented decision ([docs/repository-structure.md](docs/repository-structure.md)).
- **Data plane vs control plane:** the data plane reads immutable, versioned topology snapshots and never does blocking journal work, infrastructure calls or consensus per query. Control publishes a snapshot only after validation. Both planes share one process failure domain in the initial daemon.
- **Session model:** a backend lease is pinned to one frontend for the whole session (no transaction or statement multiplexing). The route is chosen at login from profile plus fresh topology; later SQL is admitted or rejected on that pinned lease and never moves endpoint. Replica profiles have no primary fallback and cannot become write profiles mid session. Clients reconnect to change profile or recover from backend loss.
- **Identity:** frontend and backend authentication are separate. An operator configured map resolves verified frontend principal plus requested database to the backend login, credential reference and routing profile. Unmapped combinations fail.
- **Cancellation:** the session manager issues per lease virtual cancellation credentials and keeps real backend keys private. A backend is retired after any forwarded cancellation.
- **Stack:** Tokio, a hand written wire state machine on `bytes` and `tokio::net` (no client driver in the data path), rustls, `pg_query` behind an adapter, TOML with Serde, Axum admin API, `prometheus-client`, `tracing`, SQLite journal with one writer. Versions and alternatives are in [docs/research/technology-evaluation.md](docs/research/technology-evaluation.md).

The ten numbered safety contracts in spec 0001 are the detailed form of the AGENTS.md invariants. Read them before touching leases, routing, cancellation or promotion.

## Working with the docs

- [docs/scope/index.md](docs/scope/index.md) is the work queue: 17 features, the last 6 deferred. A status change must update both the index row and the feature file's heading. Feature numbers are stable across renames.
- Feature files list their steps as slash commands (`/architect`, `/develop`, `/check verify`, `/test`, `/check review`, `/document`) that correspond to installed skills. Feature 1 is done; its Linux build evidence is in the verification record. The next queued feature is 2 (standards and tooling), then the first walking slice whose acceptance is in [docs/testing/strategy.md](docs/testing/strategy.md).
- [docs/implementation-order.md](docs/implementation-order.md) gives the first handoff and the entry steps for later slices; the scope index owns the order. Create only what the current slice needs; do not scaffold the full proposed tree from `docs/repository-structure.md`.
- Each fact has one owning file (see "Documentation ownership" in `docs/repository-structure.md`). Edit it there and link from other views. Terms are owned by [docs/glossary.md](docs/glossary.md). Adding or moving a document also means updating [docs/index.md](docs/index.md).
- Feature status advances only with linked build and test evidence. Documentation alone never moves a scope feature to `done` or a feature inventory row to Implemented or Tested. The states are defined once in the scope index's status lifecycle.
- The functional baseline is Pgpool-II 4.7.3 ([docs/feature-matrix.md](docs/feature-matrix.md)). The project does not claim full parity; keep that wording when editing.
- Use conventional commits and small pull requests ([CONTRIBUTING.md](CONTRIBUTING.md)).
