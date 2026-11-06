import type { Expect, Page, TestType } from "@playwright/test";

/**
 * `@evinvest/kitstart/testing/e2e` — the section-baseline runner a brand's
 * Playwright suite calls. One baseline per section at both designed
 * breakpoints; every section reached by its own URL (a `#` the site itself
 * links to), never by scrolling to it — a scroll is a moving target the runner
 * would have to guess is over. The same pages, settled the same way, also go
 * to Figma on request (`defineFigmaSuite`).
 *
 * Types only from `@playwright/test`: the brand passes its own `test` and
 * `expect`, so this module loads nothing at run time and cannot end up with a
 * second copy of the runner (Playwright refuses to run with two).
 */

/**
 * Waits until a section is what a visitor sees: web fonts applied, every
 * image in it loaded (lazy ones forced), the network idle, and every finite
 * animation finished — `animations: "disabled"` freezes CSS animations, not
 * ones driven through the Web Animations API.
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

export interface Section {
  name: string;
  /** Relative to the config's `baseURL`, or absolute for another host. */
  url: string;
  selector: string;
  /** `md:hidden` in the design: shot only in the `mobile` project. */
  mobileOnly?: boolean;
  /** Shot without the section stylesheet (the call bar hides itself in it). */
  bare?: boolean;
}

/** The two designed breakpoints, as Playwright projects. */
export const BREAKPOINTS = [
  { name: "desktop", viewport: { width: 1440, height: 900 } },
  { name: "mobile", viewport: { width: 390, height: 844 } },
] as const;

// eslint-disable-next-line @typescript-eslint/no-explicit-any -- the runner's fixture types are the brand's; only `page` is read
type AnyTest = TestType<any, any>;

/** The brand's own runner: `import { test, expect } from "@playwright/test"`. */
export interface Runner {
  test: AnyTest;
  expect: Expect;
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

/**
 * One `@figma <page>` test per page of the sections, at every project: the
 * page as the baselines see it, serialized into a Figma frame. A capture id is
 * single-use, so each runs only for a `<project> <page>` key `FIGMA_CAPTURES` names.
 */
export function defineFigmaSuite({ test }: Pick<Runner, "test">, sections: readonly Section[]): void {
  const captures = requestedCaptures(process.env["FIGMA_CAPTURES"]);
  const pages = new Map<string, string>();
  for (const { url } of sections) {
    const u = new URL(url, "relative:/");
    const page = u.protocol === "relative:" ? u.pathname : u.origin + u.pathname;
    if (!pages.has(page)) pages.set(page, url);
  }

  test.describe("@figma", () => {
    test.use({ bypassCSP: true }); // capture.js and its POST to Figma
    for (const [name, url] of pages) {
      test(name, async ({ page }: { page: Page }, info: { project: { name: string } }) => {
        const capture = captures[`${info.project.name} ${name}`];
        if (capture === undefined) return test.skip(true, "frame not requested");
        test.setTimeout(120_000); // a capture uploads up to 50 MB
        await page.emulateMedia({ reducedMotion: "reduce" });
        await page.goto(url);
        await settle(page, "body");
        await page.addStyleTag({ content: "[data-ab-switcher] { display: none !important; }" });
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

/** One `test` per section, each a `toHaveScreenshot` of the settled section, and the `@figma` frames of their pages. */
export function defineSectionSuite({ test, expect }: Runner, sections: readonly Section[]): void {
  defineFigmaSuite({ test }, sections);

  // The site scrolls smoothly for everyone who has not asked it not to;
  // `use.reducedMotion` does not reach the page, `emulateMedia` does.
  test.beforeEach(async ({ page }: { page: Page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
  });

  for (const section of sections) {
    test(`section: ${section.name}`, async ({ page }: { page: Page }, info: { project: { name: string } }) => {
      test.skip(Boolean(section.mobileOnly) && info.project.name !== "mobile");
      await page.goto(section.url);
      const locator = page.locator(section.selector);
      await expect(locator).toBeVisible();
      await settle(page, section.selector);
      await expect(locator).toHaveScreenshot(`${section.name}-${info.project.name}.png`, section.bare ? { stylePath: [] } : {});
    });
  }
}
