import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const schemaPath = resolve(root, "protocol/schema/krp.v1.schema.json");
const coreFixturePath = resolve(root, "protocol/fixtures/v1/core.json");
const adversarialPath = resolve(root, "protocol/fixtures/v1/adversarial.json");
const fixtureContract = "kernux.krp.adversarial/v1";
const requiredInvariantIds = Array.from(
  { length: 20 },
  (_, index) => `ADV-${String(index + 1).padStart(2, "0")}`,
);
const expectedEvidenceBoundary = [
  "exactly_once_execution",
  "durable_replay_protection",
  "cancellation_delivery",
  "reconnect_persistence",
  "runtime_transport_behavior",
];
const requiredPolicyRules = new Map([
  ["ADV-01", "contact_loss_preserves_execution"],
  ["ADV-02", "timeout_preserves_execution"],
  ["ADV-03", "cancellation_acceptance_preserves_execution"],
  ["ADV-04", "cancellation_transport_loss_is_unverifiable"],
  ["ADV-05", "stale_observation_is_rejected"],
  ["ADV-06", "same_fingerprint_deduplicates"],
  ["ADV-07", "fingerprint_conflict_rejects"],
  ["ADV-08", "ambiguous_start_preserves_operation_id"],
  ["ADV-09", "may_have_started_forbids_fresh_operation"],
  ["ADV-10", "unknown_retry_guidance_reconciles"],
  ["ADV-11", "authority_requires_intersection"],
  ["ADV-12", "authority_requires_intersection"],
  ["ADV-13", "runtime_type_does_not_imply_capability"],
  ["ADV-14", "reconnect_does_not_restore_grant"],
  ["ADV-15", "trust_identity_change_is_new_runtime"],
  ["ADV-16", "provider_diagnostic_cannot_lower_uncertainty"],
  ["ADV-17", "request_timeout_is_not_operation_failure"],
  ["ADV-18", "controller_cache_loss_is_not_exit"],
  ["ADV-19", "equal_generation_conflict_fails"],
  ["ADV-20", "unknown_operation_preserves_uncertainty"],
]);
const requiredWireCases = new Map([
  ["WIRE-01", "unknown_operation_start_field"],
  ["WIRE-02", "missing_operation_start_protocol_contract"],
  ["WIRE-03", "malformed_operation_uuid"],
  ["WIRE-04", "malformed_payload_digest"],
  ["WIRE-05", "malformed_resource_uri"],
  ["WIRE-06", "unknown_error_category"],
  ["WIRE-07", "unknown_retry_guidance"],
  ["WIRE-08", "malformed_nested_runtime_revision"],
  ["WIRE-09", "runtime_descriptor_protocol_mismatch"],
  ["WIRE-10", "runtime_capability_version_mismatch"],
  ["WIRE-11", "operation_protocol_mismatch"],
  ["WIRE-12", "operation_capability_version_mismatch"],
  ["WIRE-13", "event_contract_version_mismatch"],
]);

function fail(message) {
  throw new Error(`KRP adversarial conformance failed: ${message}`);
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
    if (
      !(error instanceof Error) ||
      !error.message.startsWith("KRP adversarial conformance failed:")
    ) {
      throw error;
    }
    return "reject";
  }
}

function setAtPath(value, path, replacement) {
  if (!Array.isArray(path) || path.length === 0) fail("set mutation path must be non-empty");
  let target = value;
  for (const segment of path.slice(0, -1)) {
    if (!target || typeof target !== "object" || Array.isArray(target)) {
      fail(`set mutation cannot traverse ${path.join(".")}`);
    }
    target = target[segment];
  }
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    fail(`set mutation cannot write ${path.join(".")}`);
  }
  target[path.at(-1)] = replacement;
}

