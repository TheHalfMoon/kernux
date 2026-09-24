# Evidence — KX-P02-EXIT (exit assessment, UNPROVEN)

Task: `KX-P02-EXIT — Prove the privileged-kernel exit gate.`
Grain: `SG-000030`, state `GRAIN`, risk `R3` (medium grain risk; slices qualify at R3).
Status: `EXIT_EVIDENCE_UNPROVEN`. No advancement claimed. P02 stays `10/10 PROVEN`,
P02 exit stays unproven, P03 stays unauthorized.
Ledgers (`specs/CURRENT.md`, `specs/tasks.md`) are intentionally untouched by this record.

## Registry reconciliation (10/10 PROVEN)

Each macro task below carries an immutable native SpecGrain record bound to its
exact implementation range. Task count is reconciled, never the proof itself;
each record was independently verified with zero issues before its task advanced.

| Task | Grain | Native record | Outcome |
| --- | --- | --- | --- |
| KX-P02-S01-T01 | SG-000018 | `sha256:c4a424bc65aed750668e3d58d7339a7c56af6a449799b9d296511a23e53520e6` | kernuxd lifecycle, private transport, graceful shutdown |
| KX-P02-S01-T02 | SG-000019 | `sha256:fb9c164cecf8a9c72b2fd47557161d0d17046656fa3ce08bfb2939e172066aa3` | launch nonce/peer identity session binding |
| KX-P02-S02-T01 | SG-000020 | `sha256:c4dc5233ff46ef728c3338182f48c1ff26cefb955b1048cf33097032b3327cb2` | device identity, key creation/rotation/recovery |
| KX-P02-S03-T01 | SG-000021 | `sha256:e7136d88dd6ec7444054fb0a21103630c296975f0a68ad8f35c996802f9229ef` | SQLite store, WAL/migration, corruption/recovery fixtures |
| KX-P02-S04-T01 | SG-000022 | `sha256:6c358678f4b2225543637212a36cda613d9a914f4e4e93fc2cebf7813ec2e9cb` | SHA-256 artifact CAS, bounded streaming, retention |
| KX-P02-S05-T01 | SG-000024 | `sha256:f6340663290f6a609777bc1c2d10269f85e5107aa5aa4f385a06ab67b41b452e` | policy evaluator, bounded grants/expiry/delegation |
| KX-P02-S05-T02 | SG-000025 | `sha256:e0a77a108922a591f6934fb1fec1fde70cdfabae93f803449223d3ce6d0f91fa` | Safe/Standard/Developer/Autonomous/Custom profiles, adversarial corpus |
| KX-P02-S06-T01 | SG-000027 | `sha256:30054f35cb6aab343d1e166741d3e08c57a67e034644b0637000eafc2de8d912` | secret-provider abstraction, OS credential-store adapters |
| KX-P02-S06-T02 | SG-000028 | `sha256:0bb0dc7d5fe578f194b2cd9bfbca61380d2add6e49194a6323d4feb816de17ba` | redaction-safe audit events, correspondence, leak corpus |
| KX-P02-S07-T01 | SG-000029 | `sha256:2273a0e1264a35a9300e715e17f102c42523d7dd373d539c9a427fb4b7c02e8b` | egress vocabulary, grant-constraining evaluator, fail-closed corpus |

## Criterion 1 — privilege bypass / adversarial bounds: HOLD

- Permission-profile compiler ceilings plus the S05-T02 adversarial corpus bound
  authority narrowing (SG-000025 proof, 14/14 acceptance).
- Secret-broker admission correspondence plus the 14-entry leak corpus over all
  events and surfaces (SG-000028 proof, 7/7 acceptance, 60/60 secret suites).
- Egress evaluation narrows exact Grant bindings only; the relabeling-exhaustion
  proof shows no class/sensitivity relabeling of a denied request can yield
  Eligible (SG-000029 proof, 8/8 acceptance).
- No layer mints authority; every bypass attempt in the corpora is denied.

## Criterion 2 — secrets outside ordinary DB/logs: HOLD

