# Graph Report - pool  (2026-11-14)

## Corpus Check
- 3 files · ~88,233 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1242 nodes · 3798 edges · 56 communities (53 shown, 3 thin omitted)
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 740 edges (avg confidence: 0.85)
- Token cost: 77,581 input · 0 output

## Community Hubs (Navigation)
- check-config Integration Tests
- pool-config Validation Tests
- Pgpool Baseline, Competitors and Feasibility
- Protocol Contract and Driver Compatibility
- Rust Technology Evaluation
- Safety Invariants and Session Risks
- Documentation Index, Handoffs and Readiness
- Leases, Cancellation and Login Routing
- pool-config Validation Library
- Slices, Milestones and Readiness Gates
- Manual Failover, Fencing and Write Barrier
- Linux Build Evidence, AC-2 and VM Architecture
- Scope 001 Coordination Brief, Log and Authorization
- Standards, Tooling and Release Qualification
- Administration, Cluster and Health PRDs
- Glossary and Routing Profiles
- Feature Inventory and Parity Matrix
- Feature Families and Planned Scopes
- Spec 0001 Acceptance Criteria
- Project Guidance, Changelog and Status Lifecycle
- IoFinding and Permission Denied Path
- Operation Journal Records and Generations
- Session Model, Pooling Modes and Data Path
- Load Balancing and Replica Selection
- pool CLI Entry and Argument Parsing
- Hand Written Deserialize and Known Fields
- Finding Codes and One Finding Rule
- Scope 001 Coordination Brief and Sub Sessions
- Scope 002 Brief Rules
- Scope 001 Review Record and Verdicts
- Workspace Layout, Bounded Read and FIFO Limit
- Verify Record First Run and Toolchain
- Reserved Capabilities and Secret Safety
- Offline Bounds, Parser Depth and Known Limits
- SQL Routing and Replica Profiles
- Status Lifecycle and Feature 1 State
- Exit Status and Output Convention
- Exit Status and Output Convention
- Scope 001 Coordination Log and Closing
- Scaffold Step and dev-f1-build Gate
- Architecture Spec and Repository Layout
- Review Clarifications and check-config Usage Rules
- Failover Records and Fencing Authority
- Proposed Stack and Deferred Features
- Delegated Spec Decisions and Linux Target
- pool main.rs Entry Point
- Linux Target and VM Architecture
- Query Cache PRD
- Scope 002 Follow-ups from Spec 0001
- Virtual Cancellation and Architecture Overview
- Virtual Cancellation Model
- Column Counting Rule and Tests
- KPIs and Proposed Gates
- chk-f2-review Session
- dev-f2-audit Session
- tst-f2-test Session

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 81 edges
2. `Spec 0001. POOL initial architecture` - 58 edges
3. `Glossary` - 58 edges
4. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 55 edges
5. `Feature inventory and parity matrix` - 47 edges
6. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 46 edges
7. `Scope: POOL, order and status index` - 46 edges
8. `Testing strategy` - 44 edges
9. `1. Stack and workspace (in-progress)` - 43 edges
10. `Security requirements` - 39 edges

## Surprising Connections (you probably didn't know these)
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `automatic_promotion_enabled_is_refused()` --implements--> `AC-6 Automatic promotion is refused (config.unsupported_capability at automatic_promotion.enabled)`  [EXTRACTED]
  crates/pool-config/tests/validate.rs → docs/specs/0001-initial-architecture/index.md
