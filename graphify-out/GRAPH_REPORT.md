# Graph Report - pool  (2026-11-14)

## Corpus Check
- 91 files · ~91,218 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1291 nodes · 3832 edges · 51 communities
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 745 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- check-config Integration Tests
- pool-config Validation Tests
- Scope 002 Brief Rules
- Pgpool Baseline, Competitors and Feasibility
- Linux Build Evidence, AC-2 and VM Architecture
- Protocol Contract and Driver Compatibility
- Slices, Milestones and Readiness Gates
- Standards, Tooling and Release Qualification
- pool CLI Entry and Argument Parsing
- Safety Invariants and Session Risks
- Scaffold Step and dev-f1-build Gate
- Feature Inventory and Parity Matrix
- Architecture Spec and Repository Layout
- Leases, Cancellation and Login Routing
- Workspace Layout, Bounded Read and FIFO Limit
- Linux Target, VM Architecture and Follow-up 11
- Scope 001 Coordination Log and Closing
- IoFinding and Permission Denied Path
- Glossary and Routing Profiles
- Scope 001 Coordination Brief and Sub Sessions
- Scope 001 Coordination Brief, Log and Authorization
- Review Clarifications and check-config Usage Rules
- Session Model, Pooling Modes and Data Path
- Operation Journal Records and Generations
- Feature Families and Planned Scopes
- Invariants, Secrets and Administration Rules
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Failover Records and Fencing Authority
- Report Writers, Human and JSON
- Scope 002 Coordination Log and Sub Sessions
- main.rs Entry, Bounded Read and Stdout
- Administration, Cluster and Health PRDs
- Protocol Contract and Driver Compatibility: Driver compatibility matrix
- Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling
- pool-config Validation Library
- Agent Workflow and Graph Refresh
- Linux Build Run, Manual Steps F1-1 and F1-2
- Spec 0001 Status and ADR 0001
- Workspace Conventions and dev-f2-audit
- Scope 001 Review Record and Verdicts
- Offline Bounds, Parser Depth and Known Limits
- Load Balancing and Replica Selection
- Administration, Cluster and Health PRDs: Administration PRD
- Safe Rust, Bounded Resources and Dependencies
- Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f
- Virtual Cancellation Model
- Feature Families and Planned Scopes: High availability PRD
- SQL Routing and Replica Profiles: Protocol engine
- Offline Bounds, Parser Depth and Known Limits: Rules made clearer in the review, each checked o
- Column Counting Rule and Tests

## God Nodes (most connected - your core abstractions)
1. `Spec 0001. POOL initial architecture` - 60 edges
2. `Glossary` - 59 edges
3. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 59 edges
4. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 51 edges
5. `Feature inventory and parity matrix` - 45 edges
6. `Testing strategy` - 45 edges
7. `Scope: POOL, order and status index` - 44 edges
8. `1. Stack and workspace (done)` - 43 edges
9. `AC-2 Linux build (container check ends with 'linux build check passed')` - 43 edges
10. `AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read)` - 39 edges

## Surprising Connections (you probably didn't know these)
- `Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing)` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
- `Reading: JSON field order is checked on the raw line (agreed, also done byte for byte)` --references--> `ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order()`  [INFERRED]
  docs/specs/0001-initial-architecture/verify.md → apps/pool/tests/check_config.rs
- `io.permission_denied test under cfg(unix): file of mode 000; returns as passed when the process can still open it (root in the container), else checks exit 3 and exact output in both formats` --references--> `ac11_a_file_that_cannot_be_opened_is_permission_denied()`  [INFERRED]
  docs/specs/0001-initial-architecture/index.md → apps/pool/tests/check_config.rs
- `Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS` --references--> `ac11_a_path_that_is_not_utf8_is_used_as_given()`  [EXTRACTED]
  docs/reviews/2026-11-14-scope-001-stack-and-workspace.md → apps/pool/tests/check_config.rs
