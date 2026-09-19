import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const schemaPath = resolve(root, "protocol/schema/krp.v1.schema.json");
const outputs = {
  ts: resolve(root, "packages/contracts/src/generated.ts"),
  rust: resolve(root, "crates/kernux-contracts/src/generated.rs"),
};

const allowedKeywords = new Set([
  "$schema",
  "$id",
  "$defs",
  "$ref",
  "title",
  "description",
  "type",
  "additionalProperties",
  "enum",
  "required",
  "properties",
  "items",
  "pattern",
  "minimum",
  "maximum",
]);

function fail(message) {
  throw new Error(`KRP schema generation failed: ${message}`);
}

function scanKeywords(value, path = "$") {
  if (Array.isArray(value)) {
    value.forEach((item, index) => scanKeywords(item, `${path}[${index}]`));
    return;
  }
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) {
    if (!allowedKeywords.has(key)) fail(`unsupported keyword ${key} at ${path}`);
    if (key === "properties" || key === "$defs") {
      if (!child || typeof child !== "object" || Array.isArray(child)) {
        fail(`${key} at ${path} must be an object`);
      }
      for (const [name, node] of Object.entries(child)) {
        scanKeywords(node, `${path}.${key}.${name}`);
      }
      continue;
    }
    scanKeywords(child, `${path}.${key}`);
  }
}

function validateSchema(schema) {
  if (schema.$schema !== "https://json-schema.org/draft/2020-12/schema") {
    fail("schema must declare JSON Schema Draft 2020-12");
  }
  if (schema.type !== "object" || schema.additionalProperties !== false) {
    fail("root must be a closed object");
  }
  if (!schema.$defs || typeof schema.$defs !== "object") {
    fail("schema must define $defs");
  }
  scanKeywords(schema);
  for (const [name, def] of Object.entries(schema.$defs)) {
    if (def.type === "string") {
      if (def.enum !== undefined && !Array.isArray(def.enum)) fail(`${name} enum must be an array`);
      continue;
    }
    if (def.type !== "object") fail(`${name} must be an object or string definition`);
    if (def.additionalProperties !== false) fail(`${name} must set additionalProperties=false`);
    if (!def.properties || typeof def.properties !== "object")
      fail(`${name} must define properties`);
    if (!Array.isArray(def.required)) fail(`${name} must define required as an array`);
  }
}

function refName(ref) {
  const prefix = "#/$defs/";
  if (typeof ref !== "string" || !ref.startsWith(prefix)) fail(`unsupported reference ${ref}`);
  const name = ref.slice(prefix.length);
  if (!name || name.includes("/")) fail(`unsupported reference ${ref}`);
  return name;
}

function enumVariant(value) {
  const chunks = String(value)
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean);
  let result = chunks.map((part) => part[0].toUpperCase() + part.slice(1)).join("");
  if (!result) fail(`cannot generate enum variant for ${value}`);
  if (/^[0-9]/.test(result)) result = `V${result}`;
  return result;
}

function tsType(node) {
  if (node.$ref) return refName(node.$ref);
  if (node.type === "string") return "string";
  if (node.type === "integer") return "number";
  if (node.type === "boolean") return "boolean";
  if (node.type === "array") return `Array<${tsType(node.items)}>`;
  fail(`unsupported TypeScript node ${JSON.stringify(node)}`);
}

function rustType(node) {
  if (node.$ref) return refName(node.$ref);
  if (node.type === "string") return "String";
  if (node.type === "integer") {
    return typeof node.maximum === "number" && node.maximum <= 4294967295 ? "u32" : "u64";
  }
  if (node.type === "boolean") return "bool";
  if (node.type === "array") return `Vec<${rustType(node.items)}>`;
  fail(`unsupported Rust node ${JSON.stringify(node)}`);
}

function header(prefix, digest) {
  return [
    `${prefix} @generated from protocol/schema/krp.v1.schema.json`,
    `${prefix} Schema SHA-256: ${digest}`,
    `${prefix} DO NOT EDIT. Change the schema and regenerate.`,
    "",
  ].join("\n");
}

