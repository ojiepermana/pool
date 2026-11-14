# Graph Report - pool  (2026-11-14)

## Corpus Check
- 5 files · ~83,588 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1170 nodes · 3532 edges · 47 communities
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 679 edges (avg confidence: 0.85)
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
- Linux Build Blocker and Linux-only Tests
- Promotion, Fencing and Split Brain Risks
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Scope 001 Coordination Log and Graph Ownership
- Pgpool Baseline, Competitors and Feasibility
- Spec 0001 Decision, ADR and Status
- Virtual Cancellation and Architecture Overview
- Manual Failover, Fencing and Write Barrier
- Core Proxy PRD, Decision Ledger and Routing PRDs
- pool-config Validation Tests
- Session Model, Pooling Modes and Data Path
- pool-config Validation Library
- Scope 001 Review Record and Verdicts
- Scope 001 Sessions, Ticks and Wave Decisions
- check-config Integration Tests
- IoFinding and Permission Denied Path
- Leases, Cancellation and Login Routing
- Safety Invariants and Session Risks
- pool CLI Entry and Argument Parsing
- Query Cache PRD
- Spec 0001 Acceptance Criteria
- check-config Report Output
- Linux Target and VM Architecture
- Review Clarifications and check-config Usage Rules
- Failover Records and Fencing Authority
- SQL Routing PRD and Pinned Profiles
- Health Monitoring PRD and Probes
- Agent Workflow, Documentation Index and Risks
- Load Balancing and Replica Selection
- Verify Record First Run and Toolchain
- Workspace Skeleton, Verify Record and Linux-only Tests
- Second Review and Scope 002 CI Follow-up
- Spec 0001 Review Clarifications Rationale
- Column Counting Rule and Tests

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 78 edges
2. `Glossary` - 58 edges
3. `Spec 0001. POOL initial architecture` - 56 edges
4. `Feature inventory and parity matrix` - 47 edges
5. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 47 edges
6. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 44 edges
7. `Testing strategy` - 43 edges
8. `Scope: POOL, order and status index` - 43 edges
9. `Security requirements` - 39 edges
10. `assert_run()` - 38 edges

## Surprising Connections (you probably didn't know these)
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `automatic_promotion_enabled_is_refused()` --implements--> `AC-6 Automatic promotion is refused (config.unsupported_capability at automatic_promotion.enabled)`  [EXTRACTED]
  crates/pool-config/tests/validate.rs → docs/specs/0001-initial-architecture/index.md
- `enabled_true_is_refused_in_every_table_spelling()` --implements--> `AC-5 Result cache is refused (config.unsupported_capability at result_cache.enabled)`  [EXTRACTED]
  crates/pool-config/tests/validate.rs → docs/specs/0001-initial-architecture/index.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Scope 001 review findings (first pass Changes requested: Major array accepted, six Minors, Nits; closed in the fix round)** — docs_reviews_2026_11_14_scope_001_stack_and_workspace_verdict_first, docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_array_accepted, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_1_config_version_constant, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_2_parser_depth_untested, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_3_permission_denied_untested, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_4_column_test_weak, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_5_no_automated_guard, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_6_verify_record_stale, docs_reviews_2026_11_14_scope_001_stack_and_workspace_nits, docs_reviews_2026_11_14_scope_001_stack_and_workspace_finding_status [EXTRACTED 1.00]