- `--format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument` --references--> `ac11_arguments_that_are_not_utf8_anywhere_else_are_unexpected()`  [INFERRED]
  docs/specs/0001-initial-architecture/index.md → apps/pool/tests/check_config.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **CHANGELOG Unreleased Added entries for the scope 001 workspace slice (workspace, check-config, config contract, finding codes, JSON, exit status, help and version, fixtures and tests, docs, conventions)** — changelog_unreleased_added, changelog_added_workspace, changelog_added_check_config, changelog_added_config_contract, changelog_added_finding_codes, changelog_added_json_report, changelog_added_exit_status, changelog_added_help_version, changelog_added_fixtures_tests, changelog_added_docs, changelog_added_shared_conventions [EXTRACTED 1.00]
- **After closing: Docker on, Linux build run by chk-f1-verify-2 and the lead, feature 1 done and spec Accepted, scope 002 shares the checkout** — docs_scope_coordination_001_stack_and_workspace_log_decision_docker_on_linux_build_runs, docs_scope_coordination_001_stack_and_workspace_log_decision_feature_1_done_spec_accepted, docs_scope_coordination_001_stack_and_workspace_log_decision_scope_002_shares_checkout, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify_2, docs_scope_coordination_001_stack_and_workspace_log_history_after_closing, docs_scope_coordination_001_stack_and_workspace_log_closing_after_closing, docs_scope_coordination_001_stack_and_workspace_log_closing_remaining_blockers [EXTRACTED 1.00]
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
- **Linux build evidence closes feature 1: container build PASS proves AC-2 and the /dev/full clause of AC-11, manual F1-1 and F1-2 PASS, feature 1 done, spec 0001 Accepted** — docs_specs_0001_initial_architecture_verify_linux_evidence, docs_specs_0001_initial_architecture_verify_linux_evidence_verdict, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_11, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build, docs_scope_001_stack_and_workspace, docs_specs_0001_initial_architecture_index_status_in_progress [INFERRED 0.85]
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off, docs_scope_coordination_001_stack_and_workspace_manual_f1_2_run_linux_build [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]
- **Context files written by the 002 /audit step** — agents, apps_pool_agents, apps_pool_claude, crates_pool_config_agents, crates_pool_config_claude [EXTRACTED 1.00]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **Scope 002 wave 1 sub sessions** — docs_scope_coordination_002_standards_and_tooling_log_arc_f2_spec, docs_scope_coordination_002_standards_and_tooling_log_dev_f2_audit, docs_scope_coordination_002_standards_and_tooling_log_dev_f2_build, docs_scope_coordination_002_standards_and_tooling_log_chk_f2_verify, docs_scope_coordination_002_standards_and_tooling_log_tst_f2_test, docs_scope_coordination_002_standards_and_tooling_log_chk_f2_review [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **Scope 001 shared conventions (configuration keys, exit status and output, log events)** — docs_engineering_standards_configuration_keys_convention, docs_engineering_standards_exit_status_output_convention, docs_engineering_standards_log_event_convention, docs_specs_0001_initial_architecture_index_configuration_rules, docs_specs_0001_initial_architecture_index_exit_status_and_output, docs_specs_0001_initial_architecture_index_log_events_and_redaction [EXTRACTED 1.00]
- **Scope 002 steps blocked on the user** — docs_scope_coordination_002_standards_and_tooling_manual_f2_1, docs_scope_coordination_002_standards_and_tooling_manual_f2_2, docs_scope_coordination_002_standards_and_tooling_manual_f2_n1, docs_scope_coordination_002_standards_and_tooling_log_open_blockers [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **No untrusted or secret text reaches output or telemetry** — apps_pool_agents_constant_output_rule, crates_pool_config_agents_rejection_no_file_text, apps_pool_agents_canary_leak_regression, agents_no_secrets_in_default_telemetry [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (51 total, 0 thin omitted)

### Community 0 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (86): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+78 more)

### Community 1 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (56): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+48 more)

### Community 2 - "Scope 002 Brief Rules"
Cohesion: 0.24
Nodes (13): Context files (nested AGENTS.md for apps/pool and crates/pool-config), apps/pool agent guide, Leaking fake canary CANARY-9f2c7b1e into output is a security regression (SEC-11), pool check-config command (reads one file, calls pool_config::validate, prints a report, exits; no socket, thread or async runtime), Exit status 0/1/2/3 contract; exit 101 (panic) is always a defect; failed write becomes exit 3, Platform gated tests (/dev/zero and mode 000 on unix, /dev/full on Linux only), Claude Code pointer to apps/pool/AGENTS.md, crates/pool-config agent guide (+5 more)

### Community 3 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.06
Nodes (45): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+37 more)

### Community 4 - "Linux Build Evidence, AC-2 and VM Architecture"
Cohesion: 0.13
Nodes (42): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Added: spec 0001 build notes, AC-1 to AC-13, five step build plan, verification record and independent review; Linux container build (AC-2) passed for x86_64-unknown-linux-gnu under emulation, image digest and binary checksum in the verification record, Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md, Feature 1 is done; its Linux build evidence is in the verification record; next queued feature is 2 (standards and tooling), then the first walking slice, Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Linux build (0001 AC-2) and the two Linux only tests passed at 18e9228; record under Linux evidence: container build in verify.md; evidence for x86_64 under emulation, Decision: feature 1 marked done and spec 0001 Accepted (all thirteen AC have a PASS record, Done when builds on Linux met for x86_64 under emulation); lead changed scope 001, scope index, spec Status, CLAUDE.md, CHANGELOG.md and the manual file; older verify.md boxes kept as history; Follow-up 7 (Start Docker Desktop) now stale, left for the next architect update (+34 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.08
Nodes (41): Invariant: promotion requires a reviewed operation and independently verified fencing, Dependency and Configuration Change Rules, Rust and Runtime Standards, Security and Review Requirements, F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-107 Rate and resource limits (admission, frame and auth CPU budgets), Configuration Generation (+33 more)

### Community 6 - "Slices, Milestones and Readiness Gates"
Cohesion: 0.08
Nodes (45): Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), Changelog (CHANGELOG.md; Keep a Changelog 1.1.0 format, Semantic Versioning 2.0.0), Changelog format: Keep a Changelog 1.1.0 and Semantic Versioning 2.0.0, Claude Code guidance (CLAUDE.md), Cargo checks run by hand today (cargo fmt --all --check, clippy --locked -D warnings, test --locked, single test, check-config on valid-minimal.toml); CI arrives with scope 002, Working with the Docs (+37 more)

### Community 7 - "Standards, Tooling and Release Qualification"
Cohesion: 0.13
Nodes (26): Tests and Evidence Standard, Slice, Verification Record, Walking Slice (glossary term), Milestones, V1 Release Gate, R-25 Reviewer or lab unavailable at a gate, criterion 0.8.2 (microbenchmarks) (+18 more)

### Community 8 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.23
Nodes (13): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), Test counts on the merged tree (milestone 5 Test it): 134 passed on macOS (pool unit 3, check_config 66, pool-config unit 3, validate 60, doc tests 2); 31 developer, 96 tester, 7 review fixes (+5 more)

### Community 9 - "Safety Invariants and Session Risks"
Cohesion: 0.10
Nodes (26): Session Model (pinned backend lease), Data and Protocol Safety Standards, No Replay After Dispatch Rule, F-011 Prepared statements (session ownership, no cross backend replay), F-024 Transaction Mode (refused, session pooling only), F-030 Validation and reset (close uncertain sessions), F-033 SQL AST classification (PostgreSQL 18 grammar adapter), F-045 Unknown and failed parsing (keep primary lease, reject on restricted replica) (+18 more)

### Community 10 - "Scaffold Step and dev-f1-build Gate"
Cohesion: 0.17
Nodes (19): Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, Box ticked: Verify it (/check verify stack and workspace), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test), Sub session chk-f1-verify continued (reverify on the merged tree after the review fixes, closes Minor 6; report blocked, commit cf928e7: revision b6ce165, 11 AC PASS, 134 tests, 398 probe checks; AC-2 and /dev/full clause BLOCKED), Decision: fix round merge 4ebcf93, review log b6ce165; re-review on 02bdcbe..4ebcf93 closes the Major (37 probe files), Minor 1 to 4, Minor 5 in part (Follow-up 11), Minor 6 by chk-f1-verify, 10 of 11 Nits; five optional new Nits routed, Row 10 evidence: from the second verification pass, checked on a debug pool at b6ce165 (macOS, Rust 1.100.0): zzz/aaa reports line 3, reversed reports line 2, unknown key in [result_cache] with wrong type in [automatic_promotion] reports the [automatic_promotion] line, two syntax errors on lines 2 and 3 report line 2, Full verify: /check verify stack and workspace (checker chk-f1-verify, 2026-11-14, rerun from its own checkout against the real pool binary) (+11 more)