- `enabled_true_is_refused_in_every_table_spelling()` --implements--> `AC-5 Result cache is refused (config.unsupported_capability at result_cache.enabled)`  [EXTRACTED]
  crates/pool-config/tests/validate.rs → docs/specs/0001-initial-architecture/index.md

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
- **Scope 002 steps blocked on the user** — docs_scope_coordination_002_standards_and_tooling_manual_f2_1, docs_scope_coordination_002_standards_and_tooling_manual_f2_2, docs_scope_coordination_002_standards_and_tooling_manual_f2_n1, docs_scope_coordination_002_standards_and_tooling_log_open_blockers [EXTRACTED 1.00]
- **Scope 002 wave 1 coordination sub sessions** — docs_scope_coordination_002_standards_and_tooling_log_arc_f2_spec, docs_scope_coordination_002_standards_and_tooling_log_dev_f2_audit, docs_scope_coordination_002_standards_and_tooling_log_dev_f2_build, docs_scope_coordination_002_standards_and_tooling_log_chk_f2_verify, docs_scope_coordination_002_standards_and_tooling_log_tst_f2_test, docs_scope_coordination_002_standards_and_tooling_log_chk_f2_review [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (56 total, 3 thin omitted)

### Community 0 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (86): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+78 more)

### Community 1 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (57): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+49 more)

### Community 2 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (45): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+37 more)

### Community 3 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.05
Nodes (44): Invariant: close uncertain pooled state, F-030 Validation and reset (close uncertain sessions), F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-107 Rate and resource limits (admission, frame and auth CPU budgets), F-108 Secure reload (validate entire generation before publication) (+36 more)

### Community 4 - "Rust Technology Evaluation"
Cohesion: 0.08
Nodes (44): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), criterion 0.8.2 (microbenchmarks) (+36 more)

### Community 5 - "Safety Invariants and Session Risks"
Cohesion: 0.07
Nodes (38): Administration requires authentication and authorization, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Invariant: preserve transaction and session ownership, Invariant: keep writes on the verified primary, Session Model (pinned backend lease) (+30 more)

### Community 6 - "Documentation Index, Handoffs and Readiness"
Cohesion: 0.09
Nodes (37): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), Knowledge graph refresh before commit (graphify-out kept current), Tracer Bullet Build Approach, Agent Workflow (+29 more)

### Community 7 - "Leases, Cancellation and Login Routing"
Cohesion: 0.07
Nodes (37): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, Independent review resolutions (seven approved fixes) (+29 more)

### Community 8 - "pool-config Validation Library"
Cohesion: 0.11
Nodes (19): capability_state(), CapabilityState, error_location(), Finding, FindingCode, KnownField, Location, position() (+11 more)

### Community 9 - "Slices, Milestones and Readiness Gates"
Cohesion: 0.18
Nodes (33): Working with the Docs, Shared Conventions Table, Pooling Features (F-023 to F-032), Starting development (implementation order), First Handoff: the Workspace, Handoff for Later Slices, Documentation index, Stable ID Citation Rule (+25 more)

### Community 10 - "Manual Failover, Fencing and Write Barrier"
Cohesion: 0.09
Nodes (32): Invariant: promotion requires a reviewed operation and independently verified fencing, Safe first party Rust and bounded resources, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Failover features F-067 to F-075 (+24 more)

### Community 11 - "Linux Build Evidence, AC-2 and VM Architecture"
Cohesion: 0.17
Nodes (32): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md, Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Open: the Linux build (0001 AC-2) and the two Linux only tests have not run, so feature 1 stays in progress until they pass, Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user) (+24 more)

### Community 12 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.13
Nodes (29): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first), Always forbidden paths for sub sessions (docs/scope, docs/prd, AGENTS.md, CLAUDE.md, graphify-out, .claude, spec content outside ownership) (+21 more)

### Community 13 - "Standards, Tooling and Release Qualification"
Cohesion: 0.13
Nodes (28): Feature 1 is built and stays in progress until its Linux build evidence exists; next queued feature is 2 (standards and tooling), then the first walking slice, No CI yet (arrives with scope 002); Linux container build per spec 0001; later property, PostgreSQL integration and compatibility/fuzz/chaos tests; do not invent passing results, Dependency and Configuration Change Rules, Git and Release Standards, Configuration Generation, Milestones, Core MVP, Release Practices (+20 more)

### Community 14 - "Administration, Cluster and Health PRDs"
Cohesion: 0.12
Nodes (28): Axum REST administration API plus CLI, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control, GET /v1/nodes node status route, HealthState (Unknown, Healthy, Suspect, Unavailable, Recovering) (+20 more)

