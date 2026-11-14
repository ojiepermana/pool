# Planning package validation

**Date:** 2026-11-14. This record concerns documents and design review only.

## Initial package checks

These results describe the original planning handoff, before the development readiness review below.

| Check | Result |
|---|---|
| Required deliverables | All 41 requested files exist and are nonempty |
| PRD structure | All 10 PRDs contain the 14 requested content sections |
| Proposed acceptance criteria | 41 criteria across the PRDs, with feature specific tests |
| Local Markdown links | All file targets resolve in the workspace |
| Scope organization | 17 feature files with prefixes 001 through 017, managed by docs/scope/index.md; original order, phase, status, intent, criteria and checklists preserved |
| Feature inventory | 165 uniquely identified rows with the required columns |
| Sample configuration inventory | 207 distinct option spellings |
| Runtime declaration inventory | 195 typed records, 186 distinct names, all 181 normalized sample names accounted for plus 5 source-only names |
| Canonical source fingerprint | Runtime declaration file digest and tagged tree object recorded in research |
| Architecture review | Another model reviewed the draft; you approved all seven recommended changes |
| Review readback | Seven decisions confirmed in foundation; two supporting wording inconsistencies corrected |
| Final acceptance | You ratified the assembled foundation on 2026-11-14 and explicitly left implementation queued |
| Implementation state | Zero Rust source files and zero Cargo manifests |
| Environment changes | No dependency installation, infrastructure deployment, database mutation or Git initialization |

## Development readiness review

**Reviewed:** 2026-11-14, starting at repository revision `5890686`. The review compared the numbered scope, delivery plan, ten proposed PRDs, ratified foundation, topic architecture, security requirements, testing and upstream inventories. It changes the plan and proposed acceptance cases; it does not record feature approval or implementation.

