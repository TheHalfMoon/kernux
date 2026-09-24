/**
 * Handshake client tests (SG-000031 PR-B). Synthetic data only.
 * Frame compatibility is asserted against the frozen KRP fixture values.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  AUTH_PREAMBLE_LENGTH,
  AUTH_PREAMBLE_PREFIX,
  HANDSHAKE_TIMEOUT_MS,
  LAUNCH_NONCE_HEX_LENGTH,
  MAX_FRAME_BYTES,
  PROBE_CONTRACT_VERSION,
  PROBE_KIND,
  encodeAuthPreamble,
  encodeProbeFrame,
  isCanonicalUuidV7,
  isLaunchNonceHex,
  parseProbeResponse,
  probeDaemonHealth,
  summarizeProbe,
  type DaemonStatus,
  type FrameSocket,
  type ProbeResponse,
} from "../src/transport-client.js";

const NONCE = "0".repeat(63) + "1";
const REQUEST_ID = "01890f3a-7b2f-7e55-aa66-6f708192a3b5";

function validResponse(): ProbeResponse {
  return {
    contract_version: "krp/1",
    daemon_version: "0.0.0",
    health: "healthy",
    implementation_revision: "0".repeat(40),
    lifecycle_state: "serving",
    request_id: REQUEST_ID,
    schema_sha256: "9".repeat(64),
  };
}

function fakeSocket(script: Array<string | "close" | "error">): FrameSocket {
  const onData: Array<(chunk: string) => void> = [];
  const onError: Array<(message: string) => void> = [];
  const onClose: Array<() => void> = [];
  let destroyed = false;
  return {
    write: (_data: string) => {
      if (destroyed) {
        throw new Error("write-after-destroy");
      }
      for (const step of script) {
        if (step === "close") {
          for (const listener of onClose) {
            listener();
          }
        } else if (step === "error") {
          for (const listener of onError) {
            listener("boom");
          }
        } else {
          for (const listener of onData) {
            listener(step);
          }
        }
      }
    },
    onData: (listener: (chunk: string) => void) => {
      onData.push(listener);
    },
    onError: (listener: (message: string) => void) => {
      onError.push(listener);
    },
    onClose: (listener: () => void) => {
      onClose.push(listener);
    },
    destroy: () => {
      destroyed = true;
    },
  };
}

describe("preamble encoding matches the daemon wire format", () => {
  it("emits the exact 71-byte preamble", () => {
    const frame = encodeAuthPreamble(NONCE);
    assert.equal(frame, `kxa/1 ${NONCE}\n`);
    assert.equal(frame.length, AUTH_PREAMBLE_LENGTH);
    assert.equal(AUTH_PREAMBLE_PREFIX, "kxa/1 ");
    assert.equal(LAUNCH_NONCE_HEX_LENGTH, 64);
    assert.equal(MAX_FRAME_BYTES, 4096);
  });

  it("rejects malformed nonces without echoing them", () => {
    const hostile = "SECRET-NONCE-VALUE-THAT-MUST-NEVER-LEAK-0123456789abcdef";
    assert.equal(isLaunchNonceHex(hostile), false);
    assert.throws(
      () => encodeAuthPreamble(hostile),
      (error: unknown) => error instanceof Error && error.message === "invalid-nonce",
    );
    for (const bad of [
      "",
      "0".repeat(63),
      "0".repeat(65),
      "G".repeat(64),
      "0".repeat(63) + " ",
      "0".repeat(63) + "\n",
      "ABCDEF".repeat(10) + "ABCD",
    ]) {
      assert.equal(isLaunchNonceHex(bad), false, JSON.stringify(bad));
      assert.throws(() => encodeAuthPreamble(bad));
    }
    assert.equal(isLaunchNonceHex("abcdef".repeat(10) + "abcd"), true);
  });
});

describe("probe frames match the frozen KRP contract", () => {
  it("emits newline-terminated health_version probes", () => {
    const frame = encodeProbeFrame(REQUEST_ID);
    assert.ok(frame.endsWith("\n"));
    const body = JSON.parse(frame.slice(0, -1)) as Record<string, unknown>;
    assert.deepEqual(body, {
      contract_version: "krp/1",
      probe: "health_version",
      request_id: REQUEST_ID,
    });
    assert.equal(PROBE_CONTRACT_VERSION, "krp/1");
    assert.equal(PROBE_KIND, "health_version");
  });

  it("validates canonical UUIDv7 request identifiers", () => {
    assert.equal(isCanonicalUuidV7(REQUEST_ID), true);
    for (const bad of [
      "",
      "not-a-uuid",
      "550e8400-e29b-41d4-a716-446655440000",
      "01890f3a-7b2f-7e55-aa66-6f708192a3b",
      "01890f3a-7b2f-7e55-aa66-6f708192a3b55",
      "01890f3a-7b2f-6e55-aa66-6f708192a3b5",
      "01890f3a-7b2f-8e55-aa66-6f708192a3b5",
    ]) {
      assert.equal(isCanonicalUuidV7(bad), false, bad);
      assert.throws(() => encodeProbeFrame(bad));
    }
  });
});

describe("response parsing binds to the sent request", () => {
  it("accepts the frozen fixture shape", () => {
    const line = JSON.stringify(validResponse());
    assert.deepEqual(parseProbeResponse(line, REQUEST_ID), validResponse());
  });

  it("rejects mismatched, truncated, and hostile responses", () => {
    const mismatched = JSON.stringify({ ...validResponse(), request_id: "01890f3a-7b2c-7d45-8a61-3c4e5f607182" });
    assert.throws(() => parseProbeResponse(mismatched, REQUEST_ID), (error: unknown) => error instanceof Error && error.message === "request-mismatch");
    const wrongContract = JSON.stringify({ ...validResponse(), contract_version: "krp/2" });
    assert.throws(() => parseProbeResponse(wrongContract, REQUEST_ID), (error: unknown) => error instanceof Error && error.message === "contract-mismatch");
    for (const hostile of [
      "",
      "{",
      "[1,2]",
      "null",
      "42",
      JSON.stringify({ ...validResponse(), health: "pwned" }),
      JSON.stringify({ ...validResponse(), lifecycle_state: "root" }),
      JSON.stringify({ ...validResponse(), request_id: 42 }),
      "x".repeat(MAX_FRAME_BYTES + 1),
    ]) {
      assert.throws(() => parseProbeResponse(hostile, REQUEST_ID));
    }
  });

  it("derives connected only from healthy serving daemons", () => {
    const connected: DaemonStatus = summarizeProbe(validResponse());
    assert.deepEqual(connected, {
      connection: "connected",
      daemonVersion: "0.0.0",
      lifecycle: "serving",
    });
    for (const lifecycle of ["starting", "shutting_down", "stopped"] as const) {
      const degraded = summarizeProbe({ ...validResponse(), lifecycle_state: lifecycle });
      assert.equal(degraded.connection, "degraded");
      assert.equal(degraded.daemonVersion, null);
    }
    const unavailable = summarizeProbe({ ...validResponse(), health: "unavailable" });
    assert.equal(unavailable.connection, "degraded");
  });
});

describe("round trip over an injected socket", () => {
  it("resolves healthy status on exact bytes", async () => {
    const line = `${JSON.stringify(validResponse())}\n`;
    const socket = fakeSocket([line]);
    const status = await probeDaemonHealth(socket, NONCE, REQUEST_ID, HANDSHAKE_TIMEOUT_MS);
    assert.equal(status.connection, "connected");
    assert.equal(status.daemonVersion, "0.0.0");
  });

  it("tolerates chunked delivery", async () => {
    const line = `${JSON.stringify(validResponse())}\n`;
    const socket = fakeSocket([line.slice(0, 17), line.slice(17, 55), line.slice(55)]);
    const status = await probeDaemonHealth(socket, NONCE, REQUEST_ID, 500);
    assert.equal(status.connection, "connected");
  });

  it("fails closed on transport error, close, and malformed bytes", async () => {
    await assert.rejects(
      probeDaemonHealth(fakeSocket(["error"]), NONCE, REQUEST_ID, 500),
      (error: unknown) => error instanceof Error && error.message === "transport-error",
    );
    await assert.rejects(
      probeDaemonHealth(fakeSocket(["close"]), NONCE, REQUEST_ID, 500),
      (error: unknown) => error instanceof Error && error.message === "connection-closed",
    );
    await assert.rejects(
      probeDaemonHealth(fakeSocket(["{oops}\n"]), NONCE, REQUEST_ID, 500),
      (error: unknown) => error instanceof Error && error.message === "malformed-response",
    );
  });

  it("rejects invalid handshake input without touching the socket", async () => {
    let writes = 0;
    const counting: FrameSocket = {
      ...fakeSocket([]),
      write: (_data: string) => {
        writes += 1;
      },
    };
    await assert.rejects(
      probeDaemonHealth(counting, "bad", REQUEST_ID, 50),
      (error: unknown) => error instanceof Error && error.message === "invalid-handshake-input",
    );
    await assert.rejects(
      probeDaemonHealth(counting, NONCE, "bad-id", 50),
      (error: unknown) => error instanceof Error && error.message === "invalid-handshake-input",
    );
    assert.equal(writes, 0);
  });
});
