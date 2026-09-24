/**
 * Connection adversarial corpus (SG-000032 PR-D). Synthetic data only.
 *
 * End-to-end invariant proof across the reducer and the bounded controller:
 * recovery journeys, budget exhaustion and episode resets, disposal races,
 * duplicate and concurrent triggers, and stale-generation rejection. Every
 * case drives the real controller through the shared manual-clock harness.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { projectConnection } from "../src/connection-projection.js";
import { backoffDelayMs, createReconnectController } from "../src/reconnect-controller.js";
import {
  createDeferredAttempt,
  createFakeClock,
  createScriptedAttempt,
  settle,
} from "./connection-harness.js";

describe("happy-path and recovery journeys", () => {
  it("connects, handshakes, and serves", async () => {
    const fake = createFakeClock();
    const controller = createReconnectController({
      attempt: createScriptedAttempt([{ kind: "serving" }]).attempt,
      clock: fake.clock,
    });
    controller.start();
    await settle();
    const projection = projectConnection(controller.getSnapshot());
    assert.equal(projection.statusLine, "Connected to the local daemon");
    assert.equal(projection.mayDispatch, true);
  });

  it("recovers from first-attempt failure through degraded retry", async () => {
    const fake = createFakeClock();
    const controller = createReconnectController({
      attempt: createScriptedAttempt([{ kind: "failed" }, { kind: "serving" }]).attempt,
      clock: fake.clock,
    });
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    controller.notifyRetry();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
  });

  it("reconnects after serving disconnect without stale serving", async () => {
    const fake = createFakeClock();
    const controller = createReconnectController({
      attempt: createScriptedAttempt([{ kind: "serving" }, { kind: "serving" }]).attempt,
      clock: fake.clock,
    });
    controller.start();
    await settle();
    controller.notifyConnectionLost();
    assert.equal(controller.getSnapshot().state, "transient-disconnect");
    assert.equal(projectConnection(controller.getSnapshot()).mayDispatch, false);
    fake.fireNext();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
    assert.equal(projectConnection(controller.getSnapshot()).mayDispatch, true);
  });

  it("fails a full degraded episode into unavailable with retry still required", async () => {
    const fake = createFakeClock();
    const controller = createReconnectController({
      attempt: createScriptedAttempt([{ kind: "failed" }]).attempt,
      clock: fake.clock,
    });
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    controller.notifyRetry();
    await settle();
    for (let round = 0; round < 10; round += 1) {
      if (fake.pending().length > 0) {
        fake.fireNext();
      }
      await settle();
    }
    assert.equal(controller.getSnapshot().state, "unavailable");
    assert.equal(projectConnection(controller.getSnapshot()).needsExplicitRetry, true);
  });
});

describe("budget exhaustion and episode resets", () => {
  it("exhausts ten attempts per episode into unavailable", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "serving" }, { kind: "failed" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    controller.notifyConnectionLost();
    for (let round = 0; round < 10; round += 1) {
      if (fake.pending().length > 0) {
        fake.fireNext();
      }
      await settle();
    }
    assert.equal(scripted.calls(), 11);
    assert.equal(controller.getSnapshot().state, "unavailable");
    assert.equal(fake.pending().length, 0);
  });

  it("recovers from degraded and resets the episode budget on serving", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([
      { kind: "failed" },
      { kind: "serving" },
      { kind: "failed" },
      { kind: "serving" },
    ]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    controller.notifyRetry();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
    assert.equal(controller.getSnapshot().attempt, 0);
    controller.notifyConnectionLost();
    fake.fireNext();
    await settle();
    assert.equal(controller.getSnapshot().state, "reconnecting");
    assert.equal(controller.getSnapshot().attempt, 1);
    fake.fireNext();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
    assert.equal(scripted.calls(), 4);
  });

  it("retriggers from unavailable with a fresh budget", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([
      { kind: "serving" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "failed" },
      { kind: "serving" },
    ]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    controller.notifyConnectionLost();
    for (let round = 0; round < 10; round += 1) {
      if (fake.pending().length > 0) {
        fake.fireNext();
      }
      await settle();
    }
    assert.equal(controller.getSnapshot().state, "unavailable");
    controller.notifyRetry();
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
  });

  it("caps backoff at 5000 ms across a full ten-attempt episode", () => {
    const delays = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].map((failures) => backoffDelayMs(failures));
    assert.deepEqual(delays, [200, 400, 800, 1600, 3200, 5000, 5000, 5000, 5000, 5000]);
    assert.ok(Math.max(...delays) <= 5000);
  });
});

describe("disposal races", () => {
  it("disposes from every reachable non-terminal state to terminal", async () => {
    const targets = ["connecting", "serving", "transient-disconnect", "reconnecting", "degraded"] as const;
    for (const target of targets) {
      const fake = createFakeClock();
      const gate = createDeferredAttempt();
      const controller = createReconnectController({ attempt: gate.attempt, clock: fake.clock });
      controller.start();
      if (target === "serving") {
        gate.resolveNext({ kind: "serving" });
        await settle();
      } else if (target === "degraded") {
        gate.resolveNext({ kind: "failed" });
        await settle();
      } else if (target === "transient-disconnect") {
        gate.resolveNext({ kind: "serving" });
        await settle();
        controller.notifyConnectionLost();
      } else if (target === "reconnecting") {
        gate.resolveNext({ kind: "serving" });
        await settle();
        controller.notifyConnectionLost();
        fake.fireNext();
      }
      assert.equal(controller.getSnapshot().state, target, target);
      controller.dispose();
      assert.equal(controller.getSnapshot().state, "terminal", target);
      assert.equal(fake.pending().length, 0, target);
    }
  });

  it("disposes during backoff with zero post-disposal activity", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "serving" }, { kind: "failed" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    controller.start();
    await settle();
    controller.notifyConnectionLost();
    fake.fireNext();
    await settle();
    assert.equal(controller.getSnapshot().state, "reconnecting");
    assert.equal(fake.pending().length, 1);
    controller.dispose();
    assert.equal(controller.getSnapshot().state, "terminal");
    assert.equal(fake.pending().length, 0);
    controller.notifyRetry();
    controller.notifyConnectionLost();
    controller.start();
    await settle();
    assert.equal(controller.getSnapshot().state, "terminal");
    assert.equal(scripted.calls(), 2);
  });

  it("disposes during connect with late success suppressed", async () => {
    const fake = createFakeClock();
    const deferred = createDeferredAttempt();
    const controller = createReconnectController({ attempt: deferred.attempt, clock: fake.clock });
    controller.start();
    controller.notifyConnectionLost();
    assert.equal(controller.getSnapshot().state, "degraded");
    controller.dispose();
    deferred.resolveNext({ kind: "serving" });
    await settle();
    assert.equal(controller.getSnapshot().state, "terminal");
    assert.equal(deferred.calls(), 1);
  });

  it("suppresses a late successful handshake from a stale generation", async () => {
    const fake = createFakeClock();
    const gate = createDeferredAttempt();
    const controller = createReconnectController({ attempt: gate.attempt, clock: fake.clock });
    controller.start();
    controller.dispose();
    gate.resolveNext({ kind: "serving" });
    await settle();
    assert.equal(controller.getSnapshot().state, "terminal");
    assert.equal(projectConnection(controller.getSnapshot()).error, "connection-closed");
  });
});

describe("duplicate, concurrent, and ordering races", () => {
  it("absorbs multiple disconnect notifications into one debounce", async () => {
    const fake = createFakeClock();
    const gate = createDeferredAttempt();
    const controller = createReconnectController({ attempt: gate.attempt, clock: fake.clock });
    controller.start();
    gate.resolveNext({ kind: "serving" });
    await settle();
    controller.notifyConnectionLost();
    controller.notifyConnectionLost();
    controller.notifyConnectionLost();
    assert.equal(fake.pending().length, 1);
    fake.fireNext();
    assert.equal(controller.getSnapshot().state, "reconnecting");
  });

  it("coalesces concurrent retry requests into one in-flight attempt", async () => {
    const fake = createFakeClock();
    const gate = createDeferredAttempt();
    const controller = createReconnectController({ attempt: gate.attempt, clock: fake.clock });
    controller.start();
    gate.resolveNext({ kind: "failed" });
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    controller.notifyRetry();
    controller.notifyRetry();
    controller.notifyRetry();
    await settle();
    assert.equal(controller.getSnapshot().state, "reconnecting");
    assert.equal(gate.calls(), 2);
  });

  it("ignores retry while an attempt is in flight", async () => {
    const fake = createFakeClock();
    const deferred = createDeferredAttempt();
    const controller = createReconnectController({ attempt: deferred.attempt, clock: fake.clock });
    controller.start();
    deferred.resolveNext({ kind: "serving" });
    await settle();
    controller.notifyConnectionLost();
    fake.fireNext();
    assert.equal(deferred.calls(), 2);
    controller.notifyRetry();
    controller.notifyRetry();
    assert.equal(deferred.calls(), 2);
    deferred.resolveNext({ kind: "serving" });
    await settle();
    assert.equal(controller.getSnapshot().state, "serving");
  });

  it("disconnects during handshake and stays bounded without a storm", async () => {
    const fake = createFakeClock();
    const gate = createDeferredAttempt();
    const controller = createReconnectController({ attempt: gate.attempt, clock: fake.clock });
    controller.start();
    controller.notifyConnectionLost();
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    gate.resolveNext({ kind: "failed" });
    await settle();
    assert.equal(controller.getSnapshot().state, "degraded");
    assert.equal(fake.pending().length, 0);
  });

  it("supports listener unsubscribe without leaking notifications", async () => {
    const fake = createFakeClock();
    const scripted = createScriptedAttempt([{ kind: "serving" }]);
    const controller = createReconnectController({ attempt: scripted.attempt, clock: fake.clock });
    let notifications = 0;
    const unsubscribe = controller.onChange(() => {
      notifications += 1;
    });
    controller.start();
    await settle();
    assert.ok(notifications > 0);
    unsubscribe();
    controller.notifyConnectionLost();
    const frozen = notifications;
    await settle();
    assert.equal(notifications, frozen);
  });
});
