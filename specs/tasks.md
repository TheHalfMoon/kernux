# Kernux Task Registry

## How to use this file

This is the canonical **macro-task registry**. It is intentionally more detailed than the master-plan phase list, but many entries will still need recursive SpecGrain refinement before implementation.

States:

- `PLANNED` — known work, not yet dependency-eligible/proven.
- `NEXT` — next macro-task after the canonical plan merges.
- `READY` — dependencies proven and bounded execution package ready.
- `IN_PROGRESS` — active execution.
- `IMPLEMENTED_UNPROVEN` — code exists but required evidence is incomplete.
- `PROVEN` — exact required evidence established.
- `BLOCKED_EXTERNAL` — only an external dependency blocks proof.
- `DEFERRED` — intentionally outside current frontier.

Risk is the minimum Diffcipline-style profile; refinement may raise it.

---

## P00 — Repository and governance foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P00-S01-T01 | PROVEN | R1 | Establish pnpm + Cargo monorepo skeleton, canonical folders, root commands, formatting/editor baseline. | plan merge |
| KX-P00-S01-T02 | PROVEN | R1 | Add baseline CI for Rust/TS format, lint, static checks, unit placeholders and cache discipline. | T01 |
| KX-P00-S02-T01 | PROVEN | R1 | Initialize repository-local SpecGrain and document deterministic authoring/refinement workflow. | T01 |
| KX-P00-S03-T01 | PROVEN | R1 | Initialize Diffcipline policy with R0-R3 verification profiles and CI gate. | T02 |
| KX-P00-S04-T01 | PROVEN | R2 | Define machine-readable donor provenance/import manifest schema and validation command. | T01 |
| KX-P00-S04-T02 | PROVEN | R1 | Add third-party notices/license inventory structure and donor import checklist. | provenance schema |
| KX-P00-S01-T03 | PROVEN | R0 | Add CONTRIBUTING, SECURITY, SUPPORT, CODE_OF_CONDUCT/issue/PR templates and repository governance docs. | T01 |
| KX-P00-S01-T04 | PROVEN | R1 | Decide/apply project license and validate compatibility strategy for planned donor imports/dependencies. | notices structure |

**P00 gate: PROVEN.** Fresh clone reproduces checks; SpecGrain/Diffcipline are usable; protected `main` requires JavaScript / TypeScript, Rust, SpecGrain, Diffcipline R1, and Provenance; provenance is enforceable before donor imports.

---

## P01 — Core contracts and protocol spine

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P01-S01-T01 | PROVEN | R2 | Define canonical IDs/revisions for Project, Task, WorkUnit, Run, AgentSession, Runtime, Artifact, Evidence, Event. | P00 |
| KX-P01-S02-T01 | PROVEN | R3 | Define v1 capability taxonomy, resource URI model, consequence classes, constraints, Grant semantics. | identities |
| KX-P01-S03-T01 | PROVEN | R3 | Define runtime capability negotiation, contact/execution state, operation IDs, cancellation and structured errors. | identities + capabilities |
| KX-P01-S04-T01 | PROVEN | R2 | Define event envelope, lineage, artifact references, evidence binding and redaction metadata. | identities |
| KX-P01-S05-T01 | PROVEN | R2 | Choose one schema source and generate Rust + TS contracts with round-trip/conformance fixtures. | prior P01 contracts |
| KX-P01-S05-T02 | PROVEN | R2 | Define compatibility/versioning rules and unknown/additive-field behavior. | generated contracts |
| KX-P01-S05-T03 | PROVEN | R2 | Build canonical protocol fixture corpus including disconnect, duplicate operation, malformed payload and version mismatch. | generated contracts |

**P01 gate: PROVEN.** Cross-language contracts executable; provider names absent from core; disconnect cannot imply exit; grants structurally bounded. Native SG-000017 verification, proof-bearing protected closeout, ordinary merge, and five-job post-merge qualification are all PROVEN.

---

## P02 — Privileged kernel (`kernuxd`)

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P02-S01-T01 | PROVEN | R3 | Create `kernuxd` lifecycle, private UDS/named-pipe transport, health/version and graceful shutdown. | P01 |
| KX-P02-S01-T02 | PROVEN | R3 | Bind local client sessions with launch nonce/peer identity and reject unauthorized local callers. | daemon transport |
| KX-P02-S02-T01 | PROVEN | R3 | Implement installation/device identity and key creation/rotation/recovery abstraction. | daemon transport |
| KX-P02-S03-T01 | PROVEN | R3 | Implement SQLite metadata store, schema versioning, WAL/migration and corruption/recovery fixtures. | P01 events/IDs |
| KX-P02-S04-T01 | PROVEN | R2 | Implement SHA-256 artifact CAS with bounded streaming, digest verification and retention metadata. | metadata store |
| KX-P02-S05-T01 | PROVEN | R3 | Implement policy evaluator and bounded grants/expiry/delegation/deny reasons. | capabilities + store |
| KX-P02-S05-T02 | PROVEN | R3 | Implement user-friendly Safe/Standard/Developer/Autonomous/Custom profiles compiling to granular grants. | policy engine |
| KX-P02-S06-T01 | PROVEN | R3 | Implement secret-provider abstraction and first OS credential-store adapters with handle-based use. | identity + policy |
| KX-P02-S06-T02 | PROVEN | R3 | Add redaction-safe security audit events and secret-leak tests. | secret broker + events |

**KX-P02-S01-T01 closeout: PROVEN.** Native SG-000018 verification, proof-bearing PR #91 exact-head qualification/review, ordinary merge `f2ea966b398e445e57af94ee7b25bfbe1af21f2b`, and six-job post-merge run `35429947987` are all PROVEN. Only KX-P02-S01-T02 is NEXT; later P02 tasks remain PLANNED.

**KX-P02-S01-T02 closeout: PROVEN.** Native SG-000019 verification, proof-bearing PR #97 exact-head qualification/review, ordinary merge `78631b3095ce83ec39985f54c0ebc8e769ca4203`, and six-job post-merge run `35435786206` are all PROVEN. Only KX-P02-S02-T01 is NEXT; all other later P02 tasks remain PLANNED.

**KX-P02-S02-T01 closeout: PROVEN.** Native SG-000020 verification, proof-bearing PR #105 exact-head qualification/review, ordinary merge `f0e457d55021e59799f898a51e634a1d8d284cdc`, and six-job post-merge run `35443098538` are all PROVEN. Only KX-P02-S03-T01 is NEXT; all later P02 tasks remain PLANNED.

**KX-P02-S03-T01 closeout: PROVEN.** Native SG-000021 verification, proof-bearing retry PR #119 exact-head qualification/review, ordinary merge `ec252919e243c2e815dd878eb2d9bd17077f11d8`, and six-job post-merge run `35454187226` are all PROVEN. Only KX-P02-S04-T01 is NEXT; all later P02 tasks remain PLANNED.