- **Scope 001 review and fix round (review Major RawCapability, reused developer and architect sessions, table-only rule, joint merge)** — docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review, docs_scope_coordination_001_stack_and_workspace_log_decision_review_major_rawcapability, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build_reused, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update_reused, docs_scope_coordination_001_stack_and_workspace_log_decision_fix_round_joint_merge, crates_pool_config_src_lib_rawcapability_deserialize, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only [EXTRACTED 1.00]
- **Review Major array accepted and its fix (RawCapability, RawConfig hand written Deserialize, KnownField, array tests, table-only spec rule)** — docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_array_accepted, docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_suggested_fix, crates_pool_config_src_lib_rawcapability, crates_pool_config_src_lib_rawcapability_deserialize, crates_pool_config_src_lib_rawconfig_deserialize, crates_pool_config_src_lib_knownfield, crates_pool_config_tests_validate_ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding, apps_pool_tests_check_config_ac7_a_reserved_capability_written_as_an_array_is_rejected_by_the_binary, docs_specs_0001_initial_architecture_index_table_only_structures [EXTRACTED 1.00]
- **Scope 001 wave 1 ticked boxes (build notes, scaffold, verify, test, review) and their log entries** — docs_scope_coordination_001_stack_and_workspace_log_checked_build_notes, docs_scope_coordination_001_stack_and_workspace_log_checked_scaffold, docs_scope_coordination_001_stack_and_workspace_log_checked_verify, docs_scope_coordination_001_stack_and_workspace_log_checked_test, docs_scope_001_stack_and_workspace_box_build_notes, docs_scope_001_stack_and_workspace_box_scaffold, docs_scope_001_stack_and_workspace_box_verify, docs_scope_001_stack_and_workspace_box_test, docs_scope_coordination_001_stack_and_workspace_log_checked_review, docs_scope_001_stack_and_workspace_box_review [EXTRACTED 1.00]
- **Spec 0001 acceptance criteria tied to test files** — docs_specs_0001_initial_architecture_verify_ac_coverage, apps_pool_tests_check_config, crates_pool_config_tests_validate, apps_pool_src_cli, crates_pool_config_src_lib [EXTRACTED 1.00]
- **Spec 0001 first build acceptance criteria AC-1 to AC-13** — docs_specs_0001_initial_architecture_index_ac_1, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_3, docs_specs_0001_initial_architecture_index_ac_4, docs_specs_0001_initial_architecture_index_ac_5, docs_specs_0001_initial_architecture_index_ac_6, docs_specs_0001_initial_architecture_index_ac_7, docs_specs_0001_initial_architecture_index_ac_8, docs_specs_0001_initial_architecture_index_ac_9, docs_specs_0001_initial_architecture_index_ac_10, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_index_ac_12, docs_specs_0001_initial_architecture_index_ac_13 [EXTRACTED 1.00]
- **Spec 0001 build plan steps 1 to 5 (tracer bullet)** — docs_specs_0001_initial_architecture_index_build_plan_step_1, docs_specs_0001_initial_architecture_index_build_plan_step_2, docs_specs_0001_initial_architecture_index_build_plan_step_3, docs_specs_0001_initial_architecture_index_build_plan_step_4, docs_specs_0001_initial_architecture_index_build_plan_step_5 [EXTRACTED 1.00]
- **Spec 0001 full verify BLOCKED on the Linux build (verdict, AC-2, AC-11 /dev/full, F1-1, F1-2, Linux-only tests)** — docs_specs_0001_initial_architecture_verify_full_verify_verdict, docs_specs_0001_initial_architecture_verify_full_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, apps_pool_tests_check_config_ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr, docs_scope_coordination_001_stack_and_workspace_log_blocker_linux_build_not_run [EXTRACTED 1.00]
- **Spec 0001 full verify evidence against the real binary (commands, fixtures, probe, scratch crate, coverage)** — docs_specs_0001_initial_architecture_verify_full_verify_commands, docs_specs_0001_initial_architecture_verify_full_verify_nm_imports, docs_specs_0001_initial_architecture_verify_full_verify_fixture_runs, docs_specs_0001_initial_architecture_verify_full_verify_byte_probe, docs_specs_0001_initial_architecture_verify_full_verify_scratch_crate, docs_specs_0001_initial_architecture_verify_full_verify_ac_coverage, docs_specs_0001_initial_architecture_verify_full_verify_ac_pass, docs_specs_0001_initial_architecture_verify_full_verify_extra_runs [EXTRACTED 1.00]
- **Spec 0001 Linux build blocked on Docker (AC-2, F1-1, F1-2, /dev/full)** — docs_specs_0001_initial_architecture_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_linux_build, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_follow_up_start_docker, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, docs_specs_0001_initial_architecture_verify_ac_11_partial [EXTRACTED 1.00]
- **Spec 0001 clarifications from the scope 001 review (nine rationale rows and the spec text they explain)** — docs_specs_0001_initial_architecture_rationale_review_clarifications, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only, docs_specs_0001_initial_architecture_rationale_clarification_2_equals_not_utf8, docs_specs_0001_initial_architecture_rationale_clarification_3_help_version_equals, docs_specs_0001_initial_architecture_rationale_clarification_4_nesting_depth, docs_specs_0001_initial_architecture_rationale_clarification_5_permission_denied_test, docs_specs_0001_initial_architecture_rationale_clarification_6_exact_column, docs_specs_0001_initial_architecture_rationale_clarification_7_fifo_limit, docs_specs_0001_initial_architecture_rationale_clarification_8_ci_follow_up, docs_specs_0001_initial_architecture_rationale_clarification_9_status_wording, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_index_equals_not_utf8_unexpected_argument, docs_specs_0001_initial_architecture_index_option_name_before_equals, docs_specs_0001_initial_architecture_index_nesting_depth_syntax_no_location, docs_specs_0001_initial_architecture_index_permission_denied_test, docs_specs_0001_initial_architecture_index_exact_column_test, docs_specs_0001_initial_architecture_index_fifo_known_limit, docs_specs_0001_initial_architecture_index_follow_up_ci_checks_ac12_ac13, docs_specs_0001_initial_architecture_index_status_follows_scope_feature [EXTRACTED 1.00]
- **Spec 0001 second run still BLOCKED on the Linux build (verdict, AC-2, AC-11 /dev/full, Linux only tests, F1-2)** — docs_specs_0001_initial_architecture_verify_second_run_verdict, docs_specs_0001_initial_architecture_verify_second_run_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_verify_second_run_linux_only_tests, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_scope_coordination_001_stack_and_workspace_log_blocker_linux_build_not_run, docs_scope_coordination_001_stack_and_workspace_log_decision_feature_1_not_done_before_linux_build [EXTRACTED 1.00]
- **Spec 0001 full verify second run on the merged tree (revision, 134 tests, commands, clarified rule checks, AC coverage)** — docs_specs_0001_initial_architecture_verify_second_run, docs_specs_0001_initial_architecture_verify_second_run_revision, docs_specs_0001_initial_architecture_verify_second_run_test_counts, docs_specs_0001_initial_architecture_verify_second_run_commands, docs_specs_0001_initial_architecture_verify_second_run_clarified_rules, docs_specs_0001_initial_architecture_verify_second_run_ac_coverage, docs_specs_0001_initial_architecture_verify_second_run_ac_pass, docs_specs_0001_initial_architecture_verify_second_run_rule_array_reserved_table, docs_specs_0001_initial_architecture_verify_second_run_rule_long_canary_array, docs_specs_0001_initial_architecture_verify_second_run_rule_schema_step_ends_run [EXTRACTED 1.00]
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

