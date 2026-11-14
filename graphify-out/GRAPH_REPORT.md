# Graph Report - pool  (2026-11-14)

## Corpus Check
- 7 files · ~94,037 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1326 nodes · 3979 edges · 49 communities
- Extraction: 81% EXTRACTED · 19% INFERRED · 0% AMBIGUOUS · INFERRED: 766 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- check-config Integration Tests
- pool-config Validation Tests
- Scope 002 Brief Rules
- Pgpool Baseline, Competitors and Feasibility
- Linux Build Evidence, AC-2 and VM Architecture
- Security Requirements and Features F-100 to F-108
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
- Changelog, Feature 1 Done and Spec Status
- IoFinding and Permission Denied Path
- Glossary and Routing Profiles
- Scope 001 Coordination Brief and Sub Sessions
- Scope 001 Coordination Brief, Log and Authorization
- Review Clarifications and check-config Usage Rules
- Session Model, Pooling Modes and Data Path
- Operation Journal Records and Generations
- Feature Families and Planned Scopes
- Hand Written Deserialize and Reserved Capability Probes
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Failover Records and Fencing Authority
- Pgpool Configuration Coverage Inventory
- Configuration Keys and Secret References
- Feasibility Study and Option Families
- Testing Strategy, CI Progression and Slices
- Protocol Contract and Driver Compatibility: Driver compatibility matrix
- Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling
- pool-config Validation Library
- Competitor Analysis and Benchmark Method
- Protocol Contract and Driver Compatibility
- Tracing, Log Redaction and Logging Decisions
- rustls TLS Provider
- Scope 001 Review Record and Verdicts
- Offline Bounds, Parser Depth and Known Limits
- Load Balancing and Replica Selection
- Administration, Cluster and Health PRDs: Administration PRD
- Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f
- Virtual Cancellation Model
- Manual Failover, Fencing and Promotion Plan
- SQL Routing and Replica Profiles: Protocol engine
- Column Counting Rule and Tests

## God Nodes (most connected - your core abstractions)
1. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 63 edges
2. `Spec 0001. POOL initial architecture` - 61 edges
3. `Glossary` - 59 edges
4. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 56 edges
5. `AC-2 Linux build (container check ends with 'linux build check passed' and shows host aarch64-unknown-linux-gnu)` - 53 edges
6. `Feature inventory and parity matrix` - 45 edges
7. `Testing strategy` - 45 edges
8. `1. Stack and workspace (done)` - 45 edges
9. `Scope: POOL, order and status index` - 44 edges
10. `AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read)` - 39 edges

## Surprising Connections (you probably didn't know these)
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md
- `Where a Change Belongs (documentation ownership table)` --semantically_similar_to--> `Documentation ownership model (scope, specs, PRDs, ADRs, research)`  [INFERRED] [semantically similar]
  docs/index.md → AGENTS.md
- `Extra runs beyond the spec: mode 000 file exits 3 io.permission_denied; closed stdout or stderr pipe exits 3 with no output and no panic (not the /dev/full case)` --references--> `ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic()`  [INFERRED]
  docs/specs/0001-initial-architecture/verify.md → apps/pool/tests/check_config.rs
- `Extra runs beyond the spec: mode 000 file exits 3 io.permission_denied; closed stdout or stderr pipe exits 3 with no output and no panic (not the /dev/full case)` --references--> `ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic()`  [INFERRED]
  docs/specs/0001-initial-architecture/verify.md → apps/pool/tests/check_config.rs
- `io.permission_denied test under cfg(unix): file of mode 000; returns as passed when the process can still open it (root in the container), else checks exit 3 and exact output in both formats` --references--> `ac11_a_file_that_cannot_be_opened_is_permission_denied()`  [INFERRED]
  docs/specs/0001-initial-architecture/index.md → apps/pool/tests/check_config.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **CHANGELOG Unreleased Added entries for the scope 001 workspace slice (workspace, check-config, config contract, finding codes, JSON, exit status, help and version, fixtures and tests, docs, conventions)** — changelog_unreleased_added, changelog_added_workspace, changelog_added_check_config, changelog_added_config_contract, changelog_added_finding_codes, changelog_added_json_report, changelog_added_exit_status, changelog_added_help_version, changelog_added_fixtures_tests, changelog_added_docs, changelog_added_shared_conventions [EXTRACTED 1.00]
