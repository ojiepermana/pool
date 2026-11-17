# 5. Clean session reuse · done

[Scope index](index.md)

Thicken the primary path with identity partitioning, cleanup quarantine and bounded pool admission.

**Done when:** A backend can be safely reused after disconnect and state cannot leak between users or generations.

Use [PRD-002 AC-1 to AC-4](../prd/002-connection-pooling.md#acceptance-criteria). The session owner defines the reusable profile, key generations and reset/close boundary. Prove actual socket reuse for a permitted identity and separation for different identities. Requalify new protocol states when 006 introduces them.

Spec: [0005](../specs/0005-clean-session-reuse/index.md).

- [x] Design it (spec): `/architect clean session reuse`, ratified through delegated auto decisions.
- [x] Build it: `/develop clean session reuse`
  - [x] Inherited tooling and lifecycle regressions, 0005 AC-9.
  - [x] Bounded physical ownership and clean reconnect, 0005 AC-1 through AC-5 and AC-7.
  - [x] Live generations and credential derivation ownership, 0005 AC-2, AC-6 and AC-8.
- [x] Verify it: `/check verify clean session reuse`
- [x] Test it: `/test clean session reuse`
- [x] Review it (fresh model): `/check review clean session reuse`
- [x] Document it: `/document changelog clean session reuse`

Code: [pool owner](../../crates/pool-core/src/pool.rs), [cleanup](../../crates/pool-core/src/reuse.rs), [session](../../crates/pool-core/src/session.rs) and [daemon](../../apps/pool/src/daemon.rs).

Evidence: [verification](../specs/0005-clean-session-reuse/verify.md), [review](../reviews/2026-11-16-scope005-clean-session-reuse.md), [tooling review](../reviews/2026-11-16-scope005-tooling.md). The run uses native task adaptations for source and regression writing; scope/spec decisions and integration remain with the Leader.

Runtime checks pass and independent review approves. On 2026-11-17 you confirmed Sol as the main chat author family, completing the contrast evidence for the fresh Terra reviews. Resumed local checks and all 36 real PostgreSQL cases pass; the artifact audit confirms that the later code changes were covered by the final review. No GitHub write is authorized by this record.
