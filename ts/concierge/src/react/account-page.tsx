import { API_VERSION, NAVIGATE_EVENT, SIGNOUT_EVENT } from '@evinvest/mfe';
import { navigateListener, RemoteElement, type NavigateHandler } from '@evinvest/mfe/react';
import type { ReactNode } from 'react';

import { ACCOUNT_TAG } from '../index';
import { signOutListener, type SignOutHandler } from './sign-out-listener';

export interface AccountPageProps {
  /** Bundle URL from the concierge registry (`conciergeEntry(entries, 'account')`). */
  scriptUrl: string;
  onNavigate?: NavigateHandler;
  /** Sign out was picked; end the host's own session. Awaited when it returns a promise. */
  onSignOut?: SignOutHandler;
  /**
   * After `onSignOut`, also end the evinvest.ltd session by navigating to
   * concierge's confirm page (`platformSignOutHref`). Default true: the host's
   * sign-out alone leaves the platform session, which signs the next person at
   * this browser back in silently. Pass false only when the host ends the
   * platform session another way, and say why in a comment at the call site.
   */
  platformSignOut?: boolean;
  minApiVersion?: number;
  className?: string;
  fallback?: ReactNode;
}

/**
 * The concierge account page — profile, security, notifications — for the
 * host's `/account` route. The element reads and writes everything itself on
 * evinvest.ltd; the host only routes its links and hears sign-out.
 */
export function AccountPage({
  scriptUrl,
  onNavigate,
  onSignOut,
  platformSignOut = true,
  minApiVersion = API_VERSION,
  className,
  fallback = null,
}: AccountPageProps) {
  return (
    <RemoteElement
      tag={ACCOUNT_TAG}
      scriptUrl={scriptUrl}
      minApiVersion={minApiVersion}
      {...(className === undefined ? {} : { className })}
      events={{
        [NAVIGATE_EVENT]: navigateListener(onNavigate),
        [SIGNOUT_EVENT]: signOutListener(onSignOut, platformSignOut),
      }}
      fallback={fallback}
    />
  );
}
