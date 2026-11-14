# Graph Report - pool  (2026-11-14)

## Corpus Check
- 3 files · ~48,157 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 639 nodes · 1648 edges · 25 communities
- Extraction: 73% EXTRACTED · 27% INFERRED · 0% AMBIGUOUS · INFERRED: 438 edges (avg confidence: 0.87)
- Token cost: 109,517 input · 0 output

## Community Hubs (Navigation)
- Scope Index and Feature Inventory
- Security and Engineering Standards
- Decision Ledger and Routing PRDs
- Scope 001 Coordination Brief and Log
- Architecture Spec and Repository Layout
- Testing Strategy and Release Gates
- Core Safety Invariants
- Protocol Contract and Driver Compatibility
- Health Monitoring and Admin API
- Threat Model and Fencing
- Manual Failover and Recovery
- Glossary and Routing Profiles
- Connection Pooling and Safety Contracts
- Roadmap and High Availability
- Rust Technology Evaluation
- Pgpool Baseline and Configuration Coverage
- SQL Routing and Replica Profiles
- Operation Journal and Control Records
- Feasibility Study Findings
- Pgpool Option Families
- Session Leases and Admission
- Competitor Analysis
- Backend Health and Drain Operations
- Approved Writer and Topology Epoch
- Pgpool Baseline Modes

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 74 edges
2. `Glossary` - 49 edges
3. `Feature inventory and parity matrix` - 47 edges
4. `Testing strategy` - 43 edges
5. `Scope: POOL, order and status index` - 42 edges
6. `Spec 0001. POOL initial architecture` - 39 edges
7. `Security requirements` - 37 edges
8. `Risk register` - 34 edges
9. `4. One real primary path (planned)` - 26 edges
10. `Health monitoring PRD` - 25 edges

