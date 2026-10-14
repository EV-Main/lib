import { API_VERSION, hostAttributes, HOST_ATTRS, NAVIGATE_EVENT, safeHref, SIGNOUT_EVENT, type HostPropsInput } from '@evinvest/mfe';
import { RemoteElement } from '@evinvest/mfe/react';
import type { ReactNode } from 'react';

import { ACCOUNT_CHIP_TAG } from '../index';
import { navigateListener, type NavigateHandler } from './events';

export interface AccountChipProps {
  /** Bundle URL from the concierge registry (`conciergeEntry(entries, 'accountChip')`). */
  scriptUrl: string;
  /**
   * Host mode: the host has its own session and says who is signed in and what
   * the menu holds. Absent, the chip asks evinvest.ltd itself (default mode).
   */
  host?: HostPropsInput;
  /** Default mode: which auth screen the signed-out CTA opens. */
  intent?: 'login';
  /** Default mode: where to come back to after sign-in. */
  returnTo?: string;
  onNavigate?: NavigateHandler;
  /** Host mode: sign out was picked; end the host's own session. */
  onSignOut?: () => void;
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
  minApiVersion = API_VERSION,
  className,
  fallback = null,
}: AccountChipProps) {
  const attributes = host
    ? hostAttributes(host)
    : {
        [HOST_ATTRS.intent]: intent,
        [HOST_ATTRS.returnTo]: returnTo === undefined ? undefined : safeHref(returnTo),
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
        [SIGNOUT_EVENT]: () => onSignOut?.(),
      }}
      fallback={fallback}
    />
  );
}
