# 5. Clean session reuse · planned · needs a decision

[Scope index](index.md)

Thicken the primary path with identity partitioning, cleanup quarantine and bounded pool admission.

**Done when:** A backend can be safely reused after disconnect and state cannot leak between users or generations.

Use [PRD-002 AC-1 to AC-4](../prd/002-connection-pooling.md#acceptance-criteria). The session owner defines the reusable profile, key generations and reset/close boundary. Prove actual socket reuse for a permitted identity and separation for different identities. Requalify new protocol states when 006 introduces them.

- [ ] Design it (spec): `/architect clean session reuse`
