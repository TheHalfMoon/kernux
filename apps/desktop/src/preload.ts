/**
 * Preload entry: expose ONLY the narrow typed bridge (SG-000031 PR-A).
 *
 * Runs with full Node privileges inside the isolated preload context, so it
 * must never widen the surface: allowlisted invoke-only channels, constant
 * errors, JSON-serializable payloads. The renderer receives `window.kernux`
 * and nothing else. Reviewed but not executed by CI; executed by the
 * packaging-grain boot smoke with TEMP-only tooling.
 */
import { contextBridge, ipcRenderer } from "electron";
import {
  buildInvokeEnvelope,
  isWellFormedResult,
  type InvokeEnvelope,
} from "./bridge.js";

/** Minimal renderer-facing API: exactly one method. */
export interface KernuxBridgeApi {
  readonly invoke: (channel: string, requestId: string, payload: unknown) => Promise<unknown>;
}

const api: KernuxBridgeApi = {
  invoke: (channel: string, requestId: string, payload: unknown): Promise<unknown> => {
    let envelope: InvokeEnvelope;
    try {
      envelope = buildInvokeEnvelope(channel, requestId, payload);
    } catch {
      return Promise.reject(new Error("unknown-channel"));
    }
    return ipcRenderer.invoke(envelope.channel, envelope).then((value: unknown) => {
      if (!isWellFormedResult(value)) {
        throw new Error("malformed-response");
      }
      if (value.ok) {
        return value.data;
      }
      throw new Error(value.error);
    });
  },
};

contextBridge.exposeInMainWorld("kernux", api);

declare global {
  interface Window {
    readonly kernux: KernuxBridgeApi;
  }
}
