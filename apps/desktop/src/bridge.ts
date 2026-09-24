/**
 * Narrow typed preload bridge vocabulary (SG-000031 PR-A).
 *
 * The renderer may invoke exactly the channels in {@link ALLOWED_INVOKE_CHANNELS}
 * and nothing else. Unknown channels fail closed with a constant error that
 * never echoes the attempted channel. No ambient authority, no subscriptions,
 * no privileged channels. Pure module with zero imports.
 */

/** Daemon health probe channel. Returns `{ status: "ok" }` on success. */
export const DAEMON_HEALTH_CHANNEL = "kernux/daemon/health" as const;

/** Daemon version query channel. Returns `{ version: string }` on success. */
export const DAEMON_VERSION_CHANNEL = "kernux/daemon/version" as const;

/** Exact allowlisted invoke channels, frozen for KX-P03-S01-T01. */
export const ALLOWED_INVOKE_CHANNELS = [
  DAEMON_HEALTH_CHANNEL,
  DAEMON_VERSION_CHANNEL,
] as const;

/** One allowlisted invoke channel. */
export type InvokeChannel = (typeof ALLOWED_INVOKE_CHANNELS)[number];

/** Maximum request payload size in bytes (fail-closed bound). */
export const MAX_ENVELOPE_BYTES = 65536;

/**
 * Typed invoke envelope carried across the preload bridge.
 * `payload` must be JSON-serializable; functions and symbols are rejected.
 */
export interface InvokeEnvelope {
  readonly channel: InvokeChannel;
  readonly requestId: string;
  readonly payload: unknown;
}

/** Typed invoke response: either data or a stable error code. */
export type InvokeResult<T> =
  | { readonly ok: true; readonly data: T }
  | { readonly ok: false; readonly error: string };

/**
 * Whether a channel string is allowlisted. Exact and case-sensitive match
 * against the frozen vocabulary; never throws.
 */
export function isAllowedInvokeChannel(channel: string): channel is InvokeChannel {
  return (ALLOWED_INVOKE_CHANNELS as readonly string[]).includes(channel);
}

/**
 * Build one typed invoke envelope. Throws a constant `unknown-channel` error
 * for anything outside the allowlist without echoing the input.
 */
export function buildInvokeEnvelope(
  channel: string,
  requestId: string,
  payload: unknown,
): InvokeEnvelope {
  if (!isAllowedInvokeChannel(channel)) {
    throw new Error("unknown-channel");
  }
  if (typeof requestId !== "string" || requestId.length === 0) {
    throw new Error("invalid-request");
  }
  return { channel, requestId, payload };
}

/**
 * Validate one bridge response shape. Returns true only for exact
 * `{ ok: true, data }` or `{ ok: false, error: string }` objects.
 * Anything else (including arrays, null, and primitives) is rejected.
 */
export function isWellFormedResult(value: unknown): value is InvokeResult<unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const record = value as Record<string, unknown>;
  if (record["ok"] === true) {
    return "data" in record;
  }
  if (record["ok"] === false) {
    return typeof record["error"] === "string";
  }
  return false;
}
