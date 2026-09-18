import { access, readFile } from "node:fs/promises";
import { constants } from "node:fs";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

const defaultRoot = resolve(import.meta.dirname, "../..");

export const governancePaths = [
  "README.md",
  "CONTRIBUTING.md",
  "SECURITY.md",
  "SUPPORT.md",
  "CODE_OF_CONDUCT.md",
  ".github/PULL_REQUEST_TEMPLATE.md",
  ".github/ISSUE_TEMPLATE/bug_report.yml",
  ".github/ISSUE_TEMPLATE/feature_request.yml",
  ".github/ISSUE_TEMPLATE/support_question.yml",
  ".github/ISSUE_TEMPLATE/config.yml",
  "docs/governance/REPOSITORY_GOVERNANCE.md",
];

export const requiredPaths = [
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
  ...governancePaths,
];

export async function checkWorkspace(root = defaultRoot) {
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
}

const invokedPath = process.argv[1] ? resolve(process.argv[1]) : null;
if (invokedPath === fileURLToPath(import.meta.url)) {
  await checkWorkspace();
  console.log("Kernux workspace baseline: PASS");
}