**KX-P02-S04-T01 closeout: PROVEN.** Native SG-000022 verification record `sha256:6c358678f4b2225543637212a36cda613d9a914f4e4e93fc2cebf7813ec2e9cb`, proof-bearing PR #125 exact-head qualification/review, ordinary merge `415f7fe41b4eff85d61137132256ef97195a4fbf`, and six-job post-merge run `35458837422` are all PROVEN. Only KX-P02-S05-T01 is NEXT; KX-P02-S05-T02 and all S06 tasks remain PLANNED.

**KX-P02-S05-T01 closeout: PROVEN.** Primary SG-000023 negative evidence remains preserved; corrective SG-000024 native verification record `sha256:f6340663290f6a609777bc1c2d10269f85e5107aa5aa4f385a06ab67b41b452e` verified the exact five-path reconciliation with acceptance 10/10 and required evidence 14/14. Proof-bearing PR #152 exact head `a8efc29352979ea6fd4651c554adda746c9e9cbb` passed native macOS qualification `35577145490`, exact-head CI `35577135982` 6/6, exact-head review `5264407121` with zero threads, ordinary merge `d93b54de0013e47809336db60f857d5435d8c6a4`, and six-job post-merge run `35579619127`. Only KX-P02-S05-T02 is NEXT; all S06 tasks remain PLANNED and P02 phase exit remains unproven.

**KX-P02-S05-T02 closeout: PROVEN.** SG-000025 native verification record `sha256:e0a77a108922a591f6934fb1fec1fde70cdfabae93f803449223d3ce6d0f91fa` verified the exact four-path implementation range at `git:d6e42a3ecccf9a23f5acb11afd97cf34a9caa6c9` with acceptance 14/14 and required evidence 19/19. The bounded chain was PR #155 packet persistence (`9177faa18a415099549bdb63cc78faca77fd2dc3`), PR #161 profile vocabulary and preset ceilings (`7f71124328f846383b76e5823ec0e4ab8c931e8c`), PR #162 profile compiler (`8b1e0b35195a17d0ecde716dc70ed222dcd3d180`), and PR #163 adversarial corpus (`d6e42a3ecccf9a23f5acb11afd97cf34a9caa6c9`); each slice passed full `pnpm check`, focused policy tests, Diffcipline R1 and R3, Alibaba OpenCodeReview accounting, linux and native macOS qualification, exact-head six-job CI, exact-head review with zero threads, ordinary merge, and six-job post-merge qualification. Protected closeout evidence PR #164 exact head `ad1ecac9de882ed8a49b4c11e63119346c423517` passed exact-head CI `35627524278` 6/6, linux and native macOS qualification `35628011667`, exact-head review `5269389086` with zero threads, ordinary merge `d480d5abf1ae5525bf38f2a2c15dda616d9640d1`, and six-job post-merge run `35628296835`. Two failed candidate heads, `ac7f1e1` and `1086bfd`, remain preserved as negative evidence. Only KX-P02-S06-T01 is NEXT; KX-P02-S06-T02 and KX-P02-S07-T01 remain PLANNED and P02 phase exit remains unproven.

**KX-P02-S06-T01 closeout: PROVEN.** SG-000027 native verification record `sha256:30054f35cb6aab343d1e166741d3e08c57a67e034644b0637000eafc2de8d912` verified the exact four-path corrective range at `git:73b7b60e5d0a7a8e952e20e84915dfc98c2d97fa` with acceptance 9/9 and required evidence 18/18. The bounded chain was PR #182 refinement (`09737241f19541fcb859f4c530ee572a0f62fac0`), PR #183 packet persistence (`937fd04fa900639ede99641a772354b3643c271f`), and PR #184 reconciliation (`73b7b60e5d0a7a8e952e20e84915dfc98c2d97fa`); each slice passed exact-head six-job CI, probe linux and native macOS qualification, Diffcipline R1 and R3, Alibaba OpenCodeReview accounting, exact-head review with zero threads, ordinary merge, and post-merge qualification. Protected closeout evidence PR #189 exact head `5e62fc217ac1ebe48d450818b201c8a816047932` passed exact-head CI `35824914642` 6/6, linux and native macOS probe `35825306218`, exact-head review with zero threads, ordinary merge `573a32b0730d053df3afb60b726491ad8e996442`, and post-merge run `35825508593`. The failed probe `35825001476` remains preserved as negative evidence. Only KX-P02-S06-T02 is NEXT; KX-P02-S07-T01 remains PLANNED and P02 phase exit remains unproven.

**KX-P02-S06-T02 closeout: PROVEN.** SG-000028 native verification record `sha256:0bb0dc7d5fe578f194b2cd9bfbca61380d2add6e49194a6323d4feb816de17ba` verified the exact two-path implementation range at `git:92f3929e26a18de25f9f3dbb2db8e68eec984bb9` with acceptance 7/7 and required evidence 21/21. The bounded chain was refinement PR #191 (`0d135be7b05926768ee97033e1a1d3158e2808ec`, exact-head CI `35834606442`), packet persistence PR #192 (`5de4bd1e886705dabc2449377510fd3e95e265ba`, exact-head CI `35836803062`, probe `35837127930`, merge `fb63a4f`, post-merge CI `35837385920`), PR-A #193 audit core (`954f6250aba98892eae76ddba8f868dca548c810`, CI `35884729510`, probe `35889111829`, merge `f4729ad`, post-merge `35889733273`), and PR-B #194 correspondence and leak corpus (`0826bc059098758a63d1fa4344e8b15063b29c16`, CI `35890939606`, probe `35891315591`, merge `92f3929`, post-merge `35891797823`); each slice passed exact-head six-job CI, probe linux and native macOS qualification, Diffcipline R1 and R3, genuine Jev review, Alibaba OpenCodeReview accounting, semantic review, and zero blocking threads before ordinary merge. Implementation evidence PR #195 (`1734f7712d2fac33ac016cc776296a7ae84dc7fe`, CI `35892325865`, probe `35892890158`, merge `afcf6fa`) and closeout PR #196 (`23dfe587080aa89e6eb8b4c89b3f1e17130fbfe8`, CI `35894087220`, probe `35894159508`, merge `db71932`, post-merge `35894496333`) completed the chain with full gates. Preserved failures (clippy type_complexity CI `35890302357` and probe `35890482058`, push-protection xoxb trip, stale probe pin `35891002014`, macOS mapfile probe `35892562976`) remain negative evidence with structural fixes and no gate weakened. Only KX-P02-S07-T01 is NEXT; P02 phase exit remains unproven.

