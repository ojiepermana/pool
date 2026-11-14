# Documentation index

**Package date:** 2026-11-14. This index covers every generated Markdown document plus the project license. All application capabilities remain planned. A ratified foundation still does not mean it has been built.

Read the architecture decision for the chosen foundation, the scope for work order, PRDs for behavior, and research for dated evidence. The final [decision ledger](plan/decision-ledger.md) records your answers and the independent critique outcome.

## Package map

| Area | Purpose |
|---|---|
| Root guidance | Identity, contribution, license, security and agent workflow |
| Scope | Coarse queue and lifecycle status |
| Specs and ADR | Canonical architecture decision and its history index |
| Research | Baseline, competitors, protocol, dependencies, census and feasibility |
| Architecture | Data/control contracts, pooling, routing, topology, failover and deployment |
| Plan | Milestones, effort, risks, dependencies and decision record |
| PRDs | Ten proposed feature contracts and 41 acceptance criteria |
| Testing and security | Evidence strategy, driver coverage, benchmarks and threat model |

## Project root

| Path | Document |
|---|---|
| [AGENTS.md](../AGENTS.md) | POOL agent guide |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contributing to POOL |
| [README.md](../README.md) | pool |
| [SECURITY.md](../SECURITY.md) | Security policy |

## docs/adr

| Path | Document |
|---|---|
| [docs/adr/0001-initial-architecture.md](adr/0001-initial-architecture.md) | ADR 0001. Initial architecture |

## docs/architecture

| Path | Document |
|---|---|
| [docs/architecture/cluster-management.md](architecture/cluster-management.md) | Cluster management |
| [docs/architecture/connection-pooling.md](architecture/connection-pooling.md) | Connection pooling |
| [docs/architecture/control-plane.md](architecture/control-plane.md) | Control plane |
| [docs/architecture/data-plane.md](architecture/data-plane.md) | Data plane |
| [docs/architecture/deployment.md](architecture/deployment.md) | Deployment reference |
| [docs/architecture/failover-recovery.md](architecture/failover-recovery.md) | Failover and recovery |
| [docs/architecture/overview.md](architecture/overview.md) | Architecture overview |
| [docs/architecture/protocol-engine.md](architecture/protocol-engine.md) | Protocol engine |
| [docs/architecture/sql-routing.md](architecture/sql-routing.md) | SQL routing |

## docs

| Path | Document |
|---|---|
| [docs/engineering-standards.md](engineering-standards.md) | Engineering standards |
| [docs/feature-matrix.md](feature-matrix.md) | Feature inventory and parity matrix |
| [docs/implementation-order.md](implementation-order.md) | Implementation starting point |
| [docs/repository-structure.md](repository-structure.md) | Repository proposal |

## docs/plan

| Path | Document |
|---|---|
| [docs/plan/decision-ledger.md](plan/decision-ledger.md) | Planning assumptions and decisions |
| [docs/plan/dependencies.md](plan/dependencies.md) | Dependencies and critical path |
| [docs/plan/milestones.md](plan/milestones.md) | Milestones |
| [docs/plan/planning-validation.md](plan/planning-validation.md) | Planning package validation |
| [docs/plan/risk-register.md](plan/risk-register.md) | Risk register |
| [docs/plan/roadmap.md](plan/roadmap.md) | Roadmap |
| [docs/plan/timeline.md](plan/timeline.md) | Timeline and estimation model |

## docs/prd

| Path | Document |
|---|---|
| [docs/prd/001-core-proxy.md](prd/001-core-proxy.md) | Core proxy PRD |
| [docs/prd/002-connection-pooling.md](prd/002-connection-pooling.md) | Connection pooling PRD |
| [docs/prd/003-sql-routing.md](prd/003-sql-routing.md) | SQL routing PRD |
| [docs/prd/004-load-balancing.md](prd/004-load-balancing.md) | Load balancing PRD |
| [docs/prd/005-health-monitoring.md](prd/005-health-monitoring.md) | Health monitoring PRD |
| [docs/prd/006-cluster-management.md](prd/006-cluster-management.md) | Cluster management PRD |
| [docs/prd/007-manual-failover.md](prd/007-manual-failover.md) | Manual failover PRD |
| [docs/prd/008-high-availability.md](prd/008-high-availability.md) | High availability PRD |
| [docs/prd/009-query-cache.md](prd/009-query-cache.md) | Query cache PRD |
| [docs/prd/010-administration.md](prd/010-administration.md) | Administration PRD |