### Community 11 - "Feature Inventory and Parity Matrix"
Cohesion: 0.09
Nodes (33): Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-065 Primary Identification, F-065 Primary identification (durably approved writer with fresh evidence), F-066 Topology reconciliation (durable writer/epoch/quarantine) (+25 more)

### Community 12 - "Architecture Spec and Repository Layout"
Cohesion: 0.42
Nodes (13): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+5 more)

### Community 13 - "Leases, Cancellation and Login Routing"
Cohesion: 0.10
Nodes (33): Tracer Bullet Build Approach, Added: three shared conventions in the engineering standards (configuration keys, exit status with command output, log event fields with redaction), Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Log event fields and redaction convention (ts, level, event; redaction by type), Shared Conventions Table, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction (+25 more)

### Community 14 - "Workspace Layout, Bounded Read and FIFO Limit"
Cohesion: 0.05
Nodes (128): ac12_dev_null_is_an_empty_file_that_misses_config_version(), Added: pool check-config --config <PATH> [--format human|json], offline check before start; opens no socket, starts no thread, reads at most 1 MiB plus one byte, Added: first configuration contract (config_version = 1 plus reserved result_cache and automatic_promotion tables; absent or enabled = false is valid, asking for either is config.unsupported_capability, SEC-10), Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish), Added: fixed finding codes (config.syntax, config.schema, config.missing_key, config.unsupported_version, config.too_large, config.not_utf8) with line and column; output from constant text, key names and positions only, so no secret appears (SEC-11), Added: fifteen configuration fixtures and 134 automated tests covering every first build acceptance criterion on the workstation; 136 on Linux because two need /dev/full, Added: pool --help and pool --version, Added: JSON report (--format json), one line with a fixed field order, for scripts (+120 more)