function removeAtPath(value, path) {
  if (!Array.isArray(path) || path.length === 0) fail("remove mutation path must be non-empty");
  let target = value;
  for (const segment of path.slice(0, -1)) {
    if (!target || typeof target !== "object" || Array.isArray(target)) {
      fail(`remove mutation cannot traverse ${path.join(".")}`);
    }
    target = target[segment];
  }
  if (!target || typeof target !== "object" || Array.isArray(target)) {
    fail(`remove mutation cannot delete ${path.join(".")}`);
  }
  if (!Object.hasOwn(target, path.at(-1))) {
    fail(`remove mutation target ${path.join(".")} is absent`);
  }
  delete target[path.at(-1)];
}

function applyMutation(value, mutation, caseId) {
  exactKeys(
    mutation,
    ["op", "path", ...(mutation.op === "set" ? ["value"] : [])],
    `${caseId}.mutation`,
  );
  if (!Array.isArray(mutation.path) || !mutation.path.every((item) => typeof item === "string")) {
    fail(`${caseId}.mutation.path must be a string array`);
  }
  if (mutation.op === "none") {
    if (mutation.path.length !== 0) fail(`${caseId} none mutation must have an empty path`);
    return value;
  }
  if (mutation.op === "set") {
    setAtPath(value, mutation.path, structuredClone(mutation.value));
    return value;
  }
  if (mutation.op === "remove") {
    removeAtPath(value, mutation.path);
    return value;
  }
  fail(`${caseId} has unsupported mutation operation ${String(mutation.op)}`);
}

function computePolicyOutcome(entry) {
  const input = entry.input;
  switch (entry.rule) {
    case "contact_loss_preserves_execution":
    case "timeout_preserves_execution":
    case "cancellation_acceptance_preserves_execution":
      return { execution_state: input.execution_state };

    case "cancellation_transport_loss_is_unverifiable":
      return { cancellation_state: "unverifiable" };

    case "stale_observation_is_rejected":
      if (input.incoming_generation >= input.current_generation) {
        fail(`${entry.id} must use a stale incoming generation`);
      }
      return { disposition: "stale_rejected", execution_state: input.current_state };

    case "same_fingerprint_deduplicates":
      if (input.stored_fingerprint !== input.incoming_fingerprint) {
        fail(`${entry.id} must use identical fingerprints`);
      }
      return { disposition: "reconcile_existing", execute: false };

    case "fingerprint_conflict_rejects":
      if (input.stored_fingerprint === input.incoming_fingerprint) {
        fail(`${entry.id} must use different fingerprints`);
      }
      return { disposition: "operation_conflict", execute: false };

    case "ambiguous_start_preserves_operation_id":
      return { operation_id: input.operation_id, retry_guidance: "reconcile_operation" };

    case "may_have_started_forbids_fresh_operation":
      if (input.side_effect_certainty !== "may_have_started") {
        fail(`${entry.id} must model may_have_started`);
      }
      return { create_fresh_operation: false, operation_id: input.operation_id };

    case "unknown_retry_guidance_reconciles":
      if (input.side_effect_certainty !== "may_have_started") {
        fail(`${entry.id} must preserve side-effect uncertainty`);
      }
      return { blind_retry: false, retry_guidance: "reconcile_operation" };

    case "authority_requires_intersection":
      return {
        authorized:
          input.advertised_capability === true &&
          input.grant === true &&
          input.policy_allows === true &&
          input.negotiated_capability === true,
      };

    case "runtime_type_does_not_imply_capability":
      return { authorized: input.advertised_actions.includes(input.requested_action) };

    case "reconnect_does_not_restore_grant":
      return { authorized: input.grant_status === "active" };

    case "trust_identity_change_is_new_runtime":
      return { same_runtime: input.enrolled_identity === input.observed_identity };

    case "provider_diagnostic_cannot_lower_uncertainty":
      return { side_effect_certainty: input.canonical_side_effect_certainty };

    case "request_timeout_is_not_operation_failure":
      return {
        execution_state:
          input.authoritative_execution_observation === null
            ? "unverifiable"
            : input.authoritative_execution_observation,
      };

    case "controller_cache_loss_is_not_exit":
      return {
        execution_state: input.authoritative_exit_observation === true ? "exited" : "unverifiable",
      };

    case "equal_generation_conflict_fails":
      if (
        input.incoming_generation !== input.current_generation ||
        input.incoming_state === input.current_state
      ) {
        fail(`${entry.id} must model contradictory equal-generation observations`);
      }
      return { disposition: "invariant_conflict" };

    case "unknown_operation_preserves_uncertainty":
      if (input.operation_lookup !== "unknown") {
        fail(`${entry.id} must model an unknown operation`);
      }
      if (input.persistence_guarantee_definitely_not_started === true) {
        return {
          retry_guidance: "retry_same_operation",
          side_effect_certainty: "definitely_not_started",
        };
      }
      return {
        retry_guidance: "reconcile_operation",
        side_effect_certainty: "may_have_started",
      };

    default:
      fail(`${entry.id} has unsupported policy rule ${String(entry.rule)}`);
  }
}

