import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const schemaPath = resolve(root, "protocol/schema/krp.v1.schema.json");
const fixturePath = resolve(root, "protocol/fixtures/v1/core.json");
const fixtureContract = "kernux.krp.conformance/v1";

function fail(message) {
  throw new Error(`KRP conformance failed: ${message}`);
}

function definitionFromRef(schema, ref) {
  const prefix = "#/$defs/";
  if (
    typeof ref !== "string" ||
    !ref.startsWith(prefix) ||
    ref.slice(prefix.length).includes("/")
  ) {
    fail(`unsupported schema reference ${String(ref)}`);
  }
  const name = ref.slice(prefix.length);
  const definition = schema.$defs?.[name];
  if (!definition) fail(`missing schema definition ${name}`);
  return definition;
}

function validateValue(schema, node, value, path) {
  if (node.$ref) return validateValue(schema, definitionFromRef(schema, node.$ref), value, path);

  if (node.enum) {
    if (!node.enum.includes(value)) fail(`${path} must be one of ${node.enum.join(", ")}`);
    return;
  }

  if (node.type === "string") {
    if (typeof value !== "string") fail(`${path} must be a string`);
    if (node.pattern && !new RegExp(node.pattern).test(value))
      fail(`${path} fails pattern ${node.pattern}`);
    return;
  }

  if (node.type === "integer") {
    if (!Number.isSafeInteger(value)) fail(`${path} must be a safe integer`);
    if (node.minimum !== undefined && value < node.minimum)
      fail(`${path} is below minimum ${node.minimum}`);
    if (node.maximum !== undefined && value > node.maximum)
      fail(`${path} is above maximum ${node.maximum}`);
    return;
  }

  if (node.type === "boolean") {
    if (typeof value !== "boolean") fail(`${path} must be a boolean`);
    return;
  }

  if (node.type === "array") {
    if (!Array.isArray(value)) fail(`${path} must be an array`);
    value.forEach((item, index) => validateValue(schema, node.items, item, `${path}[${index}]`));
    return;
  }

  if (node.type === "object") {
    if (!value || typeof value !== "object" || Array.isArray(value))
      fail(`${path} must be an object`);
    const required = new Set(node.required ?? []);
    for (const property of required) {
      if (!Object.hasOwn(value, property)) fail(`${path}.${property} is required`);
    }
    for (const [property, child] of Object.entries(value)) {
      const propertySchema = node.properties?.[property];
      if (!propertySchema) {
        if (node.additionalProperties === false) fail(`${path}.${property} is not allowed`);
        continue;
      }
      validateValue(schema, propertySchema, child, `${path}.${property}`);
    }
    return;
  }

  fail(`${path} uses unsupported schema node ${JSON.stringify(node)}`);
}

const [schemaBytes, fixtureBytes] = await Promise.all([
  readFile(schemaPath),
  readFile(fixturePath),
]);
const schemaDigest = createHash("sha256").update(schemaBytes).digest("hex");
const schema = JSON.parse(schemaBytes.toString("utf8"));
const fixture = JSON.parse(fixtureBytes.toString("utf8"));

const fixtureKeys = Object.keys(fixture).sort();
if (!isDeepStrictEqual(fixtureKeys, ["cases", "schema", "schema_sha256"])) {
  fail("fixture envelope must contain cases, schema, schema_sha256 only");
}
if (fixture.schema !== fixtureContract) fail(`fixture schema must be ${fixtureContract}`);
if (fixture.schema_sha256 !== schemaDigest) {
  fail(`fixture schema digest ${fixture.schema_sha256} does not match sha256:${schemaDigest}`);
}
if (!Array.isArray(fixture.cases) || fixture.cases.length === 0)
  fail("fixture cases must be a non-empty array");

const names = new Set();
const definitions = new Set();
for (const [index, entry] of fixture.cases.entries()) {
  const path = `cases[${index}]`;
  if (!entry || typeof entry !== "object" || Array.isArray(entry))
    fail(`${path} must be an object`);
  const keys = Object.keys(entry).sort();
  if (!isDeepStrictEqual(keys, ["definition", "name", "value"]))
    fail(`${path} must contain name, definition, value only`);
  if (typeof entry.name !== "string" || !/^[a-z][a-z0-9_]*$/.test(entry.name))
    fail(`${path}.name is invalid`);
  if (names.has(entry.name)) fail(`duplicate fixture case name ${entry.name}`);
  names.add(entry.name);
  if (typeof entry.definition !== "string" || !schema.$defs?.[entry.definition])
    fail(`${path}.definition is unknown`);
  definitions.add(entry.definition);
  validateValue(schema, schema.$defs[entry.definition], entry.value, `${path}.value`);
  const roundTrip = JSON.parse(JSON.stringify(entry.value));
  if (!isDeepStrictEqual(roundTrip, entry.value))
    fail(`${path}.value does not preserve semantic JSON equality`);
}

for (const definition of [
  "CapabilityRequest",
  "Grant",
  "RuntimeContactObservation",
  "OperationStart",
  "OperationObservation",
  "RuntimeError",
  "Event",
  "Evidence",
]) {
  if (!definitions.has(definition)) fail(`required fixture definition ${definition} is missing`);
}

console.log(
  `KRP shared fixture conformance: PASS (${fixture.cases.length} cases, sha256:${schemaDigest})`,
);
