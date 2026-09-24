/**
 * Canonical Kernux design tokens (SG-000033 PR-A).
 *
 * This renderer-safe module owns spacing, typography, sizing, radii, surfaces,
 * borders, semantic status, focus, and motion. It exposes no host authority.
 */

export const SPACING = Object.freeze({
  none: "0rem",
  xxs: "0.125rem",
  xs: "0.25rem",
  sm: "0.5rem",
  md: "0.75rem",
  lg: "1rem",
  xl: "1.5rem",
  xxl: "2rem",
} as const);

export const TYPOGRAPHY = Object.freeze({
  body: Object.freeze({ family: "system-ui, sans-serif", size: "1rem", lineHeight: 1.5 }),
  label: Object.freeze({ family: "system-ui, sans-serif", size: "0.875rem", lineHeight: 1.4 }),
  heading: Object.freeze({ family: "system-ui, sans-serif", size: "1.5rem", lineHeight: 1.25 }),
  code: Object.freeze({ family: "ui-monospace, monospace", size: "0.875rem", lineHeight: 1.5 }),
} as const);

export const SIZE = Object.freeze({
  targetMinRem: "2.75rem",
  controlHeightRem: "2.5rem",
  focusWidthRem: "0.125rem",
  controlHeightCompactRem: "2rem",
} as const);

export const RADIUS = Object.freeze({
  none: "0rem",
  sm: "0.25rem",
  md: "0.5rem",
  pill: "999rem",
} as const);

export const BORDER = Object.freeze({
  width: "0.0625rem",
  strongWidth: "0.125rem",
} as const);

export const MOTION = Object.freeze({
  fastMs: 100,
  normalMs: 160,
  reducedMs: 0,
} as const);

export type MotionDuration = (typeof MOTION)[keyof typeof MOTION];

/** Select zero motion whenever the user requests reduced motion. */
export function motionDurationMs(
  duration: MotionDuration,
  prefersReducedMotion: boolean,
): MotionDuration {
  if (!Object.values(MOTION).includes(duration)) {
    throw new Error("unknown-motion-token");
  }
  return prefersReducedMotion ? MOTION.reducedMs : duration;
}

export const COLOR = Object.freeze({
  canvasLight: "#f7f8fa",
  surfaceLight: "#ffffff",
  textLight: "#172033",
  mutedTextLight: "#465166",
  borderLight: "#6b7280",
  focusLight: "#0b57d0",
  primaryLight: "#1749b3",
  primaryTextLight: "#ffffff",
  successLight: "#146c43",
  warningLight: "#6b4700",
  dangerLight: "#a51d2d",
  infoLight: "#175c87",
  canvasDark: "#11151c",
  surfaceDark: "#1b222d",
  textDark: "#f2f5f9",
  mutedTextDark: "#c1cad8",
  borderDark: "#9aa6b6",
  focusDark: "#8ab4f8",
  primaryDark: "#8ab4f8",
  primaryTextDark: "#0b1f3a",
  successDark: "#75d69b",
  warningDark: "#ffd166",
  dangerDark: "#ff9da6",
  infoDark: "#83cfff",
} as const);

export type TokenCategory =
  | "spacing"
  | "typography"
  | "size"
  | "radius"
  | "border"
  | "motion"
  | "color";

export type TokenPath = `${TokenCategory}.${string}`;
export type ColorTokenPath = Extract<TokenPath, `color.${string}`>;

export interface ContrastPair {
  readonly foreground: ColorTokenPath;
  readonly background: ColorTokenPath;
  readonly minimum: 3 | 4.5;
}

