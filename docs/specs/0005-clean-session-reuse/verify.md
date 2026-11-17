# Verify: clean session reuse, spec 0005

Updated 2026-11-17. All ten acceptance criteria pass for the declared profile. You confirmed the main chat author family as Sol, resolving the remaining independence gate for the fresh Terra reviews. Scope 005 is done and spec 0005 is Accepted.

## Environment and change set

Base: `2dbc03780b16abafcfa7c5308a56107005c1049b`, branch main, with uncommitted scope 005 changes. Existing local context edits and removed local skills were preserved. Native source/test tasks used isolated worktrees and were integrated only after their writers stopped. Review covered 41 selected files plus the fixes; unrelated initial context/skill changes were excluded from its checkout.

The original source digest `46c255769dc5678363fe575c9e2a5466fe8251c563bba9d495748e66a1e5d784` lacks a reproducible serialization recipe and is retained only as historical provenance. The current [source manifest](source-manifest.json) records 51 selected source, test and tooling files, including the final control test correction, with digest `13248d15a681e227e9faef83a205f6bbb6b772712491ce5038bdfaa96b463cca`. It specifies both the per file hashes and the aggregate algorithm. Worker provenance, paths and decisions are in the [coordination state](../../scope/coordination/005-clean-session-reuse.state.json).

PostgreSQL: the disposable PostgreSQL 18.7 image pinned in scripts/check.sh. Tests run the actual daemon, TLS/SCRAM and real PostgreSQL. The reusable test login explicitly receives pg_read_all_settings and TEMP and then loses those grants; product code never grants database permissions. Workstation runs use the macOS proxy and Linux PostgreSQL containers. Linux proxy integration is not claimed.

Linux: aarch64-unknown-linux-gnu, Rust 1.100.0, pinned image `rust:1.100.0-slim-bookworm@sha256:7c7d4eb4d05b7781187a327809466cfddf29753f6defdc3d19a0e291951b7a37`, read only source mount and a native Docker volume for target/temp files. The final build was limited to two CPUs, two Cargo jobs and 2 GiB.

## Checks

- [x] Six local required checks: fmt, clippy, test, deny, guards and docs. `scripts/check.sh all` passed; after the final test-only correction, `scripts/check.sh fmt clippy test` and final docs/guards checks passed. AC-10.
- [x] `scripts/check.sh integration`, unfiltered, 36 passed, zero failures. Final production code, including the owned derivation seam, was exercised after Docker recovered. The later change only installs subscribers in two unit tests. AC-1 through AC-8, AC-10.
- [x] Direct Linux Release build and default workspace tests passed. The affected app test group also passed five consecutive two-thread runs. This is direct validation of the changed tree; the clean committed-tree `scripts/check.sh linux-build` was not run. AC-9, AC-10.
- [x] Same PostgreSQL PID at global/node cap 1 after idle, open-transaction and failed-transaction cleanup, with defaults restored and no temp object, SQL prepared statement, held cursor, advisory lock or LISTEN residue. A later notification is not delivered to the next owner. AC-1, AC-5.
- [x] Exact key fields and canonical startup values separate owners; a wrong credential reference cannot borrow an authenticated idle backend. Unknown SQL, unsafe startup paths and absent profile permissions retain dedicated execution. AC-2, AC-5.
- [x] Global and nested budgets, all physical states, useful idle eviction, FIFO, zero waiters, cancelled heads and handshakes with available capacity have regression coverage in the working tree. Real PostgreSQL tests prove global saturation across mapping keys/databases and progress after release. AC-3, AC-4.
- [x] Retirement before handoff, while waiting, during handshake/cleanup, and after the actual mark with greeting blocked. The last case observes reused increment while sessions is zero, rotates, then proves the pinned owner succeeds and cannot return under the old generation. Real SIGHUP retires old idle and returned owners. AC-6.
- [x] Idle expiry, physical lifetime and client write bounds are tested; active owners are never moved to another backend. AC-7.
- [x] Same-challenge coalescing, cancelled callers, concurrent different challenges, 64-slot saturation across credentials and blocking-worker panic use the actual production derivation owner path. Temporary early-slot-release and post-handoff retired-owner mutations failed the new tests before restoration. AC-8.
- [x] All 21 assigned tooling mutations and additional review bypass mutations were rejected after positive baselines passed. Exact test-call expectations preserve grouped thread/socket bans, and every PEM header occurrence is scanned. The full clean committed-tree proof script was not run. AC-9.
- [x] Forced connection-task drop, daemon task supervision/failure events and safe allocation reservation seams have regression tests. Rejected lengths never reach the production body reservation callback at any header split; this observes that reservation site rather than every process allocation, with no unsafe allocator. AC-9.
- [x] Technical security/protocol review approved all fixes on a fresh Terra context. The reviewer independently ran four ownership/handoff regressions. Both review reports are linked below. AC-10, technical portion.
- [x] Main chat author family confirmed as Sol by your explicit answer on 2026-11-17. Core/tooling authors were configured Sol; the original reviewers were configured Terra with fork_turns none. This establishes family contrast and fresh context for the accepted reviews. Exact release execution metadata and effort remain unobserved. AC-10, independent review gate.

