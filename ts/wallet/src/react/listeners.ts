import { navigateHref } from '@evinvest/mfe';
import type { RemoteEventHandler } from '@evinvest/mfe/react';

import { openWalletAction, walletActionFromEvent, walletNavigationHref, type WalletActionOutcome } from '../index';

/**
 * Offered an in-element link: return true to take it into the host's router,
 * false to let the browser follow it. `href` is already a validated root-relative path.
 */
export type NavigateHandler = (href: string) => boolean;

/** The `ev:navigate` listener: validates the untrusted detail, then asks the host. */
export function navigateListener(onNavigate: NavigateHandler | undefined): RemoteEventHandler {
  return event => {
    const href = navigateHref(event);
    if (href !== null && onNavigate?.(href) === true) event.preventDefault();
  };
}

/**
 * The `ev:wallet-action` listener. Runs inside the element's click, so the popup
 * keeps the user's activation; when the popup is blocked anyway, the action
 * becomes a full navigation that returns here. A detail that is not a wallet
 * action on `origin` is left to the element.
 */
export function walletActionListener(onResult: ((outcome: WalletActionOutcome) => void) | undefined, origin: string | undefined): RemoteEventHandler {
  return event => {
    const action = walletActionFromEvent(event, origin);
    if (!action) return;
    event.preventDefault();
    void openWalletAction(action, origin === undefined ? {} : { origin }).then(outcome => {
      if (outcome.status === 'blocked') window.location.assign(walletNavigationHref(action));
      else onResult?.(outcome);
    });
  };
}
