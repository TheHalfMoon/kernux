import assert from "node:assert/strict";
import { copyFile, mkdir, mkdtemp, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";

import { checkDesignSystem } from "./check.mjs";

async function fixture(renderer = "export const clean = true;") {
  const root = await mkdtemp(resolve(tmpdir(), "kernux-ui-foundation-"));
  await mkdir(resolve(root, "apps/desktop/src"), { recursive: true });
  await mkdir(resolve(root, "packages/ui-system"), { recursive: true });
  await writeFile(resolve(root, "apps/desktop/src/renderer.tsx"), renderer, "utf8");
  await copyFile(
    resolve(process.cwd(), "packages/ui-system/tokens.ts"),
    resolve(root, "packages/ui-system/tokens.ts"),
  );
  await copyFile(
    resolve(process.cwd(), "packages/ui-system/primitives.ts"),
    resolve(root, "packages/ui-system/primitives.ts"),
  );
  return root;
}

async function withFixture(renderer, action) {
  const root = await fixture(renderer);
  try {
    return await action(root);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
}

test("clean source graph and valid TypeScript pass", async () => {
  await withFixture(undefined, async (root) => {
    assert.deepEqual(await checkDesignSystem(root), []);
  });
});

test("missing renderer fails closed", async () => {
  const root = await fixture();
  try {
    await rm(resolve(root, "apps/desktop/src/renderer.tsx"));
    await assert.rejects(checkDesignSystem(root), /renderer-source-missing/u);
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("strict malformed canonical color and contrast values fail closed", async () => {
  const root = await fixture();
  try {
    const tokenPath = resolve(root, "packages/ui-system/tokens.ts");
    const source = await readFile(tokenPath, "utf8");
    await writeFile(
      tokenPath,
      source.replace('successLight: "#146c43"', 'successLight: "#f7f8fa"'),
      "utf8",
    );
    const diagnostics = await checkDesignSystem(root);
    assert.ok(
      diagnostics.some(
        ({ code, detail }) => code === "invalid-contrast-pair" && detail.includes("successLight"),
      ),
    );
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("renderer authority and dynamic imports fail closed", async () => {
  const root = await fixture(`
    import fs from "node:fs";
    import electron from "electron";
    export const escape = () => process.env.SECRET ?? eval("1");
  `);
  try {
    const diagnostics = await checkDesignSystem(root);
    assert.ok(diagnostics.some(({ code }) => code === "renderer-authority-import"));
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("reachable re-export and directory-index authority files are inspected", async () => {
  const root = await fixture(`
    export { authority } from "../../../authority";
  `);
  try {
    await mkdir(resolve(root, "authority"), { recursive: true });
    await writeFile(
      resolve(root, "authority/index.ts"),
      'import fs from "node:fs"; export const authority = fs;',
      "utf8",
    );
    const diagnostics = await checkDesignSystem(root);
    assert.ok(
      diagnostics.some(
        ({ code, path }) =>
          code === "renderer-authority-import" && path.includes("authority/index.ts"),
      ),
    );
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});
test("primitive descriptor shapes, JSX semantics, and raw style bypasses fail closed", async () => {
  const root = await fixture(`
    export const primitive = { tag: "button", attributes: { type: "submit" }, children: [""] };
    export const view = <button role="button" aria-invalid="true" style="padding:13px" />;
  `);
  try {
    const diagnostics = await checkDesignSystem(root);
    for (const expected of [
      "missing-accessible-name",
      "invalid-primitive-attribute",
      "redundant-native-role",
      "invalid-button-aria-invalid",
      "raw-visual-literal",
    ]) {
      assert.ok(
        diagnostics.some(({ code }) => code === expected),
        expected,
      );
    }
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("malformed TypeScript reports a structural diagnostic", async () => {
  await withFixture("export const broken: = 1;", async (root) => {
    const diagnostics = await checkDesignSystem(root);
    assert.ok(
      diagnostics.some(
        ({ code, path }) => code === "typescript-syntax-error" && path.endsWith("renderer.tsx"),
      ),
    );
  });
});

test("absolute, traversal, and missing paths fail closed", async () => {
  const root = await fixture();
  try {
    await assert.rejects(
      checkDesignSystem(root, [resolve(root, "apps/desktop/src/renderer.tsx")]),
      /design-system-path-absolute/u,
    );
    await assert.rejects(checkDesignSystem(root, ["../escape.tsx"]), /design-system-path-escape/u);
    await assert.rejects(
      checkDesignSystem(root, ["apps/desktop/src/missing.tsx"]),
      /design-system-file-unreadable/u,
    );
  } finally {
    await rm(root, { recursive: true, force: true });
  }
});

test("symlink escape fails closed when the platform permits symlinks", async () => {
  const outside = await mkdtemp(resolve(tmpdir(), "kernux-ui-outside-"));
  const root = await fixture();
  const outsideFile = resolve(outside, "escape.ts");
  try {
    await writeFile(outsideFile, "export const escape = true;\n", "utf8");
    try {
      await symlink(outsideFile, resolve(root, "apps/desktop/src/escape.ts"), "file");
      await assert.rejects(
        checkDesignSystem(root, ["apps/desktop/src/escape.ts"]),
        /design-system-path-escape/u,
      );
    } catch (error) {
      if (error?.code !== "EPERM") throw error;
    }
  } finally {
    await rm(root, { recursive: true, force: true });
    await rm(outside, { recursive: true, force: true });
  }
});

test("duplicate paths are deterministic and custom input must be strings", async () => {
  await withFixture(undefined, async (root) => {
    const path = "apps/desktop/src/renderer.tsx";
    assert.deepEqual(await checkDesignSystem(root, [path, path]), []);
    await assert.rejects(checkDesignSystem(root, [42]), /invalid-design-system-file-list/u);
  });
});