export const COLOR_CONTRAST_PAIRS = Object.freeze([
  Object.freeze({ foreground: "color.textLight", background: "color.canvasLight", minimum: 4.5 }),
  Object.freeze({ foreground: "color.textLight", background: "color.surfaceLight", minimum: 4.5 }),
  Object.freeze({
    foreground: "color.mutedTextLight",
    background: "color.surfaceLight",
    minimum: 4.5,
  }),
  Object.freeze({
    foreground: "color.primaryTextLight",
    background: "color.primaryLight",
    minimum: 4.5,
  }),
  Object.freeze({
    foreground: "color.successLight",
    background: "color.surfaceLight",
    minimum: 4.5,
  }),
  Object.freeze({
    foreground: "color.warningLight",
    background: "color.surfaceLight",
    minimum: 4.5,
  }),
  Object.freeze({
    foreground: "color.dangerLight",
    background: "color.surfaceLight",
    minimum: 4.5,
  }),
  Object.freeze({ foreground: "color.infoLight", background: "color.surfaceLight", minimum: 4.5 }),
  Object.freeze({ foreground: "color.borderLight", background: "color.surfaceLight", minimum: 3 }),
  Object.freeze({ foreground: "color.focusLight", background: "color.surfaceLight", minimum: 3 }),
  Object.freeze({ foreground: "color.focusLight", background: "color.canvasLight", minimum: 3 }),
  Object.freeze({ foreground: "color.textDark", background: "color.canvasDark", minimum: 4.5 }),
  Object.freeze({ foreground: "color.textDark", background: "color.surfaceDark", minimum: 4.5 }),
  Object.freeze({
    foreground: "color.mutedTextDark",
    background: "color.surfaceDark",
    minimum: 4.5,
  }),
  Object.freeze({
    foreground: "color.primaryTextDark",
    background: "color.primaryDark",
    minimum: 4.5,
  }),
  Object.freeze({ foreground: "color.successDark", background: "color.surfaceDark", minimum: 4.5 }),
  Object.freeze({ foreground: "color.warningDark", background: "color.surfaceDark", minimum: 4.5 }),
  Object.freeze({ foreground: "color.dangerDark", background: "color.surfaceDark", minimum: 4.5 }),
  Object.freeze({ foreground: "color.infoDark", background: "color.surfaceDark", minimum: 4.5 }),
  Object.freeze({ foreground: "color.borderDark", background: "color.surfaceDark", minimum: 3 }),
  Object.freeze({ foreground: "color.focusDark", background: "color.surfaceDark", minimum: 3 }),
  Object.freeze({ foreground: "color.focusDark", background: "color.canvasDark", minimum: 3 }),
] as const satisfies readonly ContrastPair[]);
/** Resolve a typed token reference without permitting arbitrary property access. */
export function token(path: TokenPath): unknown {
  const separator = path.indexOf(".");
  if (separator <= 0 || path.indexOf(".", separator + 1) !== -1) {
    throw new Error("invalid-token-reference");
  }
  const category = path.slice(0, separator) as TokenCategory;
  const key = path.slice(separator + 1);
  if (!Object.hasOwn(TOKENS, category) || !Object.hasOwn(TOKENS[category], key)) {
    throw new Error("unknown-token-reference");
  }
  return TOKENS[category][key];
}

/** Return the complete category surface for conformance tooling and tests. */
export function tokenCategories(): Readonly<Record<TokenCategory, readonly string[]>> {
  return Object.freeze(
    Object.fromEntries(
      (Object.keys(TOKENS) as TokenCategory[]).map((category) => [
        category,
        Object.freeze(Object.keys(TOKENS[category])),
      ]),
    ) as Record<TokenCategory, readonly string[]>,
  );
}

function channel(value: number): number {
  const normalized = value / 255;
  return normalized <= 0.04045 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4;
}

/** Calculate WCAG relative luminance; malformed colors fail closed. */
export function relativeLuminance(color: string): number {
  const match = /^#([0-9a-f]{6})$/u.exec(color.toLowerCase());
  if (match === null) {
    throw new Error("invalid-color");
  }
  const packed = Number.parseInt(match[1]!, 16);
  const channels = [(packed >> 16) & 0xff, (packed >> 8) & 0xff, packed & 0xff].map(channel);
  return 0.2126 * channels[0]! + 0.7152 * channels[1]! + 0.0722 * channels[2]!;
}

/** Calculate the WCAG contrast ratio for two strict six-digit sRGB colors. */
export function contrastRatio(foreground: string, background: string): number {
  const first = relativeLuminance(foreground);
  const second = relativeLuminance(background);
  return (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
}

const TOKENS: Readonly<Record<TokenCategory, Readonly<Record<string, unknown>>>> = Object.freeze({
  spacing: SPACING,
  typography: TYPOGRAPHY,
  size: SIZE,
  radius: RADIUS,
  border: BORDER,
  motion: MOTION,
  color: COLOR,
});
