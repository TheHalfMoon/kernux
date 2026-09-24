/** Design-token authority and WCAG foundation corpus (SG-000033 PR-A). */
import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  BORDER,
  COLOR,
  COLOR_CONTRAST_PAIRS,
  MOTION,
  RADIUS,
  SIZE,
  SPACING,
  TYPOGRAPHY,
  contrastRatio,
  motionDurationMs,
  relativeLuminance,
  token,
  tokenCategories,
} from "../../../packages/ui-system/tokens.js";
import { KERNUX_STYLE_SHEET } from "../../../packages/ui-system/styles.js";
import {
  STATUS_TONES,
  buttonPrimitive,
  statusPrimitive,
  surfacePrimitive,
  visuallyHiddenPrimitive,
} from "../../../packages/ui-system/primitives.js";

const EXPECTED_CATEGORIES = Object.freeze({
  spacing: ["none", "xxs", "xs", "sm", "md", "lg", "xl", "xxl"],
  typography: ["body", "label", "heading", "code"],
  size: ["targetMinRem", "controlHeightRem", "focusWidthRem", "controlHeightCompactRem"],
  radius: ["none", "sm", "md", "pill"],
  border: ["width", "strongWidth"],
  motion: ["fastMs", "normalMs", "reducedMs"],
  color: [
    "canvasLight",
    "surfaceLight",
    "textLight",
    "mutedTextLight",
    "borderLight",
    "focusLight",
    "primaryLight",
    "primaryTextLight",
    "successLight",
    "warningLight",
    "dangerLight",
    "infoLight",
    "canvasDark",
    "surfaceDark",
    "textDark",
    "mutedTextDark",
    "borderDark",
    "focusDark",
    "primaryDark",
    "primaryTextDark",
    "successDark",
    "warningDark",
    "dangerDark",
    "infoDark",
  ],
});

const EXPECTED_CONTRAST_PAIRS = Object.freeze([
  ["color.textLight", "color.canvasLight", 4.5],
  ["color.textLight", "color.surfaceLight", 4.5],
  ["color.mutedTextLight", "color.surfaceLight", 4.5],
  ["color.primaryTextLight", "color.primaryLight", 4.5],
  ["color.successLight", "color.surfaceLight", 4.5],
  ["color.warningLight", "color.surfaceLight", 4.5],
  ["color.dangerLight", "color.surfaceLight", 4.5],
  ["color.infoLight", "color.surfaceLight", 4.5],
  ["color.borderLight", "color.surfaceLight", 3],
  ["color.focusLight", "color.surfaceLight", 3],
  ["color.focusLight", "color.canvasLight", 3],
  ["color.textDark", "color.canvasDark", 4.5],
  ["color.textDark", "color.surfaceDark", 4.5],
  ["color.mutedTextDark", "color.surfaceDark", 4.5],
  ["color.primaryTextDark", "color.primaryDark", 4.5],
  ["color.successDark", "color.surfaceDark", 4.5],
  ["color.warningDark", "color.surfaceDark", 4.5],
  ["color.dangerDark", "color.surfaceDark", 4.5],
  ["color.infoDark", "color.surfaceDark", 4.5],
  ["color.borderDark", "color.surfaceDark", 3],
  ["color.focusDark", "color.surfaceDark", 3],
  ["color.focusDark", "color.canvasDark", 3],
] as const);

describe("renderer style contract", () => {
  it("preserves focus-visible, reduced-motion, and visually-hidden behavior", () => {
    assert.ok(/:focus-visible/u.test(KERNUX_STYLE_SHEET));
    assert.ok(/prefers-reduced-motion: reduce/u.test(KERNUX_STYLE_SHEET));
    assert.ok(/kernux-visually-hidden/u.test(KERNUX_STYLE_SHEET));
    assert.ok(/--kernux-reduced-motion: 0ms;/u.test(KERNUX_STYLE_SHEET));
  });
});