### Community 15 - "Glossary and Routing Profiles"
Cohesion: 0.14
Nodes (27): Separate Frontend and Backend Identity, F-066 Topology reconciliation (durable writer/epoch/quarantine), Glossary, Admission, Approved Writer, Backend, Credential Generation, Detach (+19 more)

### Community 16 - "Feature Inventory and Parity Matrix"
Cohesion: 0.11
Nodes (27): Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-060 Network partitions (health changes eligibility only), F-065 Primary Identification, F-065 Primary identification (durably approved writer with fresh evidence) (+19 more)

### Community 17 - "Feature Families and Planned Scopes"
Cohesion: 0.11
Nodes (27): HA Features (F-081 to F-087), Tracer Bullet build approach, V1.5 Milestone, Walking Slice Milestone, R-21 Proxmox host correlated failure, R-24 Walking slice exceeds its estimate, Roadmap, Delivery slices (+19 more)

### Community 18 - "Spec 0001 Acceptance Criteria"
Cohesion: 0.32
Nodes (26): Review conformance by criterion table (AC-1 to AC-13: AC-2 not judged, AC-7 deviates for the array row, AC-11 /dev/full clause not judged), Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary, Delegated decision (0001 AC-7, AC-11): nine spec clarifications from the review, spec follows the existing code and the reviewer and tester recommendation; runner up change the code (--format=<bytes> as wrong format value); status berlaku, AC-10 JSON contract (one JSON object line on stdout, empty stderr), AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full), AC-13 Stop line and hygiene (no later crates, unsafe_code forbid, fmt, clippy, test pass), AC-3 Valid configuration (valid-* fixtures exit 0 with exact report), AC-4 Off is safe (reserved capability absent, empty or false validates as disabled) (+18 more)

### Community 19 - "Project Guidance, Changelog and Status Lifecycle"
Cohesion: 0.17
Nodes (25): POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust; the repository holds the planning package and the scope 001 workspace slice (apps/pool, crates/pool-config, offline pool check-config), not an implemented proxy, User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested, Cargo workspace holds only the pool and pool-config packages, and there is no CI yet (inspect it before relying on documentation), Changelog (CHANGELOG.md; Keep a Changelog 1.1.0 format, Semantic Versioning 2.0.0), Added: Cargo workspace with the pool daemon binary (apps/pool) and the pool-config library (crates/pool-config); Rust 1.100.0 pinned by rust-toolchain.toml; unsafe code forbidden in both, Changelog format: Keep a Changelog 1.1.0 and Semantic Versioning 2.0.0, Claude Code guidance (CLAUDE.md), Current state: planning package plus the scope 001 workspace (apps/pool with the pool binary, crates/pool-config); only command is the offline pool check-config; no listener, proxy path or CI; Rust 1.100.0 pinned by rust-toolchain.toml (+17 more)

### Community 20 - "IoFinding and Permission Denied Path"
Cohesion: 0.15
Nodes (18): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+10 more)

### Community 21 - "Operation Journal Records and Generations"
Cohesion: 0.14
Nodes (24): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Control plane, ApprovedTopology record, AuditEvent record (+16 more)

### Community 22 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.12
Nodes (24): Weighted round robin replica selection, Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Durable approved writer authority (+16 more)

### Community 23 - "Load Balancing and Replica Selection"
Cohesion: 0.13
Nodes (24): Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Eligible Replica, Replica Profile, Routing Preview, R-07 Replica lag mistaken for freshness, SQL routing PRD (+16 more)

### Community 24 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.17
Nodes (19): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), ac11_a_first_argument_that_is_not_a_command_is_unknown() (+11 more)

### Community 25 - "Hand Written Deserialize and Known Fields"
Cohesion: 0.19
Nodes (20): RawCapability, RawConfig, Option, ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding(), D, Deserialize, Probes of the hand written Deserialize (37 made files at 4ebcf93: key given twice, unknown keys, enabled not boolean, capability not a table, other spellings, config_version, canary, valid files), Major: a reserved capability written as an array (result_cache = [false]) is accepted as valid, because derived RawCapability Deserialize accepts a positional sequence (lib.rs:207) (+12 more)

