/**
 * Dependency-injected daemon handshake client (SG-000031 PR-B).
 *
 * Mirrors the kernuxd local transport exactly: `kxa/1 <64 lowercase hex>\n`
 * authentication preamble, newline-terminated JSON probe frames bounded at
 * 4096 bytes, and `health_version` probe request/response shapes from the
 * frozen KRP contract. The socket is injected so all logic is unit-testable
 * with zero imports; the Electron main process supplies the node:net adapter.
 * The launch nonce never appears in errors, logs, or responses.
 */

/** Preamble version token: `kxa/1 ` (trailing space included). */
export const AUTH_PREAMBLE_PREFIX = "kxa/1 ";

/** Exact lowercase hexadecimal nonce length in characters. */
export const LAUNCH_NONCE_HEX_LENGTH = 64;

/** Exact complete preamble length in characters: 6 + 64 + 1. */
export const AUTH_PREAMBLE_LENGTH = 71;

/** Maximum JSON frame bytes accepted per request or response. */
export const MAX_FRAME_BYTES = 4096;

/** Frozen KRP contract version for probe frames. */
export const PROBE_CONTRACT_VERSION = "krp/1";

/** Sole probe kind for the shell handshake. */
export const PROBE_KIND = "health_version";

/** Default handshake timeout in milliseconds. */
export const HANDSHAKE_TIMEOUT_MS = 2000;

/** Daemon health states from the frozen contract. */
export type DaemonHealth = "healthy" | "unavailable";

/** Daemon lifecycle states from the frozen contract. */
export type DaemonLifecycle = "starting" | "serving" | "shutting_down" | "stopped";

/** Renderer-facing connection states derived from a probe. */
export type ConnectionState = "connected" | "degraded" | "unknown";

/** Minimal probe request shape mirrored from the KRP contract. */
export interface ProbeRequest {
  readonly contract_version: "krp/1";
  readonly probe: "health_version";
  readonly request_id: string;
}

/** Minimal probe response shape mirrored from the KRP contract. */
export interface ProbeResponse {
  readonly contract_version: string;
  readonly daemon_version: string;
  readonly health: string;
  readonly implementation_revision: string;
  readonly lifecycle_state: string;
  readonly request_id: string;
  readonly schema_sha256: string;
}

/** Renderer-facing daemon status derived from one probe. */
export interface DaemonStatus {
  readonly connection: ConnectionState;
  readonly daemonVersion: string | null;
  readonly lifecycle: DaemonLifecycle | null;
}

/** Dependency-injected newline-frame socket (no node:net import here). */
export interface FrameSocket {
  readonly write: (data: string) => void;
  readonly onData: (listener: (chunk: string) => void) => void;
  readonly onError: (listener: (message: string) => void) => void;
  readonly onClose: (listener: () => void) => void;
  readonly destroy: () => void;
}

/** Whether a value is exactly 64 lowercase hexadecimal characters. */
export function isLaunchNonceHex(value: string): boolean {
  if (value.length !== LAUNCH_NONCE_HEX_LENGTH) {
    return false;
  }
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    const digit = code >= 48 && code <= 57;
    const lower = code >= 97 && code <= 102;
    if (!digit && !lower) {
      return false;
    }
  }
  return true;
}

/**
 * Encode the exact authentication preamble. Throws a constant error for
 * malformed nonces without echoing the input.
 */
export function encodeAuthPreamble(nonceHex: string): string {
  if (!isLaunchNonceHex(nonceHex)) {
    throw new Error("invalid-nonce");
  }
  return `${AUTH_PREAMBLE_PREFIX}${nonceHex}\n`;
}

/** Whether a string is a canonical UUIDv7 (mirrors the kernel validator). */
export function isCanonicalUuidV7(value: string): boolean {
  if (value.length !== 36) {
    return false;
  }
  const bytes = value;
  for (const dash of [8, 13, 18, 23]) {
    const mark: string | undefined = bytes[dash];
    if (mark !== "-") {
      return false;
    }
  }
  const version: string | undefined = bytes[14];
  const variant: string | undefined = bytes[19];
  if (version !== "7" || (variant !== "8" && variant !== "9" && variant !== "a" && variant !== "b")) {
    return false;
  }
  for (let index = 0; index < 36; index += 1) {
    if ([8, 13, 18, 23].includes(index)) {
      continue;
    }
    const code = bytes.charCodeAt(index);
    const digit = code >= 48 && code <= 57;
    const lower = code >= 97 && code <= 102;
    if (!digit && !lower) {
      return false;
    }
  }
  return true;
}

