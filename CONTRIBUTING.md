# Contributing to POOL

The project has its first workspace slice and is otherwise in planning. You can begin with [the documentation index](docs/index.md), [scope](docs/scope/index.md) and [engineering standards](docs/engineering-standards.md). The checks you can run today are listed below.

For a change, describe the problem, relevant feature/spec, intended behavior, safety boundary and validation. Propose architectural changes before implementing them. Use small pull requests and conventional commits. Include the acceptance criteria and real test evidence, and document limitations. Protocol, authentication, parser and failover work requires the appropriate independent review.

The workspace pins Rust 1.100.0 in `rust-toolchain.toml`. Before you propose a change, you can run these from the repository root:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

The Linux container build is the command under [Linux build](docs/specs/0001-initial-architecture/index.md#linux-build) in spec 0001. There is no CI yet; it arrives with [scope 002](docs/scope/002-standards-and-tooling.md). Later checks will include property tests, PostgreSQL integration, and targeted compatibility/fuzz/chaos tests. Do not invent passing results while these facilities are still planned.

POOL uses Apache 2.0. Contributions should be compatible with that license and preserve required attribution. Do not copy third party code without a per file license review and provenance record. A separate contributor agreement is not required by this initial plan; any future governance change needs maintainer discussion.

Avoid posting secrets, user SQL or sensitive database data in issues. Security reporting guidance is in [SECURITY.md](SECURITY.md). The project still needs a named maintainer team, a real private reporting channel, and remote repository review/branch protection settings before production releases.
