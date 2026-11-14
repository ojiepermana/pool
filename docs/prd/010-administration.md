# Administration PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Operators need reliable status and safe actions without exposing database or host control to application users.

## Goals

Provide CLI and versioned REST equivalents for the required PCP operational capabilities.

## Non goals

No PCP wire protocol, pgpool_adm extension, management dashboard or arbitrary shell endpoint in V1.

## Functional requirements

1. Expose configuration validation, health, pool, node, operation and version diagnostics.
2. Authenticate all management requests through protected local transport or configured mTLS.
3. Authorize viewer, operator, promoter and configuration actions explicitly.
4. Require request IDs, expected generations and durable operation IDs for mutations.
5. Validate whole config generations before reload and redact effective configuration.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Axum/CLI contract, authentication, control model, journal, metrics and per-operation safety specs.

## Data structures

AdminPrincipal: transport identity and role set required. APIError: stable code, request ID and redacted message required. Operation summary derives from journal; node status derives from fresh observations; config generation derives from validated loaded config.

## Interfaces

GET `/v1/status`, `/v1/cluster`, `/v1/nodes`, `/v1/pools`, `/v1/operations/{id}` for viewers. POST `/v1/config/validate` and `/reload` for configuration role. Node and failover routes use their PRD contracts. 400 invalid, 401 unauthenticated, 403 denied, 409 generation conflict, 412 unsafe, 429 limited and 503 unavailable. CLI offers JSON and human output.

## Failure scenarios

Unknown API version, invalid config, stale mutation, missing local credentials, bad mTLS identity, large request body, disk full and management overload.

## Security requirements

No unauthenticated loopback assumption: local port access is not identity. Use protected Unix transport or actual credentials; remote binding requires mTLS and explicit configuration. Metrics have a separately restricted exposure policy.

## Observability

Request counts/latency, role denials, generation conflicts, operation outcomes and audit/journal health, without high cardinality user input labels.

## Tests

Role/action denial matrix, redaction snapshots, idempotent request replay, schema limits, TLS failures and management access during data-plane overload.

## Acceptance criteria

1. **AC-1:** Every supported mutation has denial, idempotency and audit evidence.
2. **AC-2:** Invalid reload retains the prior safe generation and returns a clear error.
3. **AC-3:** Diagnostics omit credentials, SQL parameters, cancellation secrets and raw SQL by default.
4. **AC-4:** All 15 PCP operations have a documented equivalent or explicit deferred/excluded disposition.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
