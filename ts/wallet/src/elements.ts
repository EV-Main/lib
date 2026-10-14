// Where the banking wallet microfrontends live and what they are called. Tags
// follow the Rust `mfe!` scheme, `mfe-{service}-{name}`.

import { fetchRegistry, findEntry, type MfeEntry, type ParseResult } from '@evinvest/mfe';

/** The one origin that may carry out a money action and report its result. */
export const WALLET_ORIGIN = 'https://evinvest.ltd';

/** The published registry of wallet bundles; relative `scriptUrl`s resolve against it. */
export const WALLET_REGISTRY_URL = `${WALLET_ORIGIN}/wallet/mfe/registry.json`;

export const WALLET_ELEMENTS = {
  wallet: { name: 'banking.wallet', tag: 'mfe-banking-wallet' },
  payInvoice: { name: 'banking.pay-invoice', tag: 'mfe-banking-pay-invoice' },
} as const;

export type WalletElement = keyof typeof WALLET_ELEMENTS;

export const WALLET_TAG = WALLET_ELEMENTS.wallet.tag;
export const PAY_INVOICE_TAG = WALLET_ELEMENTS.payInvoice.tag;

/** Fetch the wallet registry (default {@link WALLET_REGISTRY_URL}); failures are results. */
export function fetchWalletRegistry(url: string | URL = WALLET_REGISTRY_URL, init?: RequestInit): Promise<ParseResult<MfeEntry[]>> {
  return fetchRegistry(url, init);
}

/**
 * The registry entry for `element`, refused when missing or naming another tag:
 * the wrappers mount the tag they know.
 */
export function walletEntry(entries: readonly MfeEntry[], element: WalletElement): ParseResult<MfeEntry> {
  const { name, tag } = WALLET_ELEMENTS[element];
  const entry = findEntry(entries, name);
  if (!entry) return { ok: false, error: `registry has no ${name}` };
  if (entry.tag !== tag) return { ok: false, error: `${name}: tag ${entry.tag}, expected ${tag}` };
  return { ok: true, value: entry };
}