- **After closing: Docker on, Linux build run by chk-f1-verify-2 and the lead, feature 1 done and spec Accepted, scope 002 shares the checkout** — docs_scope_coordination_001_stack_and_workspace_log_decision_docker_on_linux_build_runs, docs_scope_coordination_001_stack_and_workspace_log_decision_feature_1_done_spec_accepted, docs_scope_coordination_001_stack_and_workspace_log_decision_scope_002_shares_checkout, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify_2, docs_scope_coordination_001_stack_and_workspace_log_history_after_closing, docs_scope_coordination_001_stack_and_workspace_log_closing_after_closing, docs_scope_coordination_001_stack_and_workspace_log_closing_remaining_blockers [EXTRACTED 1.00]
- **Target ARM: F1-N1 answered, arc-f1-update-2 moves the Linux target to aarch64, chk-f1-verify-3 records the aarch64 evidence, feature 1 stays done** — docs_scope_coordination_001_stack_and_workspace_log_decision_f1_n1_arm_target, docs_scope_coordination_001_stack_and_workspace_log_decision_feature_1_stays_done, docs_scope_coordination_001_stack_and_workspace_log_decision_checker_merges_architect_branch, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update_2, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify_3, docs_scope_coordination_001_stack_and_workspace_log_history_target_arm, docs_scope_coordination_001_stack_and_workspace_log_closing_target_arm, docs_scope_coordination_001_stack_and_workspace_log_blocker_vm_architecture_unconfirmed [EXTRACTED 1.00]
- **aarch64 Linux evidence: verdict, revision, image variant, compiler, binary, tests, fixture blocks and AC coverage for the declared target** — docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_why, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_verdict, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_revision, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_checkout_docker, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_architecture, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_command, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_image, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_compiler, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_build_binary, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_test_counts, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_linux_only_tests, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_fixture_blocks, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_ui_box, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_ac_coverage, docs_specs_0001_initial_architecture_verify_linux_evidence_aarch64_files [EXTRACTED 1.00]
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

## Communities (49 total, 0 thin omitted)

### Community 0 - "check-config Integration Tests"
Cohesion: 0.07
Nodes (86): a_value_after_config_is_always_the_path(), ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied() (+78 more)

### Community 1 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (57): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+49 more)

### Community 2 - "Scope 002 Brief Rules"
Cohesion: 0.07
Nodes (41): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Context files (nested AGENTS.md for apps/pool and crates/pool-config), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Safe first party Rust and bounded resources, Workspace lint unsafe_code = forbid applied to every member, Dependency declared once under workspace.dependencies, taken only when the current path uses it; Cargo.lock committed (0001 AC-12) (+33 more)

### Community 3 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.14
Nodes (14): Pgpool baseline modes and POOL dispositions, native_replication and snapshot_isolation modes (deferred), streaming_replication mode (initial operating model), Pgpool Watchdog coordination, Connections and identity option family (48 options), Logs, files and reset option family (28 options), Relation metadata option family (6 options), Replication and routing option family (17 options) (+6 more)

### Community 4 - "Linux Build Evidence, AC-2 and VM Architecture"
Cohesion: 0.16
Nodes (39): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md, Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Linux build (0001 AC-2) and the two Linux only tests passed for the declared target aarch64-unknown-linux-gnu at 41e8eda; record under Linux evidence: container build for aarch64 in verify.md; the earlier x86_64 run at 18e9228 stays there as history, Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user) (+31 more)

### Community 5 - "Security Requirements and Features F-100 to F-108"
Cohesion: 0.10
Nodes (30): Security and Review Requirements, F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-107 Rate and resource limits (admission, frame and auth CPU budgets), F-108 Secure reload (validate entire generation before publication), Recovery Features (F-076 to F-080) (+22 more)

### Community 6 - "Slices, Milestones and Readiness Gates"
Cohesion: 0.10
Nodes (40): POOL agent guide (AGENTS.md), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested, Agent Workflow, Workspace conventions (inherited workspace.package fields, workspace lints with unsafe_code forbid, single workspace.dependencies, committed Cargo.lock, no sockets/threads/runtime, cargo test harness only, commits name Build plan step), Claude Code guidance (CLAUDE.md) (+32 more)