## docs/research

| Path | Document |
|---|---|
| [docs/research/competitor-analysis.md](research/competitor-analysis.md) | Competitor analysis |
| [docs/research/feasibility-study.md](research/feasibility-study.md) | Feasibility study |
| [docs/research/pgpool-baseline.md](research/pgpool-baseline.md) | Pgpool baseline |
| [docs/research/pgpool-configuration-inventory.md](research/pgpool-configuration-inventory.md) | Pgpool configuration coverage inventory |
| [docs/research/protocol-analysis.md](research/protocol-analysis.md) | PostgreSQL protocol analysis |
| [docs/research/technology-evaluation.md](research/technology-evaluation.md) | Rust technology evaluation |

## docs/scope

| Path | Document |
|---|---|
| [docs/scope/index.md](scope/index.md) | Scope: POOL, order and status index |
| [docs/scope/stack-and-workspace.md](scope/stack-and-workspace.md) | 1. Stack and workspace |
| [docs/scope/standards-and-tooling.md](scope/standards-and-tooling.md) | 2. Standards and tooling |
| [docs/scope/control-and-session-data-model.md](scope/control-and-session-data-model.md) | 3. Control and session data model |
| [docs/scope/one-real-primary-path.md](scope/one-real-primary-path.md) | 4. One real primary path |
| [docs/scope/clean-session-reuse.md](scope/clean-session-reuse.md) | 5. Clean session reuse |
| [docs/scope/protocol-breadth-and-cancellation.md](scope/protocol-breadth-and-cancellation.md) | 6. Protocol breadth and cancellation |
| [docs/scope/restricted-replica-sessions.md](scope/restricted-replica-sessions.md) | 7. Restricted replica sessions |
| [docs/scope/health-diagnostics-and-drain.md](scope/health-diagnostics-and-drain.md) | 8. Health diagnostics and drain |
| [docs/scope/safe-topology-administration.md](scope/safe-topology-administration.md) | 9. Safe topology administration |
| [docs/scope/fenced-manual-promotion.md](scope/fenced-manual-promotion.md) | 10. Fenced manual promotion |
| [docs/scope/release-qualification-harness.md](scope/release-qualification-harness.md) | 11. Release qualification harness |
| [docs/scope/multiple-proxy-authority.md](scope/multiple-proxy-authority.md) | 12. Multiple proxy authority |
| [docs/scope/checked-online-recovery.md](scope/checked-online-recovery.md) | 13. Checked online recovery |
| [docs/scope/advanced-pooling-and-routing.md](scope/advanced-pooling-and-routing.md) | 14. Advanced pooling and routing |
| [docs/scope/optional-result-cache.md](scope/optional-result-cache.md) | 15. Optional result cache |
| [docs/scope/additional-replication-modes.md](scope/additional-replication-modes.md) | 16. Additional replication modes |
| [docs/scope/administrative-dashboard.md](scope/administrative-dashboard.md) | 17. Administrative dashboard |

## docs/security

| Path | Document |
|---|---|
| [docs/security/security-requirements.md](security/security-requirements.md) | Security requirements |
| [docs/security/threat-model.md](security/threat-model.md) | Threat model |

## docs/specs/0001-initial-architecture

| Path | Document |
|---|---|
| [docs/specs/0001-initial-architecture/index.md](specs/0001-initial-architecture/index.md) | 0001. POOL initial architecture |
| [docs/specs/0001-initial-architecture/rationale.md](specs/0001-initial-architecture/rationale.md) | Initial architecture rationale |

## docs/testing

| Path | Document |
|---|---|
| [docs/testing/benchmark-plan.md](testing/benchmark-plan.md) | Benchmark plan |
| [docs/testing/compatibility-matrix.md](testing/compatibility-matrix.md) | Driver compatibility matrix |
| [docs/testing/strategy.md](testing/strategy.md) | Testing strategy |

## License

[LICENSE](../LICENSE) contains the official Apache License 2.0 text selected by you. Its appendix contains the standard example notice fields; those are part of the canonical license, not unfinished project documentation.

This index lists 66 other Markdown documents and itself, plus LICENSE. No source files, build scaffolding or deployment assets are represented as implemented.