### Community 26 - "Finding Codes and One Finding Rule"
Cohesion: 0.19
Nodes (22): Minor 5: AC-8 constructor guarantee, AC-12 package set and grep, AC-13 stop line have no automated guard (suggested compile_fail doc tests and scope 002 CI steps), Sub session arc-f1-update continued (architect update: deny_unknown_fields phrase in step 4, one syntax or schema finding chosen by the parser; done at 5670851 without AC change, lead gate passed, merged into the parent of the docs(changelog) commit), Decision: of two schema errors the one with the alphabetically first key is reported, not the first in the file; spec states the choice belongs to the parser (mode auto, follows the code); runner up sort by file position; sent to arc-f1-update, SEC-12 Validate a whole configuration generation before publication; rotation drains retired identities, AC-8 Whole configuration (all findings in one run; validate is the only path to ValidatedConfig), Build plan step 2: Validation in pool-config, Consequence: only one syntax or schema error is reported per run, and with several it need not be the first one in the file, FindingCode (config.* codes) (+14 more)

### Community 27 - "Scope 001 Coordination Brief and Sub Sessions"
Cohesion: 0.16
Nodes (22): Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Box ticked: Verify it (/check verify stack and workspace), Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Standby first turn of a new session (STATUS: standby until lead sends 'mulai'), Checked in wave 1: F1 Scaffold from the decision (dev-f1-build done and chk-f1-verify report, merged 5f44d88; manual F1-1, F1-2 NOT_RUN), Checked in wave 1: F1 Test it (tst-f1-test commits 5ad687d, fae4bc6; 127 tests green on main; F1-2 NOT_RUN for two Linux-only tests), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user) (+14 more)

### Scope 002 Brief Rules - "Scope 002 Brief Rules"
Cohesion: 0.13
Nodes (21): Brief general rules: 002-standards-and-tooling, Allowed downloads and installs (crates adopted by spec 0002, pinned toolchain, cargo-deny, cargo-fuzz), Decision mode auto, Disk budget rule (max 2 concurrent Rust builds, delete target before done), GitHub read-only rule for sessions, Sub sessions never update graphify-out; lead updates graph in merge commit, Manual steps policy (Docker, GitHub settings, push, PRs run only by user), Scope 002 boundary: align written rules with workspace and install required checks (+13 more)

### Community 29 - "Scope 001 Review Record and Verdicts"
Cohesion: 0.19
Nodes (18): Security and Review Requirements, docs/reviews section of the documentation index (one review row), Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), First review nits (11): --format=<bytes> equals form, --help=x after check-config, u64::MAX read fallback, FIFO blocks, duplicated value block, capability_state comment, repeated numbers in messages, three weak tests, stale status sentence, Review summary: two package workspace and offline pool check-config follow spec 0001; output from constants only, bounded read, enabled = true refused in every spelling, Review test coverage: what is covered well and the gaps (array starting with a boolean, permission denied arm, no span arm, column through validate, AC-8/12/13 guards, --format=<bytes>), Review method: two passes on the same reviewer model; the second pass checked each finding and found the Major by running the binary (+10 more)

### Community 30 - "Workspace Layout, Bounded Read and FIFO Limit"
Cohesion: 0.23
Nodes (17): ac12_dev_null_is_an_empty_file_that_misses_config_version(), Added: pool check-config --config <PATH> [--format human|json], offline check before start; opens no socket, starts no thread, reads at most 1 MiB plus one byte, First review run at 02bdcbe: tests 3 + 64 + 3 + 57, fmt, clippy, release build, metadata, grep, nm -u; second pass with twelve made files, AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read), Bounded configuration read (at most 1048577 bytes via Read::take), Build plan step 1: Workspace skeleton, Cargo.lock with exactly sixteen locked packages (+9 more)

