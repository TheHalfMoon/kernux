/**
 * Closed nine-state daemon connection vocabulary (SG-000032 PR-A).
 *
 * Pure deterministic transition reducer over a closed event set. Zero
 * imports so the lifecycle truth stays testable without Electron, Node,
 * timers, or sockets. All reconnect timing, socket ownership, and handshake
 * I/O live in the reconnect controller; this module owns only state truth.
 *
 * Frozen vocabulary (exactly nine states):
 * `initial`, `connecting`, `serving`, `transient-disconnect`,
 * `reconnecting`, `unavailable`, `incompatible`, `degraded`, `terminal`.
 *
 * Frozen corrections preserved:
 * - first-attempt degraded: a non-serving or failed first handshake from
 *   `connecting` projects `degraded`, never a reconnect storm;
 * - degraded recovery: `degraded` reaches `serving` only through a fresh
 *   valid handshake (`explicit-retry` schedules the attempt, the success
 *   event completes it);
 * - unavailable retrigger: `unavailable` leaves only via `explicit-retry`
 *   (new episode, budget reset by the controller) or `dispose`;
 * - disposal-terminal: `dispose` reaches `terminal` from any state, and
 *   `terminal` has no outgoing transitions (no resurrection).
 *
 * Transport connection alone never projects serving: there is no event that
 * moves any state to `serving` except `handshake-success-serving`, which the
 * controller emits only after a fresh valid handshake and compatibility
 * check on a fresh socket.
 */

/** Exact frozen nine-state connection vocabulary. */
export const CONNECTION_STATES = [
  "initial",
  "connecting",
  "serving",
  "transient-disconnect",
  "reconnecting",
  "unavailable",
  "incompatible",
  "degraded",
  "terminal",
] as const;

Object.freeze(CONNECTION_STATES);

/** One frozen connection state. */
export type ConnectionLifecycleState = (typeof CONNECTION_STATES)[number];

/** Closed connection event set. No other event may drive the reducer. */
export const CONNECTION_EVENTS = [
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
] as const;

Object.freeze(CONNECTION_EVENTS);

/** One member of the closed connection event set. */
export type ConnectionLifecycleEvent = (typeof CONNECTION_EVENTS)[number];

/**
 * Whether a fresh valid handshake has completed in this state. Only
 * `serving` (healthy serving daemon) and `degraded` (reachable daemon with
 * a valid handshake proving non-serving truth) hold handshake completeness.
 * `incompatible` attempted a handshake but failed compatibility, so it is
 * incomplete by design.
 */
export const HANDSHAKE_COMPLETE: Readonly<Record<ConnectionLifecycleState, boolean>> = {
  initial: false,
  connecting: false,
  serving: true,
  "transient-disconnect": false,
  reconnecting: false,
  unavailable: false,
  incompatible: false,
  degraded: true,
  terminal: false,
};

/**
 * Whether daemon operations may be dispatched in this state. Only `serving`
 * dispatches; every other state fails closed and must surface honest
 * degraded truth instead of issuing daemon work.
 */
export const DISPATCH_ALLOWED: Readonly<Record<ConnectionLifecycleState, boolean>> = {
  initial: false,
  connecting: false,
  serving: true,
  "transient-disconnect": false,
  reconnecting: false,
  unavailable: false,
  incompatible: false,
  degraded: false,
  terminal: false,
};

/**
 * Whether a reconnect timer is expected to be armed in this state.
 * `transient-disconnect` holds the 200 ms debounce; `reconnecting` holds
 * the exponential backoff. All other states must have no reconnect timer.
 */
export const RECONNECT_SCHEDULED: Readonly<Record<ConnectionLifecycleState, boolean>> = {
  initial: false,
  connecting: false,
  serving: false,
  "transient-disconnect": true,
  reconnecting: true,
  unavailable: false,
  incompatible: false,
  degraded: false,
  terminal: false,
};

/**
 * Whether this state can still reach `serving` without disposal.
 * `terminal` is the only irreversible state.
 */
