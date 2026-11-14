# Graph Report - pool  (2026-11-14)

## Corpus Check
- 40 files · ~74,817 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 591 nodes · 1521 edges · 15 communities
- Extraction: 73% EXTRACTED · 27% INFERRED · 0% AMBIGUOUS · INFERRED: 411 edges (avg confidence: 0.87)
- Token cost: 289,222 input · 82,073 output

## Community Hubs (Navigation)
- Agent Guides and Workflow
- Cluster and Control Plane Records
- Security and Engineering Standards
- Stack, Competitors and Feasibility
- Pooling, Data Plane and Cancellation
- Decision Ledger and Failover Authority
- Core Safety Invariants
- Protocol Contract and Core MVP
- Feature Inventory and Parity
- Health Monitoring and Admin API
- Roadmap, Milestones and Slices
- Repository Layout and Crates
- Core Proxy and Pooling PRDs
- Operation Journal and Idempotency
- Benchmark KPIs and Gates

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 74 edges
2. `Glossary` - 49 edges
3. `Feature inventory and parity matrix` - 47 edges
4. `Testing strategy` - 43 edges
5. `Scope: POOL, order and status index` - 42 edges
6. `Spec 0001. POOL initial architecture` - 38 edges
7. `Security requirements` - 37 edges
8. `Risk register` - 35 edges
9. `4. One real primary path (planned)` - 26 edges
10. `Health monitoring PRD` - 25 edges