const [schema, core, fixture] = await Promise.all(
  [schemaPath, coreFixturePath, adversarialPath].map(async (path) =>
    JSON.parse(await readFile(path, "utf8")),
  ),
);

exactKeys(
  fixture,
  ["schema", "required_invariant_ids", "policy_cases", "wire_cases", "evidence_boundary"],
  "fixture",
);
if (fixture.schema !== fixtureContract) fail(`fixture schema must be ${fixtureContract}`);

if (!isDeepStrictEqual(fixture.required_invariant_ids, requiredInvariantIds)) {
  fail("required_invariant_ids must enumerate ADV-01 through ADV-20 exactly once and in order");
}

if (
  !Array.isArray(fixture.policy_cases) ||
  fixture.policy_cases.length !== requiredInvariantIds.length
) {
  fail("policy_cases must contain exactly twenty cases");
}

const policyIds = new Set();
for (const entry of fixture.policy_cases) {
  exactKeys(entry, ["id", "rule", "input", "expected"], "policy case");
  if (!requiredInvariantIds.includes(entry.id)) fail(`unknown policy case id ${String(entry.id)}`);
  if (policyIds.has(entry.id)) fail(`duplicate policy case id ${entry.id}`);
  policyIds.add(entry.id);
  if (typeof entry.rule !== "string" || !entry.rule) fail(`${entry.id}.rule must be non-empty`);
  const requiredRule = requiredPolicyRules.get(entry.id);
  if (entry.rule !== requiredRule) {
    fail(`${entry.id}.rule must be ${requiredRule}`);
  }
  if (!entry.input || typeof entry.input !== "object" || Array.isArray(entry.input)) {
    fail(`${entry.id}.input must be an object`);
  }
  if (!entry.expected || typeof entry.expected !== "object" || Array.isArray(entry.expected)) {
    fail(`${entry.id}.expected must be an object`);
  }

  const observed = computePolicyOutcome(entry);
  if (!isDeepStrictEqual(observed, entry.expected)) {
    fail(
      `${entry.id} expected ${JSON.stringify(entry.expected)} but observed ${JSON.stringify(observed)}`,
    );
  }
}

for (const requiredId of requiredInvariantIds) {
  if (!policyIds.has(requiredId)) fail(`missing policy case ${requiredId}`);
}

if (!Array.isArray(fixture.wire_cases) || fixture.wire_cases.length !== requiredWireCases.size) {
  fail("wire_cases must contain exactly the thirteen required cases");
}

const byName = sourceCases(core);
const wireIds = new Set();
const versionDefinitions = new Set();
const allowedRustExpectations = new Set(["reject", "schema_only"]);
const allowedSchemaOnlyCases = new Set([
  "malformed_operation_uuid",
  "malformed_payload_digest",
  "malformed_resource_uri",
]);

