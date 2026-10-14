/**
 * @module @evinvest/wallet
 *
 * Host side of the banking wallet microfrontends (`Wallet`, `PayInvoiceButton`):
 * where they live, their tags, and the popup round trip that carries a money
 * action to evinvest.ltd and brings its result back. Server-safe: nothing
 * touches the DOM at import time. The React wrappers live in `./react`. Built
 * on @evinvest/mfe; the elements themselves are banking's.
 */

export {
  fetchWalletRegistry,
  PAY_INVOICE_TAG,
  WALLET_ELEMENTS,
  WALLET_ORIGIN,
  WALLET_REGISTRY_URL,
  WALLET_TAG,
  walletEntry,
} from './elements';
export type { WalletElement } from './elements';

export {
  OPENER_PARAM,
  openWalletAction,
  parseWalletMessage,
  RETURN_TO_PARAM,
  WALLET_ACTION_EVENT,
  WALLET_ACTION_PATH,
  WALLET_RESULT_MESSAGE,
  walletActionFromEvent,
  walletActionUrl,
  walletNavigationHref,
} from './action';
export type { WalletActionOptions, WalletActionOutcome, WalletResult } from './action';
