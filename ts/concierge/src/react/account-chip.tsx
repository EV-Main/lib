import { API_VERSION, hostAttributes, HOST_ATTRS, NAVIGATE_EVENT, SIGNOUT_EVENT, type HostPropsInput } from '@evinvest/mfe';
import { navigateListener, RemoteElement, type NavigateHandler } from '@evinvest/mfe/react';
import type { ReactNode } from 'react';

import { ACCOUNT_CHIP_TAG, hostReturnTo, type SignInIntent } from '../index';
import { signOutListener, type SignOutHandler } from './sign-out-listener';

export interface AccountChipProps {
  /** Bundle URL from the concierge registry (`conciergeEntry(entries, 'accountChip')`). */
  scriptUrl: string;
  /**
   * Host mode: the host has its own session and says who is signed in and what
   * the menu holds. Absent, the chip asks evinvest.ltd itself (default mode).
   */
  host?: HostPropsInput;
  /** Default mode: which auth screen the signed-out CTA opens. */
  intent?: SignInIntent;
  /**
   * Default mode: where to come back to after sign-in — an absolute URL on this
   * host. No default; a relative or non-http(s) value is left out (`hostReturnTo`).
   */
  returnTo?: string;
  onNavigate?: NavigateHandler;
  /** Host mode: sign out was picked; end the host's own session. Awaited when it returns a promise. */
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
  /** Kept until the chip has painted with its styles (`data-ready`). */
  fallback?: ReactNode;
}

/** The concierge account chip, in host mode when `host` is given. */
export function AccountChip({
  scriptUrl,
  host,
  intent,
  returnTo,
  onNavigate,
  onSignOut,
  platformSignOut = true,
  minApiVersion = API_VERSION,
  className,
  fallback = null,
}: AccountChipProps) {
  const attributes = host
    ? hostAttributes(host)
    : {
        [HOST_ATTRS.intent]: intent,
        [HOST_ATTRS.returnTo]: returnTo === undefined ? undefined : hostReturnTo(returnTo),
      };
  return (
    <RemoteElement
      tag={ACCOUNT_CHIP_TAG}
      scriptUrl={scriptUrl}
      minApiVersion={minApiVersion}
      awaitPaint
      {...(className === undefined ? {} : { className })}
      attributes={attributes}
      events={{
        [NAVIGATE_EVENT]: navigateListener(onNavigate),
        [SIGNOUT_EVENT]: signOutListener(onSignOut, platformSignOut),
      }}
      fallback={fallback}
    />
  );
}
