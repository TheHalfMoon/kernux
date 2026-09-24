/**
 * Connection projection tests (SG-000032 PR-C). Synthetic data only.
 * Proves the renderer-facing projection is a mechanical derivation of the
 * single connection authority with stable categories and no second truth.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  CONNECTION_ERROR_CATEGORIES,
  isConnectionErrorCategory,
  projectConnection,
  projectionMayDispatch,
} from "../src/connection-projection.js";
import { CONNECTION_STATES, type ConnectionLifecycleState } from "../src/connection-state.js";
import type { ReconnectSnapshot } from "../src/reconnect-controller.js";

function snapshotFor(state: ConnectionLifecycleState, attempt = 0): ReconnectSnapshot {
  return {
    state,
    episode: 0,
    attempt,
    inFlight: false,
    timerArmed: false,
    pendingDelayMs: null,
    disposed: false,
  };
}

describe("error categories are closed and stable", () => {
  it("freezes exactly five categories", () => {
    assert.deepEqual([...CONNECTION_ERROR_CATEGORIES], [
      "none",
      "daemon-unreachable",
      "daemon-unavailable",
      "daemon-incompatible",
      "connection-closed",
    ]);
    assert.equal(Object.isFrozen(CONNECTION_ERROR_CATEGORIES), true);
  });

  it("rejects values outside the closed set", () => {
    assert.equal(isConnectionErrorCategory("none"), true);
    assert.equal(isConnectionErrorCategory("timeout"), false);
    assert.equal(isConnectionErrorCategory("ENOENT"), false);
    assert.equal(isConnectionErrorCategory(""), false);
  });
});

describe("projection derives mechanically from the single authority", () => {
  it("covers all nine states with exact status lines", () => {
    const expected: Readonly<Record<ConnectionLifecycleState, string>> = {
      initial: "Not connected yet",
      connecting: "Connecting to the local daemon",
      serving: "Connected to the local daemon",
      "transient-disconnect": "Connection interrupted; retrying",
      reconnecting: "Reconnecting to the local daemon",
      degraded: "Daemon reachable but not serving",
      unavailable: "Daemon unavailable after repeated retries",
      incompatible: "Daemon version incompatible",
      terminal: "Connection closed",
    };
    assert.equal(CONNECTION_STATES.length, 9);
    for (const state of CONNECTION_STATES) {
      const projection = projectConnection(snapshotFor(state, 3));
      assert.equal(projection.state, state);
      assert.equal(projection.statusLine, expected[state], state);
      assert.equal(projection.attempt, 3, state);
    }
  });

  it("permits dispatch only while serving", () => {
    for (const state of CONNECTION_STATES) {
      const projection = projectConnection(snapshotFor(state));
      assert.equal(projection.mayDispatch, state === "serving", state);
      assert.equal(projectionMayDispatch(projection), state === "serving", state);
    }
  });

  it("requests explicit retry only from degraded, unavailable, and incompatible", () => {
    for (const state of CONNECTION_STATES) {
      const projection = projectConnection(snapshotFor(state));
      const expected =
        state === "degraded" || state === "unavailable" || state === "incompatible";
      assert.equal(projection.needsExplicitRetry, expected, state);
    }
  });

  it("maps errors deterministically per state", () => {
    const expected: Readonly<Record<ConnectionLifecycleState, string>> = {
      initial: "none",
      connecting: "none",
      serving: "none",
      "transient-disconnect": "daemon-unreachable",
      reconnecting: "daemon-unreachable",
      degraded: "daemon-unreachable",
      unavailable: "daemon-unavailable",
      incompatible: "daemon-incompatible",
      terminal: "connection-closed",
    };
    for (const state of CONNECTION_STATES) {
      const projection = projectConnection(snapshotFor(state));
      assert.equal(projection.error, expected[state], state);
      assert.equal(isConnectionErrorCategory(projection.error), true, state);
    }
  });

  it("never leaks secrets, paths, or identifiers in projected strings", () => {
    const hostile = "kxa/1 deadbeef /tmp/kernuxd.sock 01890f3a-7b2f-7e55-aa66-6f708192a3b5";
    for (const state of CONNECTION_STATES) {
      const projection = projectConnection(snapshotFor(state));
      assert.ok(!projection.statusLine.includes("kxa/1"), state);
      assert.ok(!projection.statusLine.includes("/tmp"), state);
      assert.ok(!projection.statusLine.includes("deadbeef"), state);
      assert.ok(!projection.error.includes(hostile), state);
    }
  });
});
