/**
 * Connection lifecycle reducer tests (SG-000032 PR-A). Synthetic data only.
 * Proves the frozen nine-state vocabulary, deterministic transitions, and
 * the four frozen corrections: first-attempt degraded, degraded recovery,
 * unavailable retrigger, and disposal-terminal paths.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  CONNECTION_EVENTS,
  CONNECTION_STATES,
  DISPATCH_ALLOWED,
  HANDSHAKE_COMPLETE,
  RECONNECT_SCHEDULED,
  RECOVERABLE,
  isConnectionLifecycleEvent,
  isConnectionLifecycleState,
  transitionConnectionState,
  type ConnectionLifecycleEvent,
  type ConnectionLifecycleState,
} from "../src/connection-state.js";

const ALL_STATES: readonly ConnectionLifecycleState[] = CONNECTION_STATES;
const ALL_EVENTS: readonly ConnectionLifecycleEvent[] = CONNECTION_EVENTS;

describe("frozen vocabulary is closed", () => {
  it("holds exactly nine states and ten events", () => {
    assert.deepEqual([...CONNECTION_STATES], [
      "initial",
      "connecting",
      "serving",
      "transient-disconnect",
      "reconnecting",
      "unavailable",
      "incompatible",
      "degraded",
      "terminal",
    ]);
    assert.deepEqual([...CONNECTION_EVENTS], [
      "start-connect",
      "handshake-success-serving",
      "handshake-success-degraded",
      "handshake-contract-mismatch",
      "handshake-failure",
      "connection-lost",
      "debounce-timer-fired",
      "reconnect-budget-exhausted",
      "explicit-retry",
      "dispose",
    ]);
    assert.equal(Object.isFrozen(CONNECTION_STATES), true);
    assert.equal(Object.isFrozen(CONNECTION_EVENTS), true);
  });

  it("rejects values outside the frozen vocabulary", () => {
    assert.equal(isConnectionLifecycleState("connected"), false);
    assert.equal(isConnectionLifecycleState("SERVING"), false);
    assert.equal(isConnectionLifecycleState(""), false);
    assert.equal(isConnectionLifecycleEvent("transport-connected"), false);
    assert.equal(isConnectionLifecycleEvent("reconnect"), false);
    assert.equal(isConnectionLifecycleEvent(""), false);
  });
});

describe("state metadata is honest", () => {
  it("dispatches daemon work only while serving", () => {
    for (const state of ALL_STATES) {
      assert.equal(DISPATCH_ALLOWED[state], state === "serving", state);
    }
  });

  it("marks handshake completeness only for serving and degraded", () => {
    for (const state of ALL_STATES) {
      const expected = state === "serving" || state === "degraded";
      assert.equal(HANDSHAKE_COMPLETE[state], expected, state);
    }
  });

  it("arms reconnect timers only in transient-disconnect and reconnecting", () => {
    for (const state of ALL_STATES) {
      const expected = state === "transient-disconnect" || state === "reconnecting";
      assert.equal(RECONNECT_SCHEDULED[state], expected, state);
    }
  });

  it("treats every state except terminal as recoverable", () => {
    for (const state of ALL_STATES) {
      assert.equal(RECOVERABLE[state], state !== "terminal", state);
    }
  });
});

describe("reducer is total and deterministic", () => {
  it("never throws and always returns a frozen state", () => {
    for (const state of ALL_STATES) {
      for (const event of ALL_EVENTS) {
        const next = transitionConnectionState(state, event);
        assert.equal(isConnectionLifecycleState(next), true, `${state}+${event}`);
        assert.equal(
          transitionConnectionState(state, event),
          next,
          `nondeterministic ${state}+${event}`,
        );
      }
    }
  });

  it("projects serving only after a fresh valid handshake", () => {
    const servingSources: Array<[ConnectionLifecycleState, ConnectionLifecycleEvent]> = [];
    for (const state of ALL_STATES) {
      if (state === "serving") {
        continue;
      }
      for (const event of ALL_EVENTS) {
        if (transitionConnectionState(state, event) === "serving") {
          servingSources.push([state, event]);
        }
      }
    }
    assert.ok(servingSources.length > 0);
    for (const [state, event] of servingSources) {
      assert.equal(event, "handshake-success-serving", `${state}+${event}`);
    }
  });
});

describe("frozen corrections hold", () => {
  it("maps first-attempt failure and non-serving truth to degraded", () => {
    assert.equal(
      transitionConnectionState("connecting", "handshake-success-degraded"),
      "degraded",
    );
    assert.equal(transitionConnectionState("connecting", "handshake-failure"), "degraded");
    assert.equal(transitionConnectionState("connecting", "connection-lost"), "degraded");
  });

  it("recovers from degraded only through a fresh serving handshake", () => {
    assert.equal(transitionConnectionState("degraded", "explicit-retry"), "reconnecting");
    assert.equal(transitionConnectionState("degraded", "handshake-success-serving"), "serving");
    assert.equal(transitionConnectionState("degraded", "handshake-success-degraded"), "degraded");
  });

  it("retriggers from unavailable only via explicit retry", () => {
    assert.equal(transitionConnectionState("unavailable", "explicit-retry"), "reconnecting");
    for (const event of ALL_EVENTS) {
      if (event === "explicit-retry" || event === "dispose") {
        continue;
      }
      assert.equal(transitionConnectionState("unavailable", event), "unavailable", event);
    }
  });

  it("moves to terminal on dispose from every non-terminal state", () => {
    for (const state of ALL_STATES) {
      assert.equal(transitionConnectionState(state, "dispose"), "terminal", state);
    }
  });

  it("keeps terminal absorbing with no resurrection", () => {
    for (const event of ALL_EVENTS) {
      assert.equal(transitionConnectionState("terminal", event), "terminal", event);
    }
  });
});

describe("honest downgrades and duplicate coalescing hold", () => {
  it("downgrades serving loss synchronously without stale serving", () => {
    assert.equal(transitionConnectionState("serving", "connection-lost"), "transient-disconnect");
    assert.equal(transitionConnectionState("serving", "handshake-failure"), "transient-disconnect");
    assert.equal(transitionConnectionState("serving", "handshake-contract-mismatch"), "incompatible");
  });

  it("holds debounce until the timer fires or an explicit retry arrives", () => {
    assert.equal(
      transitionConnectionState("transient-disconnect", "debounce-timer-fired"),
      "reconnecting",
    );
    assert.equal(
      transitionConnectionState("transient-disconnect", "explicit-retry"),
      "reconnecting",
    );
    assert.equal(
      transitionConnectionState("transient-disconnect", "connection-lost"),
      "transient-disconnect",
    );
  });

  it("ignores duplicate and concurrent triggers without parallel loops", () => {
    assert.equal(transitionConnectionState("serving", "explicit-retry"), "serving");
    assert.equal(transitionConnectionState("reconnecting", "explicit-retry"), "reconnecting");
    assert.equal(
      transitionConnectionState("reconnecting", "debounce-timer-fired"),
      "reconnecting",
    );
    assert.equal(transitionConnectionState("initial", "explicit-retry"), "initial");
  });

  it("exhausts the reconnect budget into unavailable", () => {
    assert.equal(
      transitionConnectionState("reconnecting", "reconnect-budget-exhausted"),
      "unavailable",
    );
    assert.equal(
      transitionConnectionState("reconnecting", "handshake-failure"),
      "reconnecting",
    );
  });

  it("retries incompatible shells through a fresh connecting handshake", () => {
    assert.equal(transitionConnectionState("incompatible", "explicit-retry"), "connecting");
    assert.equal(transitionConnectionState("incompatible", "handshake-failure"), "incompatible");
  });
});
