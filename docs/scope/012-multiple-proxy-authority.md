# 12. Multiple proxy authority · planned · needs a decision

[Scope index](index.md)

Introduce endpoint HA and one distributed operation authority without custom consensus.

**Done when:** Minority and stale leaders cannot mutate topology; cancellation ownership and client reconnect behavior are proven.

Reopen after V1 only with [PRD-008](../prd/008-high-availability.md#acceptance-criteria) and a proposal for independent failure domains and a single operation authority. Automatic failover needs its own decision.

- [ ] Design it (spec): `/architect multiple proxy authority`
