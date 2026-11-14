# High availability PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.5, deferred. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

One proxy VM is an endpoint failure domain, while independent controllers can create conflicting database operations.

## Goals

Support multiple proxies with one tested operation authority and safe endpoint failover.

## Non goals

No homegrown consensus, no transparent migration of active transactions, no automatic failover enabled by default.

## Functional requirements

1. Use established consensus for ordered configuration, membership and authority epochs.
2. Keep query routing on local immutable snapshots with explicit expiry policy.
3. Design cancellation dispatch to the session-owning proxy.
4. Integrate external load balancing or a controlled VIP without pretending sockets migrate.
5. Require fencing in addition to consensus for any future promotion authority.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Proven V1, separate HA spec, independent hosts, operated consensus, external endpoint mechanism and fencing.

## Data structures

ProxyMember: stable ID, advertised endpoints, health lease and applied generation. ControllerEpoch: monotonic fencing/authority generation. TopologyRecord: cluster ID, version, content hash and source authority. Local live sessions are never replicated as transferable transactions.

## Interfaces

External coordination adapter offers compare-and-swap topology, watches and leases. Membership/admin status reports leader, quorum, applied generation and stale state. Remote operations use mTLS and explicit roles.

## Failure scenarios

Minority partition, paused former leader, quorum loss, watch gaps, stale snapshot, concurrent VIP ownership and cancellation arriving at another proxy.

## Security requirements

Consensus credentials scoped by cluster. A controller lease is not a PostgreSQL fencing mechanism. Explicitly test authority revalidation at every irreversible operation.

## Observability

Member/quorum state, applied generation lag, stale admissions, ownership changes and cancellation dispatch failures.

## Tests

Partition/stale leader histories, consensus outage, restarted nodes, VIP duplicate ownership and cross proxy cancellation races.

## Acceptance criteria

1. **AC-1:** Minority or stale authority cannot authorize a new topology mutation or promotion.
2. **AC-2:** Queries do not perform a consensus round trip per request.
3. **AC-3:** Cancellation reaches only its live owning lease across supported endpoint topology.
4. **AC-4:** Endpoint failover documents client reconnect behavior and never claims transaction migration.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