## Communities (47 total, 0 thin omitted)

### Community 0 - "Standards, Tooling and Release Qualification"
Cohesion: 0.13
Nodes (29): Tests and Evidence Standard, Milestones, V1 Release Gate, R-16 Dependency drift or abandoned crate, criterion 0.8.2 (microbenchmarks), proptest 1.12.0 (state and framing properties), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply), 11. Release qualification harness (planned) (+21 more)

### Community 1 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.14
Nodes (26): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first) (+18 more)

### Community 2 - "Feature Inventory and Parity Matrix"
Cohesion: 0.07
Nodes (43): Invariant: fail closed on unknown role or unsafe routing, Invariant: keep writes on the verified primary, Planned Cargo Check Commands, Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-024 Transaction Mode (refused, session pooling only) (+35 more)

### Community 3 - "Administration, Cluster and Health PRDs"
Cohesion: 0.12
Nodes (25): Security and Review Requirements, Request ID (idempotency key), Axum REST administration API plus CLI, R-18 Journal disk loss or replay, Cluster management PRD, Drain, detach and attach node operations, Two observed primaries close write admission, Generation checked journaled mutations (+17 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.38
Nodes (14): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+6 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.07
Nodes (33): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Support Profile, Proposed stopping rules, PostgreSQL protocol analysis (+25 more)

### Community 6 - "Operation Journal Records and Generations"
Cohesion: 0.30
Nodes (12): Control plane, Durable approved writer authority, ApprovedTopology record, AuditEvent record, FenceReceipt record, Observation record, Operation record, Local SQLite operation journal (single writer) (+4 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.19
Nodes (17): Finding (stable rejection code), Five new nits in the fix range: deny_unknown_fields phrase at index.md:431, config_version start value at lib.rs:354, 1_048_577 literal at main.rs:89, compile_fail breadth at lib.rs:28, stale CLAUDE.md and AGENTS.md lines, Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Sub session arc-f1-update continued (architect update: deny_unknown_fields phrase in step 4, one syntax or schema finding chosen by the parser; active since 20:15), Decision: of two schema errors the one with the alphabetically first key is reported, not the first in the file; spec states the choice belongs to the parser (mode auto, follows the code); runner up sort by file position; sent to arc-f1-update, config_version key (required, supports exactly 1), FindingCode (config.* codes), Findings (stable finding codes) (+9 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.12
Nodes (30): Dependency and Configuration Change Rules, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), F-066 Topology reconciliation (durable writer/epoch/quarantine), Glossary, Approved Writer, Backend Login (+22 more)

### Community 9 - "Delivery Phases, Milestones and HA Features"
Cohesion: 0.16
Nodes (15): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-21 Proxmox host correlated failure, High availability PRD, Endpoint failover via external load balancing or controlled VIP, External coordination adapter, Query routing on local immutable snapshots (+7 more)

### Community 10 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.29
Nodes (11): ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Rules made clearer in the review, each checked on the binary (17 rows, all PASS), Rule check: 1 MiB files of brackets and nested inline tables end with one config.syntax, no crash or exit 101, Rule check: a dotted key with 81 parts is config.syntax with no location; 80 parts is inside the parser limit (+3 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.14
Nodes (25): Protocol Features (F-001 to F-022), Walking Slice (glossary term), Core MVP, Walking Slice Milestone, R-24 Walking slice exceeds its estimate, Roadmap, Phase 0 Research and architecture, Phase 1 Foundation (+17 more)

### Community 12 - "Linux Build Blocker and Linux-only Tests"
Cohesion: 0.21
Nodes (25): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Box ticked: Verify it (/check verify stack and workspace), Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user), Open blocker: Linux build evidence blocked while the Docker daemon is off (+17 more)

### Community 13 - "Promotion, Fencing and Split Brain Risks"
Cohesion: 0.18
Nodes (18): Invariant: promotion requires a reviewed operation and independently verified fencing, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), Failover features F-067 to F-075, R-08 Split brain, R-20 Fencing provider unavailable (+10 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.09
Nodes (34): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy (+26 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.16
Nodes (19): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table (+11 more)

### Community 16 - "Scope 001 Coordination Log and Graph Ownership"
Cohesion: 0.17
Nodes (20): Graph update ownership: sub sessions never touch graphify-out, lead refreshes it in the merge commit, Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only), Log koordinasi: 001-stack-and-workspace (coordination log), Sub session arc-f1-update (architect update: record build notes in spec 0001; merged to bed9fad, spec commit a6e0042), Sub session arc-f1-update reused (architect update: nine spec 0001 clarifications from the review in index.md and a new rationale.md table, commit 5f1e995, no AC added or changed; merged with dev-f1-build), Open blocker: untracked .graphifyignore and graphify-out/cache/last_query_stamp block a clean closing push, Checked in wave 1: F1 build notes recorded (arc-f1-update done, commit a6e0042, merged to bed9fad; AC-1..AC-13 and 5 step Build plan present, docs only), Decision: 'Record the build notes' runs as an architect (update) item on spec 0001 (+12 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (46): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+38 more)

### Community 18 - "Spec 0001 Decision, ADR and Status"
Cohesion: 0.31
Nodes (10): ADR 0001. Initial architecture, Decision: Build plan step 1 sets spec Status to In Progress; the stale 'lifecycle status remains Proposed' wording is tidied at /sync, not by sub sessions, Spec 0001. POOL initial architecture, Follow-up 1: foundation and decision ledger ratified; the build of scope 001 was requested on 2026-11-14; every later scope stays queued until requested, Modular daemon with independent CLI decision, Shared process failure domain, Decision status sentence: ratification settles the content; the Status line follows scope feature 001 per the status lifecycle, Clarification 9: Decision status sentence and Follow-up 1 reworded to hold in every status; runner up leave to the lead at close (+2 more)

### Community 19 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.21
Nodes (14): Virtual Cancellation Model, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), No automatic retry after dispatch, Architecture overview, Failure behavior (no transparent failover, no implicit replay), Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), F-018 CancelRequest and BackendKeyData (+6 more)

