# Initial architecture rationale

## Context

POOL is intended to carry arbitrary PostgreSQL application traffic and eventually manage failure sensitive infrastructure. Incorrect routing, session reuse, or promotion can damage data even when the Rust program has no memory safety defect. Your first environment has one proxy VM and three PostgreSQL VMs. At the start of the initial planning exercise, the project had no implementation, manifests or Git history. This describes the starting point of the decision, not the current checkout.

The full Pgpool feature set is larger than a safe first release. Your initial scenario still requires health, replica lag, routing, diagnostics, and manual failover. The roadmap therefore distinguishes a core proxy MVP from an operational V1 rather than quietly removing those requirements.

## Options considered

| Option | Benefit | Cost |
|---|---|---|
| Modular Rust daemon and CLI, recommended | Small deployment surface and direct socket ownership | Control and data share process failures; extraction requires stable contracts |
| Proxy and controller services from the beginning | Fault isolation and independent release cadence | Requires authentication, availability, versioning, and reconciliation across processes before the first proxy works |
| Build orchestration around existing poolers | Fastest path to an operating cluster | Does not fulfill the goal of a new Rust protocol and routing implementation; still a useful operational comparison |
| Reproduce Pgpool process and feature architecture | Familiar operational model and clearer compatibility ambition | Imports legacy constraints and a large parity burden before correctness is established |

The recommendation follows your deployment size and the practice of proving a thin working path before adding distributed coordination. Separate services remain appropriate when a measured isolation or ownership requirement outweighs their operating cost. (basis: your brief, vertical delivery, explicit failure domains)

## Rationale

The protocol engine, authentication, and session cleanup dominate the initial correctness risk. They benefit from explicit state and narrow ownership. A large set of small crates would make interfaces rigid before this behavior is understood. A single daemon makes observations and configuration publication simpler; it does not make promotion safe by itself.

The syntax parser choice favors PostgreSQL 18 grammar coverage over a pure Rust build. Its native code must be isolated, bounded, reviewed, and fuzzed. The raw parser cannot resolve functions, rules, views, or operators against the live catalog. Replica eligibility therefore requires a declared application contract and backend enforcement in addition to syntax. (basis: PostgreSQL parser and hot standby documentation)

Session pooling is the first safe mode because it preserves a real backend session throughout a client connection. Its scaling benefit is smaller than transaction pooling. That limitation is accepted explicitly rather than hidden in a pooling performance claim.

## References

The user requested verified sources, so references are included. The research check date is 2026-11-14. Registry versions are a dated snapshot, not a claim that a compatible dependency graph has been compiled.

