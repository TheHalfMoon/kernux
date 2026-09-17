import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";
import test from "node:test";

const root = resolve(import.meta.dirname, "../..");

test("workspace baseline checker passes", () => {
  const result = spawnSync(process.execPath, ["tools/workspace/check.mjs"], {
    cwd: root,
    encoding: "utf8"
  });

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /Kernux workspace baseline: PASS/);
});