### Community 15 - "Linux Target, VM Architecture and Follow-up 11"
Cohesion: 0.18
Nodes (21): No CI yet (arrives with scope 002); Linux container build per spec 0001; later property, PostgreSQL integration and compatibility/fuzz/chaos tests; do not invent passing results, 2. Standards and tooling (planned), Open blocker: proxy and PostgreSQL VM architecture (x86_64 or ARM) unconfirmed for 0001 AC-2; x86_64 target inferred (F1-N1), Closing: all green branches merged; Document and /sync recorded; closing commit docs(scope): close 001-stack-and-workspace coordination; pushed 2026-11-14; five sub sessions archived and cleaned up; nothing retained; after closing chk-f1-verify-2 merged and feature 1 done; remaining blockers VM architecture (F1-N1), Follow-up 11 waits for /scope, Closing remaining blockers: VM architecture (F1-N1, x86_64 assumed, Linux evidence only for x86_64); Follow-up 11 of spec 0001 waits for /scope; nothing holds the feature 1 status, Decision: after the fix merge chk-f1-verify reverifies into verify.md, chk-f1-review confirms the fix diff; stale CLAUDE.md and AGENTS.md tidied by /sync at close; spec 0001 Follow-up 11 copied to scope 002 at close, Decision: spec 0001 Follow-up 11 (CI checks for AC-12 and AC-13) not yet copied into scope 002; adding sub tasks belongs to /scope and waits for user approval, Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; awaiting user VM architecture (+13 more)

### Community 16 - "Scope 001 Coordination Log and Closing"
Cohesion: 0.13
Nodes (28): POOL agent guide (AGENTS.md), POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust; the repository holds the planning package and the scope 001 workspace slice (apps/pool, crates/pool-config, offline pool check-config), not an implemented proxy, User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested, Evidence: verification record, independent review and changelog, Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Graph update ownership: sub sessions never touch graphify-out, lead refreshes it in the merge commit, Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only), Log koordinasi: 001-stack-and-workspace (coordination log) (+20 more)

