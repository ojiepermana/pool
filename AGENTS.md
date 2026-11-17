# POOL agent guide

POOL is a PostgreSQL 18 proxy, session pooler and cluster operations project in Rust. The current repository contains the planning package, the first workspace slice (scope 001: `apps/pool`, `crates/pool-config` and the offline `pool check-config` command) and the required checks of scope 002 (`scripts/check.sh`, `tools/repo-check`, `deny.toml` and the CI definition), and the build of scope 003 (the configuration keys for clusters, nodes and the identity map, and `crates/pool-control`, the authority rules as plain functions), and the first real path of scope 004 (`crates/pool-protocol`, `crates/pool-core`, the journal on disk in `pool-control`, the probe, and the commands `pool bootstrap`, `pool show-authority` and `pool run`). `pool run` serves one path: a TLS client, authenticated with SCRAM, on the verified primary, with a new backend for each session. It is not a full proxy: there is no backend reuse, no extended protocol, no replica session and no administration interface. The user asked for scope 001 and then scope 002 on 2026-11-14, and for scope 003 and scope 004 on 2026-11-15, and nothing beyond them. Begin further implementation only when explicitly requested.

For clean session reuse, credential rotation, cleanup or physical backend budgets, read [spec 0005](docs/specs/0005-clean-session-reuse/index.md). Scope 005 is built and verified, extending the dedicated scope 004 behavior described above with a conservative reusable profile, bounded quarantine, nested physical budgets and generation retirement. Other supported simple queries retain dedicated connections. It is the latest explicitly requested slice; further features still need an explicit request.

## Workflow

1. Read [scope](docs/scope/index.md) and the relevant PRD from the [documentation index](docs/index.md). Before architecture or boundary changes, read [spec 0001](docs/specs/0001-initial-architecture/index.md) and the owning feature spec.
2. Inspect the actual code, manifest, Git state and environment before relying on documentation. The Cargo workspace holds the `pool`, `pool-config`, `pool-control`, `pool-core` and `pool-protocol` packages. CI is defined in `.github/workflows/ci.yml`, and each of its jobs runs `scripts/check.sh <name>`, which you can run yourself ([spec 0002](docs/specs/0002-standards-and-tooling/index.md#the-check-entry-point)). Do not report a planned command as an executed check.
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
- A dependency is declared once under `[workspace.dependencies]` and taken with `workspace = true`, only when the current path uses it ([adopted dependencies](docs/specs/0001-initial-architecture/index.md#adopted-dependencies)). The exception that spec 0003 wrote for the `pool-control` entry ended in scope 004, where `apps/pool` takes it ([adopted dependencies of spec 0004](docs/specs/0004-one-real-primary-path/index.md#adopted-dependencies)). `Cargo.lock` is committed and its package set is part of `0001 AC-12`, `0003 AC-13` and `0004 AC-3`. `.cargo/config.toml` makes every build compile AWS-LC and SQLite from the source inside the locked crates ([native code](docs/specs/0004-one-real-primary-path/index.md#native-code)). No package has `dev-dependencies` or a build script.
- Sockets, the async runtime, TLS and SQLite may be named only where the baseline of rule W3 allows them ([source guard](docs/specs/0004-one-real-primary-path/index.md#source-guard)): `rusqlite` only in `crates/pool-control/src/journal.rs` and the tests of `pool-control`, `aws_lc_rs` only in `pool-core`, and no code starts a thread of its own ([one writer](docs/specs/0004-one-real-primary-path/index.md#one-writer)). `pool-protocol` has no dependency. `pool-core` and `pool-control` never depend on each other. `apps/pool-cli` does not exist yet. The checks are `0004 AC-1` and `0004 AC-2` in [spec 0004](docs/specs/0004-one-real-primary-path/index.md#requirements). `0001 AC-12`, `0001 AC-13` and `0003 AC-13` describe earlier shapes of the workspace.
- Configuration keys, exit status with command output, and log events follow the [shared conventions](docs/engineering-standards.md#shared-conventions). Public names come from the [glossary](docs/glossary.md).
- Tests use the built in `cargo test` harness only, in the files [spec 0001](docs/specs/0001-initial-architecture/index.md#tests-and-who-owns-them), [spec 0003](docs/specs/0003-control-and-session-data-model/index.md#tests-and-who-owns-them) and [spec 0004](docs/specs/0004-one-real-primary-path/index.md#tests-and-who-owns-them) assign to each package. The tests against real PostgreSQL run only through `scripts/check.sh integration`, which needs Docker. The checks you can run today are in [CONTRIBUTING.md](CONTRIBUTING.md).
- `tools/repo-check/` (the `pool-repo-check` program), `crates/pool-config/fuzz/` and `crates/pool-protocol/fuzz/` are packages outside the workspace, each with its own lockfile. A change to a locked package set, to a dependency between workspace packages or to the CI files also changes a baseline under `tools/repo-check/baseline/` or the rule text in [spec 0002](docs/specs/0002-standards-and-tooling/index.md#the-repository-check-program).
- A commit names the Build plan step it implements ([change path](docs/specs/0001-initial-architecture/index.md#change-path)).
- A change reaches `main` only through a pull request whose seven checks passed and whose review threads are resolved. A push to `main` is refused by the branch rule ([spec 0002](docs/specs/0002-standards-and-tooling/index.md#branch-rule-and-change-path)). Writing to GitHub needs the user's permission each time.

## Context files

- [apps/pool/AGENTS.md](apps/pool/AGENTS.md) (the `pool` binary: the four commands, the daemon, output, exit status and test rules)
- [crates/pool-config/AGENTS.md](crates/pool-config/AGENTS.md) (the validation library: its public boundary, findings and fixtures)
- [crates/pool-control/AGENTS.md](crates/pool-control/AGENTS.md) (the authority rules: start classification, bootstrap and writer admission, and the journal on disk)
- [crates/pool-protocol/AGENTS.md](crates/pool-protocol/AGENTS.md) (the wire format: startup packets, message headers and SCRAM messages, with no dependency)
- [crates/pool-core/AGENTS.md](crates/pool-core/AGENTS.md) (TLS, frontend authentication, the backend connection, the session and the route view)