### Community 20 - "Manual Failover, Fencing and Write Barrier"
Cohesion: 0.17
Nodes (15): F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Fence Receipt, Write Barrier, Manual failover first; automatic failover and cache disabled, R-09 Failover loses acknowledged commits, Approved writer, epoch and quarantine authority, Manual failover PRD, Durable intent before fencing and reconciliation of unknown outcomes (+7 more)

### Community 21 - "Core Proxy PRD, Decision Ledger and Routing PRDs"
Cohesion: 0.14
Nodes (14): Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, One daemon with separate modules plus separate CLI, pg_query parser with PostgreSQL 18 grammar, Prometheus metrics and tracing logs (+6 more)

### Community 22 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (57): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+49 more)

### Community 23 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.11
Nodes (22): Session Model (pinned backend lease), Connection pooling, Pool key (pool identity), Pool limits and operations, Statement pooling (not planned for V1), Transaction pooling (deferred), Data plane, Frontend owner task (+14 more)

### Community 24 - "pool-config Validation Library"
Cohesion: 0.07
Nodes (40): capability_state(), CapabilityState, error_location(), Finding, FindingCode, KnownField, Location, position() (+32 more)

### Community 25 - "Scope 001 Review Record and Verdicts"
Cohesion: 0.22
Nodes (15): docs/reviews section of the documentation index (one review row), Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), Review summary: two package workspace and offline pool check-config follow spec 0001; output from constants only, bounded read, enabled = true refused in every spelling, Review test coverage: what is covered well and the gaps (array starting with a boolean, permission denied arm, no span arm, column through validate, AC-8/12/13 guards, --format=<bytes>), Review method: two passes on the same reviewer model; the second pass checked each finding and found the Major by running the binary, Review verdict: Approve with nits after the fix round at 4ebcf93 (first pass at 02bdcbe was Changes requested), First review verdict at 02bdcbe: Changes requested (0 Blocker, 1 Major, 6 Minor, 11 Nit) (+7 more)

