import { useCallback, useState } from 'react';

import { rpLoginHref } from '../index';

export interface SignInRequiredOptions {
  /**
   * What to do once the user has signed in. Default: a full navigation to the
   * host's RP login (`/auth/login?return_to=<this page>`), which completes
   * silently because the evinvest.ltd session now exists.
   */
  onSignedIn?: () => void;
}

/** The status of a response, as `fetch` and most HTTP clients expose it. */
export interface StatusLike {
  status: number;
}

export interface SignInRequired {
  open: boolean;
  /** Open the sign-in dialog. */
  requireSignIn: () => void;
  /** Open the dialog when `response` is a 401; true when it did. */
  guardResponse: (response: StatusLike) => boolean;
  /** Spread into `<SignInDialog scriptUrl={…} {...dialogProps} />`. */
  dialogProps: {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onSignedIn: () => void;
  };
}

function rpLoginHere(): void {
  const { pathname, search, hash } = window.location;
  window.location.assign(rpLoginHref(pathname + search + hash));
}

/** "This needs a session": a host that got a 401 opens the concierge sign-in dialog. */
export function useSignInRequired({ onSignedIn = rpLoginHere }: SignInRequiredOptions = {}): SignInRequired {
  const [open, setOpen] = useState(false);
  const requireSignIn = useCallback(() => setOpen(true), []);
  const guardResponse = useCallback((response: StatusLike) => {
    if (response.status !== 401) return false;
    setOpen(true);
    return true;
  }, []);
  return { open, requireSignIn, guardResponse, dialogProps: { open, onOpenChange: setOpen, onSignedIn } };
}
