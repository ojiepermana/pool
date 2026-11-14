# Graph Report - pool  (2026-11-14)

## Corpus Check
- 2 files · ~155,724 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1976 nodes · 5500 edges · 100 communities (87 shown, 13 thin omitted)
- Extraction: 86% EXTRACTED · 14% INFERRED · 0% AMBIGUOUS · INFERRED: 777 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Doc
- pool-config Validation Tests
- Slices, Milestones and Readiness Gates
- Spec 0002: Standards and tooling
- pool-config Validation Library
- Scope 001 Coordination Brief, Log and Authorization
- repo_check/docs.rs
- Security Requirements and Features F-100 to F-108: Reviewer roster (12 review and risk owner roles,
- Scaffold Step and dev-f1-build Gate
- Offline Bounds, Parser Depth and Known Limits
- Workspace Layout, Bounded Read and FIFO Limit
- Linux Build Evidence, AC-2 and VM Architecture
- pool CLI Entry and Argument Parsing: cli.rs: ci.rs
- check-config Integration Tests
- Scaffold Step and dev-f1-build Gate: 0001. Verification record
- Scope 002 Brief Rules
- Review Clarifications and check-config Usage Rules
- Security Requirements and Features F-100 to F-108: Trust boundaries
- Configuration Keys and Secret References
- check.sh
- check-config Integration Tests: assert_run()
- Load Balancing and Replica Selection
- Tracing, Log Redaction and Logging Decisions
- Leases, Cancellation and Login Routing
- repo_check/workspace.rs
- Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende: Rust technology evaluation
- Feature Inventory and Parity Matrix
- workspace.rs
- Scope 001 Coordination Brief, Log and Authorization: Aturan umum brief: 001-stack-and-workspace (gene
- Security Requirements and Features F-100 to F-108
- Glossary and Routing Profiles
- Review Clarifications and check-config Usage Rules: src/cli.rs
- Safety Invariants and Session Risks: Routing Features (F-033 to F-047)
- Scope 002 Brief Rules: apps/pool agent guide
- repo_check/ci.rs
- Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture
- SQL Routing and Replica Profiles: Protocol engine
- Report
- .new()
- Workspace Layout, Bounded Read and FIFO Limit: Adopted dependencies (serde, toml, serde_json)
- Changelog, Feature 1 Done and Spec Status
- Report: Report
- Slices, Milestones and Readiness Gates: Engineering standards
- pool-config Validation Library: pool/src/report.rs
- Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende: Adopted in scope 002 (tools, not product depende
- Outcome
- Scope 002 Brief Rules: Seven required checks: fmt, clippy, test, deny, 
- Load Balancing and Replica Selection: SQL routing PRD
- Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende
- prove-checks.sh
- text.rs
- Administration, Cluster and Health PRDs: Administration PRD
- Changelog, Feature 1 Done and Spec Status: 1. Stack and workspace (done)
- Standards, Tooling and Release Qualification
- Protocol Contract and Driver Compatibility: Driver compatibility matrix
- check-config Integration Tests: pool()
- Security Requirements and Features F-100 to F-108: Health monitoring PRD
- Feature Families and Planned Scopes: Roadmap
- Fixture
- Workspace Layout, Bounded Read and FIFO Limit: Shared Conventions Table
- Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Workspace guards W1-W7
- SQL Routing and Replica Profiles: Protocol engine: Control plane
- Pgpool Baseline, Competitors and Feasibility: Pgpool configuration coverage inventory
- SQL Routing and Replica Profiles: Protocol engine: Cluster management
- SQL Routing and Replica Profiles: Protocol engine: Protocol engine
- Glossary and Routing Profiles: Approved Writer
- Pgpool Baseline, Competitors and Feasibility: Feasibility study
- Scope 002 Brief Rules: 2. Standards and tooling (in-progress)
- Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Document checks D1-D6
- Review Clarifications and check-config Usage Rules: repo-check/src/main.rs
- Review Clarifications and check-config Usage Rules: pool/src/main.rs
- Safety Invariants and Session Risks
- Pgpool Baseline, Competitors and Feasibility: Competitor analysis
- Protocol Contract and Driver Compatibility: Driver compatibility matrix: PostgreSQL protocol analysis
- Pgpool Baseline, Competitors and Feasibility: Streaming sample census (CFG-001 to CFG-207, 207
- Glossary and Routing Profiles: Topology Snapshot
- SQL Routing and Replica Profiles: Protocol engine: Promotion operation stages (Plan, Quiesce, Fence
- SQL Routing and Replica Profiles: Protocol engine: Promotion operation stages (Plan, Quiesce, Fence: Promotion operation stages (Plan, Quiesce, Fence
- Glossary and Routing Profiles: Lease
- Glossary and Routing Profiles: Session Model (pinned backend lease)
- Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling
- Feature Inventory and Parity Matrix: Query cache PRD
- Pgpool Baseline, Competitors and Feasibility: Streaming sample census (CFG-001 to CFG-207, 207: Pgpool baseline modes and POOL dispositions
- Administration, Cluster and Health PRDs: Administration PRD: R-09 Failover loses acknowledged commits
- Slices, Milestones and Readiness Gates: KPIs and proposed gates
- linux-build.sh
- pool-repo-check
- ids.rs
- Review Clarifications and check-config Usage Rules: BTreeSet
- Review Clarifications and check-config Usage Rules: Fn
- Review Clarifications and check-config Usage Rules: Item
- Review Clarifications and check-config Usage Rules: Iterator
- Review Clarifications and check-config Usage Rules: Option
- Review Clarifications and check-config Usage Rules: PathBuf
- Review Clarifications and check-config Usage Rules: Result
- Review Clarifications and check-config Usage Rules: Self
- Review Clarifications and check-config Usage Rules: String
- Review Clarifications and check-config Usage Rules: Vec

## God Nodes (most connected - your core abstractions)
1. `Spec 0002: Standards and tooling` - 95 edges
2. `Spec 0001. POOL initial architecture` - 87 edges
3. `Log koordinasi: 001-stack-and-workspace (coordination log)` - 63 edges
4. `Glossary` - 62 edges
5. `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` - 56 edges
6. `AC-2 Linux build (container check ends with 'linux build check passed' and shows host aarch64-unknown-linux-gnu)` - 52 edges
7. `Scope: POOL, order and status index` - 45 edges
8. `good_tree()` - 45 edges
9. `Testing strategy` - 43 edges
10. `Report` - 42 edges

## Surprising Connections (you probably didn't know these)
- `Where a Change Belongs (documentation ownership table)` --semantically_similar_to--> `Documentation ownership model (scope, specs, PRDs, ADRs, research)`  [INFERRED] [semantically similar]
  docs/index.md → AGENTS.md
- `Safety Over Throughput Priority` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
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
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
- **Tools adopted in scope 002** — docs_research_technology_evaluation_github_actions_runner, docs_research_technology_evaluation_cargo_deny, docs_research_technology_evaluation_cargo_fuzz, docs_research_technology_evaluation_libfuzzer_sys, docs_research_technology_evaluation_cargo_cyclonedx, docs_research_technology_evaluation_postgresql_container_image [EXTRACTED 1.00]
- **Scope 002 Build it milestone rollup** — docs_scope_002_standards_and_tooling_build_it, docs_scope_002_standards_and_tooling_milestone_1_dependency_policy_fuzz, docs_scope_002_standards_and_tooling_milestone_2_check_entry_ci, docs_scope_002_standards_and_tooling_milestone_3_guards_doc_checks, docs_scope_002_standards_and_tooling_milestone_4_proofs_inventory_docs [EXTRACTED 1.00]
- **Delegated berat decisions on 0002 AC-13 branch rule and change path** — docs_scope_coordination_002_standards_and_tooling_log_dd_ruleset_no_bypass, docs_scope_coordination_002_standards_and_tooling_log_dd_scope_close_via_pr, docs_scope_coordination_002_standards_and_tooling_log_dd_zero_approvals [EXTRACTED 1.00]
- **Criteria needing a real GitHub pull request** — docs_specs_0002_standards_and_tooling_index_ac_3, docs_specs_0002_standards_and_tooling_index_ac_5, docs_specs_0002_standards_and_tooling_index_ac_12, docs_specs_0002_standards_and_tooling_index_ac_13, docs_specs_0002_standards_and_tooling_index_remote_proof, docs_specs_0002_standards_and_tooling_index_build_plan_step_11 [EXTRACTED 1.00]
- **pool-repo-check guard families (workspace, CI, docs)** — docs_specs_0002_standards_and_tooling_index_repository_check_program, docs_specs_0002_standards_and_tooling_index_workspace_guards, docs_specs_0002_standards_and_tooling_index_ci_definition_guards, docs_specs_0002_standards_and_tooling_index_document_checks [EXTRACTED 1.00]
- **Cancellation isolation across protocol, threat model and driver tests** — docs_research_protocol_analysis_variable_length_cancellation_keys, docs_research_protocol_analysis_virtual_cancellation_keys, docs_testing_compatibility_matrix_libpq_psql [INFERRED 0.85]
- **Late cancellation safety: feature, risk and threat** — docs_feature_matrix_f_018_cancelrequest_and_backendkeydata, docs_security_threat_model_t_05_late_cancellation_affects_another_user [INFERRED 0.85]
- **Fenced manual promotion flow** — docs_architecture_failover_recovery_promotion_operation_stages, docs_architecture_failover_recovery_fencing_contract, docs_architecture_failover_recovery_all_writer_barrier, docs_architecture_control_plane_fencereceipt, docs_architecture_control_plane_approvedtopology, docs_architecture_control_plane_operation, docs_architecture_deployment_independent_fencing_authority [INFERRED 0.85]
- **Fenced manual promotion and split brain prevention across invariants, features, risks, requirements and threats** — agents_promotion_requires_verified_fencing, docs_feature_matrix_f_067_manual_plan_and_promotion, docs_feature_matrix_f_069_old_primary_fencing, docs_feature_matrix_f_070_split_brain_prevention, docs_security_security_requirements_sec_7_persist_intent_before_external_effect, docs_security_security_requirements_sec_9_allowlisted_fencing_recovery_adapters, docs_security_threat_model_t_10_split_brain_or_stale_leader [INFERRED 0.85]
- **Fenced manual promotion safety flow** — docs_prd_007_manual_failover_promotionplan, docs_prd_007_manual_failover_durable_intent_and_reconciliation, docs_prd_007_manual_failover_rpo_disclosure, docs_prd_007_manual_failover_retire_candidate_replica_leases, docs_prd_006_cluster_management_approved_writer_epoch_quarantine, docs_plan_decision_ledger_authoritative_write_barrier [INFERRED 0.85]
- **Lease ownership and cancellation safety across core proxy, pooling and HA** — docs_prd_001_core_proxy_lease, docs_prd_001_core_proxy_cancellationentry, docs_prd_001_core_proxy_cancellation_through_lease_owner, docs_plan_decision_ledger_private_backend_keys, docs_prd_002_connection_pooling_reset_before_reuse, docs_prd_008_high_availability_cancellation_dispatch_to_owning_proxy [INFERRED 0.85]
- **No untrusted or secret text reaches output or telemetry** — apps_pool_agents_constant_output_rule, crates_pool_config_agents_rejection_no_file_text, apps_pool_agents_canary_leak_regression, agents_no_secrets_in_default_telemetry [INFERRED 0.85]
- **Restricted replica read routing: login selection, eligibility, pinned admission, no fallback** — docs_plan_decision_ledger_routing_decision, docs_prd_003_sql_routing_select_backend, docs_prd_003_sql_routing_admit_on_pinned_backend, docs_prd_003_sql_routing_restricted_replica_profile, docs_prd_004_load_balancing_eligibility_filter, docs_prd_004_load_balancing_weighted_round_robin, docs_prd_004_load_balancing_primary_fallback_disabled, docs_prd_005_health_monitoring_unknown_lag [INFERRED 0.85]
- **Same command locally and in CI** — docs_specs_0002_standards_and_tooling_index_check_entry_point, docs_specs_0002_standards_and_tooling_index_ci_definition, docs_specs_0002_standards_and_tooling_index_c3, docs_specs_0002_standards_and_tooling_index_required_checks [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (100 total, 13 thin omitted)

### Community 0 - "Doc"
Cohesion: 0.06
Nodes (85): BTreeMap, Anchors, append(), assert_ok(), assert_problem(), check(), d1_absolute_and_escaping_paths(), d1_missing_file() (+77 more)

### Community 1 - "pool-config Validation Tests"
Cohesion: 0.06
Nodes (57): ac4_sec10_automatic_promotion_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_automatic_promotion_enabled_false_validates_in_every_spelling(), ac4_sec10_automatic_promotion_left_out_validates_as_disabled(), ac4_sec10_result_cache_empty_table_validates_as_disabled_in_every_spelling(), ac4_sec10_result_cache_enabled_false_validates_in_every_spelling(), ac4_sec10_result_cache_left_out_validates_as_disabled(), ac4_validated_config_debug_holds_only_states(), ac5_ac6_sec10_one_capability_off_does_not_hide_the_other_turned_on() (+49 more)

### Community 2 - "Slices, Milestones and Readiness Gates"
Cohesion: 0.06
Nodes (66): Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), Working with the Docs, Contributing to POOL, Project has its first workspace slice and is otherwise in planning; start from the docs index, scope and engineering standards, Git and Release Standards (main only via PR with seven required checks, scripts/check.sh <name>, empty bypass list), Tests and Evidence Standard (+58 more)

### Community 3 - "Spec 0002: Standards and tooling"
Cohesion: 0.08
Nodes (59): ADR 0002. Standards and tooling, Spec 0002: Standards and tooling, AC-1 One entry point, AC-11 Each check catches a wrong change, locally, AC-12 A failed check blocks a merge, on the remote, AC-13 Branch rule, AC-14 Pull request path, AC-15 Fuzz harness (+51 more)

### Community 4 - "pool-config Validation Library"
Cohesion: 0.08
Nodes (38): Added: fifteen configuration fixtures and 134 automated tests covering every first build acceptance criterion on the workstation; 136 on Linux because two need /dev/full, capability_state(), CapabilityState, error_location(), Finding, FindingCode, KnownField, Location (+30 more)

### Community 5 - "Scope 001 Coordination Brief, Log and Authorization"
Cohesion: 0.10
Nodes (46): Box ticked: Scaffold from the decision (/develop stack and workspace), Box ticked: Test it (/test stack and workspace), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Graph update ownership: sub sessions never touch graphify-out, lead refreshes it in the merge commit, Session model per role (architect, debugger, checker on fable-5-2; developer, tester on opus-5-5), Spec 0001 build notes (adopted crates, build plan, ID'd criteria, fixture and Linux build commands), Standby first turn of a new session (STATUS: standby until lead sends 'mulai') (+38 more)

### Community 6 - "repo_check/docs.rs"
Cohesion: 0.09
Nodes (45): Fixture, base_could_not_run_for_a_revision_that_does_not_exist(), base_could_not_run_when_the_root_is_a_folder_inside_a_repository(), base_could_not_run_when_the_root_is_not_a_git_repository(), base_refuses_a_revision_that_starts_with_a_hyphen_as_not_a_commit(), d1_checks_images_reference_definitions_and_links_to_directories(), d1_decodes_percent_twenty_and_drops_angle_brackets(), d1_names_the_file_line_and_target_of_a_missing_link_target() (+37 more)

### Community 7 - "Security Requirements and Features F-100 to F-108: Reviewer roster (12 review and risk owner roles,"
Cohesion: 0.06
Nodes (41): Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), GA workflow depth (behavior verification, tests, fresh review, documentation), Tracer Bullet Build Approach, Agent Workflow, Cargo workspace holds only the pool and pool-config packages, and there is no CI yet (inspect it before relying on documentation), A review is never done by the session or person that wrote the change, Repository owner decides while Project lead, Maintainer or Release maintainer is not assigned, Reviewer roster (12 review and risk owner roles, all Not assigned, with fresh model eligibility per role) (+33 more)

### Community 8 - "Scaffold Step and dev-f1-build Gate"
Cohesion: 0.14
Nodes (41): a_failed_write_exits_3_without_stderr(), ac11_a_failed_write_of_help_version_or_a_rejection_exits_3_without_stderr(), Added: spec 0001 build notes, AC-1 to AC-13, five step build plan, verification record and independent review; Linux container build (AC-2) passed for aarch64-unknown-linux-gnu, the one declared Linux target, image digest and binary checksum in the verification record, Never run anywhere yet: the two cfg(target_os = linux) /dev/full tests and the non UTF-8 path test fallback on APFS, Linux build (0001 AC-2) and the two Linux only tests passed for the declared target aarch64-unknown-linux-gnu at 41e8eda; record under Linux evidence: container build for aarch64 in verify.md; the earlier x86_64 run at 18e9228 stays there as history, Resolved former blocker: 0001 AC-2 and the /dev/full clause of AC-11 not run; Linux build passed at 18e9228 with 136 tests (F1-2 PASS), no longer holds feature 1 done, Sub session chk-f1-verify-2 (checker verify after closing: Linux build evidence for AC-2 and the /dev/full clause of AC-11; report done, commit 972e160: revision 18e9228, exit 0, host x86_64-unknown-linux-gnu, 136 tests (3 + 68 + 3 + 60 + 2), two Linux only tests ok, 30 fixture blocks, no canary leaked; merged to main), Sub session chk-f1-verify-3 (checker verify: Linux build evidence for aarch64, 0001 AC-2 and the /dev/full clause of AC-11; first turn standby, done, commit 38da818: revision 41e8eda, exit 0, host aarch64, 136 tests, two Linux only tests ok, 30 fixture blocks, no emulation; merged to main) (+33 more)

### Community 9 - "Offline Bounds, Parser Depth and Known Limits"
Cohesion: 0.10
Nodes (39): position_counts_lines_and_scalars(), ac7_column_counts_characters_not_bytes_before_the_error(), Review: scope 001 stack and workspace (2026-11-14), reviewer claude-fable-5-2, author claude-opus-5-5, Status of each finding: Major closed, Minor 1 to 4 closed, Minor 5 closed in part, Minor 6 open, first review Nits closed, First review scope: 30 files in range a82cf67..02bdcbe on main (26 code, manifest and fixture files, Cargo.lock, 3 docs), Review known open items: --format=<bytes> unexpected argument needs a spec sentence, config_version constant, Linux tests never ran, stale Decision sentence, Minor 1: ValidatedConfig::config_version() returns the constant SUPPORTED_CONFIG_VERSION, not the parsed number (value sourcing deviation), Minor 4: the column test cannot fail for a byte count (multibyte key is the unknown key, column 1 either way) (+31 more)

### Community 10 - "Workspace Layout, Bounded Read and FIFO Limit"
Cohesion: 0.24
Nodes (37): Added: first configuration contract (config_version = 1 plus reserved result_cache and automatic_promotion tables; absent or enabled = false is valid, asking for either is config.unsupported_capability, SEC-10), First Handoff: the Workspace, Review conformance by criterion table (AC-1 to AC-13: AC-2 not judged, AC-7 deviates for the array row, AC-11 /dev/full clause not judged), The eight items asked of the review (conformance, SEC-10 and SEC-11, no secret in output or Debug, read limit and offline, unsafe_code and package set, crate boundaries, test quality, rubric), First review run at 02bdcbe: tests 3 + 64 + 3 + 57, fmt, clippy, release build, metadata, grep, nm -u; second pass with twelve made files, SEC-10 Disable automatic promotion and result caching by default, AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), AC-10 JSON contract (one JSON object line on stdout, empty stderr) (+29 more)

### Community 11 - "Linux Build Evidence, AC-2 and VM Architecture"
Cohesion: 0.14
Nodes (37): Sub session arc-f1-update-2 (architect update: spec 0001 Linux target to aarch64-unknown-linux-gnu after the user's F1-N1 value ARM; done, commit 41e8eda: AC-2, target table, Linux build --platform linux/arm64, Follow-up 6, 7, 9, rationale Linux target update; merged to main), Resolved former blocker: proxy and PostgreSQL VM architecture for 0001 AC-2 (F1-N1) answered ARM by the user 2026-11-14, target moved to aarch64-unknown-linux-gnu; no open blocker remains (Follow-up 11 waits for /scope and holds nothing), Closing remaining blockers: none for scope 001 (F1-N1 answered ARM, Linux evidence recorded for aarch64); Follow-up 11 of spec 0001 waits for /scope, Decision: user answered F1-N1 'VM saya ARM'; the delegated x86_64 target is withdrawn and the aarch64 runner up applies; lead checked the pinned digest is an index with linux/arm64/v8 (pin kept), trial build with linux/arm64 from 2bc4f90 passed (136 tests, sha256 cb184231...2d3532), then started arc-f1-update-2 and chk-f1-verify-3 (standby), Delegated decision (berat, 0001 AC-2): Linux target x86_64-unknown-linux-gnu over aarch64, inferred from Proxmox VE; withdrawn by the user 2026-11-14 ('VM saya ARM'): runner up aarch64-unknown-linux-gnu applies, applied by arc-f1-update-2 (41e8eda), build rerun by chk-f1-verify-3 (38da818), History Target ARM (2026-11-14): user answered F1-N1 = ARM; spec 0001 moved to aarch64-unknown-linux-gnu (41e8eda), Linux aarch64 build passed (38da818, 136 tests), both merged in one commit; feature 1 stays done; remaining Follow-up 11, F1-2 after the F1-N1 answer (ARM): the spec block uses --platform linux/arm64 and expects host aarch64-unknown-linux-gnu, no emulation on Apple silicon, about 283 MB; rerun by chk-f1-verify-3 at 41e8eda PASS, 136 tests, sha256 cb184231...2d3532, User value F1-N1: proxy and PostgreSQL VM architecture, answered by the user 2026-11-14: ARM; spec 0001 target changed to aarch64-unknown-linux-gnu (--platform linux/arm64) via architect (update) and the Linux build rerun; 0001 AC-2 PASS for aarch64 at 41e8eda (+29 more)

### Community 12 - "pool CLI Entry and Argument Parsing: cli.rs: ci.rs"
Cohesion: 0.14
Nodes (36): assert_ok(), assert_problem(), c1_checks_every_workflow_file(), c1_hash_without_version_comment(), c1_tag_instead_of_hash(), c2_renamed_job(), c3_job_without_run(), c3_last_run_is_another_check() (+28 more)

### Community 13 - "check-config Integration Tests"
Cohesion: 0.11
Nodes (33): ac10_every_outcome_that_opens_the_file_prints_one_json_line_with_fields_in_order(), ac10_valid_capabilities_object_lists_result_cache_first(), ac11_a_closed_stderr_pipe_exits_3_without_stdout_or_panic(), ac11_a_closed_stdout_pipe_exits_3_without_stderr_or_panic(), ac11_a_format_value_that_is_not_exactly_human_or_json_is_refused(), ac11_a_path_that_is_not_utf8_is_used_as_given(), ac11_an_option_given_a_second_time_is_refused_before_its_value_is_read(), ac11_an_option_without_a_value_requires_one() (+25 more)

### Community 14 - "Scaffold Step and dev-f1-build Gate: 0001. Verification record"
Cohesion: 0.09
Nodes (36): Linux container build is the spec 0001 Linux build command; needs a running Docker daemon; a macOS build is not Linux evidence; latest result in verify.md, Minor 6: verify.md records only revision 5f44d88 and not the merged tree with the tester's tests, What is left after the second review: Minor 6 verify record on the merged tree; AC-12 and AC-13 guards wait for scope 002 (Follow-up 11), Box ticked: Record the build notes the handoff requires in spec 0001 ([x], via merge bed9fad), Box ticked: Verify it (/check verify stack and workspace), Checked in wave 1: F1 build notes recorded (arc-f1-update done, commit a6e0042, merged to bed9fad; AC-1..AC-13 and 5 step Build plan present, docs only), Checked in wave 1: F1 Verify it (chk-f1-verify commit 7167f8f, verify.md Full verify: 11 AC PASS, AC-2 and Linux AC-11 BLOCKED, owner user), Sub session chk-f1-verify (checker verify: Verify it, closes Scaffold together; report blocked, 11 AC PASS, AC-2 and AC-11 /dev/full BLOCKED; merged together with tst-f1-test) (+28 more)

### Community 15 - "Scope 002 Brief Rules"
Cohesion: 0.09
Nodes (34): Knowledge graph refresh before commit (graphify-out kept current), GitHub Actions hosted runner ubuntu-24.04-arm (seven required checks, 64 bit ARM), Build it: /develop standards and tooling (four milestones, open), Milestone: dependency policy and fuzz harness, Build plan steps 1-2 (AC-4, AC-15, AC-20), Milestone: check entry point and CI files, steps 3-4 (AC-1, AC-2, AC-5, AC-13, AC-14), needs spec 0001 ARM update on main, Milestone: repository guards and document checks, steps 5-6 (AC-6 to AC-10), Milestone: proofs, inventory, documents and workstation evidence, steps 7-10 (AC-11, AC-16 to AC-19), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply) (+26 more)

### Community 16 - "Review Clarifications and check-config Usage Rules"
Cohesion: 0.16
Nodes (19): Vec, top_level_keys(), BTreeSet, Fn, Option, Path, PathBuf, Result (+11 more)

### Community 17 - "Security Requirements and Features F-100 to F-108: Trust boundaries"
Cohesion: 0.09
Nodes (34): F-069 Old primary fencing (external verified exclusion), F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-101 TLS and mTLS verification, F-105 Administrative roles (viewer, operator, promoter, configuration), F-106 Audit and log redaction, F-107 Rate and resource limits (admission, frame and auth CPU budgets), F-108 Secure reload (validate entire generation before publication), Security features F-100 to F-108 (+26 more)

### Community 18 - "Configuration Keys and Secret References"
Cohesion: 0.11
Nodes (34): Minor 3: the io.permission_denied arm of IoFinding::from_kind (report.rs:30) has no test, although the workstation suite runs as a user that is not root, Five new nits in the fix range: deny_unknown_fields phrase at index.md:431, config_version start value at lib.rs:354, 1_048_577 literal at main.rs:89, compile_fail breadth at lib.rs:28, stale CLAUDE.md and AGENTS.md lines, Sub session arc-f1-update continued (architect update: deny_unknown_fields phrase in step 4, one syntax or schema finding chosen by the parser; done at 5670851 without AC change, lead gate passed, merged into the parent of the docs(changelog) commit), Decision: build notes summary 'Accept this spec?' answered accept without changes (mode auto), checked by lead, Decision: of two schema errors the one with the alphabetically first key is reported, not the first in the file; spec states the choice belongs to the parser (mode auto, follows the code); runner up sort by file position; sent to arc-f1-update, Delegated decision (berat, 0001 AC-9): check-config diagnostics contain only constant text, schema key path, line and column, never file content, secret-* canary fixtures (CANARY-9f2c7b1e), config_version key (required, supports exactly 1) (+26 more)

### Community 19 - "check.sh"
Cohesion: 0.13
Nodes (33): ci workflow (.github/workflows/ci.yml): seven required checks on GitHub Actions, Concurrency group per event name and ref; only pull request runs are cancelled, Job clippy: runs scripts/check.sh clippy, Job deny: runs scripts/check.sh deny, Job docs: runs scripts/check.sh docs, Job fmt: runs scripts/check.sh fmt, Job guards: runs scripts/check.sh guards, Job linux-build: runs scripts/check.sh linux-build (+25 more)

### Community 20 - "check-config Integration Tests: assert_run()"
Cohesion: 0.14
Nodes (33): ac11_a_directory_given_without_a_trailing_slash_cannot_be_read(), ac11_a_file_that_cannot_be_opened_is_permission_denied(), ac11_a_path_through_a_regular_file_cannot_be_read(), ac11_a_path_under_a_missing_directory_is_not_found(), ac11_a_symlink_is_followed_and_a_dangling_one_is_not_found(), ac12_dev_zero_is_too_large_in_the_json_format_too(), ac3_a_path_after_the_first_equals_sign_keeps_its_own_equals_signs(), ac4_sec10_automatic_promotion_as_an_empty_table_reports_disabled() (+25 more)

### Community 21 - "Load Balancing and Replica Selection"
Cohesion: 0.09
Nodes (30): Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path, First deployment: one proxy VM, primary and two replicas on Proxmox, GA verification depth for data safety features, Independent review resolutions (seven approved fixes) (+22 more)

### Community 22 - "Tracing, Log Redaction and Logging Decisions"
Cohesion: 0.09
Nodes (29): Added: pool check-config --config <PATH> [--format human|json], offline check before start; opens no socket, starts no thread, reads at most 1 MiB plus one byte, Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish), Added: fixed finding codes (config.syntax, config.schema, config.missing_key, config.unsupported_version, config.too_large, config.not_utf8) with line and column; output from constant text, key names and positions only, so no secret appears (SEC-11), Added: pool --help and pool --version, Added: JSON report (--format json), one line with a fixed field order, for scripts, Unreleased, Added: the first workspace slice of scope 001 (ten entries), clap 4.6.7 (CLI argument contract, not adopted in 001), tracing 0.1.44 (structured events and spans) (+21 more)

### Community 23 - "Leases, Cancellation and Login Routing"
Cohesion: 0.11
Nodes (29): Cluster Features (F-061 to F-066), F-011 Prepared statements (session ownership, no cross backend replay), F-065 Primary Identification, F-066 Topology reconciliation (durable writer/epoch/quarantine), Pooling Features (F-023 to F-032), Protocol Features (F-001 to F-022), Support Profile, Critical Path Handoffs (+21 more)

### Community 24 - "repo_check/workspace.rs"
Cohesion: 0.14
Nodes (28): line_of(), good_tree(), w1_counts_a_package_that_appears_twice(), w1_names_a_baseline_package_missing_from_the_fuzz_lockfile(), w1_names_a_package_that_the_lockfile_has_and_the_baseline_does_not(), w1_rejects_a_second_package_in_the_tool_lockfile(), w2_names_a_forbidden_directory_that_exists(), w3_counts_a_match_in_a_comment_and_skips_target_and_files_that_are_not_rust() (+20 more)

### Community 25 - "Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende: Rust technology evaluation"
Cohesion: 0.09
Nodes (28): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), External etcd for later HA (no consensus in initial proxy), Hybrid parser policy, Linux builder image rust:1.100.0-slim-bookworm by index digest (+20 more)

### Community 26 - "Feature Inventory and Parity Matrix"
Cohesion: 0.10
Nodes (28): Feature inventory and parity matrix, Cache features F-088 to F-093, Inventory Coverage Limits, F-056 Primary Standby Role Observation, F-065 Primary identification (durably approved writer with fresh evidence), F-088 Optional Memory Cache (disabled in V1), F-088 Optional memory cache (result caching disabled in V1), F-134 pcp_invalidate_query_cache (+20 more)

### Community 27 - "workspace.rs"
Cohesion: 0.21
Nodes (26): assert_ok(), assert_problem(), good(), good_tree_passes(), lock(), w1_counts_duplicates(), w1_extra_package_in_lockfile(), w1_missing_lockfile_is_a_fault() (+18 more)

### Community 28 - "Scope 001 Coordination Brief, Log and Authorization: Aturan umum brief: 001-stack-and-workspace (gene"
Cohesion: 0.14
Nodes (26): Aturan umum brief: 001-stack-and-workspace (general brief rules), Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first), Always forbidden paths for sub sessions (docs/scope, docs/prd, AGENTS.md, CLAUDE.md, graphify-out, .claude, spec content outside ownership), Role files written for another project (Stackly references do not apply) (+18 more)

### Community 29 - "Security Requirements and Features F-100 to F-108"
Cohesion: 0.12
Nodes (25): Fence Receipt, Axum REST administration API plus CLI, No silent replay of uncertain work (lost COMMIT response), GET /v1/nodes node status route, Cluster management PRD, Drain, detach and attach node operations, Two observed primaries close write admission, Generation checked journaled mutations (+17 more)

### Community 30 - "Glossary and Routing Profiles"
Cohesion: 0.12
Nodes (24): Glossary, Baseline File (text list of what a workspace guard allows today), Branch Rule (GitHub ruleset for main kept as a repository file; avoid branch protection), Configuration Generation, Configuration Version (config_version key), Core MVP, Routing preview, V1 (release gates, not version numbers), Credential Generation, Detach (+16 more)

### Community 31 - "Review Clarifications and check-config Usage Rules: src/cli.rs"
Cohesion: 0.17
Nodes (19): Format, Invocation, parse(), parse_check_config(), Option, run(), take_value(), ac11_a_first_argument_that_is_not_a_command_is_unknown() (+11 more)

### Community 32 - "Safety Invariants and Session Risks: Routing Features (F-033 to F-047)"
Cohesion: 0.11
Nodes (22): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: never replay uncertain dispatched work, Invariant: preserve transaction and session ownership (+14 more)

### Community 33 - "Scope 002 Brief Rules: apps/pool agent guide"
Cohesion: 0.13
Nodes (22): Context files (nested AGENTS.md for apps/pool and crates/pool-config), Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Safe first party Rust and bounded resources, Workspace lint unsafe_code = forbid applied to every member, Dependency declared once under workspace.dependencies, taken only when the current path uses it; Cargo.lock committed (0001 AC-12), apps/pool agent guide, Bounded file read: at most MAX_CONFIG_BYTES plus one byte via Read::take, Leaking fake canary CANARY-9f2c7b1e into output is a security regression (SEC-11) (+14 more)

### Community 34 - "repo_check/ci.rs"
Cohesion: 0.17
Nodes (21): a_second_workflow_file_is_checked_only_by_c1_and_c5(), c1_checks_every_workflow_file(), c1_names_an_action_pinned_by_tag(), c1_requires_the_version_comment(), c2_names_a_job_that_is_not_a_required_check(), c2_names_a_required_check_that_has_no_job(), c3_does_not_read_the_shell_key(), c3_names_a_run_step_with_logic_of_its_own() (+13 more)

### Community 35 - "Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture"
Cohesion: 0.18
Nodes (21): Added: three shared conventions in the engineering standards (configuration keys, exit status with command output, log event fields with redaction), Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Log event fields and redaction convention (ts, level, event; redaction by type), Pending shared conventions (identifier/generation types 003, SQLSTATEs 004, metric names by 006, admin API versioning 009), Shared Conventions Table, Output Version (output_version field), Spec 0001. POOL initial architecture (+13 more)

### Community 36 - "SQL Routing and Replica Profiles: Protocol engine"
Cohesion: 0.14
Nodes (21): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool key (pool identity), Pool limits and operations, Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Transaction pooling (deferred) (+13 more)

### Community 37 - "Report"
Cohesion: 0.22
Nodes (21): key_value(), toml_lines(), Tree, baseline(), check(), dependencies(), dependency_keys(), forbidden_paths() (+13 more)

### Community 38 - ".new()"
Cohesion: 0.23
Nodes (17): a_cargo_exit_3_outside_pool_repo_check_is_a_failed_check(), a_failed_check_wins_over_one_that_could_not_run(), a_missing_cargo_exits_3_with_one_line_that_names_it(), a_missing_git_makes_docs_exit_3(), a_missing_repo_check_manifest_makes_guards_exit_3(), a_passing_check_ends_with_its_ok_line_and_exits_0(), a_wrong_cargo_deny_version_exits_3_and_names_the_tools_command(), all_keeps_going_after_a_failure_and_exits_1() (+9 more)

### Community 39 - "Workspace Layout, Bounded Read and FIFO Limit: Adopted dependencies (serde, toml, serde_json)"
Cohesion: 0.16
Nodes (20): ADR 0001. Initial architecture, Automatic Promotion (reserved and off), Reserved Capability, Result Cache (reserved and off), Delegated decision (berat, 0001 AC-4, AC-6): automatic promotion capability key named automatic_promotion (cache: result_cache) over automatic_failover, Security requirements, SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL, SEC-12 Validate a whole configuration generation before publication; rotation drains retired identities (+12 more)

### Community 40 - "Changelog, Feature 1 Done and Spec Status"
Cohesion: 0.18
Nodes (19): Added: Cargo workspace with the pool daemon binary (apps/pool) and the pool-config library (crates/pool-config); Rust 1.100.0 pinned by rust-toolchain.toml; unsafe code forbidden in both, Current state: planning package plus the scope 001 workspace (apps/pool with the pool binary, crates/pool-config); only command is the offline pool check-config; no listener, proxy path or CI; Rust 1.100.0 pinned by rust-toolchain.toml, serde_json 1.0.154 (JSON command output, adopted in 001), Code in apps/pool/ and crates/pool-config/, Delegated decisions (biasa, 0001 AC-1, AC-12): 17 build notes choices (pinned rust slim bookworm image and git archive HEAD, std argument parsing, serde_json, no thiserror/anyhow/tracing, two pass validation, 1 MiB limit, ...), Adopted dependencies (serde, toml, serde_json), Build plan step 1: Workspace skeleton, Cargo.lock with exactly sixteen locked packages (+11 more)

### Community 41 - "Report: Report"
Cohesion: 0.19
Nodes (10): Into, Problem, Report, Display, Fn, Formatter, Option, Result (+2 more)

### Community 42 - "Slices, Milestones and Readiness Gates: Engineering standards"
Cohesion: 0.13
Nodes (18): POOL agent guide (AGENTS.md), POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust; the repository holds the planning package and the scope 001 workspace slice (apps/pool, crates/pool-config, offline pool check-config), not an implemented proxy, User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested, Workspace conventions (inherited workspace.package fields, workspace lints with unsafe_code forbid, single workspace.dependencies, committed Cargo.lock, no sockets/threads/runtime, cargo test harness only, commits name Build plan step), Engineering standards, Data and Protocol Safety Standards, Dependency and Configuration Change Rules (dependency policy checks advisories, licenses and sources on every PR; locked package change updates its baseline file), No Replay After Dispatch Rule (+10 more)

### Community 43 - "pool-config Validation Library: pool/src/report.rs"
Cohesion: 0.18
Nodes (12): IoFinding, JsonCapabilities, JsonFinding, JsonReport, Outcome, Option, Result, Self (+4 more)

### Community 44 - "Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende: Adopted in scope 002 (tools, not product depende"
Cohesion: 0.16
Nodes (18): actions/cache v6.1.0 pinned by commit (keeps built cargo-deny), actions/checkout v7.0.1 pinned by commit, Adopted in scope 002 (tools, not product dependencies; pool and pool-config gain none), cargo-cyclonedx 0.5.10 (CycloneDX JSON SBOM, run by hand until scope 011), cargo-deny 0.20.2 (advisories, licenses, bans, sources), cargo deny check passed on both lockfiles in the scratch build (advisories, bans, licenses, sources ok), cargo-fuzz 0.13.2 (builds and runs fuzz targets), Ten extra packages in crates/pool-config/fuzz/Cargo.lock (+10 more)

### Community 45 - "Outcome"
Cohesion: 0.14
Nodes (5): every_command_passes_on_the_committed_tree(), the_root_defaults_to_the_current_directory(), Outcome, repository_root(), run()

### Community 46 - "Scope 002 Brief Rules: Seven required checks: fmt, clippy, test, deny, "
Cohesion: 0.19
Nodes (17): Claude Code guidance (CLAUDE.md), Cargo checks run by hand today (cargo fmt --all --check, clippy --locked -D warnings, test --locked, single test, check-config on valid-minimal.toml); CI arrives with scope 002, Checks to run before proposing a change (Rust 1.100.0 pinned; cargo fmt --all --check, clippy --locked -D warnings, test --locked), No CI yet (arrives with scope 002); Linux container build per spec 0001; later property, PostgreSQL integration and compatibility/fuzz/chaos tests; do not invent passing results, Stable ID Citation Rule, 2. Standards and tooling (in-progress), Documentation checks (links/anchors, scope status agreement, stable requirement IDs), Done when: documented conventions match the workspace and format, lint, test and dependency checks run in CI (+9 more)

### Community 47 - "Load Balancing and Replica Selection: SQL routing PRD"
Cohesion: 0.20
Nodes (17): SQL routing PRD, admit_on_pinned_backend(frame, session_state, classification, policy), Classification, classify(sql, parser_version), No primary fallback in V1, Primary profile sessions stay pinned, Restricted read only replica profile, RouteProfile (+9 more)

### Community 48 - "Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende"
Cohesion: 0.20
Nodes (16): ac12_dev_null_is_an_empty_file_that_misses_config_version(), ac7_arrays_and_inline_tables_nested_far_past_the_limit_give_one_finding(), ac7_nesting_deeper_than_the_parser_allows_is_a_syntax_finding_without_location(), Minor 2: the toml parser depth limit (80 levels) and the Location::None branch of error_location have no test; the unbounded feature is not pinned off, Known limit: check-config reads special files like any file, so a FIFO with no writer blocks the command with no time limit; no code change in scope 001, Nesting deeper than the toml parser allows (e.g. a dotted key with 81 parts) is config.syntax; for a dotted key or table header the finding has no location (line and column null), toml unbounded feature stays off: parser stops at a fixed nesting depth (80 levels in toml 1.2.1), so a 1 MiB file of brackets cannot exhaust the stack, Clarification 4: nesting past the toml limit is config.syntax without location for dotted keys and headers; unbounded stays off; runner up own depth check (+8 more)

### Community 49 - "prove-checks.sh"
Cohesion: 0.31
Nodes (15): append(), apply_case_4(), check(), delete_lines_starting(), delete_table(), expect(), insert_after(), matched() (+7 more)

### Community 50 - "text.rs"
Cohesion: 0.18
Nodes (10): baseline_lines(), code_lines(), fence_closes(), fence_open(), Option, String, Vec, strip_code_spans() (+2 more)

### Community 51 - "Administration, Cluster and Health PRDs: Administration PRD"
Cohesion: 0.17
Nodes (15): Invariant: promotion requires a reviewed operation and independently verified fencing, F-067 Manual plan and promotion (operator plan plus verified fence), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-075 Automatic Failover (disabled in V1), Failover features F-067 to F-075, V1 Release Gate, R-08 Split brain (+7 more)

### Community 52 - "Changelog, Feature 1 Done and Spec Status: 1. Stack and workspace (done)"
Cohesion: 0.25
Nodes (15): Changelog (CHANGELOG.md; Keep a Changelog 1.1.0 format, Semantic Versioning 2.0.0), Changelog format: Keep a Changelog 1.1.0 and Semantic Versioning 2.0.0, Feature 1 is done; its Linux build evidence is in the verification record; next queued feature is 2 (standards and tooling), then the first walking slice, 1. Stack and workspace (done), Box ticked: Document it (/document stack and workspace), Evidence: verification record, independent review and changelog, Scope 001 authorization (/coordinate run on 2026-11-14 is the explicit request to build scope 001 only), Checked in wave 1: F1 Document it (lead, CHANGELOG.md range a2dca30..HEAD, real commands in CLAUDE.md, CONTRIBUTING.md, README.md; commit docs(changelog); manual F1-1, F1-2 NOT_RUN) (+7 more)

### Community 53 - "Standards, Tooling and Release Qualification"
Cohesion: 0.18
Nodes (15): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), V1.5 Milestone, R-21 Proxmox host correlated failure, High availability PRD, ControllerEpoch, Endpoint failover via external load balancing or controlled VIP, Established consensus for one operation authority (+7 more)

### Community 54 - "Protocol Contract and Driver Compatibility: Driver compatibility matrix"
Cohesion: 0.17
Nodes (15): Protocol compatibility contract (3.0 and 3.2), COPY bounded chunk streaming, Extended Query and pipelining behavior, Variable length cancellation keys (protocol 3.2, up to 256 bytes), Virtual per lease cancellation keys, Driver compatibility matrix, Python asyncpg (V1 gate), Bun SQL/PostgreSQL client (V1 gate) (+7 more)

### Community 55 - "check-config Integration Tests: pool()"
Cohesion: 0.19
Nodes (14): a_value_after_config_is_always_the_path(), ac10_help_and_usage_errors_are_not_json_even_when_json_was_asked(), ac11_a_usage_error_never_repeats_what_was_typed(), ac11_an_argument_that_is_not_utf8_is_ignored_after_help_or_version(), ac11_an_error_met_before_help_still_wins(), ac11_arguments_that_are_not_utf8_anywhere_else_are_unexpected(), ac11_help_and_version_as_first_argument_ignore_everything_after(), ac11_usage_errors_look_the_same_whatever_the_format_says() (+6 more)

### Community 56 - "Security Requirements and Features F-100 to F-108: Health monitoring PRD"
Cohesion: 0.25
Nodes (14): F-060 Network partitions (health changes eligibility only), Health features F-054 to F-060, Health monitoring PRD, AC-1 Reachable wrong-role or wrong-cluster node cannot become eligible, AC-3 Expired observation refused at acquisition after publisher pause, Bounded authenticated probes with dedicated monitor role, Failure/recovery thresholds and flapping control, HealthState (Unknown, Healthy, Suspect, Unavailable, Recovering) (+6 more)

### Community 57 - "Feature Families and Planned Scopes: Roadmap"
Cohesion: 0.23
Nodes (14): Tracer Bullet build approach, Walking Slice Milestone, R-24 Walking slice exceeds its estimate, Roadmap, Delivery slices, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy (+6 more)

### Community 58 - "Fixture"
Cohesion: 0.30
Nodes (4): Fixture, PathBuf, Self, String

### Community 59 - "Workspace Layout, Bounded Read and FIFO Limit: Shared Conventions Table"
Cohesion: 0.42
Nodes (13): Data Plane vs Control Plane Separation, Planned Architecture, pool check-config Command, Repository proposal, apps/pool daemon, apps/pool-cli admin CLI, Core and Control Independence, Merge or Defer Candidate Crates (no common crate) (+5 more)

### Community 60 - "Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Workspace guards W1-W7"
Cohesion: 0.15
Nodes (13): W1 lockfile package set guard, W2 forbidden paths guard, W3 forbidden source patterns guard, W4 lint inheritance guard, W5 internal dependencies guard, W6 unexpected Cargo.lock guard, Workspace guards W1-W7, Baseline W2: paths that must not exist (apps/pool-cli, crates/pool-protocol, crates/pool-core, crates/pool-control) (+5 more)

### Community 61 - "SQL Routing and Replica Profiles: Protocol engine: Control plane"
Cohesion: 0.29
Nodes (12): Cluster management, Authenticated SQL health probes, Configured backend registration, Drain, attach and quarantine operations, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Control plane, ApprovedTopology record, AuditEvent record (+4 more)

### Community 62 - "Pgpool Baseline, Competitors and Feasibility: Pgpool configuration coverage inventory"
Cohesion: 0.23
Nodes (12): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Pgpool baseline, Baseline gaps (planning inventory, not parity certification), License and source reuse policy, PCP commands, SQL commands and pgpool_adm reference surface, Tagged sample and source manifest (V4_7_3) (+4 more)

### Community 63 - "SQL Routing and Replica Profiles: Protocol engine: Cluster management"
Cohesion: 0.29
Nodes (11): Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table, Operation admission boundary, SQL routing, Routing decision matrix, Eventual read consistency for replica sessions, No primary fallback for replica profiles (+3 more)

### Community 64 - "SQL Routing and Replica Profiles: Protocol engine: Protocol engine"
Cohesion: 0.18
Nodes (11): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Versioned local admin REST API (Axum), pg_query 18.1.0 behind an adapter, Proposed stack (+3 more)

### Community 65 - "Glossary and Routing Profiles: Approved Writer"
Cohesion: 0.22
Nodes (11): Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Approved Writer, Eligible Primary, Eligible Replica, Fresh (observation freshness), Quarantine (+3 more)

### Community 66 - "Pgpool Baseline, Competitors and Feasibility: Feasibility study"
Cohesion: 0.18
Nodes (11): Feasibility study, Automatic promotion (not ready for initial release), Go or no go milestones, Commercial and maintenance assumptions, Manual promotion (feasible with external dependencies), Operational ownership boundaries, Query result caching (high uncertainty), Session pooling feasibility (high feasibility, high correctness risk) (+3 more)

### Community 67 - "Scope 002 Brief Rules: 2. Standards and tooling (in-progress)"
Cohesion: 0.33
Nodes (11): Linux build evidence (a macOS build is not proof of the Linux build), Local toolchain (Rust stable 1.99.1, 1.100.0 available via rustup, host aarch64-apple-darwin, Docker 29.9.2 with daemon off), Manual steps policy (Docker Desktop, GitHub settings, PR, push, system tools, real PostgreSQL or VM are user only; blocked steps recorded as BLOCKED owner user), Resolved former blocker: Linux build evidence blocked while the Docker daemon was off; user started Docker Desktop (F1-1 PASS), History after closing (2026-11-14): Linux build passed at 18e9228 (F1-1, F1-2 PASS), evidence merged from claude/chk-f1-verify-2-dcd4b8, feature 1 done, spec 0001 Accepted; remaining VM architecture (F1-N1) and Follow-up 11, History wave 1 (from 591317f): all seven scope 001 boxes ticked; open at that time the Linux build (AC-2, /dev/full clause of AC-11, two Linux only tests), so feature 1 and spec 0001 stayed in progress; blockers Docker off (F1-1, F1-2), VM architecture (F1-N1); the two untracked files that held the push done at d2fda37; scope 002 next, Follow-up 11 waits for /scope, Manual step F1-1: start Docker Desktop so the developer and verify sessions can run the Linux build (0001 AC-2); PASS 2026-11-14, user started Docker Desktop, docker info exit 0 (server 29.9.2), Follow-up 7: Docker Desktop started 2026-11-14 (docker info exits 0, server 29.9.2, aarch64), AC-2 no longer blocked; each Linux build run still needs the daemon (+3 more)

### Community 68 - "Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Document checks D1-D6"
Cohesion: 0.29
Nodes (11): AC-10 Stable requirement IDs, AC-8 Links and anchors, AC-9 Scope status agreement, Build plan step 6: Repository check program, document rules, D1 local link path exists, D2 link fragment is a heading anchor, D3 scope index and feature files agree, D4 no ID defined twice (+3 more)

### Community 69 - "Review Clarifications and check-config Usage Rules: repo-check/src/main.rs"
Cohesion: 0.31
Nodes (8): Args, main(), parse(), Option, PathBuf, Result, String, run()

### Community 70 - "Review Clarifications and check-config Usage Rules: pool/src/main.rs"
Cohesion: 0.33
Nodes (9): check_config(), main(), read_bounded(), Result, Vec, write_stdout(), Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary, Rule check: the read bound fallback is 1_048_577, not u64::MAX; /dev/zero ends with config.too_large (+1 more)

### Community 71 - "Safety Invariants and Session Risks"
Cohesion: 0.20
Nodes (10): Separate Frontend and Backend Identity, F-030 Validation and reset (close uncertain sessions), Backend Login, Identity Map, Pool Key, Verified Principal, R-03 Session state leakage, R-11 Authentication or role confusion (+2 more)

### Community 72 - "Pgpool Baseline, Competitors and Feasibility: Competitor analysis"
Cohesion: 0.47
Nodes (9): Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC), PgCat (Rust/Tokio routing proxy, MIT), Competitor reference roles recommendation, Pgpool-II 4.7.3 (selected baseline) (+1 more)

### Community 73 - "Protocol Contract and Driver Compatibility: Driver compatibility matrix: PostgreSQL protocol analysis"
Cohesion: 0.22
Nodes (9): Proposed stopping rules, PostgreSQL protocol analysis, Close uncertain sessions cleanup policy, Independent backend credentials per mapped login identity, Proposed protocol limits (64 KiB startup, 16 MiB message), Replication and special clients, SCRAM channel binding per terminated TLS channel, Session state inventory (+1 more)

### Community 74 - "Pgpool Baseline, Competitors and Feasibility: Streaming sample census (CFG-001 to CFG-207, 207"
Cohesion: 0.22
Nodes (9): Pgpool Watchdog coordination, Connections and identity option family (48 options), Logs, files and reset option family (28 options), Relation metadata option family (6 options), Canonical runtime declaration census (RT-001 to RT-186), Source-only delta names (hidden aliases, health_check_test, replication_mode), Streaming and health option family (24 options), Streaming sample census (CFG-001 to CFG-207, 207 spellings) (+1 more)

### Community 75 - "Glossary and Routing Profiles: Topology Snapshot"
Cohesion: 0.32
Nodes (8): Frontend owner task, Architecture overview, Application path sequence, Proposed public interfaces (FrameDecoder, SessionState, Lease, RouteDecision, ProbeObservation, OperationPlan), Admission, Lease Generation, Snapshot Generation, Topology Snapshot

### Community 76 - "SQL Routing and Replica Profiles: Protocol engine: Promotion operation stages (Plan, Quiesce, Fence"
Cohesion: 0.32
Nodes (8): Deployment reference, Initial four VM lab, Future multi proxy deployment, Independent fencing authority, Deployment ownership matrix, External etcd for future coordination, Shared process failure domain, Operational V1 (four VM scenario with replica routing and fenced manual promotion)

### Community 77 - "SQL Routing and Replica Profiles: Protocol engine: Promotion operation stages (Plan, Quiesce, Fence: Promotion operation stages (Plan, Quiesce, Fence"
Cohesion: 0.43
Nodes (8): Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter), Manual failover policy, Recovery with pg_rewind and pg_basebackup, Promotion operation stages (Plan, Quiesce, Fence, Revalidate, Promote, Reconcile, Recover), PostgreSQL 18 documentation references

### Community 78 - "Glossary and Routing Profiles: Lease"
Cohesion: 0.33
Nodes (7): Virtual Cancellation Model, F-018 CancelRequest and BackendKeyData, Backend, Frontend, Lease, Session Permit, Virtual Cancellation Credential

### Community 79 - "Glossary and Routing Profiles: Session Model (pinned backend lease)"
Cohesion: 0.29
Nodes (7): Session Model (pinned backend lease), F-024 Transaction Mode (refused, session pooling only), Primary Profile, Replica Profile, Routing Profile, Routing Preview, R-06 Transaction consistency change

### Community 80 - "Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling"
Cohesion: 0.48
Nodes (7): Spec 0001 lifecycle status mirrors scope feature 1: In Progress while the foundation is built and proven, Accepted when the feature is done, Decision: Build plan step 1 sets spec Status to In Progress; the stale 'lifecycle status remains Proposed' wording is tidied at /sync, not by sub sessions, Feature 2 is not done without linked real CI evidence, Status lifecycle, Follow-up 1: foundation and decision ledger ratified; the build of scope 001 was requested on 2026-11-14; every later scope stays queued until requested, Decision status sentence: ratification settles the content; the Status line follows scope feature 001 per the status lifecycle, Clarification 9: Decision status sentence and Follow-up 1 reworded to hold in every status; runner up leave to the lead at close

### Community 81 - "Feature Inventory and Parity Matrix: Query cache PRD"
Cohesion: 0.38
Nodes (7): Manual failover first; automatic failover and cache disabled, V2 Milestone, Phase 6 Advanced features, Query cache PRD, Caching disabled until a separate contract is approved and proven, Candidate CacheKey, Cache invalidation and dependency contract

### Community 82 - "Pgpool Baseline, Competitors and Feasibility: Streaming sample census (CFG-001 to CFG-207, 207: Pgpool baseline modes and POOL dispositions"
Cohesion: 0.50
Nodes (4): Pgpool baseline modes and POOL dispositions, native_replication and snapshot_isolation modes (deferred), streaming_replication mode (initial operating model), Replication and routing option family (17 options)

### Community 83 - "Administration, Cluster and Health PRDs: Administration PRD: R-09 Failover loses acknowledged commits"
Cohesion: 0.67
Nodes (3): F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Write Barrier, R-09 Failover loses acknowledged commits

### Community 84 - "Slices, Milestones and Readiness Gates: KPIs and proposed gates"
Cohesion: 0.67
Nodes (3): Failover safety KPI (zero unfenced promotions or silent replays), KPIs and proposed gates, Overhead investigation triggers (10 percent TPS, 1 ms p99)

## Knowledge Gaps
- **92 isolated node(s):** `User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested`, `Protected assets and threat actors`, `F-060 Network partitions (health changes eligibility only)`, `Concurrency group per event name and ref; only pull request runs are cancelled`, `Permissions: contents read only, no secret` (+87 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **13 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Spec 0001. POOL initial architecture` connect `Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture` to `Slices, Milestones and Readiness Gates`, `Spec 0002: Standards and tooling`, `Scope 001 Coordination Brief, Log and Authorization`, `Security Requirements and Features F-100 to F-108: Reviewer roster (12 review and risk owner roles,`, `Offline Bounds, Parser Depth and Known Limits`, `Workspace Layout, Bounded Read and FIFO Limit`, `Linux Build Evidence, AC-2 and VM Architecture`, `Scaffold Step and dev-f1-build Gate: 0001. Verification record`, `Scope 002 Brief Rules`, `Configuration Keys and Secret References`, `Load Balancing and Replica Selection`, `Tracing, Log Redaction and Logging Decisions`, `Leases, Cancellation and Login Routing`, `Rust Technology Evaluation: Adopted in scope 002 (tools, not product depende: Rust technology evaluation`, `Security Requirements and Features F-100 to F-108`, `Glossary and Routing Profiles`, `Safety Invariants and Session Risks: Routing Features (F-033 to F-047)`, `Scope 002 Brief Rules: apps/pool agent guide`, `SQL Routing and Replica Profiles: Protocol engine`, `Workspace Layout, Bounded Read and FIFO Limit: Adopted dependencies (serde, toml, serde_json)`, `Changelog, Feature 1 Done and Spec Status`, `Slices, Milestones and Readiness Gates: Engineering standards`, `Scope 002 Brief Rules: Seven required checks: fmt, clippy, test, deny, `, `Load Balancing and Replica Selection: SQL routing PRD`, `Changelog, Feature 1 Done and Spec Status: 1. Stack and workspace (done)`, `Standards, Tooling and Release Qualification`, `Security Requirements and Features F-100 to F-108: Health monitoring PRD`, `Workspace Layout, Bounded Read and FIFO Limit: Shared Conventions Table`, `Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Workspace guards W1-W7`, `SQL Routing and Replica Profiles: Protocol engine: Protocol engine`, `Glossary and Routing Profiles: Approved Writer`, `Scope 002 Brief Rules: 2. Standards and tooling (in-progress)`, `Safety Invariants and Session Risks`, `Glossary and Routing Profiles: Topology Snapshot`, `SQL Routing and Replica Profiles: Protocol engine: Promotion operation stages (Plan, Quiesce, Fence`, `Glossary and Routing Profiles: Lease`, `Scope 002 Brief Rules: Brief general rules: 002-standards-and-tooling`, `Feature Inventory and Parity Matrix: Query cache PRD`?**
  _High betweenness centrality (0.257) - this node is a cross-community bridge._
- **Why does `Workspace guards W1-W7` connect `Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture: Workspace guards W1-W7` to `Spec 0002: Standards and tooling`, `Slices, Milestones and Readiness Gates: Spec 0001. POOL initial architecture`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Why does `MemTree` connect `Doc` to `pool-config Validation Library`, `Report`, `pool CLI Entry and Argument Parsing: cli.rs: ci.rs`, `Review Clarifications and check-config Usage Rules`, `workspace.rs`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Spec 0002: Standards and tooling` (e.g. with `docs/adr section row: ADR 0002. Standards and tooling` and `F2-N3 Whether and when a second maintainer GitHub account exists (approvals 0 to 1)`) actually correct?**
  _`Spec 0002: Standards and tooling` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` (e.g. with `Added: exit status convention for POOL commands (0 success, 1 input rejected, 2 usage error, 3 could not finish)` and `Review strengths: no file text reaches any output (toml error used for its span only), bounded read checked twice, locked writes with exit 3 on closed pipe, byte for byte tests on the real binary`) actually correct?**
  _`AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full)` has 6 INFERRED edges - model-reasoned connections that need verification._
- **What connects `User asked for scope 001 on 2026-11-14 and nothing beyond it; further implementation begins only when explicitly requested`, `Protected assets and threat actors`, `F-060 Network partitions (health changes eligibility only)` to the rest of the system?**
  _92 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Doc` be split into smaller, more focused modules?**
  _Cohesion score 0.05554386703134862 - nodes in this community are weakly interconnected._