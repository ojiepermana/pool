# 3. Control and session data model · done

[Scope index](index.md)

Set identity, lease, configuration, observation and durable operation records before state is shared.

**Done when:** Each record has an owner, lifecycle, source and restart rule, with durable authority distinct from observed database state.

Before 004, the control and protocol owners agree how to distinguish first bootstrap from lost or restored authority, which generations survive restart, and how the verified principal maps to the actual backend login. The spec must walk through clean startup, missing journal and conflicting role evidence. Use the [control records](../architecture/control-plane.md) and [initial architecture](../specs/0001-initial-architecture/index.md); the full operation API belongs to 009.

- [x] Design it (spec): `/architect control and session data model`
- [x] Build it: `/develop control and session data model`
  - [x] Workspace member and guards: Build plan step 1 (`AC-13`)
  - [x] Identifiers, configuration keys and the finding line: steps 2 to 4 (`AC-1` to `AC-6`), can run beside the milestone above
  - [x] Authority model and the four walks: step 5 (`AC-7` to `AC-12`, code half of `AC-14`), needs the two milestones above
  - [x] Document corrections and evidence: steps 6 and 7 (`AC-14`, evidence for `AC-1` to `AC-14`), after the three milestones above
  - [x] Validation time: step 8 (`AC-15`), after the milestone "Identifiers, configuration keys and the finding line"
  - [x] Witness in writer admission: step 9 (`AC-16`), after the milestone "Authority model and the four walks"
- [x] Verify it: `/check verify control and session data model`
- [x] Test it: `/test control and session data model`
- [x] Review it (fresh model): `/check review control and session data model`
- [x] Document it: `/document control and session data model`

Spec [0003](../specs/0003-control-and-session-data-model/index.md) · code in `crates/pool-config/`, `crates/pool-control/`, `apps/pool/src/report.rs`, `tools/repo-check/`.

Evidence: [verification record](../specs/0003-control-and-session-data-model/verify.md), [independent review](../reviews/2026-11-15-scope-003-control-and-session-data-model.md) and [changelog](../../CHANGELOG.md). Every box is ticked on workstation evidence, with a Linux container build for `aarch64-unknown-linux-gnu`. The seven checks passed on [pull request 4](https://github.com/ojiepermana/pool/pull/4), which is merged; the record is under "Remote evidence: pull request 4" in the verification record. Still open, and written down as follow ups in spec 0003: the duties of scope 004 before anything calls `writer_admission`, and two decisions that are yours (a limit on the number of findings, and a note on `PRD-006 AC-2`).
