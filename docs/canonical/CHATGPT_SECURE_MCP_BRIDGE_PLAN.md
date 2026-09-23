# Kernux ChatGPT Secure MCP Bridge Plan
## Status
- Planning only; research snapshot 2026-09-23.
- Canonical base observed before this branch: `main@a55832a5e5f6b2cf2abd6ac72c756c8ab2a01235`.
- No `specs/CURRENT.md` change, P02 advancement, donor import, dependency admission, or implementation authority.
- Source ledger: `docs/research/CHATGPT_SECURE_MCP_BRIDGE_SOURCE_LEDGER.md`.
- Open Computer Use detail remains separately owned by draft PR #200.
- Working component: **Kernux ChatGPT Bridge**; proposed package/binary: `kernux-mcp-bridge`.
## 1. Decision
Build a **thin ChatGPT/OpenAI MCP edge backed by Kernux**. Do not create a second privileged gateway and do not make MCP the host-security boundary.
```text
ChatGPT / supported OpenAI surface
  -> OpenAI Secure MCP Tunnel
  -> openai/tunnel-client
  -> kernux-mcp-bridge                 [unprivileged translation edge]
  -> authenticated local IPC / KRP
  -> kernuxd                           [privileged authority]
       -> Grants / policy / approval / secrets / audit / egress / workspace identity
       -> Files / Process / PTY / Git / System / ComputerUse / Browser providers
```
Kernux already owns the required security primitives. A standalone privileged MCP project would duplicate policy, secret, audit, workspace, and runtime truth. The bridge may later ship independently, but authority remains `kernuxd`.
Rejected topology:
```text
ChatGPT -> tunnel -> Node MCP server -> arbitrary shell / desktop authority
```
## 2. Transport boundary
Use Secure MCP Tunnel as the default remote transport:
```text
NO public inbound listener
NO router port forwarding
NO public relay
NO firewall hole
Windows PC -> outbound HTTPS -> OpenAI Secure MCP Tunnel -> OpenAI product
```
The tunnel credential authenticates `tunnel-client`; it does **not** authorize host actions. Every MCP request is re-authorized by `kernuxd`.
## 3. ChatGPT compatibility
Local safety must not depend on one ChatGPT UI or plan. Current product constraints require: a useful read-only path; versioned tool schemas; explicit unsupported errors on incompatible snapshots; local approval independent of ChatGPT confirmation; no assumption that agent mode/deep research can perform the same write-capable custom-app actions as ordinary supported app use.
## 4. Trust boundaries
| Zone | Trusted for | Not trusted for |
| --- | --- | --- |
| Human / org policy | Granting authority | Runtime correctness |
| OpenAI product | Sending MCP calls | Local host authorization |
| Secure MCP Tunnel | Private transport | Host policy |
| `tunnel-client` | Transport | Capability decisions |
| `kernux-mcp-bridge` | Schema validation + translation | Granting privilege |
| `kernuxd` | Local policy, approval, evidence | External-content truth |
| Provider | One authorized operation | Expanding authority |
| Web/repo/document/tool output | Data | Authorization |
| Agent/model | Proposal/request | Permission or completion proof |
## 5. MCP surface
Do not expose `run_anything`. Split operations when permissions/consequences differ.
```text
system.describe              workspace.describe
files.list/read/search       files.write/edit/move/delete
process.start/status/read_output/cancel
shell.powershell             shell.session_start/write
git.status/diff/log          git.run_declared_operation
apps.list                    windows.list/observe/activate
browser.observe/navigate/act screen.capture
computer.act                 clipboard.read/write
```
Every tool compiles into Kernux `CapabilityRequest` values. MCP annotations must accurately describe read-only/destructive/open-world/idempotency behavior but never replace server-side authorization.
## 6. Capability families
```text
files.observe/read/write/delete
process.observe/spawn/signal
pty.open/write
git.observe/mutate
system.observe
app.launch
window.observe/activate
computer.observe/semantic_act/coordinate_act/global_input
screen.capture
clipboard.read/write
browser.observe/navigate/mutate/download/upload
network.direct
secret.use/disclose
system.privileged
```
A Grant binds subject + action + resource + runtime + constraints + expiry + policy revision.
## 7. Permission profiles
| Profile | Compiled behavior |
| --- | --- |
| `READ_ONLY` | Observation in selected workspaces only; no mutation/process/browser/input/clipboard read |
| `DEVELOPMENT` | Repository-scoped read/write + declared build/test/Git; raw shell/network/destructive/secret/desktop input gated |
| `BALANCED` | Broader scoped control with local approval for material mutations and sensitive observations |
| `FULL` | Owner-only broad host mode; protected targets, audit, destructive/system controls remain active |
`FULL` is never default.
## 8. Workspace isolation
Bind each session to:
```text
workspace_id
runtime_id
repository_identity
read_roots / write_roots / delete_roots
browser_context_id
network_policy_id
secret_policy_id
grant_revision
```
Workspace A cannot silently resolve project B. Cross-workspace access requires fresh explicit authority and visible audit/approval.
## 9. Windows filesystem policy
String-prefix authorization is forbidden as the sole boundary. Account for normalization/case, symlinks, junctions, mount/reparse points, hard links, UNC, device namespaces, short/alternate paths, `..`, environment expansion, ADS, WSL translation, removable media, and TOCTOU.
Existing-path authorization should follow:
```text
parse resource
-> reject unsupported namespace
-> open target/ancestor with safe flags
-> resolve final path from handle
-> capture volume + file identity
-> evaluate resource/root policy
-> perform operation through validated identity
```
For creation: authorize nearest existing parent identity, reject unexpected reparse traversal, create with bounded semantics, then revalidate resulting identity. Read/write/delete roots may differ. Unsupported namespace behavior fails closed.
## 10. Process / PowerShell
Prefer direct argv execution over shell interpolation. Keep distinct: declared project command; argv process; PowerShell script; interactive shell/PowerShell; background job; PTY; system mutation; privileged execution.
Required invariants: executable and argv stored separately; cwd is scoped; child env is explicit/allowlisted; tunnel/unrelated secrets removed; process tree supervised; Windows Job Objects used where appropriate; output bounded; process identity not inferred from reusable PID alone; cancellation request differs from verified termination; ambiguous side-effect retry fails closed; command blocklists are never the security boundary. Constrained PowerShell modes are defense-in-depth only.
## 11. Secrets and tunnel credential
Use a dedicated minimum-permission tunnel credential. Requirements:
- plaintext tunnel key never enters ordinary SQLite, logs, approval history, model context, crash dumps, or child env;
- Kernux secret broker owns at-rest protection using Windows-native facilities such as DPAPI/approved credential storage;
- `tunnel-client` receives the key through a narrow launch/handoff path;
- bridge children inherit an allowlisted environment;
- credential rotation/revocation is independent from workspace data;
- `.env`, SSH keys, browser cookies, Git/cloud credentials are sensitive resources;
- secret use and disclosure are separate capabilities;
- known secret forms are redacted before persistence.
## 12. Non-self-approvable local approval
Approval is independent of ChatGPT and cannot be actuated by the agent.
1. Bind approval to a digest of normalized target/action/argv-or-URL/workspace/runtime/data+secret refs/policy revision/nonce/expiry/consequence class.
2. Any target/argument/policy drift invalidates approval.
3. Approval process/window is a protected resource denied to UIA, Window, Browser, coordinate and global-input providers.
4. While approval is pending, the agent interactive-input lease for that Windows session is suspended.
5. Running background processes do not gain approval authority.
6. One-time approval cannot replay after expiry, target change or policy revision.
7. Destructive/privileged actions use fresh authority.
8. Denial is typed.
Injected-input detection may be defense-in-depth; it is not the sole invariant.
## 13. Desktop hierarchy
Preferred order:
```text
typed app/tool API
> Windows UI Automation / semantic accessibility
> bounded Win32/window-targeted action
> browser DOM for browser content
> vision
> coordinate input
> raw global keyboard/mouse
```
Element refs are session-local + observation-revision-bound. Stale/substituted refs fail rather than reusing coordinates. Record actual method: `typed_api | uia_pattern | win32_targeted | browser_dom | vision | coordinate_input | global_input`. Failure at a safer level never grants a stronger fallback. PR #200 owns detailed Open Computer Use integration; this bridge consumes only the qualified Kernux ComputerUse contract.
## 14. Human input
Use `input_lease = none | background | foreground`, bounded expiry, visible foreground-control indicator, human override, emergency stop, takeover/relinquish semantics. Human takeover invalidates stale UI observations. Prefer background/window-targeted interaction when reliable.
## 15. Browser boundary
Preferred order:
```text
WebMCP / typed page capability
> Playwright / DOM / accessibility
> semantic browser provider
> qualified native ComputerUse
> vision
> coordinates
```
Use an isolated automation profile by default. Personal authenticated profile reuse is explicit and workspace/session scoped. Mutation binds expected origin; unexpected navigation/origin drift returns `BROWSER_ORIGIN_CHANGED`. Page content is hostile provenance and cannot expand Grants. Downloads/uploads/clipboard/cookies/network destinations are separately governed.
## 16. Network / egress
Shell does not imply network. Preserve canonical P02 classes:
```text
NONE
DIRECT_DESTINATION
CONNECTED_ACCOUNT
EXTERNAL_MODEL
EXTERNAL_TOOL
REMOTE_RUNTIME
UPDATE
TELEMETRY
```
Unknown sensitive egress fails closed. Material destinations are auditable without persisting secret payloads.
## 17. Audit and failures
Every material operation records: timestamp, external call ID, session/workspace/runtime, tool, CapabilityRequest, Grant/policy revision, consequence class, normalized resource, approval decision, actual execution method, result, duration, artifact/evidence refs, redaction metadata. Raw secrets are excluded.
Baseline typed failures:
```text
PERMISSION_DENIED
LOCAL_APPROVAL_REQUIRED / LOCAL_APPROVAL_DENIED
PATH_OUTSIDE_SCOPE / RESOURCE_IDENTITY_CHANGED
SECRET_READ_BLOCKED
PROCESS_FAILED / PROCESS_TIMEOUT / PROCESS_CANCELLED / PROCESS_STATE_UNKNOWN
WINDOW_NOT_FOUND / ELEMENT_NOT_FOUND / ELEMENT_AMBIGUOUS / STALE_UI_STATE
SESSION_LOCKED / UAC_REQUIRED / MFA_REQUIRED / CAPTCHA_REQUIRED
NETWORK_DENIED / NETWORK_FAILURE / BROWSER_ORIGIN_CHANGED
HUMAN_TAKEOVER / CAPABILITY_UNAVAILABLE
PROTOCOL_VERSION_UNSUPPORTED / TUNNEL_UNAVAILABLE
```
Unknown state is not success.
## 18. Threat model
| Threat | Control | Required evidence | Residual risk |
| --- | --- | --- | --- |
| Malicious page steals secrets | Untrusted provenance; separate secret+egress Grants | Injection corpus cannot expand authority | User may intentionally approve disclosure |
| Malicious repo instruction | Repo text cannot grant shell/network/secret | Source injection fixtures | Human approval remains powerful |
| Shell output injection | Output is data | Tool-output injection fixtures | Model remains probabilistic |
| Project A -> B | Workspace/resource identity | Cross-workspace denial | Explicit multi-project Grant |
| Junction/reparse escape | Handle/final-path identity | Reparse suite | Compromised kernel/driver out of scope |
| Hard-link alias | Volume/file identity policy | Cross-root hard-link suite | Unsupported forms denied |
| Agent clicks approval | Protected surface + suspended lease | UIA/Win32/coordinate/global attempts denied | Same-user OS compromise out of scope |
| Approval drift/replay | Digest + nonce + expiry + policy revision | Mutate target/argv after approval | Fresh human approval |
| Shell semantic bypass | argv-first + shell separately gated | substitution/interpreter corpus | Approved raw shell is powerful |
| Env credential leak | allowlisted projection | child asserts no tunnel/ambient secrets | Explicitly granted secrets remain usable |
| Screenshot leaks secret | protected target/redaction/approval | secure-field fixtures | Unknown pixels may still be sensitive |
| Clipboard password | separate sensitive capability | deny/approve fixtures | Explicit approval |
| Browser session leak | isolated profile/context | cross-workspace cookie isolation | Explicit personal-profile reuse |
| Tunnel key stolen | protected store + narrow handoff | DPAPI + env/log scans | Same-user host compromise |
| Audit leaks secrets | pre-persistence redaction | seeded-secret corpus | Unknown secret shapes |
| Updater compromise | signed provenance + rollback | release verification | Signing/build key trust |
| Duplicate after reconnect | operation IDs/idempotency/ambiguity | disconnect/retry fixtures | External non-idempotent service |
| UAC/MFA/CAPTCHA/locked session | typed stop; no bypass | native qualification | Human completion required |
## 19. Capability comparison
Descriptive only; no marketing score.
| Dimension | Desktop Commander | Unlimited Agent | LocalAnt | Open Computer Use | Kernux Bridge target |
| --- | --- | --- | --- | --- | --- |
| Files | Yes | Yes | Yes | Not primary | Kernux-scoped |
| Shell/process | Yes | Yes | Yes | Not primary | Separate scoped capabilities |
| Git/dev | Via shell/files | Via shell/files | Documented Git/coding tools | Not primary | Yes |
| Browser | Not core structured path | Chrome control | Browser capability | UI/native automation | Playwright/DOM first |
| Native desktop | Not primary | Windows input/window | Not primary focus | Core | Qualified ComputerUse |
| Accessibility-first | No | Not primary | Not primary | Yes | Required |
| Local approval | Guardrails; not sandbox authority | Yes | Yes | Not policy kernel | Kernux broker |
| Policy | Advisory guardrails | Tool/path policy | Risk/security modes | Execution layer | Scoped Grants |
| Workspace isolation | Shell can bypass directory policy | Allowed roots | Gateway concepts | Not project authority | Required |
| Secrets | Not primary boundary | DPAPI tunnel key + env sanitation | Gateway concepts | Not gateway focus | Kernux broker |
| Audit | Local tool history | Audit | Dashboard/audit | Not gateway focus | Durable events/evidence |
| Network control | Shell broad | HTTP + approval | Gateway controls | Not primary | P02 egress |
| Secure MCP Tunnel | Not baseline | Yes | Not primary | No | Official transport |
| Privileged boundary | Trusted connected AI; guardrails not sandbox | Node policy | Gateway runtime | Native automation | Rust `kernuxd` |
## 20. Lifecycle / status
Owner operations: `install, pair_tunnel, start, stop, restart, status, rotate_credential, revoke_chatgpt_access, delete_tunnel_binding, repair, update, rollback, uninstall`.
Local status: `CONNECTED/DISCONNECTED, ACTIVE_SESSION, PENDING_APPROVAL, CURRENT_WORKSPACE, CURRENT_PROFILE, ACTIVE_INPUT_LEASE, LAST_ACTION, TUNNEL_HEALTH`. Revocation stops new bridge calls without ChatGPT cooperation.
## 21. First usable release
Required:
```text
system/workspace describe
files list/read/search/write/edit/move/delete
process start/status/read_output/cancel
gated PowerShell
Git status/diff/basic operations
local approval
audit/evidence
Secure MCP Tunnel
basic app/window inspection
revocation/status
```
Deferred: vision-first automation, global input, personal browser-profile attachment, arbitrary network access, privileged Windows/service/registry mutation.
## 22. Delivery slices
| Slice | Outcome | Dependencies | Acceptance |
| --- | --- | --- | --- |
| `CGB-000` | Research + architecture | Canonical docs | Pins, threat model, no task-state change |
| `CGB-001` | MCP bridge contract | P01 + P02 IPC | Versioned MCP->Capability mapping; no adapter privilege |
| `CGB-002` | Tunnel lifecycle/secret handoff | P02 secrets + egress | Restricted credential protected; bridge/children cannot read it |
| `CGB-003` | Read-only system/workspace | 001/002 | Real tunnel E2E; no mutation |
| `CGB-004` | Files read/search | P04 files | Path-escape corpus fails closed |
| `CGB-005` | File mutation + approval | P04 files + approval | Request-bound approval, atomic mutation, evidence |
| `CGB-006` | Process/PowerShell | P04 process/PTY | argv-first, env isolation, Job Objects, typed cancellation |
| `CGB-007` | Git workflow | P04 Git | Scoped status/diff/test journey |
| `CGB-008` | App/window observation | P04 ComputerUse | UIA/Win32 read-only, stale identity tests |
| `CGB-009` | Installer/status/revocation | 002-008 | Install/pair/connect/revoke/uninstall |
| `CGB-010` | First-release E2E | all above | ChatGPT->tunnel->bridge->Kernux real repo + adversarial suite |
| `CGB-011` | Structured browser | P06 | Isolated Playwright, origin drift/injection/upload/download tests |
| `CGB-012` | Semantic desktop actions | qualified P04 ComputerUse | Structured/background before coordinates |
| `CGB-013` | Vision/coordinate fallback | P06/P18 | Explicit method ceiling + input lease + evidence |
| `CGB-014` | Clipboard/sensitive observation | policy + ComputerUse | Separate Grants + protected fixtures |
| `CGB-015` | Release hardening | P15 | Signed provenance, update/rollback, native Windows matrix |
Each becomes a bounded SpecGrain Grain with exact dependencies/paths/negative cases/evidence and Diffcipline qualification.
## 23. Test strategy
**Unit:** capability mapping, path identity, policy compilation, redaction, tool schemas, operation IDs, audit construction.
**Integration:** Windows files/process/PowerShell/Git/local IPC/secret broker/tunnel lifecycle/UIA/Playwright/clipboard when enabled.
**Security:** traversal/reparse/hard-link/device/UNC/ADS, shell injection, env leakage, approval self-click, stale approval/UI refs, cross-workspace/browser leakage, secret/tunnel exfiltration, prompt/output injection, duplicate reconnect.
**Failure injection:** tunnel/network loss, sleep, process/bridge/daemon/browser crash, locked desktop, revoked permission, app closes, partial file write, approval denial/expiry.
**E2E:**
```text
ChatGPT -> Secure MCP Tunnel -> bridge -> kernuxd
-> inspect repo -> bounded edit -> tests -> inspect failure -> repair
-> rerun -> Git diff -> audit/evidence
```
Later desktop E2E: observe app -> semantic target -> act -> fresh observation -> approved evidence.
## 24. Review gates
High-risk slices require exact-head CI; native Windows qualification; applicable Linux/macOS checks without false parity; SpecGrain WorkPacket; Diffcipline exact-diff evidence; genuine Jev review; Alibaba Open Code Review accounting; semantic security review; zero blocking threads; post-merge verification before effectiveness. Review tools never replace deterministic tests.
## 25. Installer / UX
Target:
```text
Install Kernux/bridge
-> create/select Secure MCP Tunnel
-> store restricted credential locally through protected setup
-> choose workspaces
-> choose permission profile
-> connect
-> create/select private ChatGPT app using tunnel
-> verify read-only call
-> optionally enable development mutations
```
No multi-file manual configuration requirement.
## 26. Cost, recovery, rollback
Local bridge must not require Kernux-hosted relay/database/browser farm/observability/execution. User hardware and user-owned OpenAI/product access supply runtime resources. Optional hosted services never become hidden dependencies.
Recovery must support stopping bridge independently, revoking tunnel credentials, disabling external MCP while retaining local data, protocol-compatible rollback, rejecting unsafe downgrade/mismatch, restoring config without revoked secrets, and uninstalling bridge integration without deleting project data.
## 27. Implementation-readiness gate
Before first implementation Grain:
1. `kernuxd` is sole privileged policy authority.
2. MCP->Capability mapping is versioned.
3. Tunnel credential ownership/handoff is specified.
4. Child env excludes tunnel/unrelated secrets.
5. Read/write/delete resources are separately scoped.
6. Windows path/reparse/hard-link identity policy is explicit.
7. Process argv/cwd/env/lifecycle is explicit.
8. Approval is non-self-actuatable.
9. Approval drift/expiry/replay is explicit.
10. Browser/desktop authorities are separate.
11. UIA/DOM precede vision/coordinates.
12. Global input is separately gated.
13. Personal browser-profile reuse is explicit.
14. P02 egress remains authoritative.
15. Audit/redaction maps to Kernux evidence.
16. Typed failures survive MCP.
17. Immediate revocation exists.
18. Threat fixtures cover this plan.
19. Source provenance is pinned.
20. Local path needs no paid Kernux infrastructure.
21. ChatGPT product limitations are documented without weakening security.
22. PR #200, if merged, is consumed through its Kernux contract rather than duplicated.
## 28. Completion rule
The bridge is complete only when a supported OpenAI surface can request a real development workflow while local policy independently preserves:
```text
least privilege
workspace isolation
local approval
secret safety
egress control
auditability
revocation
human override
typed failures
no ambient shell authority
```
Success is not “ChatGPT can run commands.” Success is useful host work without moving authority into the model or MCP edge.