### Community 26 - "Scope 001 Sessions, Ticks and Wave Decisions"
Cohesion: 0.23
Nodes (15): Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Standby first turn of a new session (STATUS: standby until lead sends 'mulai'), Checked in wave 1: F1 Scaffold from the decision (dev-f1-build done and chk-f1-verify report, merged 5f44d88; manual F1-1, F1-2 NOT_RUN), Checked in wave 1: F1 Test it (tst-f1-test commits 5ad687d, fae4bc6; 127 tests green on main; F1-2 NOT_RUN for two Linux-only tests), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test), Decision: after the fix merge chk-f1-verify reverifies into verify.md, chk-f1-review confirms the fix diff; stale CLAUDE.md and AGENTS.md tidied by /sync at close; spec 0001 Follow-up 11 copied to scope 002 at close (+7 more)

### Community 27 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (86): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+78 more)

### Community 28 - "IoFinding and Permission Denied Path"
Cohesion: 0.24
Nodes (10): IoFinding, Self, Minor 3: the io.permission_denied arm of IoFinding::from_kind (report.rs:30) has no test, although the workstation suite runs as a user that is not root, io.permission_denied test under cfg(unix): file of mode 000; returns as passed when the process can still open it (root in the container), else checks exit 3 and exact output in both formats, Tests and who owns them, Clarification 5: a self-skipping cfg(unix) mode 000 test for io.permission_denied; runner up keep it untested, Extra runs beyond the spec: mode 000 file exits 3 io.permission_denied; closed stdout or stderr pipe exits 3 with no output and no panic (not the /dev/full case), io.permission_denied has no test, as the spec says (+2 more)

