import { execFileSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { setTimeout as sleep } from "node:timers/promises";
import { type Frame, frameKey } from "./frames";
import { connect } from "./mcp";

const GAP = 200;
const POLLS = 10;

/** The `captureForDesign({ captureId, endpoint })` an issuing `generate_figma_design` answers. */
export function issued(text: string): { captureId: string; endpoint: string } {
  const m = /captureForDesign\(\{ captureId: '([^']+)', endpoint: '([^']+)'/.exec(text);
  if (m?.[1] === undefined || m[2] === undefined) throw new Error(`generate_figma_design: no captureForDesign({ captureId, endpoint }) in\n${text}`);
  return { captureId: m[1], endpoint: m[2] };
}

/** A polling `generate_figma_design`'s frame node id, or null while the capture is pending. */
export function polled(text: string): string | null {
  const done = /^The design has been added to your existing file: https:\/\/www\.figma\.com\/design\/\w+\?node-id=(\d+)-(\d+)\s/.exec(text);
  if (done) return `${done[1]}:${done[2]}`;
  if (/^Status for capture ID `[^`]+`: \*\*(pending|processing)\*\*/.test(text)) return null;
  throw new Error(`generate_figma_design: an unknown poll answer\n${text}`);
}

interface Placed {
  id: string;
  name: string;
  row: number;
  col: number;
}

interface FigmaNode {
  id: string;
  name: string;
  type: string;
  x: number;
  y: number;
  width: number;
  height: number;
  parent: FigmaNode | null;
  children: FigmaNode[];
  appendChild(n: FigmaNode): void;
  remove(): void;
  loadAsync(): Promise<void>;
}
declare const figma: {
  root: FigmaNode;
  getNodeByIdAsync(id: string): Promise<FigmaNode | null>;
  createPage(): FigmaNode;
  setCurrentPageAsync(page: FigmaNode): Promise<void>;
};

/**
 * Runs inside Figma (`use_figma`, as its source text): the frames onto one page named `name`, in a grid.
 * Captures land wherever Figma puts them, so the page is the first frame's when it holds nothing else.
 */
async function arrange({ name, frames, gap }: { name: string; frames: Placed[]; gap: number }): Promise<{ pageId: string }> {
  const nodes: FigmaNode[] = [];
  for (const f of frames) {
    const n = await figma.getNodeByIdAsync(f.id);
    if (n?.parent?.type !== "PAGE") throw new Error(`frame ${f.id}: ${n === null ? "missing" : `under a ${n.parent?.type}`}`);
    nodes.push(n);
  }
  const ours = new Set(nodes);
  const homes = [...new Set(nodes.map(n => n.parent as FigmaNode))];
  for (const p of homes) await p.loadAsync();
  const onlyOurs = new Set(homes.filter(p => p.children.every(c => ours.has(c))));
  const first = homes[0] as FigmaNode;
  const page = onlyOurs.has(first) ? first : figma.createPage();
  page.name = name;
  await figma.setCurrentPageAsync(page);

  const widths: number[] = [];
  const heights: number[] = [];
  frames.forEach((f, i) => {
    const n = nodes[i] as FigmaNode;
    widths[f.col] = Math.max(widths[f.col] ?? 0, n.width);
    heights[f.row] = Math.max(heights[f.row] ?? 0, n.height);
  });
  const offset = (sizes: number[], i: number) => sizes.slice(0, i).reduce((sum, s) => sum + s + gap, 0);
  frames.forEach((f, i) => {
    const n = nodes[i] as FigmaNode;
    n.name = f.name;
    page.appendChild(n);
    n.x = offset(widths, f.col);
    n.y = offset(heights, f.row);
  });
  for (const p of onlyOurs) if (p !== page && p.children.length === 0 && figma.root.children.length > 1) p.remove();
  return { pageId: page.id };
}

/** `<short sha> <YYYY-MM-DD>`, ` dirty` when the working tree is: the render is of it. */
function pageName(): string {
  const git = (...args: string[]) => execFileSync("git", args, { encoding: "utf8" }).trim();
  const d = new Date();
  const date = [d.getFullYear(), d.getMonth() + 1, d.getDate()].map(n => String(n).padStart(2, "0")).join("-");
  return `${git("rev-parse", "--short", "HEAD")} ${date}${git("status", "--porcelain") === "" ? "" : " dirty"}`;
}

/**
 * Captures `frames` into Figma file `file` through `run` (the suite, given `FIGMA_CAPTURES`), lays them out on one
 * page and prints its link. Only `use_figma` counts toward Figma's rate limit. Answers the exit code.
 */
export async function render(file: string, frames: Frame[], run: (captures: string) => number): Promise<number> {
  if (frames.length === 0) throw new Error("no @figma frames to render");
  const name = pageName();
  const call = await connect();
  const promptId = randomUUID();
  const captures = new Map<string, { captureId: string; endpoint: string }>();
  for (const f of frames) captures.set(frameKey(f), issued(await call("generate_figma_design", { fileKey: file, promptId })));

  const status = run(JSON.stringify(Object.fromEntries(captures)));

  const nodes = new Map<string, string>();
  const pending = new Map<string, string>();
  for (let round = 0; round < POLLS && nodes.size < captures.size; round++) {
    if (round > 0) await sleep(5_000);
    for (const [key, { captureId }] of captures) {
      if (nodes.has(key)) continue;
      const text = await call("generate_figma_design", { fileKey: file, captureId });
      const node = polled(text);
      if (node === null) {
        pending.set(key, text.split("\n")[0] as string);
      } else {
        nodes.set(key, node);
        pending.delete(key);
      }
    }
  }

  if (nodes.size > 0) {
    const pages = [...new Set(frames.map(f => f.page))];
    const projects = [...new Set(frames.map(f => f.project))];
    const placed: Placed[] = frames.flatMap(f => {
      const id = nodes.get(frameKey(f));
      return id === undefined ? [] : [{ id, name: frameKey(f), row: pages.indexOf(f.page), col: projects.indexOf(f.project) }];
    });
    const text = await call("use_figma", {
      fileKey: file,
      description: `Name the ${placed.length} captured frames and lay them out on page "${name}"`,
      code: `return await (${arrange.toString()})(${JSON.stringify({ name, frames: placed, gap: GAP })});`,
    });
    const { pageId } = JSON.parse(text) as { pageId?: unknown };
    if (typeof pageId !== "string" || !/^\d+:\d+$/.test(pageId)) throw new Error(`use_figma: no page id in ${text}`);
    console.log(`https://www.figma.com/design/${file}?node-id=${pageId.replace(":", "-")}`);
  }
  for (const [key, text] of pending) console.error(`✘ ${key} not captured (its test failed): ${text}`);
  return pending.size > 0 ? 1 : status;
}
