# 2. Standards and tooling · in-progress

[Scope index](index.md)

Reconcile the planning rules with the actual scaffold and install the required checks.

**Done when:** Documented conventions match the workspace and format, lint, test and dependency checks run in CI.

The maintainer uses the real manifest to select required CI checks, branch review rules and a path to reproduce failures locally. Include documentation checks for local links/anchors, scope status agreement and stable requirement IDs. Verify the checks detect a deliberately failing change before relying on them. Record the choices this feature makes, because no earlier document selects them: the CI service, the dependency, license and advisory checker, the SBOM generator, the fuzz harness, and how PostgreSQL 18 is supplied to integration tests locally and in CI. Add each adopted tool to the [technology evaluation](../research/technology-evaluation.md). Also record the reviewer roster: who holds each review role named in the [standards](../engineering-standards.md#security-and-reviews) and the [risk register](../plan/risk-register.md), and which reviews a fresh model may perform instead of a human specialist. See [engineering standards](../engineering-standards.md) and [CI progression](../testing/strategy.md#ci-progression).

- [x] Capture conventions: `/audit`
- [x] Build it: `/develop standards and tooling`
  - [x] Dependency policy and fuzz harness: Build plan steps 1 and 2 (`AC-4`, `AC-15`, `AC-20`)
  - [x] Check entry point and CI files: steps 3 and 4 (`AC-1`, `AC-2`, `AC-5`, `AC-13`, `AC-14`), needs the spec 0001 ARM update on `main`
  - [x] Repository guards and document checks: steps 5 and 6 (`AC-6` to `AC-10`), after the two milestones above
  - [x] Proofs, inventory, documents and workstation evidence: steps 7 to 10 (`AC-11`, `AC-16` to `AC-19`), after the three milestones above
- [x] Verify it: `/check verify standards and tooling`
- [x] Test it: `/test standards and tooling`
- [ ] Review it (fresh model): `/check review standards and tooling`
- [ ] Document it: `/document standards and tooling`
- [ ] Verify the required checks on a pull request and link the result.

Spec [0002](../specs/0002-standards-and-tooling/index.md) · code in `deny.toml`, `crates/pool-config/fuzz/`, `scripts/`, `.github/`, `tools/repo-check/`.

Evidence: [verification record](../specs/0002-standards-and-tooling/verify.md). Open: the four remote items (`0002 AC-3`, `0002 AC-12`, the CI half of `0002 AC-5` and the remote half of `0002 AC-13`) have not run. They are Build plan step 11 and the last box.
