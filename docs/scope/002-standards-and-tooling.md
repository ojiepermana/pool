# 2. Standards and tooling · planned

[Scope index](index.md)

Reconcile the planning rules with the actual scaffold and install the required checks.

**Done when:** Documented conventions match the workspace and format, lint, test and dependency checks run in CI.

The maintainer uses the real manifest to select required CI checks, branch review rules and a path to reproduce failures locally. Include documentation checks for local links/anchors, scope status agreement and stable requirement IDs. Verify the checks detect a deliberately failing change before relying on them. Record the choices this feature makes, because no earlier document selects them: the CI service, the dependency, license and advisory checker, the SBOM generator, the fuzz harness, and how PostgreSQL 18 is supplied to integration tests locally and in CI. Add each adopted tool to the [technology evaluation](../research/technology-evaluation.md). Also record the reviewer roster: who holds each review role named in the [standards](../engineering-standards.md#security-and-reviews) and the [risk register](../plan/risk-register.md), and which reviews a fresh model may perform instead of a human specialist. See [engineering standards](../engineering-standards.md) and [CI progression](../testing/strategy.md#ci-progression).

- [ ] Capture conventions: `/audit`
- [ ] Verify the required checks on a pull request and link the result.
