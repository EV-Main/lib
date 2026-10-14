import { API_VERSION, NAVIGATE_EVENT, SIGNOUT_EVENT } from '@evinvest/mfe';
import { navigateListener, RemoteElement, type NavigateHandler } from '@evinvest/mfe/react';
import type { ReactNode } from 'react';

import { ACCOUNT_TAG } from '../index';

export interface AccountPageProps {
  /** Bundle URL from the concierge registry (`conciergeEntry(entries, 'account')`). */
  scriptUrl: string;
  onNavigate?: NavigateHandler;
  /** Sign out was picked; end the host's own session. */
  onSignOut?: () => void;
  minApiVersion?: number;
  className?: string;
  fallback?: ReactNode;
}

/**
 * The concierge account page — profile, security, notifications — for the
 * host's `/account` route. The element reads and writes everything itself on
 * evinvest.ltd; the host only routes its links and hears sign-out.
 */
export function AccountPage({ scriptUrl, onNavigate, onSignOut, minApiVersion = API_VERSION, className, fallback = null }: AccountPageProps) {
  return (
    <RemoteElement
      tag={ACCOUNT_TAG}
      scriptUrl={scriptUrl}
      minApiVersion={minApiVersion}
      {...(className === undefined ? {} : { className })}
      events={{
        [NAVIGATE_EVENT]: navigateListener(onNavigate),
        [SIGNOUT_EVENT]: () => onSignOut?.(),
      }}
      fallback={fallback}
    />
  );
}
