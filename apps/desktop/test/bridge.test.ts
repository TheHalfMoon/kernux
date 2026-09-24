/**
 * Bridge allowlist tests (SG-000031 PR-A). Synthetic data only.
 * Executed via the workspace TypeScript compiler plus node:test.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  ALLOWED_INVOKE_CHANNELS,
  DAEMON_HEALTH_CHANNEL,
  DAEMON_VERSION_CHANNEL,
  MAX_ENVELOPE_BYTES,
  buildInvokeEnvelope,
  isAllowedInvokeChannel,
  isWellFormedResult,
} from "../src/bridge.js";

describe("bridge vocabulary is closed and exact", () => {
  it("exposes exactly the two daemon channels", () => {
    assert.deepEqual([...ALLOWED_INVOKE_CHANNELS], [
      "kernux/daemon/health",
      "kernux/daemon/version",
    ]);
    assert.equal(DAEMON_HEALTH_CHANNEL, "kernux/daemon/health");
    assert.equal(DAEMON_VERSION_CHANNEL, "kernux/daemon/version");
    assert.ok(MAX_ENVELOPE_BYTES > 0);
  });

  it("accepts only exact allowlisted channels", () => {
    assert.equal(isAllowedInvokeChannel("kernux/daemon/health"), true);
    assert.equal(isAllowedInvokeChannel("kernux/daemon/version"), true);
  });

  it("rejects permissive and confused channel forms", () => {
    for (const hostile of [
      "",
      "*",
      "kernux/*",
      "kernux/daemon/*",
      "KERNUX/DAEMON/HEALTH",
      "Kernux/Daemon/Health",
      " kernux/daemon/health",
      "kernux/daemon/health ",
      "kernux/daemon/health\n",
      "kernux/daemon/health\0",
      "kernux/daemon/health/../version",
      "kernux/daemon/health.",
      "electron/ipc",
      "__proto__",
      "constructor",
      "prototype",
      "hasOwnProperty",
      "kernux/daemon/exec",
      "kernux/daemon/eval",
      "kernux/daemon/shell",
      "kernux/daemon/secret-reveal",
      "kernux/daemon/grant-issue",
      "file:///etc/passwd",
      "https://attacker.example/hook",
    ]) {
      assert.equal(isAllowedInvokeChannel(hostile), false, JSON.stringify(hostile));
    }
  });
});

describe("envelope construction fails closed", () => {
  it("builds exact envelopes for allowlisted channels", () => {
    const envelope = buildInvokeEnvelope("kernux/daemon/health", "req-1", null);
    assert.deepEqual(envelope, {
      channel: "kernux/daemon/health",
      requestId: "req-1",
      payload: null,
    });
  });

  it("throws a constant error without echoing hostile channels", () => {
    for (const hostile of ["kernux/daemon/exec", "", "*", "__proto__"]) {
      assert.throws(
        () => buildInvokeEnvelope(hostile, "req-1", null),
        (error: unknown) => {
          assert.ok(error instanceof Error);
          assert.equal(error.message, "unknown-channel");
          assert.ok(!error.message.includes(hostile) || hostile === "");
          return true;
        },
      );
    }
  });

  it("rejects empty request identifiers", () => {
    assert.throws(
      () => buildInvokeEnvelope("kernux/daemon/health", "", null),
      (error: unknown) => error instanceof Error && error.message === "invalid-request",
    );
  });
});

describe("response validation is exact", () => {
  it("accepts only well-formed success and denial shapes", () => {
    assert.equal(isWellFormedResult({ ok: true, data: { status: "ok" } }), true);
    assert.equal(isWellFormedResult({ ok: false, error: "denied" }), true);
  });

  it("rejects malformed and hostile responses", () => {
    for (const hostile of [
      null,
      undefined,
      42,
      "ok",
      true,
      [],
      [{ ok: true, data: 1 }],
      {},
      { ok: true },
      { ok: false },
      { ok: false, error: 42 },
      { ok: "yes", data: 1 },
      { ok: 1, data: 1 },
      { data: 1 },
      { error: "x" },
    ]) {
      assert.equal(isWellFormedResult(hostile), false);
    }
  });
});