### Community 7 - "Standards, Tooling and Release Qualification"
Cohesion: 0.09
Nodes (33): Invariant: promotion requires a reviewed operation and independently verified fencing, Tests and Evidence Standard, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), Failover features F-067 to F-075, First deployment: one proxy VM, primary and two replicas on Proxmox (+25 more)

### Community 8 - "pool CLI Entry and Argument Parsing"
Cohesion: 0.12
Nodes (29): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), check_config() (+21 more)

### Community 9 - "Safety Invariants and Session Risks"
Cohesion: 0.07
Nodes (40): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Invariant: preserve transaction and session ownership, Invariant: keep writes on the verified primary (+32 more)

### Community 10 - "Scaffold Step and dev-f1-build Gate"
Cohesion: 0.08
Nodes (39): Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Box ticked: Verify it (/check verify stack and workspace), Checked in wave 1: F1 build notes recorded (arc-f1-update done, commit a6e0042, merged to bed9fad; AC-1..AC-13 and 5 step Build plan present, docs only), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test), Sub session chk-f1-verify continued (reverify on the merged tree after the review fixes, closes Minor 6; report blocked, commit cf928e7: revision b6ce165, 11 AC PASS, 134 tests, 398 probe checks; AC-2 and /dev/full clause BLOCKED), Decision: build notes summary 'Accept this spec?' answered accept without changes (mode auto), checked by lead (+31 more)

### Community 11 - "Feature Inventory and Parity Matrix"
Cohesion: 0.09
Nodes (32): Feature inventory and parity matrix, Cache features F-088 to F-093, Cluster Features (F-061 to F-066), Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-065 Primary Identification, F-065 Primary identification (durably approved writer with fresh evidence), F-066 Topology reconciliation (durable writer/epoch/quarantine) (+24 more)

### Community 12 - "Architecture Spec and Repository Layout"
Cohesion: 0.30
Nodes (17): Data Plane vs Control Plane Separation, Planned Architecture, ADR 0001. Initial architecture, Topology Snapshot, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI (+9 more)

### Community 13 - "Leases, Cancellation and Login Routing"
Cohesion: 0.16
Nodes (27): Tracer Bullet Build Approach, Shared Conventions Table, Pooling Features (F-023 to F-032), Protocol Features (F-001 to F-022), Critical Path Handoffs, Core MVP, Development Readiness Review, Connection pooling PRD (+19 more)

### Community 14 - "Workspace Layout, Bounded Read and FIFO Limit"
Cohesion: 0.05
Nodes (125): POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust; the repository holds the planning package and the scope 001 workspace slice (apps/pool, crates/pool-config, offline pool check-config), not an implemented proxy, ac12_dev_null_is_an_empty_file_that_misses_config_version(), Added: pool check-config --config <PATH> [--format human|json], offline check before start; opens no socket, starts no thread, reads at most 1 MiB plus one byte, Added: first configuration contract (config_version = 1 plus reserved result_cache and automatic_promotion tables; absent or enabled = false is valid, asking for either is config.unsupported_capability, SEC-10), Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish), Added: fixed finding codes (config.syntax, config.schema, config.missing_key, config.unsupported_version, config.too_large, config.not_utf8) with line and column; output from constant text, key names and positions only, so no secret appears (SEC-11), Added: fifteen configuration fixtures and 134 automated tests covering every first build acceptance criterion on the workstation; 136 on Linux because two need /dev/full, Added: pool --help and pool --version (+117 more)