**KX-P02-S07-T01 closeout: PROVEN.** SG-000029 native verification record `sha256:2273a0e1264a35a9300e715e17f102c42523d7dd373d539c9a427fb4b7c02e8b` verified the exact two-path implementation range at `git:8d9cb45338b7baeb93e7212419524f48aaf26906` with acceptance 8/8 and required evidence 22/22. The bounded chain was PR-A #201 egress primitives (`7e3f87a`, exact-head CI `35915267257`, probe `35917714275`, merge `a9f697c`, post-merge `35918418274`), PR-B #203 evaluator (`27736f7`, CI `35920766134`, probe `35920791174`, merge `912bbf7`, post-merge `35971021047`), PR-C #204 corpus (`789331a`, CI `35971871519`, probe `35972003724`, merge `8d9cb45`, post-merge `35972325065`); each slice passed exact-head six-job CI, probe linux and native macOS qualification, Diffcipline R1 and R3, genuine Jev review, Alibaba OpenCodeReview accounting, semantic review, and zero blocking threads before ordinary merge. Implementation evidence PR #205 (`252b537`, CI `35972678340`, probe `35972743726`, merge `ed2d381`) and closeout PR #206 (`74a670e`, CI `35973637861`, probe `35973692486`, merge `cc426e9`, post-merge `35973916612`) completed the chain with full gates. Preserved failures (PR-C 127-byte fixture CI `35971586095` and probe `35971670743`, CI-side Jev key-unavailable probe `35921203543`) remain negative evidence with structural fixes and no gate weakened. KX-P02-S07-T01 is PROVEN; P02 is 10/10; P02 phase exit remains unproven; only P02 EXIT is NEXT.

| KX-P02-S07-T01 | PROVEN | R3 | Define and implement kernel egress-class/destination policy primitives for NONE, DIRECT_DESTINATION, CONNECTED_ACCOUNT, EXTERNAL_MODEL, EXTERNAL_TOOL, REMOTE_RUNTIME, UPDATE and TELEMETRY; unknown sensitive egress fails closed. | policy + secret broker |

**P02 EXIT: PROVEN.** SG-000030 native verification record `sha256:6c7134350d40134d576788fee7a7a06f940cb3757664c6e4d8c8117dd5c2b8cb` verified the exit assessment with acceptance 7/7 and required evidence 18/18. The bounded chain was refinement PR #208 (`3ae67a1`, exact-head CI `35975498403`, probe `35975563684`, merge `48ffffc`, post-merge `35975821048`), packet persistence PR #209 (`bb05be1`, CI `35976237526`, probe `35976296891`, merge `f6b4d53`, post-merge `35976559786`), exit evidence PR #210 (`0168a6a`, CI `35976792750`, probe `35976843452`, merge `37e6c08`, post-merge `35977057808`), and closeout PR #211 (`28ba8d6`, CI `35977524900`, probe `35977578871`, merge `b1fa75b`, post-merge `35977801272`); each slice passed exact-head six-job CI, probe linux and native macOS qualification, Diffcipline R1 and R3, genuine Jev review, Alibaba OpenCodeReview accounting, semantic review, and zero blocking threads before ordinary merge. Preserved failures: none new (exit adds no product code); chain negatives preserved in task records. P02 EXIT is PROVEN; only KX-P03-S01-T01 is NEXT.

**P02 gate:** privilege bypass/adversarial tests pass; secrets stay outside ordinary DB/logs; restart/migration recovery proven; egress class/destination policy can fail closed before later network-capable phases.

---

## P03 — Desktop workspace foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P03-S01-T01 | PLANNED | R2 | Create Electron/React desktop shell with unprivileged renderer and typed preload bridge. | P02 IPC |
| KX-P03-S01-T02 | PLANNED | R2 | Implement daemon connection/reconnect projection and honest degraded states. | desktop shell |
| KX-P03-S02-T01 | PLANNED | R1 | Establish Kernux design tokens, primitives, accessibility and design-system lint rules. | desktop shell |
| KX-P03-S03-T01 | PLANNED | R1 | Implement tabs/splits/docking/layout persistence and keyboard navigation. | design system |
| KX-P03-S04-T01 | PLANNED | R1 | Implement project rail, command bar, onboarding/empty states and run strip. | pane framework |
| KX-P03-S05-T01 | PLANNED | R1 | Implement basic timeline projection and typed artifact preview shell. | event store + desktop |
| KX-P03-S05-T02 | PLANNED | R2 | Add hidden/headless desktop UI qualification that never steals user focus. | workspace |
| KX-P03-S06-T01 | PLANNED | R2 | Implement universal quick-open/search plus inspectable ContextBundle/source view over local project/task/artifact sources. | context contracts + desktop |
| KX-P03-S06-T02 | PLANNED | R2 | Implement memory inspector with review/edit/pin/expire/delete controls and provenance visibility. | memory contract + desktop |
| KX-P03-S06-T03 | PLANNED | R2 | Implement useful local retrieval without external embeddings: exact lookup, lexical/FTS-BM25, temporal/provenance filters, rebuildable local indexes, cross-project denial and deletion/index invalidation. | context/memory contracts + local store |
| KX-P03-S07-T01 | PLANNED | R2 | Implement CLI bootstrap/doctor and authenticated headless local client against the same daemon/task contracts. | P02 IPC + CLI envelopes |
| KX-P03-S08-T01 | PLANNED | R2 | Implement Design Mode entry/capture inspector for structured browser/native visual context artifacts. | desktop + context/artifacts |

| KX-P03-S09-T01 | PLANNED | R2 | Implement Privacy Inspector and privacy-mode UX over real kernel/provider state: accounts, external AI, remote runtimes, recent egress, local data locations, telemetry/update controls and deletion/export entry points. | P02 egress + desktop |

**P03 gate:** renderer has no ambient host authority; layout/project state survives restart; quick-open/context/memory and CLI bootstrap are usable; Design Mode capture shell exists; Privacy Inspector reflects real kernel/provider state; accessibility and hidden E2E baseline pass.

---

## P04 — Local computer runtime

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P04-S01-T01 | PLANNED | R3 | Implement granted-root filesystem read/list/metadata primitives with canonical path resolution. | P02 policy |
| KX-P04-S01-T02 | PLANNED | R3 | Implement write/move/create/search with symlink/junction/reparse defenses and atomic-write patterns. | filesystem read |
| KX-P04-S02-T01 | PLANNED | R3 | Implement argv-first process start, cwd/env policy, lifecycle, bounded streaming and cancellation. | P02 policy |
| KX-P04-S03-T01 | PLANNED | R3 | Implement cross-platform PTY session lifecycle, streaming, writes, stable session IDs and scrollback artifacts. | process layer |
| KX-P04-S04-T01 | PLANNED | R2 | Implement process enumerate/inspect/terminate adapters on macOS/Windows/Linux without unsafe shell shortcuts. | process layer |
| KX-P04-S05-T01 | PLANNED | R2 | Import/adapt bounded Desktop Commander document/data primitives behind Kernux contracts with provenance. | provenance + files |
| KX-P04-S06-T01 | PLANNED | R3 | Define/implement structured host app/window/clipboard/notification/accessibility capability subset. | policy + platform adapters |
| KX-P04-S06-T02 | PLANNED | R3 | Build host-boundary adversarial suite: path escape, argv injection, destructive action, output exhaustion. | local runtime |

