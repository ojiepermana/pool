# Graph Report - pool  (2026-11-14)

## Corpus Check
- 8 files · ~74,783 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1104 nodes · 3190 edges · 37 communities
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 618 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Standards, Tooling and Release Qualification
- Scope 001 Coordination Brief, Log and Authorization
- Feature Inventory and Parity Matrix
- Administration, Cluster and Health PRDs
- Architecture Spec and Repository Layout
- Protocol Contract and Driver Compatibility
- Operation Journal Records and Generations
- Reserved Capabilities and First Build Criteria
- Glossary and Routing Profiles
- Delivery Phases, Milestones and HA Features
- Offline Bounds, Parser Depth and Known Limits
- Feature Families and Planned Scopes
- Session Pooling and Architecture Rationale
- Promotion, Fencing and Split Brain Risks
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Protocol Engine and Proposed Stack
- Pgpool Baseline, Competitors and Feasibility
- Scope Status and Documentation Ownership
- Virtual Cancellation and Architecture Overview
- Resource Limits and Trust Domains
- Core Proxy PRD, Decision Ledger and Routing PRDs
- pool-config Validation Tests
- Session Model, Pooling Modes and Data Path
- pool-config Validation Library
- check-config Integration Tests
- Safety Invariants and Session Risks
- pool CLI Entry and Argument Parsing
- Query Cache PRD
- Spec 0001 Acceptance Criteria
- Review Clarifications and check-config Usage Rules
- Failover Records and Fencing Authority
- Security Features and Review Roles
- Agent Workflow, Documentation Index and Risks
- Workspace Skeleton, Verify Record and Linux-only Tests
- Reserved Capabilities and Config Rules
- KPIs and Proposed Gates

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 76 edges
2. `Glossary` - 58 edges
3. `Spec 0001. POOL initial architecture` - 55 edges
4. `Feature inventory and parity matrix` - 47 edges
5. `Testing strategy` - 43 edges
6. `Scope: POOL, order and status index` - 43 edges
7. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 43 edges
8. `Security requirements` - 39 edges
9. `assert_run()` - 38 edges
10. `Risk register` - 34 edges

## Surprising Connections (you probably didn't know these)
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12)` --references--> `ac7_column_counts_characters_not_bytes_before_the_error()`  [INFERRED]
  docs/specs/0001-initial-architecture/index.md → crates/pool-config/tests/validate.rs
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Reading: --help=x and -h=x after check-config print the help and exit 0; as first argument they are unknown command (agrees with the spec, one example line would remove doubt)` --references--> `parse_check_config()`  [INFERRED]
  docs/specs/0001-initial-architecture/verify.md → apps/pool/src/cli.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Scope 001 review and fix round (review Major RawCapability, reused developer and architect sessions, table-only rule, joint merge)** — docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review, docs_scope_coordination_001_stack_and_workspace_log_decision_review_major_rawcapability, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build_reused, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update_reused, docs_scope_coordination_001_stack_and_workspace_log_decision_fix_round_joint_merge, crates_pool_config_src_lib_rawcapability_deserialize, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only [EXTRACTED 1.00]
