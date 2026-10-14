// Signing out of a scope host ends two sessions: the host's own (its RP
// session, which only the host can end) and the platform's on evinvest.ltd,
// which the host cannot touch — the cookie is first-party there. Left alive,
// the platform session signs the next visitor of a shared browser straight
// back in through the silent RP login. Concierge ends it on its own confirm
// page, reached by a top-level navigation rather than a cross-site request.

import { CONCIERGE_ORIGIN } from './elements';
import { hostReturnTo } from './sign-in';

/** Concierge's page that ends the evinvest.ltd session and sends the user back to `return_to`. */
export const PLATFORM_SIGN_OUT_URL = `${CONCIERGE_ORIGIN}/api/auth/logout/confirm`;

/**
 * Where to send the user to end the evinvest.ltd session:
 * `PLATFORM_SIGN_OUT_URL?return_to=<origin + pathname>` (`hostReturnTo`).
 * Concierge takes `return_to` only when its origin is in a scope's registry and
 * its path is byte for byte one of the scope's `return_paths`; anything else —
 * including no `return_to` at all — is a 400 and the session stays. A
 * `returnTo` that is not an absolute https URL therefore yields the bare URL,
 * which concierge refuses: pass this host's origin plus `/`.
 */
export function platformSignOutHref(returnTo: string): string {
  const back = hostReturnTo(returnTo);
  if (back === null) return PLATFORM_SIGN_OUT_URL;
  return `${PLATFORM_SIGN_OUT_URL}?return_to=${encodeURIComponent(back)}`;
}
