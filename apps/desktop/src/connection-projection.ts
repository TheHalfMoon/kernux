/**
 * Single connection authority projection (SG-000032 PR-C).
 *
 * The reconnect controller snapshot is the sole connection truth. This
 * module derives the renderer-facing projection mechanically: a stable
 * status line, a stable error category, and dispatch permission. The
 * renderer creates no second boolean authority; it renders exactly what
 * this projection returns. Pure module with zero imports beyond types.
 *
 * Stable error categories (closed set, constant strings only):
 * - `none`: serving or transient working states with no terminal error;
 * - `daemon-unreachable`: degraded or transient loss without handshake truth;
 * - `daemon-unavailable`: budget exhausted, explicit retry required;
 * - `daemon-incompatible`: version negotiation failed, upgrade required;
 * - `connection-closed`: terminal after disposal, no resurrection.
 *
 * No secrets, nonces, paths, request identifiers, or socket details ever
 * appear in projected strings.
 */

import type { ConnectionLifecycleState } from "./connection-state.js";
import { DISPATCH_ALLOWED } from "./connection-state.js";
import type { ReconnectSnapshot } from "./reconnect-controller.js";

/** Closed renderer-facing error category set. */
export const CONNECTION_ERROR_CATEGORIES = [
  "none",
  "daemon-unreachable",
  "daemon-unavailable",
  "daemon-incompatible",
  "connection-closed",
] as const;

Object.freeze(CONNECTION_ERROR_CATEGORIES);

/** One stable renderer-facing error category. */
export type ConnectionErrorCategory = (typeof CONNECTION_ERROR_CATEGORIES)[number];

/** Renderer-facing connection projection from the single authority. */
export interface ConnectionProjection {
  readonly state: ConnectionLifecycleState;
  readonly statusLine: string;
  readonly error: ConnectionErrorCategory;
  readonly mayDispatch: boolean;
  readonly needsExplicitRetry: boolean;
  readonly attempt: number;
}

/** Whether a value is a stable connection error category. */
export function isConnectionErrorCategory(value: string): value is ConnectionErrorCategory {
  return (CONNECTION_ERROR_CATEGORIES as readonly string[]).includes(value);
}

/**
 * Derive the renderer-facing projection from one controller snapshot.
 * Mechanical mapping only: no socket access, no handshake, no second truth.
 */
export function projectConnection(snapshot: ReconnectSnapshot): ConnectionProjection {
  const state = snapshot.state;
  switch (state) {
    case "initial":
      return {
        state, statusLine: "Not connected yet", error: "none",
        mayDispatch: false, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
    case "connecting":
      return {
        state, statusLine: "Connecting to the local daemon", error: "none",
        mayDispatch: false, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
    case "serving":
      return {
        state, statusLine: "Connected to the local daemon", error: "none",
        mayDispatch: true, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
    case "transient-disconnect":
      return {
        state, statusLine: "Connection interrupted; retrying", error: "daemon-unreachable",
        mayDispatch: false, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
    case "reconnecting":
      return {
        state, statusLine: "Reconnecting to the local daemon", error: "daemon-unreachable",
        mayDispatch: false, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
    case "degraded":
      return {
        state, statusLine: "Daemon reachable but not serving", error: "daemon-unreachable",
        mayDispatch: false, needsExplicitRetry: true, attempt: snapshot.attempt,
      };
    case "unavailable":
      return {
        state, statusLine: "Daemon unavailable after repeated retries", error: "daemon-unavailable",
        mayDispatch: false, needsExplicitRetry: true, attempt: snapshot.attempt,
      };
    case "incompatible":
      return {
        state, statusLine: "Daemon version incompatible", error: "daemon-incompatible",
        mayDispatch: false, needsExplicitRetry: true, attempt: snapshot.attempt,
      };
    case "terminal":
      return {
        state, statusLine: "Connection closed", error: "connection-closed",
        mayDispatch: false, needsExplicitRetry: false, attempt: snapshot.attempt,
      };
  }
}

/**
 * Whether daemon work may be dispatched under this projection. Mirrors the
 * frozen reducer dispatch table exactly; projection never widens authority.
 */
export function projectionMayDispatch(projection: ConnectionProjection): boolean {
  return projection.mayDispatch && DISPATCH_ALLOWED[projection.state];
}
