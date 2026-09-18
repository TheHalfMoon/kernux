# Donor Import Checklist

## Purpose

This checklist is the canonical pre-merge control for source copied, adapted, ported, generated from, or materially derived from a donor system.

It complements the machine-readable provenance manifest and third-party notice inventory. It does not replace license review, security review, characterization, or exact-diff proof.

A donor import is not complete because the source is publicly licensed or because the founder has additional permission.

## Applicability

Use this checklist for every donor import wave, including Orca, TinyFish/AgentQL, Desktop Commander, and future third-party source.

A reference-only study with no copied source still requires exact source identity and notice inventory coverage when it is represented by a tracked provenance manifest.

## 1. Source identity and authorization

Before copying source:

- [ ] Identify the exact donor repository or artifact.
- [ ] Pin a full Git commit or immutable content digest.
- [ ] Record the source path or bounded source set.
- [ ] Record the authorization basis: public license, founder authorization, or other written authorization.
- [ ] Do not substitute a branch, tag, moving release channel, or remembered revision for the exact source pin.
- [ ] Do not invent a public repository identity for non-public source.

For a new import wave, re-check upstream truth and intentionally choose the pinned revision to import.

## 2. License and notice evidence

Before copying source:

- [ ] Add or verify the matching entry in `third_party/notices/inventory.json`.
- [ ] Preserve the exact upstream license text under `third_party/notices/licenses/`.
- [ ] Record and verify the SHA-256 digest of the preserved license bytes.
- [ ] Ensure the provenance manifest uses the same source identity, SPDX expression, and upstream license path.
- [ ] Include the preserved notice snapshot in the manifest's license evidence paths.
- [ ] Preserve required copyright and permission notices.

The notice inventory records source-license evidence. It does not decide the Kernux project license and does not establish future distribution compatibility. Those decisions remain separate governed work.

## 3. Embedded dependency, asset, and service review

Founder permission over donor source does not automatically change third-party obligations.

Review the bounded source set for:

- [ ] vendored libraries and package dependencies;
- [ ] fonts, icons, images, audio, video, and other media;
- [ ] model weights, datasets, prompts, and generated assets with separate terms;
- [ ] SDKs, native binaries, browser extensions, and bundled executables;
- [ ] trademarks, product names, logos, screenshots, and store assets;
- [ ] telemetry endpoints, update channels, hosted APIs, account backends, and service domains;
- [ ] code copied into the donor from another upstream project.

Record evidence or an explicit bounded finding for each applicable category. Do not silently inherit a donor's service dependency or branding.

## 4. Kernux receiving boundary

Before import:

- [ ] Identify the Kernux-owned contract that receives the donor behavior.
- [ ] Record exact source-to-destination mappings.
- [ ] Assign a transformation class: `verbatim`, `adapted`, `ported`, `reference-only`, or `generated`.
- [ ] Confirm the destination does not bypass the Kernux capability, runtime, event, policy, or evidence boundary.
- [ ] Confirm the import does not create a provider-specific public core contract unless explicitly authorized.

Reuse implementation aggressively where permitted; keep Kernux architecture provider-neutral.

## 5. Characterization before adaptation

Before semantic refactoring:

- [ ] Capture characterization tests for the donor behavior being preserved.
- [ ] If characterization is genuinely not applicable, record the reason.
- [ ] Preserve representative negative and boundary behavior, not only happy paths.
- [ ] Record platform assumptions relevant to macOS, Windows, Linux, WSL, SSH, browser, or remote execution.

Do not call an adaptation equivalent merely because it compiles.

## 6. Security review

A security review is required when the bounded import touches any of:

- host filesystem or process authority;
- shell, PTY, command execution, or native IPC;
- browser automation, authenticated browser state, or computer use;
- authentication, accounts, credentials, secrets, or tokens;
- remote execution, SSH, device pairing, or network listeners;
- updates, installers, plugins, extensions, or executable downloads;
- policy decisions, grants, approvals, or destructive operations.

For applicable imports:

- [ ] Define the trust boundary.
- [ ] Identify attacker-controlled inputs.
- [ ] Add adversarial tests for privilege/path/command/injection/secret boundaries.
- [ ] Confirm donor guardrails are not being treated as a Kernux sandbox or authorization kernel.
- [ ] Record the security evidence paths in the provenance manifest.

## 7. Mechanical import history

Prefer reviewable history:

1. provenance, notice evidence, and characterization preparation;
2. donor import with minimal mechanical path/name changes;
3. Kernux adaptation in one or more separate commits.

For every import:

- [ ] Keep the mechanical import commit distinct where practical.
- [ ] Record the exact import commit in the provenance manifest.
- [ ] Record subsequent adaptation commits in order.
- [ ] Ensure the import commit actually touches each claimed destination.
- [ ] Avoid silently mixing multiple donor revisions in one record.

Do not force-push or rewrite shared import history to make provenance cleaner after review.

## 8. Post-import verification

Before merge:

- [ ] Run `python3 tools/provenance/validate_notices.py check`.
- [ ] Run `python3 tools/provenance/validate.py check`.
- [ ] Run focused characterization and security tests.
- [ ] Run the required Diffcipline risk profile.
- [ ] Review the exact candidate with the repository-approved review process.
- [ ] Confirm existing CI remains green.
- [ ] Verify no unrelated donor source, assets, or dependencies entered the diff.
- [ ] Bind evidence to the exact candidate head.

After merge:

- [ ] Confirm required main-branch CI gates pass on the merge commit.
- [ ] Preserve post-merge evidence before advancing canonical task state.

## Prohibited shortcuts

Do not:

- import source first and reconstruct provenance later;
- treat an SPDX identifier as a replacement for preserved license text;
- treat founder permission as permission for embedded third-party material;
- copy donor branding or hosted-service configuration by default;
- bypass the notice inventory because no release has happened yet;
- mark characterization or security review complete without evidence;
- weaken CI, Diffcipline, or provenance validation to admit an import;
- claim a donor import is complete from an agent's self-report.

## Completion rule

A donor import is eligible for canonical completion only when its exact source, authorization, notices, dependency/asset review, mappings, characterization, applicable security review, import history, and exact-head verification are all inspectable.

This checklist does not authorize an import by itself. The active canonical task or Grain must explicitly authorize the bounded donor source change.
