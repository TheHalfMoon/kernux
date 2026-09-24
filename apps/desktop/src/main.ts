/**
 * Main-process shell wiring (SG-000031 PR-B).
 *
 * Single workspace window, frozen unprivileged web preferences, strict CSP,
 * allowlisted invoke handlers backed by the local daemon transport. Source
 * reviewed here; executed by the packaging grain once Electron lands with
 * admission (no npm dependencies in this slice).
 */
import { app, BrowserWindow, ipcMain } from "electron";
import net from "node:net";
import path from "node:path";
import { ALLOWED_INVOKE_CHANNELS, isAllowedInvokeChannel } from "./bridge.js";
import {
  PRELOAD_ENTRY,
  RENDERER_SECURITY_FLAGS,
  SINGLE_WINDOW_LIMIT,
  WINDOW_DEFAULT_HEIGHT,
  WINDOW_DEFAULT_WIDTH,
  WINDOW_MIN_HEIGHT,
  WINDOW_MIN_WIDTH,
  contentSecurityPolicy,
  satisfiesWindowPolicy,
} from "./window-config.js";
import { probeDaemonHealth } from "./transport-client.js";

let mainWindow: BrowserWindow | null = null;

function windowCount(): number {
  return mainWindow !== null && !mainWindow.isDestroyed() ? 1 : 0;
}

function daemonEndpoint(): string {
  if (process.platform === "win32") {
    return process.env["KERNUXD_PIPE_NAME"] ?? "kernuxd";
  }
  const dir = process.env["KERNUXD_RUNTIME_DIR"] ?? "/tmp";
  return path.join(dir, "kernuxd.sock");
}

function launchNonce(): string {
  const nonce = process.env["KERNUXD_LAUNCH_NONCE"] ?? "";
  if (!/^[0-9a-f]{64}$/.test(nonce)) {
    throw new Error("missing-launch-nonce");
  }
  return nonce;
}

function createMainWindow(): BrowserWindow {
  if (windowCount() >= SINGLE_WINDOW_LIMIT) {
    const existing = mainWindow as BrowserWindow;
    if (existing.isMinimized()) {
      existing.restore();
    }
    existing.focus();
    return existing;
  }
  if (!satisfiesWindowPolicy({ ...RENDERER_SECURITY_FLAGS })) {
    throw new Error("window-policy-violation");
  }
  const created = new BrowserWindow({
    width: WINDOW_DEFAULT_WIDTH,
    height: WINDOW_DEFAULT_HEIGHT,
    minWidth: WINDOW_MIN_WIDTH,
    minHeight: WINDOW_MIN_HEIGHT,
    webPreferences: {
      preload: path.join(__dirname, PRELOAD_ENTRY),
      contextIsolation: RENDERER_SECURITY_FLAGS.contextIsolation,
      sandbox: RENDERER_SECURITY_FLAGS.sandbox,
      nodeIntegration: RENDERER_SECURITY_FLAGS.nodeIntegration,
    },
  });
  const policy = contentSecurityPolicy();
  const filter = { urls: ["*://*/*"] };
  created.webContents.session.webRequest.onHeadersReceived(filter, (details, callback) => {
    const headers = { ...details.responseHeaders };
    headers["Content-Security-Policy"] = [policy];
    callback({ responseHeaders: headers });
  });
  created.on("closed", () => {
    mainWindow = null;
  });
  mainWindow = created;
  return created;
}

function registerBridgeHandlers(): void {
  for (const channel of ALLOWED_INVOKE_CHANNELS) {
    ipcMain.handle(channel, async (_event, envelope: unknown) => {
      if (!isAllowedInvokeChannel(channel)) {
        return { ok: false as const, error: "unknown-channel" };
      }
      const endpoint = daemonEndpoint();
      const socket = net.createConnection(endpoint);
      const requestId =
        typeof envelope === "object" && envelope !== null && "requestId" in envelope
          ? String((envelope as { requestId: unknown }).requestId ?? "")
          : "";
      try {
        const status = await probeDaemonHealth(
          {
            write: (data: string) => {
              socket.write(data, "utf8");
            },
            onData: (listener: (chunk: string) => void) => {
              socket.on("data", (chunk: unknown) =>
                listener(typeof chunk === "string" ? chunk : String(chunk)),
              );
            },
            onError: (listener: (message: string) => void) => {
              socket.on("error", () => listener("transport-error"));
            },
            onClose: (listener: () => void) => {
              socket.on("close", () => listener());
            },
            destroy: () => socket.destroy(),
          },
          launchNonce(),
          requestId,
          2000,
        );
        return { ok: true as const, data: status };
      } catch {
        return { ok: false as const, error: "daemon-unreachable" };
      }
    });
  }
}

export async function startShell(): Promise<BrowserWindow> {
  const singleInstance = app.requestSingleInstanceLock();
  if (!singleInstance) {
    app.quit();
    throw new Error("single-instance");
  }
  registerBridgeHandlers();
  await app.whenReady();
  return createMainWindow();
}

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (windowCount() === 0) {
    createMainWindow();
  }
});