### Community 15 - "Linux Target, VM Architecture and Follow-up 11"
Cohesion: 0.10
Nodes (53): Added: spec 0001 build notes, AC-1 to AC-13, five step build plan, verification record and independent review; Linux container build (AC-2) passed for aarch64-unknown-linux-gnu, the one declared Linux target, image digest and binary checksum in the verification record, Sub session arc-f1-update-2 (architect update: spec 0001 Linux target to aarch64-unknown-linux-gnu after the user's F1-N1 value ARM; done, commit 41e8eda: AC-2, target table, Linux build --platform linux/arm64, Follow-up 6, 7, 9, rationale Linux target update; merged to main), Resolved former blocker: proxy and PostgreSQL VM architecture for 0001 AC-2 (F1-N1) answered ARM by the user 2026-11-14, target moved to aarch64-unknown-linux-gnu; no open blocker remains (Follow-up 11 waits for /scope and holds nothing), Sub session chk-f1-verify-3 (checker verify: Linux build evidence for aarch64, 0001 AC-2 and the /dev/full clause of AC-11; first turn standby, done, commit 38da818: revision 41e8eda, exit 0, host aarch64, 136 tests, two Linux only tests ok, 30 fixture blocks, no emulation; merged to main), Closing remaining blockers: none for scope 001 (F1-N1 answered ARM, Linux evidence recorded for aarch64); Follow-up 11 of spec 0001 waits for /scope, Decision: checker merged the architect branch into its worktree so the spec update and evidence reach main in one merge commit and one graph update; built revision 41e8eda; x86_64 no longer declared (architect, biasa, as the lead recommended), 18e9228 record stays as history; lead told the scope 002 lead Follow-up 9 now means a Linux arm64 runner, Decision: user wrote Docker sudah nyala (F1-1); lead ran the spec 0001 Linux build block from main at 18e9228 (exit 0, 136 tests, linux build check passed, sha256 0481be18...f7bdb3); new session chk-f1-verify-2 ran the same block itself and recorded it in verify.md with the same checksum and test count, Decision: user answered F1-N1 'VM saya ARM'; the delegated x86_64 target is withdrawn and the aarch64 runner up applies; lead checked the pinned digest is an index with linux/arm64/v8 (pin kept), trial build with linux/arm64 from 2bc4f90 passed (136 tests, sha256 cb184231...2d3532), then started arc-f1-update-2 and chk-f1-verify-3 (standby) (+45 more)

### Community 16 - "Changelog, Feature 1 Done and Spec Status"
Cohesion: 0.19
Nodes (19): Changelog (CHANGELOG.md; Keep a Changelog 1.1.0 format, Semantic Versioning 2.0.0), Changelog format: Keep a Changelog 1.1.0 and Semantic Versioning 2.0.0, Feature 1 is done; its Linux build evidence is in the verification record; next queued feature is 2 (standards and tooling), then the first walking slice, Spec 0001 lifecycle status mirrors scope feature 1: In Progress while the foundation is built and proven, Accepted when the feature is done, 1. Stack and workspace (done), Box ticked: Document it (/document stack and workspace), Evidence: verification record, independent review and changelog, Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only) (+11 more)

### Community 17 - "IoFinding and Permission Denied Path"
Cohesion: 0.15
Nodes (18): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+10 more)

### Community 18 - "Glossary and Routing Profiles"
Cohesion: 0.11
Nodes (33): Separate Frontend and Backend Identity, Session Model (pinned backend lease), Balancing features F-048 to F-053, F-024 Transaction Mode (refused, session pooling only), F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Admission (+25 more)

### Community 19 - "Scope 001 Coordination Brief and Sub Sessions"
Cohesion: 0.12
Nodes (37): Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Standby first turn of a new session (STATUS: standby until lead sends 'mulai'), Log koordinasi: 001-stack-and-workspace (coordination log), Sub session arc-f1-update (architect update: record build notes in spec 0001; merged to bed9fad, spec commit a6e0042), Resolved former blocker: untracked .graphifyignore and graphify-out/cache/last_query_stamp held the closing push; done at d2fda37 (.graphifyignore committed, last_query_stamp gitignored), Checked in wave 1: F1 Scaffold from the decision (dev-f1-build done and chk-f1-verify report, merged 5f44d88; manual F1-1, F1-2 NOT_RUN) (+29 more)

### Community 20 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.15
Nodes (27): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first) (+19 more)

### Community 21 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.21
Nodes (18): Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Sub session arc-f1-update reused (architect update: nine spec 0001 clarifications from the review in index.md and a new rationale.md table, commit 5f1e995, no AC added or changed; merged with dev-f1-build), Decision: non-blocking spec gap from tst-f1-test: --format=<bytes not UTF-8> gives unexpected argument but the spec lacks 'separate'; architect to add a sentence, joined with the review round; checker notes config_version() constant and --config=<bytes not UTF-8>, Decision: review Minors (config_version() constant, Location::None untested, io.permission_denied untested, loose column test, no automatic AC-8/AC-12/AC-13 guard, verify.md only at 5f44d88) routed to the developer fix, scope 002 Follow-up and reverify; cheap Nits fixed, Delegated decision (0001 AC-7, AC-11): nine spec clarifications from the review, spec follows the existing code and the reviewer and tester recommendation; runner up change the code (--format=<bytes> as wrong format value); status berlaku, --format=<bytes> and --config=<bytes> (name, equals sign and non UTF-8 bytes in one argument) are unexpected argument; 'value of --format' means the separate argument, Value sourcing (where each output value comes from) (+10 more)

