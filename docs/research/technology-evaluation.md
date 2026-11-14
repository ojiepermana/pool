# Rust technology evaluation

**Checked:** 2026-11-14. The registry check selected the newest stable, non yanked semantic version. A prerelease such as rustls 0.24 development was not counted as stable. Rust 1.100.0 was released on 2026-11-12. These are candidate pins, not an installed or compiled dependency graph. [Rust release](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/).

Each version link points to its primary registry record. Licenses are package declarations; transitive native libraries and selected features need a separate SBOM and notice review. Recent releases establish activity, not security or a support promise.

| Dependency | Stable snapshot | Declared license | Purpose and selection reason | Limitation and alternative | Timing |
|---|---|---|---|---|---|
| tokio | [1.54.1](https://crates.io/crates/tokio/1.54.1) | MIT | Async sockets and scheduling | Cooperative runtime can be starved by blocking work; threads or async-std are alternatives | Core |
| tokio-util | [0.7.20](https://crates.io/crates/tokio-util/0.7.20) | MIT | Cancellation and codec helpers | Does not define PostgreSQL state; direct runtime primitives are the alternative | Core as needed |
| bytes | [1.12.1](https://crates.io/crates/bytes/1.12.1) | MIT | Shared bounded buffers | Retained slices can retain large allocations; Vec is simpler for small owned values | Core |
| rustls | [0.23.46](https://crates.io/crates/rustls/0.23.46) | Apache-2.0 OR ISC OR MIT | TLS with explicit verification | Crypto provider and native dependencies need review; OpenSSL alternative | Core |
| tokio-rustls | [0.26.6](https://crates.io/crates/tokio-rustls/0.26.6) | MIT OR Apache-2.0 | Async TLS streams | Version coupling with rustls; native-tls alternative | Core |
| axum | [0.8.9](https://crates.io/crates/axum/0.8.9) | MIT | Typed administration endpoints | Body limits, auth and exposure remain application duties; direct hyper alternative | Admin slice |
| serde | [1.0.229](https://crates.io/crates/serde/1.0.229) | MIT OR Apache-2.0 | Config and API encoding | Deserialization is not semantic validation; manual parsing alternative | Adopted in 001 |
| serde_json | [1.0.154](https://crates.io/crates/serde_json/1.0.154) | MIT OR Apache-2.0 | JSON command output, later the administration API | A generic JSON map sorts its keys, so fixed field order needs derived structures; a hand written writer is the alternative | Adopted in 001 |
| toml | [1.2.1+spec-1.1.0](https://crates.io/crates/toml/1.2.1+spec-1.1.0) | MIT OR Apache-2.0 | Human configuration | Validate schema and unknown keys; its error text can repeat file content, so only the span is used; JSON alternative | Adopted in 001 |
| thiserror | [2.0.21](https://crates.io/crates/thiserror/2.0.21) | MIT OR Apache-2.0 | Typed library errors | Error contents need redaction; manual enums alternative | Core |
| anyhow | [1.0.104](https://crates.io/crates/anyhow/1.0.104) | MIT OR Apache-2.0 | Binary boundary context | Avoid losing typed protocol semantics; typed errors elsewhere | Binary only |
| tracing | [0.1.44](https://crates.io/crates/tracing/0.1.44) | MIT | Structured events and spans | Instrumentation can expose secrets or add cost; log facade alternative | Core |
| tracing-subscriber | [0.3.23](https://crates.io/crates/tracing-subscriber/0.3.23) | MIT | Redacted JSON log output | Filters are not a redaction boundary; explicit event fields needed | Core |
| prometheus-client | [0.26.0](https://crates.io/crates/prometheus-client/0.26.0) | Apache-2.0 OR MIT | Explicit typed metric registry | Cardinality is still an application limit; prometheus or OTel metrics alternative | Core metrics |
| opentelemetry | [0.33.1](https://crates.io/crates/opentelemetry/0.33.1) | Apache-2.0 | Standard tracing interfaces | SDK and bridge versions must align; tracing only initially | Later optional |
| opentelemetry-otlp | [0.33.1](https://crates.io/crates/opentelemetry-otlp/0.33.1) | Apache-2.0 | Collector export | Bounded exporter queues and outage behavior required | Later optional |
| tracing-opentelemetry | [0.34.0](https://crates.io/crates/tracing-opentelemetry/0.34.0) | MIT | Span bridge | Coupled release families; no bridge initially | Later optional |
| proptest | [1.12.0](https://crates.io/crates/proptest/1.12.0) | MIT OR Apache-2.0 | State and framing properties | Generated tests do not replace real drivers; deterministic model tests alternative | Test only |
| criterion | [0.8.2](https://crates.io/crates/criterion/0.8.2) | Apache-2.0 OR MIT | Codec and route microbenchmarks | Microbenchmarks do not establish end to end throughput | Benchmark only |
| clap | [4.6.7](https://crates.io/crates/clap/4.6.7) | MIT OR Apache-2.0 | CLI argument contract | Keep generated help and API versions aligned; manual parser alternative | CLI slice |
| arc-swap | [1.9.5](https://crates.io/crates/arc-swap/1.9.5) | MIT OR Apache-2.0 | Immutable snapshot publication | Adopt only when simpler watch/Arc ownership is insufficient | Conditional |
| tokio-postgres | [0.7.18](https://crates.io/crates/tokio-postgres/0.7.18) | MIT OR Apache-2.0 | Monitor and control SQL connections | Terminates its own protocol, unsuitable for transparent frontend forwarding | Evaluate for the first writer identity/role probe; broader health checks later |
| pg_query | [18.1.0](https://crates.io/crates/pg_query/18.1.0) | MIT | PostgreSQL 18 raw grammar AST | C, FFI, native build and catalog blind semantics; sqlparser alternative | Routing slice |
| sqlparser | [0.64.0](https://crates.io/crates/sqlparser/0.64.0) | Apache-2.0 | Pure Rust dialect parser alternative | Syntax coverage differs from PostgreSQL; not selected as arbitrary SQL authority | Evaluation |
| pgwire | [0.42.2](https://crates.io/crates/pgwire/0.42.2) | MIT OR Apache-2.0 | Candidate PostgreSQL server SCRAM and codec pieces | Proxy parameter/status semantics must override server defaults; rsasl alternative | Auth evaluation |
| rsasl | [2.3.1](https://crates.io/crates/rsasl/2.3.1) | Apache-2.0 OR MIT | General SASL alternative | More PostgreSQL integration than pgwire; not selected or installed | Auth evaluation |
| rusqlite | [0.42.0](https://crates.io/crates/rusqlite/0.42.0) | MIT | Durable local operation journal | Synchronous FFI and one writer; isolate from runtime. External control DB alternative | Minimal durable authority before first write; operation history in the control slice |
| zeroize | [1.9.1](https://crates.io/crates/zeroize/1.9.1) | Apache-2.0 OR MIT | Reduce retained secret material | Cannot erase all allocator, kernel or library copies | Auth slice |
| secrecy | [0.10.3](https://crates.io/crates/secrecy/0.10.3) | Apache-2.0 OR MIT | Restrict secret exposure in interfaces | Not encryption or a secret manager | Auth slice |
| rand | [0.10.3](https://crates.io/crates/rand/0.10.3) | MIT OR Apache-2.0 | Secure token generation through appropriate RNG API | Review algorithm and OS entropy failure paths | As required |
| subtle | [2.6.1](https://crates.io/crates/subtle/2.6.1) | BSD-3-Clause | Constant time primitives if auth adapter needs them | Primitive use alone does not prove timing safety | Conditional |
| sha2 | [0.11.1](https://crates.io/crates/sha2/0.11.1) | MIT OR Apache-2.0 | Standard hash primitive if not encapsulated by auth library | Avoid duplicating crypto implementations | Conditional |
| hmac | [0.13.0](https://crates.io/crates/hmac/0.13.0) | MIT OR Apache-2.0 | Standard MAC primitive | Prefer reviewed SCRAM library integration | Conditional |
| pbkdf2 | [0.13.0](https://crates.io/crates/pbkdf2/0.13.0) | MIT OR Apache-2.0 | SCRAM derivation primitive | Bound iteration counts and CPU admission | Conditional |

## Parser options

| Approach | Compatibility and performance implications | Safety, portability, license and maintenance |
|---|---|---|
| sqlparser | Useful dialect coverage and visitors; benchmark real PostgreSQL corpus | Pure Rust and Apache licensed; upstream states syntax focus rather than catalog semantics |
| pg_query and libpg_query, recommended | Uses actual PostgreSQL raw parser; benchmark FFI conversion, parse memory, and cache cost | Native compiler and binding requirements; isolate unsafe boundary. Upstream PG18 line is active, older lines have reduced support |
| Maintain a grammar fork or port | Maximum control, no automatic fidelity | Large ongoing grammar, encoding and error behavior burden; not recommended initially |
| Hybrid policy | Parse with pg_query, use session state and declared read profile, enforce database constraints | Recommended composition. Server enforcement cannot contain arbitrary external side effects in extensions; operator trust contract remains necessary |

[pg_query](https://github.com/pganalyze/pg_query.rs), [libpg_query](https://github.com/pganalyze/libpg_query), [pure Rust parser](https://github.com/apache/datafusion-sqlparser-rs), [PostgreSQL parser stages](https://www.postgresql.org/docs/18/parser-stage.html).

libpg_query includes PostgreSQL licensed server parts, BSD licensed library code, BSD 3 Clause upb, MIT utf8_range, and BSD 2 Clause xxHash. The Rust crate's MIT declaration alone is not enough for redistribution. Check the exact locked release rather than assuming the current branch matches it. [Native library notices](https://github.com/pganalyze/libpg_query/blob/18-latest/README.md).

## Authentication, journal, and consensus

Evaluate pgwire's low level SCRAM support before implementing protocol crypto. Its recent change log includes channel binding and proof validation fixes, a reason for careful version review rather than automatic trust. Test PostgreSQL SASLprep compatibility, username source, PLUS negotiation, and separate TLS channel binding. The older `scram` crate lacks channel binding and is not the recommendation. [pgwire docs](https://docs.rs/pgwire/latest/pgwire/), [change log](https://docs.rs/crate/pgwire/0.42.2/source/CHANGELOG.md), [SASL](https://www.postgresql.org/docs/18/sasl-authentication.html).

For a local operation journal, recommend SQLite WAL with synchronous FULL, bounded checkpoints and a dedicated writer. Acknowledged journal operations must survive process and power loss within the storage system's guarantees. WAL is local concurrency, not distributed coordination. [SQLite WAL](https://www.sqlite.org/wal.html), [synchronous modes](https://www.sqlite.org/pragma.html).

Do not add consensus to the initial proxy. For later HA, external etcd is the recommended candidate; OpenRaft is a credible embedded alternative but its evolving APIs and required storage/network integration enlarge POOL's responsibility. A lease alone cannot stop a paused old leader from promoting PostgreSQL. Fencing and operation epochs remain required. [etcd guarantees](https://etcd.io/docs/v3.6/learning/api_guarantees/), [OpenRaft](https://github.com/databendlabs/openraft).

## Adopted in scope 001

The first build adopts three direct dependencies, recorded in [spec 0001](../specs/0001-initial-architecture/index.md#adopted-dependencies). This is the review the [adoption gate](#adoption-gate) asks for, done on 2026-11-14 with a throwaway build outside the repository on Rust 1.99.1. The build with the pinned Rust 1.100.0 and the committed `Cargo.lock` is the developer's evidence, not this one.

| Dependency | Version and features | Used by | Runtime cost | Native code |
|---|---|---|---|---|
| serde | 1.0.229, feature `derive` | `pool-config`, `pool` | Compile time code generation only | None |
| toml | 1.2.1, no default features, features `parse` and `serde` | `pool-config` | Parses one file of at most 1 MiB per run | None |
| serde_json | 1.0.154, default features | `pool` | Writes one small document per run | None |

Locked packages in that build, sixteen in all. A different set of names in the real `Cargo.lock` is a finding to report, not to accept.

| Package | Version | Declared license | Minimum Rust |
|---|---|---|---|
| itoa | 1.0.19 | MIT OR Apache-2.0 | 1.68 |
| memchr | 2.8.3 | Unlicense OR MIT | 1.61 |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | 1.71 |
| quote | 1.0.47 | MIT OR Apache-2.0 | 1.71 |
| serde | 1.0.229 | MIT OR Apache-2.0 | 1.56 |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | 1.56 |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | 1.71 |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | 1.71 |
| serde_spanned | 1.1.3 | MIT OR Apache-2.0 | 1.85 |
| syn | 3.0.10 | MIT OR Apache-2.0 | 1.71 |
| toml | 1.2.1+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 |
| toml_datetime | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 |
| toml_parser | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | 1.85 |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | 1.71 |
| winnow | 1.0.5 | MIT | 1.65 |
| zmij | 1.0.23 | MIT | 1.71 |

`proc-macro2`, `quote`, `syn`, `serde_derive` and `unicode-ident` run only at build time. Advisories and license files were not checked by a tool; that check arrives with [scope 002](../scope/002-standards-and-tooling.md) under `SEC-13`. Not adopted for this build, with reasons in the [spec rationale](../specs/0001-initial-architecture/rationale.md#decisions-and-runner-ups): clap, thiserror, anyhow, tracing and tracing-subscriber.

The Linux builder image is `rust:1.100.0-slim-bookworm`, index digest `sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37`, read from the Docker Hub tag listing on 2026-11-14 and not yet pulled.

## Adopted in scope 002

Scope 002 adopts tools, not product dependencies. `pool` and `pool-config` gain none. The decisions are in [spec 0002](../specs/0002-standards-and-tooling/index.md), with the alternatives in its [rationale](../specs/0002-standards-and-tooling/rationale.md#decisions-and-runner-ups). This is the review the [adoption gate](#adoption-gate) asks for, done on 2026-11-14 in a scratch directory outside the repository. What was run there and what was only read is listed under [evidence gathered](../specs/0002-standards-and-tooling/rationale.md#evidence-gathered). The versions the developer observes in the real build are added below when the build is done.

| Tool | Version | Purpose | License | Native code | Cost at run time | Alternative |
|---|---|---|---|---|---|---|
| GitHub Actions, hosted runner `ubuntu-24.04-arm` | Service; the image label is fixed. 64 bit ARM, the architecture of the Linux target | Runs the seven required checks on every pull request, on a push to `main` and once a day | Service terms, not a package | Not applicable | None in the product. Seven short jobs per run, free for a public repository | A self hosted runner; `ubuntu-24.04` (Intel or AMD), which would emulate the Linux build |
| `actions/checkout` | `v7.0.1`, commit `3d3c42e5aac5ba805825da76410c181273ba90b1` | Fetches the repository in a job | MIT | Runs on the runner's Node runtime | None in the product | `git clone` in a `run:` step |
| `actions/cache` | `v6.1.0`, commit `55cc8345863c7cc4c66a329aec7e433d2d1c52a9` | Keeps the built `cargo-deny` binary between runs | MIT | Runs on the runner's Node runtime | None in the product | Build the tool in every run |
| cargo-deny | [0.20.2](https://crates.io/crates/cargo-deny/0.20.2), minimum Rust 1.88.0 | Advisories, licenses, banned and duplicate packages, package sources | MIT OR Apache-2.0 | A separate program, not linked into the product | None in the product. About 3 seconds per check locally once the advisory database is fetched | cargo-audit plus cargo-about |
| cargo-fuzz | [0.13.2](https://crates.io/crates/cargo-fuzz/0.13.2) | Builds and runs fuzz targets | MIT OR Apache-2.0 | A separate program | None in the product | cargo-afl |
| libfuzzer-sys | [0.4.13](https://crates.io/crates/libfuzzer-sys/0.4.13), pinned with `=` | The libFuzzer runtime that a fuzz target links | (MIT OR Apache-2.0) AND NCSA | Yes: LLVM libFuzzer, C++, compiled by the `cc` crate. Only in `crates/pool-config/fuzz/` | None in the product; it is never a dependency of `pool` or `pool-config` | AFL |
| Rust toolchain `nightly-2026-10-17` | Pinned by date | Needed by cargo-fuzz for the sanitizer flags | Same as Rust | Not applicable | Used for fuzz runs only. The product builds with 1.100.0 | None; cargo-fuzz needs a nightly toolchain |
| cargo-cyclonedx | [0.5.10](https://crates.io/crates/cargo-cyclonedx/0.5.10), minimum Rust 1.86.0 | Writes the software inventory (SBOM) in CycloneDX JSON | Apache-2.0 | A separate program | None in the product. Run by hand until scope 011 | cargo-sbom 0.10.0 |
| PostgreSQL container image | `postgres:18.<minor>-bookworm` by digest, which has an ARM variant. Not adopted yet; the rule is fixed and scope 004 pins the tag | The database for integration tests, the same locally and in CI | PostgreSQL License | A separate container | None in the product | Distribution packages; a `services:` block in the workflow |

The fuzz package has its own lockfile, `crates/pool-config/fuzz/Cargo.lock`. Besides the packages the product already locks, the scratch build resolved these ten:

| Package | Version | Declared license | Role |
|---|---|---|---|
| arbitrary | 1.5.0 | MIT OR Apache-2.0 | Structured input support that libfuzzer-sys depends on |
| cc | 1.11.2 | MIT OR Apache-2.0 | Build time: compiles libFuzzer |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | Build configuration helper |
| find-msvc-tools | 0.1.15 | MIT OR Apache-2.0 | Build time, used by `cc` |
| getrandom | 0.4.3 | MIT OR Apache-2.0 | Used by `jobserver` |
| jobserver | 0.1.35 | MIT OR Apache-2.0 | Build time, used by `cc` |
| libc | 0.2.191 | MIT OR Apache-2.0 | System interface |
| libfuzzer-sys | 0.4.13 | (MIT OR Apache-2.0) AND NCSA | The fuzz runtime |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | A target specific dependency of `getrandom`, not built for Linux or macOS |
| shlex | 2.0.1 | MIT OR Apache-2.0 | Build time, used by `cc` |

This is the one native code exception so far, and it is test only. The justification the [adoption gate](#adoption-gate) asks for: fuzzing is the validation the [risk register](../plan/risk-register.md) names for malformed input (`R-12`), libFuzzer is the standard engine for it, and the package that links it is outside the workspace, is never published and cannot become a dependency of the product. The guard on the product lockfile (rule W1 of spec 0002) fails if any of these ten packages appears there.

Once spec 0002 is built, licenses and advisories are checked by a tool on every pull request. In the scratch build, `cargo deny check` with the configuration in spec 0002 passed on both lockfiles: `advisories ok, bans ok, licenses ok, sources ok`. The sentence under [Adopted in scope 001](#adopted-in-scope-001) which says they were not checked by a tool describes the state before that.

The PostgreSQL image was looked up and not pulled. On 2026-11-14 the newest Debian 12 tag was `postgres:18.7-bookworm`, pushed 2026-11-12, index digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`. Scope 004 reads the tag again when it adopts the image.

## Adoption gate

Before installing a dependency, verify its stable version, selected features, MSRV, advisories, native dependencies, license files, and maintainer activity again. Commit Cargo.lock for binaries, generate an SBOM, and test Linux targets. First party production Rust should forbid unsafe code; a documented dependency exception covers native parser, TLS provider, and SQLite boundaries after review. The dependencies adopted so far are listed under [Adopted in scope 001](#adopted-in-scope-001) and [Adopted in scope 002](#adopted-in-scope-002).
