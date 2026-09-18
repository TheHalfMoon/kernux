# Third-party notice inventory

This directory preserves license evidence for third-party material that Kernux may study, prepare, or import under canonical provenance governance.

The root Apache-2.0 license applies to Kernux-owned code and does not relicense the third-party evidence or material represented here. Preserved upstream license text remains authoritative for that upstream material.

`inventory.json` is the versioned machine-readable index. Each entry binds an exact source identity to:

- an SPDX license expression;
- the upstream license path;
- a repository-local byte-preserved license snapshot;
- the SHA-256 digest of that snapshot.

The inventory does not mean source code has been imported and does not grant additional rights.

For Git sources, pin a full upstream commit. For non-public or artifact sources, record an exact artifact identity and content digest rather than inventing a public repository revision.

Run:

```bash
python3 tools/provenance/validate_notices.py check
```

The validator fails closed on malformed inventory state, ambiguous source identities, unsafe/symlinked snapshot paths, missing license files, digest mismatches, and provenance records without matching notice evidence.

See `docs/canonical/LICENSE_POLICY.md` for the repository-level project license and third-party compatibility policy.
