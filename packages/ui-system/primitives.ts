/**
 * Renderer-agnostic semantic primitives for the Kernux design system.
 *
 * These descriptors select native HTML elements and explicit accessibility
 * attributes. They do not import React, access a host bridge, or create capability.
 */

export const STATUS_TONES = Object.freeze([
  "neutral",
  "info",
  "success",
  "warning",
  "danger",
] as const);
export type StatusTone = (typeof STATUS_TONES)[number];
export type PrimitiveTag = "button" | "output" | "section" | "span";
export type PrimitiveAttributeValue = string | boolean | number;
export type PrimitiveAttributes = Readonly<Record<string, PrimitiveAttributeValue>>;
export type PrimitiveNode = string | PrimitiveElement;

export interface PrimitiveElement {
  readonly tag: PrimitiveTag;
  readonly attributes: PrimitiveAttributes;
  readonly children: readonly PrimitiveNode[];
}

export interface ButtonPrimitiveOptions {
  readonly label: string;
  readonly disabled?: boolean;
  readonly loading?: boolean;
  readonly errorDescriptionId?: string;
}

export interface StatusPrimitiveOptions {
  readonly tone: StatusTone;
  readonly message: string;
}

export interface SurfacePrimitiveOptions {
  readonly labelId: string;
}

export interface VisuallyHiddenPrimitiveOptions {
  readonly text: string;
}

const IDENTIFIER = /^[A-Za-z][A-Za-z0-9_-]*$/u;

export function buttonPrimitive(options: ButtonPrimitiveOptions): PrimitiveElement {
  const label = requiredText(options.label, "button-label");
  const loading = optionalBoolean(options.loading, false, "loading-state");
  const disabled = optionalBoolean(options.disabled, false, "disabled-state");
  const attributes: Record<string, PrimitiveAttributeValue> = {
    type: "button",
  };

  if (disabled || loading) {
    attributes.disabled = true;
  }
  if (loading) {
    attributes["aria-busy"] = true;
  }
  const errorDescriptionId = options.errorDescriptionId;
  if (errorDescriptionId !== undefined) {
    attributes["aria-describedby"] = identifier(errorDescriptionId, "error-description-id");
  }

  return element("button", attributes, [label]);
}

export function statusPrimitive(options: StatusPrimitiveOptions): PrimitiveElement {
  const tone = options.tone;
  if (!STATUS_TONES.includes(tone)) {
    throw new Error("invalid-status-tone");
  }
  const attributes: Record<string, PrimitiveAttributeValue> = {
    "aria-live": tone === "danger" ? "assertive" : "polite",
    "data-tone": tone,
  };
  return element("output", attributes, [requiredText(options.message, "status-message")]);
}

export function surfacePrimitive(options: SurfacePrimitiveOptions): PrimitiveElement {
  return element(
    "section",
    { "aria-labelledby": identifier(options.labelId, "surface-label-id") },
    [],
  );
}

export function visuallyHiddenPrimitive(options: VisuallyHiddenPrimitiveOptions): PrimitiveElement {
  return element("span", { class: "kernux-visually-hidden" }, [
    requiredText(options.text, "visually-hidden-text"),
  ]);
}

function element(
  tag: PrimitiveTag,
  attributes: Record<string, PrimitiveAttributeValue>,
  children: PrimitiveNode[],
): PrimitiveElement {
  return Object.freeze({
    tag,
    attributes: Object.freeze(attributes),
    children: Object.freeze(children),
  });
}

function requiredText(value: string, code: string): string {
  if (typeof value !== "string" || value.trim().length === 0) {
    throw new Error(code);
  }
  return value;
}

function optionalBoolean(value: boolean | undefined, fallback: boolean, code: string): boolean {
  if (value === undefined) {
    return fallback;
  }
  if (typeof value !== "boolean") {
    throw new Error(code);
  }
  return value;
}

function identifier(value: string, code: string): string {
  const normalized = requiredText(value, code).trim();
  if (!IDENTIFIER.test(normalized)) {
    throw new Error(code);
  }
  return normalized;
}
