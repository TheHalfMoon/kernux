/**
 * Minimal test-only ambient declarations for the Node test runner
 * (SG-000031 PR-A).
 *
 * The shell carries no npm dependencies yet, so `@types/node` is unavailable.
 * This shim declares ONLY the runner APIs used by the tests below. It must
 * stay in sync with that surface; the packaging grain replaces it with real
 * type packages when Electron and React dependencies land with admission.
 */
declare module "node:test" {
  export function describe(name: string, fn: () => void): void;
  export function it(name: string, fn: () => void): void;
}

declare module "node:assert/strict" {
  function assert(value: unknown, message?: string): asserts value;
  namespace assert {
    function deepEqual(actual: unknown, expected: unknown, message?: string): void;
    function equal(actual: unknown, expected: unknown, message?: string): void;
    function ok(value: unknown, message?: string): asserts value;
    function throws(fn: () => void, validator?: (error: unknown) => boolean): void;
    function rejects(block: Promise<unknown>, validator?: (error: unknown) => boolean): Promise<void>;
  }
  export default assert;
}
