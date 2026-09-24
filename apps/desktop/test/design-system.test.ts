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
  relativeLuminance,
  token,
  tokenCategories,
} from "../../../packages/ui-system/tokens.js";

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
    "primaryDark",
    "primaryTextDark",
    "successDark",
    "warningDark",
    "dangerDark",
    "infoDark",
  ],
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
    ];
    for (const value of values) {
      assert.equal(Object.isFrozen(value), true);
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
  it("covers light and dark text, controls, and all status meanings", () => {
    assert.ok(COLOR_CONTRAST_PAIRS.length >= 16);
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

describe("scalable accessibility foundations are closed", () => {
  it("uses rem sizing, relative line heights, and non-negative motion", () => {
    assert.ok(Object.values(SPACING).every((value) => value.endsWith("rem")));
    assert.ok(Object.values(SIZE).every((value) => value.endsWith("rem")));
    for (const value of Object.values(TYPOGRAPHY)) {
      assert.ok(value.size.endsWith("rem"));
      assert.ok(value.lineHeight > 1);
    }
    assert.ok(Object.values(MOTION).every((value) => value >= 0));
    assert.equal(MOTION.reducedMs, 0);
    assert.ok(Object.values(RADIUS).every((value) => value.endsWith("rem")));
    assert.ok(Object.values(BORDER).every((value) => value.endsWith("rem")));
  });
});
