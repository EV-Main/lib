#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import { parseArgs } from "node:util";
import { FIGMA_SUITE, frames } from "./frames";

const USAGE = "usage: evinvest-figma-render [frames] -c <playwright config> [--file <figma fileKey>]";

const { values, positionals } = parseArgs({
  allowPositionals: true,
  options: { config: { type: "string", short: "c" }, file: { type: "string" } },
});
const [command, ...rest] = positionals;
const config = values.config;
if (config === undefined || rest.length > 0 || (command !== undefined && command !== "frames")) {
  console.error(USAGE);
  process.exit(1);
}

// `playwright` is the caller's PATH's: the pinned runner its browsers match.
const playwright = (args: string[], stdout: "pipe" | "inherit") => {
  const run = spawnSync("playwright", ["test", "-c", config, "--grep", FIGMA_SUITE, ...args], {
    stdio: ["inherit", stdout, "inherit"],
    encoding: "utf8",
    maxBuffer: 1 << 30,
  });
  if (run.error) throw run.error;
  return run;
};

if (command === "frames") {
  const run = playwright(["--list", "--reporter=json"], "pipe");
  if (run.status !== 0) process.exit(run.status ?? 1);
  console.log(JSON.stringify({ file: values.file ?? null, frames: frames(JSON.parse(run.stdout)) }, null, 2));
} else {
  if (process.env["FIGMA_CAPTURES"] === undefined) {
    console.error("FIGMA_CAPTURES: a JSON object of captureId and endpoint per frame, keyed as `frames` lists them");
    process.exit(1);
  }
  process.exit(playwright([], "inherit").status ?? 1);
}
