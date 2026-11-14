# Graph Report - pool  (2026-11-14)

## Corpus Check
- 3 files · ~58,768 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 759 nodes · 2085 edges · 34 communities
- Extraction: 76% EXTRACTED · 24% INFERRED · 0% AMBIGUOUS · INFERRED: 504 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Scope Index and Documentation Map
- Scope 001 Coordination Brief and Log
- Feature Inventory and Safety Invariants
- Administration and Cluster PRDs
- Architecture Spec and Repository Layout
- Protocol Contract and Driver Compatibility
- Risk Register
- Reserved Capabilities and First Build Criteria
- Glossary and Routing Profiles
- Delivery Phases, Milestones and Risks
- Roadmap and Routing PRDs
- Feature Families and Planned Scopes
- Connection Pooling and Leases
- Fencing and Split Brain
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Manual Failover and Recovery
- Pgpool Baseline, Competitors and Feasibility
- Scope Status and Documentation Ownership
- Operation Journal and Control Records
- Agent Guide, Standards and Security Requirements
- Core Proxy PRD and Decision Ledger
- Agent Workflow and Walking Slice
- Virtual Cancellation and Architecture Overview
- Session Model and Data Plane Terms
- HA and Recovery Features and Threats
- Approved Writer and Quarantine Terms
- Planning Validation
- SQL Routing PRD
- Load Balancing PRD
- Session Isolation and Identity Risks
- High Availability PRD
- Query Cache PRD
- Connection Pooling PRD

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 75 edges
2. `Glossary` - 57 edges
3. `Spec 0001. POOL initial architecture` - 50 edges
4. `Feature inventory and parity matrix` - 47 edges
5. `Testing strategy` - 43 edges
6. `Scope: POOL, order and status index` - 42 edges
7. `Security requirements` - 39 edges
8. `Risk register` - 34 edges
9. `First Handoff: the Workspace` - 30 edges
10. `4. One real primary path (planned)` - 28 edges

