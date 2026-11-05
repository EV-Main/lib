import { readFileSync } from "node:fs";
import { createServer, type Server } from "node:http";
import type { AddressInfo } from "node:net";
import { join } from "node:path";
import { afterAll, afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { siteReportRoute } from "../src/next/index";
import { createExperimentsSource, createPlaceSource, createPricingSource, parseServerEnv } from "../src/server/index";
import { fixtureSite } from "./support/fixtures";

// sites_panel as a landing reaches it: a real HTTP server under `/api/internal`,
// whose answers each test sets per path.
const site = fixtureSite("aquafix");
const BRAND = "/api/internal/brands/aquafix";
const VERSION = (JSON.parse(readFileSync(join(import.meta.dirname, "../package.json"), "utf8")) as { version: string }).version;
const MODEL: unknown = JSON.parse(readFileSync(join(import.meta.dirname, "fixtures/pricing/valid/cleaning.json"), "utf8"));
const quiet = { log: { error: () => undefined } };

type Answer = { status: number; json?: unknown; text?: string };
let answers: Record<string, Answer> = {};
const seen: string[] = [];
let server: Server;
let internal = "";

beforeAll(async () => {
  server = createServer((req, res) => {
    const url = req.url ?? "/";
    seen.push(url);
    const answer = answers[url.split("?")[0] ?? ""] ?? { status: 404, text: "<html>no route</html>" };
    if (answer.json !== undefined) {
      res.writeHead(answer.status, { "content-type": "application/json" }).end(JSON.stringify(answer.json));
    } else {
      res.writeHead(answer.status, { "content-type": "text/html" }).end(answer.text ?? "");
    }
  });
  await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
  internal = `http://127.0.0.1:${(server.address() as AddressInfo).port}/api/internal`;
});
afterAll(() => new Promise<void>(resolve => server.close(() => resolve())));
afterEach(() => {
  vi.unstubAllEnvs();
  answers = {};
  seen.length = 0;
});

const connect = () => vi.stubEnv("SITES_PANEL_INTERNAL_URL", internal);

describe("the boot", () => {
  it("refuses LOCATIONS_API_URL, naming what replaced it", () => {
    expect(() => parseServerEnv(site, { LOCATIONS_API_URL: "https://panel.example/api/internal/brands/aquafix" })).toThrow(/SITES_PANEL_INTERNAL_URL/);
    expect(() => parseServerEnv(site, { SITES_PANEL_INTERNAL_URL: "not a url" })).toThrow(/SITES_PANEL_INTERNAL_URL is not a URL/);
  });
});

describe("the sources", () => {
  it("ask sites_panel under the brand's path", async () => {
    connect();
    await createPlaceSource(site, quiet).getPlace("royat", "fr");
    await createPricingSource(site, quiet).model();
    await createExperimentsSource(site, quiet).overrides();
    expect(seen).toEqual([`${BRAND}/locations/royat?locale=fr`, `${BRAND}/pricing`, `${BRAND}/experiments`]);
  });
});

describe("/.well-known/sa-site", () => {
  const report = async () => {
    const response = await siteReportRoute(site)();
    expect(response.status).toBe(200);
    expect(response.headers.get("cache-control")).toBe("no-store");
    return (await response.json()) as { brand: string; kitstart: string; checked_at: string; sources: Record<string, unknown> };
  };
  const slugs = site.places.map(p => p.slug);

  it("is unset everywhere, asking nothing, on a landing that is not connected", async () => {
    const body = await report();
    expect(body).toMatchObject({ brand: "aquafix", kitstart: VERSION });
    expect(body.sources).toEqual({
      places: Object.fromEntries(slugs.map(s => [s, { state: "unset" }])),
      pricing: { state: "unset" },
      experiments: { state: "unset" },
    });
    expect(seen).toEqual([]);
  });

  it("says what each source answers now", async () => {
    connect();
    const [live, withdrawn, gone, empty, failing] = slugs;
    answers = {
      [`${BRAND}/locations/${live}`]: { status: 200, json: { phone: "+33 4 11 11 11 11" } },
      [`${BRAND}/locations/${withdrawn}`]: { status: 404, json: { error: "withdrawn" } },
      [`${BRAND}/locations/${gone}`]: { status: 410 },
      [`${BRAND}/locations/${empty}`]: { status: 200, json: {} },
      [`${BRAND}/locations/${failing}`]: { status: 503, json: { error: "down" } },
      [`${BRAND}/pricing`]: { status: 200, json: {} },
      [`${BRAND}/experiments`]: { status: 200, json: { experiments: { hero: { enabled: false } } } },
    };
    const body = await report();
    expect(Date.parse(body.checked_at)).not.toBeNaN();
    expect(body.sources).toEqual({
      places: {
        [live ?? ""]: { state: "live" },
        [withdrawn ?? ""]: { state: "withdrawn" },
        [gone ?? ""]: { state: "withdrawn" },
        [empty ?? ""]: { state: "empty" },
        [failing ?? ""]: { state: "down", detail: "answered 503" },
        // Not the source's word: an ingress with no route.
        ...Object.fromEntries(slugs.slice(5).map(s => [s, { state: "down", detail: "answered 404 without JSON" }])),
      },
      pricing: { state: "empty" },
      experiments: { state: "live" },
    });
  });

  it("is live and empty for the price list and the experiments by their own shapes", async () => {
    connect();
    answers = {
      [`${BRAND}/pricing`]: { status: 200, json: MODEL },
      [`${BRAND}/experiments`]: { status: 200, json: { experiments: {} } },
    };
    const body = await report();
    expect(body.sources["pricing"]).toEqual({ state: "live" });
    expect(body.sources["experiments"]).toEqual({ state: "empty" });
  });

  it("is down, with why, for a body that is not the source's shape or a source that is gone", async () => {
    connect();
    answers = {
      [`${BRAND}/pricing`]: { status: 200, json: { ...(MODEL as object), roundToCents: 0 } },
      [`${BRAND}/experiments`]: { status: 200, text: "<html>" },
    };
    const body = await report();
    expect(body.sources["pricing"]).toEqual({ state: "down", detail: expect.stringMatching(/^model refused/) });
    expect(body.sources["experiments"]).toEqual({ state: "down", detail: "not JSON" });
    vi.stubEnv("SITES_PANEL_INTERNAL_URL", "http://127.0.0.1:1/api/internal");
    const dead = await report();
    expect(dead.sources["pricing"]).toEqual({ state: "down", detail: expect.stringMatching(/^unreachable/) });
  });

  it("asks sites_panel once however often it is read", async () => {
    connect();
    const route = siteReportRoute(site);
    await Promise.all([route(), route()]);
    await route();
    expect(seen).toHaveLength(slugs.length + 2);
  });
});