### Community 17 - "IoFinding and Permission Denied Path"
Cohesion: 0.26
Nodes (9): IoFinding, Self, Minor 3: the io.permission_denied arm of IoFinding::from_kind (report.rs:30) has no test, although the workstation suite runs as a user that is not root, io.permission_denied test under cfg(unix): file of mode 000; returns as passed when the process can still open it (root in the container), else checks exit 3 and exact output in both formats, Tests and who owns them, Clarification 5: a self-skipping cfg(unix) mode 000 test for io.permission_denied; runner up keep it untested, io.permission_denied has no test, as the spec says, Rule check: a mode 000 file exits 3 io.permission_denied in both formats; new test ac11_a_file_that_cannot_be_opened_is_permission_denied passed (+1 more)

### Community 18 - "Glossary and Routing Profiles"
Cohesion: 0.14
Nodes (27): Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Approved Writer, Backend Login, Credential Generation, Detach (+19 more)

### Community 19 - "Scope 001 Coordination Brief and Sub Sessions"
Cohesion: 0.15
Nodes (24): Spec 0001 lifecycle status mirrors scope feature 1: In Progress while the foundation is built and proven, Accepted when the feature is done, 1. Stack and workspace (done), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Box ticked: Document it (/document stack and workspace), Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Spec 0001 build notes (adopted crates, build plan, ID'd criteria, fixture and Linux build commands) (+16 more)

### Community 20 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.14
Nodes (26): Aturan umum brief: 001-stack-and-workspace (general brief rules), Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first), Always forbidden paths for sub sessions (docs/scope, docs/prd, AGENTS.md, CLAUDE.md, graphify-out, .claude, spec content outside ownership), Role files written for another project (Stackly references do not apply) (+18 more)

### Community 21 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.33
Nodes (12): Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, --format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument, Value sourcing (where each output value comes from), Clarification 2: --format=<bytes> and --config=<bytes> are unexpected argument; runner up byte level split, For /check review: notes handed to the review (+4 more)

### Community 22 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.15
Nodes (18): Connection pooling, Pool key (pool identity), Pool limits and operations, Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Transaction pooling (deferred), Durable approved writer authority, Initial admission boundary (+10 more)

### Community 23 - "Operation Journal Records and Generations"
Cohesion: 0.27
Nodes (13): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Observation record, Operation record, Local SQLite operation journal (single writer), Expected Generation (+5 more)

### Community 24 - "Feature Families and Planned Scopes"
Cohesion: 0.23
Nodes (14): Tracer Bullet build approach, Walking Slice Milestone, R-24 Walking slice exceeds its estimate, Roadmap, Delivery slices, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy (+6 more)

### Community 25 - "Invariants, Secrets and Administration Rules"
Cohesion: 0.18
Nodes (12): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: fail closed on unknown role or unsafe routing, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Invariant: preserve transaction and session ownership, Invariant: keep writes on the verified primary (+4 more)

### Community 26 - "Rust Technology Evaluation"
Cohesion: 0.08
Nodes (39): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy, Linux builder image rust:1.100.0-slim-bookworm by index digest (+31 more)

### Community 27 - "SQL Routing and Replica Profiles"
Cohesion: 0.18
Nodes (17): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table (+9 more)

### Community 28 - "Failover Records and Fencing Authority"
Cohesion: 0.19
Nodes (16): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+8 more)

### Community 29 - "Report Writers, Human and JSON"
Cohesion: 0.31
Nodes (9): JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, write_human(), write_json() (+1 more)

### Community 30 - "Scope 002 Coordination Log and Sub Sessions"
Cohesion: 0.22
Nodes (11): Reviewer roster (who holds each review role, where a fresh model may review), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply), Verify the required checks on a pull request and link the result (open), Coordination log: 002-standards-and-tooling, arc-f2-spec (architect, writes spec 0002), chk-f2-verify (checker verify), dev-f2-build (developer, builds checks from spec 0002), Seven documentation mismatches from dev-f2-audit forwarded to arc-f2-spec as spec 0002 AC and Build plan input (+3 more)

### Community 31 - "main.rs Entry, Bounded Read and Stdout"
Cohesion: 0.38
Nodes (9): check_config(), main(), read_bounded(), Result, Vec, write_stdout(), Changed since 5f44d88: lib.rs, cli.rs, main.rs, the two test files, spec index.md and rationale.md; manifests, Cargo.lock, toolchain, .gitignore and fixtures unchanged, ExitCode (+1 more)