| When it matters | Finding and disposition | Owner in the plan |
|---|---|---|
| Before the first build | Different entry points could lead to scaffolding the target workspace or setting up a full database lab before testing the config parser. The first handoff now ends at the Linux build and offline validation. | [Scope 001 handoff](../implementation-order.md#first-handoff-the-workspace) |
| Before the first write | The old delivery graph connected the journal only to failover, leaving the first write without an explicit authority prerequisite. Scope 003 must settle bootstrap, loss/restore and identity mapping before 004 can admit a write. The auth adapter, grants and negative fixtures remain spec work. | [003 control model](../scope/003-control-and-session-data-model.md), [004 walking slice](../scope/004-one-real-primary-path.md) |
| Before pooling or replica claims | An isolation test using different roles could pass without reusing a socket. Some routing wording also allowed a reader to infer a retry on the primary. The PRDs now require observable reuse, refusal before dispatch and a check when the role changes during acquisition. | [Pooling](../prd/002-connection-pooling.md#acceptance-criteria), [routing](../prd/003-sql-routing.md#acceptance-criteria), [selection](../prd/004-load-balancing.md#acceptance-criteria) |
| Before administrative mutations | A lost reply, or a failed journal write after an external action, had no unambiguous caller outcome. The retry contract now distinguishes an identical request from a changed one and calls for reconciliation if the effect is unknown. The retry window and provider evidence still need a decision. | [Administration](../prd/010-administration.md#interfaces), [manual promotion](../scope/010-fenced-manual-promotion.md) |
| Before making a release claim | The compatibility list, inventories and plan could be read as separate promises. Release gates now distinguish the walking client, Core MVP and operational V1. Deferred features and configuration names point to their dispositions. | [Milestones](milestones.md), [driver profiles](../testing/compatibility-matrix.md), [configuration mapping](../research/pgpool-configuration-inventory.md) |
| At every change | Each PRD repeated the closure policy and used local AC numbers. Scope now links to owning criteria, and each slice records proof once under a qualified reference such as `PRD-001 AC-1`. | [Ownership](../index.md#where-a-change-belongs), [acceptance evidence](../testing/strategy.md#acceptance-evidence) |

**Checks run on the revised package:** All 68 tracked Markdown documents (63 under `docs/`) and the license are accounted for by the index. Local file and heading links resolve. All 17 scope numbers and statuses agree with their index; all ten PRDs retain their section structure and the same 41 AC identifiers. The 165 feature IDs are unique. Across the 207 sample entries and 186 unique runtime declaration names, all original spellings and sample relationships were preserved; all 68 feature IDs referenced by the configuration inventory exist. `git diff --check` passed. External research remains dated; upstream releases, driver support and runtime behavior were not reverified in this review.

## Cross check before development

**Reviewed:** 2026-11-14, starting at repository revision `1246c0d`. Every document under `docs/` and the root guides were read against each other and against the workflow steps the scope files name. This pass changes documents only. It approves no feature and records no build.

| When it matters | Finding and disposition | Owner in the plan |
|---|---|---|
| Before the first build | The handoff listed values to settle before editing, but no step recorded them and spec 0001 holds no build notes. Scope 001 now has that step ahead of scaffolding. The handoff also names the Linux build source for a workstation on another platform and the branch path used before CI exists. | [Scope 001](../scope/001-stack-and-workspace.md), [first handoff](../implementation-order.md#first-handoff-the-workspace) |
| Before the first build | Configuration keys, exit status, generated errors, metric names and API errors had no single owner, so each slice could choose again. One table now names the slice that fixes each rule. Every rule is still pending. | [Shared conventions](../engineering-standards.md#shared-conventions) |
| Before naming types | Several words described one thing, such as approved writer and authorized writer, and generation was used for six different counters. Canonical terms are recorded once. Existing prose was not rewritten. | [Glossary](../glossary.md) |
| Before tooling | No document selected the CI service, dependency and license checker, SBOM generator, fuzz harness or the PostgreSQL supply for tests. Twelve role names were assigned to a team of two with no rule for when a fresh model may review. Scope 002 now records both. | [Scope 002](../scope/002-standards-and-tooling.md) |
| Before reuse, replicas and administration | Three criteria needed a surface that a later slice delivers: generation retirement before the reload route, node status and drain evidence before the authenticated API, and basic diagnostics at the Core MVP gate. The API schema shared by the CLI and control handlers had no home. Each is assigned to the spec that must decide it. | [Dependencies](dependencies.md) |
| Before a release | Packaging and the release pipeline had no owning feature, and the V1 gate could be read as version 1.0.0 while releases stay before 1.0. | [Dependencies](dependencies.md), [scope 011](../scope/011-release-qualification-harness.md), [milestones](milestones.md#release-practices) |
| At every change | Statuses were defined in four places with different words. Risks and threats had no IDs, security requirements had no first slice, and evidence files had no stated path. | [Status lifecycle](../scope/index.md#status-lifecycle), [risk register](risk-register.md), [threat model](../security/threat-model.md), [security requirements](../security/security-requirements.md), [acceptance evidence](../testing/strategy.md#acceptance-evidence) |
| At every change | The feature inventory header had twelve columns and its delimiter row eleven, so the table did not render. The proposed layout showed test and benchmark folders that Cargo cannot build without an owning package. | [Feature inventory](../feature-matrix.md), [repository proposal](../repository-structure.md) |

**Checks run on the revised package:** All 69 Markdown documents (64 under `docs/`, including the new glossary) and the license are accounted for by the index. All 429 local file and heading links resolve. Each of the 60 tables has the same column count in every row. All 17 scope numbers and statuses agree with their index. The ten PRDs keep the same 41 AC identifiers and the 165 feature IDs stay unique. 25 risks, 14 threats and 14 security requirements carry IDs. `git diff --check` passed. External sources were not rechecked, and nothing was built or run.

**Still open for you:** the build note values for 001, the reviewer roster and tooling choices in 002, whether agents branch and commit for you during a build, and the version scheme. The proposed PRD state `Qualified at <milestone>` in the status lifecycle is new wording that you can change.

## Limits

No executable proxy exists. No Cargo build, runtime test, benchmark, protocol conformance run, penetration test or failover drill has run. Counts and link checks do not establish software correctness. The parameter census accounts for its stated source corpus, not all configuration behaviors or full Pgpool parity. Release archive/tag commit hashes, detailed 4.7.2 release notes and actual driver/dependency compatibility still need their recorded qualification gates.

The root scope tracks implementation as planned until authorized work and evidence advance it. A confirmed architecture can be in progress as a scope feature while its unbuilt governing spec remains Proposed.
