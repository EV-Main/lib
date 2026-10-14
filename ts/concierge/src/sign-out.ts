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
 * `PLATFORM_SIGN_OUT_URL?return_to=<origin + pathname>`. Query and hash are
 * dropped — they can carry tokens or ids that have no business in a URL on
 * another origin. A `returnTo` that is not an absolute https URL
 * (`hostReturnTo`) leaves `return_to` out, and concierge decides where the
 * user lands. Concierge sends the user back only to an origin in a scope's
 * registry.
 */
export function platformSignOutHref(returnTo: string): string {
  const back = hostReturnTo(returnTo);
  if (back === null) return PLATFORM_SIGN_OUT_URL;
  const url = new URL(back);
  return `${PLATFORM_SIGN_OUT_URL}?return_to=${encodeURIComponent(url.origin + url.pathname)}`;
}