- **Scope 001 wave 1 ticked boxes (build notes, scaffold, verify, test) and their log entries** — docs_scope_coordination_001_stack_and_workspace_log_checked_build_notes, docs_scope_coordination_001_stack_and_workspace_log_checked_scaffold, docs_scope_coordination_001_stack_and_workspace_log_checked_verify, docs_scope_coordination_001_stack_and_workspace_log_checked_test, docs_scope_001_stack_and_workspace_box_build_notes, docs_scope_001_stack_and_workspace_box_scaffold, docs_scope_001_stack_and_workspace_box_verify, docs_scope_001_stack_and_workspace_box_test [EXTRACTED 1.00]
- **Spec 0001 acceptance criteria tied to test files** — docs_specs_0001_initial_architecture_verify_ac_coverage, apps_pool_tests_check_config, crates_pool_config_tests_validate, apps_pool_src_cli, crates_pool_config_src_lib [EXTRACTED 1.00]
- **Spec 0001 first build acceptance criteria AC-1 to AC-13** — docs_specs_0001_initial_architecture_index_ac_1, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_3, docs_specs_0001_initial_architecture_index_ac_4, docs_specs_0001_initial_architecture_index_ac_5, docs_specs_0001_initial_architecture_index_ac_6, docs_specs_0001_initial_architecture_index_ac_7, docs_specs_0001_initial_architecture_index_ac_8, docs_specs_0001_initial_architecture_index_ac_9, docs_specs_0001_initial_architecture_index_ac_10, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_index_ac_12, docs_specs_0001_initial_architecture_index_ac_13 [EXTRACTED 1.00]
- **Spec 0001 build plan steps 1 to 5 (tracer bullet)** — docs_specs_0001_initial_architecture_index_build_plan_step_1, docs_specs_0001_initial_architecture_index_build_plan_step_2, docs_specs_0001_initial_architecture_index_build_plan_step_3, docs_specs_0001_initial_architecture_index_build_plan_step_4, docs_specs_0001_initial_architecture_index_build_plan_step_5 [EXTRACTED 1.00]
- **Spec 0001 full verify BLOCKED on the Linux build (verdict, AC-2, AC-11 /dev/full, F1-1, F1-2, Linux-only tests)** — docs_specs_0001_initial_architecture_verify_full_verify_verdict, docs_specs_0001_initial_architecture_verify_full_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, apps_pool_tests_check_config_ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr, docs_scope_coordination_001_stack_and_workspace_log_blocker_linux_build_not_run [EXTRACTED 1.00]
- **Spec 0001 full verify evidence against the real binary (commands, fixtures, probe, scratch crate, coverage)** — docs_specs_0001_initial_architecture_verify_full_verify_commands, docs_specs_0001_initial_architecture_verify_full_verify_nm_imports, docs_specs_0001_initial_architecture_verify_full_verify_fixture_runs, docs_specs_0001_initial_architecture_verify_full_verify_byte_probe, docs_specs_0001_initial_architecture_verify_full_verify_scratch_crate, docs_specs_0001_initial_architecture_verify_full_verify_ac_coverage, docs_specs_0001_initial_architecture_verify_full_verify_ac_pass, docs_specs_0001_initial_architecture_verify_full_verify_extra_runs [EXTRACTED 1.00]
- **Spec 0001 Linux build blocked on Docker (AC-2, F1-1, F1-2, /dev/full)** — docs_specs_0001_initial_architecture_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_linux_build, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_follow_up_start_docker, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, docs_specs_0001_initial_architecture_verify_ac_11_partial [EXTRACTED 1.00]
- **Spec 0001 clarifications from the scope 001 review (nine rationale rows and the spec text they explain)** — docs_specs_0001_initial_architecture_rationale_review_clarifications, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only, docs_specs_0001_initial_architecture_rationale_clarification_2_equals_not_utf8, docs_specs_0001_initial_architecture_rationale_clarification_3_help_version_equals, docs_specs_0001_initial_architecture_rationale_clarification_4_nesting_depth, docs_specs_0001_initial_architecture_rationale_clarification_5_permission_denied_test, docs_specs_0001_initial_architecture_rationale_clarification_6_exact_column, docs_specs_0001_initial_architecture_rationale_clarification_7_fifo_limit, docs_specs_0001_initial_architecture_rationale_clarification_8_ci_follow_up, docs_specs_0001_initial_architecture_rationale_clarification_9_status_wording, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_index_equals_not_utf8_unexpected_argument, docs_specs_0001_initial_architecture_index_option_name_before_equals, docs_specs_0001_initial_architecture_index_nesting_depth_syntax_no_location, docs_specs_0001_initial_architecture_index_permission_denied_test, docs_specs_0001_initial_architecture_index_exact_column_test, docs_specs_0001_initial_architecture_index_fifo_known_limit, docs_specs_0001_initial_architecture_index_follow_up_ci_checks_ac12_ac13, docs_specs_0001_initial_architecture_index_status_follows_scope_feature [EXTRACTED 1.00]
- **Spec 0001 workstation evidence (commands, tests, fixtures, locked set)** — docs_specs_0001_initial_architecture_verify_workstation_evidence, docs_specs_0001_initial_architecture_verify_workstation_commands, docs_specs_0001_initial_architecture_verify_test_results, docs_specs_0001_initial_architecture_verify_fixture_transcripts, docs_specs_0001_initial_architecture_verify_locked_dependencies, docs_specs_0001_initial_architecture_verify_toolchain_selection, docs_specs_0001_initial_architecture_verify_fixtures, docs_specs_0001_initial_architecture_verify_ac_pass_workstation [EXTRACTED 1.00]
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **Scope 001 shared conventions (configuration keys, exit status and output, log events)** — docs_engineering_standards_configuration_keys_convention, docs_engineering_standards_exit_status_output_convention, docs_engineering_standards_log_event_convention, docs_specs_0001_initial_architecture_index_configuration_rules, docs_specs_0001_initial_architecture_index_exit_status_and_output, docs_specs_0001_initial_architecture_index_log_events_and_redaction [EXTRACTED 1.00]
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
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (37 total, 0 thin omitted)