| KX-P04-S07-T01 | PLANNED | R3 | Define and qualify protection-at-rest for Kernux-managed sensitive local content, key ownership, backup/restore/migration and no-plaintext-key invariants. | P02 secrets + metadata/artifacts |
| KX-P04-S07-T02 | PLANNED | R3 | Prove a no-network local files/process/PTY execution fixture and truthful host-mode network limitations. | local runtime + P02 egress |

**P04 gate:** project-scoped host work proven on all desktop OS families; long-running sessions survive renderer restart; donor import characterized; Kernux-managed sensitive local content has a qualified protection-at-rest strategy and local execution can be proven without non-loopback network.

---

## P05 — Agent integration foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P05-S01-T01 | PLANNED | R2 | Implement stable ACP v1 client adapter and capability/session mapping. | P01 + P03 |
| KX-P05-S02-T01 | PLANNED | R2 | Implement generic supervised CLI/PTTY agent adapter with raw transcript capture. | P04 PTY |
| KX-P05-S03-T01 | PLANNED | R2 | Qualify first external coding agent end-to-end through ACP or structured adapter. | ACP adapter |
| KX-P05-S03-T02 | PLANNED | R2 | Qualify second independent agent and generic CLI fallback. | ACP + PTY |
| KX-P05-S04-T01 | PLANNED | R2 | Implement provider-native auth/account/usage metadata boundary without credential capture. | agent adapters |
| KX-P05-S05-T01 | PLANNED | R1 | Implement Agent pane with structured status/tool/file/permission events and interrupt/resume controls. | desktop + adapters |
| KX-P05-S05-T02 | PLANNED | R2 | Add agent capability/version matrix and unsupported-feature tests. | two agents |
| KX-P05-S06-T01 | PLANNED | R2 | Implement provider-neutral ModelProvider contract with negotiated modalities/context/tool features, local endpoints, BYOK auth, usage/cost metadata and failure semantics. | P01 + P02 secrets |
| KX-P05-S06-T02 | PLANNED | R3 | Implement Kernux Native Agent loop that plans/acts only through typed capability requests and kernel policy. | ModelProvider + P04/P06 tool surfaces |
| KX-P05-S06-T03 | PLANNED | R2 | Integrate ContextBundle assembly, bounded tool schemas, artifact outputs and observable agent state into the native agent path. | native agent + context |

| KX-P05-S07-T01 | PLANNED | R2 | Define DecisionProvider/DecisionRequest/DecisionResult contracts with provider/model revision, calibration, data-boundary, abstain/OOD, resource and non-authority semantics. | ModelProvider + context |
| KX-P05-S07-T02 | PLANNED | R2 | Qualify at least one local DecisionProvider through Kernux-owned calibration/OOD/multilingual/latency fixtures while preserving provider-optional correctness. | DecisionProvider |
| KX-P05-S08-T01 | PLANNED | R3 | Qualify at least one local generative-model adapter for a bounded native-agent task with offline-after-install proof, cancellation/OOM handling and deterministic fake-provider CI. | ModelProvider + native agent |
| KX-P05-S08-T02 | PLANNED | R3 | Enforce no-silent-cloud-fallback and bind every external/local model invocation to a minimized/redacted ContextBundle digest plus explicit privacy mode/egress evidence. | local/external providers + P02 egress |

**P05 gate:** two external agents and one Kernux-native model-backed path can perform bounded tasks; provider differences are negotiated honestly; at least one qualified local generative path and one local DecisionProvider exist for applicable claims; no local failure silently widens to cloud; all agent capability use remains policy-bound.

---

## P06 — Browser and web runtime

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P06-S01-T01 | PLANNED | R3 | Implement isolated Chromium context/profile lifecycle, persistence policy, download/upload roots. | P02 + desktop |
| KX-P06-S02-T01 | PLANNED | R2 | Implement deterministic CDP/Playwright navigate/observe/act/wait/extract/trace contract. | browser contexts |
| KX-P06-S03-T01 | PLANNED | R3 | Implement negotiated WebMCP discovery/invocation with schema/provenance/policy validation. | browser contract |
| KX-P06-S04-T01 | PLANNED | R2 | Import/adapt AgentQL/TinyFish semantic targeting/extraction behind Kernux browser provider API. | provenance + deterministic browser |
| KX-P06-S05-T01 | PLANNED | R3 | Implement screenshot/vision + input fallback and human takeover state. | browser/computer abstractions |
| KX-P06-S06-T01 | PLANNED | R2 | Capture browser evidence: origin, action method, observation, screenshot/artifact and lineage. | all browser paths |
| KX-P06-S06-T02 | PLANNED | R3 | Build malicious page/WebMCP/tool-output injection and cross-project auth leakage suite. | browser runtime |
| KX-P06-S06-T03 | PLANNED | R2 | Prove research/extraction golden journey with inspectable source provenance. | browser evidence |
| KX-P06-S07-T01 | PLANNED | R2 | Define/implement provider-neutral web Search/Fetch with freshness/cache/source metadata, bounded outputs and local/self-hosted/BYOK or explicitly managed provider paths. | context + browser/web contracts |
| KX-P06-S07-T02 | PLANNED | R2 | Implement bounded crawl/site traversal and change-detection primitives that preserve source lineage and can feed later monitor/condition-watch tasks. | Search/Fetch + evidence |

| KX-P06-S08-T01 | PLANNED | R3 | Enforce browser privacy boundary: local browser execution, DIRECT_DESTINATION distinct from EXTERNAL_MODEL, redirect/DNS/private-network re-evaluation, profile isolation and explicit download/upload scope. | browser contexts + P02 egress |

**P06 gate:** structured-to-visual hierarchy works; search/fetch/browser paths share inspectable provenance; auth contexts isolated; page content cannot self-authorize elevated action; DIRECT_DESTINATION web access remains distinct from EXTERNAL_MODEL processing; a useful web research path does not require founder-funded metered credentials.

---

## P07 — Task engine and agent fleet

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P07-S01-T01 | PLANNED | R2 | Implement Task/WorkUnit plan revisions, dependencies, acceptance/evidence/capability/runtime constraints. | P01/P02 |
| KX-P07-S02-T01 | PLANNED | R3 | Implement scheduler, dependency eligibility, cancellation and safe retry/idempotency enforcement. | task model + runtime |
| KX-P07-S03-T01 | PLANNED | R2 | Implement Fleet fan-out with isolated candidates and bounded concurrency. | scheduler + agents |
| KX-P07-S04-T01 | PLANNED | R2 | Implement verification/evidence comparison matrix with cost/latency metadata. | fleet + evidence |
| KX-P07-S04-T02 | PLANNED | R1 | Implement Fleet/Compare UX separating measured evidence from model-assisted analysis. | compare data |
| KX-P07-S05-T01 | PLANNED | R2 | Implement synthesis as new candidate lineage with mandatory fresh verification. | compare |
| KX-P07-S06-T01 | PLANNED | R2 | Implement provider/runtime budget and concurrency control with graceful unknown-usage behavior. | scheduler |

