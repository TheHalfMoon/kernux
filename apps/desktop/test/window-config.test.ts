/**
 * Window policy tests (SG-000031 PR-A). Synthetic data only.
 */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  PRELOAD_ENTRY,
  RENDERER_SECURITY_FLAGS,
  SINGLE_WINDOW_LIMIT,
  WINDOW_DEFAULT_HEIGHT,
  WINDOW_DEFAULT_WIDTH,
  WINDOW_MIN_HEIGHT,
  WINDOW_MIN_WIDTH,
  contentSecurityPolicy,
  satisfiesWindowPolicy,
} from "../src/window-config.js";

describe("renderer security flags are frozen", () => {
  it("locks the exact unprivileged configuration", () => {
    assert.deepEqual({ ...RENDERER_SECURITY_FLAGS }, {
      contextIsolation: true,
      sandbox: true,
      nodeIntegration: false,
      webSecurity: true,
      allowRunningInsecureContent: false,
    });
  });

  it("validates candidate configurations exactly", () => {
    assert.equal(satisfiesWindowPolicy({ ...RENDERER_SECURITY_FLAGS }), true);
    const escalations = [
      { ...RENDERER_SECURITY_FLAGS, nodeIntegration: true },
      { ...RENDERER_SECURITY_FLAGS, sandbox: false },
      { ...RENDERER_SECURITY_FLAGS, contextIsolation: false },
      { ...RENDERER_SECURITY_FLAGS, webSecurity: false },
      { ...RENDERER_SECURITY_FLAGS, allowRunningInsecureContent: true },
    ];
    for (const candidate of escalations) {
      assert.equal(satisfiesWindowPolicy(candidate), false);
    }
  });

  it("serves local content only through a strict policy", () => {
    const csp = contentSecurityPolicy();
    assert.ok(csp.includes("default-src 'self'"));
    assert.ok(csp.includes("object-src 'none'"));
    assert.ok(csp.includes("form-action 'none'"));
    for (const forbidden of [
      "unsafe-inline",
      "unsafe-eval",
      "http://",
      "https://",
      "data:script",
      "blob:",
      "wasm-unsafe-eval",
    ]) {
      assert.ok(!csp.includes(forbidden), forbidden);
    }
  });

  it("fixes geometry and the single-window invariant", () => {
    assert.equal(WINDOW_DEFAULT_WIDTH, 1280);
    assert.equal(WINDOW_DEFAULT_HEIGHT, 800);
    assert.equal(WINDOW_MIN_WIDTH, 960);
    assert.equal(WINDOW_MIN_HEIGHT, 600);
    assert.equal(SINGLE_WINDOW_LIMIT, 1);
    assert.equal(PRELOAD_ENTRY, "preload.js");
  });
});
