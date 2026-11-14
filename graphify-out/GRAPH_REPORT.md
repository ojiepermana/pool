# Graph Report - pool  (2026-11-14)

## Corpus Check
- 5 files · ~57,973 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 745 nodes · 2001 edges · 27 communities
- Extraction: 76% EXTRACTED · 24% INFERRED · 0% AMBIGUOUS · INFERRED: 489 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Scope Index and Documentation Map
- Scope 001 Coordination Brief and Log
- Feature Inventory and Safety Invariants
- Administration and Cluster PRDs
- Architecture Spec and Repository Layout
- Protocol Contract and Driver Compatibility
- Linux Build and Toolchain Evidence
- Reserved Capabilities and First Build Criteria
- Glossary and Routing Profiles
- Delivery Phases, Milestones and Risks
- Roadmap and Routing PRDs
- Architecture Rationale and Stack
- Connection Pooling and Leases
- Fencing and Split Brain
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Manual Failover and Recovery
- Pgpool Baseline, Competitors and Feasibility
- Configuration Validation Contract
- Operation Journal and Control Records
- Agent Guide, Standards and Security Requirements
- Workspace Skeleton and Locked Dependencies
- Findings, Validation Order and Logging
- Deployment Topology and Fencing Authority
- JSON Output and No Leak Rule
- check-config Command and Exit Status
- Approved Writer and Quarantine Terms

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
- `GA workflow depth` --semantically_similar_to--> `GA workflow depth (behavior verification, tests, fresh review, documentation)`  [INFERRED] [semantically similar]
  docs/scope/index.md → AGENTS.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Health failure alone produces no promotion` --semantically_similar_to--> `Invariant: health failure does not authorize promotion`  [INFERRED] [semantically similar]
  docs/prd/005-health-monitoring.md → AGENTS.md
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md

## Hyperedges (group relationships)
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Spec 0001 first build acceptance criteria AC-1 to AC-13** — docs_specs_0001_initial_architecture_index_ac_1, docs_specs_0001_initial_architecture_index_ac_2, docs_specs_0001_initial_architecture_index_ac_3, docs_specs_0001_initial_architecture_index_ac_4, docs_specs_0001_initial_architecture_index_ac_5, docs_specs_0001_initial_architecture_index_ac_6, docs_specs_0001_initial_architecture_index_ac_7, docs_specs_0001_initial_architecture_index_ac_8, docs_specs_0001_initial_architecture_index_ac_9, docs_specs_0001_initial_architecture_index_ac_10, docs_specs_0001_initial_architecture_index_ac_11, docs_specs_0001_initial_architecture_index_ac_12, docs_specs_0001_initial_architecture_index_ac_13 [EXTRACTED 1.00]
- **Spec 0001 build plan steps 1 to 5 (tracer bullet)** — docs_specs_0001_initial_architecture_index_build_plan_step_1, docs_specs_0001_initial_architecture_index_build_plan_step_2, docs_specs_0001_initial_architecture_index_build_plan_step_3, docs_specs_0001_initial_architecture_index_build_plan_step_4, docs_specs_0001_initial_architecture_index_build_plan_step_5 [EXTRACTED 1.00]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Scope 001 shared conventions (configuration keys, exit status and output, log events)** — docs_engineering_standards_configuration_keys_convention, docs_engineering_standards_exit_status_output_convention, docs_engineering_standards_log_event_convention, docs_specs_0001_initial_architecture_index_configuration_rules, docs_specs_0001_initial_architecture_index_exit_status_and_output, docs_specs_0001_initial_architecture_index_log_events_and_redaction [EXTRACTED 1.00]
- **Competitor reference set for POOL comparisons** — docs_research_pgpool_baseline_pgpool_ii_4_7_3, docs_research_competitor_analysis_pgbouncer, docs_research_competitor_analysis_pgcat, docs_research_competitor_analysis_odyssey, docs_research_competitor_analysis_haproxy [EXTRACTED 1.00]
- **POOL Workspace Units (two apps and four internal crates)** — docs_repository_structure_apps_pool, docs_repository_structure_apps_pool_cli, docs_repository_structure_pool_protocol, docs_repository_structure_pool_config, docs_repository_structure_pool_core, docs_repository_structure_pool_control [EXTRACTED 1.00]
- **Release Gate Ladder (Walking slice to Eventual parity)** — docs_plan_milestones_walking_slice, docs_plan_milestones_core_mvp, docs_plan_milestones_routing_preview, docs_plan_milestones_v1, docs_plan_milestones_v1_5, docs_plan_milestones_v2, docs_plan_milestones_eventual_parity [EXTRACTED 1.00]
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
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]

## Communities (27 total, 0 thin omitted)

### Community 0 - "Scope Index and Documentation Map"
Cohesion: 0.06
Nodes (96): Documentation Ownership Roles (scope, specs, PRDs, ADRs), Invariant: preserve transaction and session ownership, Planned Cargo Check Commands, Working with the Docs, Configuration keys convention (config_version, snake_case, unit suffixes, _file secrets, reserved tables), Exit status and command output convention (0/1/2/3, stdout result, one line JSON), Git and Release Standards, Log event fields and redaction convention (ts, level, event; redaction by type) (+88 more)

### Community 1 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.09
Nodes (53): Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change), Disk constraint (29 GiB free, max 2 full Rust builds, CARGO_PROFILE_DEV_DEBUG=0, CARGO_INCREMENTAL=0, delete target/ before done), Final report rule (one report per item via SendMessage, done only when committed, clean tree and merged main; STATUS line first) (+45 more)

### Community 2 - "Feature Inventory and Safety Invariants"
Cohesion: 0.06
Nodes (52): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust, Invariant: keep writes on the verified primary (+44 more)

### Community 3 - "Administration and Cluster PRDs"
Cohesion: 0.08
Nodes (43): Authenticated SQL health probes, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Observation record, F-101 TLS and mTLS verification, Authoritative barrier across every write path, Axum REST administration API plus CLI, pool check-config, Health monitoring PRD (+35 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.11
Nodes (39): Virtual Cancellation Model, Data Plane vs Control Plane Separation, Planned Architecture, Session Model (pinned backend lease), Pool key (pool identity), Data plane, Frontend owner task, Initial admission boundary (+31 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.07
Nodes (34): Operation admission boundary, Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Routing decision matrix, SQL classification through PostgreSQL 18 raw grammar (+26 more)

### Community 6 - "Linux Build and Toolchain Evidence"
Cohesion: 0.17
Nodes (17): First deployment: one proxy VM, primary and two replicas on Proxmox, Linux builder image rust:1.100.0-slim-bookworm by index digest, AC-2 Linux build (container check ends with 'linux build check passed'), Build plan step 4: Workstation evidence, Build plan step 5: Linux evidence, Follow-up: confirm the PostgreSQL and proxy VMs are x86_64, Follow-up: scope 002 runs the Linux build on a Linux runner without emulation, Linux build (container command and workstation checks) (+9 more)

### Community 7 - "Reserved Capabilities and First Build Criteria"
Cohesion: 0.22
Nodes (20): Automatic Promotion (reserved and off), Reserved Capability, Result Cache (reserved and off), SEC-10 Disable automatic promotion and result caching by default, AC-3 Valid configuration (valid-* fixtures exit 0 with exact report), AC-4 Off is safe (reserved capability absent, empty or false validates as disabled), AC-5 Result cache is refused (config.unsupported_capability at result_cache.enabled), AC-6 Automatic promotion is refused (config.unsupported_capability at automatic_promotion.enabled) (+12 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.16
Nodes (22): Separate Frontend and Backend Identity, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), Glossary, Backend Login, Credential Generation, Detach (+14 more)

### Community 9 - "Delivery Phases, Milestones and Risks"
Cohesion: 0.10
Nodes (30): HA Features (F-081 to F-087), Recovery Features (F-076 to F-080), Tracer Bullet build approach, V1.5 Milestone, Walking Slice Milestone, R-21 Proxmox host correlated failure, R-24 Walking slice exceeds its estimate, Roadmap (+22 more)

### Community 10 - "Roadmap and Routing PRDs"
Cohesion: 0.06
Nodes (49): Policy Version, Planning assumptions and decisions, Retry only before initial lease and ReadyForQuery exposure, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Manual failover first; automatic failover and cache disabled, GA verification depth for data safety features, Independent review resolutions (seven approved fixes) (+41 more)

### Community 11 - "Architecture Rationale and Stack"
Cohesion: 0.29
Nodes (7): rustls 0.23.46 (TLS), Single rustls crypto provider (aws-lc-rs feature family), Versioned local admin REST API (Axum), Proposed stack, rustls and tokio-rustls TLS, Tokio runtime with bounded tasks and channels, Typed Serde configuration from TOML

### Community 12 - "Connection Pooling and Leases"
Cohesion: 0.22
Nodes (11): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool limits and operations, Session pooling (one frontend owns one backend lease), Statement pooling (not planned for V1), Provisional resource defaults and backpressure, Backend (+3 more)

### Community 13 - "Fencing and Split Brain"
Cohesion: 0.11
Nodes (29): Invariant: promotion requires a reviewed operation and independently verified fencing, Safe first party Rust and bounded resources, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Failover features F-067 to F-075 (+21 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.18
Nodes (19): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), Rust technology evaluation, Adopted in scope 001 (three direct dependencies reviewed 2026-11-14), Dependency adoption gate, axum 0.8.9 (administration endpoints), clap 4.6.7 (CLI argument contract, not adopted in 001), Hybrid parser policy (+11 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.17
Nodes (19): Cluster management, Configured backend registration, Drain, attach and quarantine operations, Replication freshness and byte lag, Weighted round robin replica selection, Transaction pooling (deferred), Durable approved writer authority, Value sourcing table (+11 more)

### Community 16 - "Manual Failover and Recovery"
Cohesion: 0.33
Nodes (10): Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter), Manual failover policy, Recovery with pg_rewind and pg_basebackup, Promotion operation stages (Plan, Quiesce, Fence, Revalidate, Promote, Reconcile, Recover), Fence Receipt (+2 more)

### Community 17 - "Pgpool Baseline, Competitors and Feasibility"
Cohesion: 0.07
Nodes (44): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC) (+36 more)

### Community 18 - "Configuration Validation Contract"
Cohesion: 0.18
Nodes (18): Dependency and Configuration Change Rules, Configuration Generation, Configuration Version (config_version key), AC-8 Whole configuration (all findings in one run; validate is the only path to ValidatedConfig), Change path (scope 001 branches merged with --no-ff, no PR), config_version key (required, supports exactly 1), Configuration rules (shared configuration key convention), FindingCode (config.* codes) (+10 more)

### Community 19 - "Operation Journal and Control Records"
Cohesion: 0.33
Nodes (10): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer), Expected Generation, Operation (journaled administrative action) (+2 more)

### Community 20 - "Agent Guide, Standards and Security Requirements"
Cohesion: 0.05
Nodes (72): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), Invariant: never replay uncertain dispatched work, Credentials, cancellation secrets and sensitive SQL excluded from default telemetry (+64 more)

### Community 21 - "Workspace Skeleton and Locked Dependencies"
Cohesion: 0.21
Nodes (13): Sixteen locked packages in the scratch build, AC-1 Workspace and toolchain pin (two members, Rust 1.100.0, Cargo.lock committed), AC-12 Offline and bounded (no socket, thread or runtime; sixteen locked packages; bounded read), Bounded configuration read (at most 1048577 bytes via Read::take), Build plan step 1: Workspace skeleton, Cargo.lock with exactly sixteen locked packages, First build consequences (no leak by construction, terse findings, fixed public contracts), rust-toolchain.toml (channel 1.100.0, rustfmt and clippy) (+5 more)

### Community 22 - "Findings, Validation Order and Logging"
Cohesion: 0.23
Nodes (12): Finding (stable rejection code), tracing 0.1.44 (structured events and spans), Findings (stable finding codes), Log events and redaction (shared log convention), prometheus-client metrics and tracing logs, Validation order (length, UTF-8, syntax, schema, meaning), Decision: hand written Display and Error over thiserror and anyhow, Decision: POOL reports config.missing_key with the key path (+4 more)

### Community 23 - "Deployment Topology and Fencing Authority"
Cohesion: 0.32
Nodes (8): Deployment reference, Initial four VM lab, Future multi proxy deployment, Independent fencing authority, Deployment ownership matrix, External etcd for future coordination, Shared process failure domain, Operational V1 (four VM scenario with replica routing and fenced manual promotion)

### Community 24 - "JSON Output and No Leak Rule"
Cohesion: 0.32
Nodes (8): Output Version (output_version field), AC-10 JSON contract (one JSON object line on stdout, empty stderr), Exit status and output (shared output convention), First build decision (two packages, three dependencies, fixed finding vocabulary), JSON output contract (output_version, command, result ... findings), No file content in output (constant text, key paths, line and column only), Decision: diagnostics contain only constants, key paths, line and column, Parser behavior evidence (toml Display and message() leak the canary)

### Community 25 - "check-config Command and Exit Status"
Cohesion: 0.40
Nodes (6): AC-11 Exit status and streams (usage 2, io errors 3, help and version 0, /dev/full), The check-config command (arguments and usage errors), Exit status 0 success, 1 rejected, 2 usage error, 3 could not complete, pool check-config (offline configuration check command), What this build is and where it stops, Decision: exit status 0, 1, 2, 3 over sysexits

### Community 26 - "Approved Writer and Quarantine Terms"
Cohesion: 0.83
Nodes (4): Approved Writer, Quarantine, Topology Epoch, R-19 Restart adopts wrong writable node

## Knowledge Gaps
- **52 isolated node(s):** `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Values from the user (none required)`, `Protected assets and threat actors`, `Observability features F-109 to F-114` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Scope Index and Documentation Map` to `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Linux Build and Toolchain Evidence`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Pgpool Baseline, Competitors and Feasibility`, `Operation Journal and Control Records`, `Agent Guide, Standards and Security Requirements`, `Deployment Topology and Fencing Authority`?**
  _High betweenness centrality (0.367) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Agent Guide, Standards and Security Requirements` to `Scope Index and Documentation Map`, `Scope 001 Coordination Brief and Log`, `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Linux Build and Toolchain Evidence`, `Reserved Capabilities and First Build Criteria`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Architecture Rationale and Stack`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Configuration Validation Contract`, `Workspace Skeleton and Locked Dependencies`, `Deployment Topology and Fencing Authority`, `JSON Output and No Leak Rule`, `check-config Command and Exit Status`, `Approved Writer and Quarantine Terms`?**
  _High betweenness centrality (0.173) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Feature Inventory and Safety Invariants` to `Scope Index and Documentation Map`, `Administration and Cluster PRDs`, `Linux Build and Toolchain Evidence`, `Glossary and Routing Profiles`, `Delivery Phases, Milestones and Risks`, `Roadmap and Routing PRDs`, `Fencing and Split Brain`, `Pgpool Baseline, Competitors and Feasibility`, `Agent Guide, Standards and Security Requirements`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Knowledge graph refresh before commit (graphify-out kept current)`, `Values from the user (none required)` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope Index and Documentation Map` be split into smaller, more focused modules?**
  _Cohesion score 0.056578947368421055 - nodes in this community are weakly interconnected._