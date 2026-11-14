# Graph Report - pool  (2026-11-14)

## Corpus Check
- 1 files · ~48,204 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 640 nodes · 1657 edges · 31 communities
- Extraction: 73% EXTRACTED · 27% INFERRED · 0% AMBIGUOUS · INFERRED: 445 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Scope Index and Documentation Map
- Scope 001 Coordination Brief and Log
- Feature Inventory and Safety Invariants
- Administration and Cluster PRDs
- Architecture Spec and Repository Layout
- Protocol Contract and Driver Compatibility
- Testing Strategy and Release Gates
- Health Monitoring and Load Balancing
- Glossary and Routing Profiles
- High Availability and Pooling PRDs
- Roadmap and Routing PRDs
- Architecture Rationale and Stack
- Connection Pooling and Leases
- Fencing and Split Brain
- Rust Technology Evaluation
- SQL Routing and Replica Profiles
- Manual Failover and Recovery
- Pgpool Baseline and Configuration Coverage
- Threat Model and Dependency Risk
- Operation Journal and Control Records
- Security Requirements and Review
- Feasibility Study Findings
- Pgpool Option Families
- Trust Boundaries
- Authentication Security Features
- Competitor Analysis
- No Replay of Uncertain Work
- Ten Safety Contracts
- Resource Limits and Frame Safety
- Pgpool Baseline Modes
- Recovery Adapters and Command Injection

## God Nodes (most connected - your core abstractions)
1. `Documentation index` - 74 edges
2. `Glossary` - 49 edges
3. `Feature inventory and parity matrix` - 47 edges
4. `Testing strategy` - 43 edges
5. `Scope: POOL, order and status index` - 42 edges
6. `Spec 0001. POOL initial architecture` - 39 edges
7. `Security requirements` - 37 edges
8. `Risk register` - 34 edges
9. `4. One real primary path (planned)` - 26 edges
10. `Health monitoring PRD` - 25 edges

