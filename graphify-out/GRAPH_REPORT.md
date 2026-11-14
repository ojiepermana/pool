# Graph Report - pool  (2026-11-14)

## Corpus Check
- 12 files · ~64,821 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 907 nodes · 2492 edges · 46 communities
- Extraction: 79% EXTRACTED · 21% INFERRED · 0% AMBIGUOUS · INFERRED: 515 edges (avg confidence: 0.87)
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
- Spec 0001, Stack and Project Guides
- Core Proxy PRD and Decision Ledger
- Testing Strategy and Release Gates
- Virtual Cancellation and Architecture Overview
- pool-config Validation Library
- HA and Recovery Features and Threats
- Approved Writer and Quarantine Terms
- check-config Integration Tests
- SQL Routing PRD
- Load Balancing PRD
- Safety Invariants and Session Risks
- pool CLI Entry and Argument Parsing
- Query Cache PRD
- Connection Pooling PRD
- check-config Report Output
- Health Monitoring and Probes
- Feature Priorities, Cache and PCP Equivalents
- Failover Promotion Stages and Safety Contracts
- Threat Model and Authentication Risks
- TLS, Admin Auth and Security Features
- Engineering Standards and Benchmarks
- Security Requirements and Journal Integrity
- Trust Boundaries and Planes
- Routing Safety and SQL Classification
- Resource Limits and Memory Safety
- KPIs and Proposed Gates

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 76 edges
2. `Glossary` - 57 edges
3. `Spec 0001. POOL initial architecture` - 52 edges
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
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md
- `Safety Over Throughput Priority` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Spec 0001 acceptance criteria tied to test files** — docs_specs_0001_initial_architecture_verify_ac_coverage, apps_pool_tests_check_config, crates_pool_config_tests_validate, apps_pool_src_cli, crates_pool_config_src_lib [EXTRACTED 1.00]
- **Spec 0001 Linux build blocked on Docker (AC-2, F1-1, F1-2, /dev/full)** — docs_specs_0001_initial_architecture_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_linux_build, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_follow_up_start_docker, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, docs_specs_0001_initial_architecture_verify_ac_11_partial [EXTRACTED 1.00]
- **Spec 0001 workstation evidence (commands, tests, fixtures, locked set)** — docs_specs_0001_initial_architecture_verify_workstation_evidence, docs_specs_0001_initial_architecture_verify_workstation_commands, docs_specs_0001_initial_architecture_verify_test_results, docs_specs_0001_initial_architecture_verify_fixture_transcripts, docs_specs_0001_initial_architecture_verify_locked_dependencies, docs_specs_0001_initial_architecture_verify_toolchain_selection, docs_specs_0001_initial_architecture_verify_fixtures, docs_specs_0001_initial_architecture_verify_ac_pass_workstation [EXTRACTED 1.00]
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

## Communities (46 total, 0 thin omitted)

### Community 0 - "Scope Index and Documentation Map"
Cohesion: 0.22
Nodes (16): Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Git and Release Standards, Log event fields and redaction convention (ts, level, event; redaction by type), Shared Conventions Table, Release Practices, Cross Check Before Development, R-16 Dependency drift or abandoned crate (+8 more)

### Community 1 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.07
Nodes (75): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach, Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon) (+67 more)

### Community 2 - "Feature Inventory and Safety Invariants"
Cohesion: 0.16
Nodes (16): Feature inventory and parity matrix, Inventory Coverage Limits, Initial V1 Exclusions, Clustering mode features F-094 to F-099, Observability features F-109 to F-114, Pgpool-II 4.7.3 Baseline, Priority Tiers P0, P1, P2, Runtime config hidden aliases F-161 to F-165 (+8 more)