### Community 29 - "Leases, Cancellation and Login Routing"
Cohesion: 0.21
Nodes (12): Retry only before initial lease and ReadyForQuery exposure, Authoritative barrier across every write path, Independent review resolutions (seven approved fixes), Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Routing decision: primary default, explicit read only session profiles, Cancellation forwarded only through the live lease owner, CancellationEntry (+4 more)

### Community 30 - "Safety Invariants and Session Risks"
Cohesion: 0.05
Nodes (54): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust (+46 more)

### Community 31 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.13
Nodes (27): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), check_config() (+19 more)

### Community 32 - "Query Cache PRD"
Cohesion: 0.22
Nodes (13): V2 Milestone, Phase 6 Advanced features, Connection pooling PRD, Credential and configuration generation retirement, PoolKey, PoolState, release(lease, terminal_state), Reset and validate before reuse; close uncertain state (+5 more)

### Community 33 - "Spec 0001 Acceptance Criteria"
Cohesion: 0.08
Nodes (91): ac12_dev_null_is_an_empty_file_that_misses_config_version(), Session pooling (one frontend owns one backend lease), Login time backend selection on pinned lease, Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Automatic Promotion (reserved and off), Output Version (output_version field), Reserved Capability, Result Cache (reserved and off) (+83 more)

### Community 34 - "check-config Report Output"
Cohesion: 0.31
Nodes (9): JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, write_human(), write_json() (+1 more)

### Community 35 - "Linux Target and VM Architecture"
Cohesion: 0.38
Nodes (11): Open blocker: proxy and PostgreSQL VM architecture (x86_64 or ARM) unconfirmed for 0001 AC-2; x86_64 target inferred (F1-N1), Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; awaiting user VM architecture, User value F1-N1: proxy and PostgreSQL VM architecture (x86_64 or ARM); x86_64-unknown-linux-gnu inferred from Proxmox VE, Values from the user (F1-N1: VM architecture), Follow-up: confirm the PostgreSQL and proxy VMs are x86_64, Linux build (container command and workstation checks), rust:1.100.0-slim-bookworm builder image (pinned by digest), Toolchain and Linux target (+3 more)

### Community 36 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.38
Nodes (10): Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, Value sourcing (where each output value comes from), For /check review: notes handed to the review, Review note: --config=<bytes that are not UTF-8> is unexpected argument (as written in the spec, may surprise an operator), Review note: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed value (lib.rs line 267); breaks silently if a second version is supported (+2 more)

