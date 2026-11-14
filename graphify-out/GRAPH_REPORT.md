# Graph Report - pool  (2026-11-14)

## Corpus Check
- 69 files · ~45,939 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 504 nodes · 1028 edges · 30 communities
- Extraction: 73% EXTRACTED · 27% INFERRED · 0% AMBIGUOUS · INFERRED: 277 edges (avg confidence: 0.87)
- Token cost: 144,733 input · 0 output

## Community Hubs (Navigation)
- Feature Matrix and Invariants
- Agent Guides and Workflow
- Control Journal and Deployment
- Decision Ledger and Assumptions
- Security Features and Requirements
- Protocol Analysis and Contract
- Roadmap and Delivery Slices
- Repository Layout and Crates
- SQL Routing PRD
- Connection Pooling and Pool Identity
- Promotion and Split Brain Risks
- Cluster Health and Replica Lag
- Milestones and Critical Path
- Deployment, Failover and Fencing
- Technology Evaluation
- Pgpool Baseline and Config Coverage
- Feasibility Study
- Pgpool Option Families
- Core Safety Invariants
- Data Plane and Admission
- Competitor Analysis
- Architecture Overview and Leases
- Control Plane Records
- Fencing Provider Risks
- Fail Closed Routing
- Dependency Supply Chain
- Protocol Engine
- Transaction and Prepared Statements
- Replication Modes
- Cancellation Safety

## God Nodes (most connected - your core abstractions)
1. `Feature inventory and parity matrix` - 42 edges
2. `Scope: POOL, order and status index` - 32 edges
3. `Documentation index` - 31 edges
4. `Security requirements` - 30 edges
5. `Spec 0001. POOL initial architecture` - 27 edges
6. `Risk register` - 27 edges
7. `Testing strategy` - 26 edges
8. `pool README` - 22 edges
9. `Threat model` - 22 edges
10. `Planning assumptions and decisions` - 19 edges

