import { MOTION, SIZE, COLOR } from "./tokens.js";

/** Canonical renderer-safe CSS contract. Host code owns insertion; this module has no DOM authority. */
export const KERNUX_STYLE_SHEET = `:root {
  --kernux-focus: ${COLOR.focusLight};
  --kernux-focus-dark: ${COLOR.focusDark};
  --kernux-focus-width: ${SIZE.focusWidthRem};
  --kernux-reduced-motion: ${MOTION.reducedMs}ms;
}
:where(button, a, input, select, textarea):focus-visible {
  outline: var(--kernux-focus-width) solid var(--kernux-focus);
  outline-offset: 0.125rem;
}
@media (prefers-color-scheme: dark) {
  :where(button, a, input, select, textarea):focus-visible { outline-color: var(--kernux-focus-dark); }
}
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: var(--kernux-reduced-motion) !important;
    scroll-behavior: auto !important;
    transition-duration: var(--kernux-reduced-motion) !important;
  }
}
.kernux-visually-hidden {
  border: 0;
  clip: rect(0 0 0 0);
  clip-path: inset(50%);
  height: 1px;
  margin: -1px;
  overflow: hidden;
  padding: 0;
  position: absolute;
  white-space: nowrap;
  width: 1px;
}
`;
