# 2. Standards and tooling · planned

[Scope index](index.md)

Reconcile the planning rules with the actual scaffold and install the required checks.

**Done when:** Documented conventions match the workspace and format, lint, test and dependency checks run in CI.

The maintainer uses the real manifest to select required CI checks, branch review rules and a path to reproduce failures locally. Include documentation checks for local links/anchors, scope status agreement and stable requirement IDs. Verify the checks detect a deliberately failing change before relying on them. See [engineering standards](../engineering-standards.md) and [CI progression](../testing/strategy.md#ci-progression).

- [ ] Capture conventions: `/audit`
- [ ] Verify the required checks on a pull request and link the result.