- Secret provider abstraction with OS credential-store adapters and
  handle-based use; no plaintext persistence in the broker path (SG-000027).
- Redaction-safe audit projection with metadata-only Debug/Display, stable
  denial codes, and frozen KRP envelope binding (SG-000028).
- Egress decisions carry no plaintext; exact-rendering locks guard the surface
  (SG-000029).

## Criterion 3 — restart/migration recovery: HOLD

- SQLite metadata store with schema versioning, WAL, migration, and
  corruption/recovery fixtures (SG-000021, KX-P02-S03-T01 PROVEN).
- SHA-256 artifact CAS with digest verification and retention metadata
  (SG-000022, KX-P02-S04-T01 PROVEN).

## Criterion 4 — fail-closed egress before network-capable phases: HOLD

- Closed 8-class vocabulary with exact parsing; unknown classes rejected
  (SG-000029 PR-A).
- Grant-constraining evaluator: missing/wrong Grant, class, destination,
  account, runtime, operation, or incomplete sensitivity always DENY
  (SG-000029 PR-B).
- Nine-test adversarial corpus with no-cloud-fallback proof, secret/audit
  compatibility, and stable redaction-safe denials (SG-000029 PR-C).
- Native record `sha256:2273a0e1264a35a9300e715e17f102c42523d7dd373d539c9a427fb4b7c02e8b`
  (8/8 acceptance, 22/22 evidence, zero issues).

## Packet identity (frozen)

- WorkPacket digest: `sha256:32ed21a014b8015084c1ec285e7313bb4b236f145419057dcad7d0fd327f94ad`
- Context-plan digest: `sha256:ff4f9ab79d82610e06369f19e6844c73c10af777ae3b149ce80a6adfe85f92e4`
- Spec revision: `sha256:9c3e61d624f19bc00ea25da06441c66e6b9fc1b3125801e7fb4011bc847b9934`
- SpecGrain pin: `TheHalfMoon/SpecGrain` commit `5de7d6499bb0a9e3a191fc0934399cf099d1980a`
  (archive SHA-256 `da6a8c006c3a0154e76e2bf08bcae58bb86e04f06dd7a03115275b466489eef9`)
- WorkPacket file SHA-256: `78453c4546f7209b163ab45470a306ca32c5dc7b0b3c1aa2cc7e0dff4ae2fb37`
  (7961 bytes; context sources 5707 bytes, 15 sources, 9515 tokens).
- `prove SG-000030`: `verified=false`, `record_count=0` (correct: native
  verification is a later dependent step, no record hand-authored).
- `prove SG-000029`: `verified=true`, `record_count=1` (prior proof preserved).

## Negative evidence (preserved, not waived)

- This assessment adds no product code, so it introduces no new defects; all
  chain negatives from the ten tasks (including the SG-000029 PR-C fixture fix
  and the CI-side Jev key-unavailable probe) remain preserved in their task
  records and are not re-litigated here.
- Local Windows limitations (`link.exe` absent; local pnpm 11.x vs required
  12.x; `.cache/specgrain` orphaned ACL; PowerShell CRLF capture) persist as
  environmental facts; affected gates run on authorized runners, never waived.

## Outstanding before P02 EXIT PROVEN (not claimed here)

1. Native `verify_execution` with the pinned SpecGrain verifier binding the
   WorkPacket to this observed evidence (7/7 acceptance, 18/18 required
   evidence, zero issues), producing the immutable record.
2. Separate protected closeout PR (`.specgrain/evidence/SG-000030/`, this
   record finalized, `specs/CURRENT.md`, `specs/tasks.md`) with its own
   exact-head CI + R3 + Jev + OpenCodeReview + platform + semantic + zero-thread
   qualification, lawful merge, and post-merge CI. Only then may `P02 EXIT`
   transition to PROVEN and P03 entry be authorized.

Status: `EXIT_EVIDENCE_UNPROVEN`. P02 stays `10/10 PROVEN`. P02 exit stays unproven.
