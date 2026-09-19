# Dependency approvals

This directory records direct external package dependencies admitted by Kernux repository governance.

`approved.json` is not a package lockfile and does not replace npm/Cargo lock resolution. It records the exact direct dependency identity, bounded purpose, scope, and observed license posture that the required dependency validator is allowed to admit.

Rules:

- npm direct versions are exact semver values, not ranges or tags;
- Cargo registry direct versions must use an exact `=x.y.z` requirement in Cargo manifests;
- internal Cargo `path` dependencies do not need external approval;
- one external package has one approval matching its observed direct use;
- stale, duplicate, malformed, or unapproved entries fail closed;
- the registry does not establish vulnerability, SBOM, signature, or release qualification;
- licenses outside the current canonical `LICENSE_POLICY.md` posture require a separately governed policy exception before admission.

Changes to this registry are governed repository changes and are independently checked against the actual manifests.
