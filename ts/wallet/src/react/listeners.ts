import type { RemoteEventHandler } from '@evinvest/mfe/react';

import { openWalletAction, walletActionFromEvent, walletNavigationHref, type WalletActionOutcome } from '../index';

/**
 * The `ev:wallet-action` listener of a scope host. Runs inside the element's
 * click, so the popup keeps the user's activation; when the popup is blocked
 * anyway, the action becomes a full navigation that returns here. A second click
 * while a popup is open only focuses it (`busy`): one action, one `onResult`.
 * A detail that is not a wallet action on `origin` is left to the element.
 */
export function walletActionListener(onResult: ((outcome: WalletActionOutcome) => void) | undefined, origin: string | undefined): RemoteEventHandler {
  return event => {
    const action = walletActionFromEvent(event, origin);
    if (!action) return;
    event.preventDefault();
    void openWalletAction(action, origin === undefined ? {} : { origin }).then(outcome => {
      if (outcome.status === 'blocked') window.location.assign(walletNavigationHref(action));
      else if (outcome.status !== 'busy') onResult?.(outcome);
    });
  };
}

/** Leaves the action to the element: on evinvest.ltd itself it runs in place. */
export const actionInPlace: RemoteEventHandler = () => {};