**P07 gate:** multi-agent work cannot corrupt shared state; retries do not duplicate side effects; synthesis is separately proven.

---

## P08 — Git, workspaces, and review

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P08-S01-T01 | PLANNED | R2 | Implement Project adapter for ordinary folders and git repositories. | P04 files |
| KX-P08-S02-T01 | PLANNED | R2 | Import/adapt worktree create/list/switch/remove/recovery primitives with provenance. | git project + provenance |
| KX-P08-S03-T01 | PLANNED | R2 | Implement runtime-scoped Git capability detection/fallback across native/WSL/SSH. | runtime contract |
| KX-P08-S03-T02 | PLANNED | R2 | Implement status/diff/stage/commit/branch/history with safe process invocation. | Git capability layer |
| KX-P08-S04-T01 | PLANNED | R1 | Implement diff/review UI, inline feedback-to-agent, staged state and candidate compare. | desktop + git |
| KX-P08-S04-T02 | PLANNED | R2 | Bind verification badges/results to exact candidate head. | quality/evidence + review |
| KX-P08-S05-T01 | PLANNED | R2 | Implement GitHub issue/PR/review adapter behind provider-neutral review contracts. | git review |

**P08 gate:** local code journey ends in proven diff + commit/PR; worktree candidates isolated; Git version/host fallbacks proven.

---

## P09 — Evidence, replay, and recovery

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P09-S01-T01 | PLANNED | R2 | Make task/run UI projections rebuild deterministically from durable events + indexed projections. | event model + product flows |
| KX-P09-S02-T01 | PLANNED | R2 | Define and implement supported checkpoint records across task/workspace/browser/runtime state. | projections |
| KX-P09-S03-T01 | PLANNED | R3 | Implement replay-semantics registry and enforcement for deterministic/idempotent/compensatable/non-replayable operations. | capability operations |
| KX-P09-S04-T01 | PLANNED | R3 | Implement fork-from-checkpoint with new lineage/operation identities. | checkpoints + replay semantics |
| KX-P09-S05-T01 | PLANNED | R2 | Implement independently verifiable evidence-bundle export with digests and redaction manifest. | evidence/artifact store |
| KX-P09-S06-T01 | PLANNED | R3 | Prove crash/restart/interruption recovery for daemon, renderer, browser and partial durable writes. | all P09 |
| KX-P09-S06-T02 | PLANNED | R3 | Prove duplicate-side-effect prevention/ambiguity surfacing after reconnect/retry. | replay + scheduler |
| KX-P09-S07-T01 | PLANNED | R2 | Implement versioned project export/import with manifest, digests, redaction/exclusion records and untrusted-import validation. | events + artifacts + context |
| KX-P09-S07-T02 | PLANNED | R3 | Implement backup/restore and reference-aware Artifact CAS GC with interrupted-operation recovery. | metadata + CAS |
| KX-P09-S07-T03 | PLANNED | R3 | Implement scoped delete semantics for run/task/project/history/memory/browser state with explicit external-provider leftovers. | lifecycle model |
| KX-P09-S08-T01 | PLANNED | R2 | Preserve ContextBundle/memory source lineage through replay, fork, export and source invalidation. | context + replay |

**P09 gate:** accepted local golden journeys recover honestly; evidence validates independently; replay never blindly repeats consequential actions; project history is portable; backup/restore/GC/delete semantics are proven.

---

## P10 — Runtime fabric

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P10-S01-T01 | PLANNED | R3 | Implement container runtime lifecycle, mounts, network/resource policy and artifact transfer. | P04 + KRP |
| KX-P10-S02-T01 | PLANNED | R3 | Define VM/microVM provider contract and qualify one stronger-isolation backend if feasible for Gen 1. | runtime fabric |
| KX-P10-S03-T01 | PLANNED | R3 | Implement WSL runtime identity/path/argv/env/files/process/PTY/Git handling. | KRP + local runtime |
| KX-P10-S04-T01 | PLANNED | R3 | Implement SSH runtime enrollment/probing/process/PTY/files/git/reconnect semantics. | KRP + identity |
| KX-P10-S05-T01 | PLANNED | R3 | Implement remote-device mutual identity, encrypted transport and revocation. | identity + KRP |
| KX-P10-S05-T02 | PLANNED | R3 | Implement resumable remote event streams and operation idempotency/duplicate-call handling. | remote transport |
| KX-P10-S05-T03 | PLANNED | R3 | Qualify mixed-version compatibility and controller/host policy intersection. | remote protocol |
| KX-P10-S06-T01 | PLANNED | R1 | Implement Machines/runtime chooser UX with trust/capability/contact visibility. | multiple runtime providers |

**P10 gate:** same bounded workload proven local + isolated + remote; disconnect remains unverifiable; replay attacks/duplicates fail safely.

---

## P11 — Tool, MCP, skills, and integration ecosystem

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P11-S01-T01 | PLANNED | R3 | Implement MCP 2026-07-28 client core, discovery/cache, auth and extension negotiation. | protocol spine + secrets |
| KX-P11-S01-T02 | PLANNED | R2 | Implement MCP Tasks mapping and compatibility fixtures for intentionally supported older MCP generation(s). | MCP core |
| KX-P11-S02-T01 | PLANNED | R3 | Implement policy-mediated Kernux MCP server surface for explicitly exposed capabilities. | MCP client/policy |
| KX-P11-S03-T01 | PLANNED | R2 | Implement Agent Skills discovery/install/project scope/provenance with no implicit privilege. | project + provenance |
| KX-P11-S04-T01 | PLANNED | R2 | Implement generic OpenAPI/HTTP and CLI tool adapters with output bounds and secret broker support. | tool runtime |
| KX-P11-S05-T01 | PLANNED | R3 | Define plugin manifest, capability/network/secret declarations and update permission-delta UX. | capabilities + provenance |
| KX-P11-S06-T01 | PLANNED | R2 | Publish internal extension SDK only for contracts proven stable by prior phases. | stable contracts |

| KX-P11-S07-T01 | PLANNED | R3 | Define ToolDescriptor/provider privacy manifest and local capability registry with effect class, schemas, secret bindings, network destinations, data classes, retention, telemetry, cost/payer, idempotency, evidence and provenance; unknown sensitive metadata fails closed. | tool runtime + P02 egress/secrets |
| KX-P11-S07-T02 | PLANNED | R2 | Implement bounded intent-to-capability discovery/routing over local/MCP/OpenAPI/CLI/Skills without flooding agent context; deterministic eligibility precedes probabilistic ranking. | ToolDescriptor + context |

