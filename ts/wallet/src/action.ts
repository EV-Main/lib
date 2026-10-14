// Money actions on a scope host. The wallet there shows balances and history
// only; a withdrawal, transfer, payment or mandate happens on evinvest.ltd, in a
// popup (or, when the popup is blocked, a full navigation), and the result comes
// back to the opener by `postMessage`. Banking's mutating routes accept only
// `Origin: https://evinvest.ltd`, so nothing here can move money by itself.
//
// Importing is server-safe; `openWalletAction` touches `window` when called.

import { WALLET_ORIGIN } from './elements';

/**
 * Cancelable, from the wallet elements: the user picked a money action. `detail`
 * is `{ href }`, a URL under `https://evinvest.ltd/wallet/`. A host that takes
 * it calls `preventDefault()` and opens it; left alone, the element navigates.
 */
export const WALLET_ACTION_EVENT = 'ev:wallet-action';

/** `type` of the message the evinvest.ltd page posts to its opener. */
export const WALLET_RESULT_MESSAGE = 'ev:wallet-result';

/** Path every money action lives under. */
export const WALLET_ACTION_PATH = '/wallet/';

/** Query parameter telling the action page which origin to post its result to. */
export const OPENER_PARAM = 'opener';
/** Query parameter telling the action page where to send the user back to when navigated, not popped up. */
export const RETURN_TO_PARAM = 'return_to';

const MAX_URL = 2048;
const MAX_REFERENCE = 128;
const DEFAULT_TIMEOUT_MS = 15 * 60_000;
const DEFAULT_POLL_MS = 500;
const POPUP_FEATURES = 'popup=yes,width=480,height=720';
/** One named window per page: a second action reuses it rather than stacking popups. */
const POPUP_NAME = 'ev-wallet-action';

/**
 * `raw` as a wallet action URL on `origin`, or null. Refused: another origin,
 * a path outside `/wallet/`, credentials, control characters and backslashes —
 * browsers strip tabs and newlines and read `\` as `/` before the check would
 * see them.
 */
export function walletActionUrl(raw: unknown, origin: string = WALLET_ORIGIN): URL | null {
  if (typeof raw !== 'string' || raw.length === 0 || raw.length > MAX_URL) return null;
  if (/[\u0000-\u001F\u007F\\]/.test(raw)) return null;
  let url: URL;
  try {
    url = new URL(raw);
  } catch {
    return null;
  }
  if (url.origin !== origin || url.username !== '' || url.password !== '') return null;
  return url.pathname.startsWith(WALLET_ACTION_PATH) ? url : null;
}

/** The action URL of a {@link WALLET_ACTION_EVENT}, or null when the detail is missing or not one. */
export function walletActionFromEvent(event: Event, origin: string = WALLET_ORIGIN): URL | null {
  const detail: unknown = event instanceof CustomEvent ? event.detail : undefined;
  if (typeof detail !== 'object' || detail === null) return null;
  return walletActionUrl(Reflect.get(detail, 'href'), origin);
}

/** What the action page reports. `reference` names what was created (payout, payment), when anything was. */
export interface WalletResult {
  outcome: 'done' | 'failed' | 'cancelled';
  reference: string | null;
}

const OUTCOMES: readonly WalletResult['outcome'][] = ['done', 'failed', 'cancelled'];

/**
 * A {@link WALLET_RESULT_MESSAGE} payload, or null for anything else on the
 * channel. The outcome is what matters — money may already have moved — so a
 * reference that cannot be used (empty, not a string, over 128 characters)
 * reads as none rather than losing the outcome. A long one is never cut: a
 * truncated payment reference names a different payment, or none.
 */
export function parseWalletMessage(data: unknown): WalletResult | null {
  if (typeof data !== 'object' || data === null) return null;
  if (Reflect.get(data, 'type') !== WALLET_RESULT_MESSAGE) return null;
  const outcome: unknown = Reflect.get(data, 'outcome');
  const reference: unknown = Reflect.get(data, 'reference');
  const known = OUTCOMES.find(value => value === outcome);
  if (known === undefined) return null;
  const usable = typeof reference === 'string' && reference !== '' && reference.length <= MAX_REFERENCE;
  return { outcome: known, reference: usable ? reference : null };
}

