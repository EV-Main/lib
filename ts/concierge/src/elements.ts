// Where the concierge microfrontends live and what they are called. Tags follow
// the Rust `mfe!` scheme, `mfe-{service}-{name}`, with `{service}.{name}` as the
// registry name; a host reads the bundle URL from the published registry.

import { entryFor, fetchRegistry, type MfeEntry, type ParseResult } from '@evinvest/mfe';

/** The origin concierge serves its pages, API and bundles from. */
export const CONCIERGE_ORIGIN = 'https://evinvest.ltd';

/**
 * The directory every concierge bundle is served from. `fetchConciergeRegistry`
 * refuses a registry with a bundle outside it, and a host's CSP names the same
 * path in `script-src`.
 */
export const CONCIERGE_MFE_PREFIX = `${CONCIERGE_ORIGIN}/api/auth/mfe/`;

/** The published registry of concierge bundles; relative `scriptUrl`s resolve against it. */
export const CONCIERGE_REGISTRY_URL = `${CONCIERGE_MFE_PREFIX}registry.json`;

/** Registry name and tag of each concierge element; {@link conciergeEntry} refuses an entry with another tag. */
export const CONCIERGE_ELEMENTS = {
  signIn: { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in' },
  account: { name: 'concierge.account', tag: 'mfe-concierge-account' },
  accountChip: { name: 'concierge.account-chip', tag: 'mfe-concierge-account-chip' },
} as const;

export type ConciergeElement = keyof typeof CONCIERGE_ELEMENTS;

export const SIGN_IN_TAG = CONCIERGE_ELEMENTS.signIn.tag;
export const ACCOUNT_TAG = CONCIERGE_ELEMENTS.account.tag;
export const ACCOUNT_CHIP_TAG = CONCIERGE_ELEMENTS.accountChip.tag;

export interface ConciergeRegistryOptions {
  /** Default {@link CONCIERGE_MFE_PREFIX}; another value is for a staging stand only. */
  allowedPrefix?: string;
  init?: RequestInit;
}

/**
 * Fetch the concierge registry (default {@link CONCIERGE_REGISTRY_URL}), with
 * every bundle held to {@link CONCIERGE_MFE_PREFIX} and redirects refused (as
 * in `fetchRegistry`). Failures are results: the host renders its fallback
 * either way.
 */
export function fetchConciergeRegistry(
  url: string | URL = CONCIERGE_REGISTRY_URL,
  { allowedPrefix = CONCIERGE_MFE_PREFIX, init }: ConciergeRegistryOptions = {},
): Promise<ParseResult<MfeEntry[]>> {
  return fetchRegistry(url, init === undefined ? { allowedPrefix } : { allowedPrefix, init });
}

/** The registry entry for `element`, refused when missing or naming another tag (`entryFor`). */
export function conciergeEntry(entries: readonly MfeEntry[], element: ConciergeElement): ParseResult<MfeEntry> {
  return entryFor(entries, CONCIERGE_ELEMENTS[element]);
}
