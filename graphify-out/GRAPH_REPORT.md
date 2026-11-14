# Graph Report - pool  (2026-11-14)

## Corpus Check
- 4 files · ~71,599 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1057 nodes · 2975 edges · 50 communities
- Extraction: 82% EXTRACTED · 18% INFERRED · 0% AMBIGUOUS · INFERRED: 547 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Standards, Tooling and Release Qualification
- Scope 001 Coordination Brief and Log
- Feature Inventory and Parity Matrix
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
- Split Brain and Journal Risks
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Manual Failover and Recovery
- Pgpool Baseline, Competitors and Feasibility
- Scope Status and Documentation Ownership
- Proposed Stack and Deferred Features
- Spec 0001, Stack and Project Guides
- Core Proxy PRD and Decision Ledger
- pool-config Validation Tests
- Virtual Cancellation and Architecture Overview
- pool-config Validation Library
- HA and Recovery Features and Threats
- Agent Guide and Scope 001 Authorization
- check-config Integration Tests
- SQL Routing PRD
- Load Balancing PRD
- Safety Invariants and Session Risks
- pool CLI Entry and Argument Parsing
- Query Cache PRD
- Spec 0001 Acceptance Criteria
- Linux-only Write Tests and Verify Blocker
- Health Monitoring and Probes
- Coordination Sessions and Merge Decisions
- Failover Records and Fencing Authority
- ADR 0001, Identity and Architecture Rationale
- Security Features and Review Roles
- Agent Workflow and Documentation Index
- Linux Target and Build Plan Evidence
- Workspace Build Notes and Offline Bounds
- Full Verify Record and Checker Session
- Reserved Capabilities and Config Rules
- KPIs and Proposed Gates
- Manual Steps and Docker Blocker
- Cluster Management and Approved Writer
- High Availability PRD and Topology
- Exit Status and JSON Output Contract

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 76 edges
2. `Glossary` - 57 edges
3. `Spec 0001. POOL initial architecture` - 52 edges
4. `Feature inventory and parity matrix` - 47 edges
5. `Scope: POOL, order and status index` - 43 edges
6. `Testing strategy` - 43 edges
7. `Security requirements` - 39 edges
8. `assert_run()` - 37 edges
9. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 35 edges
10. `Risk register` - 34 edges

## Surprising Connections (you probably didn't know these)
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md
- `Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing)` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Scope 001 wave 1 ticked boxes (build notes, scaffold, verify, test) and their log entries** — docs_scope_coordination_001_stack_and_workspace_log_checked_build_notes, docs_scope_coordination_001_stack_and_workspace_log_checked_scaffold, docs_scope_coordination_001_stack_and_workspace_log_checked_verify, docs_scope_coordination_001_stack_and_workspace_log_checked_test, docs_scope_001_stack_and_workspace_box_build_notes, docs_scope_001_stack_and_workspace_box_scaffold, docs_scope_001_stack_and_workspace_box_verify, docs_scope_001_stack_and_workspace_box_test [EXTRACTED 1.00]
- **Spec 0001 acceptance criteria tied to test files** — docs_specs_0001_initial_architecture_verify_ac_coverage, apps_pool_tests_check_config, crates_pool_config_tests_validate, apps_pool_src_cli, crates_pool_config_src_lib [EXTRACTED 1.00]
- **Spec 0001 full verify BLOCKED on the Linux build (verdict, AC-2, AC-11 /dev/full, F1-1, F1-2, Linux-only tests)** — docs_specs_0001_initial_architecture_verify_full_verify_verdict, docs_specs_0001_initial_architecture_verify_full_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, apps_pool_tests_check_config_ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr, docs_scope_coordination_001_stack_and_workspace_log_blocker_linux_build_not_run [EXTRACTED 1.00]
- **Spec 0001 full verify evidence against the real binary (commands, fixtures, probe, scratch crate, coverage)** — docs_specs_0001_initial_architecture_verify_full_verify_commands, docs_specs_0001_initial_architecture_verify_full_verify_nm_imports, docs_specs_0001_initial_architecture_verify_full_verify_fixture_runs, docs_specs_0001_initial_architecture_verify_full_verify_byte_probe, docs_specs_0001_initial_architecture_verify_full_verify_scratch_crate, docs_specs_0001_initial_architecture_verify_full_verify_ac_coverage, docs_specs_0001_initial_architecture_verify_full_verify_ac_pass, docs_specs_0001_initial_architecture_verify_full_verify_extra_runs [EXTRACTED 1.00]
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