### Community 22 - "Session Model, Pooling Modes and Data Path"
Cohesion: 0.14
Nodes (21): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Pool limits and operations, Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Durable approved writer authority (+13 more)

### Community 23 - "Operation Journal Records and Generations"
Cohesion: 0.31
Nodes (11): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer), Expected Generation, Operation (journaled administrative action) (+3 more)

### Community 24 - "Feature Families and Planned Scopes"
Cohesion: 0.11
Nodes (28): HA Features (F-081 to F-087), Tracer Bullet build approach, V1.5 Milestone, Walking Slice Milestone, R-21 Proxmox host correlated failure, R-24 Walking slice exceeds its estimate, Roadmap, Delivery slices (+20 more)

### Community 25 - "Hand Written Deserialize and Reserved Capability Probes"
Cohesion: 0.16
Nodes (18): KnownField, Result, ac7_a_reserved_capability_written_as_an_array_is_a_schema_finding(), D, DeserializeSeed, Probes of the hand written Deserialize (37 made files at 4ebcf93: key given twice, unknown keys, enabled not boolean, capability not a table, other spellings, config_version, canary, valid files), Major: a reserved capability written as an array (result_cache = [false]) is accepted as valid, because derived RawCapability Deserialize accepts a positional sequence (lib.rs:207), Suggested fix for the Major: hand written Deserialize whose visitor implements visit_map only, tests for = [false] and = [true] for both capabilities, one spec sentence that structures accept a table only (+10 more)

### Community 26 - "Rust Technology Evaluation"
Cohesion: 0.17
Nodes (21): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy, Linux builder image rust:1.100.0-slim-bookworm by index digest (+13 more)

### Community 27 - "SQL Routing and Replica Profiles"
Cohesion: 0.18
Nodes (16): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Replication freshness and byte lag, Weighted round robin replica selection, Observation record (+8 more)

### Community 28 - "Failover Records and Fencing Authority"
Cohesion: 0.18
Nodes (17): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+9 more)

### Community 29 - "Pgpool Configuration Coverage Inventory"
Cohesion: 0.23
Nodes (12): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Pgpool baseline, Baseline gaps (planning inventory, not parity certification), License and source reuse policy, PCP commands, SQL commands and pgpool_adm reference surface, Tagged sample and source manifest (V4_7_3) (+4 more)

### Community 30 - "Configuration Keys and Secret References"
Cohesion: 0.24
Nodes (10): Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Dependency and Configuration Change Rules, Configuration Generation, Configuration Version (config_version key), Secret Reference (_file key), config_version key (required, supports exactly 1), Configuration rules (shared configuration key convention), Secret references only by a _file key (+2 more)

### Community 31 - "Feasibility Study and Option Families"
Cohesion: 0.20
Nodes (10): Feasibility study, Automatic promotion (not ready for initial release), Commercial and maintenance assumptions, Manual promotion (feasible with external dependencies), Operational ownership boundaries, Query result caching (high uncertainty), Session pooling feasibility (high feasibility, high correctness risk), Transparent arbitrary read splitting (low confidence) (+2 more)

### Community 32 - "Testing Strategy, CI Progression and Slices"
Cohesion: 0.16
Nodes (18): No CI yet (arrives with scope 002); Linux container build per spec 0001; later property, PostgreSQL integration and compatibility/fuzz/chaos tests; do not invent passing results, Slice, Verification Record, Walking Slice (glossary term), Handoff for Later Slices, R-25 Reviewer or lab unavailable at a gate, Go or no go milestones, proptest 1.12.0 (state and framing properties) (+10 more)

### Community 33 - "Protocol Contract and Driver Compatibility: Driver compatibility matrix"
Cohesion: 0.08
Nodes (28): Backend Login, Support Profile, Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming, Extended Query and pipelining behavior (+20 more)