**P11 gate:** external tools cannot self-expand permissions; tool/provider data boundaries are inspectable and fail closed when unknown for sensitive use; bounded capability discovery works without giant tool lists; MCP conformance and one real integration journey proven.

---

## P12 — Automation and long-running work

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P12-S01-T01 | PLANNED | R3 | Implement local scheduler with durable intent, missed-run policy and machine availability semantics. | P09 recovery |
| KX-P12-S02-T01 | PLANNED | R3 | Implement scoped event/webhook triggers through authorized connectors. | scheduler + integrations |
| KX-P12-S03-T01 | PLANNED | R3 | Implement background supervision, wake/restart/approval wait/budget notifications. | scheduler + runtimes |
| KX-P12-S04-T01 | PLANNED | R2 | Implement reusable task templates that compile to ordinary Task/WorkUnit contracts. | task engine |
| KX-P12-S04-T02 | PLANNED | R3 | Prove no duplicate scheduled side effect after crash/restart/clock or connectivity edge cases. | automation stack |
| KX-P12-S05-T01 | PLANNED | R3 | Make privacy mode/data boundary durable automation state: scheduled/background work cannot activate external AI/tools/accounts or widen egress merely because the user is absent. | scheduler + P02 egress + provider manifests |

**P12 gate:** delayed, scheduled and event-driven work survives restart without duplicate side effects and preserves the same privacy mode, egress limits, budgets and approval semantics as interactive work.

---

## P13 — Mobile companion

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P13-S01-T01 | PLANNED | R3 | Implement mobile pairing/auth/revocation against Kernux identity/relay model. | P10 remote |
| KX-P13-S02-T01 | PLANNED | R1 | Implement active tasks/status/results/notification experience. | mobile auth + task APIs |
| KX-P13-S03-T01 | PLANNED | R3 | Implement complete approval cards with action/target/data/origin/duration/risk context. | policy APIs |
| KX-P13-S04-T01 | PLANNED | R2 | Implement follow-up, pause/resume/stop and simple task launch. | task remote control |
| KX-P13-S05-T01 | PLANNED | R2 | Implement approved screenshot/browser/computer observation stream and artifact view. | runtime streams |
| KX-P13-S05-T02 | PLANNED | R3 | Prove stale/lost mobile clients cannot replay expired grants or duplicate commands. | mobile complete |

---

## P14 — Teams and optional Kernux cloud

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P14-S01-T01 | PLANNED | R3 | Define organization/user/team/project identity and membership model. | stable local identity |
| KX-P14-S02-T01 | PLANNED | R3 | Implement org policy intersection that can tighten but not bypass kernel/runtime hard constraints. | org identity + policy |
| KX-P14-S03-T01 | PLANNED | R3 | Implement shared runtime/skill/secret-reference ownership and revocation metadata. | teams + runtime/integrations |
| KX-P14-S04-T01 | PLANNED | R2 | Implement shared tasks, human/agent assignments, comments/notes and audit projection. | teams + task engine |
| KX-P14-S05-T01 | PLANNED | R3 | Implement authenticated encrypted relay/sync for NAT/mobile/remote scenarios with local-first fallback. | remote fabric |
| KX-P14-S06-T01 | DEFERRED | R3 | Design signed/provenanced marketplace/registry only after plugin ecosystem and governance prove mature. | P11 + team policy |

---

## P15 — Hardening, release, and launch

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P15-S01-T01 | PLANNED | R3 | Execute supported macOS/Windows/Linux qualification matrix plus applicable WSL/SSH journeys. | release feature set |
| KX-P15-S02-T01 | PLANNED | R3 | Close threat-model/adversarial corpus and perform independent/manual R3 security review. | feature complete |
| KX-P15-S03-T01 | PLANNED | R2 | Establish/ratchet startup, terminal, workspace, event, browser and multi-agent performance budgets from measurement. | feature complete |
| KX-P15-S03-T02 | PLANNED | R2 | Stress multi-agent, multi-terminal, browser and large-repository bounds; fix unbounded scans/output. | performance baseline |
| KX-P15-S04-T01 | PLANNED | R3 | Qualify installer, update, migration, rollback/recovery and uninstall on each supported desktop OS. | packaging |
| KX-P15-S05-T01 | PLANNED | R3 | Generate/verify SBOM, checksums, signatures/notarization, provenance attestations and notices. | release pipeline |
| KX-P15-S06-T01 | PLANNED | R1 | Complete user/admin/security/developer/onboarding/troubleshooting/privacy documentation. | stable product |
| KX-P15-S07-T01 | PLANNED | R2 | Preregister and run comparative evaluation before any superiority claims; preserve all negative evidence. | stable benchmark build |
| KX-P15-S08-T01 | PLANNED | R3 | Qualify exact release commit independently from ordinary CI and publish evidence packet. | all release gates |
| KX-P15-S09-T01 | PLANNED | R2 | Qualify telemetry-disabled operation and verify no hidden cloud/analytics dependency for supported local journeys. | stable product |
| KX-P15-S09-T02 | PLANNED | R3 | Qualify reviewable support bundles against synthetic secret/prompt/file fixtures and redaction manifests. | diagnostics + security |
| KX-P15-S10-T01 | PLANNED | R3 | Qualify project portability, backup/restore migration, deletion and uninstall cleanup on supported platforms. | P09 lifecycle + packaging |
| KX-P15-S11-T01 | PLANNED | R3 | Prove the local-only end-to-end privacy golden journey with telemetry disabled and a network oracle confirming zero non-loopback traffic for the fully offline fixture. | P02-P10 privacy/local path + P15 security |
| KX-P15-S12-T01 | PLANNED | R3 | Implement and qualify signed local commercial entitlement validation with public-key verification, bounded offline grace, key rotation and no user-work-content transmission. | packaging + identity + X16 |
| KX-P15-S12-T02 | PLANNED | R2 | Prove renewal/expiry/cancellation/device-seat behavior, data export/readability after expiry, and separation of commercial entitlement from kernuxd capability Grants. | entitlement + lifecycle |
| KX-P15-S12-T03 | PLANNED | R2 | Audit paid-product runtime economics and prove ordinary subscribed-user founder-funded variable COGS is zero across inference, browser/search, storage, agents, connectors and automation. | all paid-product runtime paths + X09 |

---

