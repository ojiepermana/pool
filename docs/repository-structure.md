# Repository proposal

This is a proposed target layout. Only documentation and the chosen project license are created during planning. The initial workspace would contain two applications and four internal crates; you can defer every other crate until a real interface boundary emerges.

```text
pool/
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  rustfmt.toml
  clippy.toml
  LICENSE
  README.md
  AGENTS.md
  CONTRIBUTING.md
  SECURITY.md
  CODE_OF_CONDUCT.md
  apps/
    pool/
    pool-cli/
  crates/
    pool-protocol/
    pool-config/
    pool-core/
    pool-control/
  tests/
    protocol/
    integration/
    compatibility/
    performance/
    failover/
    chaos/
  benchmarks/
  examples/
  deploy/
    docker/
    systemd/
    proxmox/
  docs/
    scope/
    specs/
    research/
    architecture/
    plan/
    prd/
    adr/
    testing/
    operations/
    security/
  .github/
    workflows/
    ISSUE_TEMPLATE/
```

## Responsibilities and public interfaces

| Unit | Responsibility | Public boundary | Permitted internal dependencies |
|---|---|---|---|
| pool-protocol | Checked frames, message types, protocol state | Decoder/encoder, validated message types, state events | None of the other POOL crates |
| pool-config | Typed syntax and semantic config validation | ValidatedConfig, IDs, profiles, generations, redacted summaries | None of the other POOL crates |
| pool-core | Auth integration, socket lifecycle, leases, pool/routing/balance modules | Session supervisor, owned Lease, RouteDecision, core route view | protocol, config |
| pool-control | Monitoring, topology, journal, safe operations, administration | Control observations/snapshots, OperationService, admin handlers | config; PostgreSQL driver and SQLite adapters |
| apps/pool | Compose runtime, listeners, adapters and shutdown | Executable/config/exit contract | All four internal crates |
| apps/pool-cli | Human/JSON administration client | CLI and versioned API schema | config initially; add a small shared API crate only if duplication warrants it |

Core and control use independently owned types at their boundaries. The daemon maps control snapshots into the core route view. A trait accepted by core can also be implemented in the composition layer. This avoids a hidden core to control dependency. Configuration types should not become a generic dumping ground for every shared runtime record.

## Merge or defer the candidate crates

Authentication, connection, pooling, SQL adapter, router, balancer and backend are modules inside core initially. Health, topology, failover, recovery and admin are modules inside control as they become necessary. Metrics belongs beside instrumented behavior with a composed registry. Security is policy across those modules, not a universal utility crate. There is no general `common` crate.

The controller executable, consensus crate, cache crate, dashboard and middleware replication crates are deferred. If a component later gains independent consumers, dependency weight, versioning, ownership or fuzzing/build isolation needs, extraction gets a documented decision. Acyclic internal dependencies are checked in CI from Cargo metadata once a workspace exists.

## Documentation ownership

Scope tracks coarse features and status. Specs own architecture and later implementation decisions. PRDs state product behavior. ADR files index canonical decisions. Research stores dated evidence. Architecture docs explain contracts. Testing docs define proof. Operations docs explain deployment/recovery. Update a fact in its owning file and link from other views instead of maintaining contradictory copies.
