import { access, readFile } from "node:fs/promises";
import { constants } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../..");

const requiredPaths = [
  ".editorconfig",
  ".gitattributes",
  ".gitignore",
  ".npmrc",
  "Cargo.toml",
  "package.json",
  "pnpm-workspace.yaml",
  "rust-toolchain.toml",
  "rustfmt.toml",
  "apps/README.md",
  "packages/README.md",
  "crates/README.md",
  "tools/README.md",
  "third_party/README.md",
  "third_party/provenance/README.md",
  "docs/DEVELOPMENT.md",
  "docs/canonical/EXECUTION_MASTER_PLAN.md",
  "specs/CURRENT.md",
  "specs/tasks.md",
];

for (const path of requiredPaths) {
  await access(resolve(root, path), constants.R_OK);
}

const packageJson = JSON.parse(await readFile(resolve(root, "package.json"), "utf8"));
if (packageJson.name !== "kernux" || packageJson.private !== true) {
  throw new Error("package.json must define the private Kernux workspace root");
}
if (packageJson.packageManager !== "pnpm@12.4.2") {
  throw new Error("package.json must pin pnpm@12.4.2");
}
if (packageJson.engines?.node !== ">=24.0.0 <27") {
  throw new Error("package.json must declare the supported Node baseline");
}

const workspace = await readFile(resolve(root, "pnpm-workspace.yaml"), "utf8");
for (const pattern of ["apps/*", "packages/*", "tools/*"]) {
  if (!workspace.includes(pattern)) {
    throw new Error(`pnpm workspace is missing ${pattern}`);
  }
}

const cargo = await readFile(resolve(root, "Cargo.toml"), "utf8");
for (const required of [
  "[workspace]",
  'resolver = "3"',
  'edition = "2024"',
  'rust-version = "1.98"',
]) {
  if (!cargo.includes(required)) {
    throw new Error(`Cargo workspace is missing ${required}`);
  }
}

const toolchain = await readFile(resolve(root, "rust-toolchain.toml"), "utf8");
for (const required of ['channel = "1.98.1"', '"clippy"', '"rustfmt"']) {
  if (!toolchain.includes(required)) {
    throw new Error(`Rust toolchain is missing ${required}`);
  }
}

console.log("Kernux workspace baseline: PASS");