### Community 0 - "Standards, Tooling and Release Qualification"
Cohesion: 0.16
Nodes (24): Git and Release Standards, Protocol Features (F-001 to F-022), Milestones, Core MVP, Release Practices, V1 Release Gate, R-16 Dependency drift or abandoned crate, Lag computed only for comparable fresh positions; missing evidence is unknown (+16 more)

### Community 1 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.05
Nodes (108): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Box ticked: Verify it (/check verify stack and workspace), 2. Standards and tooling (planned), Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon) (+100 more)

### Community 2 - "Feature Inventory and Parity Matrix"
Cohesion: 0.10
Nodes (30): Invariant: health failure does not authorize promotion, Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-060 Network partitions (health changes eligibility only), F-065 Primary Identification (+22 more)

### Community 3 - "Administration, Cluster and Health PRDs"
Cohesion: 0.14
Nodes (24): Axum REST administration API plus CLI, Health monitoring PRD, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control, Health monitoring failure scenarios (SQL failure, revoked permission, clock jump, replay pause, primary isolation), GET /v1/nodes node status route, HealthState (Unknown, Healthy, Suspect, Unavailable, Recovering) (+16 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.38
Nodes (14): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+6 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.09
Nodes (27): Support Profile, Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming, Extended Query and pipelining behavior, Independent backend credentials per mapped login identity (+19 more)

### Community 6 - "Operation Journal Records and Generations"
Cohesion: 0.25
Nodes (14): Control plane, Durable approved writer authority, ApprovedTopology record, AuditEvent record, FenceReceipt record, Observation record, Operation record, Local SQLite operation journal (single writer) (+6 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.16
Nodes (22): Finding (stable rejection code), Output Version (output_version field), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Checked in wave 1: F1 build notes recorded (arc-f1-update done, commit a6e0042, merged to bed9fad; AC-1..AC-13 and 5 step Build plan present, docs only), config_version key (required, supports exactly 1), Exit status and output (shared output convention), FindingCode (config.* codes), Findings (stable finding codes) (+14 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.11
Nodes (32): Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Dependency and Configuration Change Rules, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Approved Writer, Backend Login (+24 more)

### Community 9 - "Delivery Phases, Milestones and HA Features"
Cohesion: 0.12
Nodes (24): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), Routing Preview, V1.5 Milestone, R-21 Proxmox host correlated failure, Roadmap, Phase 0 Research and architecture, Phase 1 Foundation (+16 more)

### Community 10 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.23
Nodes (13): ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Decision: dev-f1-build branch passed the gate on the staged merge (Rust 1.100.0: fmt, clippy, 31 tests, four fixture commands, Cargo.lock 18 packages, AC-12 grep empty); merged as 5f44d88, Delegated decisions (biasa, 0001 AC-1, AC-12): 17 build notes choices (pinned rust slim bookworm image and git archive HEAD, std argument parsing, serde_json, no thiserror/anyhow/tracing, two pass validation, 1 MiB limit, ...), AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read), Bounded configuration read (at most 1048577 bytes via Read::take), Cargo.lock with exactly sixteen locked packages, Known limit: check-config reads special files like any file, so a FIFO with no writer blocks the command with no time limit; no code change in scope 001 (+5 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.15
Nodes (18): Tracer Bullet Build Approach, Slice, Verification Record, Walking Slice (glossary term), Terminated SCRAM over TLS with optional mTLS and separate backend credentials, Walking Slice Milestone, R-24 Walking slice exceeds its estimate, R-25 Reviewer or lab unavailable at a gate (+10 more)

### Community 12 - "Session Pooling and Architecture Rationale"
Cohesion: 0.23
Nodes (13): Session pooling (one frontend owns one backend lease), Login time backend selection on pinned lease, SEC-4 Secrets in protected files or provider references; redaction, No file content in output (constant text, key paths, line and column only), Ten safety contracts, Initial architecture rationale, Build notes context (macOS workstation, three independent roles, three forces), Independent critique and seven resolutions (+5 more)

### Community 13 - "Promotion, Fencing and Split Brain Risks"
Cohesion: 0.11
Nodes (26): Invariant: promotion requires a reviewed operation and independently verified fencing, Safe first party Rust and bounded resources, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), Failover features F-067 to F-075, Fence Receipt (+18 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.08
Nodes (41): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), criterion 0.8.2 (microbenchmarks) (+33 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.21
Nodes (15): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table (+7 more)

### Community 16 - "Protocol Engine and Proposed Stack"
Cohesion: 0.18
Nodes (13): Operation admission boundary, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Routing decision matrix, SQL classification through PostgreSQL 18 raw grammar (+5 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.07
Nodes (44): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+36 more)

### Community 18 - "Scope Status and Documentation Ownership"
Cohesion: 0.19
Nodes (14): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Feature status model (Research, Planned; none Implemented or Tested), Documentation checks (links/anchors, scope status agreement, stable requirement IDs), Decision: Build plan step 1 sets spec Status to In Progress; the stale 'lifecycle status remains Proposed' wording is tidied at /sync, not by sub sessions, Scope file ownership rules, GA workflow depth (+6 more)

### Community 19 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.17
Nodes (16): Virtual Cancellation Model, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), No automatic retry after dispatch, Future multi proxy deployment, Architecture overview, Failure behavior (no transparent failover, no implicit replay), Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan) (+8 more)

