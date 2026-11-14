# Planning assumptions and decisions

**Date:** 2026-11-14. You accepted the assembled initial foundation after independent review and approved fixes. Feature PRDs remain proposed contracts. Your current instruction stops work after planning; implementation stays queued.

| Dimension | Source or proposed default | State and effect |
|---|---|---|
| Language and database | Your brief: Rust, PostgreSQL 18 | Fixed requirement |
| First deployment | Your brief: one proxy VM, primary and two replicas on Proxmox | Fixed requirement; physical host separation is unknown |
| Failover and caching | Your brief: manual first, automatic failover and cache disabled | Fixed safety boundary |
| References | Your brief requests official sources and release verification | Web research authorized; direct links included |
| Team | Two experienced systems engineers plus periodic independent specialist review | Confirmed planning baseline; specialist availability still needs staffing |
| Load | Lab tiers, no throughput promise before measurement | Confirmed; actual VM resources and workload remain unknown |
| License | Apache 2.0 | Confirmed; official license text installed in root LICENSE |
| Build approach | Tracer Bullet, a real protocol path before breadth | Confirmed |
| Verification depth | GA for data safety features | Confirmed: verification, tests, independent review, and documentation |
| Process boundary | One daemon with separate modules; separate CLI | Confirmed; split processes deferred |
| Routing | Primary default, explicit read only session profiles first | Confirmed; endpoint chosen at login, SQL admitted on pinned lease, lag screened at admission only, eventual reads, no primary fallback |
| Frontend authentication | Terminated SCRAM over TLS and optional mTLS, separately provisioned backend credentials | Confirmed boundary; implementation needs a feature spec |
| Local persistence | SQLite journal for operations, audit and approved writer/epoch/quarantine; never observed backend role truth | Confirmed; bootstrap once, reconcile durable authorization with fresh roles before restart admission |
| Runtime | Tokio with bounded tasks and queues | Confirmed |
| TLS | rustls and tokio-rustls, one crypto provider | Confirmed; exact feature graph reviewed at dependency adoption |
| Parser | pg_query with PostgreSQL 18 grammar | Confirmed; native code isolated and reviewed |
| Administration | Axum REST API plus CLI, protected local transport and mTLS for remote exposure | Confirmed; PCP compatibility deferred |
| Observability | Prometheus metrics and tracing logs, optional OTLP later | Confirmed |
| Public surfaces | PostgreSQL wire protocol, CLI, local authenticated administration API | Inferred infrastructure product; dashboard deferred |
| Business features | No hosted billing, marketing website, SEO, tracking SDK, or email service in this slice | Inferred from the open source infrastructure brief; no SaaS product was requested |
| Accessibility and localization | Clear CLI text, machine readable JSON, English docs initially | Proposal; a future dashboard needs its own accessibility and locale design |
| Compliance | Secrets and SQL may contain sensitive data; no certification target supplied | Security requirements apply; no SOC 2, PCI, or other compliance claim is made |
| Skills and MCP discovery | Not now, later | Confirmed deferral; no discovery or installation performed |
| Independent design critique | Another model requested and completed | All seven fixes approved and applied; targeted readback confirmed them in the foundation, then two supporting table/replay clarifications were aligned by the author |
| Assembled foundation | Accept, looks solid | Ratified by you on 2026-11-14; scope feature 1 design checked, execution queued, governing spec Proposed until built |

## Decisions before implementation

The initial architecture is ratified. Each safety sensitive feature still gets a scoped build specification with concrete defaults, interfaces, acceptance criteria, and source values. The PRDs in this package are proposed product contracts, not evidence that those features exist. Future amendments record an explicit decision rather than silently changing the accepted foundation.

For open choices, the numbered scope file identifies the owner role and the first slice that needs a decision. Use the [dependency handoffs](dependencies.md) to see which work is blocked. Record alternatives, the chosen value or interface, reviewer, date and affected criteria in that slice's spec. If a new choice changes a ratified safety contract, amend the governing decision and link the superseding spec here before building against it. A proposed default, an assigned role or a passing experiment alone does not ratify a change.

## Independent review resolutions

You requested a critique from another model, then explicitly chose to apply all recommended fixes. The review changed these decisions:

| Gap | Approved resolution | Owning document |
|---|---|---|
| Observed role was confused with authorized writer after restart | Persist approved writer, epoch and quarantine; fresh probes must agree; reconcile incomplete effects | Control plane and initial spec |
| Login selection depended on SQL not yet available; lag and role could change later | Select at login, admit later SQL on lease, eventual reads with admission screen, no primary fallback, retire candidate replica leases before promotion | SQL routing and balancing PRDs |
| Cancellation could arrive after lease reuse | Private backend keys, virtual frontend credentials, serialized retirement and physical backend closure after forwarding | Core proxy and pooling contracts |
| Credential selection authority was implicit | Operator map from verified principal/requested DB to permitted backend identity and secret reference/generation | Authentication and control contracts |
| Acquisition retry wording could imply session replacement | Retry only before initial lease and ReadyForQuery exposure, never after established session loss | Data plane |
| Library types appeared owned by the binary | Control owns observations, core owns routing inputs; daemon adapts between them | Repository and initial spec |
| POOL-only drain could imply a final WAL bound while direct writers remain | Require authoritative barrier across every write path before known final flush evidence; otherwise unknown loss window | Failover contract and PRD |

The independent critique is a design review, not an executable test, security audit, or certification of future code.