### Community 34 - "Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling"
Cohesion: 0.15
Nodes (16): Brief general rules: 002-standards-and-tooling, Allowed downloads and installs (crates adopted by spec 0002, pinned toolchain, cargo-deny, cargo-fuzz), Decision mode auto, Disk budget rule (max 2 concurrent Rust builds, delete target before done), GitHub read-only rule for sessions, Sub sessions never update graphify-out; lead updates graph in merge commit, Manual steps policy (Docker, GitHub settings, push, PRs run only by user), Scope 002 boundary: align written rules with workspace and install required checks (+8 more)

### Community 35 - "pool-config Validation Library"
Cohesion: 0.13
Nodes (17): capability_state(), CapabilityState, Finding, FindingCode, Location, position(), RawCapability, RawConfig (+9 more)

### Community 36 - "Competitor Analysis and Benchmark Method"
Cohesion: 0.47
Nodes (9): Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC), PgCat (Rust/Tokio routing proxy, MIT), Competitor reference roles recommendation, Pgpool-II 4.7.3 (selected baseline) (+1 more)

### Community 37 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.46
Nodes (8): Control module, Data plane, Database trust domain, Operation journal, Operator management domain, Telemetry domain, Trust boundaries, Untrusted client network

### Community 38 - "Tracing, Log Redaction and Logging Decisions"
Cohesion: 0.47
Nodes (6): tracing 0.1.44 (structured events and spans), Follow-up: file permission check in scope 004 and tracing with the first log event, Log events and redaction (shared log convention), prometheus-client metrics and tracing logs, Decision: hand written Display and Error over thiserror and anyhow, Decision: no logging in this build, rule fixed for later

### Community 39 - "rustls TLS Provider"
Cohesion: 0.67
Nodes (3): rustls 0.23.46 (TLS), Single rustls crypto provider (aws-lc-rs feature family), rustls and tokio-rustls TLS

### Community 40 - "Scope 001 Review Record and Verdicts"
Cohesion: 0.27
Nodes (13): Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), Review summary: two package workspace and offline pool check-config follow spec 0001; output from constants only, bounded read, enabled = true refused in every spelling, Review test coverage: what is covered well and the gaps (array starting with a boolean, permission denied arm, no span arm, column through validate, AC-8/12/13 guards, --format=<bytes>), Review method: two passes on the same reviewer model; the second pass checked each finding and found the Major by running the binary, Review verdict: Approve with nits after the fix round at 4ebcf93 (first pass at 02bdcbe was Changes requested), First review verdict at 02bdcbe: Changes requested (0 Blocker, 1 Major, 6 Minor, 11 Nit), Second review verdict: Approve with nits; no blocker or major open; Minor 6 (verification record) still open, handled by the verify session (+5 more)

### Community 41 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.23
Nodes (14): error_location(), ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check, Rules made clearer in the review, each checked on the binary (17 rows, all PASS) (+6 more)

### Community 42 - "Load Balancing and Replica Selection"
Cohesion: 0.20
Nodes (17): SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), No primary fallback in V1, Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+9 more)

### Community 43 - "Administration, Cluster and Health PRDs: Administration PRD"
Cohesion: 0.12
Nodes (29): F-060 Network partitions (health changes eligibility only), Health features F-054 to F-060, Axum REST administration API plus CLI, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control (+21 more)

### Community 46 - "Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f"
Cohesion: 0.10
Nodes (28): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path, GA verification depth for data safety features, Independent review resolutions (seven approved fixes), One daemon with separate modules plus separate CLI (+20 more)

### Community 47 - "Virtual Cancellation Model"
Cohesion: 0.25
Nodes (11): Virtual Cancellation Model, Frontend owner task, Architecture overview, Application path sequence, Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), F-018 CancelRequest and BackendKeyData, F-018 CancelRequest and BackendKeyData (virtual per lease keys), Lease Generation (+3 more)

### Community 54 - "Manual Failover, Fencing and Promotion Plan"
Cohesion: 0.16
Nodes (17): Invariant: health failure does not authorize promotion, Fence Receipt, Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Health failure alone produces no promotion, Manual failover PRD, Durable intent before fencing and reconciliation of unknown outcomes (+9 more)

### Community 58 - "SQL Routing and Replica Profiles: Protocol engine"
Cohesion: 0.11
Nodes (20): Transaction pooling (deferred), Operation admission boundary, Future multi proxy deployment, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status) (+12 more)

