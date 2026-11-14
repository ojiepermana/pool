# POOL agent guide

POOL is a PostgreSQL 18 proxy, session pooler and cluster operations project in Rust. The current repository contains a planning package, not an implemented proxy. The user's current instruction stops after planning. Begin implementation only when explicitly requested.

## Workflow

1. Read [scope](docs/scope/index.md) and the relevant PRD from the [documentation index](docs/index.md). Before architecture or boundary changes, read [spec 0001](docs/specs/0001-initial-architecture/index.md) and the owning feature spec.
2. Inspect the actual code, manifest, Git state and environment before relying on documentation. There is no Cargo workspace yet. Do not report a planned command as an executed check.
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
