import type { Page, TestType } from "@playwright/test";
import { FIGMA_SUITE, frameKey } from "./frames";

/**
 * `@evinvest/figma-render` — pages of a Playwright suite, serialized into Figma
 * frames through `generate_figma_design`'s capture script. Types only from
 * `@playwright/test`: the caller passes its own `test`, so this module loads
 * nothing at run time and cannot end up with a second copy of the runner
 * (Playwright refuses to run with two).
 */

/**
 * Waits until a selector's subtree is what a visitor sees: web fonts applied,
 * every image in it loaded (lazy ones forced), the network idle, and every
 * finite animation finished — `animations: "disabled"` freezes CSS animations,
 * not ones driven through the Web Animations API.
 */
export async function settle(page: Page, selector: string): Promise<void> {
  await page.evaluate(() => document.fonts.ready);
  await page.locator(selector).evaluate(root =>
    Promise.all(
      Array.from(root.querySelectorAll("img"), img => {
        img.loading = "eager";
        if (img.complete) return null;
        return new Promise(done => {
          img.addEventListener("load", done, { once: true });
          img.addEventListener("error", done, { once: true });
        });
      }),
    ),
  );
  await page.waitForLoadState("networkidle");
  await page.waitForFunction(
    () =>
      document
        .getAnimations()
        .filter(a => (a.effect?.getComputedTiming().iterations ?? 1) !== Infinity)
        .every(a => a.playState === "finished" || a.playState === "idle"),
    undefined,
    { timeout: 10_000 },
  );
}

/** What `generate_figma_design` hands out per frame, verbatim. */
interface Capture {
  captureId: string;
  endpoint: string;
}

const CAPTURE_JS = "https://mcp.figma.com/mcp/html-to-design/capture.js";

function requestedCaptures(raw: string | undefined): Record<string, Capture> {
  if (raw === undefined) return {};
  const parsed: unknown = JSON.parse(raw);
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed))
    throw new Error(`FIGMA_CAPTURES: want {"<project> <page>": {captureId, endpoint}}, got ${raw}`);
  for (const [key, c] of Object.entries(parsed))
    if (typeof c?.captureId !== "string" || typeof c?.endpoint !== "string")
      throw new Error(`FIGMA_CAPTURES["${key}"]: want {captureId, endpoint}, got ${JSON.stringify(c)}`);
  return parsed as Record<string, Capture>;
}

export interface FigmaSuiteOptions {
  /** Selectors `display: none`d before the capture: dev overlays, QA menus. */
  hide?: readonly string[];
}

/**
 * One `@figma <page>` test per page at every project, `<page>` being the URL's
 * path (with its origin when it names another host), so URLs differing only
 * by query or `#` share one frame. A capture id is single-use, so each runs
 * only for a `<project> <page>` key `FIGMA_CAPTURES` names.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any -- the runner's fixture types are the caller's; only `page` is read
export function defineFigmaSuite({ test }: { test: TestType<any, any> }, urls: readonly string[], { hide = [] }: FigmaSuiteOptions = {}): void {
  const captures = requestedCaptures(process.env["FIGMA_CAPTURES"]);
  const pages = new Map<string, string>();
  for (const url of urls) {
    const u = new URL(url, "relative:/");
    const page = u.protocol === "relative:" ? u.pathname : u.origin + u.pathname;
    if (!pages.has(page)) pages.set(page, url);
  }

  test.describe(FIGMA_SUITE, () => {
    test.use({ bypassCSP: true }); // capture.js and its POST to Figma
    for (const [name, url] of pages) {
      test(name, async ({ page }: { page: Page }, info: { project: { name: string } }) => {
        const capture = captures[frameKey({ project: info.project.name, page: name })];
        if (capture === undefined) return test.skip(true, "frame not requested");
        test.setTimeout(120_000); // a capture uploads up to 50 MB
        await page.emulateMedia({ reducedMotion: "reduce" });
        await page.goto(url);
        await settle(page, "body");
        if (hide.length > 0) await page.addStyleTag({ content: `${hide.join(", ")} { display: none !important; }` });
        await page.addScriptTag({ url: CAPTURE_JS });
        const submitted = page.waitForResponse(r => r.url() === capture.endpoint && r.request().method() === "POST");
        // Its promise never settles; the submit's response is the end.
        await page.evaluate(c => {
          void (window as unknown as { figma: { captureForDesign(o: Capture & { selector: string }): Promise<unknown> } }).figma.captureForDesign({ ...c, selector: "body" });
        }, capture);
        const response = await submitted;
        if (!response.ok()) throw new Error(`Figma refused the capture: ${response.status()} ${await response.text()}`);
      });
    }
  });
}
