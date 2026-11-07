import { spawnSync } from "node:child_process";
import { parseArgs } from "node:util";
import { FIGMA_SUITE, frameKey, frames } from "./frames";
import { render } from "./render";

const USAGE = `usage: evinvest-figma-render frames -c <playwright config> [--file <figma fileKey>]
       evinvest-figma-render -c <playwright config> --file <figma fileKey>`;

const { values, positionals } = parseArgs({
  allowPositionals: true,
  options: { config: { type: "string", short: "c" }, file: { type: "string" } },
});
const [command, ...rest] = positionals;
const { config, file } = values;
if (config === undefined || rest.length > 0 || (command === undefined ? file === undefined : command !== "frames")) {
  console.error(USAGE);
  process.exit(1);
}

// `playwright` is the caller's PATH's: the pinned runner its browsers match.
const playwright = (args: string[], stdout: "pipe" | 2, env: NodeJS.ProcessEnv = process.env) => {
  const run = spawnSync("playwright", ["test", "-c", config, "--grep", FIGMA_SUITE, ...args], {
    stdio: ["inherit", stdout, "inherit"],
    encoding: "utf8",
    maxBuffer: 1 << 30,
    env,
  });
  if (run.error) throw run.error;
  return run;
};

const list = playwright(["--list", "--reporter=json"], "pipe");
if (list.status !== 0) process.exit(list.status ?? 1);
const listed = frames(JSON.parse(list.stdout));

if (command === "frames") {
  console.log(JSON.stringify({ file: file ?? null, frames: listed.map(frameKey) }, null, 2));
} else {
  // stdout carries only the page link; the run's report goes to stderr
  const run = (captures: string) => playwright([], 2, { ...process.env, FIGMA_CAPTURES: captures }).status ?? 1;
  process.exit(await render(file as string, listed, run));
}