## Surprising Connections (you probably didn't know these)
- `Credentials, cancellation secrets and sensitive SQL excluded from default telemetry` --semantically_similar_to--> `SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Administration requires authentication and authorization` --semantically_similar_to--> `SEC-6 Authenticate all administration`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Invariant: automatic failover and result caching stay disabled until approved and proven` --semantically_similar_to--> `SEC-10 Disable automatic promotion and result caching by default`  [INFERRED] [semantically similar]
  AGENTS.md → docs/security/security-requirements.md
- `Highest initial risk priorities (identity/state, unsafe routing, cancel races, uncertain outcomes, fencing)` --semantically_similar_to--> `POOL Safety Invariants`  [INFERRED] [semantically similar]
  docs/plan/risk-register.md → AGENTS.md
- `Engineering Workflow` --semantically_similar_to--> `Agent Workflow`  [INFERRED] [semantically similar]
  docs/engineering-standards.md → AGENTS.md

## Hyperedges (group relationships)
- **Graph refresh ownership during coordination (lead updates graphify-out on main, deviating from AGENTS.md step 6)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_graph_update_ownership, docs_scope_coordination_001_stack_and_workspace_brief_umum_forbidden_paths, agents, docs_scope_coordination_001_stack_and_workspace_brief_umum_lead_session, docs_scope_coordination_001_stack_and_workspace_log_decision_graph_ownership [INFERRED 0.85]
- **Scope 001 wave 1 delivery pipeline (build notes, scaffold, verify, test, review)** — docs_scope_coordination_001_stack_and_workspace_brief_umum_work_order, docs_scope_coordination_001_stack_and_workspace_log_arc_f1_update, docs_scope_coordination_001_stack_and_workspace_log_dev_f1_build, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_verify, docs_scope_coordination_001_stack_and_workspace_log_tst_f1_test, docs_scope_coordination_001_stack_and_workspace_log_chk_f1_review [INFERRED 0.95]
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
- **Linux build evidence blocked on the user starting the Docker daemon** — docs_scope_coordination_001_stack_and_workspace_brief_umum_manual_steps_policy, docs_scope_coordination_001_stack_and_workspace_brief_umum_linux_build_evidence, docs_scope_coordination_001_stack_and_workspace_brief_umum_local_toolchain, docs_scope_coordination_001_stack_and_workspace_manual_f1_1_start_docker_desktop, docs_scope_coordination_001_stack_and_workspace_log_blocker_docker_daemon_off [INFERRED 0.85]
- **Session pinned lease model** — docs_architecture_connection_pooling_session_pooling, docs_architecture_sql_routing_login_time_backend_selection, docs_architecture_data_plane_initial_admission, docs_architecture_data_plane_operation_admission, docs_architecture_sql_routing_no_primary_fallback, docs_architecture_data_plane_no_retry_after_dispatch, docs_specs_0001_initial_architecture_index_virtual_cancellation_credentials [INFERRED 0.85]
- **Write Authority Verification (approved writer plus fresh matching observation)** — docs_glossary_approved_writer, docs_glossary_eligible_primary, docs_glossary_fresh, docs_glossary_observation, docs_glossary_topology_epoch, docs_feature_matrix_f_065, docs_plan_risk_register_r_19 [INFERRED 0.85]
- **Automatic failover and result caching disabled by default** — agents_automatic_failover_and_result_caching_disabled, docs_feature_matrix_f_088_optional_memory_cache, docs_security_security_requirements_sec_10_disable_automatic_promotion_and_caching, docs_feature_matrix_initial_exclusions [INFERRED 0.95]

## Communities (31 total, 0 thin omitted)

### Community 0 - "Scope Index and Documentation Map"
Cohesion: 0.06
Nodes (91): POOL agent guide (AGENTS.md), Agent workflow (read scope, inspect reality, define slice, apply standards, validate, update graph), Apache 2.0 project license, Documentation Ownership Roles (scope, specs, PRDs, ADRs), Documentation ownership model (scope, specs, PRDs, ADRs, research), GA workflow depth (behavior verification, tests, fresh review, documentation), Knowledge graph refresh before commit (graphify-out kept current), POOL: PostgreSQL 18 proxy, session pooler and cluster operations project in Rust (+83 more)

### Community 1 - "Scope 001 Coordination Brief and Log"
Cohesion: 0.09
Nodes (54): First Handoff: the Workspace, pool check-config Command, Aturan umum brief: 001-stack-and-workspace (general brief rules), Allowed downloads (crates adopted by build notes, pinned Rust toolchain, container images only with running Docker daemon), Change path for scope 001: no PR, per sub session branch, lead merges to main with --no-ff after green gate, Commit convention (conventional commits per Build plan step, subject 'type(scope): spec 0001 step n', spec status Proposed to In Progress), coordinate skill (.claude/skills/coordinate: roles, decision modes, report template), Decision mode auto (architect asks lead to accept spec; others interpret ambiguous criteria, needs-decision only when spec text must change) (+46 more)

### Community 2 - "Feature Inventory and Safety Invariants"
Cohesion: 0.06
Nodes (52): Administration requires authentication and authorization, Invariant: automatic failover and result caching stay disabled until approved and proven, Invariant: close uncertain pooled state, Invariant: fail closed on unknown role or unsafe routing, Invariant: health failure does not authorize promotion, POOL Safety Invariants, Invariant: preserve transaction and session ownership, Invariant: keep writes on the verified primary (+44 more)

### Community 3 - "Administration and Cluster PRDs"
Cohesion: 0.07
Nodes (42): Planning assumptions and decisions, Apache 2.0 license, Persisted approved writer, epoch and quarantine, Authoritative barrier across every write path, Axum REST administration API plus CLI, Manual failover first; automatic failover and cache disabled, GA verification depth for data safety features, Independent review resolutions (seven approved fixes) (+34 more)

### Community 4 - "Architecture Spec and Repository Layout"
Cohesion: 0.14
Nodes (32): Virtual Cancellation Model, Data Plane vs Control Plane Separation, Planned Architecture, ADR 0001. Initial architecture, Frontend owner task, No automatic retry after dispatch, Architecture overview, Application path sequence (+24 more)

### Community 5 - "Protocol Contract and Driver Compatibility"
Cohesion: 0.07
Nodes (32): Protocol engine, Bounded frame validation, Authentication adapter (reviewed pgwire components), Protocol finite state machine, Transaction state tracking (ReadyForQuery status), Backend Login, Support Profile, Proposed stopping rules (+24 more)

### Community 6 - "Testing Strategy and Release Gates"
Cohesion: 0.12
Nodes (29): Tests and Evidence Standard, First deployment: one proxy VM, primary and two replicas on Proxmox, V1 Release Gate, criterion 0.8.2 (microbenchmarks), proptest 1.12.0 (state and framing properties), Tooling choices owed by feature 2 (CI service, dependency/license/advisory checker, SBOM generator, fuzz harness, PostgreSQL 18 supply), 11. Release qualification harness (planned), Qualified release artifacts (tagged package or image, checksums, SBOM, notices) (+21 more)

### Community 7 - "Health Monitoring and Load Balancing"
Cohesion: 0.12
Nodes (27): Authenticated SQL health probes, Backend health state machine (Unknown, Healthy, Suspect, Unavailable, Recovering), Observation record, Health features F-054 to F-060, No primary fallback in V1, Load balancing PRD, CandidateSet, choose(profile, snapshot, selector_state) (+19 more)

### Community 8 - "Glossary and Routing Profiles"
Cohesion: 0.16
Nodes (24): Separate Frontend and Backend Identity, Balancing features F-048 to F-053, F-053 Lag Filtering and Primary Fallback, F-053 Lag filtering and primary fallback (fallback disabled in V1), F-056 Primary Standby Role Observation, Glossary, Approved Writer, Credential Generation (+16 more)

### Community 9 - "High Availability and Pooling PRDs"
Cohesion: 0.11
Nodes (24): Retry only before initial lease and ReadyForQuery exposure, Operator map from verified principal and database to backend identity, Private backend keys and virtual frontend cancellation credentials, Cancellation forwarded only through the live lease owner, CancellationEntry, Lease, Session, Connection pooling PRD (+16 more)

### Community 10 - "Roadmap and Routing PRDs"
Cohesion: 0.14
Nodes (23): Policy Version, pg_query parser with PostgreSQL 18 grammar, V2 Milestone, Roadmap, Phase 0 Research and architecture, Phase 1 Foundation, Phase 2 Core proxy, Phase 3 SQL routing (+15 more)

### Community 11 - "Architecture Rationale and Stack"
Cohesion: 0.11
Nodes (20): Session pooling (one frontend owns one backend lease), Transaction pooling (deferred), Future multi proxy deployment, Transparent per query routing (later feature), rustls 0.23.46 (TLS), Single rustls crypto provider (aws-lc-rs feature family), Versioned local admin REST API (Axum), Deferred features (cache, automatic failover, native replication, multiplexing) (+12 more)

### Community 12 - "Connection Pooling and Leases"
Cohesion: 0.16
Nodes (17): Connection pooling, Backend cleanup and reset (rollback, DISCARD ALL, idle ReadyForQuery), Backend connection lifecycle state machine (Connecting, Idle, Leased, Cleaning, Draining, Closed), Pool limits and operations, Statement pooling (not planned for V1), Data plane, Provisional resource defaults and backpressure, Admission (+9 more)

### Community 13 - "Fencing and Split Brain"
Cohesion: 0.21
Nodes (14): Invariant: promotion requires a reviewed operation and independently verified fencing, F-067 Manual plan and promotion (operator plan plus verified fence), F-069 Old primary fencing (external verified exclusion), F-070 Split brain prevention (mutation authority distinct from fencing), F-071 Uncertain promotion result (durable intent and reconciliation), F-072 RPO and commit uncertainty (all-writer barrier, explicit unknown loss window), Failover features F-067 to F-075, Fence Receipt (+6 more)

### Community 14 - "Rust Technology Evaluation"
Cohesion: 0.18
Nodes (14): Technology Stack (Tokio, rustls, pg_query, Axum, SQLite journal), Transparent arbitrary read splitting (low confidence), SCRAM channel binding per terminated TLS channel, Rust technology evaluation, Dependency adoption gate, axum 0.8.9 (administration endpoints), Hybrid parser policy, pg_query 18.1.0 and libpg_query (PostgreSQL raw parser) (+6 more)

### Community 15 - "SQL Routing and Replica Profiles"
Cohesion: 0.23
Nodes (14): Cluster management, Configured backend registration, Drain, attach and quarantine operations, Replication freshness and byte lag, Weighted round robin replica selection, Value sourcing table, Operation admission boundary, SQL routing (+6 more)

### Community 16 - "Manual Failover and Recovery"
Cohesion: 0.26
Nodes (13): Deployment reference, Initial four VM lab, Independent fencing authority, Deployment ownership matrix, Failover and recovery, All-writer write barrier, Candidate and RPO policy, Fencing contract (allowlisted adapter) (+5 more)

### Community 17 - "Pgpool Baseline and Configuration Coverage"
Cohesion: 0.23
Nodes (12): Configuration features F-115 to F-119, F-117 Pgpool Parameter Coverage (207 sample spellings), F-117 Pgpool parameter coverage (207 sample spellings mapped), Pgpool baseline, Baseline gaps (planning inventory, not parity certification), License and source reuse policy, PCP commands, SQL commands and pgpool_adm reference surface, Tagged sample and source manifest (V4_7_3) (+4 more)

### Community 18 - "Threat Model and Dependency Risk"
Cohesion: 0.20
Nodes (11): Safe first party Rust and bounded resources, Dependency and Configuration Change Rules, F-101 TLS and mTLS verification, Configuration Generation, R-16 Dependency drift or abandoned crate, SEC-1 Require TLS and verify backend CA plus server name, Threat model, Protected assets and threat actors (+3 more)

### Community 19 - "Operation Journal and Control Records"
Cohesion: 0.31
Nodes (11): Control plane, ApprovedTopology record, AuditEvent record, FenceReceipt record, Operation record, Local SQLite operation journal (single writer), Expected Generation, Operation (journaled administrative action) (+3 more)

### Community 20 - "Security Requirements and Review"
Cohesion: 0.29
Nodes (10): Credentials, cancellation secrets and sensitive SQL excluded from default telemetry, Security and Review Requirements, F-106 Audit and log redaction, Reviewer roster (who holds each review role, where a fresh model may review), Security requirements, SEC-11 Logs/metrics/traces free of credentials, cancellation keys and raw SQL, SEC-14 Independent review of security sensitive, protocol, parser and HA changes, SEC-4 Secrets in protected files or provider references; redaction (+2 more)

### Community 21 - "Feasibility Study Findings"
Cohesion: 0.20
Nodes (10): Feasibility study, Automatic promotion (not ready for initial release), Go or no go milestones, Commercial and maintenance assumptions, Manual promotion (feasible with external dependencies), Operational ownership boundaries, Query result caching (high uncertainty), Session pooling feasibility (high feasibility, high correctness risk) (+2 more)

### Community 22 - "Pgpool Option Families"
Cohesion: 0.20
Nodes (10): Pgpool Watchdog coordination, Connections and identity option family (48 options), Logs, files and reset option family (28 options), Relation metadata option family (6 options), Canonical runtime declaration census (RT-001 to RT-186), Source-only delta names (hidden aliases, health_check_test, replication_mode), Streaming and health option family (24 options), Streaming sample census (CFG-001 to CFG-207, 207 spellings) (+2 more)

### Community 23 - "Trust Boundaries"
Cohesion: 0.33
Nodes (10): Operational security defaults, Control module, Data plane, Database trust domain, Operation journal, Operator management domain, T-11 Journal tampering, loss or disk full, Telemetry domain (+2 more)

### Community 24 - "Authentication Security Features"
Cohesion: 0.31
Nodes (9): F-100 SCRAM SHA 256 and PLUS (terminated frontend auth, separate backend mapping), F-108 Secure reload (validate entire generation before publication), Security features F-100 to F-108, R-11 Authentication or role confusion, SEC-2 Independently authenticated frontend and backend identities, SEC-3 SCRAM PLUS only with valid channel binding; bounded work factors, T-02 Password or SCRAM downgrade, T-04 Role confusion and cross tenant reuse (+1 more)

### Community 25 - "Competitor Analysis"
Cohesion: 0.47
Nodes (9): Competitor analysis, Compare only equivalent pool modes at pinned releases, HAProxy (general TCP/HTTP proxy, GPL), Odyssey (C multithreaded pooler, BSD 3 Clause), PgBouncer (C event driven pooler, ISC), PgCat (Rust/Tokio routing proxy, MIT), Competitor reference roles recommendation, Pgpool-II 4.7.3 (selected baseline) (+1 more)

### Community 26 - "No Replay of Uncertain Work"
Cohesion: 0.33
Nodes (7): Invariant: never replay uncertain dispatched work, Data and Protocol Safety Standards, No Replay After Dispatch Rule, R-10 Ambiguous COMMIT replay, R-12 Malformed frame vulnerability, No silent replay of uncertain work (lost COMMIT response), T-07 Commit replay duplicates effects

### Community 27 - "Ten Safety Contracts"
Cohesion: 0.53
Nodes (6): Pool key (pool identity), Durable approved writer authority, Initial admission boundary, Login time backend selection on pinned lease, Ten safety contracts, Independent critique and seven resolutions

### Community 28 - "Resource Limits and Frame Safety"
Cohesion: 0.40
Nodes (6): Rust and Runtime Standards, F-107 Rate and resource limits (admission, frame and auth CPU budgets), R-13 Memory exhaustion, SEC-8 Global and per principal connection, byte, queue, parser and auth CPU budgets, T-01 Frame confusion or memory exhaustion, Protocol unit tests (golden frames, split at every byte boundary)

### Community 29 - "Pgpool Baseline Modes"
Cohesion: 0.50
Nodes (4): Pgpool baseline modes and POOL dispositions, native_replication and snapshot_isolation modes (deferred), streaming_replication mode (initial operating model), Replication and routing option family (17 options)

### Community 30 - "Recovery Adapters and Command Injection"
Cohesion: 0.67
Nodes (3): Recovery Features (F-076 to F-080), SEC-9 Allowlisted structured fencing/recovery adapters, T-09 Command injection through fencing/recovery hooks

## Knowledge Gaps
- **52 isolated node(s):** `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165`, `Documentation ownership model (scope, specs, PRDs, ADRs, research)`, `Protected assets and threat actors` (+47 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Documentation index` connect `Scope Index and Documentation Map` to `Scope 001 Coordination Brief and Log`, `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Architecture Spec and Repository Layout`, `Protocol Contract and Driver Compatibility`, `Testing Strategy and Release Gates`, `Health Monitoring and Load Balancing`, `Glossary and Routing Profiles`, `High Availability and Pooling PRDs`, `Roadmap and Routing PRDs`, `Architecture Rationale and Stack`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `SQL Routing and Replica Profiles`, `Manual Failover and Recovery`, `Pgpool Baseline and Configuration Coverage`, `Threat Model and Dependency Risk`, `Operation Journal and Control Records`, `Security Requirements and Review`, `Feasibility Study Findings`, `Competitor Analysis`?**
  _High betweenness centrality (0.412) - this node is a cross-community bridge._