## Surprising Connections (you probably didn't know these)
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing)` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md

## Hyperedges (group relationships)
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build [INFERRED 0.85]
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **Scope 001 shared conventions (configuration keys, exit status and output, log events)** — docs_engineering_standards_configuration_keys_convention, docs_engineering_standards_exit_status_output_convention, docs_engineering_standards_log_event_convention, docs_specs_0001_initial_architecture_index_configuration_rules, docs_specs_0001_initial_architecture_index_exit_status_and_output, docs_specs_0001_initial_architecture_index_log_events_and_redaction [EXTRACTED 1.00]
- **Deferred features outside the current release queue** — docs_scope_012_multiple_proxy_authority, docs_scope_013_checked_online_recovery, docs_scope_014_advanced_pooling_and_routing, docs_scope_015_optional_result_cache, docs_scope_016_additional_replication_modes, docs_scope_017_administrative_dashboard [EXTRACTED 1.00]
- **Foundation phase features (stack, standards, data model)** — docs_scope_001_stack_and_workspace, docs_scope_002_standards_and_tooling, docs_scope_003_control_and_session_data_model [EXTRACTED 1.00]
- **V1 tracer bullet vertical slices 1 to 7 plus V1 gate** — docs_scope_004_one_real_primary_path, docs_scope_005_clean_session_reuse, docs_scope_006_protocol_breadth_and_cancellation, docs_scope_007_restricted_replica_sessions, docs_scope_008_health_diagnostics_and_drain, docs_scope_009_safe_topology_administration, docs_scope_010_fenced_manual_promotion, docs_scope_011_release_qualification_harness [EXTRACTED 1.00]
- **Spec 0001 first build acceptance criteria AC-1 to AC-13** — docs_specs_0001_initial_architecture_index_ac_1, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_3, docs_specs_0001_initial_architecture_index_ac_4, docs_specs_0001_initial_architecture_index_ac_5, docs_specs_0001_initial_architecture_index_ac_6, docs_specs_0001_initial_architecture_index_ac_7, docs_specs_0001_initial_architecture_index_ac_8, docs_specs_0001_initial_architecture_index_ac_9, docs_specs_0001_initial_architecture_index_ac_10, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_index_ac_12, docs_specs_0001_initial_architecture_index_ac_13 [EXTRACTED 1.00]
- **Spec 0001 build plan steps 1 to 5 (tracer bullet)** — docs_specs_0001_initial_architecture_index_build_plan_step_1, docs_specs_0001_initial_architecture_index_build_plan_step_2, docs_specs_0001_initial_architecture_index_build_plan_step_3, docs_specs_0001_initial_architecture_index_build_plan_step_4, docs_specs_0001_initial_architecture_index_build_plan_step_5 [EXTRACTED 1.00]
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

## Communities (34 total, 0 thin omitted)

### Community 0 - "Scope Index and Documentation Map"
Cohesion: 0.19
Nodes (21): Git and Release Standards, Release Practices, V1 Release Gate, R-16 Dependency drift or abandoned crate, 2. Standards and tooling (planned), Documentation checks (links/anchors, scope status agreement, stable requirement IDs), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply), 11. Release qualification harness (planned) (+13 more)

### Community 1 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.06
Nodes (82): POOL agent guide (AGENTS.md), Documentation ownership model (scope, specs, PRDs, ADRs, research), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template) (+74 more)

### Community 2 - "Feature Inventory and Safety Invariants"
Cohesion: 0.11
Nodes (26): Feature inventory and parity matrix, Cache features F-088 to F-093, Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-060 Network partitions (health changes eligibility only), F-088 Optional Memory Cache (disabled in V1), F-088 Optional memory cache (result caching disabled in V1), F-134 pcp_invalidate_query_cache (+18 more)

### Community 3 - "Administration and Cluster PRDs"
Cohesion: 0.09
Nodes (34): Authenticated SQL health probes, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Observation record, F-101 TLS and mTLS verification, Axum REST administration API plus CLI, pool check-config, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible (+26 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.36
Nodes (15): Data Plane vs Control Plane Separation, Planned Architecture, Topology Snapshot, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence (+7 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.08
Nodes (31): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Transaction state tracking (ReadyForQuery status), Support Profile, Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy (+23 more)

### Community 6 - "Risk Register"
Cohesion: 0.20
Nodes (16): Documentation Ownership Roles (scope, specs, PRDs, ADRs), Invariant: preserve transaction and session ownership, F-011 Prepared statements (session ownership, no cross backend replay), Where a Change Belongs (documentation ownership table), Dependencies and critical path, Milestones, Risk register, R-04 Prepared statement incompatibility (+8 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.06
Nodes (80): Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Dependency and Configuration Change Rules, Log event fields and redaction convention (ts, level, event; redaction by type), Automatic Promotion (reserved and off), Configuration Generation, Configuration Version (config_version key), Output Version (output_version field), Reserved Capability (+72 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.18
Nodes (21): Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Backend Login, Credential Generation, Eligible Primary, Eligible Replica (+13 more)

### Community 9 - "Delivery Phases, Milestones and Risks"
Cohesion: 0.26
Nodes (12): Tracer Bullet build approach, Roadmap, Delivery slices, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy, Phase 3 SQL routing, Phase 4 Production reliability (+4 more)

### Community 10 - "Roadmap and Routing PRDs"
Cohesion: 0.16
Nodes (14): Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, Independent review resolutions (seven approved fixes), One daemon with separate modules plus separate CLI, pg_query parser with PostgreSQL 18 grammar (+6 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.26
Nodes (15): Pooling Features (F-023 to F-032), Protocol Features (F-001 to F-022), First Handoff: the Workspace, Documentation index, Stable ID Citation Rule, Critical Path Handoffs, Core MVP, Development Readiness Review (+7 more)

### Community 12 - "Connection Pooling and Leases"
Cohesion: 0.28
Nodes (9): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool limits and operations, Statement pooling (not planned for V1), Provisional resource defaults and backpressure, Backend, Pool Key (+1 more)

### Community 13 - "Fencing and Split Brain"
Cohesion: 0.07
Nodes (43): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: promotion requires a reviewed operation and independently verified fencing, Safe first party Rust and bounded resources, Invariant: keep writes on the verified primary (+35 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.08
Nodes (37): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Operation admission boundary, Protocol finite state machine, Routing decision matrix, SQL classification through PostgreSQL 18 raw grammar, Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14) (+29 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.15
Nodes (21): Cluster management, Configured backend registration, Drain, attach and quarantine operations, Replication freshness and byte lag, Weighted round robin replica selection, Pool key (pool identity), Transaction pooling (deferred), Durable approved writer authority (+13 more)

### Community 16 - "Manual Failover and Recovery"
Cohesion: 0.13
Nodes (24): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+16 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (46): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+38 more)

### Community 18 - "Scope Status and Documentation Ownership"
Cohesion: 0.21
Nodes (14): Claude Code guidance (CLAUDE.md), Working with the Docs, Slice, Verification Record, Starting development (implementation order), Handoff for Later Slices, R-25 Reviewer or lab unavailable at a gate, Documentation Ownership (+6 more)

### Community 19 - "Operation Journal and Control Records"
Cohesion: 0.31
Nodes (11): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer), Expected Generation, Operation (journaled administrative action) (+3 more)

### Community 20 - "Agent Guide, Standards and Security Requirements"
Cohesion: 0.05
Nodes (63): Apache 2.0 project license, Administration requires authentication and authorization, Invariant: never replay uncertain dispatched work, Agent Workflow, Contributing to POOL, ADR 0001. Initial architecture, Session pooling (one frontend owns one backend lease), Engineering standards (+55 more)

### Community 21 - "Core Proxy PRD and Decision Ledger"
Cohesion: 0.21
Nodes (13): Retry only before initial lease and ReadyForQuery exposure, Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Terminated SCRAM over TLS with optional mTLS and separate backend credentials, Core proxy PRD, Bounded admission, deadlines and graceful teardown, Cancellation forwarded only through the live lease owner, CancellationEntry (+5 more)

### Community 22 - "Agent Workflow and Walking Slice"
Cohesion: 0.25
Nodes (11): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach, Walking Slice (glossary term), Walking Slice Milestone, R-24 Walking slice exceeds its estimate, 4. One real primary path (planned) (+3 more)

### Community 23 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.21
Nodes (14): Virtual Cancellation Model, No automatic retry after dispatch, Future multi proxy deployment, Architecture overview, Failure behavior (no transparent failover, no implicit replay), Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), F-018 CancelRequest and BackendKeyData, F-018 CancelRequest and BackendKeyData (virtual per lease keys) (+6 more)

### Community 24 - "Session Model and Data Plane Terms"
Cohesion: 0.22
Nodes (11): Session Model (pinned backend lease), Data plane, Frontend owner task, Application path sequence, F-024 Transaction Mode (refused, session pooling only), Admission, Detach, Drain (+3 more)

### Community 25 - "HA and Recovery Features and Threats"
Cohesion: 0.24
Nodes (11): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-21 Proxmox host correlated failure, 10. Fenced manual promotion (planned), 12. Multiple proxy authority (deferred), 13. Checked online recovery (deferred), SEC-5 Least privilege database logins; separate monitor and promotion roles (+3 more)

### Community 26 - "Approved Writer and Quarantine Terms"
Cohesion: 0.43
Nodes (7): Cluster Features (F-061 to F-066), F-065 Primary Identification, F-066 Topology reconciliation (durable writer/epoch/quarantine), Approved Writer, Quarantine, Topology Epoch, R-19 Restart adopts wrong writable node

### Community 27 - "Planning Validation"
Cohesion: 0.31
Nodes (9): Planned Cargo Check Commands, Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Shared Conventions Table, Planning package validation, Cross Check Before Development, Initial Package Checks, Validation Limits (+1 more)

### Community 28 - "SQL Routing PRD"
Cohesion: 0.36
Nodes (9): Policy Version, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+1 more)

### Community 29 - "Load Balancing PRD"
Cohesion: 0.36
Nodes (9): No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state), Replica eligibility filter (role, identity, freshness, maintenance, lag), Primary fallback disabled in V1, ReplicaPolicy, SelectorState (+1 more)

### Community 30 - "Session Isolation and Identity Risks"
Cohesion: 0.29
Nodes (8): Invariant: close uncertain pooled state, Separate Frontend and Backend Identity, F-030 Validation and reset (close uncertain sessions), Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing), R-03 Session state leakage, R-11 Authentication or role confusion, T-04 Role confusion and cross tenant reuse, Pool isolation tests

### Community 31 - "High Availability PRD"
Cohesion: 0.36
Nodes (8): High availability PRD, ControllerEpoch, Endpoint failover via external load balancing or controlled VIP, Established consensus for one operation authority, External coordination adapter, Query routing on local immutable snapshots, ProxyMember, TopologyRecord

### Community 32 - "Query Cache PRD"
Cohesion: 0.38
Nodes (7): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Query cache PRD, Caching disabled until a separate contract is approved and proven, Candidate CacheKey, Cache invalidation and dependency contract

### Community 33 - "Connection Pooling PRD"
Cohesion: 0.43
Nodes (7): Connection pooling PRD, Credential and configuration generation retirement, PoolKey, PoolState, release(lease, terminal_state), Reset and validate before reuse; close uncertain state, Session pooling only (no transaction or statement multiplexing in V1)

## Knowledge Gaps
- **51 isolated node(s):** `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Protected assets and threat actors` (+46 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Feature Families and Planned Scopes` to `Scope Index and Documentation Map`, `Scope 001 Coordination Brief and Log`, `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Risk Register`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Pgpool Baseline, Competitors and Feasibility`, `Scope Status and Documentation Ownership`, `Operation Journal and Control Records`, `Agent Guide, Standards and Security Requirements`, `Core Proxy PRD and Decision Ledger`, `Agent Workflow and Walking Slice`, `Virtual Cancellation and Architecture Overview`, `Session Model and Data Plane Terms`, `HA and Recovery Features and Threats`, `Planning Validation`, `SQL Routing PRD`, `Load Balancing PRD`, `High Availability PRD`, `Query Cache PRD`, `Connection Pooling PRD`?**
  _High betweenness centrality (0.363) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Agent Guide, Standards and Security Requirements` to `Scope 001 Coordination Brief and Log`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Risk Register`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Roadmap and Routing PRDs`, `Feature Families and Planned Scopes`, `Fencing and Split Brain`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Scope Status and Documentation Ownership`, `Core Proxy PRD and Decision Ledger`, `Virtual Cancellation and Architecture Overview`, `Session Model and Data Plane Terms`, `Approved Writer and Quarantine Terms`, `Planning Validation`, `SQL Routing PRD`, `Load Balancing PRD`, `High Availability PRD`, `Query Cache PRD`, `Connection Pooling PRD`?**
  _High betweenness centrality (0.171) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Feature Inventory and Safety Invariants` to `Scope Index and Documentation Map`, `Administration and Cluster PRDs`, `Risk Register`, `Glossary and Routing Profiles`, `Feature Families and Planned Scopes`, `Fencing and Split Brain`, `Manual Failover and Recovery`, `Pgpool Baseline, Competitors and Feasibility`, `Scope Status and Documentation Ownership`, `Agent Guide, Standards and Security Requirements`, `Core Proxy PRD and Decision Ledger`, `HA and Recovery Features and Threats`, `Approved Writer and Quarantine Terms`, `Planning Validation`, `SQL Routing PRD`, `Load Balancing PRD`, `High Availability PRD`, `Query Cache PRD`, `Connection Pooling PRD`?**
  _High betweenness centrality (0.079) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)` to the rest of the system?**
  _51 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope 001 Coordination Brief and Log` be split into smaller, more focused modules?**
  _Cohesion score 0.06142728093947606 - nodes in this community are weakly interconnected._