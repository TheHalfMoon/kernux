/**
 * Bounded reconnect controller (SG-000032 PR-B).
 *
 * Owns all reconnect timing, attempt ownership, and stale-generation
 * protection. State truth stays in the pure reducer
 * (`connection-state.ts`); this module drives it with events and never
 * assigns states directly. Zero Electron/Node imports: the handshake attempt
 * and the clock are injected, so every race is deterministically testable
 * with fakes. The Electron main process supplies a fresh-socket
 * `probeDaemonHealth` attempt per try; tests supply scripted outcomes.
 *
 * Frozen bounds:
 * - debounce 200 ms in `transient-disconnect`;
 * - exponential backoff base 200 ms, multiplier 2x, ceiling 5000 ms;
 * - 10 attempts per episode, exhaustion projects `unavailable`;
 * - successful serving resets the episode (budget restored, new generation);
 * - disposal cancels all timers and suppresses every late callback;
 * - duplicate triggers coalesce to a single timer and single in-flight
 *   attempt (no parallel loops);
 * - every attempt uses a fresh socket and fresh handshake supplied by the
 *   injected attempt function; stale generations are ignored.
 */

import {
  transitionConnectionState,
  type ConnectionLifecycleEvent,
  type ConnectionLifecycleState,
} from "./connection-state.js";

/** Frozen debounce hold in `transient-disconnect` (milliseconds). */
export const RECONNECT_DEBOUNCE_MS = 200;

/** Frozen backoff base delay (milliseconds). */
export const RECONNECT_BACKOFF_BASE_MS = 200;

/** Frozen backoff multiplier. */
export const RECONNECT_BACKOFF_MULTIPLIER = 2;

/** Frozen backoff ceiling (milliseconds). */
export const RECONNECT_BACKOFF_CEILING_MS = 5000;

/** Frozen per-episode attempt budget. */
export const RECONNECT_ATTEMPT_BUDGET = 10;

/** One handshake attempt outcome from the injected attempt function. */
export type AttemptOutcome =
  | { readonly kind: "serving" }
  | { readonly kind: "degraded" }
  | { readonly kind: "incompatible" }
  | { readonly kind: "failed" };

/** Injected handshake attempt over a fresh socket and fresh handshake. */
export type HandshakeAttempt = () => Promise<AttemptOutcome>;

/** Injected clock so tests drive timing deterministically. */
export interface ReconnectClock {
  readonly setTimeout: (callback: () => void, delayMs: number) => unknown;
  readonly clearTimeout: (handle: unknown) => void;
}

/** Immutable controller snapshot for the single connection authority. */
export interface ReconnectSnapshot {
  readonly state: ConnectionLifecycleState;
  readonly episode: number;
  readonly attempt: number;
  readonly inFlight: boolean;
  readonly timerArmed: boolean;
  readonly pendingDelayMs: number | null;
  readonly disposed: boolean;
}

/** Snapshot listener for shell projection. */
export type ReconnectListener = (snapshot: ReconnectSnapshot) => void;

/**
 * Backoff delay before the next attempt after `failedAttempts` consecutive
 * failures in the current episode. Deterministic and capped:
 * `min(200 * 2^failedAttempts, 5000)`.
 */
export function backoffDelayMs(failedAttempts: number): number {
  let delay = RECONNECT_BACKOFF_BASE_MS;
  for (let step = 0; step < failedAttempts; step += 1) {
    delay *= RECONNECT_BACKOFF_MULTIPLIER;
  }
  return delay > RECONNECT_BACKOFF_CEILING_MS ? RECONNECT_BACKOFF_CEILING_MS : delay;
}

/** Options for one reconnect controller. */
export interface ReconnectControllerOptions {
  readonly attempt: HandshakeAttempt;
  readonly clock: ReconnectClock;
}

/** Bounded reconnect controller with generation-guarded async work. */
export interface ReconnectController {
  readonly start: () => void;
  readonly notifyConnectionLost: () => void;
  readonly notifyRetry: () => void;
  readonly dispose: () => void;
  readonly getSnapshot: () => ReconnectSnapshot;
  readonly onChange: (listener: ReconnectListener) => () => void;
}