## Communities (50 total, 0 thin omitted)

### Community 0 - "Standards, Tooling and Release Qualification"
Cohesion: 0.15
Nodes (24): Git and Release Standards, Tests and Evidence Standard, Release Practices, R-16 Dependency drift or abandoned crate, criterion 0.8.2 (microbenchmarks), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply), 11. Release qualification harness (planned), Qualified release artifacts (tagged package or image, checksums, SBOM, notices) (+16 more)

### Community 1 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.16
Nodes (25): Aturan umum brief: 001-stack-and-workspace (general brief rules), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first), Always forbidden paths for sub sessions (docs/scope, docs/prd, AGENTS.md, CLAUDE.md, graphify-out, .claude, spec content outside ownership), Role files written for another project (Stackly references do not apply) (+17 more)

### Community 2 - "Feature Inventory and Parity Matrix"
Cohesion: 0.09
Nodes (35): Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-024 Transaction Mode (refused, session pooling only), F-056 Primary Standby Role Observation, F-065 Primary Identification, F-065 Primary identification (durably approved writer with fresh evidence) (+27 more)

### Community 3 - "Administration and Cluster PRDs"
Cohesion: 0.21
Nodes (12): Axum REST administration API plus CLI, pool check-config, GET /v1/nodes node status route, Promoter role distinct from observer/operator, Administration PRD, AdminPrincipal, APIError, Authenticated management transport (protected local or mTLS) (+4 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.36
Nodes (15): Data Plane vs Control Plane Separation, Planned Architecture, Topology Snapshot, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence (+7 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.08
Nodes (30): Invariant: close uncertain pooled state, F-030 Validation and reset (close uncertain sessions), Support Profile, Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming (+22 more)

### Community 6 - "Risk Register"
Cohesion: 0.16
Nodes (19): Documentation Ownership Roles (scope, specs, PRDs, ADRs), Slice, Verification Record, Handoff for Later Slices, Stable ID Citation Rule, Where a Change Belongs (documentation ownership table), Dependencies and critical path, Milestones (+11 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.15
Nodes (22): Finding (stable rejection code), secret-* canary fixtures (CANARY-9f2c7b1e), The check-config command (arguments and usage errors), config_version key (required, supports exactly 1), FindingCode (config.* codes), Findings (stable finding codes), First build consequences (no leak by construction, terse findings, fixed public contracts), Fixtures (valid-*, invalid-*, secret-* TOML files) (+14 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.12
Nodes (31): Control plane, Dependency and Configuration Change Rules, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), F-066 Topology reconciliation (durable writer/epoch/quarantine), F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Glossary (+23 more)

### Community 9 - "Delivery Phases, Milestones and Risks"
Cohesion: 0.33
Nodes (10): Roadmap, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy, Phase 3 SQL routing, Phase 4 Production reliability, Phase 5 High availability, Timeline and estimation model (+2 more)

### Community 10 - "Roadmap and Routing PRDs"
Cohesion: 0.17
Nodes (12): Planning assumptions and decisions, Apache 2.0 license, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, One daemon with separate modules plus separate CLI, pg_query parser with PostgreSQL 18 grammar, Prometheus metrics and tracing logs, rustls and tokio-rustls with one crypto provider (+4 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.20
Nodes (17): Tracer Bullet Build Approach, Protocol Features (F-001 to F-022), Walking Slice (glossary term), Core MVP, Walking Slice Milestone, R-01 Protocol incompatibility, R-24 Walking slice exceeds its estimate, Core proxy PRD (+9 more)

### Community 12 - "Connection Pooling and Leases"
Cohesion: 0.16
Nodes (18): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Durable approved writer authority, Initial admission boundary (+10 more)

### Community 13 - "Split Brain and Journal Risks"
Cohesion: 0.24
Nodes (11): F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing), R-03 Session state leakage, R-08 Split brain, R-18 Journal disk loss or replay, Operational security defaults, SEC-7 Persist administrative intent before an external effect (+3 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.11
Nodes (28): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy (+20 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.12
Nodes (25): Cluster management, Authenticated SQL health probes, Configured backend registration, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Observation record, Value sourcing table (+17 more)

### Community 16 - "Manual Failover and Recovery"
Cohesion: 0.27
Nodes (10): Invariant: health failure does not authorize promotion, F-060 Network partitions (health changes eligibility only), Health failure alone produces no promotion, Manual failover PRD, Durable intent before fencing and reconciliation of unknown outcomes, Failover plan and execute API (/v1/failover/plans), Failover operation phases (Planned to Completed or Failed/NeedsReview), PromotionPlan (+2 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (46): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+38 more)

### Community 18 - "Scope Status and Documentation Ownership"
Cohesion: 0.23
Nodes (19): Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Log event fields and redaction convention (ts, level, event; redaction by type), Shared Conventions Table, First Handoff: the Workspace, Critical Path Handoffs, Cross Check Before Development, 1. Stack and workspace (in-progress) (+11 more)

### Community 19 - "Proposed Stack and Deferred Features"
Cohesion: 0.17
Nodes (13): Transaction pooling (deferred), Local SQLite operation journal (single writer), No automatic retry after dispatch, Future multi proxy deployment, Failure behavior (no transparent failover, no implicit replay), Transparent per query routing (later feature), Versioned local admin REST API (Axum), Deferred features (cache, automatic failover, native replication, multiplexing) (+5 more)

### Community 20 - "Spec 0001, Stack and Project Guides"
Cohesion: 0.17
Nodes (18): tracing 0.1.44 (structured events and spans), Delegated decision (berat, 0001 AC-9): check-config diagnostics contain only constant text, schema key path, line and column, never file content, SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL, SEC-4 Secrets in protected files or provider references; redaction, T-12 Secret disclosure through diagnostics, First build decision (two packages, three dependencies, fixed finding vocabulary), Follow-up: file permission check in scope 004 and tracing with the first log event, Log events and redaction (shared log convention) (+10 more)

### Community 21 - "Core Proxy PRD and Decision Ledger"
Cohesion: 0.18
Nodes (14): Retry only before initial lease and ReadyForQuery exposure, Authoritative barrier across every write path, Independent review resolutions (seven approved fixes), Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Routing decision: primary default, explicit read only session profiles, Cancellation forwarded only through the live lease owner, CancellationEntry (+6 more)

### Community 22 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (62): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+54 more)

### Community 23 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.15
Nodes (19): Virtual Cancellation Model, Session Model (pinned backend lease), Pool limits and operations, Data plane, Frontend owner task, Provisional resource defaults and backpressure, Architecture overview, Application path sequence (+11 more)

### Community 24 - "pool-config Validation Library"
Cohesion: 0.07
Nodes (32): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+24 more)

### Community 25 - "HA and Recovery Features and Threats"
Cohesion: 0.27
Nodes (10): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-21 Proxmox host correlated failure, 10. Fenced manual promotion (planned), 12. Multiple proxy authority (deferred), 13. Checked online recovery (deferred), SEC-5 Least privilege database logins; separate monitor and promotion roles (+2 more)

### Community 26 - "Agent Guide and Scope 001 Authorization"
Cohesion: 0.13
Nodes (23): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress) (+15 more)

### Community 27 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (84): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_first_argument_that_is_not_a_command_is_unknown() (+76 more)

### Community 28 - "SQL Routing PRD"
Cohesion: 0.36
Nodes (9): Policy Version, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+1 more)

### Community 29 - "Load Balancing PRD"
Cohesion: 0.36
Nodes (9): No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state), Replica eligibility filter (role, identity, freshness, maintenance, lag), Primary fallback disabled in V1, ReplicaPolicy, SelectorState (+1 more)

### Community 30 - "Safety Invariants and Session Risks"
Cohesion: 0.07
Nodes (39): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: fail closed on unknown role or unsafe routing, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: preserve transaction and session ownership, Invariant: promotion requires a reviewed operation and independently verified fencing (+31 more)

### Community 31 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.16
Nodes (18): Format, Invocation, parse(), parse_check_config(), run(), check_config(), main(), read_bounded() (+10 more)

### Community 32 - "Query Cache PRD"
Cohesion: 0.24
Nodes (10): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Credential and configuration generation retirement, PoolKey, PoolState, Query cache PRD, Caching disabled until a separate contract is approved and proven (+2 more)

### Community 33 - "Spec 0001 Acceptance Criteria"
Cohesion: 0.37
Nodes (21): AC-10 JSON contract (one JSON object line on stdout, empty stderr), AC-13 Stop line and hygiene (no later crates, unsafe_code forbid, fmt, clippy, test pass), AC-3 Valid configuration (valid-* fixtures exit 0 with exact report), AC-4 Off is safe (reserved capability absent, empty or false validates as disabled), AC-5 Result cache is refused (config.unsupported_capability at result_cache.enabled), AC-6 Automatic promotion is refused (config.unsupported_capability at automatic_promotion.enabled), AC-7 Invalid configurations fail (invalid-*, non UTF-8 and over 1048576 bytes exit 1), AC-8 Whole configuration (all findings in one run; validate is the only path to ValidatedConfig) (+13 more)

### Community 34 - "Linux-only Write Tests and Verify Blocker"
Cohesion: 0.17
Nodes (22): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Open blocker: 0001 AC-2 and the /dev/full clause of AC-11 not run (docker info failed at 14:30, 16:40, 17:10); two Linux-only tests NOT_RUN; blocks feature 1 done, not review, Decision: developer AC readings passed to chk-f1-verify (--help=x and -h=x as help, JSON field order on the raw line, /dev/full test Linux only, milestone number 3), F1-2 pass criteria: exit 0, host x86_64-unknown-linux-gnu, tests pass including the two Linux-only failed-write tests, one block per fixture and format (0 valid-*, 1 others), no canary leaked, last line linux build check passed, AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full), Follow-up: start Docker so the Linux build (AC-2) can run, 0001. Verification record (+14 more)