## Surprising Connections (you probably didn't know these)
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Engineering Workflow` --semantically_similar_to--> `Agent Workflow`  [INFERRED] [semantically similar]
  docs/engineering-standards.md → AGENTS.md
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md

## Hyperedges (group relationships)
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Foundation phase features (stack, standards, data model)** — docs_scope_001_stack_and_workspace, docs_scope_002_standards_and_tooling, docs_scope_003_control_and_session_data_model [EXTRACTED 1.00]
- **V1 tracer bullet vertical slices 1 to 7 plus V1 gate** — docs_scope_004_one_real_primary_path, docs_scope_005_clean_session_reuse, docs_scope_006_protocol_breadth_and_cancellation, docs_scope_007_restricted_replica_sessions, docs_scope_008_health_diagnostics_and_drain, docs_scope_009_safe_topology_administration, docs_scope_010_fenced_manual_promotion, docs_scope_011_release_qualification_harness [EXTRACTED 1.00]
- **Deferred features outside the current release queue** — docs_scope_012_multiple_proxy_authority, docs_scope_013_checked_online_recovery, docs_scope_014_advanced_pooling_and_routing, docs_scope_015_optional_result_cache, docs_scope_016_additional_replication_modes, docs_scope_017_administrative_dashboard [EXTRACTED 1.00]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (15 total, 0 thin omitted)

### Community 0 - "Agent Guides and Workflow"
Cohesion: 0.07
Nodes (82): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach (+74 more)

### Community 1 - "Cluster and Control Plane Records"
Cohesion: 0.05
Nodes (66): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Pool key (pool identity) (+58 more)

### Community 2 - "Security and Engineering Standards"
Cohesion: 0.05
Nodes (63): Administration requires authentication and authorization, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Safe first party Rust and bounded resources, Dependency and Configuration Change Rules, Rust and Runtime Standards, Security and Review Requirements, F-069 Old primary fencing (external verified exclusion), F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping) (+55 more)

### Community 3 - "Stack, Competitors and Feasibility"
Cohesion: 0.05
Nodes (57): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause) (+49 more)

### Community 4 - "Pooling, Data Plane and Cancellation"
Cohesion: 0.07
Nodes (53): Virtual Cancellation Model, Separate Frontend and Backend Identity, Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool limits and operations, Statement pooling (not planned for V1), Data plane (+45 more)

### Community 5 - "Decision Ledger and Failover Authority"
Cohesion: 0.06
Nodes (52): F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Fence Receipt, Policy Version, Write Barrier, Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path (+44 more)

### Community 6 - "Core Safety Invariants"
Cohesion: 0.08
Nodes (38): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: preserve transaction and session ownership, Invariant: promotion requires a reviewed operation and independently verified fencing (+30 more)

### Community 7 - "Protocol Contract and Core MVP"
Cohesion: 0.07
Nodes (34): Invariant: close uncertain pooled state, F-030 Validation and reset (close uncertain sessions), Protocol Features (F-001 to F-022), Backend Login, Support Profile, Core MVP, R-01 Protocol incompatibility, Proposed stopping rules (+26 more)

### Community 8 - "Feature Inventory and Parity"
Cohesion: 0.09
Nodes (34): Session Model (pinned backend lease), Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-024 Transaction Mode (refused, session pooling only), F-024 Transaction mode pooling (refused; session pooling only), F-056 Primary Standby Role Observation (+26 more)

### Community 9 - "Health Monitoring and Admin API"
Cohesion: 0.11
Nodes (30): Health features F-054 to F-060, Axum REST administration API plus CLI, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control, Health monitoring failure scenarios (SQL failure, revoked permission, clock jump, replay pause, primary isolation) (+22 more)

### Community 10 - "Roadmap, Milestones and Slices"
Cohesion: 0.11
Nodes (28): HA Features (F-081 to F-087), Tracer Bullet build approach, Routing Preview, V1.5 Milestone, Walking Slice Milestone, R-21 Proxmox host correlated failure, R-24 Walking slice exceeds its estimate, Roadmap (+20 more)

### Community 11 - "Repository Layout and Crates"
Cohesion: 0.23
Nodes (22): Data Plane vs Control Plane Separation, Planned Architecture, ADR 0001. Initial architecture, Topology Snapshot, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI (+14 more)

### Community 12 - "Core Proxy and Pooling PRDs"
Cohesion: 0.14
Nodes (20): Retry only before initial lease and ReadyForQuery exposure, Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Terminated SCRAM over TLS with optional mTLS and separate backend credentials, Core proxy PRD, Bounded admission, deadlines and graceful teardown, Cancellation forwarded only through the live lease owner, CancellationEntry (+12 more)

### Community 13 - "Operation Journal and Idempotency"
Cohesion: 0.31
Nodes (9): F-071 Uncertain promotion result (durable intent and reconciliation), Expected Generation, Operation (journaled administrative action), Request ID (idempotency key), R-18 Journal disk loss or replay, Operational security defaults, SEC-7 Persist administrative intent before an external effect, T-11 Journal tampering, loss or disk full (+1 more)

### Community 14 - "Benchmark KPIs and Gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

## Knowledge Gaps
- **52 isolated node(s):** `Failover safety KPI (zero unfenced promotions or silent replays)`, `pgbench and purpose built workload generator`, `Benchmark workload matrix`, `Security triage scope`, `Configured backend registration` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Agent Guides and Workflow` to `Cluster and Control Plane Records`, `Security and Engineering Standards`, `Stack, Competitors and Feasibility`, `Pooling, Data Plane and Cancellation`, `Decision Ledger and Failover Authority`, `Core Safety Invariants`, `Protocol Contract and Core MVP`, `Feature Inventory and Parity`, `Health Monitoring and Admin API`, `Roadmap, Milestones and Slices`, `Repository Layout and Crates`, `Core Proxy and Pooling PRDs`?**
  _High betweenness centrality (0.431) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Feature Inventory and Parity` to `Agent Guides and Workflow`, `Security and Engineering Standards`, `Stack, Competitors and Feasibility`, `Pooling, Data Plane and Cancellation`, `Decision Ledger and Failover Authority`, `Core Safety Invariants`, `Protocol Contract and Core MVP`, `Health Monitoring and Admin API`, `Roadmap, Milestones and Slices`, `Core Proxy and Pooling PRDs`?**
  _High betweenness centrality (0.115) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Repository Layout and Crates` to `Agent Guides and Workflow`, `Cluster and Control Plane Records`, `Stack, Competitors and Feasibility`, `Pooling, Data Plane and Cancellation`, `Decision Ledger and Failover Authority`, `Core Safety Invariants`, `Health Monitoring and Admin API`, `Roadmap, Milestones and Slices`, `Core Proxy and Pooling PRDs`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Failover safety KPI (zero unfenced promotions or silent replays)`, `pgbench and purpose built workload generator`, `Benchmark workload matrix` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Agent Guides and Workflow` be split into smaller, more focused modules?**
  _Cohesion score 0.07136404697380307 - nodes in this community are weakly interconnected._