/** Create one bounded reconnect controller. */
export function createReconnectController(options: ReconnectControllerOptions): ReconnectController {
  const attemptHandshake = options.attempt;
  const clock = options.clock;
  let state: ConnectionLifecycleState = "initial";
  let episode = 0;
  let attempt = 0;
  let failures = 0;
  let inFlight = false;
  let inFlightOp = 0;
  let opSequence = 0;
  let timer: unknown = null;
  let pendingDelayMs: number | null = null;
  let started = false;
  let disposed = false;
  const listeners = new Set<ReconnectListener>();

  function snapshot(): ReconnectSnapshot {
    return {
      state,
      episode,
      attempt,
      inFlight,
      timerArmed: timer !== null,
      pendingDelayMs,
      disposed,
    };
  }

  function emit(): void {
    const current = snapshot();
    for (const listener of [...listeners]) {
      listener(current);
    }
  }

  function clearTimer(): void {
    if (timer !== null) {
      clock.clearTimeout(timer);
      timer = null;
      pendingDelayMs = null;
    }
  }

  function dispatch(event: ConnectionLifecycleEvent): void {
    if (disposed && event !== "dispose") {
      return;
    }
    const next = transitionConnectionState(state, event);
    if (next !== state) {
      state = next;
      emit();
    }
  }

  function armTimer(delayMs: number, callback: () => void): void {
    clearTimer();
    const capturedEpisode = episode;
    pendingDelayMs = delayMs;
    timer = clock.setTimeout(() => {
      timer = null;
      pendingDelayMs = null;
      if (disposed || capturedEpisode !== episode) {
        return;
      }
      callback();
    }, delayMs);
  }

  function beginAttempt(): void {
    if (disposed || inFlight) {
      return;
    }
    if (state !== "connecting" && state !== "reconnecting") {
      return;
    }
    attempt += 1;
    inFlight = true;
    opSequence += 1;
    const capturedOp = opSequence;
    const capturedEpisode = episode;
    inFlightOp = capturedOp;
    emit();
    attemptHandshake().then(
      (outcome) => {
        settleAttempt(capturedEpisode, capturedOp, outcome);
      },
      () => {
        settleAttempt(capturedEpisode, capturedOp, { kind: "failed" });
      },
    );
  }

  function settleAttempt(capturedEpisode: number, capturedOp: number, outcome: AttemptOutcome): void {
    if (disposed || capturedEpisode !== episode || capturedOp !== inFlightOp) {
      return;
    }
    inFlight = false;
    switch (outcome.kind) {
      case "serving": {
        failures = 0;
        attempt = 0;
        episode += 1;
        inFlightOp = 0;
        clearTimer();
        dispatch("handshake-success-serving");
        emit();
        return;
      }
      case "degraded": {
        failures = 0;
        inFlightOp = 0;
        clearTimer();
        dispatch("handshake-success-degraded");
        emit();
        return;
      }
      case "incompatible": {
        failures = 0;
        inFlightOp = 0;
        clearTimer();
        dispatch("handshake-contract-mismatch");
        emit();
        return;
      }
      case "failed": {
        failures += 1;
        if (attempt >= RECONNECT_ATTEMPT_BUDGET) {
          inFlightOp = 0;
          clearTimer();
          dispatch("reconnect-budget-exhausted");
          emit();
          return;
        }
        dispatch("handshake-failure");
        emit();
        if (state === "reconnecting" && !disposed) {
          armTimer(backoffDelayMs(failures), () => {
            dispatch("debounce-timer-fired");
            beginAttempt();
          });
        }
        emit();
        return;
      }
    }
  }

  function ensureDebounce(): void {
    if (disposed || inFlight || timer !== null) {
      return;
    }
    if (state !== "transient-disconnect") {
      return;
    }
    armTimer(RECONNECT_DEBOUNCE_MS, () => {
      dispatch("debounce-timer-fired");
      beginAttempt();
    });
  }

  return {
    start: () => {
      if (disposed || started) {
        return;
      }
      started = true;
      failures = 0;
      attempt = 0;
      dispatch("start-connect");
      beginAttempt();
    },
    notifyConnectionLost: () => {
      if (disposed || !started) {
        return;
      }
      dispatch("connection-lost");
      ensureDebounce();
      emit();
    },
    notifyRetry: () => {
      if (disposed || !started) {
        return;
      }
      const before = state;
      dispatch("explicit-retry");
      if (state === "connecting" && before === "incompatible") {
        episode += 1;
        attempt = 0;
        failures = 0;
        beginAttempt();
        return;
      }
      if (state === "reconnecting" && before === "unavailable") {
        episode += 1;
        attempt = 0;
        failures = 0;
        beginAttempt();
        return;
      }
      if (state === "reconnecting") {
        if (timer !== null || inFlight) {
          return;
        }
        if (before === "transient-disconnect" || before === "degraded") {
          beginAttempt();
          return;
        }
        ensureDebounce();
        return;
      }
      if (state === "connecting") {
        beginAttempt();
      }
    },
    dispose: () => {
      if (disposed) {
        return;
      }
      disposed = true;
      clearTimer();
      listeners.clear();
      const next = transitionConnectionState(state, "dispose");
      state = next;
    },
    getSnapshot: () => snapshot(),
    onChange: (listener: ReconnectListener) => {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}