### Community 35 - "Health Monitoring and Probes"
Cohesion: 0.18
Nodes (18): F-101 TLS and mTLS verification, Health features F-054 to F-060, Routing Preview, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control (+10 more)

### Community 36 - "Coordination Sessions and Merge Decisions"
Cohesion: 0.20
Nodes (16): Box ticked: Test it (/test stack and workspace), Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Standby first turn of a new session (STATUS: standby until lead sends 'mulai'), Checked in wave 1: F1 Test it (tst-f1-test commits 5ad687d, fae4bc6; 127 tests green on main; F1-2 NOT_RUN for two Linux-only tests), Sub session chk-f1-review (checker review on a fresh model, reserve; writes docs/reviews/), Decision: dev-f1-build branch passed the gate on the staged merge (Rust 1.100.0: fmt, clippy, 31 tests, four fixture commands, Cargo.lock 18 packages, AC-12 grep empty); merged as 5f44d88, Decision: five sessions started on the first user turn (one live, four standby), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8> (+8 more)

### Community 37 - "Failover Records and Fencing Authority"
Cohesion: 0.18
Nodes (18): Drain, attach and quarantine operations, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Deployment reference, Initial four VM lab, Independent fencing authority (+10 more)

### Community 38 - "ADR 0001, Identity and Architecture Rationale"
Cohesion: 0.22
Nodes (14): Apache 2.0 project license, Separate Frontend and Backend Identity, ADR 0001. Initial architecture, Backend Login, Identity Map, Verified Principal, Confirmed project defaults (two engineers, conservative lab validation, Apache 2.0, one modular daemon plus CLI), Spec 0001. POOL initial architecture (+6 more)

