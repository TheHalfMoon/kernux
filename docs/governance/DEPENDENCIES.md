# Dependency governance

Kernux admits external package dependencies through a repository-owned, fail-closed gate.

## Authority

Direct dependency approvals live in `third_party/dependencies/approved.json`.

The branch-protected `Provenance` GitHub Actions job executes the dependency conformance suite and repository admission check, alongside existing donor provenance, third-party notice, and project-license validation.

## Admission rules

- every direct external npm or Cargo dependency requires exactly one approval;
- npm versions are exact semver values;
- Cargo registry dependencies use exact `=x.y.z` requirements;
- Cargo git dependencies pin a full commit revision;
- internal Cargo path/workspace dependencies are not external approvals;
- manifest source, exact version, dependency scope, purpose, and observed license posture must match the approval;
- stale, duplicate, malformed, unsupported, or unapproved entries fail closed;
- when external Cargo dependencies exist, `cargo metadata --locked` must resolve the admitted direct package/version and reported license;
- licenses remain bounded by `docs/canonical/LICENSE_POLICY.md`; the dependency registry cannot broaden project license policy.

This gate is package-admission evidence. It is not vulnerability scanning, SBOM generation, signature verification, or release qualification.

## Diffcipline relationship

Diffcipline v1 returns REVIEW for dependency-manifest and lockfile changes when configured that way, and its GitHub Action preserves that exit status as a failed job. There is no v1 approval hook that can turn a reviewed dependency change into a deterministic PASS.

Kernux therefore keeps dependency judgment in the separately required Provenance gate and configures those two Diffcipline observations as ALLOW. This does not disable Diffcipline verification: exact diff limits, untracked-file rejection, scope rules, risk profile, and `pnpm check` remain active.

## Recovery

If the dependency-admission validator or required Provenance enforcement is ever removed or weakened, restore Diffcipline dependency-manifest and lockfile decisions to REVIEW before that change lands.
