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

## Limits

No executable proxy exists. No Cargo build, runtime test, benchmark, protocol conformance run, penetration test or failover drill has run. Counts and link checks do not establish software correctness. The parameter census accounts for its stated source corpus, not all configuration behaviors or full Pgpool parity. Release archive/tag commit hashes, detailed 4.7.2 release notes and actual driver/dependency compatibility still need their recorded qualification gates.

The root scope tracks implementation as planned until authorized work and evidence advance it. A confirmed architecture can be in progress as a scope feature while its unbuilt governing spec remains Proposed.