### Community 39 - "Security Features and Review Roles"
Cohesion: 0.11
Nodes (27): Administration requires authentication and authorization, Rust and Runtime Standards, Security and Review Requirements, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-107 Rate and resource limits (admission, frame and auth CPU budgets), F-108 Secure reload (validate entire generation before publication) (+19 more)

### Community 40 - "Agent Workflow and Documentation Index"
Cohesion: 0.21
Nodes (19): Agent Workflow, Claude Code guidance (CLAUDE.md), Planned Cargo Check Commands, Working with the Docs, Contributing to POOL, Engineering standards, Engineering Workflow, Starting development (implementation order) (+11 more)

### Community 41 - "Linux Target and Build Plan Evidence"
Cohesion: 0.27
Nodes (17): Open blocker: proxy and PostgreSQL VM architecture (x86_64 or ARM) unconfirmed for 0001 AC-2; x86_64 target inferred (F1-N1), Decision: Build plan steps 4 and 5 make the developer write verify.md and its docs/index.md entry; chk-f1-verify runs and ticks it, Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; awaiting user VM architecture, User value F1-N1: proxy and PostgreSQL VM architecture (x86_64 or ARM); x86_64-unknown-linux-gnu inferred from Proxmox VE, AC-2 Linux build (container check ends with 'linux build check passed'), Build plan (tracer bullet, five steps), Build plan step 4: Workstation evidence, Build plan step 5: Linux evidence (+9 more)

