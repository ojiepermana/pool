# 1. Stack and workspace · done

[Scope index](index.md)

Ratify the foundation and create only the minimal workspace needed by the first real client path.

**Done when:** The architecture is recorded, the minimal daemon builds on Linux and configuration validation runs.

Use the [workspace handoff](../implementation-order.md#first-handoff-the-workspace) for the Linux build, offline validation fixtures and explicit exclusions. The feature spec holds the build notes and verification record.

- [x] Decide the stack (spec): `/architect stack and architecture`
- [x] Record the build notes the handoff requires in spec 0001: `/architect stack and workspace build notes`
- [x] Scaffold from the decision: `/develop stack and workspace`
- [x] Verify it: `/check verify stack and workspace`
- [x] Test it: `/test stack and workspace`
- [x] Review it (fresh model): `/check review stack and workspace`
- [x] Document it: `/document stack and workspace`

Spec [0001](../specs/0001-initial-architecture/index.md) · code in `apps/pool/`, `crates/pool-config/`.

Evidence: [verification record](../specs/0001-initial-architecture/verify.md), [independent review](../reviews/2026-11-14-scope-001-stack-and-workspace.md) and [changelog](../../CHANGELOG.md). The Linux container build (`0001 AC-2`) and the two Linux only tests passed for the declared target, `aarch64-unknown-linux-gnu`, at `41e8eda`; the record is under "Linux evidence: container build for aarch64" in the verification record. An earlier run for `x86_64` at `18e9228` stays there as history.
