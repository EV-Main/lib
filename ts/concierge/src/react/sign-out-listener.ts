import type { RemoteEventHandler } from '@evinvest/mfe/react';

import { platformSignOutHref } from '../index';

/** What `AccountChip` and `AccountPage` call on `signout`; a promise is awaited before the platform sign-out. */
export type SignOutHandler = () => void | Promise<void>;

/**
 * The `signout` listener: the host's `onSignOut` first, then — unless the host
 * opted out — a full navigation to concierge's confirm page, which ends the
 * evinvest.ltd session and comes back to this page. The navigation runs even
 * when `onSignOut` rejects or throws: a platform session left alive would sign
 * the user straight back in; the error still reaches the page's handlers.
 */
export function signOutListener(onSignOut: SignOutHandler | undefined, platformSignOut: boolean): RemoteEventHandler {
  return () => {
    // The executor turns a synchronous throw into a rejection, so `finally` still runs.
    void new Promise<void>((resolve) => resolve(onSignOut?.())).finally(() => {
      if (platformSignOut) window.location.assign(platformSignOutHref(window.location.origin + window.location.pathname));
    });
  };
}