/** How a popup action ended. Every way is a value: the host decides what each means. */
export type WalletActionOutcome =
  | { status: 'result'; result: WalletResult }
  /** The user closed the popup before the page reported anything. */
  | { status: 'closed' }
  /** `window.open` was refused (popup blocker); navigate with {@link walletNavigationHref} instead. */
  | { status: 'blocked' }
  /** Another action's popup is still open on this page; it was focused instead. Report nothing for this click. */
  | { status: 'busy' }
  /** No result within `timeoutMs`; the popup was closed. */
  | { status: 'timeout' }
  /** `signal` aborted, before the popup opened or while it was open; the popup was closed. */
  | { status: 'aborted' }
  /** `url` is not a wallet action on `origin` ({@link walletActionUrl}); nothing was opened. */
  | { status: 'invalid_url' };

export interface WalletActionOptions {
  /** The origin the result must come from. Default {@link WALLET_ORIGIN}; other values are for a staging stand. */
  origin?: string;
  /** Give up — and close the popup — after this long. Default 15 minutes. */
  timeoutMs?: number;
  /** How often to look whether the user closed the popup. Default 500 ms. */
  pollMs?: number;
  /** Aborting closes the popup and resolves `aborted`. */
  signal?: AbortSignal;
}

/** `url` with this page's origin as {@link OPENER_PARAM}, so the action page knows where to post. */
function withOpener(url: URL): string {
  const next = new URL(url);
  next.searchParams.set(OPENER_PARAM, window.location.origin);
  return next.href;
}

/**
 * The same action as a full navigation, for when the popup is blocked: the page
 * sends the user back to `returnTo` (default: here) instead of posting.
 */
export function walletNavigationHref(url: URL, returnTo: string = window.location.href): string {
  const next = new URL(url);
  next.searchParams.set(RETURN_TO_PARAM, returnTo);
  return next.href;
}

// The popup of the action in flight on this page. Page-wide, like the window
// name: a second click anywhere — another button, the same one again — must
// not reload the first action's page under it, nor report a second result.
let inFlight: Window | null = null;

/**
 * Open a money action on evinvest.ltd in a popup and wait for its result. Only
 * a message from the popup itself, from `origin`, in the
 * {@link WALLET_RESULT_MESSAGE} shape counts; everything else on the channel is
 * ignored. While one action is in flight, another call focuses its popup and
 * resolves `busy`. A popup this call opened is closed whatever the outcome.
 * Call it inside the user's click — a popup opened later is blocked. Never
 * rejects. Browser only.
 */
export function openWalletAction(url: string | URL, options: WalletActionOptions = {}): Promise<WalletActionOutcome> {
  const origin = options.origin ?? WALLET_ORIGIN;
  const target = walletActionUrl(String(url), origin);
  if (!target) return Promise.resolve({ status: 'invalid_url' });
  if (options.signal?.aborted) return Promise.resolve({ status: 'aborted' });
  if (inFlight && !inFlight.closed) {
    inFlight.focus();
    return Promise.resolve({ status: 'busy' });
  }
  // Not `noopener`: the page needs `window.opener` to post back. It is ours.
  const popup = window.open(withOpener(target), POPUP_NAME, POPUP_FEATURES);
  if (!popup) return Promise.resolve({ status: 'blocked' });
  inFlight = popup;

  return new Promise<WalletActionOutcome>(resolve => {
    let settled = false;
    const settle = (outcome: WalletActionOutcome) => {
      if (settled) return;
      settled = true;
      if (inFlight === popup) inFlight = null;
      window.removeEventListener('message', onMessage);
      clearInterval(poll);
      clearTimeout(timer);
      options.signal?.removeEventListener('abort', onAbort);
      if (!popup.closed) popup.close();
      resolve(outcome);
    };
    const onMessage = (event: MessageEvent) => {
      // The source check stops another evinvest.ltd window from answering for this popup.
      if (event.origin !== origin || event.source !== popup) return;
      const result = parseWalletMessage(event.data);
      if (result) settle({ status: 'result', result });
    };
    const onAbort = () => settle({ status: 'aborted' });
    const poll = setInterval(() => {
      if (!popup.closed) return;
      clearInterval(poll);
      // A page that posts and then closes itself can be seen closed before its
      // message is dispatched here; one more macrotask lets the result win.
      setTimeout(() => settle({ status: 'closed' }), 0);
    }, options.pollMs ?? DEFAULT_POLL_MS);
    const timer = setTimeout(() => settle({ status: 'timeout' }), options.timeoutMs ?? DEFAULT_TIMEOUT_MS);
    window.addEventListener('message', onMessage);
    options.signal?.addEventListener('abort', onAbort, { once: true });
  });
}
