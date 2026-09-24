import { dirname, relative, resolve } from "node:path";
import {
  isArrayLiteralExpression,
  isAsExpression,
  isExportDeclaration,
  isIdentifier,
  isImportDeclaration,
  isImportEqualsDeclaration,
  isImportExpression,
  isExternalModuleReference,
  isObjectLiteralExpression,
  isPropertyAssignment,
  isSatisfiesExpression,
  isStringLiteral,
  isVariableDeclaration,
  visitEachChild,
} from "typescript/unstable/ast";

const TOKEN_SOURCE = "packages/ui-system/tokens.ts";
const TOKEN_CATEGORIES = new Set([
  "SPACING",
  "TYPOGRAPHY",
  "SIZE",
  "RADIUS",
  "BORDER",
  "MOTION",
  "COLOR",
]);
const AUTHORITY_MODULES = new Set([
  "buffer",
  "child_process",
  "cluster",
  "crypto",
  "dgram",
  "dns",
  "fs",
  "http",
  "http2",
  "https",
  "module",
  "net",
  "os",
  "path",
  "perf_hooks",
  "process",
  "stream",
  "tls",
  "url",
  "util",
  "v8",
  "vm",
  "worker_threads",
  "zlib",
]);

export function checkAuthority(root, files, parsed) {
  const diagnostics = [];
  inspectTokenAuthority(parsed.get(TOKEN_SOURCE)?.file, diagnostics);
  const graph = rendererGraph(parsed, root);
  for (const path of graph) inspectRendererFile(path, parsed.get(path)?.file, diagnostics);
  return diagnostics;
}

function inspectTokenAuthority(file, diagnostics) {
  if (file === undefined) throw new Error("token-authority-missing");
  const colors = new Map();
  const pairs = [];
  const visit = (node) => {
    if (isVariableDeclaration(node) && isIdentifier(node.name)) {
      if (node.name.text === "COLOR") {
        const object = unwrap(node.initializer);
        if (!isObjectLiteralExpression(object)) throw new Error("invalid-color-authority");
        for (const property of object.properties) {
          if (
            !isPropertyAssignment(property) ||
            !isStringLiteral(property.initializer) ||
            !/^#[0-9a-f]{6}$/iu.test(property.initializer.text)
          ) {
            diagnostics.push(
              atNode(TOKEN_SOURCE, file, node, "invalid-color-authority", propertyName(property)),
            );
          } else colors.set(propertyName(property), property.initializer.text);
        }
      }
      if (node.name.text === "COLOR_CONTRAST_PAIRS") {
        const array = unwrap(node.initializer);
        if (!isArrayLiteralExpression(array)) throw new Error("invalid-contrast-authority");
        for (const element of array.elements) {
          const pair = unwrap(element);
          if (!isObjectLiteralExpression(pair)) throw new Error("invalid-contrast-pair");
          pairs.push(readPair(pair));
        }
      }
      if (
        TOKEN_CATEGORIES.has(node.name.text) &&
        !isObjectLiteralExpression(unwrap(node.initializer))
      ) {
        diagnostics.push(
          atNode(TOKEN_SOURCE, file, node, "invalid-token-category", node.name.text),
        );
      }
    }
    return visitEachChild(node, visit);
  };
  visit(file);
  const seen = new Set();
  for (const pair of pairs) {
    const key = `${pair.foreground}/${pair.background}`;
    if (seen.has(key))
      diagnostics.push(atNode(TOKEN_SOURCE, file, file, "duplicate-contrast-pair", key));
    seen.add(key);
    const foreground = colors.get(pair.foreground?.replace(/^color\./u, ""));
    const background = colors.get(pair.background?.replace(/^color\./u, ""));
    const minimum = pair.minimum === "3" ? 3 : pair.minimum === "4.5" ? 4.5 : 0;
    if (
      !foreground ||
      !background ||
      minimum === 0 ||
      contrastRatio(foreground, background) < minimum
    ) {
      diagnostics.push(atNode(TOKEN_SOURCE, file, file, "invalid-contrast-pair", key));
    }
  }
}

