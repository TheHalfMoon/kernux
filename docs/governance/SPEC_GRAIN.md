# SpecGrain governance

Kernux uses SpecGrain to turn dependency-eligible macro tasks into bounded, independently verifiable Grains before implementation.

## Pinned authority

Kernux does not install a floating SpecGrain branch or package release.

The repository pins:

- repository: `TheHalfMoon/SpecGrain`;
- commit: `5de7d6499bb0a9e3a191fc0934399cf099d1980a`;
- archive SHA-256: `da6a8c006c3a0154e76e2bf08bcae58bb86e04f06dd7a03115275b466489eef9`;
- minimum Python: 3.11;
- license: MIT.

The pin lives in `tools/specgrain/pin.json`. The wrapper refuses an archive whose SHA-256 differs from the tracked pin.

## Authority boundaries

Kernux deliberately keeps program authority and Grain lifecycle separate.

- `specs/CURRENT.md` owns the active macro frontier.
- `specs/tasks.md` owns dependency order and macro task state.
- `.specgrain/` owns deterministic refinement state for bounded work.
- A SpecGrain `GRAIN` or `next` result does not override the canonical macro frontier.
- A Grain may execute only when its mapped macro task is dependency-eligible and authorized by current repository truth.

This prevents two independent planning systems from silently granting work.

## Repository-local state

Tracked state begins with:

```text
.specgrain/
  project.json
  policies/
    default.json
  specs/
    SG-000001.json
```

Runtime mutation locks and interrupted-authoring journals live under `.specgrain/tmp/` and are ignored by Git.

The initial policy uses `readiness_mode=report`. Readiness reporting therefore does not break unrelated repository checks, while explicit `grain` promotion still fails closed unless the candidate satisfies the current Grain readiness contract.

## Running SpecGrain

The portable launcher is exposed through pnpm:

```bash
pnpm specgrain -- --show-pin
pnpm specgrain -- check .
pnpm specgrain -- next .
```

Direct Python invocation is also supported:

```bash
python3 tools/specgrain/run.py check .
```

On Windows, the Node launcher probes `py -3` and `python`; on POSIX systems it probes `python3` and `python`. `PYTHON` may explicitly select an interpreter.

The Python runner uses only the standard library. On first use it downloads the exact pinned archive over verified HTTPS into `.cache/specgrain/<commit>/`, verifies the tracked SHA-256, performs bounded path-safe extraction, and runs SpecGrain directly from the verified source tree. Kernux does not vendor SpecGrain source into this repository.

If Python's local CA configuration cannot perform a verified HTTPS download, the runner may fall back to system `curl` with HTTPS-only transport, TLS verification, and a bounded timeout. It never disables certificate verification.

## Deterministic authoring loop

A normal bounded task follows:

```text
macro task becomes dependency-eligible
  -> draft
  -> shape
  -> refine
  -> check
  -> grain
  -> next
  -> explicit context sources
  -> packet
  -> execute
  -> independent proof
```

Use CLI mutations rather than hand-editing lifecycle state.

Example:

```bash
pnpm specgrain -- draft . \
  --title "KX-... — Bounded outcome" \
  --outcome "One independently verifiable result"
```

```bash
pnpm specgrain -- shape SG-000001 . \
  --scope-in "..." \
  --scope-out "..." \
  --acceptance "..." \
  --risk-level low \
  --recovery "Revert the bounded change." \
  --context-budget 4000 \
  --context-estimate 1500 \
  --change-surface "path/**" \
  --evidence "focused-proof" \
  --minimality-choice reuse-existing \
  --minimality-rationale "..." \
  --safety-status none-identified

pnpm specgrain -- refine SG-000001 .
pnpm specgrain -- check .
pnpm specgrain -- grain SG-000001 .
pnpm specgrain -- next .
```

## Mapping Kernux tasks to Grains

The SpecNode title begins with the canonical Kernux macro task ID, for example:

`KX-P00-S03-T01 — Activate Diffcipline proof policy and CI gate`

The macro task remains the program-level dependency identifier. The `SG-XXXXXX` ID is the bounded SpecGrain identity.

Do not create detailed Grains far ahead of the active frontier. Refine only work whose dependencies and current repository evidence are stable enough to make the bounds meaningful.

## Initial Grain

`SG-000001` prepares `KX-P00-S03-T01`, the Diffcipline activation task.

It has been taken through the supported current-source lifecycle:

`DRAFT -> SHAPED -> REFINING -> GRAIN`

At creation time, `specgrain check .` reported the REFINING leaf ready, explicit `grain` promotion succeeded, and `specgrain next .` projected the Grain in wave 1.

That is refinement evidence only. Execution remains blocked until this SpecGrain activation task is proven and canonical project state advances to Diffcipline.

## WorkPackets and context

A WorkPacket must use explicit `ContextSource` records bound to provenance and revisions. Packet export does not discover context, execute an agent, mutate lifecycle state, or confer verification authority.

Context selection should prefer:

1. exact canonical Kernux authority needed by the Grain;
2. exact source files in its declared change surface;
3. exact external source revisions required for the work;
4. the smallest additional evidence necessary to avoid guessing.

## Proof and completion

Agent or executor self-report never proves a Grain.

Kernux completion requires repository-owned evidence bound to the exact implementation revision. Diffcipline becomes the exact-diff proof layer in the next P00 Grain. Higher-risk phases add their own security, compatibility, recovery, and release gates.

## Pin upgrades

Changing the SpecGrain pin is a governed change. An upgrade must:

- reverify upstream live truth;
- pin a full commit;
- recompute and record the archive SHA-256;
- review lifecycle/schema/CLI changes;
- prove existing `.specgrain/` state still validates;
- run the full Kernux CI suite on the exact candidate;
- preserve negative evidence if the upgrade fails.

Never replace the pin with `main`, `latest`, or an unverified archive.