### Community 37 - "Failover Records and Fencing Authority"
Cohesion: 0.21
Nodes (15): Deployment reference, Initial four VM lab, Future multi proxy deployment, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy (+7 more)

### Community 38 - "SQL Routing PRD and Pinned Profiles"
Cohesion: 0.36
Nodes (9): Policy Version, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+1 more)

### Community 39 - "Health Monitoring PRD and Probes"
Cohesion: 0.14
Nodes (22): F-060 Network partitions (health changes eligibility only), F-101 TLS and mTLS verification, Health features F-054 to F-060, Routing Preview, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role (+14 more)

### Community 40 - "Agent Workflow, Documentation Index and Risks"
Cohesion: 0.10
Nodes (53): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach (+45 more)

### Community 41 - "Load Balancing and Replica Selection"
Cohesion: 0.36
Nodes (9): No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state), Replica eligibility filter (role, identity, freshness, maintenance, lag), Primary fallback disabled in V1, ReplicaPolicy, SelectorState (+1 more)

### Community 42 - "Verify Record First Run and Toolchain"
Cohesion: 0.13
Nodes (23): Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, Sub session chk-f1-verify continued (reverify on the merged tree after the review fixes, closes Minor 6; report blocked, commit cf928e7: revision b6ce165, 11 AC PASS, 134 tests, 398 probe checks; AC-2 and /dev/full clause BLOCKED), rust-toolchain.toml (channel 1.100.0, rustfmt and clippy), Spec 0001 status: In Progress (was Proposed; foundation build under way), Spec 0001 verify.md (workstation and Linux evidence record), 0001. Verification record, Full verify: /check verify stack and workspace (checker chk-f1-verify, 2026-11-14, rerun from its own checkout against the real pool binary), Binary under test: target/release/pool from cargo build --locked --release (Mach-O arm64; macOS checksum, not the AC-2 record) (+15 more)

### Community 43 - "Workspace Skeleton, Verify Record and Linux-only Tests"
Cohesion: 0.33
Nodes (9): Decision: Build plan steps 4 and 5 make the developer write verify.md and its docs/index.md entry; chk-f1-verify runs and ticks it, Decision: developer AC readings passed to chk-f1-verify (--help=x and -h=x as help, JSON field order on the raw line, /dev/full test Linux only, milestone number 3), Build plan (tracer bullet, five steps), Build plan step 4: Workstation evidence, Build plan step 5: Linux evidence, Milestone 3: Scaffold from the decision (build plan steps 1 to 5, dev-f1-build, 2026-11-14), Reading: the /dev/full test is compiled only on Linux (cfg target_os linux), check_config.rs line 525, not run anywhere yet, Reading: milestone 3 fits (Scaffold from the decision is the third box of scope 001) (+1 more)

### Community 44 - "Second Review and Scope 002 CI Follow-up"
Cohesion: 0.25
Nodes (8): Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, Second review run at 4ebcf93: fmt and clippy exit 0, tests 3 + 66 + 3 + 60 + 2 passed, 18 lock packages, AC-12 grep empty; Linux build not run, Second review, fix round at 4ebcf93 (range 02bdcbe..4ebcf93; code a705659, 5a99563, b18b288, 453ae73; spec 5f1e995; same reviewer session), Spec text against the code: the nine clarifications in index.md and the rationale table match the fixed binary; AC-1 to AC-13 unchanged, What is left after the second review: Minor 6 verify record on the merged tree; AC-12 and AC-13 guards wait for scope 002 (Follow-up 11), Follow-up 11: scope 002 adds automatic CI checks for the hand-checked parts of AC-12 (Cargo.lock package set, grep) and AC-13 (four absent paths, lint inheritance); from the scope 001 review, Follow-up: scope 002 runs the Linux build on a Linux runner without emulation, Clarification 8: missing automatic guards for AC-12 and AC-13 become a scope 002 Follow-up; runner up shell checks in cargo test now