### Community 31 - "Verify Record First Run and Toolchain"
Cohesion: 0.15
Nodes (17): Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, Five new nits in the fix range: deny_unknown_fields phrase at index.md:431, config_version start value at lib.rs:354, 1_048_577 literal at main.rs:89, compile_fail breadth at lib.rs:28, stale CLAUDE.md and AGENTS.md lines, Second review run at 4ebcf93: fmt and clippy exit 0, tests 3 + 66 + 3 + 60 + 2 passed, 18 lock packages, AC-12 grep empty; Linux build not run, Second review, fix round at 4ebcf93 (range 02bdcbe..4ebcf93; code a705659, 5a99563, b18b288, 453ae73; spec 5f1e995; same reviewer session), Spec text against the code: the nine clarifications in index.md and the rationale table match the fixed binary; AC-1 to AC-13 unchanged, What is left after the second review: Minor 6 verify record on the merged tree; AC-12 and AC-13 guards wait for scope 002 (Follow-up 11), Decision: fix round merge 4ebcf93, review log b6ce165; re-review on 02bdcbe..4ebcf93 closes the Major (37 probe files), Minor 1 to 4, Minor 5 in part (Follow-up 11), Minor 6 by chk-f1-verify, 10 of 11 Nits; five optional new Nits routed (+9 more)

### Community 32 - "Reserved Capabilities and Secret Safety"
Cohesion: 0.23
Nodes (16): Invariant: automatic failover and result caching stay disabled until approved and proven, Added: first configuration contract (config_version = 1 plus reserved result_cache and automatic_promotion tables; absent or enabled = false is valid, asking for either is config.unsupported_capability, SEC-10), Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Automatic Promotion (reserved and off), Configuration Version (config_version key), Reserved Capability, Result Cache (reserved and off), Secret Reference (_file key) (+8 more)

### Community 33 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.20
Nodes (16): ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check, Clarification 6: exact column checked for one unambiguous made input with a multi-byte character, others keep at least 1; runner up exact columns everywhere, Clarification evidence: review record docs/reviews/2026-11-14-scope-001-stack-and-workspace.md (rows 1 to 7); debug pool build at 07f7994 rerun for rows 2, 3, 4, 6 (81-part key config.syntax null location, 80-part key config.schema, x = "éé" junk at column 10) (+8 more)

### Community 34 - "SQL Routing and Replica Profiles"
Cohesion: 0.18
Nodes (16): Replication freshness and byte lag, Value sourcing table, Operation admission boundary, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status) (+8 more)

### Status Lifecycle and Feature 1 State - "Status Lifecycle and Feature 1 State"
Cohesion: 0.20
Nodes (15): GA workflow depth (behavior verification, tests, fresh review, documentation), Spec 0001 lifecycle status mirrors scope feature 1: In Progress while the foundation is built and proven, Accepted when the feature is done, Feature status model (Research, Planned; none Implemented or Tested), Documentation checks (links/anchors, scope status agreement, stable requirement IDs), Decision: after all boxes are ticked feature 1 is NOT marked done and spec 0001 stays In Progress until AC-2 Linux build runs (F1-1, F1-2 PASS); deviates from the skill default of done at the last tick, Decision: Build plan step 1 sets spec Status to In Progress; the stale 'lifecycle status remains Proposed' wording is tidied at /sync, not by sub sessions, Feature 1 in progress: every box ticked and workstation evidence recorded; stays in progress until the Linux build named in its Done when has run; no proxy behavior implemented, Scope file ownership rules (+7 more)

### Community 36 - "Exit Status and Output Convention"
Cohesion: 0.18
Nodes (15): Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Added: fixed finding codes (config.syntax, config.schema, config.missing_key, config.unsupported_version, config.too_large, config.not_utf8) with line and column; output from constant text, key names and positions only, so no secret appears (SEC-11), Added: fifteen configuration fixtures and 134 automated tests covering every first build acceptance criterion on the workstation, Added: three shared conventions in the engineering standards (configuration keys, exit status with command output, log event fields with redaction), Unreleased, Added: the first workspace slice of scope 001 (ten entries), Log event fields and redaction convention (ts, level, event; redaction by type), The eight items asked of the review (conformance, SEC-10 and SEC-11, no secret in output or Debug, read limit and offline, unsafe_code and package set, crate boundaries, test quality, rubric), SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL (+7 more)

