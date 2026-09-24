/**
 * Frozen unprivileged window policy (SG-000031 PR-A).
 *
 * The desktop renderer must never receive ambient Node or arbitrary host
 * authority. These values are the single source of truth for the Electron
 * BrowserWindow configuration; any deviation fails the policy tests.
 * Pure module with zero imports.
 */

/** Renderer security flags: exact, closed, and non-negotiable. */
export interface RendererSecurityFlags {
  readonly contextIsolation: true;
  readonly sandbox: true;
  readonly nodeIntegration: false;
  readonly webSecurity: true;
  readonly allowRunningInsecureContent: false;
}

/** Frozen renderer security flags for every Kernux workspace window. */
export const RENDERER_SECURITY_FLAGS: RendererSecurityFlags = {
  contextIsolation: true,
  sandbox: true,
  nodeIntegration: false,
  webSecurity: true,
  allowRunningInsecureContent: false,
} as const;

/** Default workspace window dimensions. */
export const WINDOW_DEFAULT_WIDTH = 1280;
export const WINDOW_DEFAULT_HEIGHT = 800;

/** Minimum workspace window dimensions. */
export const WINDOW_MIN_WIDTH = 960;
export const WINDOW_MIN_HEIGHT = 600;

/** Exactly one workspace window may exist per shell instance. */
export const SINGLE_WINDOW_LIMIT = 1;

/** Preload entry point, relative to the built output directory. */
export const PRELOAD_ENTRY = "preload.js";

/**
 * Build the renderer Content-Security-Policy. Local-only: no remote sources,
 * no unsafe inline scripts, no unsafe eval, no plugins.
 */
export function contentSecurityPolicy(): string {
  return [
    "default-src 'self'",
    "script-src 'self'",
    "style-src 'self'",
    "img-src 'self' data:",
    "font-src 'self' data:",
    "connect-src 'self'",
    "object-src 'none'",
    "base-uri 'self'",
    "form-action 'none'",
  ].join("; ");
}

/**
 * Validate a candidate window configuration against the frozen policy.
 * Returns true only when every security flag matches exactly.
 */
export function satisfiesWindowPolicy(candidate: {
  readonly contextIsolation: boolean;
  readonly sandbox: boolean;
  readonly nodeIntegration: boolean;
  readonly webSecurity: boolean;
  readonly allowRunningInsecureContent: boolean;
}): boolean {
  return (
    candidate.contextIsolation === true &&
    candidate.sandbox === true &&
    candidate.nodeIntegration === false &&
    candidate.webSecurity === true &&
    candidate.allowRunningInsecureContent === false
  );
}
