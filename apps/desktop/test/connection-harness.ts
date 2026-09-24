/**
 * Shared deterministic connection-test harness (SG-000032 PR-B).
 *
 * Manual clock, scripted and deferred handshake attempts, and a microtask
 * flush for controller settlement. Synthetic data only. The controller and
 * corpus suites share these fakes so duplicate, stale, and timing races
 * are proven against one consistent fixture style.
 */
import assert from "node:assert/strict";
import type { AttemptOutcome } from "../src/reconnect-controller.js";

/** One pending manual timer. */
export interface FakeTimer {
  readonly id: number;
  readonly delayMs: number;
  readonly callback: () => void;
  cancelled: boolean;
}

/** Manual clock context with inspection and firing primitives. */
export interface FakeClockContext {
  readonly clock: {
    readonly setTimeout: (callback: () => void, delayMs: number) => unknown;
    readonly clearTimeout: (handle: unknown) => void;
  };
  readonly pending: () => FakeTimer[];
  readonly fireNext: () => void;
}

/** Create one manual clock; timers fire only when the test fires them. */
export function createFakeClock(): FakeClockContext {
  const timers = new Map<number, FakeTimer>();
  let sequence = 0;
  const pending = (): FakeTimer[] =>
    [...timers.values()].filter((entry) => !entry.cancelled).sort((a, b) => a.id - b.id);
  return {
    clock: {
      setTimeout: (callback: () => void, delayMs: number): unknown => {
        sequence += 1;
        timers.set(sequence, { id: sequence, delayMs, callback, cancelled: false });
        return sequence;
      },
      clearTimeout: (handle: unknown): void => {
        const entry = timers.get(handle as number);
        if (entry !== undefined) {
          entry.cancelled = true;
          timers.delete(handle as number);
        }
      },
    },
    pending,
    fireNext: (): void => {
      const next = pending()[0];
      assert.ok(next !== undefined, "no pending timer to fire");
      timers.delete(next.id);
      next.callback();
    },
  };
}

/** Scripted attempt queue; exhausts to `failed` once the script is consumed. */
export function createScriptedAttempt(script: AttemptOutcome[]): {
  readonly attempt: () => Promise<AttemptOutcome>;
  readonly calls: () => number;
} {
  let calls = 0;
  return {
    attempt: (): Promise<AttemptOutcome> => {
      calls += 1;
      const outcome: AttemptOutcome = script[calls - 1] ?? { kind: "failed" };
      return Promise.resolve(outcome);
    },
    calls: () => calls,
  };
}

/** Deferred attempt with manual resolution and in-flight accounting. */
export function createDeferredAttempt(): {
  readonly attempt: () => Promise<AttemptOutcome>;
  readonly calls: () => number;
  readonly resolveNext: (outcome: AttemptOutcome) => void;
} {
  let calls = 0;
  const waiting: Array<(outcome: AttemptOutcome) => void> = [];
  return {
    attempt: (): Promise<AttemptOutcome> => {
      calls += 1;
      return new Promise<AttemptOutcome>((resolve) => {
        waiting.push(resolve);
      });
    },
    calls: () => calls,
    resolveNext: (outcome: AttemptOutcome): void => {
      const resolve = waiting.shift();
      assert.ok(resolve !== undefined, "no in-flight attempt to resolve");
      resolve(outcome);
    },
  };
}

/** Flush microtasks and macrotasks so controller promises settle. */
export async function settle(): Promise<void> {
  await Promise.resolve();
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}