### Exit Status and Output Convention - "Exit Status and Output Convention"
Cohesion: 0.18
Nodes (15): Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish), Added: pool --help and pool --version, Added: JSON report (--format json), one line with a fixed field order, for scripts, Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Output Version (output_version field), Delegated decision (berat, 0001 AC-3, AC-7, AC-11): exit status 0 success, 1 input rejected, 2 usage, 3 could not complete, as a Shared convention, The check-config command (arguments and usage errors), Exit status and output (shared output convention) (+7 more)

### Scope 001 Coordination Log and Closing - "Scope 001 Coordination Log and Closing"
Cohesion: 0.25
Nodes (15): Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only), Log koordinasi: 001-stack-and-workspace (coordination log), Resolved former blocker: untracked .graphifyignore and graphify-out/cache/last_query_stamp held the closing push; done at d2fda37 (.graphifyignore committed, last_query_stamp gitignored), Closing: all green branches merged; Document and /sync recorded; closing commit docs(scope): close 001-stack-and-workspace coordination; pushed 2026-11-14; five sub sessions archived and cleaned up; nothing retained; remaining blockers Linux build AC-2 (F1-1, F1-2), VM architecture (F1-N1), Follow-up 11 waits for /scope, Closing cleanup: five sub sessions archived (arc-f1-update, dev-f1-build, chk-f1-verify, tst-f1-test, chk-f1-review); each worktree clean and branch an ancestor of origin/main before git worktree remove and git branch -d; git worktree prune; nothing retained, branch docs/cross-check-before-development untouched, Closing push: git push origin main without --force, 1246c0d..d2fda37; main = origin/main = d2fda37; the closing commit is pushed afterwards, Decision: closing push held while .graphifyignore and graphify-out/cache/last_query_stamp are untracked; lead proposes committing .graphifyignore and gitignoring last_query_stamp; waits for user; worktree and branch cleanup after push, Decision: running /coordinate on scope 001 is read as the explicit request to build scope 001 (+7 more)

### Community 39 - "Scaffold Step and dev-f1-build Gate"
Cohesion: 0.23
Nodes (13): Added: spec 0001 build notes, AC-1 to AC-13, five step build plan, verification record and independent review; Linux container build (AC-2) recorded as blocked, not run yet, Decision: developer AC readings passed to chk-f1-verify (--help=x and -h=x as help, JSON field order on the raw line, /dev/full test Linux only, milestone number 3), Build plan (tracer bullet, five steps), Build plan step 4: Workstation evidence, 0001. Verification record, Full verify workstation: macOS Apple silicon, rustc 1.100.0 aarch64-apple-darwin selected by rust-toolchain.toml (not the Linux build), Milestone 3: Scaffold from the decision (build plan steps 1 to 5, dev-f1-build, 2026-11-14), Reading: JSON field order is checked on the raw line (agreed, also done byte for byte) (+5 more)

### Community 40 - "Architecture Spec and Repository Layout"
Cohesion: 0.42
Nodes (13): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+5 more)

### Community 41 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.33
Nodes (12): Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, --format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument, Value sourcing (where each output value comes from), Clarification 2: --format=<bytes> and --config=<bytes> are unexpected argument; runner up byte level split, For /check review: notes handed to the review (+4 more)

### Community 42 - "Failover Records and Fencing Authority"
Cohesion: 0.29
Nodes (12): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+4 more)

### Community 43 - "Proposed Stack and Deferred Features"
Cohesion: 0.18
Nodes (11): Transaction pooling (deferred), Transparent per query routing (later feature), rustls 0.23.46 (TLS), Single rustls crypto provider (aws-lc-rs feature family), Versioned local admin REST API (Axum), Deferred features (cache, automatic failover, native replication, multiplexing), Proposed stack, rustls and tokio-rustls TLS (+3 more)