## P16 — Work-product and artifact studio

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P16-S01-T01 | PLANNED | R2 | Define provider-neutral artifact-authoring/editing contracts, capability scopes, source lineage, validation and round-trip semantics. | P09 + P11 |
| KX-P16-S02-T01 | PLANNED | R2 | Implement local Markdown/HTML/PDF/DOCX artifact production and bounded round-trip editing with validation evidence. | artifact contract + P04 docs |
| KX-P16-S03-T01 | PLANNED | R3 | Implement spreadsheet/data-workbook production for XLSX/CSV with formula/style preservation boundaries and recalculation/validation strategy. | artifact contract + P04 data |
| KX-P16-S04-T01 | PLANNED | R2 | Implement editable PPTX/presentation production with template/brand/layout/notes preservation and evidence. | artifact contract |
| KX-P16-S05-T01 | PLANNED | R2 | Implement reproducible analysis/notebook/chart/report packaging with source/data lineage and rerun metadata. | artifact contract + runtimes |
| KX-P16-S06-T01 | PLANNED | R3 | Implement bounded lightweight site/app generation, preview, packaging and optional publish adapters under external.publish policy. | artifacts + browser + integrations |
| KX-P16-S07-T01 | PLANNED | R2 | Qualify template reuse, edit-in-place, export/import and native connected-editor adapters without making any one office suite mandatory. | P16 surfaces |

**P16 gate:** Kernux can create and revise representative editable documents, spreadsheets, presentations, analysis packages and lightweight sites/apps with source lineage, validation, portability and no mandatory Kernux-hosted compute.

---

## P17 — Integration and event fabric

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P17-S01-T01 | PLANNED | R2 | Define connector registry/discovery contract across MCP, OpenAPI/HTTP, CLI, A2A, Skills and native adapters. | P11 stable contracts |
| KX-P17-S02-T01 | PLANNED | R3 | Implement OAuth/API-key/account-selection lifecycle through the secret broker with revocation, refresh, scope and audit semantics. | P02 secrets + connector contract |
| KX-P17-S03-T01 | PLANNED | R2 | Implement intent-based tool discovery/routing so large integration catalogs do not flood agent context. | context + connector registry |
| KX-P17-S04-T01 | PLANNED | R3 | Implement connector health, capability drift, permission-delta, data-boundary and cost-owner inspection. | registry + policy |
| KX-P17-S05-T01 | PLANNED | R3 | Implement normalized webhook/event/subscription adapters with dedupe, replay protection and provenance. | P12 triggers |
| KX-P17-S06-T01 | PLANNED | R2 | Qualify representative integrations across email/calendar, messaging, cloud storage/docs, project tracking, source control and one business system. | connector/auth/event fabric |
| KX-P17-S07-T01 | PLANNED | R2 | Define generated-adapter and third-party integration-provider boundaries so breadth can grow without core coupling. | stable connector contracts |

| KX-P17-S08-T01 | PLANNED | R2 | Define normalized IntegrationEvent contract with source/account identity, dedupe, trust/sensitivity, provenance and connector revision; keep it separate from kernel audit Event authority. | P12 triggers + connector registry |
| KX-P17-S08-T02 | PLANNED | R3 | Implement event normalization/dedupe/association/enrichment pipeline with restart-safe state and no event-to-authority shortcut. | IntegrationEvent |
| KX-P17-S09-T01 | PLANNED | R2 | Define and implement LiveDataObject schema/row/source/freshness/conflict/refresh history with field-level provenance and local storage-of-record semantics. | web acquisition + integration fabric + evidence |
| KX-P17-S09-T02 | PLANNED | R3 | Implement incremental refresh, partial-failure/stale-row/deletion/schema-migration semantics and cost/privacy budgets for LiveDataObjects. | LiveDataObject + P12 automation |

**P17 gate:** broad app-category work is possible through scoped accounts, discoverable tools and event sources without exposing giant tool lists, leaking secrets or binding Kernux to one integration vendor; IntegrationEvents and LiveDataObjects preserve local storage-of-record, provenance, freshness and explicit external data boundaries.

---

## P18 — Multimodal and cross-device interaction

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P18-S01-T01 | PLANNED | R2 | Define multimodal ContextItem contracts for audio, image, screen/window and optional camera/video sources with provenance/sensitivity metadata. | context model |
| KX-P18-S02-T01 | PLANNED | R2 | Implement local/BYOK speech input/transcription and optional speech output behind provider-neutral contracts. | model providers + owner-sustainable execution |
| KX-P18-S03-T01 | PLANNED | R3 | Implement bounded screenshot/screen/window observation and visual computer-use fallback behind policy, approvals and evidence. | P04/P06 + model/runtime |
| KX-P18-S04-T01 | PLANNED | R3 | Extend mobile from steering to authenticated task handoff, artifact inspection, capture/share and resumable cross-device continuity. | P13 + P10 remote |
| KX-P18-S05-T01 | PLANNED | R2 | Implement locale/timezone/RTL/Unicode/IME foundations and multilingual UX/content boundaries. | UI + scheduling + context |
| KX-P18-S06-T01 | PLANNED | R2 | Qualify keyboard-only, semantic assistive-technology and scalable-text journeys across primary desktop surfaces. | accessibility baseline |

**P18 gate:** users can move between text, voice, images/screens and devices while preserving one task identity, explicit authority, accessibility, locale correctness and evidence lineage.

---

## P19 — Proactive personal/work agent

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P19-S01-T01 | PLANNED | R2 | Define user-governed personal/work relationship context over files, connected apps, tasks, memory and time without turning graph state into authority. | context/memory + P17 |
| KX-P19-S02-T01 | PLANNED | R2 | Implement universal capture/inbox for user intents, shared items, notifications and deferred work that compiles to ordinary Tasks/WorkUnits. | task engine + UI |
| KX-P19-S03-T01 | PLANNED | R3 | Implement condition watches/change monitors with durable evaluation state, notification suppression/deduplication and evidence. | P12 + web/integration events |
| KX-P19-S04-T01 | PLANNED | R3 | Implement proactive suggestions that are policy-scoped, explainable, dismissible and unable to execute consequential actions without authority. | memory + policy + monitors |
| KX-P19-S05-T01 | PLANNED | R2 | Implement reusable routines across email/calendar/files/web/apps using local/BYOK/BYOC/provider-funded execution paths. | P17 + P12 |
| KX-P19-S06-T01 | PLANNED | R3 | Qualify representative personal/work journeys including morning review, meeting follow-up, research-to-deliverable and cross-app operations. | P19 complete |

| KX-P19-S07-T01 | PLANNED | R3 | Implement ActionProposal/Action Inbox over ordinary task/capability/grant semantics with destination/account/data/credential/cost/evidence preview, expiry and approval state. | P17 events + policy + desktop |
| KX-P19-S08-T01 | PLANNED | R3 | Implement deterministic/model-evaluated rules with simulation, firing log, rate/suppression/loop prevention and no model-evaluated self-authorization. | ActionProposal + automation |
| KX-P19-S08-T02 | PLANNED | R2 | Implement local-first briefings and cross-app association/coherence as derived views with correction learning under explicit user control. | events + memory/context |

