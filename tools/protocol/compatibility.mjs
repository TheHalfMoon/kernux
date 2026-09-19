import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const schemaPath = resolve(root, "protocol/schema/krp.v1.schema.json");
const coreFixturePath = resolve(root, "protocol/fixtures/v1/core.json");
const compatibilityPath = resolve(root, "protocol/fixtures/v1/compatibility.json");
const matrixContract = "kernux.krp.compatibility/v1";

function fail(message) {
  throw new Error(`KRP compatibility failed: ${message}`);
}

function exactKeys(value, expected, path) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    fail(`${path} must be an object`);
  }
  const actual = Object.keys(value).sort();
  const wanted = [...expected].sort();
  if (!isDeepStrictEqual(actual, wanted)) {
    fail(`${path} must contain exactly ${wanted.join(", ")}`);
  }
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
    if (!node.enum.includes(value)) fail(`${path} has unsupported value ${String(value)}`);
    return;
  }

  if (node.type === "string") {
    if (typeof value !== "string") fail(`${path} must be a string`);
    if (node.pattern && !new RegExp(node.pattern).test(value)) {
      fail(`${path} fails pattern ${node.pattern}`);
    }
    return;
  }

  if (node.type === "integer") {
    if (!Number.isSafeInteger(value)) fail(`${path} must be a safe integer`);
    if (node.minimum !== undefined && value < node.minimum) {
      fail(`${path} is below minimum ${node.minimum}`);
    }
    if (node.maximum !== undefined && value > node.maximum) {
      fail(`${path} is above maximum ${node.maximum}`);
    }
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
    if (!value || typeof value !== "object" || Array.isArray(value)) {
      fail(`${path} must be an object`);
    }
    for (const property of node.required ?? []) {
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

  fail(`${path} uses unsupported schema node`);
}

function sourceCases(core) {
  if (!Array.isArray(core.cases)) fail("core fixture cases must be an array");
  const byName = new Map();
  for (const entry of core.cases) {
    if (byName.has(entry.name)) fail(`duplicate core fixture case ${entry.name}`);
    byName.set(entry.name, entry);
  }
  return byName;
}

function observedAcceptance(schema, definition, value) {
  try {
    validateValue(schema, definition, value, "payload");
    return "accept";
  } catch (error) {
    if (!(error instanceof Error) || !error.message.startsWith("KRP compatibility failed:")) {
      throw error;
    }
    return "reject";
  }
}

function mayEmitAdditive(feature, senderFeatures, peerFeatures) {
  return senderFeatures.includes(feature) && peerFeatures.includes(feature);
}

const [schema, core, matrix] = await Promise.all(
  [schemaPath, coreFixturePath, compatibilityPath].map(async (path) =>
    JSON.parse(await readFile(path, "utf8")),
  ),
);

exactKeys(
  matrix,
  [
    "schema",
    "protocol_contract",
    "capability_version",
    "payload_cases",
    "truth_cases",
    "event_projection_cases",
    "feature_gate_cases",
    "capability_admission_cases",
  ],
  "matrix",
);
if (matrix.schema !== matrixContract) fail(`matrix schema must be ${matrixContract}`);

if (!isDeepStrictEqual(schema.$defs?.ProtocolContract?.enum, [matrix.protocol_contract])) {
  fail("matrix protocol_contract must equal the sole generated ProtocolContract value");
}
if (!isDeepStrictEqual(schema.$defs?.CapabilityVersion?.enum, [matrix.capability_version])) {
  fail("matrix capability_version must equal the sole generated CapabilityVersion value");
}

const byName = sourceCases(core);
const caseNames = new Set();

if (!Array.isArray(matrix.payload_cases) || matrix.payload_cases.length === 0) {
  fail("payload_cases must be a non-empty array");
}

for (const entry of matrix.payload_cases) {
  exactKeys(
    entry,
    [
      "name",
      "source_case",
      "set_fields",
      "remove_fields",
      "expected",
      "assert_fields",
      "assert_absent",
    ],
    "payload case",
  );
  if (caseNames.has(entry.name)) fail(`duplicate compatibility case ${entry.name}`);
  caseNames.add(entry.name);

  const source = byName.get(entry.source_case);
  if (!source) fail(`unknown source case ${entry.source_case}`);
  const definition = schema.$defs?.[source.definition];
  if (!definition) fail(`unknown source definition ${String(source.definition)}`);

  const value = structuredClone(source.value);
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    fail(`${entry.name} source value must be an object`);
  }
  if (
    !entry.set_fields ||
    typeof entry.set_fields !== "object" ||
    Array.isArray(entry.set_fields)
  ) {
    fail(`${entry.name}.set_fields must be an object`);
  }
  if (!Array.isArray(entry.remove_fields)) fail(`${entry.name}.remove_fields must be an array`);

  for (const [field, fieldValue] of Object.entries(entry.set_fields)) value[field] = fieldValue;
  for (const field of entry.remove_fields) delete value[field];

  const observed = observedAcceptance(schema, definition, value);
  if (observed !== entry.expected) {
    fail(`${entry.name} expected ${entry.expected} but observed ${observed}`);
  }

  for (const [field, expected] of Object.entries(entry.assert_fields)) {
    if (!isDeepStrictEqual(value[field], expected)) {
      fail(`${entry.name} changed asserted field ${field}`);
    }
  }
  for (const field of entry.assert_absent) {
    if (Object.hasOwn(value, field)) fail(`${entry.name} expected ${field} to remain absent`);
  }

  if (observed === "accept") {
    const roundTrip = JSON.parse(JSON.stringify(value));
    if (!isDeepStrictEqual(roundTrip, value)) {
      fail(`${entry.name} changed semantic JSON during round trip`);
    }
  }
}