for (const entry of fixture.wire_cases) {
  const keys = [
    "id",
    "name",
    "definition",
    ...(Object.hasOwn(entry, "source_case") ? ["source_case"] : ["inline_value"]),
    "mutation",
    "expected_node",
    "expected_rust",
  ];
  exactKeys(entry, keys, "wire case");

  if (typeof entry.id !== "string" || !/^WIRE-[0-9]{2}$/.test(entry.id)) {
    fail("wire case id must match WIRE-NN");
  }
  if (wireIds.has(entry.id)) fail(`duplicate wire case id ${entry.id}`);
  wireIds.add(entry.id);
  const requiredWireName = requiredWireCases.get(entry.id);
  if (!requiredWireName) fail(`unexpected wire case id ${entry.id}`);

  if (typeof entry.name !== "string" || !/^[a-z][a-z0-9_]*$/.test(entry.name)) {
    fail(`${entry.id}.name is invalid`);
  }
  if (entry.name !== requiredWireName) {
    fail(`${entry.id}.name must be ${requiredWireName}`);
  }
  const definition = schema.$defs?.[entry.definition];
  if (!definition) fail(`${entry.id} references unknown definition ${String(entry.definition)}`);

  let value;
  if (Object.hasOwn(entry, "source_case")) {
    const source = byName.get(entry.source_case);
    if (!source) fail(`${entry.id} references unknown source case ${entry.source_case}`);
    if (source.definition !== entry.definition) {
      fail(`${entry.id} source definition ${source.definition} does not match ${entry.definition}`);
    }
    value = structuredClone(source.value);
  } else {
    value = structuredClone(entry.inline_value);
  }

  applyMutation(value, entry.mutation, entry.id);

  const observedNode = observedAcceptance(schema, definition, value);
  if (entry.expected_node !== observedNode) {
    fail(`${entry.id} expected Node ${entry.expected_node} but observed ${observedNode}`);
  }
  if (!allowedRustExpectations.has(entry.expected_rust)) {
    fail(`${entry.id}.expected_rust is unsupported`);
  }
  if (entry.expected_rust === "schema_only" && !allowedSchemaOnlyCases.has(entry.name)) {
    fail(`${entry.id} uses schema_only outside the explicitly pattern-validated cases`);
  }
  if (entry.expected_rust === "reject" && allowedSchemaOnlyCases.has(entry.name)) {
    fail(`${entry.id} must preserve the schema-only Rust expectation boundary`);
  }

  if (entry.name.includes("mismatch")) versionDefinitions.add(entry.definition);
}

for (const [requiredId] of requiredWireCases) {
  if (!wireIds.has(requiredId)) fail(`missing wire case ${requiredId}`);
}

for (const definition of ["RuntimeDescriptor", "RuntimeCapability", "OperationStart", "Event"]) {
  if (!versionDefinitions.has(definition)) {
    fail(`version mismatch coverage is missing ${definition}`);
  }
}

exactKeys(fixture.evidence_boundary, ["proves", "does_not_prove"], "evidence_boundary");
if (fixture.evidence_boundary.proves !== "contract_and_policy_fixture_conformance") {
  fail("evidence_boundary.proves is invalid");
}
if (!isDeepStrictEqual(fixture.evidence_boundary.does_not_prove, expectedEvidenceBoundary)) {
  fail("evidence_boundary.does_not_prove must preserve the exact runtime-evidence exclusions");
}

const fixtureText = JSON.stringify(fixture).toLowerCase();
for (const provider of [
  "openai",
  "anthropic",
  "claude",
  "gemini",
  "qwen",
  "ollama",
  "mistral",
  "groq",
]) {
  if (fixtureText.includes(provider)) fail(`provider-neutral fixture contains ${provider}`);
}

console.log(
  `KRP adversarial conformance: PASS (${fixture.policy_cases.length} policy, ${fixture.wire_cases.length} wire cases)`,
);