### Community 61 - "Column Counting Rule and Tests"
Cohesion: 0.27
Nodes (10): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way), Second review run at 4ebcf93: fmt and clippy exit 0, tests 3 + 66 + 3 + 60 + 2 passed, 18 lock packages, AC-12 grep empty; Linux build not run, Second review, fix round at 4ebcf93 (range 02bdcbe..4ebcf93; code a705659, 5a99563, b18b288, 453ae73; spec 5f1e995; same reviewer session), Spec text against the code: the nine clarifications in index.md and the rationale table match the fixed binary; AC-1 to AC-13 unchanged, What is left after the second review: Minor 6 verify record on the merged tree; AC-12 and AC-13 guards wait for scope 002 (Follow-up 11) (+2 more)

## Knowledge Gaps
- **62 isolated node(s):** `F-067 Manual plan and promotion (operator plan plus verified fence)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested`, `Platform gated tests (/dev/zero and mode 000 on unix, /dev/full on Linux only)` (+57 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Spec 0001. POOL initial architecture` connect `Slices, Milestones and Readiness Gates` to `Scope 002 Brief Rules`, `Linux Build Evidence, AC-2 and VM Architecture`, `Safety Invariants and Session Risks`, `Scaffold Step and dev-f1-build Gate`, `Architecture Spec and Repository Layout`, `Leases, Cancellation and Login Routing`, `Workspace Layout, Bounded Read and FIFO Limit`, `Linux Target, VM Architecture and Follow-up 11`, `Changelog, Feature 1 Done and Spec Status`, `Glossary and Routing Profiles`, `Scope 001 Coordination Brief and Sub Sessions`, `Scope 001 Coordination Brief, Log and Authorization`, `Review Clarifications and check-config Usage Rules`, `Session Model, Pooling Modes and Data Path`, `Feature Families and Planned Scopes`, `Rust Technology Evaluation`, `Failover Records and Fencing Authority`, `Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling`, `Tracing, Log Redaction and Logging Decisions`, `Scope 001 Review Record and Verdicts`, `Load Balancing and Replica Selection`, `Administration, Cluster and Health PRDs: Administration PRD`, `Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f`, `Virtual Cancellation Model`, `Manual Failover, Fencing and Promotion Plan`, `SQL Routing and Replica Profiles: Protocol engine`?**
  _High betweenness centrality (0.230) - this node is a cross-community bridge._
- **Why does `1. Stack and workspace (done)` connect `Changelog, Feature 1 Done and Spec Status` to `Testing Strategy, CI Progression and Slices`, `Linux Build Evidence, AC-2 and VM Architecture`, `Slices, Milestones and Readiness Gates`, `Scope 001 Review Record and Verdicts`, `Safety Invariants and Session Risks`, `Scaffold Step and dev-f1-build Gate`, `Leases, Cancellation and Login Routing`, `Workspace Layout, Bounded Read and FIFO Limit`, `Linux Target, VM Architecture and Follow-up 11`, `Scope 001 Coordination Brief and Sub Sessions`, `Scope 001 Coordination Brief, Log and Authorization`, `Configuration Keys and Secret References`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Feature Inventory and Parity Matrix` to `Testing Strategy, CI Progression and Slices`, `Security Requirements and Features F-100 to F-108`, `Slices, Milestones and Readiness Gates`, `Standards, Tooling and Release Qualification`, `Safety Invariants and Session Risks`, `Load Balancing and Replica Selection`, `Administration, Cluster and Health PRDs: Administration PRD`, `Leases, Cancellation and Login Routing`, `Leases, Cancellation and Login Routing: Independent review resolutions (seven approved f`, `Glossary and Routing Profiles`, `Manual Failover, Fencing and Promotion Plan`, `Feature Families and Planned Scopes`, `Pgpool Configuration Coverage Inventory`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` (e.g. with `Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish)` and `Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary`) actually correct?**
  _`AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Are the 4 inferred relationships involving `AC-2 Linux build (container check ends with 'linux build check passed' and shows host aarch64-unknown-linux-gnu)` (e.g. with `Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md` and `Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS`) actually correct?**
  _`AC-2 Linux build (container check ends with 'linux build check passed' and shows host aarch64-unknown-linux-gnu)` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `F-067 Manual plan and promotion (operator plan plus verified fence)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165` to the rest of the system?**
  _62 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `check-config Integration Tests` be split into smaller, more focused modules?**
  _Cohesion score 0.06920326864147089 - nodes in this community are weakly interconnected._