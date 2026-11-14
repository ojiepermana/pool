# Graph Report - pool  (2026-11-14)

## Corpus Check
- 84 files · ~85,230 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1217 nodes · 3759 edges · 52 communities
- Extraction: 80% EXTRACTED · 20% INFERRED · 0% AMBIGUOUS · INFERRED: 737 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Standards, Tooling and Release Qualification
- Scope 001 Coordination Brief, Log and Authorization
- Feature Inventory and Parity Matrix
- Administration, Cluster and Health PRDs
- Architecture Spec and Repository Layout
- Protocol Contract and Driver Compatibility
- Operation Journal Records and Generations
- Documentation Index, Handoffs and Readiness
- Glossary and Routing Profiles
- Delivery Phases, Milestones and HA Features
- Offline Bounds, Parser Depth and Known Limits
- Feature Families and Planned Scopes
- Scaffold Step and dev-f1-build Gate
- Finding Codes and One Finding Rule
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Scope 001 Coordination Brief and Sub Sessions
- Pgpool Baseline, Competitors and Feasibility
- Project Guidance, Changelog and Status Lifecycle
- Virtual Cancellation and Architecture Overview
- Manual Failover, Fencing and Write Barrier
- Core Proxy PRD, Decision Ledger and Routing PRDs
- pool-config Validation Tests
- Session Model, Pooling Modes and Data Path
- pool-config Validation Library
- Scope 001 Review Record and Verdicts
- Delegated Spec Decisions and Linux Target
- check-config Integration Tests
- IoFinding and Permission Denied Path
- Leases, Cancellation and Login Routing
- Safety Invariants and Session Risks
- pool CLI Entry and Argument Parsing
- Query Cache PRD
- Spec 0001 Acceptance Criteria
- Security Requirements, HA Features and V1.5
- Linux Build Evidence, AC-2 and VM Architecture
- Review Clarifications and check-config Usage Rules
- Failover Records and Fencing Authority
- SQL Routing PRD and Pinned Profiles
- Authentication, Security Features and Health PRD
- Slices, Milestones and Readiness Gates
- Load Balancing and Replica Selection
- Verify Record First Run and Toolchain
- Reserved Capabilities and Secret Safety
- Column Counting Rule and Tests
- Hand Written Deserialize and Known Fields
- Exit Status and Output Convention
- Reserved Capability Table Fix
- Proposed Stack and Deferred Features
- Review Rule Checks and Whole Config Validation
- ADR 0001 and Modular Daemon Decision
- Workspace Layout, Bounded Read and FIFO Limit

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 81 edges
2. `Glossary` - 58 edges
3. `Spec 0001. POOL initial architecture` - 57 edges
4. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 55 edges
5. `Feature inventory and parity matrix` - 47 edges
6. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 46 edges
7. `Scope: POOL, order and status index` - 46 edges
8. `Testing strategy` - 43 edges
9. `1. Stack and workspace (in-progress)` - 43 edges
10. `Security requirements` - 39 edges

