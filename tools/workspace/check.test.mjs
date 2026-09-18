import assert from "node:assert/strict";
import { copyFile, mkdir, mkdtemp, rm, unlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import test from "node:test";

import { checkWorkspace, governancePaths, requiredPaths } from "./check.mjs";

const root = resolve(import.meta.dirname, "../..");

test("workspace baseline checker passes", () => {
  const result = spawnSync(process.execPath, ["tools/workspace/check.mjs"], {
    cwd: root,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stderr);
  assert.match(result.stdout, /Kernux workspace baseline: PASS/);
});

test("workspace checker fails closed when a governance surface disappears", async () => {
  const fixture = await mkdtemp(resolve(tmpdir(), "kernux-workspace-"));

  try {
    for (const path of requiredPaths) {
      const target = resolve(fixture, path);
      await mkdir(dirname(target), { recursive: true });
      await copyFile(resolve(root, path), target);
    }

    await checkWorkspace(fixture);

    for (const path of governancePaths) {
      const target = resolve(fixture, path);
      await unlink(target);
      await assert.rejects(checkWorkspace(fixture), { code: "ENOENT" });
      await copyFile(resolve(root, path), target);
    }

    await checkWorkspace(fixture);
  } finally {
    await rm(fixture, { recursive: true, force: true });
  }
});