### Community 20 - "Resource Limits and Trust Domains"
Cohesion: 0.20
Nodes (15): Rust and Runtime Standards, F-107 Rate and resource limits (admission, frame and auth CPU budgets), R-13 Memory exhaustion, SEC-8 Global and per principal connection, byte, queue, parser and auth CPU budgets, Control module, Data plane, Database trust domain, Operation journal (+7 more)

### Community 21 - "Core Proxy PRD, Decision Ledger and Routing PRDs"
Cohesion: 0.06
Nodes (43): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, Independent review resolutions (seven approved fixes), One daemon with separate modules plus separate CLI (+35 more)

### Community 22 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (58): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+50 more)

### Community 23 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.11
Nodes (22): Session Model (pinned backend lease), Connection pooling, Pool key (pool identity), Pool limits and operations, Statement pooling (not planned for V1), Transaction pooling (deferred), Data plane, Frontend owner task (+14 more)

### Community 24 - "pool-config Validation Library"
Cohesion: 0.06
Nodes (45): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+37 more)

### Community 27 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (87): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+79 more)

### Community 30 - "Safety Invariants and Session Risks"
Cohesion: 0.09
Nodes (33): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: preserve transaction and session ownership, Invariant: keep writes on the verified primary (+25 more)

### Community 31 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.17
Nodes (19): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), check_config() (+11 more)

### Community 32 - "Query Cache PRD"
Cohesion: 0.29
Nodes (10): V2 Milestone, Phase 6 Advanced features, Connection pooling PRD, Credential and configuration generation retirement, PoolKey, PoolState, Session pooling only (no transaction or statement multiplexing in V1), Query cache PRD (+2 more)

### Community 33 - "Spec 0001 Acceptance Criteria"
Cohesion: 0.30
Nodes (25): First Handoff: the Workspace, AC-10 JSON contract (one JSON object line on stdout, empty stderr), AC-13 Stop line and hygiene (no later crates, unsafe_code forbid, fmt, clippy, test pass), AC-3 Valid configuration (valid-* fixtures exit 0 with exact report), AC-4 Off is safe (reserved capability absent, empty or false validates as disabled), AC-5 Result cache is refused (config.unsupported_capability at result_cache.enabled), AC-6 Automatic promotion is refused (config.unsupported_capability at automatic_promotion.enabled), AC-7 Invalid configurations fail (invalid-*, non UTF-8 and over 1048576 bytes exit 1) (+17 more)

