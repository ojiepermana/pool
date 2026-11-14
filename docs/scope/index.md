# Scope: POOL

You are building an open source PostgreSQL proxy, session pooler and cluster operations service. The first complete operating release covers your one proxy, one primary and two replica VMs. The core MVP is an earlier, deliberately smaller milestone.

**Build approach:** Tracer Bullet, prove one real client path, then thicken it through tested vertical slices.
**Workflow:** GA, behavior verification, tests, independent review and documentation.
**Confirmed:** Two experienced engineers, conservative lab validation, Apache 2.0 and one modular daemon plus CLI. Exact scope decisions remain proposals until their specs are ratified.

You ratified the initial architecture on 2026-11-14 after independent review. Feature 1 is in progress because its design is complete; all execution boxes remain queued. No implementation is complete.

## At a glance

| # | Feature | Phase | Status |
|---|---|---|---|
| 1 | [Stack and workspace](stack-and-workspace.md) | Foundation | in-progress |
| 2 | [Standards and tooling](standards-and-tooling.md) | Foundation | planned |
| 3 | [Control and session data model](control-and-session-data-model.md) | Foundation | planned |
| 4 | [One real primary path](one-real-primary-path.md) | Slice 1 | planned |
| 5 | [Clean session reuse](clean-session-reuse.md) | Slice 2 | planned |
| 6 | [Protocol breadth and cancellation](protocol-breadth-and-cancellation.md) | Slice 3 | planned |
| 7 | [Restricted replica sessions](restricted-replica-sessions.md) | Slice 4 | planned |
| 8 | [Health diagnostics and drain](health-diagnostics-and-drain.md) | Slice 5 | planned |
| 9 | [Safe topology administration](safe-topology-administration.md) | Slice 6 | planned |
| 10 | [Fenced manual promotion](fenced-manual-promotion.md) | Slice 7 | planned |
| 11 | [Release qualification harness](release-qualification-harness.md) | V1 gate | planned |
| 12 | [Multiple proxy authority](multiple-proxy-authority.md) | Deferred | planned |
| 13 | [Checked online recovery](checked-online-recovery.md) | Deferred | planned |
| 14 | [Advanced pooling and routing](advanced-pooling-and-routing.md) | Deferred | planned |
| 15 | [Optional result cache](optional-result-cache.md) | Deferred | planned |
| 16 | [Additional replication modes](additional-replication-modes.md) | Deferred | planned |
| 17 | [Administrative dashboard](administrative-dashboard.md) | Deferred | planned |

## File ownership

This index manages build order, phase, project defaults, and the status overview. Each linked feature file owns its intent, Done when criteria, checklist, and spec/evidence pointers. When a feature changes status, update both its index row and its heading. Keep its existing number when renaming or moving the file, and update the link here.

## Legend

Only the foundation is in progress, reflecting your accepted design. Other features remain planned, including deferred features. A decision box is checked only after you accept its spec. Later feature specs add a coarse build milestone rollup and GA verification boxes; atomic tasks stay in the spec. Build and test evidence are required before completed implementation is claimed.

Needs a decision means behavior or safety is not yet fully settled. Deferred means outside the current release queue, not silently removed from the parity inventory. You can revisit scope after each shipped slice.

## References

Your brief supplies the operating scenario and initial exclusions. [Architecture](../specs/0001-initial-architecture/index.md), [roadmap](../plan/roadmap.md), [testing](../testing/strategy.md), and [feature inventory](../feature-matrix.md) provide the reasoning and verified source trail. The order follows foundations before dependent behavior and the confirmed vertical delivery approach.