export const RECOVERABLE: Readonly<Record<ConnectionLifecycleState, boolean>> = {
  initial: true,
  connecting: true,
  serving: true,
  "transient-disconnect": true,
  reconnecting: true,
  unavailable: true,
  incompatible: true,
  degraded: true,
  terminal: false,
};

/** Honest user-visible meaning per state; no state claims serving falsely. */
export const USER_VISIBLE_MEANING: Readonly<Record<ConnectionLifecycleState, string>> = {
  initial: "Not connected yet",
  connecting: "Connecting to the local daemon",
  serving: "Connected to the local daemon",
  "transient-disconnect": "Connection interrupted; retrying",
  reconnecting: "Reconnecting to the local daemon",
  unavailable: "Daemon unavailable after repeated retries",
  incompatible: "Daemon version incompatible",
  degraded: "Daemon reachable but not serving",
  terminal: "Connection closed",
};

/** Whether a value is a member of the frozen nine-state vocabulary. */
export function isConnectionLifecycleState(value: string): value is ConnectionLifecycleState {
  return (CONNECTION_STATES as readonly string[]).includes(value);
}

/** Whether a value is a member of the closed connection event set. */
export function isConnectionLifecycleEvent(value: string): value is ConnectionLifecycleEvent {
  return (CONNECTION_EVENTS as readonly string[]).includes(value);
}

/**
 * Pure deterministic transition reducer. Total over the state/event cross
 * product: forbidden transitions return the current state unchanged (fail
 * closed, never throw, never invent an escape). `terminal` is absorbing:
 * every event maps to `terminal`.
 */
export function transitionConnectionState(
  state: ConnectionLifecycleState,
  event: ConnectionLifecycleEvent,
): ConnectionLifecycleState {
  if (state === "terminal") {
    return "terminal";
  }
  if (event === "dispose") {
    return "terminal";
  }
  switch (state) {
    case "initial": {
      return event === "start-connect" ? "connecting" : "initial";
    }
    case "connecting": {
      switch (event) {
        case "handshake-success-serving":
          return "serving";
        case "handshake-success-degraded":
          return "degraded";
        case "handshake-failure":
        case "connection-lost":
          return "degraded";
        case "handshake-contract-mismatch":
          return "incompatible";
        default:
          return "connecting";
      }
    }
    case "serving": {
      switch (event) {
        case "connection-lost":
        case "handshake-failure":
          return "transient-disconnect";
        case "handshake-contract-mismatch":
          return "incompatible";
        case "handshake-success-serving":
        case "handshake-success-degraded":
        case "start-connect":
        case "debounce-timer-fired":
        case "reconnect-budget-exhausted":
        case "explicit-retry":
          return "serving";
        default:
          return "serving";
      }
    }
    case "transient-disconnect": {
      switch (event) {
        case "debounce-timer-fired":
        case "explicit-retry":
          return "reconnecting";
        case "handshake-success-serving":
          return "serving";
        case "handshake-success-degraded":
          return "degraded";
        case "handshake-contract-mismatch":
          return "incompatible";
        default:
          return "transient-disconnect";
      }
    }
    case "reconnecting": {
      switch (event) {
        case "handshake-success-serving":
          return "serving";
        case "handshake-success-degraded":
          return "degraded";
        case "handshake-contract-mismatch":
          return "incompatible";
        case "reconnect-budget-exhausted":
          return "unavailable";
        default:
          return "reconnecting";
      }
    }
    case "degraded": {
      switch (event) {
        case "explicit-retry":
          return "reconnecting";
        case "handshake-success-serving":
          return "serving";
        case "handshake-success-degraded":
          return "degraded";
        case "handshake-contract-mismatch":
          return "incompatible";
        case "connection-lost":
          return "transient-disconnect";
        default:
          return "degraded";
      }
    }
    case "unavailable": {
      return event === "explicit-retry" ? "reconnecting" : "unavailable";
    }
    case "incompatible": {
      return event === "explicit-retry" ? "connecting" : "incompatible";
    }
  }
}