### Community 32 - "Administration, Cluster and Health PRDs"
Cohesion: 0.18
Nodes (18): F-060 Network partitions (health changes eligibility only), F-101 TLS and mTLS verification, Health features F-054 to F-060, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control (+10 more)

### Community 33 - "Protocol Contract and Driver Compatibility: Driver compatibility matrix"
Cohesion: 0.07
Nodes (32): Invariant: close uncertain pooled state, Separate Frontend and Backend Identity, Support Profile, R-11 Authentication or role confusion, Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Protocol compatibility contract (3.0 and 3.2) (+24 more)

### Community 34 - "Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling"
Cohesion: 0.15
Nodes (16): Brief general rules: 002-standards-and-tooling, Allowed downloads and installs (crates adopted by spec 0002, pinned toolchain, cargo-deny, cargo-fuzz), Decision mode auto, Disk budget rule (max 2 concurrent Rust builds, delete target before done), GitHub read-only rule for sessions, Sub sessions never update graphify-out; lead updates graph in merge commit, Manual steps policy (Docker, GitHub settings, push, PRs run only by user), Scope 002 boundary: align written rules with workspace and install required checks (+8 more)

### Community 35 - "pool-config Validation Library"
Cohesion: 0.08
Nodes (36): capability_state(), CapabilityState, error_location(), Finding, FindingCode, KnownField, Location, position() (+28 more)

### Community 36 - "Agent Workflow and Graph Refresh"
Cohesion: 0.22
Nodes (9): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Agent Workflow, Cargo workspace holds only the pool and pool-config packages, and there is no CI yet (inspect it before relying on documentation), Engineering Workflow, chk-f2-review (checker review, fresh model), Agreement with scope 001 lead to alternate commits on shared main checkout (per-path git add, no concurrent graph updates) (+1 more)

### Community 37 - "Linux Build Run, Manual Steps F1-1 and F1-2"
Cohesion: 0.31
Nodes (15): Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user), Resolved former blocker: Linux build evidence blocked while the Docker daemon was off; user started Docker Desktop (F1-1 PASS), Resolved former blocker: 0001 AC-2 and the /dev/full clause of AC-11 not run; Linux build passed at 18e9228 with 136 tests (F1-2 PASS), no longer holds feature 1 done, Checked in wave 1: F1 Document it (lead, CHANGELOG.md range a2dca30..HEAD, real commands in CLAUDE.md, CONTRIBUTING.md, README.md; commit docs(changelog); manual F1-1, F1-2 NOT_RUN), Sub session chk-f1-verify-2 (checker verify after closing: Linux build evidence for AC-2 and the /dev/full clause of AC-11; report done, commit 972e160: revision 18e9228, exit 0, host x86_64-unknown-linux-gnu, 136 tests (3 + 68 + 3 + 60 + 2), two Linux only tests ok, 30 fixture blocks, no canary leaked; merged to main), Decision: user wrote Docker sudah nyala (F1-1); lead ran the spec 0001 Linux build block from main at 18e9228 (exit 0, 136 tests, linux build check passed, sha256 0481be18...f7bdb3); new session chk-f1-verify-2 ran the same block itself and recorded it in verify.md with the same checksum and test count (+7 more)

### Community 38 - "Spec 0001 Status and ADR 0001"
Cohesion: 0.43
Nodes (8): ADR 0001. Initial architecture, Decision: Build plan step 1 sets spec Status to In Progress; the stale 'lifecycle status remains Proposed' wording is tidied at /sync, not by sub sessions, Spec 0001. POOL initial architecture, Follow-up 1: foundation and decision ledger ratified; the build of scope 001 was requested on 2026-11-14; every later scope stays queued until requested, Modular daemon with independent CLI decision, Shared process failure domain, Decision status sentence: ratification settles the content; the Status line follows scope feature 001 per the status lifecycle, Clarification 9: Decision status sentence and Follow-up 1 reworded to hold in every status; runner up leave to the lead at close

### Community 39 - "Workspace Conventions and dev-f2-audit"
Cohesion: 0.40
Nodes (6): Workspace conventions (inherited workspace.package fields, workspace lints with unsafe_code forbid, single workspace.dependencies, committed Cargo.lock, no sockets/threads/runtime, cargo test harness only, commits name Build plan step), Dependency declared once under workspace.dependencies, taken only when the current path uses it; Cargo.lock committed (0001 AC-12), Arguments parsed by hand with the standard library (no parser crate), Capture conventions: /audit (checked), Lead gate for dev-f2-audit merge (clean worktree, 6-file diff, 0 deletions, links/anchors present, sampled claims match manifest), dev-f2-audit (developer, /audit conventions; done and merged)

