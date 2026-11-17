# Review, scope005 clean session reuse, 2026-11-16

**Reviewed by**: gpt-6.1-terra/high (core/tooling author configured as gpt-6.1-sol/high; Leader author family unconfirmed, so overall family independence is unverified)
**Scope**: 41 files, uncommitted against 2dbc037, with incremental fixes reviewed
**Verdict**: Approve

## Summary
The patch adds bounded backend ownership, conservative reset and attestation, credential retirement, and repository checks. Both prior major findings are resolved: deterministic tests now exercise the derivation ownership and admission failure paths and the post-handoff, pre-greeting retirement window. The final test-only capture fix installs the existing subscriber for both state-only node-loss cases, and the supplied Linux repetitions pass. This review does not establish overall GA closure or model-family independence.

## Strengths
- The single pool mutex reserves nested physical budgets together; permit ownership travels with the physical connection and ticket drop wakes the queue.
- Cleanup uses bounded cumulative reads and an immutable startup baseline; notification handling and post-reset validation have adversarial scripted cases.
- Derivation tests use the same owner protocol and production budget factory with an isolated semaphore, controlled completion, caller cancellation and a real blocking-worker panic. The post-handoff test observes the actual reused handoff and holds the greeting on a bounded transport.

## Test coverage
The review independently ran `cargo test -p pool-core --locked derivation005` (three passed) and `cargo test -p pool-core --locked reuse005_rotation_after_actual_handoff_before_greeting_preserves_only_the_pinned_owner` (one passed). The earlier filtered real PostgreSQL 18 PID-reuse test passed (one passed, 35 filtered). The supplied negative-control logs show the saturation and handoff tests fail when their respective guarantees are deliberately broken; the supplied positive core test and Clippy logs pass. Existing tests cover reset, partial messages, notifications, zero waiters, FIFO, cancellation, key isolation, retirement before handoff and slow readers. The separate tooling review covers its stated scope. The supplied final Linux log records five passing 32-test parallel repetitions, a passing workspace test run, and a release build. The supplied integration log records 36 passing real PostgreSQL tests. These are inspected logs, not commands independently run during this final re-review.
