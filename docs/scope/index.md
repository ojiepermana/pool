# Scope: POOL

You are building an open source PostgreSQL proxy, session pooler and cluster operations service. The first complete operating release covers your one proxy, one primary and two replica VMs. The core MVP is an earlier, deliberately smaller milestone.

**Build approach:** Tracer Bullet, prove one real client path, then thicken it through tested vertical slices.
**Workflow:** GA, behavior verification, tests, independent review and documentation.
**Confirmed:** Two experienced engineers, conservative lab validation, Apache 2.0 and one modular daemon plus CLI. Exact scope decisions remain proposals until their specs are ratified.

You ratified the initial architecture on 2026-11-14 after independent review. Feature 1 is done: every box is ticked, and its workstation evidence and its Linux build evidence are recorded. Feature 2 is done: the required checks run in CI on every pull request, and `main` takes a change only through a pull request whose checks passed. Feature 3 is done: [spec 0003](../specs/0003-control-and-session-data-model/index.md) fixes the control and session records, and its configuration keys and authority rules are built, verified, reviewed and merged through a pull request whose checks passed. Its design decisions were taken under the delegated decision mode of the coordination run and are listed in the [coordination log](coordination/003-control-and-session-data-model.log.md). Feature 4 is done: [spec 0004](../specs/0004-one-real-primary-path/index.md) builds the first real path, one authenticated TLS client through POOL to the verified primary, and it is built, verified, tested, reviewed and documented with workstation evidence and a Linux container build. Its design decisions were taken under the same delegated decision mode ([coordination log](coordination/004-one-real-primary-path.log.md)). Three manual steps of that feature are not run yet and are listed in its [manual file](coordination/004-one-real-primary-path.manual.md): the checks on its pull request, the CI jobs `integration` and `fuzz`, and with them the first run of the integration check on a Linux host. POOL proxies that one primary path and nothing more: there is no backend reuse, no extended protocol, no replica session and no administration interface.

## At a glance

| # | Feature | Phase | Status |
|---|---|---|---|
| 1 | [Stack and workspace](001-stack-and-workspace.md) | Foundation | done |
| 2 | [Standards and tooling](002-standards-and-tooling.md) | Foundation | done |
| 3 | [Control and session data model](003-control-and-session-data-model.md) | Foundation | done |
| 4 | [One real primary path](004-one-real-primary-path.md) | Slice 1 | done |
| 5 | [Clean session reuse](005-clean-session-reuse.md) | Slice 2 | planned |
| 6 | [Protocol breadth and cancellation](006-protocol-breadth-and-cancellation.md) | Slice 3 | planned |
| 7 | [Restricted replica sessions](007-restricted-replica-sessions.md) | Slice 4 | planned |
| 8 | [Health diagnostics and drain](008-health-diagnostics-and-drain.md) | Slice 5 | planned |
| 9 | [Safe topology administration](009-safe-topology-administration.md) | Slice 6 | planned |
| 10 | [Fenced manual promotion](010-fenced-manual-promotion.md) | Slice 7 | planned |
| 11 | [Release qualification harness](011-release-qualification-harness.md) | V1 gate | planned |
| 12 | [Multiple proxy authority](012-multiple-proxy-authority.md) | Deferred | planned |
| 13 | [Checked online recovery](013-checked-online-recovery.md) | Deferred | planned |
| 14 | [Advanced pooling and routing](014-advanced-pooling-and-routing.md) | Deferred | planned |
| 15 | [Optional result cache](015-optional-result-cache.md) | Deferred | planned |
| 16 | [Additional replication modes](016-additional-replication-modes.md) | Deferred | planned |
| 17 | [Administrative dashboard](017-administrative-dashboard.md) | Deferred | planned |

Use the [first development handoff](../implementation-order.md) to start, [dependencies](../plan/dependencies.md) for prerequisites and [milestones](../plan/milestones.md) for release gates. Feature numbers identify work; PRD numbers identify contracts. For example, scope 004 delivers part of PRD-001, not PRD-004. Each feature links to its contract and names what will prove its part.

## File ownership

This index manages build order, phase, project defaults, and the status overview. Feature filenames use a three digit prefix matching their feature number, such as `001-stack-and-workspace.md` through `017-administrative-dashboard.md`. The central index remains `index.md`. Each linked feature file owns its intent, Done when criteria, checklist, and spec/evidence pointers. When a feature changes status, update both its index row and its heading. Keep its existing number when renaming or moving the file, and update the link here.

## Legend

Features 1, 2, 3 and 4 are done, reflecting your accepted designs and their recorded evidence. Other features remain planned, including deferred features. A decision box is checked only after you accept its spec. Later feature specs add a coarse build milestone rollup and GA verification boxes; atomic tasks stay in the spec. A design or feasibility decision can unblock its dependent slice; it does not mark a capability implemented. Build and test evidence are required before completed implementation is claimed.

Needs a decision means behavior or safety is not yet fully settled. Deferred means outside the current release queue, not silently removed from the parity inventory. You can revisit scope after each shipped slice.

## Status lifecycle

Each record has its own states. A state changes only when the evidence in the last column exists.

| Record | States | What advances it |
|---|---|---|
| Scope feature, in this index and its file | `planned`, `in-progress`, `done`; `dropped` if removed from scope | `in-progress` when you accept its design or building starts. `done` only when the GA steps link their evidence: verification, tests, independent review and documentation. |
| Feature spec | `Proposed`, `In Progress`, `Accepted`; `Superseded by NNNN` | It mirrors its scope feature: `In Progress` when building starts, `Accepted` when the feature is `done`. Your ratification settles its content, not this status. |
| PRD | `Proposed`, then `Qualified at <milestone>` | A feature spec adopts criteria by ID for its slice. The state line changes only when the [milestone gate](../plan/milestones.md) that assesses the PRD passes. |
| [Feature inventory](../feature-matrix.md) row | `Research`, `Planned`, `Implemented`, `Tested` | `Implemented` needs build evidence. `Tested` needs a passing verification record for the row's acceptance. |

A deferred feature stays `planned`. Needs a decision and Deferred are tags, not statuses.

## References

Your brief supplies the operating scenario and initial exclusions. [Architecture](../specs/0001-initial-architecture/index.md), [roadmap](../plan/roadmap.md), [testing](../testing/strategy.md), and [feature inventory](../feature-matrix.md) provide the reasoning and verified source trail. The order follows foundations before dependent behavior and the confirmed vertical delivery approach.