### Community 40 - "Scope 001 Review Record and Verdicts"
Cohesion: 0.18
Nodes (19): Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), First review nits (11): --format=<bytes> equals form, --help=x after check-config, u64::MAX read fallback, FIFO blocks, duplicated value block, capability_state comment, repeated numbers in messages, three weak tests, stale status sentence, Second review run at 4ebcf93: fmt and clippy exit 0, tests 3 + 66 + 3 + 60 + 2 passed, 18 lock packages, AC-12 grep empty; Linux build not run, Second review, fix round at 4ebcf93 (range 02bdcbe..4ebcf93; code a705659, 5a99563, b18b288, 453ae73; spec 5f1e995; same reviewer session), Spec text against the code: the nine clarifications in index.md and the rationale table match the fixed binary; AC-1 to AC-13 unchanged, Review summary: two package workspace and offline pool check-config follow spec 0001; output from constants only, bounded read, enabled = true refused in every spelling (+11 more)

### Community 41 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.30
Nodes (12): ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Clarification 3: --help=x/-h=x after check-config print help, --version/-V there are unknown option, all four =x forms first are unknown command; runner up usage error for =x, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check, Clarification 6: exact column checked for one unambiguous made input with a multi-byte character, others keep at least 1; runner up exact columns everywhere (+4 more)

### Community 42 - "Load Balancing and Replica Selection"
Cohesion: 0.16
Nodes (20): Policy Version, pg_query parser with PostgreSQL 18 grammar, SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), No primary fallback in V1, Primary profile sessions stay pinned (+12 more)

### Community 43 - "Administration, Cluster and Health PRDs: Administration PRD"
Cohesion: 0.10
Nodes (29): Invariant: health failure does not authorize promotion, Authoritative barrier across every write path, Axum REST administration API plus CLI, No silent replay of uncertain work (lost COMMIT response), pool check-config, GET /v1/nodes node status route, Health failure alone produces no promotion, Cluster management PRD (+21 more)

### Community 44 - "Safe Rust, Bounded Resources and Dependencies"
Cohesion: 0.50
Nodes (4): Safe first party Rust and bounded resources, Workspace lint unsafe_code = forbid applied to every member, Bounded file read: at most MAX_CONFIG_BYTES plus one byte via Read::take, Dependencies serde and toml only; toml feature unbounded kept off so nesting limit protects the stack

### Community 46 - "Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f"
Cohesion: 0.07
Nodes (37): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Manual failover first; automatic failover and cache disabled, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, Independent review resolutions (seven approved fixes) (+29 more)

### Community 47 - "Virtual Cancellation Model"
Cohesion: 0.15
Nodes (21): Virtual Cancellation Model, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Data plane, Frontend owner task, No automatic retry after dispatch, Architecture overview, Application path sequence (+13 more)

### Community 54 - "Feature Families and Planned Scopes: High availability PRD"
Cohesion: 0.17
Nodes (16): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), Fence Receipt, V1.5 Milestone, R-21 Proxmox host correlated failure, High availability PRD, ControllerEpoch, Endpoint failover via external load balancing or controlled VIP (+8 more)

### Community 58 - "SQL Routing and Replica Profiles: Protocol engine"
Cohesion: 0.17
Nodes (12): Future multi proxy deployment, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Versioned local admin REST API (Axum), External etcd for future coordination (+4 more)

### Community 60 - "Offline Bounds, Parser Depth and Known Limits: Rules made clearer in the review, each checked o"
Cohesion: 0.29
Nodes (8): ac11_a_first_argument_that_is_not_a_command_is_unknown(), First argument compared whole: --help=x, -h=x, --version=x and -V=x as the first argument are unknown command, Rules made clearer in the review, each checked on the binary (17 rows, all PASS), Rule check: --format=<bytes> and --config=<bytes> not UTF-8 exit 2 unexpected argument; --format <bytes> as separate value is wrong format value, Rule check: --help=x and -h=x after check-config print help; the first argument is compared whole; --version and -V are unknown option after check-config, Rule check: a long array holding the canary 2000 times gives one config.schema and no canary in output, Debug or Display, Rule check: the read bound fallback is 1_048_577, not u64::MAX; /dev/zero ends with config.too_large, Rule check: the schema step ends the run (config_version = 2 with result_cache = [true] gives one config.schema only)

