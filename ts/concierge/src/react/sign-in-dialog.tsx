import { API_VERSION, HOST_ATTRS, SIGNED_IN_EVENT } from '@evinvest/mfe';
import { RemoteElement } from '@evinvest/mfe/react';
import { useState, type ReactNode } from 'react';

import { CLOSE_EVENT, hostReturnTo, SIGN_IN_TAG, type SignInIntent } from '../index';

export interface SignInDialogProps {
  /** Bundle URL from the concierge registry (`conciergeEntry(entries, 'signIn')`). */
  scriptUrl: string;
  open: boolean;
  /** Called with false when the user dismisses the dialog, or once sign-in is done. */
  onOpenChange: (open: boolean) => void;
  /** The evinvest.ltd session exists now; start the host's RP login (`rpLoginHref`). */
  onSignedIn: () => void;
  /** Which screen opens first. */
  intent?: SignInIntent;
  /**
   * Where an OAuth round trip (Google, GitHub) lands back: an absolute URL on
   * this host, whose origin concierge holds in the scope's registry. Default:
   * the current page without query or hash. A relative or non-https value is
   * left out (http only to loopback).
   */
  returnTo?: string;
  minApiVersion?: number;
  /**
   * Shown while the dialog is open and the bundle loads, and for good if it
   * fails — a plain link to the RP login is a fair one. Hidden while closed.
   */
  fallback?: ReactNode;
}

/**
 * The concierge sign-in dialog. The element draws its own backdrop and traps
 * focus; the host only says whether it is open. Mounted on the first open and
 * kept afterwards, so a signed-in visitor never loads the bundle and a second
 * open does not reload it.
 */
export function SignInDialog({
  scriptUrl,
  open,
  onOpenChange,
  onSignedIn,
  intent,
  returnTo,
  minApiVersion = API_VERSION,
  fallback = null,
}: SignInDialogProps) {
  const [armed, setArmed] = useState(open);
  // Latched during render, not in an effect: the first open mounts in the same commit.
  if (open && !armed) setArmed(true);
  if (!armed) return null;

  // Read only once armed, which takes an open — i.e. in the browser. Query and
  // hash stay behind: they can hold tokens or ids, and this URL travels to
  // evinvest.ltd and the OAuth provider.
  const back = returnTo ?? window.location.origin + window.location.pathname;
  return (
    <RemoteElement
      tag={SIGN_IN_TAG}
      scriptUrl={scriptUrl}
      minApiVersion={minApiVersion}
      attributes={{
        open,
        [HOST_ATTRS.intent]: intent,
        [HOST_ATTRS.returnTo]: hostReturnTo(back),
      }}
      events={{
        [SIGNED_IN_EVENT]: () => {
          onOpenChange(false);
          onSignedIn();
        },
        [CLOSE_EVENT]: () => onOpenChange(false),
      }}
      fallback={open ? fallback : null}
    />
  );
}
