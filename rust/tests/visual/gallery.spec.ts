import { test } from "@playwright/test";
import { readFileSync } from "node:fs";

// Capture only — the comparison is a byte-exact one in `cargo test --test gallery`,
// not `toHaveScreenshot`. Renders are deterministic once the env is pinned
// (0 px on re-run, 0 px across CPU architectures), so a diff-ratio threshold
// buys nothing and hides the drift it is meant to absorb: the host font set
// moving under the baselines lands well inside the old 1 % allowance.
//
// The list comes from the Rust side (dist/manifest.json), so adding a primitive
// to the gallery (one line in tests/support/gallery.rs) adds a baseline here
// with no TypeScript change.
const OUT = process.env.EV_SNAPSHOT_OUT;
if (!OUT) throw new Error("EV_SNAPSHOT_OUT unset — this runs through `cargo test --test gallery`");

const NAMES: string[] = JSON.parse(readFileSync(new URL("./dist/manifest.json", import.meta.url), "utf8"));

// Each theme's root as ev.css selects it. Written into the served page rather
// than set after load, so no `transition-*` ever animates from one theme to the other.
const THEMES: Record<string, string> = {
  dark: '<html class="dark">',
  light: '<html class="light">',
  oldschool: '<html class="dark" data-theme="oldschool">',
};

for (const name of NAMES) for (const [theme, root] of Object.entries(THEMES)) {
  test(`${name}.${theme}`, async ({ page }) => {
    await page.route(`**/${name}.html`, async (route) => {
      const response = await route.fetch();
      const html = await response.text();
      if (!html.includes(THEMES.dark)) throw new Error(`${name}.html has no ${THEMES.dark} to theme`);
      await route.fulfill({ response, body: html.replace(THEMES.dark, root) });
    });
    await page.goto(`/${name}.html`);

    // Tailwind compiles in-browser after load; networkidle fires once its
    // script has loaded and stopped fetching, then the small buffer covers the
    // synchronous compile + paint. The viewport (not #stage) is the screenshot
    // target so `position: fixed` overlays — dialog, sheet, drawer — are
    // captured even though they contribute no height to the stage box.
    await page.waitForLoadState("networkidle");
    await page.evaluate(() => document.fonts.ready);
    await page.waitForTimeout(800);

    await page.screenshot({ path: `${OUT}/${name}.${theme}.png`, fullPage: true, animations: "disabled" });
  });
}