### Community 44 - "Delegated Spec Decisions and Linux Target"
Cohesion: 0.29
Nodes (10): Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Decision: build notes summary 'Accept this spec?' answered accept without changes (mode auto), checked by lead, Delegated decisions (biasa, 0001 AC-1, AC-12): 17 build notes choices (pinned rust slim bookworm image and git archive HEAD, std argument parsing, serde_json, no thiserror/anyhow/tracing, two pass validation, 1 MiB limit, ...), Keputusan delegasi (delegated spec decisions taken on the user's behalf; four berat rows plus one biasa summary), Delegated decision (berat, 0001 AC-9): check-config diagnostics contain only constant text, schema key path, line and column, never file content, secret-* canary fixtures (CANARY-9f2c7b1e), Fixtures (valid-*, invalid-*, secret-* TOML files), Decision: diagnostics contain only constants, key paths, line and column (+2 more)

### pool main.rs Entry Point - "pool main.rs Entry Point"
Cohesion: 0.44
Nodes (8): check_config(), main(), read_bounded(), Result, Vec, write_stdout(), ExitCode, Path

### Linux Target and VM Architecture - "Linux Target and VM Architecture"
Cohesion: 0.46
Nodes (8): Open blocker: proxy and PostgreSQL VM architecture (x86_64 or ARM) unconfirmed for 0001 AC-2; x86_64 target inferred (F1-N1), Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; awaiting user VM architecture, User value F1-N1: proxy and PostgreSQL VM architecture (x86_64 or ARM); x86_64-unknown-linux-gnu inferred from Proxmox VE, Values from the user (F1-N1: VM architecture), Follow-up: confirm the PostgreSQL and proxy VMs are x86_64, Toolchain and Linux target, x86_64-unknown-linux-gnu Linux target, Decision: Linux target x86_64-unknown-linux-gnu over aarch64

### Community 47 - "Query Cache PRD"
Cohesion: 0.38
Nodes (7): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Query cache PRD, Caching disabled until a separate contract is approved and proven, Candidate CacheKey, Cache invalidation and dependency contract

### Scope 002 Follow-ups from Spec 0001 - "Scope 002 Follow-ups from Spec 0001"
Cohesion: 0.52
Nodes (7): 2. Standards and tooling (planned), Decision: after the fix merge chk-f1-verify reverifies into verify.md, chk-f1-review confirms the fix diff; stale CLAUDE.md and AGENTS.md tidied by /sync at close; spec 0001 Follow-up 11 copied to scope 002 at close, Decision: spec 0001 Follow-up 11 (CI checks for AC-12 and AC-13) not yet copied into scope 002; adding sub tasks belongs to /scope and waits for user approval, History wave 1 (from 591317f): all seven scope 001 boxes ticked; open Linux build (AC-2, /dev/full clause of AC-11, two Linux only tests), so feature 1 and spec 0001 stay in progress; blockers Docker off (F1-1, F1-2), VM architecture (F1-N1); the two untracked files that held the push done at d2fda37; scope 002 next, Follow-up 11 waits for /scope, Follow-up 11: scope 002 adds automatic CI checks for the hand-checked parts of AC-12 (Cargo.lock package set, grep) and AC-13 (four absent paths, lint inheritance); from the scope 001 review, Follow-up: scope 002 runs the Linux build on a Linux runner without emulation, Clarification 8: missing automatic guards for AC-12 and AC-13 become a scope 002 Follow-up; runner up shell checks in cargo test now

### Community 49 - "Virtual Cancellation and Architecture Overview"
Cohesion: 0.40
Nodes (6): Pool limits and operations, Data plane, Frontend owner task, Provisional resource defaults and backpressure, Frontend, Session Permit

### Virtual Cancellation Model - "Virtual Cancellation Model"
Cohesion: 0.50
Nodes (5): Virtual Cancellation Model, F-018 CancelRequest and BackendKeyData, F-018 CancelRequest and BackendKeyData (virtual per lease keys), R-05 Incorrect reuse or late cancel, T-05 Late cancellation affects another user

### Community 51 - "Column Counting Rule and Tests"
Cohesion: 0.70
Nodes (5): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way), Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12), Rule check: the column counts characters (x = "éé" junk is line 2, column 10; bytes would give 12)

