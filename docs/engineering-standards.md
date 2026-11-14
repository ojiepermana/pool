# Engineering standards

These requirements govern future implementation. This phase has not scaffolded a Cargo workspace.

## Rust and runtime

Use stable Rust, safe first party code, explicit ownership and typed errors. Keep hot paths free of unnecessary allocations and blocking work. Never hold a general lock across await without a documented necessity and a contention/cancellation test. Use bounded channels, buffers, task counts and queues. Handle OS errors without uncontrolled panics. `unwrap()` and `expect()` in production hot paths require a documented invariant and review. Public APIs and configuration need documentation.

Use rustfmt and Clippy. First party crates should forbid unsafe code. A technical exception needs a narrow interface, documented invariants, alternatives considered, review and targeted tests. Native parser, TLS provider and SQLite dependencies need their own boundary review; a safe wrapper is not proof that its internals are safe.

## Data and protocol safety

Preserve transaction boundaries and backend session identity. Keep writes on the eligible primary. Fail new admission closed when role or authority is uncertain. Once bytes may have reached a backend, never replay the statement or transaction, even if it appears idempotent. A lost COMMIT response leaves the outcome for the application to reconcile. Follow the [initial architecture safety contracts](specs/0001-initial-architecture/index.md#safety-contracts) for replica admission, cancellation and promotion authority.

Validate message lengths and arithmetic before allocation, support partial reads/writes, preserve asynchronous messages and SQLSTATE, and model every state transition. Cancellation belongs to the live lease and must be safe across completion and reuse. Unknown session cleanup closes the backend. Bound large payloads and auth/parser work explicitly.

## Dependency and configuration changes

Each dependency addition records purpose, current version, maintenance signal, selected features, license/native code, alternatives and runtime cost. Commit Cargo.lock for binaries and review transitive changes. Pin a stable toolchain and container digests, capture build metadata and use reproducible release steps. Document every public key's units, default, validation, secrecy, reload/restart behavior and migration rule. Invalid reload retains the previous safe generation.

## Security and reviews

Follow [security requirements](security/security-requirements.md). Keep secrets out of code, command arguments, Debug output, logs, traces and metrics. Require administrative authentication and action authorization. Protocol/parser/auth changes receive security review; pooling/cancel changes receive protocol review; failover and HA changes receive database/infrastructure review. No broad host administrator credential is justified by a routine monitor task.

## Shared conventions

Some surfaces are extended by every later slice. The slice that first exposes one records its rule in this table once. Later specs cite the rule instead of choosing again. An empty rule is an open decision for the named slice, not permission to improvise.

| Surface | First fixed in | Rule |
|---|---|---|
| Configuration keys: naming, units, unknown keys, secret references and reserved names | [001](scope/001-stack-and-workspace.md) | One TOML file with a required `config_version`. Lowercase `snake_case` names from the glossary. Integer quantities with a unit suffix. Unknown keys are rejected. Secrets only by a `_file` reference. A reserved capability is a table with `enabled`, default `false`, and `true` is rejected. Full rule: [spec 0001, configuration rules](specs/0001-initial-architecture/index.md#configuration-rules) |
| Process exit status, and command output for people and as JSON | [001](scope/001-stack-and-workspace.md) | Exit 0 success, 1 input rejected, 2 usage error, 3 could not complete. The result goes to stdout, usage errors to stderr. JSON is one object on one line that starts with `output_version`, `command` and `result` and ends with `findings`. Output is constant text and never repeats input. Full rule: [spec 0001, exit status and output](specs/0001-initial-architecture/index.md#exit-status-and-output) |
| Log event fields and redaction | [001](scope/001-stack-and-workspace.md) | One JSON object per line on stderr with `ts`, `level` and `event`, and named fields for anything that varies. Secrets are redacted by type, not by filter. Credentials, cancellation keys, SQL, database error text and configuration content are never logged by default. Fixed ahead of the first log event. Full rule: [spec 0001, log events and redaction](specs/0001-initial-architecture/index.md#log-events-and-redaction) |
| Identifier and generation types | [003](scope/003-control-and-session-data-model.md) | Pending |
| SQLSTATEs and messages that POOL itself generates | [004](scope/004-one-real-primary-path.md) | Pending |
| Metric names, units and bounded labels | The first slice that exports a metric, no later than [006](scope/006-protocol-breadth-and-cancellation.md) | Pending |
| Administration API versioning, error body and request ID | [009](scope/009-safe-topology-administration.md) | Pending |

Take every public name from the [glossary](glossary.md). A new term enters the glossary before it enters code.

## Tests and evidence

Test observable behavior and safety invariants. Choose the test layer and record the result using the [testing strategy](testing/strategy.md#acceptance-evidence). Follow the [benchmark method](testing/benchmark-plan.md) for performance claims.

## Git and releases

Use protected main, pull requests, required CI and review for safety changes. Keep changes small and use conventional commits. Never commit secrets. Keep a changelog, semantic versioning policy, release tags, checksums, SBOM, notices, functioning examples and an upgrade/rollback guide. Verify branch protection and required checks on the remote when setting up [standards and tooling](scope/002-standards-and-tooling.md).

## Workflow

Before code, read the relevant spec/architecture and scope, inspect actual existing functionality, identify dependencies, define an implementation plan and acceptance evidence, then implement only the authorized slice. Record load bearing changes in a spec/ADR rather than silently changing contracts. Run the required checks, document results and update feature status honestly. Architecture proposals do not authorize deployment or implementation beyond the user's current request.