- **Why does `Spec 0001. POOL initial architecture` connect `Architecture Spec and Repository Layout` to `Scope Index and Documentation Map`, `Scope 001 Coordination Brief and Log`, `Feature Inventory and Safety Invariants`, `Administration and Cluster PRDs`, `Health Monitoring and Load Balancing`, `Glossary and Routing Profiles`, `High Availability and Pooling PRDs`, `Roadmap and Routing PRDs`, `Architecture Rationale and Stack`, `Connection Pooling and Leases`, `Rust Technology Evaluation`, `No Replay of Uncertain Work`, `Ten Safety Contracts`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Why does `Feature inventory and parity matrix` connect `Feature Inventory and Safety Invariants` to `Scope Index and Documentation Map`, `Administration and Cluster PRDs`, `Testing Strategy and Release Gates`, `Health Monitoring and Load Balancing`, `Glossary and Routing Profiles`, `High Availability and Pooling PRDs`, `Roadmap and Routing PRDs`, `Fencing and Split Brain`, `Pgpool Baseline and Configuration Coverage`, `Authentication Security Features`, `Recovery Adapters and Command Injection`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Feature inventory and parity matrix` (e.g. with `Initial Package Checks` and `Testing strategy`) actually correct?**
  _`Feature inventory and parity matrix` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Are the 2 inferred relationships involving `Testing strategy` (e.g. with `Feature inventory and parity matrix` and `proptest 1.12.0 (state and framing properties)`) actually correct?**
  _`Testing strategy` has 2 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Knowledge graph refresh before commit (graphify-out kept current)`, `Observability features F-109 to F-114`, `Runtime config hidden aliases F-161 to F-165` to the rest of the system?**
  _52 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope Index and Documentation Map` be split into smaller, more focused modules?**
  _Cohesion score 0.05714285714285714 - nodes in this community are weakly interconnected._