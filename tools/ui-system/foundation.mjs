import { readdir, realpath, stat } from "node:fs/promises";
import { isAbsolute, relative, resolve, sep } from "node:path";
import { API } from "typescript/unstable/sync";

const EXCLUDED_DIRECTORIES = new Set([
  ".git",
  ".specgrain",
  ".tmp",
  "dist",
  "docs",
  "node_modules",
  "out",
  "target",
  "test",
  "tests",
]);
const SOURCE_PATTERN = /\.(?:ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u;

export async function checkFoundation(root, extraPaths = []) {
  const canonicalRoot = await realpath(resolve(root));
  const discovered = await discoverSourceFiles(canonicalRoot);
  if (!discovered.includes("apps/desktop/src/renderer.tsx")) {
    throw new Error("renderer-source-missing");
  }
  if (!Array.isArray(extraPaths) || extraPaths.some((path) => typeof path !== "string")) {
    throw new Error("invalid-design-system-file-list");
  }
  const requested = [...new Set([...discovered, ...extraPaths.map(normalize)])];
  const files = [];
  for (const path of requested) files.push(await confineFile(canonicalRoot, path));

  const api = new API({ cwd: canonicalRoot });
  const snapshot = api.updateSnapshot({ openFiles: files.map(({ absolute }) => absolute) });
  const diagnostics = [];
  try {
    for (const entry of files) {
      const project = snapshot.getDefaultProjectForFile(entry.absolute);
      const file = project?.program.getSourceFile(entry.absolute);
      if (file === undefined) {
        throw new Error(`design-system-file-unreadable:${entry.path}`);
      }
      for (const error of project.program.getSyntacticDiagnostics(entry.absolute)) {
        diagnostics.push({
          path: entry.path,
          line: file.getLineAndCharacterOfPosition(error.start ?? 0).line + 1,
          code: "typescript-syntax-error",
          detail: String(error.messageText),
        });
      }
    }
  } finally {
    api.close();
  }
  return { canonicalRoot, files, diagnostics };
}

export async function discoverSourceFiles(root) {
  const found = [];
  async function walk(current) {
    for (const entry of await readdir(current, { withFileTypes: true })) {
      if (EXCLUDED_DIRECTORIES.has(entry.name)) continue;
      const absolute = resolve(current, entry.name);
      if (entry.isDirectory()) await walk(absolute);
      else if (SOURCE_PATTERN.test(entry.name)) found.push(normalize(relative(root, absolute)));
    }
  }
  await walk(root);
  return found.sort();
}

export async function confineFile(root, input) {
  const path = normalize(input);
  if (isAbsolute(path)) throw new Error(`design-system-path-absolute:${path}`);
  const absolute = resolve(root, path);
  const lexical = relative(root, absolute);
  if (lexical === ".." || lexical.startsWith(`..${sep}`)) {
    throw new Error(`design-system-path-escape:${path}`);
  }
  let canonical;
  try {
    canonical = await realpath(absolute);
  } catch {
    throw new Error(`design-system-file-unreadable:${path}`);
  }
  const rel = relative(root, canonical);
  if (rel === "" || rel === ".." || rel.startsWith(`..${sep}`)) {
    throw new Error(`design-system-path-escape:${path}`);
  }
  if (!(await stat(canonical)).isFile()) {
    throw new Error(`design-system-file-unreadable:${path}`);
  }
  return { path, absolute: canonical };
}

function normalize(path) {
  return path.replaceAll("\\", "/");
}
