# Engineering standards

These requirements govern implementation. The workspace they apply to today is `apps/pool`, `crates/pool-config` and `crates/pool-control`. The checks that enforce them are fixed in [spec 0002](specs/0002-standards-and-tooling/index.md#required-checks).

## Rust and runtime

Use stable Rust, safe first party code, explicit ownership and typed errors. Keep hot paths free of unnecessary allocations and blocking work. Never hold a general lock across await without a documented necessity and a contention/cancellation test. Use bounded channels, buffers, task counts and queues. Handle OS errors without uncontrolled panics. `unwrap()` and `expect()` in production hot paths require a documented invariant and review. Public APIs and configuration need documentation.

Use rustfmt and Clippy with the defaults of the pinned toolchain; the project has no `rustfmt.toml` or `clippy.toml`, and lint levels live in the lint tables of the root `Cargo.toml`. First party crates forbid unsafe code: the workspace sets `unsafe_code = "forbid"`, and every workspace member inherits it (rule W4 of [spec 0002](specs/0002-standards-and-tooling/index.md#workspace-guards) guards that). The one place outside this rule is a fuzz package, which is not a workspace member and is never a dependency of anything: the `fuzz_target!` macro needs an unsafe attribute, so `crates/pool-config/fuzz/` has no lint table ([fuzz harness](specs/0002-standards-and-tooling/index.md#fuzz-harness)). A technical exception needs a narrow interface, documented invariants, alternatives considered, review and targeted tests. Native parser, TLS provider and SQLite dependencies need their own boundary review; a safe wrapper is not proof that its internals are safe.

## Data and protocol safety

Preserve transaction boundaries and backend session identity. Keep writes on the eligible primary. Fail new admission closed when role or authority is uncertain. Once bytes may have reached a backend, never replay the statement or transaction, even if it appears idempotent. A lost COMMIT response leaves the outcome for the application to reconcile. Follow the [initial architecture safety contracts](specs/0001-initial-architecture/index.md#safety-contracts) for replica admission, cancellation and promotion authority.

Validate message lengths and arithmetic before allocation, support partial reads/writes, preserve asynchronous messages and SQLSTATE, and model every state transition. Cancellation belongs to the live lease and must be safe across completion and reuse. Unknown session cleanup closes the backend. Bound large payloads and auth/parser work explicitly.

## Dependency and configuration changes

Each dependency addition records purpose, current version, maintenance signal, selected features, license/native code, alternatives and runtime cost. Commit Cargo.lock for binaries and review transitive changes. The [dependency policy](specs/0002-standards-and-tooling/index.md#dependency-policy) checks advisories, licenses and package sources on every pull request, and a change to the set of locked packages must change its baseline file in the same pull request. Pin a stable toolchain and container digests, capture build metadata and use reproducible release steps. Document every public key's units, default, validation, secrecy, reload/restart behavior and migration rule. Invalid reload retains the previous safe generation.

## Security and reviews

Follow [security requirements](security/security-requirements.md). Keep secrets out of code, command arguments, Debug output, logs, traces and metrics. Require administrative authentication and action authorization. Protocol/parser/auth changes receive security review; pooling/cancel changes receive protocol review; failover and HA changes receive database/infrastructure review. No broad host administrator credential is justified by a routine monitor task.

### Reviewer roster

This table names who holds each review named above and each owner role in the [risk register](plan/risk-register.md). The rule for what a fresh model may review, and its limits, are decided in [spec 0002](specs/0002-standards-and-tooling/index.md#reviewer-roster-and-fresh-model-reviews). [Fresh model](glossary.md#delivery) is a glossary term. A review is never done by the session or person that wrote the change.

Change a holder here, in one edit. `Not assigned` means nobody holds the role yet. While Project lead, Maintainer or Release maintainer is not assigned, the repository owner decides in that role and the record says so. For every other role, where the last column says a person is required and the role is not assigned, the gate that needs that review stays open.

| Role | Reviews or owns | Holder | May a fresh model do the review? |
|---|---|---|---|
| Security engineer | Security review of protocol, parser and authentication changes. `R-11`, `R-12` | Not assigned | Yes for the independent review of a slice. A person handles security reports |
| Protocol engineer | Protocol review of pooling and cancellation changes. `R-01`, `R-03`, `R-04`, `R-10` | Not assigned | Yes for the independent review of a slice |
| Database architect | Database review of failover and HA changes. `R-02`, `R-06`, `R-09` | Not assigned | Routing and consistency code: yes. A promotion, recovery or HA design: a person is required |
| HA engineer | `R-08` | Not assigned | No. A person approves fencing and authority designs and drills |
| Infrastructure operator | Infrastructure review of failover and HA changes. `R-20`, `R-21` | Not assigned | No. A person signs off lab and drill results |
| Systems engineer | `R-05`, `R-13`, `R-14` | Not assigned | Yes for the independent review of a slice |
| SRE | `R-07` | Not assigned | Code: yes. A drill result: a person is required |
| Control engineer | `R-18`, `R-19` | Not assigned | Code: yes. A crash or disk fault drill result: a person is required |
| Performance engineer | `R-15` | Not assigned | The method and the numbers may be checked by a fresh model. A public performance claim needs a person |
| Maintainer | `R-16`. Dependency updates and advisory `ignore` entries | Not assigned | A first pass over an update: yes. An `ignore` entry: a person is required |
| Release maintainer | `R-17`. The license list, notices and release sign off | Not assigned | No. A person decides licenses and signs off a release |
| Project lead | `R-22`, `R-23`, `R-24`, `R-25`. Accepting a risk, closing a gate, the branch rule on the remote | Not assigned | No |

## Shared conventions

Some surfaces are extended by every later slice. The slice that first exposes one records its rule in this table once. Later specs cite the rule instead of choosing again. An empty rule is an open decision for the named slice, not permission to improvise.

| Surface | First fixed in | Rule |
|---|---|---|
| Configuration keys: naming, units, unknown keys, secret references and reserved names | [001](scope/001-stack-and-workspace.md) | One TOML file with a required `config_version`. Lowercase `snake_case` names from the glossary. Integer quantities with a unit suffix. Unknown keys are rejected. Secrets only by a `_file` reference. A reserved capability is a table with `enabled`, default `false`, and `true` is rejected. Full rule: [spec 0001, configuration rules](specs/0001-initial-architecture/index.md#configuration-rules) |
| Process exit status, and command output for people and as JSON | [001](scope/001-stack-and-workspace.md) | Exit 0 success, 1 input rejected, 2 usage error, 3 could not complete. The result goes to stdout, usage errors to stderr. JSON is one object on one line that starts with `output_version`, `command` and `result` and ends with `findings`. Output is constant text and never repeats input. Full rule: [spec 0001, exit status and output](specs/0001-initial-architecture/index.md#exit-status-and-output) |
| Log event fields and redaction | [001](scope/001-stack-and-workspace.md) | One JSON object per line on stderr with `ts`, `level` and `event`, and named fields for anything that varies. Secrets are redacted by type, not by filter. Credentials, cancellation keys, SQL, database error text and configuration content are never logged by default. Fixed ahead of the first log event. Full rule: [spec 0001, log events and redaction](specs/0001-initial-architecture/index.md#log-events-and-redaction) |
| Identifier and generation types | [003](scope/003-control-and-session-data-model.md) | One type per identifier and per generation, which never compares with or converts into another kind. An identifier is built only by a function that checks its syntax. A generation advances by one and fails closed at the top. Types that more than one crate needs live in `pool-config`. A generation kept in memory is only comparable inside one start sequence. Full rule: [spec 0003, identifier and generation types](specs/0003-control-and-session-data-model/index.md#identifier-and-generation-types) |
| SQLSTATEs and messages that POOL itself generates | [004](scope/004-one-real-primary-path.md) | Pending |
| Metric names, units and bounded labels | The first slice that exports a metric, no later than [006](scope/006-protocol-breadth-and-cancellation.md) | Pending |
| Administration API versioning, error body and request ID | [009](scope/009-safe-topology-administration.md) | Pending |

Take every public name from the [glossary](glossary.md). A new term enters the glossary before it enters code.

## Tests and evidence

Test observable behavior and safety invariants. Choose the test layer and record the result using the [testing strategy](testing/strategy.md#acceptance-evidence). Follow the [benchmark method](testing/benchmark-plan.md) for performance claims.

## Git and releases

Use the [branch rule](glossary.md#delivery) on `main`, pull requests, required CI and review for safety changes. [Spec 0002](specs/0002-standards-and-tooling/index.md#branch-rule-and-change-path) fixes how: a change reaches `main` only through a pull request on which the seven required checks passed, each check is one command (`scripts/check.sh <name>`) that also runs on a workstation, and nobody is on the bypass list. Whether the rule is active on the remote is recorded in that spec's verification record, not assumed. Keep changes small and use conventional commits. Never commit secrets. Keep a changelog, semantic versioning policy, release tags, checksums, SBOM, notices, functioning examples and an upgrade/rollback guide. Verify the branch rule and required checks on the remote when setting up [standards and tooling](scope/002-standards-and-tooling.md).

## Workflow

Before code, read the relevant spec/architecture and scope, inspect actual existing functionality, identify dependencies, define an implementation plan and acceptance evidence, then implement only the authorized slice. Record load bearing changes in a spec/ADR rather than silently changing contracts. Run the required checks, document results and update feature status honestly. Architecture proposals do not authorize deployment or implementation beyond the user's current request.
