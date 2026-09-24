import { rendererGraph } from "./authority.mjs";
import {
  isCallExpression,
  isIdentifier,
  isImportExpression,
  isJsxAttribute,
  isJsxElement,
  isJsxExpression,
  isJsxOpeningElement,
  isJsxSelfClosingElement,
  isJsxSpreadAttribute,
  isNoSubstitutionTemplateLiteral,
  isObjectLiteralExpression,
  isPropertyAssignment,
  isShorthandPropertyAssignment,
  isSpreadAssignment,
  isStringLiteral,
  visitEachChild,
} from "typescript/unstable/ast";

const STYLE_PROPERTIES = new Set([
  "background",
  "backgroundColor",
  "backgroundImage",
  "border",
  "borderColor",
  "borderRadius",
  "borderWidth",
  "boxShadow",
  "color",
  "columnGap",
  "fontFamily",
  "fontSize",
  "fontWeight",
  "gap",
  "height",
  "letterSpacing",
  "lineHeight",
  "margin",
  "maxHeight",
  "maxWidth",
  "minHeight",
  "minWidth",
  "outline",
  "padding",
  "rowGap",
  "style",
  "textTransform",
  "transition",
  "transitionDelay",
  "transitionDuration",
  "width",
  "animation",
  "animationDelay",
  "animationDuration",
]);
const RAW_VISUAL_STRING =
  /^(?:#[0-9a-f]{3,8}|(?:rgb|rgba|hsl|hsla)\(|\d+(?:\.\d+)?(?:px|rem|em|%|s|ms)$)/iu;
const ALLOWED_TAGS = new Set(["button", "output", "section", "span"]);
const ALLOWED_ATTRIBUTES = Object.freeze({
  button: new Set(["type", "disabled", "aria-busy", "aria-describedby"]),
  output: new Set(["aria-live", "data-tone"]),
  section: new Set(["aria-labelledby"]),
  span: new Set(["class"]),
});
const STATUS_TONES = new Set(["danger", "info", "neutral", "success", "warning"]);
export function checkPrimitiveStyle(root, files, parsed) {
  const diagnostics = [];
  for (const path of rendererGraph(parsed, root)) {
    const file = parsed.get(path)?.file;
    if (file !== undefined) inspectFile(path, file, diagnostics);
  }
  return diagnostics;
}

function inspectFile(path, file, diagnostics) {
  if (file === undefined) return;
  const visit = (node) => {
    if (isObjectLiteralExpression(node)) validatePrimitive(path, file, node, diagnostics);
    if (isJsxOpeningElement(node) || isJsxSelfClosingElement(node))
      inspectJsx(path, file, node, diagnostics);
    if (isJsxElement(node)) {
      const tag = isIdentifier(node.openingElement.tagName) ? node.openingElement.tagName.text : "";
      if (
        ["button", "output", "section", "span"].includes(tag) &&
        !node.children.some((child) => child.kind === 10 || isJsxExpression(child))
      )
        add(diagnostics, path, file, node, "missing-accessible-name", tag);
    }
    if (
      isPropertyAssignment(node) &&
      STYLE_PROPERTIES.has(propertyName(node)) &&
      isRawStyle(node.initializer)
    )
      add(diagnostics, path, file, node, "raw-visual-literal", propertyName(node));
    if (
      (isStringLiteral(node) || isNoSubstitutionTemplateLiteral(node)) &&
      RAW_VISUAL_STRING.test(node.text)
    )
      add(diagnostics, path, file, node, "raw-visual-literal", node.text);
    if (isImportExpression(node) && !isStringLiteral(node.argument))
      add(diagnostics, path, file, node, "dynamic-import", "nonliteral");
    return visitEachChild(node, visit);
  };
  visit(file);
}

function inspectJsx(path, file, node, diagnostics) {
  const tag = isIdentifier(node.tagName) ? node.tagName.text : "";
  if (isJsxSelfClosingElement(node) && ["button", "output", "section", "span"].includes(tag))
    add(diagnostics, path, file, node, "missing-accessible-name", tag);
  for (const property of node.attributes.properties) {
    if (isJsxSpreadAttribute(property))
      add(diagnostics, path, file, property, "dynamic-jsx-attributes", tag);
    if (
      !isJsxAttribute(property) ||
      (!isStringLiteral(property.name) && !isIdentifier(property.name))
    )
      continue;
    if (
      property.name.text === "role" &&
      property.initializer &&
      isStringLiteral(property.initializer) &&
      ["alert", "button", "region", "status"].includes(property.initializer.text)
    )
      add(diagnostics, path, file, property, "redundant-native-role", property.initializer.text);
    if (property.name.text === "aria-invalid" && tag === "button")
      add(diagnostics, path, file, property, "invalid-button-aria-invalid", tag);
    if (
      property.name.text === "style" &&
      property.initializer &&
      isStringLiteral(property.initializer)
    )
      add(diagnostics, path, file, property, "raw-visual-literal", property.initializer.text);
  }
}

function validatePrimitive(path, file, object, diagnostics) {
  const properties = new Map(
    object.properties
      .filter(isPropertyAssignment)
      .map((property) => [propertyName(property), property.initializer]),
  );
  const tag = stringValue(properties.get("tag"));
  if (!tag) return;
  if (!ALLOWED_TAGS.has(tag)) add(diagnostics, path, file, object, "invalid-primitive-tag", tag);
  if (!properties.has("attributes") || !isObjectLiteralExpression(properties.get("attributes"))) {
    add(diagnostics, path, file, object, "invalid-primitive-attributes", tag);
    return;
  }
  const attributes = properties.get("attributes");
  if (
    attributes.properties.some(isSpreadAssignment) ||
    attributes.properties.some(isShorthandPropertyAssignment)
  )
    add(diagnostics, path, file, attributes, "invalid-primitive-attributes", tag);
  const attrs = new Map(
    attributes.properties
      .filter(isPropertyAssignment)
      .map((property) => [propertyName(property), property.initializer]),
  );
  for (const name of attrs.keys())
    if (!ALLOWED_ATTRIBUTES[tag]?.has(name))
      add(diagnostics, path, file, attributes, "invalid-primitive-attribute", name);
  if (tag === "button") {
    if (stringValue(attrs.get("type")) !== "button")
      add(diagnostics, path, file, attributes, "invalid-primitive-attribute", "type");
    if (!hasText(properties.get("children")))
      add(diagnostics, path, file, object, "missing-accessible-name", tag);
  }
  if (
    tag === "output" &&
    (!hasText(properties.get("children")) ||
      !STATUS_TONES.has(stringValue(attrs.get("data-tone"))) ||
      !new Set(["polite", "assertive"]).has(stringValue(attrs.get("aria-live"))))
  )
    add(diagnostics, path, file, object, "invalid-status-primitive", tag);
  if (
    tag === "section" &&
    !/^[A-Za-z][A-Za-z0-9_-]*$/u.test(stringValue(attrs.get("aria-labelledby")))
  )
    add(diagnostics, path, file, object, "invalid-surface-primitive", tag);
  if (
    tag === "span" &&
    (!hasText(properties.get("children")) ||
      stringValue(attrs.get("class")) !== "kernux-visually-hidden")
  )
    add(diagnostics, path, file, object, "invalid-visually-hidden-primitive", tag);
  if (attrs.has("aria-busy") && stringValue(attrs.get("disabled")) !== "true")
    add(diagnostics, path, file, attributes, "invalid-loading-primitive", tag);
}

function propertyName(property) {
  return isIdentifier(property.name) || isStringLiteral(property.name) ? property.name.text : "";
}
function stringValue(node) {
  return node !== undefined && isStringLiteral(node) ? node.text : "";
}
function hasText(node) {
  return (
    node?.kind === 235 &&
    node.elements.some((element) => isStringLiteral(element) && element.text.trim().length > 0)
  );
}
function isRawStyle(node) {
  return (
    node !== undefined &&
    !isCallExpression(node) &&
    (node.kind === 8 || node.kind === 10 || isStringLiteral(node))
  );
}
function add(diagnostics, path, file, node, code, detail) {
  diagnostics.push({
    path,
    line: file.getLineAndCharacterOfPosition(node.pos).line + 1,
    code,
    detail,
  });
}