## Surprising Connections (you probably didn't know these)
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Probes of the hand written Deserialize (37 made files at 4ebcf93: key given twice, unknown keys, enabled not boolean, capability not a table, other spellings, config_version, canary, valid files)` --references--> `error_location()`  [INFERRED]
  docs/reviews/2026-11-14-scope-001-stack-and-workspace.md → crates/pool-config/src/lib.rs
- `Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12)` --references--> `error_location()`  [INFERRED]
  docs/specs/0001-initial-architecture/index.md → crates/pool-config/src/lib.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **CHANGELOG Unreleased Added entries for the scope 001 workspace slice (workspace, check-config, config contract, finding codes, JSON, exit status, help and version, fixtures and tests, docs, conventions)** — changelog_unreleased_added, changelog_added_workspace, changelog_added_check_config, changelog_added_config_contract, changelog_added_finding_codes, changelog_added_json_report, changelog_added_exit_status, changelog_added_help_version, changelog_added_fixtures_tests, changelog_added_docs, changelog_added_shared_conventions [EXTRACTED 1.00]
- **Scope 001 Document it and /sync (CHANGELOG.md, real commands in CLAUDE.md, CONTRIBUTING.md and README.md per Follow-up 8, AGENTS.md, strategy and scope index corrections)** — docs_scope_coordination_001_stack_and_workspace_log_decision_document_it_by_lead, docs_scope_coordination_001_stack_and_workspace_log_decision_sync_branch, changelog, docs_specs_0001_initial_architecture_index_follow_up_document_real_commands, claude_planned_commands, contributing_checks, readme_build_and_check, agents_workspace_two_packages_no_ci, docs_testing_strategy_proposed_cargo_checks, docs_scope_index_feature_1_state [EXTRACTED 1.00]
- **Linux target confirmation for 0001 AC-2 (delegated x86_64 choice, blocker, user value F1-N1, F1-2 build)** — docs_scope_coordination_001_stack_and_workspace_log_delegated_linux_target, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed, docs_scope_coordination_001_stack_and_workspace_manual_f1_n1_vm_architecture, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_x86_64_linux_target [EXTRACTED 1.00]
- **Scope 001 review findings (first pass Changes requested: Major array accepted, six Minors, Nits; closed in the fix round)** — docs_reviews_2026_11_14_scope_001_stack_and_workspace_verdict_first, docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_array_accepted, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_1_config_version_constant, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_2_parser_depth_untested, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_3_permission_denied_untested, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_4_column_test_weak, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_5_no_automated_guard, docs_reviews_2026_11_14_scope_001_stack_and_workspace_minor_6_verify_record_stale, docs_reviews_2026_11_14_scope_001_stack_and_workspace_nits, docs_reviews_2026_11_14_scope_001_stack_and_workspace_finding_status [EXTRACTED 1.00]
- **Scope 001 review and fix round (review Major RawCapability, reused developer and architect sessions, table-only rule, joint merge)** — docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review, docs_scope_coordination_001_stack_and_workspace_log_decision_review_major_rawcapability, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build_reused, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update_reused, docs_scope_coordination_001_stack_and_workspace_log_decision_fix_round_joint_merge, crates_pool_config_src_lib_rawcapability_deserialize, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only [EXTRACTED 1.00]
- **Review Major array accepted and its fix (RawCapability, RawConfig hand written Deserialize, KnownField, array tests, table-only spec rule)** — docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_array_accepted, docs_reviews_2026_11_14_scope_001_stack_and_workspace_major_suggested_fix, crates_pool_config_src_lib_rawcapability, crates_pool_config_src_lib_rawcapability_deserialize, crates_pool_config_src_lib_rawconfig_deserialize, crates_pool_config_src_lib_knownfield, crates_pool_config_tests_validate_ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding, apps_pool_tests_check_config_ac7_a_reserved_capability_written_as_an_array_is_rejected_by_the_binary, docs_specs_0001_initial_architecture_index_table_only_structures [EXTRACTED 1.00]
- **Scope 001 wave 1 ticked boxes (build notes, scaffold, verify, test, review, document) and their log entries** — docs_scope_coordination_001_stack_and_workspace_log_checked_build_notes, docs_scope_coordination_001_stack_and_workspace_log_checked_scaffold, docs_scope_coordination_001_stack_and_workspace_log_checked_verify, docs_scope_coordination_001_stack_and_workspace_log_checked_test, docs_scope_001_stack_and_workspace_box_build_notes, docs_scope_001_stack_and_workspace_box_scaffold, docs_scope_001_stack_and_workspace_box_verify, docs_scope_001_stack_and_workspace_box_test, docs_scope_coordination_001_stack_and_workspace_log_checked_review, docs_scope_001_stack_and_workspace_box_review, docs_scope_coordination_001_stack_and_workspace_log_checked_document, docs_scope_001_stack_and_workspace_box_document [EXTRACTED 1.00]
- **Deferred features outside the current release queue** — docs_scope_012_multiple_proxy_authority, docs_scope_013_checked_online_recovery, docs_scope_014_advanced_pooling_and_routing, docs_scope_015_optional_result_cache, docs_scope_016_additional_replication_modes, docs_scope_017_administrative_dashboard [EXTRACTED 1.00]
- **Foundation phase features (stack, standards, data model)** — docs_scope_001_stack_and_workspace, docs_scope_002_standards_and_tooling, docs_scope_003_control_and_session_data_model [EXTRACTED 1.00]
- **V1 tracer bullet vertical slices 1 to 7 plus V1 gate** — docs_scope_004_one_real_primary_path, docs_scope_005_clean_session_reuse, docs_scope_006_protocol_breadth_and_cancellation, docs_scope_007_restricted_replica_sessions, docs_scope_008_health_diagnostics_and_drain, docs_scope_009_safe_topology_administration, docs_scope_010_fenced_manual_promotion, docs_scope_011_release_qualification_harness [EXTRACTED 1.00]
- **Spec 0001 acceptance criteria tied to test files** — docs_specs_0001_initial_architecture_verify_ac_coverage, apps_pool_tests_check_config, crates_pool_config_tests_validate, apps_pool_src_cli, crates_pool_config_src_lib [EXTRACTED 1.00]
- **Spec 0001 first build acceptance criteria AC-1 to AC-13** — docs_specs_0001_initial_architecture_index_ac_1, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_3, docs_specs_0001_initial_architecture_index_ac_4, docs_specs_0001_initial_architecture_index_ac_5, docs_specs_0001_initial_architecture_index_ac_6, docs_specs_0001_initial_architecture_index_ac_7, docs_specs_0001_initial_architecture_index_ac_8, docs_specs_0001_initial_architecture_index_ac_9, docs_specs_0001_initial_architecture_index_ac_10, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_index_ac_12, docs_specs_0001_initial_architecture_index_ac_13 [EXTRACTED 1.00]
- **Spec 0001 build plan steps 1 to 5 (tracer bullet)** — docs_specs_0001_initial_architecture_index_build_plan_step_1, docs_specs_0001_initial_architecture_index_build_plan_step_2, docs_specs_0001_initial_architecture_index_build_plan_step_3, docs_specs_0001_initial_architecture_index_build_plan_step_4, docs_specs_0001_initial_architecture_index_build_plan_step_5 [EXTRACTED 1.00]
- **Spec 0001 full verify BLOCKED on the Linux build (verdict, AC-2, AC-11 /dev/full, F1-1, F1-2, Linux-only tests)** — docs_specs_0001_initial_architecture_verify_full_verify_verdict, docs_specs_0001_initial_architecture_verify_full_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, apps_pool_tests_check_config_ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr, docs_scope_coordination_001_stack_and_workspace_log_blocker_linux_build_not_run [EXTRACTED 1.00]
- **Spec 0001 full verify evidence against the real binary (commands, fixtures, probe, scratch crate, coverage)** — docs_specs_0001_initial_architecture_verify_full_verify_commands, docs_specs_0001_initial_architecture_verify_full_verify_nm_imports, docs_specs_0001_initial_architecture_verify_full_verify_fixture_runs, docs_specs_0001_initial_architecture_verify_full_verify_byte_probe, docs_specs_0001_initial_architecture_verify_full_verify_scratch_crate, docs_specs_0001_initial_architecture_verify_full_verify_ac_coverage, docs_specs_0001_initial_architecture_verify_full_verify_ac_pass, docs_specs_0001_initial_architecture_verify_full_verify_extra_runs [EXTRACTED 1.00]
- **Spec 0001 Linux build blocked on Docker (AC-2, F1-1, F1-2, /dev/full)** — docs_specs_0001_initial_architecture_verify_linux_build_blocked, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_linux_build, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_specs_0001_initial_architecture_index_follow_up_start_docker, apps_pool_tests_check_config_a_failed_write_exits_3_without_stderr, docs_specs_0001_initial_architecture_verify_ac_11_partial [EXTRACTED 1.00]
- **Spec 0001 clarifications from the scope 001 review and the second verification pass (ten rationale rows and the spec text they explain)** — docs_specs_0001_initial_architecture_rationale_review_clarifications, docs_specs_0001_initial_architecture_rationale_clarification_1_table_only, docs_specs_0001_initial_architecture_rationale_clarification_2_equals_not_utf8, docs_specs_0001_initial_architecture_rationale_clarification_3_help_version_equals, docs_specs_0001_initial_architecture_rationale_clarification_4_nesting_depth, docs_specs_0001_initial_architecture_rationale_clarification_5_permission_denied_test, docs_specs_0001_initial_architecture_rationale_clarification_6_exact_column, docs_specs_0001_initial_architecture_rationale_clarification_7_fifo_limit, docs_specs_0001_initial_architecture_rationale_clarification_8_ci_follow_up, docs_specs_0001_initial_architecture_rationale_clarification_9_status_wording, docs_specs_0001_initial_architecture_index_table_only_structures, docs_specs_0001_initial_architecture_index_equals_not_utf8_unexpected_argument, docs_specs_0001_initial_architecture_index_option_name_before_equals, docs_specs_0001_initial_architecture_index_nesting_depth_syntax_no_location, docs_specs_0001_initial_architecture_index_permission_denied_test, docs_specs_0001_initial_architecture_index_exact_column_test, docs_specs_0001_initial_architecture_index_fifo_known_limit, docs_specs_0001_initial_architecture_index_follow_up_ci_checks_ac12_ac13, docs_specs_0001_initial_architecture_index_status_follows_scope_feature, docs_specs_0001_initial_architecture_rationale_clarification_10_one_finding_parser_choice, docs_specs_0001_initial_architecture_index_one_finding_parser_choice [EXTRACTED 1.00]
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
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (52 total, 0 thin omitted)

### Community 0 - "Standards, Tooling and Release Qualification"
Cohesion: 0.16
Nodes (23): User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested, Added: three shared conventions in the engineering standards (configuration keys, exit status with command output, log event fields with redaction), Feature 1 is built and stays in progress until its Linux build evidence exists; next queued feature is 2 (standards and tooling), then the first walking slice, No CI yet (arrives with scope 002); Linux container build per spec 0001; later property, PostgreSQL integration and compatibility/fuzz/chaos tests; do not invent passing results, Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Git and Release Standards, Log event fields and redaction convention (ts, level, event; redaction by type), Shared Conventions Table (+15 more)

### Community 1 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.14
Nodes (30): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first), Always forbidden paths for sub sessions (docs/scope, docs/prd, AGENTS.md, CLAUDE.md, graphify-out, .claude, spec content outside ownership), Role files written for another project (Stackly references do not apply) (+22 more)

### Community 2 - "Feature Inventory and Parity Matrix"
Cohesion: 0.08
Nodes (36): Invariant: health failure does not authorize promotion, Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-060 Network partitions (health changes eligibility only), F-065 Primary Identification (+28 more)

### Community 3 - "Administration, Cluster and Health PRDs"
Cohesion: 0.16
Nodes (18): Axum REST administration API plus CLI, pool check-config, GET /v1/nodes node status route, Cluster management PRD, Drain, detach and attach node operations, Two observed primaries close write admission, Generation checked journaled mutations, Separation of observed health/role from maintenance and desired membership (+10 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.42
Nodes (13): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+5 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.10
Nodes (25): Backend Login, Support Profile, PostgreSQL protocol analysis, Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming, Extended Query and pipelining behavior, Independent backend credentials per mapped login identity, Proposed protocol limits (64 KiB startup, 16 MiB message) (+17 more)

### Community 6 - "Operation Journal Records and Generations"
Cohesion: 0.25
Nodes (14): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Control plane, ApprovedTopology record, AuditEvent record (+6 more)

### Community 7 - "Documentation Index, Handoffs and Readiness"
Cohesion: 0.13
Nodes (40): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach (+32 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.15
Nodes (24): Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Approved Writer, Credential Generation, Detach, Drain (+16 more)

### Community 9 - "Delivery Phases, Milestones and HA Features"
Cohesion: 0.36
Nodes (8): High availability PRD, ControllerEpoch, Endpoint failover via external load balancing or controlled VIP, Established consensus for one operation authority, External coordination adapter, Query routing on local immutable snapshots, ProxyMember, TopologyRecord

### Community 10 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.28
Nodes (13): ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Clarification 2: --format=<bytes> and --config=<bytes> are unexpected argument; runner up byte level split, Clarification 3: --help=x/-h=x after check-config print help, --version/-V there are unknown option, all four =x forms first are unknown command; runner up usage error for =x, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check (+5 more)

### Community 11 - "Feature Families and Planned Scopes"
Cohesion: 0.13
Nodes (27): Protocol Features (F-001 to F-022), Core MVP, Routing Preview, Walking Slice Milestone, R-01 Protocol incompatibility, R-24 Walking slice exceeds its estimate, Roadmap, Phase 0 Research and architecture (+19 more)

### Community 12 - "Scaffold Step and dev-f1-build Gate"
Cohesion: 0.25
Nodes (11): Box ticked: Scaffold from the decision (/develop stack and workspace), Checked in wave 1: F1 Scaffold from the decision (dev-f1-build done and chk-f1-verify report, merged 5f44d88; manual F1-1, F1-2 NOT_RUN), Decision: dev-f1-build branch passed the gate on the staged merge (Rust 1.100.0: fmt, clippy, 31 tests, four fixture commands, Cargo.lock 18 packages, AC-12 grep empty); merged as 5f44d88, Decision: Build plan steps 4 and 5 make the developer write verify.md and its docs/index.md entry; chk-f1-verify runs and ticks it, Decision: developer AC readings passed to chk-f1-verify (--help=x and -h=x as help, JSON field order on the raw line, /dev/full test Linux only, milestone number 3), Decision: 'Scaffold from the decision' treated as Build it, closed together with Verify it, Sub session dev-f1-build (developer: scaffold from the decision; merged as 5f44d88, build plan steps 1 to 5 done, step 5 Linux build BLOCKED, owner user), Milestone 3: Scaffold from the decision (build plan steps 1 to 5, dev-f1-build, 2026-11-14) (+3 more)

### Community 13 - "Finding Codes and One Finding Rule"
Cohesion: 0.14
Nodes (26): Configuration Version (config_version key), Finding (stable rejection code), Five new nits in the fix range: deny_unknown_fields phrase at index.md:431, config_version start value at lib.rs:354, 1_048_577 literal at main.rs:89, compile_fail breadth at lib.rs:28, stale CLAUDE.md and AGENTS.md lines, Sub session arc-f1-update continued (architect update: deny_unknown_fields phrase in step 4, one syntax or schema finding chosen by the parser; done at 5670851 without AC change, lead gate passed, merged into the parent of the docs(changelog) commit), Decision: of two schema errors the one with the alphabetically first key is reported, not the first in the file; spec states the choice belongs to the parser (mode auto, follows the code); runner up sort by file position; sent to arc-f1-update, config_version key (required, supports exactly 1), Configuration rules (shared configuration key convention), Consequence: only one syntax or schema error is reported per run, and with several it need not be the first one in the file (+18 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.07
Nodes (47): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), criterion 0.8.2 (microbenchmarks), Hybrid parser policy (+39 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.14
Nodes (20): Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table, Operation admission boundary, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine (+12 more)

### Community 16 - "Scope 001 Coordination Brief and Sub Sessions"
Cohesion: 0.10
Nodes (40): Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Box ticked: Test it (/test stack and workspace), Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only), Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Standby first turn of a new session (STATUS: standby until lead sends 'mulai'), Log koordinasi: 001-stack-and-workspace (coordination log), Sub session arc-f1-update (architect update: record build notes in spec 0001; merged to bed9fad, spec commit a6e0042), Sub session arc-f1-update reused (architect update: nine spec 0001 clarifications from the review in index.md and a new rationale.md table, commit 5f1e995, no AC added or changed; merged with dev-f1-build) (+32 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (48): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+40 more)

### Community 18 - "Project Guidance, Changelog and Status Lifecycle"
Cohesion: 0.13
Nodes (29): POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust; the repository holds the planning package and the scope 001 workspace slice (apps/pool, crates/pool-config, offline pool check-config), not an implemented proxy, Cargo workspace holds only the pool and pool-config packages, and there is no CI yet (inspect it before relying on documentation), Changelog (CHANGELOG.md; Keep a Changelog 1.1.0 format, Semantic Versioning 2.0.0), Changelog format: Keep a Changelog 1.1.0 and Semantic Versioning 2.0.0, Claude Code guidance (CLAUDE.md), Current state: planning package plus the scope 001 workspace (apps/pool with the pool binary, crates/pool-config); only command is the offline pool check-config; no listener, proxy path or CI; Rust 1.100.0 pinned by rust-toolchain.toml, Cargo checks run by hand today (cargo fmt --all --check, clippy --locked -D warnings, test --locked, single test, check-config on valid-minimal.toml); CI arrives with scope 002, Spec 0001 lifecycle status mirrors scope feature 1: In Progress while the foundation is built and proven, Accepted when the feature is done (+21 more)

### Community 19 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.16
Nodes (18): Virtual Cancellation Model, Session Model (pinned backend lease), Data plane, Frontend owner task, Architecture overview, Application path sequence, Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), F-018 CancelRequest and BackendKeyData (+10 more)

### Community 20 - "Manual Failover, Fencing and Write Barrier"
Cohesion: 0.23
Nodes (12): F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Fence Receipt, Write Barrier, R-09 Failover loses acknowledged commits, No silent replay of uncertain work (lost COMMIT response), Manual failover PRD, Durable intent before fencing and reconciliation of unknown outcomes, Failover plan and execute API (/v1/failover/plans) (+4 more)

### Community 21 - "Core Proxy PRD, Decision Ledger and Routing PRDs"
Cohesion: 0.13
Nodes (15): Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, One daemon with separate modules plus separate CLI, pg_query parser with PostgreSQL 18 grammar, Prometheus metrics and tracing logs (+7 more)

### Community 22 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (60): CapabilityState, validate(), ValidatedConfig, ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling() (+52 more)

### Community 23 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.14
Nodes (20): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Pool limits and operations, Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Durable approved writer authority (+12 more)

### Community 24 - "pool-config Validation Library"
Cohesion: 0.17
Nodes (11): error_location(), Finding, FindingCode, Location, position(), Rejection, to_u32(), Display (+3 more)

### Community 25 - "Scope 001 Review Record and Verdicts"
Cohesion: 0.24
Nodes (14): docs/reviews section of the documentation index (one review row), Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), Review summary: two package workspace and offline pool check-config follow spec 0001; output from constants only, bounded read, enabled = true refused in every spelling, Review test coverage: what is covered well and the gaps (array starting with a boolean, permission denied arm, no span arm, column through validate, AC-8/12/13 guards, --format=<bytes>), Review method: two passes on the same reviewer model; the second pass checked each finding and found the Major by running the binary, Review verdict: Approve with nits after the fix round at 4ebcf93 (first pass at 02bdcbe was Changes requested), First review verdict at 02bdcbe: Changes requested (0 Blocker, 1 Major, 6 Minor, 11 Nit) (+6 more)

### Community 26 - "Delegated Spec Decisions and Linux Target"
Cohesion: 0.24
Nodes (12): coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Sub session flow (read brief, general rules, role file, invoke skill, work in existing worktree), Decision: main fast-forwarded from 1246c0d to 591317f so the lead runs on main with the coordinate skill folder, Keputusan delegasi (delegated spec decisions taken on the user's behalf; four berat rows plus one biasa summary), Delegated decision (berat, 0001 AC-9): check-config diagnostics contain only constant text, schema key path, line and column, never file content, Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; awaiting user VM architecture, User value F1-N1: proxy and PostgreSQL VM architecture (x86_64 or ARM); x86_64-unknown-linux-gnu inferred from Proxmox VE (+4 more)

### Community 27 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (87): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+79 more)

### Community 28 - "IoFinding and Permission Denied Path"
Cohesion: 0.15
Nodes (18): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+10 more)

### Community 29 - "Leases, Cancellation and Login Routing"
Cohesion: 0.18
Nodes (14): Retry only before initial lease and ReadyForQuery exposure, Authoritative barrier across every write path, Independent review resolutions (seven approved fixes), Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Routing decision: primary default, explicit read only session profiles, Cancellation forwarded only through the live lease owner, CancellationEntry (+6 more)

### Community 30 - "Safety Invariants and Session Risks"
Cohesion: 0.07
Nodes (45): Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Invariant: preserve transaction and session ownership, Invariant: promotion requires a reviewed operation and independently verified fencing (+37 more)

### Community 31 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.13
Nodes (26): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), check_config() (+18 more)

### Community 32 - "Query Cache PRD"
Cohesion: 0.38
Nodes (7): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Query cache PRD, Caching disabled until a separate contract is approved and proven, Candidate CacheKey, Cache invalidation and dependency contract

### Community 33 - "Spec 0001 Acceptance Criteria"
Cohesion: 0.29
Nodes (29): Review conformance by criterion table (AC-1 to AC-13: AC-2 not judged, AC-7 deviates for the array row, AC-11 /dev/full clause not judged), Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary, Delegated decision (0001 AC-7, AC-11): nine spec clarifications from the review, spec follows the existing code and the reviewer and tester recommendation; runner up change the code (--format=<bytes> as wrong format value); status berlaku, AC-10 JSON contract (one JSON object line on stdout, empty stderr), AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full), AC-13 Stop line and hygiene (no later crates, unsafe_code forbid, fmt, clippy, test pass), AC-3 Valid configuration (valid-* fixtures exit 0 with exact report), AC-4 Off is safe (reserved capability absent, empty or false validates as disabled) (+21 more)

### Community 34 - "Security Requirements, HA Features and V1.5"
Cohesion: 0.13
Nodes (19): Rust and Runtime Standards, Security and Review Requirements, F-107 Rate and resource limits (admission, frame and auth CPU budgets), HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-13 Memory exhaustion, R-21 Proxmox host correlated failure (+11 more)

### Community 35 - "Linux Build Evidence, AC-2 and VM Architecture"
Cohesion: 0.15
Nodes (35): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Added: spec 0001 build notes, AC-1 to AC-13, five step build plan, verification record and independent review; Linux container build (AC-2) recorded as blocked, not run yet, Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md, Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Open: the Linux build (0001 AC-2) and the two Linux only tests have not run, so feature 1 stays in progress until they pass, Open blocker: 0001 AC-2 and the /dev/full clause of AC-11 not run (docker info failed at 14:30, 16:40, 17:10, 19:17 and 20:15; 136 tests expected on Linux; holds feature 1 done, not the review), Decision: after all boxes are ticked feature 1 is NOT marked done and spec 0001 stays In Progress until AC-2 Linux build runs (F1-1, F1-2 PASS); deviates from the skill default of done at the last tick (+27 more)

### Community 36 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.36
Nodes (11): Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, --format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument, Value sourcing (where each output value comes from), For /check review: notes handed to the review, Review note: --config=<bytes that are not UTF-8> is unexpected argument (as written in the spec, may surprise an operator) (+3 more)

### Community 37 - "Failover Records and Fencing Authority"
Cohesion: 0.26
Nodes (13): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+5 more)

### Community 38 - "SQL Routing PRD and Pinned Profiles"
Cohesion: 0.36
Nodes (9): Policy Version, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+1 more)

### Community 39 - "Authentication, Security Features and Health PRD"
Cohesion: 0.08
Nodes (36): Administration requires authentication and authorization, Separate Frontend and Backend Identity, Dependency and Configuration Change Rules, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-108 Secure reload (validate entire generation before publication) (+28 more)

### Community 40 - "Slices, Milestones and Readiness Gates"
Cohesion: 0.12
Nodes (29): Tests and Evidence Standard, Slice, Verification Record, Walking Slice (glossary term), Handoff for Later Slices, Stable ID Citation Rule, Where a Change Belongs (documentation ownership table), Milestones (+21 more)

### Community 41 - "Load Balancing and Replica Selection"
Cohesion: 0.36
Nodes (9): No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state), Replica eligibility filter (role, identity, freshness, maintenance, lag), Primary fallback disabled in V1, ReplicaPolicy, SelectorState (+1 more)

### Community 42 - "Verify Record First Run and Toolchain"
Cohesion: 0.20
Nodes (16): Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, Box ticked: Verify it (/check verify stack and workspace), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test), Sub session chk-f1-verify continued (reverify on the merged tree after the review fixes, closes Minor 6; report blocked, commit cf928e7: revision b6ce165, 11 AC PASS, 134 tests, 398 probe checks; AC-2 and /dev/full clause BLOCKED), Decision: fix round merge 4ebcf93, review log b6ce165; re-review on 02bdcbe..4ebcf93 closes the Major (37 probe files), Minor 1 to 4, Minor 5 in part (Follow-up 11), Minor 6 by chk-f1-verify, 10 of 11 Nits; five optional new Nits routed, Row 10 evidence: from the second verification pass, checked on a debug pool at b6ce165 (macOS, Rust 1.100.0): zzz/aaa reports line 3, reversed reports line 2, unknown key in [result_cache] with wrong type in [automatic_promotion] reports the [automatic_promotion] line, two syntax errors on lines 2 and 3 report line 2, Full verify: /check verify stack and workspace (checker chk-f1-verify, 2026-11-14, rerun from its own checkout against the real pool binary) (+8 more)

### Community 43 - "Reserved Capabilities and Secret Safety"
Cohesion: 0.20
Nodes (19): Added: first configuration contract (config_version = 1 plus reserved result_cache and automatic_promotion tables; absent or enabled = false is valid, asking for either is config.unsupported_capability, SEC-10), Added: fixed finding codes (config.syntax, config.schema, config.missing_key, config.unsupported_version, config.too_large, config.not_utf8) with line and column; output from constant text, key names and positions only, so no secret appears (SEC-11), Automatic Promotion (reserved and off), Reserved Capability, Result Cache (reserved and off), The eight items asked of the review (conformance, SEC-10 and SEC-11, no secret in output or Debug, read limit and offline, unsafe_code and package set, crate boundaries, test quality, rubric), Delegated decision (berat, 0001 AC-4, AC-6): automatic promotion capability key named automatic_promotion (cache: result_cache) over automatic_failover, SEC-10 Disable automatic promotion and result caching by default (+11 more)

### Community 44 - "Column Counting Rule and Tests"
Cohesion: 0.27
Nodes (10): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way), Second review run at 4ebcf93: fmt and clippy exit 0, tests 3 + 66 + 3 + 60 + 2 passed, 18 lock packages, AC-12 grep empty; Linux build not run, Second review, fix round at 4ebcf93 (range 02bdcbe..4ebcf93; code a705659, 5a99563, b18b288, 453ae73; spec 5f1e995; same reviewer session), Spec text against the code: the nine clarifications in index.md and the rationale table match the fixed binary; AC-1 to AC-13 unchanged, What is left after the second review: Minor 6 verify record on the merged tree; AC-12 and AC-13 guards wait for scope 002 (Follow-up 11) (+2 more)

### Community 45 - "Hand Written Deserialize and Known Fields"
Cohesion: 0.20
Nodes (12): KnownField, Result, D, DeserializeSeed, Probes of the hand written Deserialize (37 made files at 4ebcf93: key given twice, unknown keys, enabled not boolean, capability not a table, other spellings, config_version, canary, valid files), Decision: three architect additions beyond the brief accepted (top level may keep derive and later slices follow the table-only rule; no location only for dotted keys and table headers; Build plan step 1 clause reworded), Schema step rule: private Serde structures reject unknown keys; a structure that stands for a table accepts a table only, via a hand written Deserialize with a map-only visitor (a derive also accepts a sequence); later slices follow the rule, E (+4 more)

### Community 46 - "Exit Status and Output Convention"
Cohesion: 0.17
Nodes (17): Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish), Added: fifteen configuration fixtures and 134 automated tests covering every first build acceptance criterion on the workstation, Added: pool --help and pool --version, Added: JSON report (--format json), one line with a fixed field order, for scripts, Added: Cargo workspace with the pool daemon binary (apps/pool) and the pool-config library (crates/pool-config); Rust 1.100.0 pinned by rust-toolchain.toml; unsafe code forbidden in both, Unreleased, Added: the first workspace slice of scope 001 (ten entries), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Output Version (output_version field) (+9 more)

### Community 47 - "Reserved Capability Table Fix"
Cohesion: 0.26
Nodes (12): capability_state(), RawCapability, RawConfig, Option, Vec, ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding(), Deserialize, Major: a reserved capability written as an array (result_cache = [false]) is accepted as valid, because derived RawCapability Deserialize accepts a positional sequence (lib.rs:207) (+4 more)

### Community 48 - "Proposed Stack and Deferred Features"
Cohesion: 0.20
Nodes (10): Transaction pooling (deferred), Future multi proxy deployment, Transparent per query routing (later feature), Versioned local admin REST API (Axum), Deferred features (cache, automatic failover, native replication, multiplexing), External etcd for future coordination, Proposed stack, Tokio runtime with bounded tasks and channels (+2 more)

### Community 49 - "Review Rule Checks and Whole Config Validation"
Cohesion: 0.29
Nodes (10): Minor 5: AC-8 constructor guarantee, AC-12 package set and grep, AC-13 stop line have no automated guard (suggested compile_fail doc tests and scope 002 CI steps), SEC-12 Validate a whole configuration generation before publication; rotation drains retired identities, AC-8 Whole configuration (all findings in one run; validate is the only path to ValidatedConfig), Scratch crate depending on pool-config by path: 5 programs that must not compile (E0451 private fields, E0277 Default/DeserializeOwned/Clone), 1 that runs, Rules made clearer in the review, each checked on the binary (17 rows, all PASS), Rule check: an array in place of a reserved table gives exactly one config.schema (result_cache = [false] now config.schema at line 2, column 16), Rule check: --format=<bytes> and --config=<bytes> not UTF-8 exit 2 unexpected argument; --format <bytes> as separate value is wrong format value, Rule check: no ValidatedConfig can be built outside the crate (two compile_fail doc tests pass; E0451 and E0277 in the scratch crate) (+2 more)

### Community 50 - "ADR 0001 and Modular Daemon Decision"
Cohesion: 0.53
Nodes (6): ADR 0001. Initial architecture, Identity Map, Verified Principal, Spec 0001. POOL initial architecture, Modular daemon with independent CLI decision, Shared process failure domain

### Community 52 - "Workspace Layout, Bounded Read and FIFO Limit"
Cohesion: 0.20
Nodes (20): ac12_dev_null_is_an_empty_file_that_misses_config_version(), Added: pool check-config --config <PATH> [--format human|json], offline check before start; opens no socket, starts no thread, reads at most 1 MiB plus one byte, First review run at 02bdcbe: tests 3 + 64 + 3 + 57, fmt, clippy, release build, metadata, grep, nm -u; second pass with twelve made files, Delegated decisions (biasa, 0001 AC-1, AC-12): 17 build notes choices (pinned rust slim bookworm image and git archive HEAD, std argument parsing, serde_json, no thiserror/anyhow/tracing, two pass validation, 1 MiB limit, ...), AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read), Bounded configuration read (at most 1048577 bytes via Read::take), Build plan step 1: Workspace skeleton (+12 more)

## Knowledge Gaps
- **53 isolated node(s):** `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors`, `Two experienced engineers planning baseline`, `Declared PostgreSQL 18 protocol profile (3.0/3.2)` (+48 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Documentation Index, Handoffs and Readiness` to `Standards, Tooling and Release Qualification`, `Feature Inventory and Parity Matrix`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Operation Journal Records and Generations`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Feature Families and Planned Scopes`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Pgpool Baseline, Competitors and Feasibility`, `Project Guidance, Changelog and Status Lifecycle`, `Virtual Cancellation and Architecture Overview`, `Manual Failover, Fencing and Write Barrier`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Scope 001 Review Record and Verdicts`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Security Requirements, HA Features and V1.5`, `Linux Build Evidence, AC-2 and VM Architecture`, `Failover Records and Fencing Authority`, `SQL Routing PRD and Pinned Profiles`, `Authentication, Security Features and Health PRD`, `Slices, Milestones and Readiness Gates`, `Load Balancing and Replica Selection`, `Reserved Capabilities and Secret Safety`, `ADR 0001 and Modular Daemon Decision`?**
  _High betweenness centrality (0.263) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `ADR 0001 and Modular Daemon Decision` to `Standards, Tooling and Release Qualification`, `Scope 001 Coordination Brief, Log and Authorization`, `Administration, Cluster and Health PRDs`, `Architecture Spec and Repository Layout`, `Operation Journal Records and Generations`, `Documentation Index, Handoffs and Readiness`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and HA Features`, `Feature Families and Planned Scopes`, `Finding Codes and One Finding Rule`, `Rust Technology Evaluation`, `Scope 001 Coordination Brief and Sub Sessions`, `Project Guidance, Changelog and Status Lifecycle`, `Virtual Cancellation and Architecture Overview`, `Manual Failover, Fencing and Write Barrier`, `Core Proxy PRD, Decision Ledger and Routing PRDs`, `Session Model, Pooling Modes and Data Path`, `Scope 001 Review Record and Verdicts`, `Delegated Spec Decisions and Linux Target`, `Safety Invariants and Session Risks`, `Query Cache PRD`, `Spec 0001 Acceptance Criteria`, `Linux Build Evidence, AC-2 and VM Architecture`, `SQL Routing PRD and Pinned Profiles`, `Authentication, Security Features and Health PRD`, `Slices, Milestones and Readiness Gates`, `Load Balancing and Replica Selection`, `Reserved Capabilities and Secret Safety`, `Exit Status and Output Convention`, `Proposed Stack and Deferred Features`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Why does `First Handoff: the Workspace` connect `Documentation Index, Handoffs and Readiness` to `Standards, Tooling and Release Qualification`, `Scope 001 Coordination Brief, Log and Authorization`, `Feature Inventory and Parity Matrix`, `Spec 0001 Acceptance Criteria`, `Architecture Spec and Repository Layout`, `Linux Build Evidence, AC-2 and VM Architecture`, `Authentication, Security Features and Health PRD`, `Slices, Milestones and Readiness Gates`, `Feature Families and Planned Scopes`, `Reserved Capabilities and Secret Safety`, `Scope 001 Coordination Brief and Sub Sessions`, `Workspace Layout, Bounded Read and FIFO Limit`?**
  _High betweenness centrality (0.085) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors` to the rest of the system?**
  _53 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope 001 Coordination Brief, Log and Authorization` be split into smaller, more focused modules?**
  _Cohesion score 0.13793103448275862 - nodes in this community are weakly interconnected._
- **Should `Feature Inventory and Parity Matrix` be split into smaller, more focused modules?**
  _Cohesion score 0.08253968253968254 - nodes in this community are weakly interconnected._