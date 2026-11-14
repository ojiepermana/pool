# 6. Protocol breadth and cancellation · planned · needs a decision

[Scope index](index.md)

Thicken the same path with extended operations, COPY, asynchronous traffic and safe cancellation.

**Done when:** The declared 3.0/3.2 driver corpus passes and cancellation cannot affect a later lease.

Complete the remaining [PRD-001 cases](../prd/001-core-proxy.md#acceptance-criteria) and repeat [PRD-002 isolation](../prd/002-connection-pooling.md#acceptance-criteria) for newly supported traffic. The protocol owner pins the supported version/driver combinations and cancel/retirement race fixture. The [Core MVP gate](../plan/milestones.md) applies here; 011 is the later V1 gate.

- [ ] Design it (spec): `/architect protocol breadth and cancellation`