### Community 42 - "Workspace Build Notes and Offline Bounds"
Cohesion: 0.24
Nodes (17): Delegated decisions (biasa, 0001 AC-1, AC-12): 17 build notes choices (pinned rust slim bookworm image and git archive HEAD, std argument parsing, serde_json, no thiserror/anyhow/tracing, two pass validation, 1 MiB limit, ...), AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read), Bounded configuration read (at most 1048577 bytes via Read::take), Build plan step 1: Workspace skeleton, Cargo.lock with exactly sixteen locked packages, rust-toolchain.toml (channel 1.100.0, rustfmt and clippy), Decision: 1 MiB configuration file size limit (+9 more)

### Community 43 - "Full Verify Record and Checker Session"
Cohesion: 0.23
Nodes (12): Box ticked: Verify it (/check verify stack and workspace), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test), Decision: 'Scaffold from the decision' treated as Build it, closed together with Verify it, Decision: chk-f1-verify blocked only on the manual Linux build; Scaffold and Verify it boxes ticked with F1-1 and F1-2 NOT_RUN, feature 1 not done until 0001 AC-2 is proven, Full verify: /check verify stack and workspace (checker chk-f1-verify, 2026-11-14, rerun from its own checkout against the real pool binary), Binary under test: target/release/pool from cargo build --locked --release (Mach-O arm64; macOS checksum, not the AC-2 record), Full verify clean checkout: empty git status and no target/ before the first command (+4 more)

### Community 44 - "Reserved Capabilities and Config Rules"
Cohesion: 0.38
Nodes (10): Automatic Promotion (reserved and off), Reserved Capability, Result Cache (reserved and off), Delegated decision (berat, 0001 AC-4, AC-6): automatic promotion capability key named automatic_promotion (cache: result_cache) over automatic_failover, SEC-10 Disable automatic promotion and result caching by default, Configuration rules (shared configuration key convention), Reserved capability tables (result_cache, automatic_promotion; enabled = true rejected), Secret references only by a _file key (+2 more)

### Community 45 - "KPIs and Proposed Gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

### Community 46 - "Manual Steps and Docker Blocker"
Cohesion: 0.44
Nodes (10): Box ticked: Scaffold from the decision (/develop stack and workspace), Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user), Open blocker: Linux build evidence blocked while the Docker daemon is off, Checked in wave 1: F1 Scaffold from the decision (dev-f1-build done and chk-f1-verify report, merged 5f44d88; manual F1-1, F1-2 NOT_RUN), Langkah manual: 001-stack-and-workspace (manual steps for the user), Manual step F1-1: start Docker Desktop so the developer and verify sessions can run the Linux build (0001 AC-2; NOT_RUN for boxes Scaffold from the decision and Verify it) (+2 more)