describe("canonical token authority is closed and deeply frozen", () => {
  it("has the exact frozen category surface", () => {
    assert.deepEqual(tokenCategories(), EXPECTED_CATEGORIES);
    for (const [category, keys] of Object.entries(tokenCategories())) {
      assert.equal(Object.isFrozen(keys), true, category);
      assert.ok(keys.length > 0, category);
    }
  });

  it("freezes every object in the token graph", () => {
    const values: object[] = [
      SPACING,
      TYPOGRAPHY,
      ...Object.values(TYPOGRAPHY),
      SIZE,
      RADIUS,
      BORDER,
      MOTION,
      COLOR,
      COLOR_CONTRAST_PAIRS,
    ];
    for (const value of values) {
      assert.equal(Object.isFrozen(value), true);
    }
    assert.equal(Object.isFrozen(COLOR_CONTRAST_PAIRS), true);
    for (const pair of COLOR_CONTRAST_PAIRS) {
      assert.equal(Object.isFrozen(pair), true);
    }
  });

  it("resolves only known one-level references", () => {
    assert.equal(token("spacing.md"), SPACING.md);
    assert.equal(token("color.successLight"), COLOR.successLight);
    assert.throws(
      () => token("spacing.md.extra" as "spacing.md"),
      (error: unknown) => error instanceof Error && error.message === "invalid-token-reference",
    );
    assert.throws(
      () => token("spacing.missing" as "spacing.md"),
      (error: unknown) => error instanceof Error && error.message === "unknown-token-reference",
    );
    assert.throws(
      () => token("missing.md" as "spacing.md"),
      (error: unknown) => error instanceof Error && error.message === "unknown-token-reference",
    );
  });

  it("rejects inherited and prototype-like token keys", () => {
    for (const path of ["spacing.toString", "color.__proto__", "color.constructor"]) {
      assert.throws(
        () => token(path as "spacing.md"),
        (error: unknown) => error instanceof Error && error.message === "unknown-token-reference",
      );
    }
  });
});

describe("semantic color pairs meet WCAG AA", () => {
  it("freezes the exact semantic pair corpus without duplicates or weakened thresholds", () => {
    const actual = COLOR_CONTRAST_PAIRS.map(({ foreground, background, minimum }) => [
      foreground,
      background,
      minimum,
    ]);
    assert.deepEqual(actual, EXPECTED_CONTRAST_PAIRS);
    assert.equal(
      new Set(actual.map(([foreground, background]) => `${foreground}/${background}`)).size,
      actual.length,
    );
    for (const pair of COLOR_CONTRAST_PAIRS) {
      const ratio = contrastRatio(
        token(pair.foreground) as string,
        token(pair.background) as string,
      );
      assert.ok(ratio >= pair.minimum, `${pair.foreground}/${pair.background}: ${ratio}`);
    }
  });

  it("calculates known black-on-white and white-on-black ratios", () => {
    assert.equal(contrastRatio("#000000", "#ffffff"), 21);
    assert.equal(contrastRatio("#ffffff", "#000000"), 21);
    assert.equal(relativeLuminance("#ffffff"), 1);
    assert.equal(relativeLuminance("#000000"), 0);
  });

  it("fails closed for malformed, short, and non-hex colors", () => {
    for (const color of ["", "#fff", "white", "#12345g", "rgb(0,0,0)", "#00000000"]) {
      assert.throws(
        () => relativeLuminance(color),
        (error: unknown) => error instanceof Error && error.message === "invalid-color",
      );
    }
  });
});