## Surprising Connections (you probably didn't know these)
- `Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing)` --semantically_similar_to--> `POOL safety invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `License and source reuse policy` --semantically_similar_to--> `Competitor reference roles recommendation`  [INFERRED] [semantically similar]
  docs/research/pgpool-baseline.md → docs/research/competitor-analysis.md

## Hyperedges (group relationships)
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **Initial workspace: two applications and four internal crates** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Deferred features outside the current release queue** — docs_scope_012_multiple_proxy_authority, docs_scope_013_checked_online_recovery, docs_scope_014_advanced_pooling_and_routing, docs_scope_015_optional_result_cache, docs_scope_016_additional_replication_modes, docs_scope_017_administrative_dashboard [EXTRACTED 1.00]
- **Foundation phase features (stack, standards, data model)** — docs_scope_001_stack_and_workspace, docs_scope_002_standards_and_tooling, docs_scope_003_control_and_session_data_model [EXTRACTED 1.00]
- **V1 tracer bullet vertical slices 1 to 7 plus V1 gate** — docs_scope_004_one_real_primary_path, docs_scope_005_clean_session_reuse, docs_scope_006_protocol_breadth_and_cancellation, docs_scope_007_restricted_replica_sessions, docs_scope_008_health_diagnostics_and_drain, docs_scope_009_safe_topology_administration, docs_scope_010_fenced_manual_promotion, docs_scope_011_release_qualification_harness [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_plan_risk_register_r_05_incorrect_reuse_or_late_cancel, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_plan_risk_register_r_08_split_brain, docs_plan_risk_register_r_20_fencing_provider_unavailable, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_fencereceipt, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier, docs_plan_dependencies_independent_fencing_provider [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_075_automatic_failover, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (30 total, 0 thin omitted)

### Community 0 - "Feature Matrix and Invariants"
Cohesion: 0.06
Nodes (63): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: health failure does not authorize promotion, Invariant: keep writes on the verified primary, Feature inventory and parity matrix, Balancing features F-048 to F-053, Cache features F-088 to F-093, Cluster features F-061 to F-066 (registry, topology), Coverage limits of the feature inventory (+55 more)

### Community 1 - "Agent Guides and Workflow"
Cohesion: 0.07
Nodes (46): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet build approach, CLAUDE.md guidance for Claude Code (+38 more)

### Community 2 - "Control Journal and Deployment"
Cohesion: 0.07
Nodes (46): Persisted approved writer, epoch and quarantine, Axum REST administration API plus CLI, First deployment: one proxy VM, primary and two replicas on Proxmox, SQLite journal for operations, audit and approved writer/epoch/quarantine, Durable operation journal, External prerequisites, Health and topology model, Independent fencing provider (+38 more)

### Community 3 - "Decision Ledger and Assumptions"
Cohesion: 0.07
Nodes (42): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Authoritative barrier across every write path, Manual failover first; automatic failover and cache disabled, GA verification depth for data safety features, Independent review resolutions (seven approved fixes), Operator map from verified principal and database to backend identity (+34 more)

### Community 4 - "Security Features and Requirements"
Cohesion: 0.08
Nodes (37): Administration requires authentication and authorization, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-107 Rate and resource limits (admission, frame and auth CPU budgets), F-108 Secure reload (validate entire generation before publication) (+29 more)

### Community 5 - "Protocol Analysis and Contract"
Cohesion: 0.09
Nodes (26): Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming, Extended Query and pipelining behavior, Independent backend credentials per mapped login identity, Proposed protocol limits (64 KiB startup, 16 MiB message) (+18 more)

### Community 6 - "Roadmap and Delivery Slices"
Cohesion: 0.13
Nodes (23): Tracer Bullet build approach, Routing preview milestone, V1.5 milestone (multiple proxies, external authority), Walking slice milestone, Roadmap, Delivery slices, Phase 0 Research and architecture, Phase 1 Foundation (+15 more)

### Community 7 - "Repository Layout and Crates"
Cohesion: 0.19
Nodes (18): TopologySnapshot, Protocol finite state machine, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Documentation ownership, pool-config crate, pool-control crate (+10 more)

### Community 8 - "SQL Routing PRD"
Cohesion: 0.20
Nodes (17): SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), No primary fallback in V1, Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+9 more)

### Community 9 - "Connection Pooling and Pool Identity"
Cohesion: 0.20
Nodes (14): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Statement pooling (not planned for V1), Transaction pooling (deferred), Durable approved writer authority, Initial admission boundary (+6 more)

### Community 10 - "Promotion and Split Brain Risks"
Cohesion: 0.18
Nodes (13): Invariant: promotion requires a reviewed operation and independently verified fencing, F-067 Manual plan and promotion (operator plan plus verified fence), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Failover features F-067 to F-075, R-08 Split brain, R-09 Failover loses acknowledged commits (+5 more)

### Community 11 - "Cluster Health and Replica Lag"
Cohesion: 0.24
Nodes (13): Cluster management, Authenticated SQL health probes, Configured backend registration, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Observation record, Value sourcing table (+5 more)

### Community 12 - "Milestones and Critical Path"
Cohesion: 0.19
Nodes (13): One daemon with separate modules plus separate CLI, Dependencies and critical path, Critical path (protocol state, auth identity, cleanup, routing semantics, promotion safety), Dependency boundaries (acyclic crate graph), Milestones, Core MVP milestone, Eventual parity milestone, Release practices (+5 more)

### Community 13 - "Deployment, Failover and Fencing"
Cohesion: 0.29
Nodes (12): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+4 more)

### Community 14 - "Technology Evaluation"
Cohesion: 0.21
Nodes (12): Rust technology evaluation, Dependency adoption gate, axum 0.8.9 (administration endpoints), Hybrid parser policy, pg_query 18.1.0 and libpg_query (PostgreSQL raw parser), proptest 1.12.0 (state and framing properties), rusqlite 0.42.0 (local operation journal store), rustls 0.23.46 (TLS) (+4 more)

### Community 15 - "Pgpool Baseline and Config Coverage"
Cohesion: 0.24
Nodes (11): Configuration features F-115 to F-119, F-117 Pgpool parameter coverage (207 sample spellings mapped), Pgpool baseline, Baseline gaps (planning inventory, not parity certification), License and source reuse policy, PCP commands, SQL commands and pgpool_adm reference surface, Tagged sample and source manifest (V4_7_3), Pgpool configuration coverage inventory (+3 more)

### Community 16 - "Feasibility Study"
Cohesion: 0.18
Nodes (11): Feasibility study, Automatic promotion (not ready for initial release), Go or no go milestones, Commercial and maintenance assumptions, Manual promotion (feasible with external dependencies), Query result caching (high uncertainty), Session pooling feasibility (high feasibility, high correctness risk), Transparent arbitrary read splitting (low confidence) (+3 more)

### Community 17 - "Pgpool Option Families"
Cohesion: 0.20
Nodes (10): Pgpool Watchdog coordination, Connections and identity option family (48 options), Logs, files and reset option family (28 options), Relation metadata option family (6 options), Canonical runtime declaration census (RT-001 to RT-186), Source-only delta names (hidden aliases, health_check_test, replication_mode), Streaming and health option family (24 options), Streaming sample census (CFG-001 to CFG-207, 207 spellings) (+2 more)

### Community 18 - "Core Safety Invariants"
Cohesion: 0.31
Nodes (9): Invariant: close uncertain pooled state, POOL safety invariants, Invariant: never replay uncertain dispatched work, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, F-030 Validation and reset (close uncertain sessions), Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing), R-03 Session state leakage, R-10 Ambiguous COMMIT replay (+1 more)

### Community 19 - "Data Plane and Admission"
Cohesion: 0.25
Nodes (9): Pool limits and operations, Data plane, Frontend owner task, Operation admission boundary, Provisional resource defaults and backpressure, Application path sequence, Routing decision matrix, SQL classification through PostgreSQL 18 raw grammar (+1 more)

### Community 20 - "Competitor Analysis"
Cohesion: 0.47
Nodes (9): Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC), PgCat (Rust/Tokio routing proxy, MIT), Competitor reference roles recommendation, Pgpool-II 4.7.3 (selected baseline) (+1 more)

### Community 21 - "Architecture Overview and Leases"
Cohesion: 0.32
Nodes (8): No automatic retry after dispatch, Future multi proxy deployment, Architecture overview, Failure behavior (no transparent failover, no implicit replay), Lease generation, Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), External etcd for future coordination, Per lease virtual cancellation credentials

### Community 22 - "Control Plane Records"
Cohesion: 0.52
Nodes (7): Drain, attach and quarantine operations, Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer)

### Community 23 - "Fencing Provider Risks"
Cohesion: 0.33
Nodes (7): F-069 Old primary fencing (external verified exclusion), Recovery features F-076 to F-080 (rejoin, pg_rewind, base backup), R-20 Fencing provider unavailable, Operational ownership boundaries, SEC-9 Allowlisted structured fencing/recovery adapters, Fencing infrastructure, T-09 Command injection through fencing/recovery hooks

### Community 24 - "Fail Closed Routing"
Cohesion: 0.40
Nodes (6): Invariant: fail closed on unknown role or unsafe routing, F-033 SQL AST classification (PostgreSQL 18 grammar adapter), F-045 Unknown and failed parsing (keep primary lease, reject on restricted replica), Routing features F-033 to F-047 (SQL classification, primary/replica), R-02 Incorrect SQL classification, T-06 Unsafe replica routing

### Community 25 - "Dependency Supply Chain"
Cohesion: 0.40
Nodes (6): Safe first party Rust and bounded resources, R-16 Dependency drift or abandoned crate, R-17 License conflict or missing notices, SEC-13 Inventory dependencies and native components with notices and advisories, Residual risks, T-13 Dependency or native parser compromise

### Community 26 - "Protocol Engine"
Cohesion: 0.40
Nodes (5): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Transaction state tracking (ReadyForQuery status), Data and protocol safety rules

### Community 27 - "Transaction and Prepared Statements"
Cohesion: 0.50
Nodes (4): Invariant: preserve transaction and session ownership, F-011 Prepared statements (session ownership, no cross backend replay), R-04 Prepared statement incompatibility, R-06 Transaction consistency change

### Community 28 - "Replication Modes"
Cohesion: 0.50
Nodes (4): Pgpool baseline modes and POOL dispositions, native_replication and snapshot_isolation modes (deferred), streaming_replication mode (initial operating model), Replication and routing option family (17 options)

### Community 29 - "Cancellation Safety"
Cohesion: 1.00
Nodes (3): F-018 CancelRequest and BackendKeyData (virtual per lease keys), R-05 Incorrect reuse or late cancel, T-05 Late cancellation affects another user

## Knowledge Gaps
- **59 isolated node(s):** `Dependency and configuration change rules`, `Scope file ownership rules`, `Apache 2.0 license`, `Prometheus metrics and tracing logs`, `rustls and tokio-rustls with one crypto provider` (+54 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Feature inventory and parity matrix` connect `Feature Matrix and Invariants` to `Agent Guides and Workflow`, `Control Journal and Deployment`, `Decision Ledger and Assumptions`, `Security Features and Requirements`, `Roadmap and Delivery Slices`, `SQL Routing PRD`, `Promotion and Split Brain Risks`, `Milestones and Critical Path`, `Pgpool Baseline and Config Coverage`, `Fencing Provider Risks`, `Fail Closed Routing`?**
  _High betweenness centrality (0.237) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Agent Guides and Workflow` to `Feature Matrix and Invariants`, `Control Journal and Deployment`, `Decision Ledger and Assumptions`, `Roadmap and Delivery Slices`, `Repository Layout and Crates`, `SQL Routing PRD`, `Connection Pooling and Pool Identity`, `Technology Evaluation`, `Architecture Overview and Leases`?**
  _High betweenness centrality (0.167) - this node is a cross-community bridge._
- **Why does `POOL agent guide (AGENTS.md)` connect `Agent Guides and Workflow` to `Feature Matrix and Invariants`, `Security Features and Requirements`, `Protocol Analysis and Contract`, `Deployment, Failover and Fencing`, `Core Safety Invariants`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **What connects `Dependency and configuration change rules`, `Scope file ownership rules`, `Apache 2.0 license` to the rest of the system?**
  _59 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Feature Matrix and Invariants` be split into smaller, more focused modules?**
  _Cohesion score 0.06093189964157706 - nodes in this community are weakly interconnected._
- **Should `Agent Guides and Workflow` be split into smaller, more focused modules?**
  _Cohesion score 0.07439613526570048 - nodes in this community are weakly interconnected._
- **Should `Control Journal and Deployment` be split into smaller, more focused modules?**
  _Cohesion score 0.06956521739130435 - nodes in this community are weakly interconnected._