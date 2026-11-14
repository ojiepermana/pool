# SQL routing PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Naive read detection can send side effects to a replica or break session and read after write semantics.

## Goals

Provide automatic replica selection inside explicit read only profiles, with primary routing for ordinary mixed sessions.

## Non goals

No arbitrary session virtualization, semantic proof from an AST, transparent retry, or causal replica read token in V1.

## Functional requirements

1. Select the initial backend from authenticated profile and fresh topology at login; parse later SQL with the PostgreSQL 18 grammar adapter for admission on that pinned backend.
2. Keep primary profile sessions pinned, including after writes.
3. Restrict replica profiles to approved query/object and trusted schema contracts.
4. Reject unsafe or uncertain operations in restricted replica profiles; do not migrate the session.
5. Handle multi statements, CTEs, locks, functions, sequences, prepared statements, portals and session changes conservatively.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Core session contract, pg_query adapter, validated policies and topology/lag observations.

## Data structures

RouteProfile: ID required, operator principal/database mapping required, eventual-read consistency enum required, allowed query surface version required, fallback disabled in V1. Classification: effects set, statement count, uncertainty reason and parser version required. InitialSelection uses profile plus snapshot; OperationAdmission uses later classification plus pinned session state. Both record reason, snapshot generation and profile version.

## Interfaces

`select_backend(profile, fresh_snapshot)` chooses the endpoint at login. `classify(sql, parser_version)` produces effects and uncertainty after SQL arrives. `admit_on_pinned_backend(frame, session_state, classification, policy)` accepts or rejects without choosing a different backend. Profile selection comes from the operator map, not client app_name alone.

## Failure scenarios

Parser timeout, new grammar, view/operator/cast indirection, schema change, stale topology, incompatible timeline, ambiguous query and backend error after dispatch.

## Security requirements

Application permissions and trusted database object ownership remain necessary. Read only does not prove arbitrary extensions lack external side effects. Hints cannot override safety policy.

## Observability

Primary/replica selection counts, profile, bounded reason codes, parser timeouts, unsupported classification and refusal counters.

## Tests

Differential parser corpus, adversarial comments/multi statements, prepared statement ownership, controlled schema changes, own-write reads and paused replica replay.

## Acceptance criteria

1. **AC-1:** Every write, unknown or stateful sample is primary routed or rejected by a replica profile.
2. **AC-2:** A configured read profile selects an eligible replica at login using an admission-time lag screen, remains eventual and pinned thereafter, and retires affected leases before controlled promotion. It has no primary fallback in V1.
3. **AC-3:** A primary session reads its committed write using the same backend and normal PostgreSQL semantics.
4. **AC-4:** SQL parser failure cannot silently authorize replica routing.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