/**
 * Encode one newline-terminated probe frame. Request identifiers must be
 * canonical UUIDv7 so responses can be matched without ambiguity.
 */
export function encodeProbeFrame(requestId: string): string {
  if (!isCanonicalUuidV7(requestId)) {
    throw new Error("invalid-request-id");
  }
  const request: ProbeRequest = {
    contract_version: PROBE_CONTRACT_VERSION,
    probe: PROBE_KIND,
    request_id: requestId,
  };
  const body = JSON.stringify(request);
  if (body.length > MAX_FRAME_BYTES) {
    throw new Error("frame-too-large");
  }
  return `${body}\n`;
}

/**
 * Parse one response line into a validated probe response bound to the sent
 * request identifier. Any mismatch, malformation, or truncation fails closed
 * with a constant error.
 */
export function parseProbeResponse(line: string, requestId: string): ProbeResponse {
  if (line.length === 0 || line.length > MAX_FRAME_BYTES) {
    throw new Error("malformed-response");
  }
  let value: unknown;
  try {
    value = JSON.parse(line) as unknown;
  } catch {
    throw new Error("malformed-response");
  }
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("malformed-response");
  }
  const record = value as Record<string, unknown>;
  const fields = [
    "contract_version",
    "daemon_version",
    "health",
    "implementation_revision",
    "lifecycle_state",
    "request_id",
    "schema_sha256",
  ];
  for (const field of fields) {
    if (typeof record[field] !== "string") {
      throw new Error("malformed-response");
    }
  }
  if (record["contract_version"] !== PROBE_CONTRACT_VERSION) {
    throw new Error("contract-mismatch");
  }
  if (record["request_id"] !== requestId) {
    throw new Error("request-mismatch");
  }
  if (record["health"] !== "healthy" && record["health"] !== "unavailable") {
    throw new Error("malformed-response");
  }
  const lifecycle = record["lifecycle_state"];
  if (
    lifecycle !== "starting" &&
    lifecycle !== "serving" &&
    lifecycle !== "shutting_down" &&
    lifecycle !== "stopped"
  ) {
    throw new Error("malformed-response");
  }
  return value as ProbeResponse;
}

/**
 * Derive renderer-facing status from one validated probe. Only a healthy
 * serving daemon reports connected; every other combination is degraded.
 */
export function summarizeProbe(response: ProbeResponse): DaemonStatus {
  if (response.health === "healthy" && response.lifecycle_state === "serving") {
    return {
      connection: "connected",
      daemonVersion: response.daemon_version,
      lifecycle: "serving",
    };
  }
  const lifecycle = response.lifecycle_state as DaemonLifecycle;
  return { connection: "degraded", daemonVersion: null, lifecycle };
}

/**
 * Probe one daemon over an injected socket with a hard timeout. Resolves to
 * validated status; rejects with a constant error on timeout, transport
 * failure, premature close, or malformed bytes. The nonce never escapes.
 */
export function probeDaemonHealth(
  socket: FrameSocket,
  nonceHex: string,
  requestId: string,
  timeoutMs: number,
): Promise<DaemonStatus> {
  let preamble: string;
  let frame: string;
  try {
    preamble = encodeAuthPreamble(nonceHex);
    frame = encodeProbeFrame(requestId);
  } catch {
    return Promise.reject(new Error("invalid-handshake-input"));
  }
  return new Promise<DaemonStatus>((resolve, reject) => {
    let settled = false;
    let buffer = "";
    const finish = (action: () => void): void => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      try {
        socket.destroy();
      } catch {
        /* destroy is best-effort */
      }
      action();
    };
    const timer = setTimeout(() => {
      finish(() => reject(new Error("handshake-timeout")));
    }, timeoutMs);
    socket.onError(() => {
      finish(() => reject(new Error("transport-error")));
    });
    socket.onClose(() => {
      finish(() => reject(new Error("connection-closed")));
    });
    socket.onData((chunk: string) => {
      buffer += chunk;
      if (buffer.length > MAX_FRAME_BYTES + 1) {
        finish(() => reject(new Error("frame-too-large")));
        return;
      }
      const newline = buffer.indexOf("\n");
      if (newline < 0) {
        return;
      }
      const line = buffer.slice(0, newline);
      try {
        const response = parseProbeResponse(line, requestId);
        const status = summarizeProbe(response);
        finish(() => resolve(status));
      } catch {
        finish(() => reject(new Error("malformed-response")));
      }
    });
    try {
      socket.write(preamble);
      socket.write(frame);
    } catch {
      finish(() => reject(new Error("transport-error")));
    }
  });
}
