# Administration PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

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

Bound management requests independently of client traffic. Operators must still be able to inspect a saturated daemon without exposing SQL, credentials or cancellation secrets.

## Dependencies

Axum/CLI contract, authentication, control model, journal, metrics and per-operation safety specs.

## Data structures

AdminPrincipal: transport identity and role set required. APIError: stable code, request ID and redacted message required. Operation summary derives from journal; node status derives from fresh observations; config generation derives from validated loaded config.

## Interfaces

GET `/v1/status`, `/v1/cluster`, `/v1/nodes`, `/v1/pools`, `/v1/operations/{id}` for viewers. POST `/v1/config/validate` and `/reload` for configuration role. Node and failover routes use their PRD contracts. 400 invalid, 401 unauthenticated, 403 denied, 409 generation conflict, 412 unsafe, 429 limited and 503 unavailable. CLI offers JSON and human output.

After authenticating and authorizing the caller, resolve a mutation retry by actor, action and request ID. Within the documented retry window, the same request returns the original operation ID and current outcome, including after a lost response or restart. Reuse with a different target, payload or precondition returns a conflict. The administration spec defines that window and how callers reconcile older or incomplete requests. Config validation reports errors without activating a generation.

## Failure scenarios

Unknown API version, invalid config, stale mutation, missing local credentials, bad mTLS identity, large request body, disk full and management overload.

## Security requirements

No unauthenticated loopback assumption: local port access is not identity. Use protected Unix transport or actual credentials; remote binding requires mTLS and explicit configuration. Metrics have a separately restricted exposure policy.

## Observability

Request counts/latency, role denials, generation conflicts, operation outcomes and audit/journal health, without high cardinality user input labels.

## Tests

Role/action denial matrix, redaction snapshots, idempotent request replay, schema limits, TLS failures and management access during data-plane overload.

## Acceptance criteria

1. **AC-1:** For each supported mutation, test denial, an identical retry after response loss, and the same key with a changed target or payload. The accepted request produces one logical operation and an attributable audit history.
2. **AC-2:** Invalid reload retains the prior safe generation and returns a clear error.
3. **AC-3:** Diagnostics omit credentials, SQL parameters, cancellation secrets and raw SQL by default.
4. **AC-4:** All 15 PCP operations have a documented equivalent or explicit deferred/excluded disposition.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Topology administration](../scope/009-safe-topology-administration.md) and [PCP dispositions](../feature-matrix.md).