Linux Release SHA-256: `34d2858af34d2bc1e8df56430423249b4bb693f8aaa5350e839e9643c232464e`. The executable reports `pool 0.1.0`.

## Reproduction

The normal checks are `scripts/check.sh all` and `scripts/check.sh integration`. The integration command must be unfiltered to count as the full check.

The direct Linux command uses the pinned image above, `--platform linux/arm64 --cpus 2 --memory 2g`, environment RUSTUP_TOOLCHAIN=1.100.0, CARGO_INCREMENTAL=0, CARGO_BUILD_JOBS=2, CARGO_PROFILE_DEV_DEBUG=0, CARGO_TARGET_DIR=/out and CARGO_HOME=/out/cargo-home. Mount the source read only at /work and a native Docker volume at /out. Run cargo build --locked --release -p pool, cargo test --locked --workspace, sha256sum /out/release/pool, then /out/release/pool --version. Five prior app-unit runs used --test-threads=2 to prove the capture correction under parallel execution.

## Failures found and corrected

1. The initial real PostgreSQL run passed 33/36: startup search_path was absent from the qualification allowlist, so real backends stayed dedicated. Actual startup messages identified the missing field; the list and baseline comparison now require it. All six new real cases and the full suite pass.
2. Static integration inspection found waiter cap zero rejected even immediately available capacity and charged connecting owners as waiters. Successful acquisition now leaves the queue immediately, and cap zero only refuses an actual capacity wait. Production regressions pass.
3. Review found two tooling bypasses, then missing derivation-bound and precise handoff coverage. Every major received a fix and same-reviewer approval. [Tooling review](../../reviews/2026-11-16-scope005-tooling.md), [complete review](../../reviews/2026-11-16-scope005-clean-session-reuse.md).
4. The first direct Linux target on a shared host filesystem failed two file-lock assertions. Repeating with native Docker storage passed them. Subsequent simultaneous Docker work was interrupted by API 500 and unable-to-start errors; those runs are recorded as failures, not passes. Docker's supported CLI restart recovered the stopped engine, and checks resumed serially.
5. The final Linux parallel run exposed a missing node.lost test event while the state assertions passed. A dynamic-interest experiment did not fix it and was removed. Two state-only tests lacked the subscriber used by their group. Installing the same capture guard in both yielded five passing parallel repetitions and the passing default workspace run; the reviewer approved that exact six-line correction.

Raw workstation logs are retained under target/scope005. No commit, push, pull request, CI execution, standard committed-tree Linux check, full committed-tree proof script or optional fuzz run is claimed. No dependency, CI definition, promotion or result-cache capability was added. The conservative profile remains the only reusable SQL/startup profile; scope 006 must requalify new protocol states.

## Resume verification, 2026-11-17

The Leader ran `scripts/check.sh all` with exit 0, saved in `target/scope005/resume-all.log`. All six local checks and 822 local test cases passed. The unfiltered `scripts/check.sh integration` also exited 0 with 36 passed and zero failures in 68.29 seconds, saved in `target/scope005/resume-integration.log`. This run launched the real daemon against disposable PostgreSQL 18.7 with TLS/SCRAM. It again proved same PID reuse with state reset at cap one, credential reference isolation, aggregate budgets, dedicated fallback and SIGHUP retirement. Both test containers were removed by the runner. Final documentation and guards checks passed after the prose edits (`target/scope005/resume-docs-guards.log`); the manifest, its documented reproduction snippet and `git diff --check` also passed.

A fresh native Terra/medium helper reconstructed every source section of `target/scope005/review-input.patch` from base `2dbc03780b16abafcfa7c5308a56107005c1049b` and compared it with the current files. The differences are the later derivation owner seam/tests, post handoff retirement test, daemon capture tests and the six line control capture correction. The final review explicitly covers these changes. This is an artifact identity audit, not a new review or a rerun of the historical Linux checks. The original review reports retain their historical unverified family note; your later Sol confirmation above resolves it without changing what was known when they were written.

No Rust, runtime configuration, dependencies or test code changed during this resume. Existing Linux Release and workspace evidence therefore remains applicable. The final edits reconcile documentation, scope and provenance only. The original native source/test tasks remain recorded as native_task_adaptation; workflow ownership and final acceptance remain with the Leader under auto decision mode.

To reproduce the new source fingerprint from the repository root:

```python
import hashlib
import json
from pathlib import Path

manifest = json.loads(Path("docs/specs/0005-clean-session-reuse/source-manifest.json").read_text())
records = []
for entry in sorted(manifest["files"], key=lambda item: item["path"]):
    digest = hashlib.sha256(Path(entry["path"]).read_bytes()).hexdigest()
    assert digest == entry["sha256"], entry["path"]
    records.append(entry["path"] + "\0" + digest + "\n")
assert hashlib.sha256("".join(records).encode()).hexdigest() == manifest["sha256"]
```

## Acceptance status

AC-1 through AC-10 pass for the declared profile. The resumed checks, artifact comparison and explicit author confirmation close the remaining gate. Scope 005 is done and spec 0005 is Accepted. Delivery is local and uncommitted; no pull request or CI result is claimed.
