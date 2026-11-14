# SQL routing PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

## Problem statement

Naive read detection can send side effects to a replica or break session and read after write semantics.

## Goals

Provide automatic replica selection inside explicit read only profiles, with primary routing for ordinary mixed sessions.

## Non goals

No arbitrary session virtualization, semantic proof from an AST, transparent retry, or causal replica read token in V1.

## Functional requirements

1. Select the backend at login from the authenticated profile and fresh topology. Use PostgreSQL 18 grammar when checking subsequent SQL for admission on a restricted replica lease.
2. Keep primary profile sessions pinned, including after writes.
3. Restrict replica profiles to approved query/object and trusted schema contracts.
4. Reject unsafe or uncertain operations in restricted replica profiles; do not migrate the session.
5. Handle multi statements, CTEs, locks, functions, sequences, prepared statements, portals and session changes conservatively.

## Non functional requirements

Classification has a finite byte and time budget. Check the [pinned session policy](../architecture/sql-routing.md) even when classification fails.

## Dependencies

The [restricted replica slice](../scope/007-restricted-replica-sessions.md) needs the Core MVP session contract, a bounded parser adapter, an approved query surface and fresh role/lag observations. It can demonstrate retirement on request. The [manual promotion slice](../scope/010-fenced-manual-promotion.md) verifies that retirement completes before the promotion side effect.

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

1. **AC-1:** In a primary session each supported sample stays on its original backend. In a restricted replica session a disallowed write, stateful operation or ambiguous statement is refused before dispatch. Include a disallowed statement inside a multi-statement Query and a disallowed prepared operation in an extended pipeline. Check the replica trace and subsequent protocol/transaction state for each refusal.
2. **AC-2:** A configured read profile selects an eligible replica at login using an admission-time lag screen, remains eventual and pinned thereafter, and retires affected leases before controlled promotion. It has no primary fallback in V1.
3. **AC-3:** A primary session reads its committed write using the same backend and normal PostgreSQL semantics.
4. **AC-4:** A parser error, input over the classification limit or parser timeout on a replica session produces a refusal. The operation reaches neither the selected replica nor a substitute backend.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[SQL routing decision matrix](../architecture/sql-routing.md#decision-matrix).