### Community 61 - "Column Counting Rule and Tests"
Cohesion: 0.70
Nodes (5): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way), Exact column test for a made input with a multi-byte character before the error: x = "éé" junk on line 2 is config.syntax at column 10 (a byte count gives 12), Rule check: the column counts characters (x = "éé" junk is line 2, column 10; bytes would give 12)

## Knowledge Gaps
- **64 isolated node(s):** `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `F-060 Network partitions (health changes eligibility only)`, `Decision mode auto`, `Session model per role (architect/checker fable-5-2, developer/tester opus-5-5)` (+59 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Spec 0001. POOL initial architecture` connect `Spec 0001 Status and ADR 0001` to `Scope 002 Brief Rules`, `Linux Build Evidence, AC-2 and VM Architecture`, `Slices, Milestones and Readiness Gates`, `Safety Invariants and Session Risks`, `Architecture Spec and Repository Layout`, `Leases, Cancellation and Login Routing`, `Workspace Layout, Bounded Read and FIFO Limit`, `Linux Target, VM Architecture and Follow-up 11`, `Scope 001 Coordination Log and Closing`, `Glossary and Routing Profiles`, `Scope 001 Coordination Brief and Sub Sessions`, `Session Model, Pooling Modes and Data Path`, `Operation Journal Records and Generations`, `Invariants, Secrets and Administration Rules`, `Rust Technology Evaluation`, `Scope 002 Coordination Log and Sub Sessions`, `Administration, Cluster and Health PRDs`, `Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling`, `Workspace Conventions and dev-f2-audit`, `Scope 001 Review Record and Verdicts`, `Load Balancing and Replica Selection`, `Administration, Cluster and Health PRDs: Administration PRD`, `Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f`, `Virtual Cancellation Model`, `Feature Families and Planned Scopes: High availability PRD`, `SQL Routing and Replica Profiles: Protocol engine`?**
  _High betweenness centrality (0.231) - this node is a cross-community bridge._
- **Why does `First Handoff: the Workspace` connect `Workspace Layout, Bounded Read and FIFO Limit` to `Linux Build Evidence, AC-2 and VM Architecture`, `Protocol Contract and Driver Compatibility`, `Slices, Milestones and Readiness Gates`, `Linux Build Run, Manual Steps F1-1 and F1-2`, `Feature Inventory and Parity Matrix`, `Architecture Spec and Repository Layout`, `Leases, Cancellation and Login Routing`, `Linux Target, VM Architecture and Follow-up 11`, `Scope 001 Coordination Log and Closing`, `Scope 001 Coordination Brief, Log and Authorization`?**
  _High betweenness centrality (0.085) - this node is a cross-community bridge._
- **Why does `1. Stack and workspace (done)` connect `Scope 001 Coordination Brief and Sub Sessions` to `Linux Build Evidence, AC-2 and VM Architecture`, `Agent Workflow and Graph Refresh`, `Slices, Milestones and Readiness Gates`, `Spec 0001 Status and ADR 0001`, `Scope 001 Review Record and Verdicts`, `Linux Build Run, Manual Steps F1-1 and F1-2`, `Scaffold Step and dev-f1-build Gate`, `Standards, Tooling and Release Qualification`, `Leases, Cancellation and Login Routing`, `Workspace Layout, Bounded Read and FIFO Limit`, `Linux Target, VM Architecture and Follow-up 11`, `Scope 001 Coordination Log and Closing`, `Scope 001 Coordination Brief, Log and Authorization`?**
  _High betweenness centrality (0.079) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` (e.g. with `Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish)` and `Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary`) actually correct?**
  _`AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `F-060 Network partitions (health changes eligibility only)` to the rest of the system?**
  _64 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `check-config Integration Tests` be split into smaller, more focused modules?**
  _Cohesion score 0.06945863125638406 - nodes in this community are weakly interconnected._