describe("semantic primitives preserve native accessibility", () => {
  it("uses a native button without a redundant role and explicit name", () => {
    assert.deepEqual(buttonPrimitive({ label: "Run task" }), {
      tag: "button",
      attributes: { type: "button" },
      children: ["Run task"],
    });
  });

  it("uses native disabled and busy semantics independently while preserving the label", () => {
    const disabled = buttonPrimitive({ label: "Run task", disabled: true });
    assert.equal(disabled.attributes.disabled, true);
    assert.equal(disabled.attributes["aria-busy"], undefined);

    const loading = buttonPrimitive({ label: "Run task", loading: true });
    assert.equal(loading.tag, "button");
    assert.equal(loading.attributes.disabled, true);
    assert.equal(loading.attributes["aria-busy"], true);
    assert.equal(loading.attributes.role, undefined);
    assert.deepEqual(loading.children, ["Run task"]);
  });

  it("associates button errors with a validated description", () => {
    const primitive = buttonPrimitive({ label: "Save", errorDescriptionId: "save-error" });
    assert.equal(primitive.attributes["aria-describedby"], "save-error");
    assert.equal(primitive.attributes["aria-invalid"], undefined);
    assert.throws(
      () => buttonPrimitive({ label: "Save", errorDescriptionId: "../../bad id" }),
      (error: unknown) => error instanceof Error && error.message === "error-description-id",
    );
  });

  it("uses output status with text and tone, never color alone", () => {
    assert.deepEqual(STATUS_TONES, ["neutral", "info", "success", "warning", "danger"]);
    for (const tone of STATUS_TONES) {
      const primitive = statusPrimitive({ tone, message: `Status: ${tone}` });
      assert.equal(primitive.tag, "output");
      assert.equal(primitive.attributes.role, undefined);
      assert.equal(primitive.attributes["data-tone"], tone);
      assert.equal(primitive.attributes["aria-live"], tone === "danger" ? "assertive" : "polite");
      assert.deepEqual(primitive.children, [`Status: ${tone}`]);
    }
  });

  it("uses a named native section and a non-hidden semantic span", () => {
    const surface = surfacePrimitive({ labelId: "workspace-heading" });
    assert.equal(surface.tag, "section");
    assert.equal(surface.attributes["aria-labelledby"], "workspace-heading");
    assert.equal(surface.attributes.role, undefined);

    const hidden = visuallyHiddenPrimitive({ text: "Current project" });
    assert.equal(hidden.tag, "span");
    assert.equal(hidden.attributes.class, "kernux-visually-hidden");
    assert.equal(hidden.attributes["aria-hidden"], undefined);
    assert.deepEqual(hidden.children, ["Current project"]);
  });

  it("freezes descriptor graphs and rejects empty text or invalid ids", () => {
    const primitive = buttonPrimitive({ label: "Run" });
    assert.equal(Object.isFrozen(primitive), true);
    assert.equal(Object.isFrozen(primitive.attributes), true);
    assert.equal(Object.isFrozen(primitive.children), true);
    for (const create of [
      () => buttonPrimitive({ label: " " }),
      () => buttonPrimitive({ label: "Run", disabled: "false" as unknown as boolean }),
      () => buttonPrimitive({ label: "Run", loading: 0 as unknown as boolean }),
      () => statusPrimitive({ tone: "red" as "danger", message: "Failed" }),
      () => surfacePrimitive({ labelId: "" }),
      () => visuallyHiddenPrimitive({ text: "" }),
    ]) {
      assert.throws(create);
    }

    let reads = 0;
    const hostileTone = {
      get tone(): "danger" {
        reads += 1;
        return reads === 1 ? "danger" : ("__proto__" as "danger");
      },
      message: "Failed",
    };
    const captured = statusPrimitive(hostileTone);
    assert.equal(captured.attributes["data-tone"], "danger");
    assert.equal(reads, 1);
  });
});

describe("scalable accessibility foundations are closed", () => {
  it("uses rem sizing, relative line heights, and non-negative motion", () => {
    assert.ok(Object.values(SPACING).every((value) => value.endsWith("rem")));
    assert.ok(Object.values(SIZE).every((value) => value.endsWith("rem")));
    for (const value of Object.values(TYPOGRAPHY)) {
      assert.ok(value.size.endsWith("rem"));
      assert.ok(value.lineHeight > 1);
    }
    assert.ok(Object.values(MOTION).every((value) => value >= 0));
    assert.equal(motionDurationMs(MOTION.normalMs, false), MOTION.normalMs);
    assert.equal(motionDurationMs(MOTION.normalMs, true), MOTION.reducedMs);
    assert.equal(motionDurationMs(MOTION.fastMs, true), MOTION.reducedMs);
    assert.throws(
      () => motionDurationMs(999 as 160, false),
      (error: unknown) => error instanceof Error && error.message === "unknown-motion-token",
    );
    assert.ok(Object.values(RADIUS).every((value) => value.endsWith("rem")));
    assert.ok(Object.values(BORDER).every((value) => value.endsWith("rem")));
  });
});