function generateTypeScript(schema, digest) {
  const lines = [header("//", digest)];
  for (const name of Object.keys(schema.$defs).sort()) {
    const def = schema.$defs[name];
    if (def.type === "string" && !def.enum) {
      lines.push(`export type ${name} = string;`, "");
      continue;
    }
    if (def.type === "string" && def.enum) {
      const members = def.enum.map((value) => JSON.stringify(value));
      const singleLine = `export type ${name} = ${members.join(" | ")};`;
      if (singleLine.length <= 100) {
        lines.push(singleLine, "");
      } else {
        lines.push(`export type ${name} =`);
        members.forEach((member, index) => {
          lines.push(`  | ${member}${index === members.length - 1 ? ";" : ""}`);
        });
        lines.push("");
      }
      continue;
    }
    const required = new Set(def.required);
    lines.push(`export interface ${name} {`);
    for (const property of Object.keys(def.properties).sort()) {
      const optional = required.has(property) ? "" : "?";
      lines.push(`  ${property}${optional}: ${tsType(def.properties[property])};`);
    }
    lines.push("}", "");
  }
  return `${lines.join("\n").trimEnd()}\n`;
}

function generateRust(schema, digest) {
  const lines = [header("//", digest), "use serde::{Deserialize, Serialize};", ""];
  for (const name of Object.keys(schema.$defs).sort()) {
    const def = schema.$defs[name];
    if (def.type === "string" && !def.enum) {
      lines.push(`pub type ${name} = String;`, "");
      continue;
    }
    if (def.type === "string" && def.enum) {
      lines.push(
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]",
        `pub enum ${name} {`,
      );
      for (const value of def.enum) {
        lines.push(`    #[serde(rename = ${JSON.stringify(value)})]`, `    ${enumVariant(value)},`);
      }
      lines.push("}", "");
      continue;
    }
    const required = new Set(def.required);
    lines.push(
      "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
      `pub struct ${name} {`,
    );
    for (const property of Object.keys(def.properties).sort()) {
      const base = rustType(def.properties[property]);
      if (required.has(property)) {
        lines.push(`    pub ${property}: ${base},`);
      } else {
        lines.push(
          '    #[serde(skip_serializing_if = "Option::is_none")]',
          `    pub ${property}: Option<${base}>,`,
        );
      }
    }
    lines.push("}", "");
  }
  return `${lines.join("\n").trimEnd()}\n`;
}

async function expectedOutputs() {
  const raw = await readFile(schemaPath);
  const digest = createHash("sha256").update(raw).digest("hex");
  const schema = JSON.parse(raw.toString("utf8"));
  validateSchema(schema);
  return {
    digest,
    ts: generateTypeScript(schema, digest),
    rust: generateRust(schema, digest),
  };
}

async function checkOne(path, expected) {
  let actual;
  try {
    actual = await readFile(path, "utf8");
  } catch {
    fail(`generated output missing: ${path}`);
  }
  if (actual !== expected) fail(`generated output drift: ${path}`);
}

function parseArgs(argv) {
  let mode = "write";
  let target = "all";
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === "--check") mode = "check";
    else if (arg === "--dry-run") mode = "dry-run";
    else if (arg === "--target") {
      target = argv[index + 1];
      index += 1;
    } else fail(`unknown argument ${arg}`);
  }
  if (!["all", "ts", "rust"].includes(target)) fail(`unknown target ${target}`);
  return { mode, target };
}

const { mode, target } = parseArgs(process.argv.slice(2));
const generated = await expectedOutputs();
const selected = target === "all" ? ["ts", "rust"] : [target];

if (mode === "dry-run") {
  console.log(`KRP schema: sha256:${generated.digest}`);
  for (const key of selected) {
    console.log(`${key}: ${generated[key].split("\n").length - 1} lines`);
  }
} else if (mode === "check") {
  for (const key of selected) await checkOne(outputs[key], generated[key]);
  console.log(`KRP generated contracts: PASS (${selected.join(", ")})`);
} else {
  for (const key of selected) {
    await mkdir(dirname(outputs[key]), { recursive: true });
    await writeFile(outputs[key], generated[key], "utf8");
    console.log(`wrote ${outputs[key]}`);
  }
}
