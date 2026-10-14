// The host's half of signing in. The sign-in element owns the dialog — backdrop,
// focus trap, every step — and talks to the host through two events: it is done
// (`ev:signed-in`, from @evinvest/mfe) or the user dismissed it (`ev:close`).
// After `ev:signed-in` the evinvest.ltd session exists, and the host starts its
// own silent RP login, which `authorize` answers without asking again.

import { rootRelative, safeHref } from '@evinvest/mfe';

/**
 * The user dismissed the sign-in dialog (Escape, the backdrop, the close
 * button). Bubbles, `composed`, no detail — like the events of @evinvest/mfe.
 */
export const CLOSE_EVENT = 'ev:close';

/** `data-intent`: which screen the sign-in flow opens on. */
export type SignInIntent = 'signin' | 'signup';

/** Where a host serves its RP login, which redirects to concierge's `authorize`. */
export const RP_LOGIN_PATH = '/auth/login';

/**
 * The host's RP login URL that comes back to `returnTo`
 * (`/auth/login?return_to=%2Fx`). A `returnTo` that could leave the origin
 * becomes `/`; the host's login route must re-check it server-side anyway.
 */
export function rpLoginHref(returnTo: string, loginPath: string = RP_LOGIN_PATH): string {
  const back = rootRelative(returnTo) ?? '/';
  return `${loginPath}?return_to=${encodeURIComponent(back)}`;
}

/**
 * `data-return-to`: the origin and path of `raw` if it is an absolute https URL
 * — http only to loopback, for a dev stand (`safeHref`) — else null, a
 * root-relative path included. Absolute because the OAuth round trip leaves for
 * evinvest.ltd and must find its way back. Query, hash and credentials are
 * always dropped: concierge refuses a `returnTo` with `?` or `#` (the OAuth
 * start answers 400), and they can carry tokens or ids that have no business on
 * another origin. That the origin is the host's is not checked here: concierge
 * accepts it only if its origin is in a scope's registry.
 */
export function hostReturnTo(raw: string): string | null {
  const href = safeHref(raw);
  if (href === null || href.startsWith('/')) return null;
  const url = new URL(href);
  return url.origin + url.pathname;
}
