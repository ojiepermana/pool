# 3. Control and session data model · in-progress

[Scope index](index.md)

Set identity, lease, configuration, observation and durable operation records before state is shared.

**Done when:** Each record has an owner, lifecycle, source and restart rule, with durable authority distinct from observed database state.

Before 004, the control and protocol owners agree how to distinguish first bootstrap from lost or restored authority, which generations survive restart, and how the verified principal maps to the actual backend login. The spec must walk through clean startup, missing journal and conflicting role evidence. Use the [control records](../architecture/control-plane.md) and [initial architecture](../specs/0001-initial-architecture/index.md); the full operation API belongs to 009.

- [x] Design it (spec): `/architect control and session data model`
- [ ] Build it: `/develop control and session data model`
  - [ ] Workspace member and guards: Build plan step 1 (`AC-13`)
  - [ ] Identifiers, configuration keys and the finding line: steps 2 to 4 (`AC-1` to `AC-6`), can run beside the milestone above
  - [ ] Authority model and the four walks: step 5 (`AC-7` to `AC-12`, code half of `AC-14`), needs the two milestones above
  - [ ] Document corrections and evidence: steps 6 and 7 (`AC-14`, evidence for `AC-1` to `AC-14`), after the three milestones above
- [ ] Verify it: `/check verify control and session data model`
- [ ] Test it: `/test control and session data model`
- [ ] Review it (fresh model): `/check review control and session data model`
- [ ] Document it: `/document control and session data model`

Spec [0003](../specs/0003-control-and-session-data-model/index.md).