### Community 47 - "Cluster Management and Approved Writer"
Cohesion: 0.28
Nodes (9): Persisted approved writer, epoch and quarantine, SQLite journal for operations, audit and approved writer/epoch/quarantine, Cluster management PRD, Approved writer, epoch and quarantine authority, Drain, detach and attach node operations, Two observed primaries close write admission, Generation checked journaled mutations, Separation of observed health/role from maintenance and desired membership (+1 more)

### Community 48 - "High Availability PRD and Topology"
Cohesion: 0.36
Nodes (8): High availability PRD, ControllerEpoch, Endpoint failover via external load balancing or controlled VIP, Established consensus for one operation authority, External coordination adapter, Query routing on local immutable snapshots, ProxyMember, TopologyRecord

### Community 49 - "Exit Status and JSON Output Contract"
Cohesion: 0.47
Nodes (6): Output Version (output_version field), Delegated decision (berat, 0001 AC-3, AC-7, AC-11): exit status 0 success, 1 input rejected, 2 usage, 3 could not complete, as a Shared convention, Exit status and output (shared output convention), Exit status 0 success, 1 rejected, 2 usage error, 3 could not complete, JSON output contract (output_version, command, result ... findings), Decision: exit status 0, 1, 2, 3 over sysexits

## Knowledge Gaps
- **52 isolated node(s):** `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Agent Workflow and Documentation Index` to `Standards, Tooling and Release Qualification`, `Feature Inventory and Parity Matrix`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Risk Register`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Feature Families and Planned Scopes`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Pgpool Baseline, Competitors and Feasibility`, `Scope Status and Documentation Ownership`, `Virtual Cancellation and Architecture Overview`, `HA and Recovery Features and Threats`, `Agent Guide and Scope 001 Authorization`, `SQL Routing PRD`, `Load Balancing PRD`, `Query Cache PRD`, `Linux-only Write Tests and Verify Blocker`, `Health Monitoring and Probes`, `Failover Records and Fencing Authority`, `ADR 0001, Identity and Architecture Rationale`, `Security Features and Review Roles`, `Linux Target and Build Plan Evidence`, `Cluster Management and Approved Writer`, `High Availability PRD and Topology`?**
  _High betweenness centrality (0.345) - this node is a cross-community bridge._
- **Why does `0001. Verification record` connect `Linux-only Write Tests and Verify Blocker` to `Standards, Tooling and Release Qualification`, `Spec 0001 Acceptance Criteria`, `Coordination Sessions and Merge Decisions`, `ADR 0001, Identity and Architecture Rationale`, `Reserved Capabilities and First Build Criteria`, `Agent Workflow and Documentation Index`, `Linux Target and Build Plan Evidence`, `Workspace Build Notes and Offline Bounds`, `Full Verify Record and Checker Session`, `Scope Status and Documentation Ownership`, `pool CLI Entry and Argument Parsing`?**
  _High betweenness centrality (0.154) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `ADR 0001, Identity and Architecture Rationale` to `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Risk Register`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Roadmap and Routing PRDs`, `Feature Families and Planned Scopes`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `Manual Failover and Recovery`, `Scope Status and Documentation Ownership`, `Proposed Stack and Deferred Features`, `Spec 0001, Stack and Project Guides`, `Virtual Cancellation and Architecture Overview`, `Agent Guide and Scope 001 Authorization`, `SQL Routing PRD`, `Load Balancing PRD`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Linux-only Write Tests and Verify Blocker`, `Health Monitoring and Probes`, `Agent Workflow and Documentation Index`, `Linux Target and Build Plan Evidence`, `Cluster Management and Approved Writer`, `High Availability PRD and Topology`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Feature Inventory and Parity Matrix` be split into smaller, more focused modules?**
  _Cohesion score 0.08571428571428572 - nodes in this community are weakly interconnected._
- **Should `Protocol Contract and Driver Compatibility` be split into smaller, more focused modules?**
  _Cohesion score 0.07816091954022988 - nodes in this community are weakly interconnected._