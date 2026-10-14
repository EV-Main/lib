// The host's half of signing in. The sign-in element owns the dialog — backdrop,
// focus trap, every step — and talks to the host through two events: it is done
// (`ev:signed-in`, from @evinvest/mfe) or the user dismissed it (`ev:close`).
// After `ev:signed-in` the evinvest.ltd session exists, and the host starts its
// own silent RP login, which `authorize` answers without asking again.

/**
 * The user dismissed the sign-in dialog (Escape, the backdrop, the close
 * button). Bubbles, `composed`, no detail — like the events of @evinvest/mfe.
 */
export const CLOSE_EVENT = 'ev:close';

/** Where a host serves its RP login, which redirects to concierge's `authorize`. */
export const RP_LOGIN_PATH = '/auth/login';

/**
 * A root-relative path on the host, or null. `//evil` and `/\evil` leave the
 * origin in a browser, so they are refused along with control characters.
 */
export function rootRelative(raw: string): string | null {
  if (!raw.startsWith('/') || raw.startsWith('//')) return null;
  if (/[\u0000-\u001F\u007F\\]/.test(raw)) return null;
  return raw;
}

/**
 * The host's RP login URL that comes back to `returnTo`
 * (`/auth/login?return_to=%2Fx`). A `returnTo` that could leave the origin
 * becomes `/`; the host's login route must re-check it server-side anyway.
 */
export function rpLoginHref(returnTo: string, loginPath: string = RP_LOGIN_PATH): string {
  const back = rootRelative(returnTo) ?? '/';
  return `${loginPath}?return_to=${encodeURIComponent(back)}`;
}
