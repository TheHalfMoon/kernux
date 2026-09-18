import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const wrapper = resolve(import.meta.dirname, "run.py");
const forwarded = process.argv.slice(2);

const candidates = process.env.PYTHON
  ? [[process.env.PYTHON]]
  : process.platform === "win32"
    ? [["py", "-3"], ["python"]]
    : [["python3"], ["python"]];

for (const [command, ...prefix] of candidates) {
  const probe = spawnSync(command, [...prefix, "--version"], {
    encoding: "utf8",
  });
  if (probe.error?.code === "ENOENT" || probe.status !== 0) {
    continue;
  }
  const result = spawnSync(command, [...prefix, wrapper, ...forwarded], {
    stdio: "inherit",
  });
  if (result.error) {
    console.error(result.error.message);
    process.exit(2);
  }
  process.exit(result.status ?? 2);
}

console.error("Kernux SpecGrain requires Python 3.11 or newer.");
process.exit(2);
