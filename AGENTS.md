# POOL agent guide

POOL is a PostgreSQL 18 proxy, session pooler and cluster operations project in Rust. The current repository contains the planning package and the first workspace slice (scope 001: `apps/pool`, `crates/pool-config` and the offline `pool check-config` command), not an implemented proxy. The user asked for scope 001 on 2026-11-14 and nothing beyond it. Begin further implementation only when explicitly requested.

## Workflow

1. Read [scope](docs/scope/index.md) and the relevant PRD from the [documentation index](docs/index.md). Before architecture or boundary changes, read [spec 0001](docs/specs/0001-initial-architecture/index.md) and the owning feature spec.
2. Inspect the actual code, manifest, Git state and environment before relying on documentation. The Cargo workspace holds only the `pool` and `pool-config` packages, and there is no CI yet. Do not report a planned command as an executed check.
3. For implementation, define the authorized slice, acceptance criteria and necessary tests before editing. The build approach is Tracer Bullet, one real path through the system before breadth. Workflow depth is GA: behavior verification, tests, fresh review and documentation.
4. Apply [engineering standards](docs/engineering-standards.md). For protocol/auth/session work also read [protocol analysis](docs/research/protocol-analysis.md) and [security requirements](docs/security/security-requirements.md). For promotion/recovery/HA read [failover and recovery](docs/architecture/failover-recovery.md) and [threat model](docs/security/threat-model.md).
5. Validate against real PostgreSQL and the declared profile, record evidence, and update scope/status without claiming unsupported capabilities. Public performance claims need [benchmark methodology](docs/testing/benchmark-plan.md).
6. Before every commit, run `/graphify . --update` so the knowledge graph in `graphify-out/` reflects the files you changed. Do this after the last edit and before `git commit`; do not commit while the graph is stale. Include the updated `graphify-out/` in the same commit; its machine local files are excluded by `.gitignore`.

## Invariants

Keep writes on the verified primary. Preserve transaction and session ownership. Never replay uncertain dispatched work. Close uncertain pooled state. Fail closed on unknown role or unsafe routing. Health failure does not authorize promotion. Promotion requires a reviewed operation and independently verified fencing. Automatic failover and result caching stay disabled until separately approved and proven.

Use safe first party Rust and bounded resources. Native/unsafe exceptions require explicit justification and review. Credentials, cancellation secrets and sensitive SQL never belong in default telemetry. Administration requires authentication and authorization.

## Documentation and tools

Scope owns coarse work/status; specs own decisions; PRDs describe behavior; ADRs link decisions. Research is dated evidence, not proof of current runtime behavior. Apache 2.0 is the chosen project license; review per file and transitive notices before source reuse.

No project community skills or MCP dependencies have been installed. Optional stack discovery was deferred by the user on 2026-11-14. Do not install dependencies, scaffold all future crates, deploy infrastructure, or mutate a database as part of documentation work.

## Workspace conventions

Each rule below is owned by the linked file; edit it there.

- Every member inherits the five `[workspace.package]` fields and sets `[lints] workspace = true`, so `unsafe_code = "forbid"` applies to it ([workspace layout and manifests](docs/specs/0001-initial-architecture/index.md#workspace-layout-and-manifests)).
- A dependency is declared once under `[workspace.dependencies]` and taken with `workspace = true`, only when the current path uses it ([adopted dependencies](docs/specs/0001-initial-architecture/index.md#adopted-dependencies)). `Cargo.lock` is committed and its package set is part of `0001 AC-12`. No package has `dev-dependencies` or a build script.
- Neither package opens a socket or starts a thread or async runtime, and `apps/pool-cli`, `crates/pool-protocol`, `crates/pool-core` and `crates/pool-control` do not exist yet. The checks are `0001 AC-12` and `0001 AC-13` in [spec 0001](docs/specs/0001-initial-architecture/index.md#requirements).
- Configuration keys, exit status with command output, and log events follow the [shared conventions](docs/engineering-standards.md#shared-conventions). Public names come from the [glossary](docs/glossary.md).
- Tests use the built in `cargo test` harness only, in the files [spec 0001](docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them) assigns to each package. The checks you can run today are in [CONTRIBUTING.md](CONTRIBUTING.md).
- A commit names the Build plan step it implements ([change path](docs/specs/0001-initial-architecture/index.md#change-path)).

## Context files

- [apps/pool/AGENTS.md](apps/pool/AGENTS.md) (the `pool` binary: output, exit status and test rules for `check-config`)
- [crates/pool-config/AGENTS.md](crates/pool-config/AGENTS.md) (the validation library: its public boundary, findings and fixtures)