**P19 gate:** Kernux can assist proactively across time and apps through ActionProposal/Action Inbox/rules/briefings without hidden surveillance, memory poisoning, notification spam, authority escalation, silent cloud processing or project-owner-subsidized compute.

---

## P20 — Ecosystem and universal-scale maturity

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P20-S01-T01 | PLANNED | R3 | Finalize signed/provenanced Capability Pack and extension package/registry contracts with capability/network/secret/data/privacy declarations, uninstall boundaries and no alternate authority/task/secret system. | P11 + P17 |
| KX-P20-S02-T01 | PLANNED | R3 | Implement curated public and organization-private registry paths, revocation, compatibility and permission-delta updates. | extension contract + org policy |
| KX-P20-S03-T01 | PLANNED | R2 | Publish stable SDKs/templates for agents, runtimes, connectors, skills, artifact adapters, policy packs and benchmark journeys. | proven internal contracts |
| KX-P20-S04-T01 | PLANNED | R3 | Define and qualify optional managed Kernux services as separate SKUs under explicit tenancy, privacy, usage limits/pricing, reliability and positive unit-economics boundaries; never subsidize them from the base local-software subscription. | P14 + X09 + X16 |
| KX-P20-S05-T01 | PLANNED | R3 | Build universal capability benchmark/regression corpus spanning build, work, research, automate, operate and create modes. | mature capability set |
| KX-P20-S06-T01 | PLANNED | R3 | Establish SLOs/chaos/recovery/load qualification for long-running tasks, integrations, browsers, devices, artifacts and storage at scale. | mature capability set |
| KX-P20-S07-T01 | PLANNED | R2 | Run versioned comparative evaluations before any broad market-leadership claim and publish reproducible methodology plus negative evidence. | benchmark corpus |
| KX-P20-S08-T01 | PLANNED | R3 | Qualify provider-loss/dependency-failure journeys proving core task truth survives replacement or disappearance of any single external provider. | provider-neutral architecture |

**P20 gate:** Kernux has a governed ecosystem, sustainable optional managed services, broad reproducible benchmark coverage, explicit reliability targets and proven resilience to provider loss.

---

# Cross-cutting permanent task classes

These are not one-off phase tasks. Every applicable Grain must account for them.

## X01 — Cross-platform

Any filesystem/process/PTY/git/browser/native behavior must state which of macOS, Windows, Linux, WSL, SSH are applicable and provide proof accordingly.

## X02 — Security

Any new privilege, secret, browser, plugin, remote, updater, installer, or destructive operation updates threat coverage and carries R3 unless explicitly justified lower.

## X03 — Provenance

Any copied/adapted donor code updates machine-readable provenance and notices in the same change or a mechanically linked import change.

## X04 — Compatibility

Any remote/wire/persisted schema change declares version/migration behavior and tests supported mixed versions.

## X05 — Accessibility

Any user-visible UI change preserves keyboard/focus/semantic status and non-color-only communication.

## X06 — Evidence

Any task completion updates exact-head/run evidence; failures and blocked checks remain visible.

## X07 — Recovery

Any durable or side-effecting workflow defines crash/cancel/retry/restart behavior before it is called complete.

## X08 — Data boundary/privacy

Any integration that sends content off-device declares what data, to whom, for what purpose, with which retention/configuration assumptions.

## X09 — Zero founder-funded runtime COGS

Any capability that can create recurring per-use cost must declare the cost owner. Core paid-product operation may use only local/user-owned hardware, user BYOK/provider-native subscriptions, organization-owned infrastructure, or an explicitly separately priced managed service with positive unit economics.

Founder-funded credentials, shared inference, browser/search pools, storage, relays, automation runtimes, connector usage, promotional credits, or hidden hosted infrastructure are not valid proof paths for core capability.

The base local-software subscription must not create variable runtime COGS for ordinary user work.

See `docs/canonical/LOCAL_SUBSCRIPTION_ZERO_RUNTIME_COGS.md`.

## X10 — Integration lifecycle

Any external account/tool/event source defines discovery, auth, scope, account selection, revocation, capability drift, data boundary, cost ownership and failure behavior.

## X11 — Artifact fidelity

Any user-facing work-product format defines editability, validation, round-trip limits, template/style preservation, provenance, export and degradation behavior.

## X12 — Multimodal and internationalization

Any new modality or user-visible surface defines sensitivity/provenance, accessibility, Unicode/locale/timezone behavior and provider/data-boundary implications.

## X13 — Evaluation and SLOs

Any major capability family defines golden journeys, regression fixtures, observable reliability/latency/cost metrics and the evidence required for public claims.

## X14 — Human agency

Any proactive, background, high-consequence or externally publishing behavior defines user control, approval, interruption, explanation, notification and rollback/compensation semantics where feasible.

---

## X16 — Commercial entitlement separation

Any paid-product implementation must keep commercial entitlement separate from Kernux capability authority.

Applicable work must define:

- plan/feature entitlement;
- signed local entitlement representation;
- issuer/key rotation;
- offline/grace behavior;
- renewal metadata;
- expiry/cancellation behavior;
- user-data availability/export after expiry;
- privacy of entitlement refresh;
- explicit open-source/commercial package boundary;
- no project-content transmission for license validation;
- no task/token/browser-minute/local-storage metering for user-supplied resources.

A subscription entitlement cannot grant filesystem, process, network, secret, browser, runtime, tool, or side-effect authority.

---

## X15 — Local privacy and data sovereignty

Every applicable Grain must state:

- privacy mode;
- egress class and exact destination class;
- local-vs-external processing boundary;
- data classes crossing the boundary;
- secret handles and injection destination;
- provider telemetry/retention assumptions;
- no-silent-cloud-fallback behavior;
- local/offline degradation path;
- context minimization/redaction;
- deletion/index invalidation behavior;
- privacy evidence and adversarial fixtures.

A capability cannot be called local-first merely because its UI runs locally. If user content is sent to an external model, embedding, search, OCR, speech, browser, tool, telemetry or remote-runtime provider, that transfer is an explicit governed external boundary.

See `docs/canonical/LOCAL_PRIVACY_IMPLEMENTATION_PLAN.md`.

---

# Initial implementation sequence

After the canonical planning PR merges, the intended immediate sequence is:

```text
KX-P00-S01-T01
  -> KX-P00-S01-T02
  -> KX-P00-S02-T01 + KX-P00-S03-T01
  -> KX-P00-S04-T01/T02
  -> P00 gate
  -> P01 contracts
```

Do **not** start by wholesale-copying Orca, TinyFish, or Desktop Commander. Large imports begin only when the receiving Kernux contract and provenance gate exist.

# Registry maintenance

When implementation discovery creates new work:

- add it under the earliest dependency-correct phase;
- explain why existing tasks do not cover it;
- avoid catch-all tasks such as `finish integration`;
- refine over-large tasks into child SpecGrain specs rather than inflating prompts;
- never mark later tasks PROVEN to make the plan look closer to completion.
