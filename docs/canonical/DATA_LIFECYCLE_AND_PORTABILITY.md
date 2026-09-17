# Data Lifecycle, Portability, and Privacy

## 1. Purpose

Kernux will accumulate sensitive local state: tasks, transcripts, browser profiles, screenshots, files, artifacts, indexes, approvals, audit events, remote-runtime metadata, and model/tool configuration. A local-first product is incomplete if it cannot explain where that state lives, how long it lives, how it moves, and how it is deleted.

This contract defines lifecycle semantics before product data becomes difficult to migrate or remove.

## 2. Data classes

Kernux should classify persisted data at least as:

1. **Canonical metadata** — projects, tasks, runs, ids, policy references, runtime records.
2. **Durable events/evidence** — approvals, capability events, verification results, lineage.
3. **Artifacts** — files, screenshots, terminal transcripts, reports, exported bundles.
4. **Derived indexes** — quick-open, full-text, semantic, symbol, projection caches.
5. **Browser state** — cookies, storage, auth profiles, downloads, recordings.
6. **Agent/model state** — messages appropriate for persistence, summaries, context bundles, usage metadata.
7. **Memory** — explicit scoped MemoryRecords.
8. **Secrets** — secret references plus provider-owned plaintext in approved stores.
9. **Diagnostics/telemetry** — logs, traces, crash/support bundles.
10. **Cloud/team replicas** — optional synchronized metadata/content under separate policy.

Each class receives explicit retention, export, backup, and deletion behavior.

## 3. Storage ownership

`kernuxd` owns authoritative local metadata locations and migrations. The renderer must not invent parallel hidden stores for privileged/product state.

Large artifacts live in the content-addressed store. Derived caches/indexes are rebuildable and must not become the only copy of user data.

Browser engines and third-party agents may keep provider-owned state outside Kernux. Kernux must surface that boundary rather than claiming deletion it cannot perform.

## 4. Retention policy

Retention is configurable by scope and data class.

Recommended defaults:

- active task/run metadata: durable until project/user deletion;
- verification/evidence: durable with task unless user policy says otherwise;
- temporary screenshots/traces: bounded retention unless pinned as evidence;
- terminal scrollback: bounded by byte/time policy, with explicit artifact promotion for durable evidence;
- browser downloads: project/download policy;
- browser authenticated profile: explicit persistence choice;
- derived indexes/caches: evictable/rebuildable;
- logs: bounded rotation;
- memory: scope/expiry aware;
- cloud replicas: follow organization/project retention plus local deletion semantics where contract permits.

No unbounded log/transcript/cache growth.

## 5. Artifact CAS lifecycle

The content-addressed artifact store needs reference-aware garbage collection.

Requirements:

- digests identify immutable blobs;
- logical artifact records reference blobs and carry metadata;
- active evidence/checkpoints/pinned exports protect referenced blobs;
- deletion removes logical references first;
- GC deletes only blobs proven unreachable under current retention policy;
- interrupted GC is recoverable/idempotent;
- quotas and largest-artifact inspection are user-visible;
- optional deduplication must not leak cross-project existence through unauthorized APIs.

## 6. Project export

A project should support a portable Kernux export format.

A bundle may contain, based on user selection:

- project manifest and schema version;
- task/run metadata;
- events/evidence;
- artifacts;
- policies without secret plaintext;
- runtime/integration references;
- context/memory records;
- provenance/notices;
- checksums/digests;
- redaction/exclusion manifest.

Exports must say what was excluded, especially credentials, provider-owned browser state, unavailable remote artifacts, or policy-managed organization data.

## 7. Import and migration

Project import must be treated as untrusted input.

Requirements:

- validate archive paths and sizes before extraction;
- verify manifest/schema/digests;
- reject path traversal/symlink abuse;
- preserve unknown compatible fields where possible;
- require explicit migration for breaking versions;
- never activate imported plugins, runtimes, credentials, or broad permissions automatically;
- imported memory retains provenance/trust rather than becoming trusted because it exists in a bundle.

## 8. Backup and restore

Kernux should support user-controlled backup of local canonical state without requiring Kernux cloud.

A backup strategy must cover:

- SQLite metadata/event state;
- artifact CAS references/blobs selected by policy;
- project-level configuration;
- explicit browser/profile state only when the user elects to include it and platform/provider rules permit;
- no ordinary plaintext export of OS-keychain secrets.

Restore must be tested against version migration and interrupted-write cases.

## 9. Delete semantics

Deletion is explicit by scope:

- delete run;
- delete task lineage;
- delete project from Kernux while leaving source folder untouched;
- delete project data including Kernux-owned artifacts/indexes/memory;
- delete browser profile/context;
- remove runtime/device enrollment;
- remove integration configuration/secret reference;
- delete user-local Kernux state during uninstall when explicitly chosen.

For each operation the UI/CLI must distinguish:

