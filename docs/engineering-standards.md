# Engineering standards

These requirements govern future implementation. This phase has not scaffolded a Cargo workspace.

## Rust and runtime

Use stable Rust, safe first party code, explicit ownership and typed errors. Keep hot paths free of unnecessary allocations and blocking work. Never hold a general lock across await without a documented necessity and a contention/cancellation test. Use bounded channels, buffers, task counts and queues. Handle OS errors without uncontrolled panics. `unwrap()` and `expect()` in production hot paths require a documented invariant and review. Public APIs and configuration need documentation.

Use rustfmt and Clippy. First party crates should forbid unsafe code. A technical exception needs a narrow interface, documented invariants, alternatives considered, review and targeted tests. Native parser, TLS provider and SQLite dependencies need their own boundary review; a safe wrapper is not proof that its internals are safe.

## Data and protocol safety

Preserve transaction boundaries and backend session identity. Keep writes on the eligible primary. Fail new admission closed when role or authority is uncertain. Never silently retry non idempotent operations or automatically replay a transaction with uncertain commit outcome. A replica observation is not proof of full catch up. Separate health evidence from promotion authority; require verified fencing before promotion.

Validate message lengths and arithmetic before allocation, support partial reads/writes, preserve asynchronous messages and SQLSTATE, and model every state transition. Cancellation belongs to the live lease and must be safe across completion and reuse. Unknown session cleanup closes the backend. Bound large payloads and auth/parser work explicitly.

## Dependency and configuration changes

Each dependency addition records purpose, current version, maintenance signal, selected features, license/native code, alternatives and runtime cost. Commit Cargo.lock for binaries and review transitive changes. Pin a stable toolchain and container digests, capture build metadata and use reproducible release steps. Document every public key's units, default, validation, secrecy, reload/restart behavior and migration rule. Invalid reload retains the previous safe generation.

## Security and reviews

Follow [security requirements](security/security-requirements.md). Keep secrets out of code, command arguments, Debug output, logs, traces and metrics. Require administrative authentication and action authorization. Protocol/parser/auth changes receive security review; pooling/cancel changes receive protocol review; failover and HA changes receive database/infrastructure review. No broad host administrator credential is justified by a routine monitor task.

## Tests and evidence

Critical logic requires unit and integration coverage plus appropriate state, property, fuzz, driver or chaos tests. A mock response cannot establish real protocol or failover behavior. Benchmark claims include environment, equivalent mode/config, raw data and distributions. Feature status advances only with linked evidence. Test code should check externally meaningful behavior and invariants rather than mirror private implementation details.

## Git and releases

Use protected main, pull requests, required CI and review for safety changes. Keep changes small and use conventional commits. Never commit secrets. Keep a changelog, semantic versioning policy, release tags, checksums, SBOM, notices, functioning examples and an upgrade/rollback guide. Branch protection cannot be configured here because no Git repository or remote exists yet.

## Workflow

Before code, read the relevant spec/architecture and scope, inspect actual existing functionality, identify dependencies, define an implementation plan and acceptance evidence, then implement only the authorized slice. Record load bearing changes in a spec/ADR rather than silently changing contracts. Run the required checks, document results and update feature status honestly. Architecture proposals do not authorize deployment or implementation beyond the user's current request.
