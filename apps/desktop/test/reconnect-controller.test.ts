/**
 * Reconnect controller unit tests (SG-000032 PR-B). Synthetic data only.
 * Proves frozen bounds, immediate first attempts, first-attempt degraded
 * honesty, duplicate-start coalescing, and the 200 ms loss debounce with
 * the shared manual-clock harness. Journeys, budget exhaustion, disposal
 * races, and stale-generation rejection are proven in the PR-D corpus.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  RECONNECT_ATTEMPT_BUDGET,
  RECONNECT_BACKOFF_BASE_MS,
  RECONNECT_BACKOFF_CEILING_MS,
  RECONNECT_BACKOFF_MULTIPLIER,
  RECONNECT_DEBOUNCE_MS,
  backoffDelayMs,
  createReconnectController,
} from "../src/reconnect-controller.js";
import {
  createDeferredAttempt,
  createFakeClock,
  createScriptedAttempt,
  settle,
} from "./connection-harness.js";

describe("frozen reconnect bounds", () => {
  it("pins debounce, backoff, ceiling, and budget constants", () => {
    assert.equal(RECONNECT_DEBOUNCE_MS, 200);
    assert.equal(RECONNECT_BACKOFF_BASE_MS, 200);
    assert.equal(RECONNECT_BACKOFF_MULTIPLIER, 2);
    assert.equal(RECONNECT_BACKOFF_CEILING_MS, 5000);
    assert.equal(RECONNECT_ATTEMPT_BUDGET, 10);
  });

  it("computes exponential backoff capped at the ceiling", () => {
    assert.deepEqual(
      [0, 1, 2, 3, 4, 5, 6, 9].map((failures) => backoffDelayMs(failures)),
      [200, 400, 800, 1600, 3200, 5000, 5000, 5000],
    );
  });
});

describe("first-attempt behavior", () => {
  it("attempts immediately on start and projects serving on success", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "serving" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
    assert.equal(scripted.calls(), 1);
    assert.equal(fake.pending().length, 0);
    assert.equal(controller.getSnapshot().attempt, 0);
  });

  it("projects degraded on first-attempt failure without a reconnect storm", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "failed" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    assert.equal(scripted.calls(), 1);
    assert.equal(fake.pending().length, 0);
  });

  it("coalesces duplicate starts into a single attempt", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "serving" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    controller.start();
    controller.start();
    await settle();
    assert.equal(scripted.calls(), 1);
    assert.equal(controller.getSnapshot().state, "serving");
  });
});

describe("loss debounce", () => {
  it("holds transient-disconnect for 200 ms before reconnecting", async () => {
    const fake = createFakeClock();
    const deferred = createDeferredAttempt();
    const controller = createReconnectController({
      attempt: deferred.attempt,
      clock: fake.clock,
    });
    controller.start();
    deferred.resolveNext({ kind: "serving" });
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
    controller.notifyConnectionLost();
    assert.equal(controller.getSnapshot().state, "transient-disconnect");
    assert.equal(fake.pending().length, 1);
    assert.equal(fake.pending()[0]?.delayMs, 200);
    fake.fireNext();
    assert.equal(controller.getSnapshot().state, "reconnecting");
    assert.equal(deferred.calls(), 2);
  });
});