if (!Array.isArray(matrix.truth_cases) || matrix.truth_cases.length === 0) {
  fail("truth_cases must be a non-empty array");
}

for (const entry of matrix.truth_cases) {
  exactKeys(
    entry,
    [
      "name",
      "protocol_contract",
      "contact_state",
      "execution_state",
      "side_effect_certainty",
      "expected_admission",
      "expected_execution_state",
      "expected_side_effect_certainty",
    ],
    "truth case",
  );
  validateValue(
    schema,
    schema.$defs.ContactState,
    entry.contact_state,
    `${entry.name}.contact_state`,
  );
  validateValue(
    schema,
    schema.$defs.ExecutionState,
    entry.execution_state,
    `${entry.name}.execution_state`,
  );
  validateValue(
    schema,
    schema.$defs.SideEffectCertainty,
    entry.side_effect_certainty,
    `${entry.name}.side_effect_certainty`,
  );

  const admission = entry.protocol_contract === matrix.protocol_contract ? "accept" : "reject";
  if (admission !== entry.expected_admission) fail(`${entry.name} admission mismatch`);
  if (entry.execution_state !== entry.expected_execution_state) {
    fail(`${entry.name} rewrote execution truth`);
  }
  if (entry.side_effect_certainty !== entry.expected_side_effect_certainty) {
    fail(`${entry.name} rewrote side-effect certainty`);
  }
}

if (!Array.isArray(matrix.event_projection_cases) || matrix.event_projection_cases.length === 0) {
  fail("event_projection_cases must be a non-empty array");
}

for (const entry of matrix.event_projection_cases) {
  exactKeys(
    entry,
    ["name", "event_type", "negotiated_event_types", "expected_projection"],
    "event projection case",
  );
  validateValue(
    schema,
    schema.$defs.Event.properties.event_type,
    entry.event_type,
    `${entry.name}.event_type`,
  );
  if (!Array.isArray(entry.negotiated_event_types)) {
    fail(`${entry.name}.negotiated_event_types must be an array`);
  }
  const projection = entry.negotiated_event_types.includes(entry.event_type)
    ? "eligible"
    : "opaque_only";
  if (projection !== entry.expected_projection) {
    fail(`${entry.name} projection policy mismatch`);
  }
}

if (
  !Array.isArray(matrix.capability_admission_cases) ||
  matrix.capability_admission_cases.length === 0
) {
  fail("capability_admission_cases must be a non-empty array");
}

for (const entry of matrix.capability_admission_cases) {
  exactKeys(
    entry,
    [
      "name",
      "advertised_capabilities",
      "requested_action",
      "requested_version",
      "expected_admission",
    ],
    "capability admission case",
  );
  if (caseNames.has(entry.name)) fail("duplicate compatibility case " + entry.name);
  caseNames.add(entry.name);
  if (!Array.isArray(entry.advertised_capabilities)) {
    fail(entry.name + ".advertised_capabilities must be an array");
  }
  validateValue(
    schema,
    schema.$defs.RuntimeCapability.properties.action,
    entry.requested_action,
    entry.name + ".requested_action",
  );
  validateValue(
    schema,
    schema.$defs.CapabilityVersion,
    entry.requested_version,
    entry.name + ".requested_version",
  );
  for (const [index, capability] of entry.advertised_capabilities.entries()) {
    validateValue(
      schema,
      schema.$defs.RuntimeCapability,
      capability,
      entry.name + ".advertised_capabilities[" + index + "]",
    );
  }
  const admission = entry.advertised_capabilities.some(
    (capability) =>
      capability.action === entry.requested_action &&
      capability.version === entry.requested_version,
  )
    ? "accept"
    : "reject";
  if (admission !== entry.expected_admission) {
    fail(entry.name + " capability admission mismatch");
  }
}

if (!Array.isArray(matrix.feature_gate_cases) || matrix.feature_gate_cases.length === 0) {
  fail("feature_gate_cases must be a non-empty array");
}

for (const entry of matrix.feature_gate_cases) {
  exactKeys(
    entry,
    ["name", "feature", "field", "sender_features", "peer_features", "expected_emit"],
    "feature gate case",
  );
  if (schema.$defs?.OperationStart?.properties?.[entry.field]) {
    fail(`${entry.name} must probe a future additive field, not an existing field`);
  }
  if (!Array.isArray(entry.sender_features) || !Array.isArray(entry.peer_features)) {
    fail(`${entry.name} feature lists must be arrays`);
  }
  const actual = mayEmitAdditive(entry.feature, entry.sender_features, entry.peer_features);
  if (actual !== entry.expected_emit) fail(`${entry.name} additive sender gate mismatch`);
}

console.log(
  `KRP compatibility matrix: PASS (${matrix.payload_cases.length} payload, ${matrix.truth_cases.length} truth, ${matrix.event_projection_cases.length} event-projection, ${matrix.capability_admission_cases.length} capability-admission, ${matrix.feature_gate_cases.length} feature-gate cases)`,
);
