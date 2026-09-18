# Diffcipline governance

Kernux uses Diffcipline as the deterministic exact-diff proof layer at the point a bounded Grain claims completion.

## Immutable authority

Kernux integrates the immutable public Diffcipline v1.0.0 release:

- repository: `TheHalfMoon/Diffcipline`;
- tag: `v1.0.0`;
- exact release commit: `5cb1c77340b75649f6168e0e8f66479ea047ea96`;
- license: MIT.

GitHub resolves `v1.0.0` to that exact commit. Kernux CI pins the full commit rather than the moving tag or `main`.

The v1 proof contract identifies:

- `PASS / 0`: every configured hard requirement observed and satisfied;
- `REVIEW / 1`: no hard failure, but judgment or missing evidence remains;
- `FAIL / 2`: a hard requirement or executed verification failed;
- `64`: usage/execution error; no proof verdict is implied.

A configured command that was not run is `NOT RUN`, never `PASS`.

## Authority boundary

Diffcipline is necessary evidence, not universal completion authority.

- `specs/CURRENT.md` authorizes the active macro frontier.
- SpecGrain defines the bounded execution packet.
- `.diffcipline.toml` defines deterministic repository proof policy.
- CI and local Diffcipline runs observe exact repository facts.
- Task-specific security, compatibility, recovery, platform, and external evidence remain required when the Grain or canonical phase requires them.
- An agent cannot convert `REVIEW`, `FAIL`, `NOT RUN`, or an unavailable check into `PASS`.

## Repository policy

Kernux starts with a conservative small-repository policy:

- maximum changed files: 12;
- maximum added lines: 600;
- dependency-manifest changes: REVIEW;
- lockfile changes: REVIEW;
- untracked files: FAIL.

These limits are review boundaries, not correctness metrics. A larger legitimate change must be refined or explicitly reviewed; code must never be split in a way that makes correctness, security, accessibility, or maintainability worse merely to satisfy a size threshold.

## Risk profiles

Diffcipline v1 risk profiles are profile-specific. Selecting `R2` does not automatically execute `R1`; every configured profile therefore names a complete deterministic baseline.

| Profile | Kernux deterministic baseline | Additional completion evidence |
| --- | --- | --- |
| R0 | formatting + SpecGrain state | syntax/build evidence when applicable |
| R1 | full `pnpm check` | focused behavior/regression evidence when applicable |
| R2 | full `pnpm check` | dependent-surface and compatibility/contract evidence |
| R3 | full `pnpm check` | R2 evidence plus adversarial/negative paths and explicit review |

The identical full repository baseline for R1-R3 is intentional: Diffcipline provides the common exact-diff floor. Stronger task-specific evidence is owned by the Grain and canonical phase rather than being falsely represented by a generic command.

## CI gate

The CI workflow:

1. checks out full Git history;
2. installs the pinned Kernux Node/pnpm dependency state;
3. invokes `TheHalfMoon/Diffcipline` at exact commit `5cb1c77340b75649f6168e0e8f66479ea047ea96`;
4. passes the exact pull-request base SHA or previous `main` push SHA;
5. requests risk `R1` for the always-on repository baseline;
6. sets `run-verification: "true"`.

The always-on R1 job is the minimum repository gate. A task whose canonical risk is R2 or R3 must additionally produce a Diffcipline proof for its declared profile and the extra evidence required by that risk class before it can become `PROVEN`.

## Local proof

Use the immutable v1 CLI, not a floating source revision. One deterministic approach is:

```bash
cargo install \
  --git https://github.com/TheHalfMoon/Diffcipline \
  --rev 5cb1c77340b75649f6168e0e8f66479ea047ea96 \
  --locked \
  diffcipline
```

Inspecting without execution cannot pass:

```bash
diffcipline check --base origin/main --risk R1
```

Execute the declared verification:

```bash
diffcipline check --base origin/main --risk R1 --run
```

For machine-readable proof:

```bash
diffcipline check --base origin/main --risk R1 --run --json
```

## Higher-risk tasks

Before closing an R2 or R3 Grain, run its declared profile explicitly. Do not infer that the always-on R1 CI job proves a higher-risk task.

Examples:

```bash
diffcipline check --base origin/main --risk R2 --run
diffcipline check --base origin/main --risk R3 --run
```

The resulting Diffcipline PASS remains only one evidence class. R3 security review, adversarial tests, platform evidence, or external qualification cannot be replaced by the generic proof card.

## Policy changes

A policy change is itself governed work. It must not:

- weaken a required gate merely to make a failing task pass;
- replace an exact release pin with a moving branch/tag;
- convert REVIEW/FAIL conditions into ALLOW without documented authority;
- add repository-controlled commands from untrusted runtime content;
- make provider/model output verification authority.

When repository capabilities grow, update profiles deliberately so the deterministic baseline remains representative without pretending it covers task-specific proof.

## Recovery

The Diffcipline integration is repository governance only. Recovery is a normal revert of the bounded adoption change. It creates no Kernux product runtime state or user data.
