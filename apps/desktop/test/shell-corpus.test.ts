/**
 * Shell adversarial corpus (SG-000031 PR-C). Synthetic data only.
 * Proves downgrade, tampering, confusion, and oversize inputs fail closed.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  ALLOWED_INVOKE_CHANNELS,
  MAX_ENVELOPE_BYTES,
  buildInvokeEnvelope,
  isAllowedInvokeChannel,
} from "../src/bridge.js";
import {
  RENDERER_SECURITY_FLAGS,
  contentSecurityPolicy,
  satisfiesWindowPolicy,
} from "../src/window-config.js";
import {
  encodeAuthPreamble,
  parseProbeResponse,
} from "../src/transport-client.js";

const REQUEST_ID = "01890f3a-7b2f-7e55-aa66-6f708192a3b5";

describe("allowlist cannot be widened at runtime", () => {
  it("is frozen and exactly two channels", () => {
    assert.equal(Object.isFrozen(ALLOWED_INVOKE_CHANNELS), true);
    assert.equal(ALLOWED_INVOKE_CHANNELS.length, 2);
  });

  it("denies relabeled privileged channels", () => {
    const envelope = buildInvokeEnvelope("kernux/daemon/health", "req-9", null);
    assert.equal(envelope.channel, "kernux/daemon/health");
    for (const hostile of [
      "kernux/daemon/health#exec",
      "kernux/daemon/health?x=1",
      "kernux/daemon/health/",
      "/kernux/daemon/health",
      "kernux//daemon//health",
      "kernux/daemon/HEALTH",
      "kernux\uFF0Fdaemon\uFF0Fhealth",
    ]) {
      assert.equal(isAllowedInvokeChannel(hostile), false, hostile);
      assert.throws(() => buildInvokeEnvelope(hostile, "req-9", null));
    }
  });
});

describe("payload bounds are enforced", () => {
  it("accepts small JSON payloads", () => {
    const envelope = buildInvokeEnvelope("kernux/daemon/version", "req-2", {
      detail: "synthetic",
    });
    assert.equal(envelope.requestId, "req-2");
  });

  it("rejects oversized, circular, and unserializable payloads", () => {
    assert.throws(() =>
      buildInvokeEnvelope("kernux/daemon/health", "req-3", "x".repeat(MAX_ENVELOPE_BYTES + 1)),
    );
    const circular: Record<string, unknown> = {};
    circular["self"] = circular;
    assert.throws(() => buildInvokeEnvelope("kernux/daemon/health", "req-3", circular));
    assert.throws(() => buildInvokeEnvelope("kernux/daemon/health", "req-3", undefined));
    assert.throws(() =>
      buildInvokeEnvelope("kernux/daemon/health", "req-3", () => undefined),
    );
  });
});

describe("policy tampering is rejected", () => {
  it("rejects every double-flag escalation", () => {
    const flags = { ...RENDERER_SECURITY_FLAGS };
    const escalated = [
      { ...flags, nodeIntegration: true, sandbox: false },
      { ...flags, contextIsolation: false, webSecurity: false },
      { ...flags, allowRunningInsecureContent: true, nodeIntegration: true },
    ];
    for (const candidate of escalated) {
      assert.equal(satisfiesWindowPolicy(candidate), false);
    }
    assert.equal(satisfiesWindowPolicy(flags), true);
  });

  it("rejects type-confused flags", () => {
    const confused = { ...RENDERER_SECURITY_FLAGS } as unknown as Record<string, unknown>;
    for (const key of Object.keys(confused)) {
      for (const hostile of [1, "true", null, undefined, 0, ""] as unknown[]) {
        const candidate = {
          ...RENDERER_SECURITY_FLAGS,
          [key]: hostile,
        } as unknown as Parameters<typeof satisfiesWindowPolicy>[0];
        assert.equal(satisfiesWindowPolicy(candidate), false, `${key}=${String(hostile)}`);
      }
    }
  });

  it("locks the exact CSP string", () => {
    assert.equal(
      contentSecurityPolicy(),
      "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; " +
        "font-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'self'; " +
        "form-action 'none'",
    );
  });
});

describe("secrets never escape in errors", () => {
  it("response errors carry only constant codes", () => {
    const nonce = `abcd${"0".repeat(60)}`;
    assert.throws(() => encodeAuthPreamble("short"), (error: unknown) => {
      assert.ok(error instanceof Error);
      assert.equal(error.message, "invalid-nonce");
      assert.ok(!error.message.includes(nonce));
      return true;
    });
    assert.throws(() => parseProbeResponse("{bad", REQUEST_ID), (error: unknown) => {
      assert.ok(error instanceof Error);
      assert.equal(error.message, "malformed-response");
      return true;
    });
  });
});