## Surprising Connections (you probably didn't know these)
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Where a Change Belongs (documentation ownership table)` --semantically_similar_to--> `Documentation Ownership Roles (scope, specs, PRDs, ADRs)`  [INFERRED] [semantically similar]
  docs/index.md → AGENTS.md
- `Engineering Workflow` --semantically_similar_to--> `Agent Workflow`  [INFERRED] [semantically similar]
  docs/engineering-standards.md → AGENTS.md

## Hyperedges (group relationships)
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off [INFERRED 0.85]
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **Deferred features outside the current release queue** — docs_scope_012_multiple_proxy_authority, docs_scope_013_checked_online_recovery, docs_scope_014_advanced_pooling_and_routing, docs_scope_015_optional_result_cache, docs_scope_016_additional_replication_modes, docs_scope_017_administrative_dashboard [EXTRACTED 1.00]
- **Foundation phase features (stack, standards, data model)** — docs_scope_001_stack_and_workspace, docs_scope_002_standards_and_tooling, docs_scope_003_control_and_session_data_model [EXTRACTED 1.00]
- **V1 tracer bullet vertical slices 1 to 7 plus V1 gate** — docs_scope_004_one_real_primary_path, docs_scope_005_clean_session_reuse, docs_scope_006_protocol_breadth_and_cancellation, docs_scope_007_restricted_replica_sessions, docs_scope_008_health_diagnostics_and_drain, docs_scope_009_safe_topology_administration, docs_scope_010_fenced_manual_promotion, docs_scope_011_release_qualification_harness [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (25 total, 0 thin omitted)

### Community 0 - "Scope Index and Feature Inventory"
Cohesion: 0.05
Nodes (99): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Invariant: preserve transaction and session ownership, Tracer Bullet Build Approach, Claude Code guidance (CLAUDE.md), Planned Cargo Check Commands, Working with the Docs (+91 more)

### Community 1 - "Security and Engineering Standards"
Cohesion: 0.05
Nodes (63): POOL agent guide (AGENTS.md), Apache 2.0 project license, Administration requires authentication and authorization, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Agent Workflow (+55 more)

### Community 2 - "Decision Ledger and Routing PRDs"
Cohesion: 0.06
Nodes (51): Policy Version, Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path, Manual failover first; automatic failover and cache disabled, First deployment: one proxy VM, primary and two replicas on Proxmox (+43 more)

### Community 3 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.09
Nodes (51): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first) (+43 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.09
Nodes (46): Virtual Cancellation Model, Data Plane vs Control Plane Separation, Planned Architecture, ADR 0001. Initial architecture, Frontend owner task, No automatic retry after dispatch, Future multi proxy deployment, Architecture overview (+38 more)

### Community 5 - "Testing Strategy and Release Gates"
Cohesion: 0.10
Nodes (34): Git and Release Standards, Tests and Evidence Standard, Release Practices, V1 Release Gate, R-16 Dependency drift or abandoned crate, Health monitoring failure scenarios (SQL failure, revoked permission, clock jump, replay pause, primary isolation), criterion 0.8.2 (microbenchmarks), proptest 1.12.0 (state and framing properties) (+26 more)

### Community 6 - "Core Safety Invariants"
Cohesion: 0.10
Nodes (29): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Invariant: keep writes on the verified primary, F-030 Validation and reset (close uncertain sessions) (+21 more)

### Community 7 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.08
Nodes (28): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Backend Login, Support Profile, PostgreSQL protocol analysis (+20 more)

### Community 8 - "Health Monitoring and Admin API"
Cohesion: 0.12
Nodes (28): Health features F-054 to F-060, Axum REST administration API plus CLI, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control, GET /v1/nodes node status route (+20 more)

### Community 9 - "Threat Model and Fencing"
Cohesion: 0.13
Nodes (24): Invariant: promotion requires a reviewed operation and independently verified fencing, Safe first party Rust and bounded resources, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), Failover features F-067 to F-075, R-08 Split brain (+16 more)

### Community 10 - "Manual Failover and Recovery"
Cohesion: 0.13
Nodes (24): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+16 more)

### Community 11 - "Glossary and Routing Profiles"
Cohesion: 0.16
Nodes (23): Separate Frontend and Backend Identity, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Credential Generation, Eligible Primary, Eligible Replica (+15 more)

### Community 12 - "Connection Pooling and Safety Contracts"
Cohesion: 0.15
Nodes (19): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Transaction pooling (deferred), Durable approved writer authority (+11 more)

### Community 13 - "Roadmap and High Availability"
Cohesion: 0.16
Nodes (18): Tracer Bullet build approach, Routing Preview, Roadmap, Delivery slices, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy, Phase 3 SQL routing (+10 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.18
Nodes (14): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), SCRAM channel binding per terminated TLS channel, Rust technology evaluation, Dependency adoption gate, axum 0.8.9 (administration endpoints), Hybrid parser policy, pg_query 18.1.0 and libpg_query (PostgreSQL raw parser) (+6 more)

### Community 15 - "Pgpool Baseline and Configuration Coverage"
Cohesion: 0.23
Nodes (12): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Pgpool baseline, Baseline gaps (planning inventory, not parity certification), License and source reuse policy, PCP commands, SQL commands and pgpool_adm reference surface, Tagged sample and source manifest (V4_7_3) (+4 more)

### Community 16 - "SQL Routing and Replica Profiles"
Cohesion: 0.29
Nodes (11): Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table, Operation admission boundary, SQL routing, Routing decision matrix, Eventual read consistency for replica sessions, No primary fallback for replica profiles (+3 more)

### Community 17 - "Operation Journal and Control Records"
Cohesion: 0.31
Nodes (11): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer), Expected Generation, Operation (journaled administrative action) (+3 more)

### Community 18 - "Feasibility Study Findings"
Cohesion: 0.20
Nodes (10): Feasibility study, Automatic promotion (not ready for initial release), Go or no go milestones, Commercial and maintenance assumptions, Manual promotion (feasible with external dependencies), Operational ownership boundaries, Query result caching (high uncertainty), Session pooling feasibility (high feasibility, high correctness risk) (+2 more)

### Community 19 - "Pgpool Option Families"
Cohesion: 0.20
Nodes (10): Pgpool Watchdog coordination, Connections and identity option family (48 options), Logs, files and reset option family (28 options), Relation metadata option family (6 options), Canonical runtime declaration census (RT-001 to RT-186), Source-only delta names (hidden aliases, health_check_test, replication_mode), Streaming and health option family (24 options), Streaming sample census (CFG-001 to CFG-207, 207 spellings) (+2 more)

### Community 20 - "Session Leases and Admission"
Cohesion: 0.28
Nodes (9): Session Model (pinned backend lease), Pool limits and operations, Data plane, Provisional resource defaults and backpressure, F-024 Transaction Mode (refused, session pooling only), Admission, Frontend, Lease (+1 more)

### Community 21 - "Competitor Analysis"
Cohesion: 0.47
Nodes (9): Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC), PgCat (Rust/Tokio routing proxy, MIT), Competitor reference roles recommendation, Pgpool-II 4.7.3 (selected baseline) (+1 more)

### Community 22 - "Backend Health and Drain Operations"
Cohesion: 0.29
Nodes (8): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Observation record, Detach, Drain

### Community 23 - "Approved Writer and Topology Epoch"
Cohesion: 0.83
Nodes (4): Approved Writer, Quarantine, Topology Epoch, R-19 Restart adopts wrong writable node

### Community 24 - "Pgpool Baseline Modes"
Cohesion: 0.50
Nodes (4): Pgpool baseline modes and POOL dispositions, native_replication and snapshot_isolation modes (deferred), streaming_replication mode (initial operating model), Replication and routing option family (17 options)

## Knowledge Gaps
- **52 isolated node(s):** `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `pgbench and purpose built workload generator` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Scope Index and Feature Inventory` to `Security and Engineering Standards`, `Decision Ledger and Routing PRDs`, `Architecture Spec and Repository Layout`, `Testing Strategy and Release Gates`, `Core Safety Invariants`, `Protocol Contract and Driver Compatibility`, `Health Monitoring and Admin API`, `Manual Failover and Recovery`, `Glossary and Routing Profiles`, `Connection Pooling and Safety Contracts`, `Roadmap and High Availability`, `Rust Technology Evaluation`, `Pgpool Baseline and Configuration Coverage`, `SQL Routing and Replica Profiles`, `Operation Journal and Control Records`, `Feasibility Study Findings`, `Session Leases and Admission`, `Competitor Analysis`, `Backend Health and Drain Operations`?**
  _High betweenness centrality (0.412) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Architecture Spec and Repository Layout` to `Scope Index and Feature Inventory`, `Security and Engineering Standards`, `Decision Ledger and Routing PRDs`, `Scope 001 Coordination Brief and Log`, `Core Safety Invariants`, `Health Monitoring and Admin API`, `Manual Failover and Recovery`, `Glossary and Routing Profiles`, `Connection Pooling and Safety Contracts`, `Roadmap and High Availability`, `Rust Technology Evaluation`, `Session Leases and Admission`, `Approved Writer and Topology Epoch`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Scope Index and Feature Inventory` to `Security and Engineering Standards`, `Decision Ledger and Routing PRDs`, `Testing Strategy and Release Gates`, `Core Safety Invariants`, `Health Monitoring and Admin API`, `Threat Model and Fencing`, `Manual Failover and Recovery`, `Glossary and Routing Profiles`, `Roadmap and High Availability`, `Pgpool Baseline and Configuration Coverage`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope Index and Feature Inventory` be split into smaller, more focused modules?**
  _Cohesion score 0.050505050505050504 - nodes in this community are weakly interconnected._