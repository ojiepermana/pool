# Documentation index

**Package date:** 2026-11-14. This index covers every generated Markdown document plus the project license. All application capabilities remain planned. A ratified foundation still does not mean it has been built.

For the next development task, start with the [scope index](scope/index.md) and the [handoff for later slices](implementation-order.md#handoff-for-later-slices). Use the [readiness review](plan/planning-validation.md#development-readiness-review) to see what changed and which decisions still gate later work.

## Where a change belongs

| If the question is… | Update this owner | Link from other views |
|---|---|---|
| What is in scope, in what order, and what is its status? | The [numbered feature and scope index](scope/index.md) | Plan references feature numbers. A feature may deliver parts of several PRDs. |
| What must a user or operator observe? | The relevant [PRD](#docsprd), including its acceptance criteria | Cite document and local ID together, e.g. `PRD-001 AC-1`. IDs stay stable when wording is clarified; retire superseded criteria explicitly. |
| How does the product meet that contract? | The owning feature spec; [0001](specs/0001-initial-architecture/index.md) governs the foundation | Architecture explains interactions; the ADR points to the decision. A PRD's suggested types and defaults are inputs until the spec settles them. |
| What blocks delivery, and what qualifies a release? | [Dependencies](plan/dependencies.md), [risks](plan/risk-register.md) or [milestones](plan/milestones.md), according to the question | Record a decision once in its spec; the [ledger](plan/decision-ledger.md) records its disposition. |
| How is a requirement proved? | [Testing strategy](testing/strategy.md#acceptance-evidence) for the method; the slice verification record for results | Scope and release reports link to that evidence. Use the [driver matrix](testing/compatibility-matrix.md) for the supported profile. |
| What is a thing called? | The [glossary](glossary.md) | Use its names for types, configuration keys, metrics and API fields. Other documents link to the term instead of defining it again. |
| Which rule do all slices share for keys, errors, metrics or output? | [Shared conventions](engineering-standards.md#shared-conventions) | The first slice that exposes a surface records the rule once. Later specs cite it. |
| What global rule or upstream behavior applies? | [Engineering standards](engineering-standards.md), [security requirements](security/security-requirements.md) or dated research | The [feature inventory](feature-matrix.md) records parity dispositions. A research entry is neither a release commitment nor a test result. |

When one change crosses these boundaries, change the owner first and follow its references to the affected scope, plan and tests. Unaffected documents need no matching paragraph. Cite by stable ID and never copy the text: `PRD-001 AC-1`, `PRD-001 FR-2` for a numbered functional requirement, `0001 SC-4` for a safety contract, `0001 AC-1` for a criterion that exists only in a feature spec, and `SEC-3`, `F-018`, `R-07` or `T-05` for security requirements, inventory rows, risks and threats. All ten PRDs are proposed contracts, subject to these shared standards and the governing spec.

## Project root

| Path | Document |
|---|---|
| [.github/pull_request_template.md](../.github/pull_request_template.md) | Pull request template |
| [AGENTS.md](../AGENTS.md) | POOL agent guide |
| [CHANGELOG.md](../CHANGELOG.md) | Changelog |
| [CLAUDE.md](../CLAUDE.md) | Claude Code guidance |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contributing to POOL |
| [README.md](../README.md) | pool |
| [SECURITY.md](../SECURITY.md) | Security policy |

## apps/pool

| Path | Document |
|---|---|
| [apps/pool/AGENTS.md](../apps/pool/AGENTS.md) | apps/pool agent guide |
| [apps/pool/CLAUDE.md](../apps/pool/CLAUDE.md) | Claude Code pointer to apps/pool/AGENTS.md |

## crates/pool-config

| Path | Document |
|---|---|
| [crates/pool-config/AGENTS.md](../crates/pool-config/AGENTS.md) | crates/pool-config agent guide |
| [crates/pool-config/CLAUDE.md](../crates/pool-config/CLAUDE.md) | Claude Code pointer to crates/pool-config/AGENTS.md |

## docs/adr

| Path | Document |
|---|---|
| [docs/adr/0001-initial-architecture.md](adr/0001-initial-architecture.md) | ADR 0001. Initial architecture |
| [docs/adr/0002-standards-and-tooling.md](adr/0002-standards-and-tooling.md) | ADR 0002. Standards and tooling |

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
| [docs/glossary.md](glossary.md) | Glossary |
| [docs/implementation-order.md](implementation-order.md) | Starting development |
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

## docs/reviews

| Path | Document |
|---|---|
| [docs/reviews/2026-11-14-scope-001-stack-and-workspace.md](reviews/2026-11-14-scope-001-stack-and-workspace.md) | Review. Scope 001, stack and workspace (2026-11-14) |

## docs/scope

| Path | Document |
|---|---|
| [docs/scope/index.md](scope/index.md) | Scope: POOL, order and status index |
| [docs/scope/001-stack-and-workspace.md](scope/001-stack-and-workspace.md) | 1. Stack and workspace |
| [docs/scope/002-standards-and-tooling.md](scope/002-standards-and-tooling.md) | 2. Standards and tooling |
| [docs/scope/003-control-and-session-data-model.md](scope/003-control-and-session-data-model.md) | 3. Control and session data model |
| [docs/scope/004-one-real-primary-path.md](scope/004-one-real-primary-path.md) | 4. One real primary path |
| [docs/scope/005-clean-session-reuse.md](scope/005-clean-session-reuse.md) | 5. Clean session reuse |
| [docs/scope/006-protocol-breadth-and-cancellation.md](scope/006-protocol-breadth-and-cancellation.md) | 6. Protocol breadth and cancellation |
| [docs/scope/007-restricted-replica-sessions.md](scope/007-restricted-replica-sessions.md) | 7. Restricted replica sessions |
| [docs/scope/008-health-diagnostics-and-drain.md](scope/008-health-diagnostics-and-drain.md) | 8. Health diagnostics and drain |
| [docs/scope/009-safe-topology-administration.md](scope/009-safe-topology-administration.md) | 9. Safe topology administration |
| [docs/scope/010-fenced-manual-promotion.md](scope/010-fenced-manual-promotion.md) | 10. Fenced manual promotion |
| [docs/scope/011-release-qualification-harness.md](scope/011-release-qualification-harness.md) | 11. Release qualification harness |
| [docs/scope/012-multiple-proxy-authority.md](scope/012-multiple-proxy-authority.md) | 12. Multiple proxy authority |
| [docs/scope/013-checked-online-recovery.md](scope/013-checked-online-recovery.md) | 13. Checked online recovery |
| [docs/scope/014-advanced-pooling-and-routing.md](scope/014-advanced-pooling-and-routing.md) | 14. Advanced pooling and routing |
| [docs/scope/015-optional-result-cache.md](scope/015-optional-result-cache.md) | 15. Optional result cache |
| [docs/scope/016-additional-replication-modes.md](scope/016-additional-replication-modes.md) | 16. Additional replication modes |
| [docs/scope/017-administrative-dashboard.md](scope/017-administrative-dashboard.md) | 17. Administrative dashboard |

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
| [docs/specs/0001-initial-architecture/verify.md](specs/0001-initial-architecture/verify.md) | 0001. Verification record |

## docs/specs/0002-standards-and-tooling

| Path | Document |
|---|---|
| [docs/specs/0002-standards-and-tooling/index.md](specs/0002-standards-and-tooling/index.md) | 0002. Standards and tooling |
| [docs/specs/0002-standards-and-tooling/rationale.md](specs/0002-standards-and-tooling/rationale.md) | Standards and tooling rationale |
| [docs/specs/0002-standards-and-tooling/verify.md](specs/0002-standards-and-tooling/verify.md) | 0002. Verification record |

## docs/testing

| Path | Document |
|---|---|
| [docs/testing/benchmark-plan.md](testing/benchmark-plan.md) | Benchmark plan |
| [docs/testing/compatibility-matrix.md](testing/compatibility-matrix.md) | Driver compatibility matrix |
| [docs/testing/strategy.md](testing/strategy.md) | Testing strategy |

## License

[LICENSE](../LICENSE) contains the official Apache License 2.0 text selected by you. Its appendix contains the standard example notice fields; those are part of the canonical license, not unfinished project documentation.

The [planning validation record](plan/planning-validation.md) describes the coverage and limits of the documentation checks.
