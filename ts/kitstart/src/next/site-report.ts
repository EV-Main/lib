import manifest from "@evinvest/kitstart/package.json" with { type: "json" };
import type { Site } from "../core/site";
import { sitesPanelBrand } from "../server/env";
import { fetchExperiments } from "../server/experiments-source";
import type { Down } from "../server/fetched";
import { fetchPlace } from "../server/place-source";
import { fetchPricing } from "../server/pricing-source";

type SourceState = { state: "unset" | "live" | "empty" | "withdrawn" } | { state: "down"; detail: string };

interface SiteReport {
  brand: string;
  kitstart: string;
  checked_at: string;
  sources: { places: Record<string, SourceState>; pricing: SourceState; experiments: SourceState };
}

/** However often the route is read, sites_panel is asked once per window. */
const REPORT_TTL_MS = 30_000;
const TIMEOUT_MS = 3_000;
const UNSET: SourceState = { state: "unset" };

const down = (fetched: Down): SourceState => ({ state: "down", detail: fetched.detail });

/**
 * `app/.well-known/sa-site/route.ts`: what each live source answers this
 * landing now, for the conductor's Sources page. Asked fresh (`no-store`) of
 * the same endpoints the pages read, in the same terms they read them.
 */
export function siteReportRoute<L extends string, P extends string>(site: Site<L, P>): () => Promise<Response> {
  let cached: { at: number; report: Promise<SiteReport> } | null = null;

  async function probe(): Promise<SiteReport> {
    const checked_at = new Date().toISOString();
    const brand = sitesPanelBrand(site, process.env);
    const init = () => ({ cache: "no-store" as const, signal: AbortSignal.timeout(TIMEOUT_MS) });
    const [places, pricing, experiments] = await Promise.all([
      Promise.all(
        site.places.map(async (place): Promise<[string, SourceState]> => {
          if (brand === null) return [place.slug, UNSET];
          const fetched = await fetchPlace(site, brand, place, site.i18n.defaultLocale, init());
          if (fetched.kind === "down") return [place.slug, down(fetched)];
          return [place.slug, { state: fetched.kind === "withdrawn" ? "withdrawn" : fetched.empty ? "empty" : "live" }];
        }),
      ),
      brand === null ? UNSET : fetchPricing(site, brand, init()).then(f => (f.kind === "down" ? down(f) : { state: f.kind })),
      brand === null ? UNSET : fetchExperiments(brand, TIMEOUT_MS).then(f => (f.kind === "down" ? down(f) : { state: f.kind })),
    ]);
    return { brand: site.brand.id, kitstart: manifest.version, checked_at, sources: { places: Object.fromEntries(places), pricing, experiments } };
  }

  return async () => {
    if (cached === null || Date.now() - cached.at >= REPORT_TTL_MS) cached = { at: Date.now(), report: probe() };
    return Response.json(await cached.report, { headers: { "Cache-Control": "no-store" } });
  };
}
