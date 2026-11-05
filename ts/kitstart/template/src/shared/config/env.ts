import "server-only";
import { createPlaceSource, createPricingSource, createServerEnv, leadNotifier, type LeadNotifier } from "@evinvest/kitstart/server";
import { site } from "./site";

/** Parsed once, lazily: `next build` imports this and must need no secrets. */
export const serverEnv = createServerEnv(site);

/** The live place data from sites_panel (`SITES_PANEL_INTERNAL_URL`); baked without it. */
export const places = createPlaceSource(site);

/** The live price list from sites_panel; `site.pricing` without it. */
export const pricing = createPricingSource(site);

let built: LeadNotifier | undefined;

/** Built at boot (`instrumentation.ts`), so SMTP with no sender fails startup. */
export function notifier(): LeadNotifier {
  built ??= leadNotifier(site, serverEnv());
  return built;
}