function rendererGraph(parsed, root) {
  const graph = new Set(["apps/desktop/src/renderer.tsx"]);
  const queue = ["apps/desktop/src/renderer.tsx"];
  while (queue.length > 0) {
    const path = queue.shift();
    const file = parsed.get(path)?.file;
    if (file === undefined) continue;
    const visit = (node) => {
      if (
        (isImportDeclaration(node) || isExportDeclaration(node)) &&
        isStringLiteral(node.moduleSpecifier) &&
        node.moduleSpecifier.text.startsWith(".")
      ) {
        for (const imported of resolveCandidates(root, path, node.moduleSpecifier.text)) {
          if (parsed.has(imported) && !graph.has(imported)) {
            graph.add(imported);
            queue.push(imported);
          }
        }
      }
      return visitEachChild(node, visit);
    };
    visit(file);
    for (const match of file.text.matchAll(/(?:from|import)\s*\(?\s*["'](\.[^"']+)["']/gu)) {
      for (const imported of resolveCandidates(root, path, match[1])) {
        if (parsed.has(imported) && !graph.has(imported)) {
          graph.add(imported);
          queue.push(imported);
        }
      }
    }
  }
  return graph;
}

function inspectRendererFile(path, file, diagnostics) {
  if (file === undefined) return;
  const visit = (node) => {
    if (
      isImportDeclaration(node) &&
      isStringLiteral(node.moduleSpecifier) &&
      forbiddenModule(node.moduleSpecifier.text)
    )
      diagnostics.push(
        atNode(path, file, node, "renderer-authority-import", node.moduleSpecifier.text),
      );
    if (
      isImportDeclaration(node) &&
      isStringLiteral(node.moduleSpecifier) &&
      !node.moduleSpecifier.text.startsWith(".") &&
      !/^react(?:\/|$)/u.test(node.moduleSpecifier.text)
    )
      diagnostics.push(
        atNode(path, file, node, "renderer-unresolved-import", node.moduleSpecifier.text),
      );
    if (
      isExportDeclaration(node) &&
      isStringLiteral(node.moduleSpecifier) &&
      forbiddenModule(node.moduleSpecifier.text)
    )
      diagnostics.push(
        atNode(path, file, node, "renderer-authority-import", node.moduleSpecifier.text),
      );
    if (
      isImportExpression(node) &&
      (!isStringLiteral(node.argument) || forbiddenModule(node.argument.text))
    )
      diagnostics.push(atNode(path, file, node, "renderer-authority-import", "dynamic-import"));
    if (
      isImportEqualsDeclaration(node) &&
      isExternalModuleReference(node.moduleReference) &&
      isStringLiteral(node.moduleReference.expression) &&
      forbiddenModule(node.moduleReference.expression.text)
    )
      diagnostics.push(
        atNode(path, file, node, "renderer-authority-import", node.moduleReference.expression.text),
      );
    return visitEachChild(node, visit);
  };
  visit(file);
}

function forbiddenModule(specifier) {
  const root = specifier.startsWith("node:")
    ? specifier.slice(5).split("/")[0]
    : specifier.split("/")[0];
  return (
    specifier === "electron" ||
    specifier.startsWith("electron/") ||
    specifier.startsWith("node:") ||
    AUTHORITY_MODULES.has(root)
  );
}

function resolveCandidates(root, path, specifier) {
  const base = resolve(root, dirname(path), specifier).replace(
    /\.(?:ts|tsx|mts|cts|js|jsx|mjs|cjs)$/u,
    "",
  );
  return [
    `${base}.ts`,
    `${base}.tsx`,
    `${base}.mts`,
    `${base}.cts`,
    `${base}.js`,
    resolve(base, "index.ts"),
    resolve(base, "index.tsx"),
  ].map((candidate) => relative(root, candidate).replaceAll("\\", "/"));
}

function unwrap(node) {
  let value = node;
  while (isAsExpression(value) || isSatisfiesExpression(value) || isObjectFreezeCall(value)) {
    value =
      isAsExpression(value) || isSatisfiesExpression(value) ? value.expression : value.arguments[0];
  }
  return value;
}

function isObjectFreezeCall(node) {
  return node.kind === 214;
}

function readPair(object) {
  const pair = {};
  for (const property of object.properties) {
    if (!isPropertyAssignment(property)) continue;
    if (isStringLiteral(property.initializer))
      pair[propertyName(property)] = property.initializer.text;
    else if (property.initializer.kind === 8)
      pair[propertyName(property)] = String(property.initializer.text);
  }
  return pair;
}

function propertyName(property) {
  return isIdentifier(property.name) || isStringLiteral(property.name) ? property.name.text : "";
}

function contrastRatio(first, second) {
  const luminance = (hex) => {
    const channels = [1, 3, 5]
      .map((index) => Number.parseInt(hex.slice(index, index + 2), 16) / 255)
      .map((value) => (value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4));
    return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];
  };
  const firstLuminance = luminance(first);
  const secondLuminance = luminance(second);
  return (
    (Math.max(firstLuminance, secondLuminance) + 0.05) /
    (Math.min(firstLuminance, secondLuminance) + 0.05)
  );
}

function atNode(path, file, node, code, detail) {
  return { path, line: file.getLineAndCharacterOfPosition(node.pos).line + 1, code, detail };
}
