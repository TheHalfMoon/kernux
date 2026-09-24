/**
 * Renderer skeleton (SG-000031 PR-C; SG-000032 projection wiring).
 *
 * Pure presentational shell over daemon status: project rail, command bar,
 * adaptive workspace, and run strip placeholders per the UX blueprint. No
 * ambient authority: this module touches no Electron, Node, or IPC APIs and
 * receives everything through props. Source reviewed here; executed by the
 * packaging grain once React lands with admission.
 *
 * SG-000032: the run strip consumes the single connection authority through
 * `authorityStatusText`. The renderer derives the line mechanically and
 * keeps no second boolean connection truth.
 */
import React from "react";
import type { DaemonStatus } from "./transport-client.js";
import { projectConnection } from "./connection-projection.js";
import type { ReconnectSnapshot } from "./reconnect-controller.js";

/** Status line derived from the single connection authority (SG-000032). */
export function authorityStatusText(snapshot: ReconnectSnapshot): string {
  return projectConnection(snapshot).statusLine;
}

/** Exact renderer-facing status line for one daemon status. */
export function statusText(status: DaemonStatus): string {
  switch (status.connection) {
    case "connected":
      return status.daemonVersion === null
        ? "Connected"
        : `Connected (daemon ${status.daemonVersion})`;
    case "degraded":
      return "Degraded: daemon reachable but not serving";
    case "unknown":
      return "Daemon state unknown";
  }
}

/** Command bar entry point shell. */
export function CommandBar(): React.ReactElement {
  return React.createElement("input", {
    placeholder: "What do you want done?",
    "aria-label": "command bar",
    disabled: true,
  });
}

/** Minimal four-region workspace shell driven by daemon status. */
export function DesktopShell(props: { readonly status: DaemonStatus }): React.ReactElement {
  const { status } = props;
  return React.createElement(
    "div",
    { id: "kernux-shell" },
    React.createElement("nav", { id: "project-rail" }, "Projects"),
    React.createElement(
      "main",
      { id: "workspace" },
      React.createElement(CommandBar, null),
      React.createElement("section", { id: "run-strip" }, statusText(status)),
    ),
  );
}