- unlink/reference removal;
- Kernux-owned data deletion;
- external/provider-owned data that Kernux cannot delete;
- recoverable/trash state vs irreversible erase.

Deletion should create an audit/tombstone record only when required for integrity/security and should not preserve the deleted sensitive payload itself.

## 10. Browser and authentication data

Authenticated browser state is highly sensitive.

Rules:

- profiles are named/scoped;
- default project isolation;
- persistence is explicit;
- cookies/storage are not ordinary task artifacts;
- screenshots/recordings can contain secrets and follow sensitive-artifact policy;
- browser profile deletion has a clear cleanup path;
- a task export does not silently include reusable session credentials.

## 11. Secret lifecycle

Secret plaintext remains in approved secret providers.

Kernux stores:

- opaque reference;
- provider id;
- scope/policy metadata;
- audit metadata;
- optional non-sensitive label.

Project export/backup includes references only unless a specialized encrypted secret-export workflow is explicitly designed later.

Revocation/deletion semantics must account for cached runtime tokens and remote devices.

## 12. Telemetry defaults

Local-first means product telemetry is not a hidden dependency.

Defaults should be privacy-conservative:

- product works with telemetry disabled;
- prompt/file/tool payload content is not exported by default;
- usage/diagnostic telemetry, if offered, is documented and configurable;
- identifiers are minimized;
- security/audit logs needed for local integrity stay local unless the user/org deliberately exports them;
- organization-managed deployments may impose declared policy, but the UI/admin docs must make that boundary visible.

## 13. Crash reports and support bundles

Support tooling should generate a reviewable bundle rather than uploading the entire local state.

A support bundle can include:

- app/daemon versions;
- OS/runtime diagnostics;
- recent structured error codes;
- redacted logs;
- protocol capability/version metadata;
- configuration schema minus secrets;
- optional selected run/event evidence.

The user can inspect the manifest before upload/export. Secret scanning/redaction runs before bundle creation and again before transmission where practical.

## 14. Encryption

Transport encryption is mandatory for remote links. At-rest strategy is layered:

- rely on OS account/disk protection as baseline for ordinary local metadata;
- use OS credential stores for secrets;
- support stronger encrypted project/export/backup modes where threat model/user need justifies them;
- document that SQLite/artifact files are not magically protected from a compromised same-user host unless an explicit encrypted mode is enabled.

Do not market `local` as equivalent to `encrypted`.

## 15. Optional cloud synchronization

Cloud/team sync is additive and explicit.

Before content leaves the local device Kernux should know:

- what object/data class is being synchronized;
- destination/service/organization;
- encryption/auth state;
- retention owner;
- revocation behavior;
- whether local-only fallback remains available.

A cloud outage must not corrupt local canonical state.

## 16. Schema and migration discipline

Persisted formats carry versions. Migrations are:

- deterministic;
- tested from every supported source version;
- crash-safe/recoverable;
- backed up or checkpointed where destructive;
- independent of a model call;
- validated before old data is discarded.

Unknown future data is not silently deleted during downgrade/rollback.

## 17. Portability principle

Kernux should avoid unnecessary lock-in to its own cloud or one model provider.

Users should be able to:

- keep source projects in ordinary folders/git repositories;
- export Kernux task/evidence/artifact history;
- move to another machine and restore local state;
- switch agents/models/tools without losing project history;
- inspect machine-readable evidence outside the UI.

## 18. Implementation mapping

- **P02** — authoritative local storage, CAS, retention metadata, migrations.
- **P03** — storage/privacy inspector and user controls.
- **P06** — browser-profile/download lifecycle.
- **P09** — export/import/checkpoint/evidence portability, GC and deletion integrity.
- **P10** — remote-runtime artifact transfer/cache lifecycle.
- **P11** — plugin/integration data-boundary declarations.
- **P14** — cloud/team replication lifecycle and organization retention.
- **P15** — installer/uninstaller migration, backup/restore qualification, privacy/support documentation.

SpecGrain refinement must produce explicit child units for backup, import/export, deletion, GC, telemetry/support bundle, and browser-state cleanup before release gates are claimed.

## 19. Acceptance gates

Before public release:

1. export/import round-trip preserves selected canonical project history and validates digests;
2. backup/restore works across at least one supported upgrade migration path;
3. deleting a project removes Kernux-owned indexed/artifact/memory state according to policy without deleting the source folder unless explicitly requested;
4. artifact GC cannot remove evidence-referenced blobs;
5. browser profile deletion removes Kernux-owned persisted state;
6. uninstall offers an explicit keep/remove local-data choice where platform packaging permits;
7. telemetry-disabled mode remains fully functional;
8. support bundle is inspectable and free of known secret fixtures;
9. cloud sync failure does not invalidate local canonical state.

## 20. Rule

**If Kernux can create data, it must define how that data is inspected, exported, retained, recovered, and deleted.**
