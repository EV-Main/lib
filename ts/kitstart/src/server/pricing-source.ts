import "server-only";
import type { PricingModel } from "../core/pricing/model";
import { parsePricingModel, pricingProblemsFor } from "../core/pricing/validate";
import type { Site } from "../core/site";
import { sitesPanelBrand } from "./env";
import { isEmptyObject, unreachable, type Down } from "./fetched";
import { PLACE_REVALIDATE_SECONDS } from "./place-source";

/**
 * The brand's live price list: `GET <sites_panel brand>/pricing`, the whole
 * model with a label in every locale, over the baked one (`site.pricing`). The
 * same failure semantics as the place source, for the same reason — a page
 * that throws under ISR is a bare 500 with no phone: an unreachable source, a
 * non-200, a body that is not JSON, and a model that does not validate all
 * serve the baked model. Taken whole or not at all: a page never prices from
 * half a model. `{}` is the source's "nothing set": the baked model, quietly.
 */
export interface PricingSourceOptions {
  /** TTL on the fetch itself (see `PlaceSourceOptions`); 600 s. */
  revalidateSeconds?: number;
  timeoutMs?: number;
  log?: Pick<Console, "error">;
}

export interface PricingSource {
  /** The model to price with now: live, else baked, else `null` (every need is a `quote`). Never throws. */
  model(): Promise<PricingModel | null>;
}

const TIMEOUT_MS = 3_000;

type NextInit = RequestInit & { next?: { revalidate: number } };

type FetchedPricing = { kind: "live"; model: PricingModel } | { kind: "empty" } | Down;

/** The price list as the source answers it now; shared with the site report, which asks `no-store`. */
export async function fetchPricing<L extends string, P extends string>(site: Site<L, P>, brand: string, init: NextInit): Promise<FetchedPricing> {
  let response: Response;
  try {
    response = await fetch(`${brand}/pricing`, init);
  } catch (cause) {
    return unreachable(cause);
  }
  if (!response.ok) return { kind: "down", detail: `answered ${response.status}` };
  let body: unknown;
  try {
    body = await response.json();
  } catch (cause) {
    return { kind: "down", detail: "not JSON", cause };
  }
  if (isEmptyObject(body)) return { kind: "empty" };
  const problems = pricingProblemsFor(body, site.i18n.locales);
  if (problems.length > 0) return { kind: "down", detail: `model refused whole (${problems.slice(0, 5).join("; ")})` };
  return { kind: "live", model: parsePricingModel(body) };
}

export function createPricingSource<L extends string, P extends string>(site: Site<L, P>, options: PricingSourceOptions = {}): PricingSource {
  const revalidate = options.revalidateSeconds ?? PLACE_REVALIDATE_SECONDS;
  const timeoutMs = options.timeoutMs ?? TIMEOUT_MS;
  const log = options.log ?? console;
  const baked = site.pricing ?? null;
  let brand: string | null | undefined;

  return {
    async model() {
      if (brand === undefined) brand = sitesPanelBrand(site, process.env);
      if (brand === null) return baked;
      const fetched = await fetchPricing(site, brand, { cache: "force-cache", next: { revalidate }, signal: AbortSignal.timeout(timeoutMs) });
      switch (fetched.kind) {
        case "live":
          return fetched.model;
        case "empty":
          return baked;
        case "down":
          log.error(`pricing: source ${fetched.detail}, pricing from the baked model`, ...(fetched.cause === undefined ? [] : [fetched.cause]));
          return baked;
      }
    },
  };
}