### Community 36 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.18
Nodes (22): Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, Delegated decision (berat, 0001 AC-3, AC-7, AC-11): exit status 0 success, 1 input rejected, 2 usage, 3 could not complete, as a Shared convention, Delegated decision (0001 AC-7, AC-11): nine spec clarifications from the review, spec follows the existing code and the reviewer and tester recommendation; runner up change the code (--format=<bytes> as wrong format value); status berlaku, AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full), The check-config command (arguments and usage errors), --format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument, Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12) (+14 more)

### Community 37 - "Failover Records and Fencing Authority"
Cohesion: 0.16
Nodes (18): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+10 more)

### Community 39 - "Security Features and Review Roles"
Cohesion: 0.09
Nodes (38): Administration requires authentication and authorization, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Separate Frontend and Backend Identity, Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Log event fields and redaction convention (ts, level, event; redaction by type), Security and Review Requirements, Shared Conventions Table, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping) (+30 more)

### Community 40 - "Agent Workflow, Documentation Index and Risks"
Cohesion: 0.09
Nodes (57): POOL agent guide (AGENTS.md), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), Agent Workflow, Claude Code guidance (CLAUDE.md), Planned Cargo Check Commands, Working with the Docs (+49 more)

### Community 42 - "Workspace Skeleton, Verify Record and Linux-only Tests"
Cohesion: 0.15
Nodes (24): Decision: developer AC readings passed to chk-f1-verify (--help=x and -h=x as help, JSON field order on the raw line, /dev/full test Linux only, milestone number 3), AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), Build plan (tracer bullet, five steps), Build plan step 1: Workspace skeleton, Build plan step 4: Workstation evidence, rust-toolchain.toml (channel 1.100.0, rustfmt and clippy), 0001. Verification record, AC-11 passes on the workstation; /dev/full part waits for the Linux build (+16 more)

### Community 44 - "Reserved Capabilities and Config Rules"
Cohesion: 0.62
Nodes (7): Automatic Promotion (reserved and off), Reserved Capability, Result Cache (reserved and off), Delegated decision (berat, 0001 AC-4, AC-6): automatic promotion capability key named automatic_promotion (cache: result_cache) over automatic_failover, SEC-10 Disable automatic promotion and result caching by default, Reserved capability tables (result_cache, automatic_promotion; enabled = true rejected), Decision: reserved key name automatic_promotion over automatic_failover

### Community 45 - "KPIs and Proposed Gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

## Knowledge Gaps
- **52 isolated node(s):** `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Protected assets and threat actors` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Agent Workflow, Documentation Index and Risks` to `Standards, Tooling and Release Qualification`, `Scope 001 Coordination Brief, Log and Authorization`, `Feature Inventory and Parity Matrix`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Operation Journal Records and Generations`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Feature Families and Planned Scopes`, `Session Pooling and Architecture Rationale`, `Promotion, Fencing and Split Brain Risks`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Protocol Engine and Proposed Stack`, `Pgpool Baseline, Competitors and Feasibility`, `Virtual Cancellation and Architecture Overview`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Failover Records and Fencing Authority`, `Security Features and Review Roles`, `Workspace Skeleton, Verify Record and Linux-only Tests`?**
  _High betweenness centrality (0.324) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Agent Workflow, Documentation Index and Risks` to `Scope 001 Coordination Brief, Log and Authorization`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Operation Journal Records and Generations`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Offline Bounds, Parser Depth and Known Limits`, `Feature Families and Planned Scopes`, `Session Pooling and Architecture Rationale`, `Promotion, Fencing and Split Brain Risks`, `Rust Technology Evaluation`, `Protocol Engine and Proposed Stack`, `Scope Status and Documentation Ownership`, `Virtual Cancellation and Architecture Overview`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Review Clarifications and check-config Usage Rules`, `Security Features and Review Roles`, `Workspace Skeleton, Verify Record and Linux-only Tests`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Why does `0001. Verification record` connect `Workspace Skeleton, Verify Record and Linux-only Tests` to `Scope 001 Coordination Brief, Log and Authorization`, `Spec 0001 Acceptance Criteria`, `Review Clarifications and check-config Usage Rules`, `Reserved Capabilities and First Build Criteria`, `Agent Workflow, Documentation Index and Risks`, `Security Features and Review Roles`, `Scope Status and Documentation Ownership`?**
  _High betweenness centrality (0.125) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope 001 Coordination Brief, Log and Authorization` be split into smaller, more focused modules?**
  _Cohesion score 0.05053651782623745 - nodes in this community are weakly interconnected._