### Community 3 - "Administration and Cluster PRDs"
Cohesion: 0.17
Nodes (17): Axum REST administration API plus CLI, pool check-config, GET /v1/nodes node status route, Cluster management PRD, Drain, detach and attach node operations, Two observed primaries close write admission, Generation checked journaled mutations, Promoter role distinct from observer/operator (+9 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.38
Nodes (14): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+6 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.07
Nodes (33): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Support Profile, Proposed stopping rules, PostgreSQL protocol analysis (+25 more)

### Community 6 - "Risk Register"
Cohesion: 0.19
Nodes (16): Documentation Ownership Roles (scope, specs, PRDs, ADRs), F-011 Prepared statements (session ownership, no cross backend replay), Stable ID Citation Rule, Where a Change Belongs (documentation ownership table), Dependencies and critical path, Milestones, Risk register, R-04 Prepared statement incompatibility (+8 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.06
Nodes (91): Automatic Promotion (reserved and off), Output Version (output_version field), Reserved Capability, Result Cache (reserved and off), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Checked in wave 1: F1 build notes recorded (arc-f1-update done, commit a6e0042, merged to bed9fad; AC-1..AC-13 and 5 step Build plan present, docs only), Delegated decision (berat, 0001 AC-4, AC-6): automatic promotion capability key named automatic_promotion (cache: result_cache) over automatic_failover (+83 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.11
Nodes (30): Dependency and Configuration Change Rules, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Backend Login, Configuration Generation, Configuration Version (config_version key) (+22 more)

### Community 9 - "Delivery Phases, Milestones and Risks"
Cohesion: 0.11
Nodes (26): F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Fence Receipt, Write Barrier, Tracer Bullet build approach, Walking Slice Milestone, R-09 Failover loses acknowledged commits, R-24 Walking slice exceeds its estimate, Roadmap (+18 more)

### Community 10 - "Roadmap and Routing PRDs"
Cohesion: 0.15
Nodes (13): Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, One daemon with separate modules plus separate CLI, pg_query parser with PostgreSQL 18 grammar, Prometheus metrics and tracing logs (+5 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.25
Nodes (14): Pooling Features (F-023 to F-032), Protocol Features (F-001 to F-022), Critical Path Handoffs, Core MVP, R-01 Protocol incompatibility, Core proxy PRD, Bounded admission, deadlines and graceful teardown, Declared PostgreSQL 18 protocol profile (3.0/3.2) (+6 more)

### Community 12 - "Connection Pooling and Leases"
Cohesion: 0.18
Nodes (13): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool limits and operations, Statement pooling (not planned for V1), Transaction pooling (deferred), Provisional resource defaults and backpressure, Transparent per query routing (later feature) (+5 more)

### Community 13 - "Fencing and Split Brain"
Cohesion: 0.25
Nodes (11): Invariant: promotion requires a reviewed operation and independently verified fencing, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-075 Automatic Failover (disabled in V1), Failover features F-067 to F-075, R-08 Split brain (+3 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.14
Nodes (24): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy (+16 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.20
Nodes (16): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table (+8 more)

### Community 16 - "Manual Failover and Recovery"
Cohesion: 0.43
Nodes (7): Manual failover PRD, Durable intent before fencing and reconciliation of unknown outcomes, Failover plan and execute API (/v1/failover/plans), Failover operation phases (Planned to Completed or Failed/NeedsReview), PromotionPlan, Candidate replica leases retired before promotion, RPO disclosure: known flush target or unknown loss window

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (45): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+37 more)

### Community 18 - "Scope Status and Documentation Ownership"
Cohesion: 0.19
Nodes (23): Claude Code guidance (CLAUDE.md), Planned Cargo Check Commands, Working with the Docs, Starting development (implementation order), First Handoff: the Workspace, Documentation index, Planning package validation, Development Readiness Review (+15 more)

### Community 19 - "Operation Journal and Control Records"
Cohesion: 0.27
Nodes (13): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Observation record, Operation record, Local SQLite operation journal (single writer), Expected Generation (+5 more)

### Community 20 - "Spec 0001, Stack and Project Guides"
Cohesion: 0.08
Nodes (42): Apache 2.0 project license, Contributing to POOL, ADR 0001. Initial architecture, Session pooling (one frontend owns one backend lease), Deployment reference, Initial four VM lab, Future multi proxy deployment, Independent fencing authority (+34 more)

### Community 21 - "Core Proxy PRD and Decision Ledger"
Cohesion: 0.21
Nodes (12): Retry only before initial lease and ReadyForQuery exposure, Authoritative barrier across every write path, Independent review resolutions (seven approved fixes), Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Routing decision: primary default, explicit read only session profiles, Cancellation forwarded only through the live lease owner, CancellationEntry (+4 more)

### Community 22 - "Testing Strategy and Release Gates"
Cohesion: 0.18
Nodes (19): Slice, Verification Record, Walking Slice (glossary term), Handoff for Later Slices, V1 Release Gate, R-25 Reviewer or lab unavailable at a gate, Go or no go milestones, proptest 1.12.0 (state and framing properties) (+11 more)

### Community 23 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.15
Nodes (21): Virtual Cancellation Model, Session Model (pinned backend lease), Data plane, Frontend owner task, No automatic retry after dispatch, Architecture overview, Application path sequence, Failure behavior (no transparent failover, no implicit replay) (+13 more)

### Community 24 - "pool-config Validation Library"
Cohesion: 0.07
Nodes (39): capability_state(), CapabilityState, error_location(), Finding, FindingCode, Location, position(), RawCapability (+31 more)

### Community 25 - "HA and Recovery Features and Threats"
Cohesion: 0.29
Nodes (8): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-21 Proxmox host correlated failure, 12. Multiple proxy authority (deferred), SEC-9 Allowlisted structured fencing/recovery adapters, T-09 Command injection through fencing/recovery hooks, T-14 Correlated host failure (single Proxmox host)

### Community 26 - "Approved Writer and Quarantine Terms"
Cohesion: 0.43
Nodes (7): Cluster Features (F-061 to F-066), F-065 Primary Identification, F-066 Topology reconciliation (durable writer/epoch/quarantine), Approved Writer, Quarantine, Topology Epoch, R-19 Restart adopts wrong writable node

### Community 27 - "check-config Integration Tests"
Cohesion: 0.18
Nodes (32): a_failed_write_exits_3_without_stderr(), a_value_after_config_is_always_the_path(), arguments_that_are_not_utf8(), assert_key_rejection(), assert_none_rejection(), assert_positioned_rejection(), assert_run(), check() (+24 more)

### Community 28 - "SQL Routing PRD"
Cohesion: 0.36
Nodes (9): Policy Version, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+1 more)

### Community 29 - "Load Balancing PRD"
Cohesion: 0.36
Nodes (9): No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state), Replica eligibility filter (role, identity, freshness, maintenance, lag), Primary fallback disabled in V1, ReplicaPolicy, SelectorState (+1 more)

### Community 30 - "Safety Invariants and Session Risks"
Cohesion: 0.14
Nodes (18): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: preserve transaction and session ownership, Data and Protocol Safety Standards (+10 more)

### Community 31 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.16
Nodes (18): Format, Invocation, parse(), parse_check_config(), run(), check_config(), main(), read_bounded() (+10 more)

### Community 32 - "Query Cache PRD"
Cohesion: 0.38
Nodes (7): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Query cache PRD, Caching disabled until a separate contract is approved and proven, Candidate CacheKey, Cache invalidation and dependency contract

### Community 33 - "Connection Pooling PRD"
Cohesion: 0.43
Nodes (7): Connection pooling PRD, Credential and configuration generation retirement, PoolKey, PoolState, release(lease, terminal_state), Reset and validate before reuse; close uncertain state, Session pooling only (no transaction or statement multiplexing in V1)

### Community 34 - "check-config Report Output"
Cohesion: 0.17
Nodes (13): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+5 more)

### Community 35 - "Health Monitoring and Probes"
Cohesion: 0.18
Nodes (17): Invariant: health failure does not authorize promotion, F-060 Network partitions (health changes eligibility only), Health features F-054 to F-060, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control (+9 more)

### Community 36 - "Feature Priorities, Cache and PCP Equivalents"
Cohesion: 0.20
Nodes (12): Invariant: keep writes on the verified primary, Cache features F-088 to F-093, F-056 Primary Standby Role Observation, F-065 Primary identification (durably approved writer with fresh evidence), F-088 Optional Memory Cache (disabled in V1), F-088 Optional memory cache (result caching disabled in V1), F-134 pcp_invalidate_query_cache, F-134 pcp_invalidate_query_cache (deferred until cache qualification) (+4 more)

### Community 37 - "Failover Promotion Stages and Safety Contracts"
Cohesion: 0.29
Nodes (12): Pool key (pool identity), Durable approved writer authority, Initial admission boundary, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter), Recovery with pg_rewind and pg_basebackup (+4 more)

### Community 38 - "Threat Model and Authentication Risks"
Cohesion: 0.24
Nodes (11): Safe first party Rust and bounded resources, Separate Frontend and Backend Identity, R-11 Authentication or role confusion, SEC-2 Independently authenticated frontend and backend identities, SEC-3 SCRAM PLUS only with valid channel binding; bounded work factors, Threat model, Protected assets and threat actors, Residual risks (+3 more)

### Community 39 - "TLS, Admin Auth and Security Features"
Cohesion: 0.20
Nodes (10): Administration requires authentication and authorization, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-108 Secure reload (validate entire generation before publication), Security features F-100 to F-108, SEC-1 Require TLS and verify backend CA plus server name (+2 more)

### Community 40 - "Engineering Standards and Benchmarks"
Cohesion: 0.28
Nodes (9): Agent Workflow, Engineering standards, Tests and Evidence Standard, Engineering Workflow, criterion 0.8.2 (microbenchmarks), Benchmark plan, pgbench and purpose built workload generator, Benchmark workload matrix (+1 more)

### Community 41 - "Security Requirements and Journal Integrity"
Cohesion: 0.36
Nodes (9): Security and Review Requirements, Reviewer roster (who holds each review role, where a fresh model may review), Security requirements, Operational security defaults, SEC-14 Independent review of security sensitive, protocol, parser and HA changes, SEC-7 Persist administrative intent before an external effect, Security review boundaries, T-11 Journal tampering, loss or disk full (+1 more)

### Community 42 - "Trust Boundaries and Planes"
Cohesion: 0.46
Nodes (8): Control module, Data plane, Database trust domain, Operation journal, Operator management domain, Telemetry domain, Trust boundaries, Untrusted client network

### Community 43 - "Routing Safety and SQL Classification"
Cohesion: 0.53
Nodes (6): Invariant: fail closed on unknown role or unsafe routing, F-033 SQL AST classification (PostgreSQL 18 grammar adapter), F-045 Unknown and failed parsing (keep primary lease, reject on restricted replica), Routing Features (F-033 to F-047), R-02 Incorrect SQL classification, T-06 Unsafe replica routing

### Community 44 - "Resource Limits and Memory Safety"
Cohesion: 0.40
Nodes (6): Rust and Runtime Standards, F-107 Rate and resource limits (admission, frame and auth CPU budgets), R-13 Memory exhaustion, SEC-8 Global and per principal connection, byte, queue, parser and auth CPU budgets, T-01 Frame confusion or memory exhaustion, Protocol unit tests (golden frames, split at every byte boundary)

### Community 45 - "KPIs and Proposed Gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

## Knowledge Gaps
- **51 isolated node(s):** `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors`, `Knowledge graph refresh before commit (graphify-out kept current)` (+46 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Scope Status and Documentation Ownership` to `Scope Index and Documentation Map`, `Scope 001 Coordination Brief and Log`, `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Risk Register`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Feature Families and Planned Scopes`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Pgpool Baseline, Competitors and Feasibility`, `Operation Journal and Control Records`, `Spec 0001, Stack and Project Guides`, `Testing Strategy and Release Gates`, `Virtual Cancellation and Architecture Overview`, `HA and Recovery Features and Threats`, `SQL Routing PRD`, `Load Balancing PRD`, `Query Cache PRD`, `Connection Pooling PRD`, `Health Monitoring and Probes`, `Failover Promotion Stages and Safety Contracts`, `Threat Model and Authentication Risks`, `Engineering Standards and Benchmarks`, `Security Requirements and Journal Integrity`?**
  _High betweenness centrality (0.360) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Spec 0001, Stack and Project Guides` to `Scope Index and Documentation Map`, `Scope 001 Coordination Brief and Log`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Risk Register`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Feature Families and Planned Scopes`, `Rust Technology Evaluation`, `Manual Failover and Recovery`, `Scope Status and Documentation Ownership`, `Operation Journal and Control Records`, `Virtual Cancellation and Architecture Overview`, `Approved Writer and Quarantine Terms`, `SQL Routing PRD`, `Load Balancing PRD`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Connection Pooling PRD`, `Health Monitoring and Probes`, `Failover Promotion Stages and Safety Contracts`, `Engineering Standards and Benchmarks`?**
  _High betweenness centrality (0.169) - this node is a cross-community bridge._
- **Why does `0001. Verification record` connect `Reserved Capabilities and First Build Criteria` to `Scope 001 Coordination Brief and Log`, `check-config Report Output`, `Scope Status and Documentation Ownership`, `Spec 0001, Stack and Project Guides`, `check-config Integration Tests`, `pool CLI Entry and Argument Parsing`?**
  _High betweenness centrality (0.146) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165` to the rest of the system?**
  _51 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope 001 Coordination Brief and Log` be split into smaller, more focused modules?**
  _Cohesion score 0.06594594594594595 - nodes in this community are weakly interconnected._