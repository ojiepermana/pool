# Contributing to POOL

The project is in planning. You can begin with [the documentation index](docs/index.md), [scope](docs/scope/index.md) and [engineering standards](docs/engineering-standards.md). There is no build command to run until the minimal workspace slice is authorized and created.

For a change, describe the problem, relevant feature/spec, intended behavior, safety boundary and validation. Propose architectural changes before implementing them. Use small pull requests and conventional commits. Include the acceptance criteria and real test evidence, and document limitations. Protocol, authentication, parser and failover work requires the appropriate independent review.

When a workspace exists, its contributor guide will record exact setup and checks from the manifests and CI. Expected checks include rustfmt, Clippy, unit/property tests, PostgreSQL integration, and targeted compatibility/fuzz/chaos tests. Do not invent passing results while these facilities are still planned.

POOL uses Apache 2.0. Contributions should be compatible with that license and preserve required attribution. Do not copy third party code without a per file license review and provenance record. A separate contributor agreement is not required by this initial plan; any future governance change needs maintainer discussion.

Avoid posting secrets, user SQL or sensitive database data in issues. Security reporting guidance is in [SECURITY.md](SECURITY.md). The project still needs a named maintainer team, a real private reporting channel, and remote repository review/branch protection settings before production releases.
