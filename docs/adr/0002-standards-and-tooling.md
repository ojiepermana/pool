# ADR 0002. Standards and tooling

The canonical decision and lifecycle status are in [spec 0002](../specs/0002-standards-and-tooling/index.md). Its [rationale](../specs/0002-standards-and-tooling/rationale.md) records alternatives, tradeoffs, and sources.

The decision is seven required checks, each one command (`scripts/check.sh <name>`) that runs the same way on a workstation and on GitHub Actions, and a branch rule under which a change reaches `main` only through a pull request whose checks passed. cargo-deny checks dependencies, licenses and advisories. A small Rust program outside the workspace guards the workspace, the CI definition and the documents. cargo-fuzz, cargo-cyclonedx and the official PostgreSQL 18 image are the chosen fuzz harness, inventory generator and test database. This ADR is an index entry, not a second decision record.
