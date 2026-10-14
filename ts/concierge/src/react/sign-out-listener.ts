import type { RemoteEventHandler } from '@evinvest/mfe/react';

import { platformSignOutHref } from '../index';

/**
 * What `AccountChip` and `AccountPage` call on `signout`: end the host's own
 * session with a `fetch` (`POST /auth/logout`) and return its promise, which
 * is awaited before the platform sign-out. Do not navigate from it — the
 * wrapper's navigation to concierge's confirm page follows and cancels the
 * host's, and a host that navigates away first never ends the platform session.
 */
export type SignOutHandler = () => void | Promise<void>;

/**
 * The `signout` listener: the host's `onSignOut` first, then — unless the host
 * opted out — a full navigation to concierge's confirm page, which ends the
 * evinvest.ltd session and comes back to this host's root. The navigation runs
 * even when `onSignOut` rejects or throws: a platform session left alive would
 * sign the user straight back in; the error still reaches the page's handlers.
 */
export function signOutListener(onSignOut: SignOutHandler | undefined, platformSignOut: boolean): RemoteEventHandler {
  return () => {
    // The executor turns a synchronous throw into a rejection, so `finally` still runs.
    void new Promise<void>((resolve) => resolve(onSignOut?.())).finally(() => {
      // The root, not this page: concierge returns only to a path listed in the
      // scope's `return_paths`, and `/` is the one every scope with this chip lists.
      if (platformSignOut) window.location.assign(platformSignOutHref(window.location.origin + '/'));
    });
  };
}