### KPIs and Proposed Gates - "KPIs and Proposed Gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

## Knowledge Gaps
- **60 isolated node(s):** `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors`, `Two experienced engineers planning baseline`, `Declared PostgreSQL 18 protocol profile (3.0/3.2)` (+55 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Slices, Milestones and Readiness Gates` to `Pgpool Baseline, Competitors and Feasibility`, `Protocol Contract and Driver Compatibility`, `Rust Technology Evaluation`, `Safety Invariants and Session Risks`, `Documentation Index, Handoffs and Readiness`, `Leases, Cancellation and Login Routing`, `Manual Failover, Fencing and Write Barrier`, `Standards, Tooling and Release Qualification`, `Administration, Cluster and Health PRDs`, `Glossary and Routing Profiles`, `Feature Inventory and Parity Matrix`, `Feature Families and Planned Scopes`, `Project Guidance, Changelog and Status Lifecycle`, `Operation Journal Records and Generations`, `Session Model, Pooling Modes and Data Path`, `Load Balancing and Replica Selection`, `Scope 001 Review Record and Verdicts`, `SQL Routing and Replica Profiles`, `Scaffold Step and dev-f1-build Gate`, `Architecture Spec and Repository Layout`, `Failover Records and Fencing Authority`, `Query Cache PRD`, `Scope 002 Follow-ups from Spec 0001`, `Virtual Cancellation and Architecture Overview`?**
  _High betweenness centrality (0.272) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Documentation Index, Handoffs and Readiness` to `Rust Technology Evaluation`, `Safety Invariants and Session Risks`, `Leases, Cancellation and Login Routing`, `Slices, Milestones and Readiness Gates`, `Manual Failover, Fencing and Write Barrier`, `Linux Build Evidence, AC-2 and VM Architecture`, `Scope 001 Coordination Brief, Log and Authorization`, `Administration, Cluster and Health PRDs`, `Glossary and Routing Profiles`, `Feature Families and Planned Scopes`, `Spec 0001 Acceptance Criteria`, `Project Guidance, Changelog and Status Lifecycle`, `Operation Journal Records and Generations`, `Session Model, Pooling Modes and Data Path`, `Load Balancing and Replica Selection`, `Hand Written Deserialize and Known Fields`, `Finding Codes and One Finding Rule`, `Scope 002 Brief Rules`, `Scope 001 Review Record and Verdicts`, `Status Lifecycle and Feature 1 State`, `Exit Status and Output Convention`, `Exit Status and Output Convention`, `Scaffold Step and dev-f1-build Gate`, `Architecture Spec and Repository Layout`, `Proposed Stack and Deferred Features`, `Linux Target and VM Architecture`, `Query Cache PRD`, `Scope 002 Follow-ups from Spec 0001`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Why does `Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5` connect `Scope 001 Review Record and Verdicts` to `Offline Bounds, Parser Depth and Known Limits`, `Status Lifecycle and Feature 1 State`, `Exit Status and Output Convention`, `Documentation Index, Handoffs and Readiness`, `Scaffold Step and dev-f1-build Gate`, `Slices, Milestones and Readiness Gates`, `Review Clarifications and check-config Usage Rules`, `Spec 0001 Acceptance Criteria`, `Column Counting Rule and Tests`, `IoFinding and Permission Denied Path`, `Project Guidance, Changelog and Status Lifecycle`, `Hand Written Deserialize and Known Fields`, `Finding Codes and One Finding Rule`, `Workspace Layout, Bounded Read and FIFO Limit`, `Verify Record First Run and Toolchain`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Protected assets and threat actors` to the rest of the system?**
  _60 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `check-config Integration Tests` be split into smaller, more focused modules?**
  _Cohesion score 0.06945863125638406 - nodes in this community are weakly interconnected._
- **Should `pool-config Validation Tests` be split into smaller, more focused modules?**
  _Cohesion score 0.06060606060606061 - nodes in this community are weakly interconnected._