### Community 45 - "Spec 0001 Review Clarifications Rationale"
Cohesion: 0.53
Nodes (6): Clarification 2: --format=<bytes> and --config=<bytes> are unexpected argument; runner up byte level split, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check, Clarification 6: exact column checked for one unambiguous made input with a multi-byte character, others keep at least 1; runner up exact columns everywhere, Clarification evidence: review record docs/reviews/2026-11-14-scope-001-stack-and-workspace.md (rows 1 to 7); debug pool build at 07f7994 rerun for rows 2, 3, 4, 6 (81-part key config.syntax null location, 80-part key config.schema, x = "éé" junk at column 10), Decision: standard library argument parsing over clap, Clarifications from the scope 001 review (nine biasa decisions taken by the lead under auto mode, following the reviewer; no acceptance criterion added or changed)

### Community 46 - "Column Counting Rule and Tests"
Cohesion: 0.70
Nodes (5): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way), Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12), Rule check: the column counts characters (x = "éé" junk is line 2, column 10; bytes would give 12)

## Knowledge Gaps
- **53 isolated node(s):** `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)` (+48 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Agent Workflow, Documentation Index and Risks` to `Standards, Tooling and Release Qualification`, `Feature Inventory and Parity Matrix`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Operation Journal Records and Generations`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Feature Families and Planned Scopes`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Pgpool Baseline, Competitors and Feasibility`, `Spec 0001 Decision, ADR and Status`, `Virtual Cancellation and Architecture Overview`, `Manual Failover, Fencing and Write Barrier`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Scope 001 Review Record and Verdicts`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Failover Records and Fencing Authority`, `SQL Routing PRD and Pinned Profiles`, `Health Monitoring PRD and Probes`, `Load Balancing and Replica Selection`, `Verify Record First Run and Toolchain`, `Workspace Skeleton, Verify Record and Linux-only Tests`?**
  _High betweenness centrality (0.294) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Spec 0001 Decision, ADR and Status` to `Scope 001 Coordination Brief, Log and Authorization`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Operation Journal Records and Generations`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Feature Families and Planned Scopes`, `Linux Build Blocker and Linux-only Tests`, `Rust Technology Evaluation`, `Scope 001 Coordination Log and Graph Ownership`, `Virtual Cancellation and Architecture Overview`, `Manual Failover, Fencing and Write Barrier`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Scope 001 Review Record and Verdicts`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Linux Target and VM Architecture`, `SQL Routing PRD and Pinned Profiles`, `Health Monitoring PRD and Probes`, `Agent Workflow, Documentation Index and Risks`, `Load Balancing and Replica Selection`, `Verify Record First Run and Toolchain`, `Workspace Skeleton, Verify Record and Linux-only Tests`, `Second Review and Scope 002 CI Follow-up`?**
  _High betweenness centrality (0.152) - this node is a cross-community bridge._
- **Why does `Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5` connect `Scope 001 Review Record and Verdicts` to `Spec 0001 Acceptance Criteria`, `Review Clarifications and check-config Usage Rules`, `Agent Workflow, Documentation Index and Risks`, `Offline Bounds, Parser Depth and Known Limits`, `Verify Record First Run and Toolchain`, `Second Review and Scope 002 CI Follow-up`, `Column Counting Rule and Tests`, `Spec 0001 Decision, ADR and Status`, `pool-config Validation Library`, `IoFinding and Permission Denied Path`, `pool CLI Entry and Argument Parsing`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165` to the rest of the system?**
  _53 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Standards, Tooling and Release Qualification` be split into smaller, more focused modules?**
  _Cohesion score 0.12807881773399016 - nodes in this community are weakly interconnected._
- **Should `Scope 001 Coordination Brief, Log and Authorization` be split into smaller, more focused modules?**
  _Cohesion score 0.14461538461538462 - nodes in this community are weakly interconnected._