| Basis | Verified source |
|---|---|
| Pgpool functional baseline | [Pgpool 4.7 manual](https://www.pgpool.net/docs/4.7/en/html/) |
| Protocol and cancellation | [PostgreSQL 18 protocol](https://www.postgresql.org/docs/18/protocol.html) |
| Parser semantics limit | [PostgreSQL parser stage](https://www.postgresql.org/docs/18/parser-stage.html) |
| Replica semantics | [PostgreSQL hot standby](https://www.postgresql.org/docs/18/hot-standby.html) |
| Fencing requirement | [PostgreSQL failover](https://www.postgresql.org/docs/18/warm-standby-failover.html) |
| Compiler choice | [Rust 1.100.0 announcement](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/) |
| Native parser | [pg_query upstream](https://github.com/pganalyze/pg_query.rs) |
| Future consensus | [etcd guarantees](https://etcd.io/docs/v3.6/learning/api_guarantees/) |
| First build dependencies | crates.io registry, read through Cargo on 2026-11-14 (see [Evidence gathered](#evidence-gathered)) |
| Linux builder image | Docker Hub tag listing for the official `rust` image, read on 2026-11-14, image not pulled |
| First build requirements | [Workspace handoff](../../implementation-order.md#first-handoff-the-workspace), [security requirements](../../security/security-requirements.md) `SEC-4`, `SEC-10`, `SEC-11`, `SEC-12`, `SEC-14` |

The requested human research views live in `docs/research/`. They include the feature inventory, competitor evidence, and dependency evaluation. This location follows your explicit requested deliverable structure.

## Independent critique

A different model reviewed the draft without fetching its reference links or changing files. It found gaps in durable writer authority, login selection versus later SQL admission, lag semantics and cancellation delivery. It also requested explicit credential mapping, initial acquisition retry boundaries, crate type ownership and an all-writer barrier for known WAL evidence. You approved all seven recommended resolutions. They are recorded in the [decision ledger](../../plan/decision-ledger.md) and incorporated into the contracts. This is design evidence only; no runtime behavior has been tested.

## Workspace build notes (2026-11-14)

### Context

The [workspace handoff](../../implementation-order.md#first-handoff-the-workspace) asks for a minimal Linux build of the daemon and an offline configuration check, and lists what must be settled first. The repository had no manifest or code. The workstation is macOS on Apple silicon with Rust 1.99.1 as its `stable`; Rust 1.100.0 is available through `rustup`; Docker 29.9.2 is installed but its daemon was not running. Three people work from the result without talking to each other: one builds, one verifies, one tests. So every value they need has to be written down once.

Three forces shaped the notes. First, `SEC-4` and `SEC-11` say a secret must never reach output, and a configuration file is exactly where an operator pastes one by mistake. Second, `SEC-10` and safety contract 10 say caching and automatic promotion stay off, and asking for them must be refused, not ignored. Third, the handoff says to adopt only what this path uses, while three shared conventions (keys, exit status and output, log events) must be fixed now because every later slice extends them.

### Decisions and runner ups

Each line is a decision taken on your behalf under the `auto` decision mode. None changes a ratified decision.

| Question | Taken | Runner up | Why |
|---|---|---|---|
| Linux target | `x86_64-unknown-linux-gnu` | `aarch64-unknown-linux-gnu`, which builds without emulation on this workstation | Proxmox VE runs on 64 bit Intel and AMD hosts, so your VMs are almost certainly `x86_64`. The GNU C library is chosen over musl because later slices link native code (`pg_query`, a TLS provider, SQLite) that is simpler to build against it. Changing it later is one flag and one triple |
| Linux builder | Official `rust:1.100.0-slim-bookworm` image by digest, native build inside it | Cross compiling from macOS | Cross compiling needs a linker or a helper tool that would have to be installed. The container needs only Docker. The slim image is chosen over the full one because the disk is nearly full and this build needs only `gcc` as a linker. Debian 12 gives an older C library than Debian 13, so the binary runs on more hosts |
| Clean checkout | `git archive HEAD` piped into the container | Mounting the working tree | A mount would include untracked files and a host `target/` directory. The archive holds only what is committed, which is what "clean checkout" means |
| Argument parsing | Standard library | `clap` 4.6.7 | The daemon has two flags. `clap` would add its own set of packages to a process that will later hold credentials. The usage exit status is 2, which is also what `clap` uses, so adopting it later breaks no script |
| JSON writing | `serde_json` 1.0.154 | A hand written writer | Escaping and field order are easy to get subtly wrong by hand, the crate is small and pure Rust, and the administration API will need it anyway |
| Error types | Hand written `Display` and `Error` | `thiserror` 2.0.21, `anyhow` 1.0.104 | The library has one small error type. A derive macro saves about ten lines |
| Logging in this build | None; rule fixed for later | Adopt `tracing` now and emit one event | `check-config` has nothing to log, and its findings are its result. Adopting a logger for an unused path goes against the handoff |
| What a diagnostic may contain | Only constants, schema key paths, line and column | Forward the parser's message, which names the unknown key | A scratch test (below) showed the parser's display prints the source line and its message repeats a wrong typed value. A rule of "no file content at all" is simple to state, to test and to keep true as keys are added. The cost is a terse syntax or schema finding |
| Syntax versus schema | Two passes: generic table, then typed structures | One pass and one code | Two codes tell an operator whether the file is broken TOML or valid TOML that POOL does not accept. The second parse is cheap at this size |
| Missing key | Every key optional in the Serde layer; POOL reports `config.missing_key` with the key path | Let Serde report it | Serde points a missing key at line 1, column 1, which misleads. A key path is exact |
| Name of the reserved promotion capability | `automatic_promotion` | `automatic_failover` | The dangerous act is promotion. Dropping a failed node from admission is ordinary health behavior and stays on. `SEC-10` and the handoff also say promotion |
| Shape of a reserved capability | A table with one key, `enabled` | A top level boolean such as `result_cache = false` | A table leaves room for the capability's own keys when a later spec approves it, with no rename |
| Other excluded capabilities (native replication, snapshot isolation replication) | No key at all | Reserve tables for them too | With unknown keys rejected, a capability without a key cannot be requested. Reserving more names now would only add contract surface |
| Smallest configuration | `config_version` and the two reserved tables | Also define listener, backend and identity map keys | Nothing in this build reads those keys. Scope 003 and 004 own their shape; defining them now would guess |
| Exit status | 0, 1, 2, 3 as in the spec | 0 and 1 only; or BSD `sysexits` numbers | A script needs to tell "your file is wrong" (1) from "I could not look" (3). Four small numbers are easy to remember; `sysexits` values are rarely known |
| Output format flag | `--format <human\|json>` | A `--json` switch | One option that takes a value can grow without a second flag |
| Help next to a usage error | One pass from left to right; whichever comes first wins | Help always wins | One rule covers every case and needs no second scan of the arguments |
| Where findings go | stdout for exit 0 and 1 | stderr | The findings are the answer the command was asked for. With JSON this keeps one parseable document on stdout |
| Fixture location | `crates/pool-config/tests/fixtures/` | A top level `tests/fixtures/` | The crate that owns the schema owns its examples, and the proposed repository layout needs no change |
| Initial version | `0.1.0` | `0.0.0` | It is the usual first number for something that runs. The version policy comes with releases |
| Configuration file size limit | 1 MiB | 64 KiB | An identity map for many principals can be long. 1 MiB is still trivial to hold in memory |

Two of these carry more weight than the rest because they become public contracts on day one: what a diagnostic may contain (it trades operator convenience for a hard guarantee), and the name `automatic_promotion` (a public key name). Both can still be changed before a release, by raising `config_version` or by a decision here.

### Evidence gathered

- **Registry check.** `cargo search` on 2026-11-14 returned `serde` 1.0.229, `serde_json` 1.0.154, `toml` 1.2.1, `thiserror` 2.0.21 and `clap` 4.6.7, matching the [technology evaluation](../../research/technology-evaluation.md) where listed.
- **Scratch build.** A throwaway project outside the repository, built with Rust 1.99.1, resolved the three direct dependencies to sixteen packages, all pure Rust, all under MIT or Apache 2.0 terms (`memchr` also offers the Unlicense; `unicode-ident` adds Unicode 3.0 terms for its data tables and is used only at build time). The highest minimum Rust version among them is 1.85. It was not built with 1.100.0; the developer's first build is that proof.
- **Parser behavior.** The same scratch project ran the planned fixture texts through `toml` 1.2.1. It confirmed the line numbers in the fixture table, that `toml::de::Error` gives a span for syntax and schema errors, that its `Display` text printed the canary for an unknown top level key and for a syntax error, and that its `message()` printed the canary for a wrong typed value. That is why the notes forbid both.
- **Image check.** The Docker Hub tag listing on 2026-11-14 shows `rust:1.100.0-slim-bookworm`, published 2026-11-12, with index digest `sha256:7c7d4eb4…51b7a37` and a `linux/amd64` variant. The image was not pulled, because the daemon was off.
- **Not verified.** The Linux build command has not been run. The exact help and usage text, the output bytes and the exit status are specified, not observed.

### Independent cross check

A different model (Claude Opus) read the drafted build notes without changing files, fetching links or running anything. Its verdict: the design is sound, the manifests and dependency choices hold, no ratified decision is changed, and the notes were not yet complete enough to build from without guessing. Under the `auto` decision mode its recommended fixes were applied, except where a simpler rule already settled the same point.

| Finding | What was done |
|---|---|
| `--help` could win or lose against a usage error depending on which rule you read | Settled as one pass from left to right in which the first event wins, with examples. The reviewer proposed that help always wins; the single pass is simpler to build and to test |
| The code for a bad `config_version` differed between the findings table and the typed structure | A table now gives the code for each kind of value. A scratch run confirmed that a value past the 64 bit range is a syntax error and one past 32 bits is a schema error |
| The human format had no stated check for syntax and schema findings | Tests now compare the three lines exactly, taking the column from the JSON output of the same file |
| The JSON criterion did not hold for help output or a failed write | **AC-10** now says when it applies |
| Order of checks for a repeated option, an empty value, case of `--format`, a value that contains an equals sign, arguments that are not UTF-8 | Each is now stated. An empty `--format=` is "option requires a value", by the same rule as `--config=`; the reviewer proposed the wrong format message |
| Derives and `Display` text of the public types were open | Stated |
| A failed write and the bounded read had no test | Added to **AC-11** (`/dev/full`, Linux only) and **AC-12** (`/dev/zero`) |
| Build plan step 1 sets the status line while a sentence in the Decision still says it stays Proposed | Step 1 keeps the status change, as the coordination rules ask, and leaves the sentence to the lead at close |
| No example per usage error, the exact size of the limit file, test file names, dotted and inline TOML forms, CRLF | Examples and cases added |
| The Linux script printed output before checking for the canary, did not assert the host line, and gave no message on a wrong exit status | Fixed in the script |
| A dependency record for `serde_json` was missing from the technology evaluation | It was added in the same change, after the reviewer had read the file |

This is a design review. Nothing in these notes has been built or run except the scratch checks listed above.
