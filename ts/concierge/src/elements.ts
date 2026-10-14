// Where the concierge microfrontends live and what they are called. Tags follow
// the Rust `mfe!` scheme, `mfe-{service}-{name}`, with `{service}.{name}` as the
// registry name; a host reads the bundle URL from the published registry.

import { entryFor, fetchRegistry, type MfeEntry, type ParseResult } from '@evinvest/mfe';

/** The origin concierge serves its pages, API and bundles from. */
export const CONCIERGE_ORIGIN = 'https://evinvest.ltd';

/** The published registry of concierge bundles; relative `scriptUrl`s resolve against it. */
export const CONCIERGE_REGISTRY_URL = `${CONCIERGE_ORIGIN}/api/auth/mfe/registry.json`;

export const CONCIERGE_ELEMENTS = {
  signIn: { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in' },
  account: { name: 'concierge.account', tag: 'mfe-concierge-account' },
  accountChip: { name: 'concierge.account-chip', tag: 'mfe-concierge-account-chip' },
} as const;

export type ConciergeElement = keyof typeof CONCIERGE_ELEMENTS;

export const SIGN_IN_TAG = CONCIERGE_ELEMENTS.signIn.tag;
export const ACCOUNT_TAG = CONCIERGE_ELEMENTS.account.tag;
export const ACCOUNT_CHIP_TAG = CONCIERGE_ELEMENTS.accountChip.tag;

/**
 * Fetch the concierge registry (default {@link CONCIERGE_REGISTRY_URL}). Failures
 * are results, as in `fetchRegistry`: the host renders its fallback either way.
 */
export function fetchConciergeRegistry(
  url: string | URL = CONCIERGE_REGISTRY_URL,
  init?: RequestInit,
): Promise<ParseResult<MfeEntry[]>> {
  return fetchRegistry(url, init);
}

/** The registry entry for `element`, refused when missing or naming another tag (`entryFor`). */
export function conciergeEntry(entries: readonly MfeEntry[], element: ConciergeElement): ParseResult<MfeEntry> {
  return entryFor(entries, CONCIERGE_ELEMENTS[element]);
}
