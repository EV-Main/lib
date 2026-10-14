import { API_VERSION, NAVIGATE_EVENT } from '@evinvest/mfe';
import { RemoteElement } from '@evinvest/mfe/react';
import type { ReactNode } from 'react';

import { WALLET_ACTION_EVENT, WALLET_TAG, type WalletActionOutcome } from '../index';
import { navigateListener, walletActionListener, type NavigateHandler } from './listeners';

export interface WalletProps {
  /** Bundle URL from the wallet registry (`walletEntry(entries, 'wallet')`). */
  scriptUrl: string;
  /**
   * A scope host's id: the element shows that scope's compartment and main,
   * balances and history only. Absent, the full wallet — evinvest.ltd itself.
   */
  scope?: string;
  /** A money action finished in its popup; refresh what the host shows. */
  onResult?: (outcome: WalletActionOutcome) => void;
  onNavigate?: NavigateHandler;
  /** Where action results must come from; only for a staging stand. */
  origin?: string;
  minApiVersion?: number;
  className?: string;
  fallback?: ReactNode;
}

/** The banking wallet. On a scope host its money actions run on evinvest.ltd, in a popup. */
export function Wallet({ scriptUrl, scope, onResult, onNavigate, origin, minApiVersion = API_VERSION, className, fallback = null }: WalletProps) {
  return (
    <RemoteElement
      tag={WALLET_TAG}
      scriptUrl={scriptUrl}
      minApiVersion={minApiVersion}
      {...(className === undefined ? {} : { className })}
      attributes={{ scope }}
      events={{
        [NAVIGATE_EVENT]: navigateListener(onNavigate),
        [WALLET_ACTION_EVENT]: walletActionListener(onResult, origin),
      }}
      fallback={fallback}
    />
  );
}
