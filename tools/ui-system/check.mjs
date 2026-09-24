#!/usr/bin/env node
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

import { checkFoundation } from "./foundation.mjs";
import { checkAuthority } from "./authority.mjs";

export async function checkDesignSystem(root, paths) {
  const extraPaths = paths ?? [];
  if (!Array.isArray(extraPaths)) throw new Error("invalid-design-system-file-list");
  const result = await checkFoundation(root, extraPaths);
  const diagnostics = [
    ...result.diagnostics,
    ...checkAuthority(result.canonicalRoot, result.files, result.parsed),
  ];
  return diagnostics;
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  const paths = process.argv.slice(2);
  const diagnostics = await checkDesignSystem(
    process.cwd(),
    paths.length === 0 ? undefined : paths,
  );
  if (diagnostics.length > 0) {
    for (const item of diagnostics) {
      console.error(`${item.path}:${item.line}: ${item.code}: ${item.detail}`);
    }
    process.exitCode = 1;
  } else {
    console.log("Kernux design-system foundation: PASS");
  }
}
