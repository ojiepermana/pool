# 3. Control and session data model · planned · needs a decision

[Scope index](index.md)

Set identity, lease, configuration, observation and durable operation records before state is shared.

**Done when:** Each record has an owner, lifecycle, source and restart rule, with durable authority distinct from observed database state.

Before 004, the control and protocol owners agree how to distinguish first bootstrap from lost or restored authority, which generations survive restart, and how the verified principal maps to the actual backend login. The spec must walk through clean startup, missing journal and conflicting role evidence. Use the [control records](../architecture/control-plane.md) and [initial architecture](../specs/0001-initial-architecture/index.md); the full operation API belongs to 009.

- [ ] Design it (spec): `/architect control and session data model`
