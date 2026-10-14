/**
 * @module @evinvest/concierge
 *
 * Host side of the concierge microfrontends (sign-in dialog, account page,
 * account chip): where they live, their tags, the session shape, the RP-login
 * and platform sign-out hrefs. Server-safe: nothing touches the DOM at import
 * time. The React wrappers live in `./react`. Built on @evinvest/mfe; the
 * elements themselves are concierge's `web/`.
 */

export {
  ACCOUNT_CHIP_TAG,
  ACCOUNT_TAG,
  CONCIERGE_ELEMENTS,
  CONCIERGE_MFE_PREFIX,
  CONCIERGE_ORIGIN,
  CONCIERGE_REGISTRY_URL,
  conciergeEntry,
  fetchConciergeRegistry,
  SIGN_IN_TAG,
} from './elements';
export type { ConciergeElement, ConciergeRegistryOptions } from './elements';

export { fetchSession, parseSession, SESSION_URL } from './session';
export type { Session, SessionUser } from './session';

export { CLOSE_EVENT, hostReturnTo, RP_LOGIN_PATH, rpLoginHref } from './sign-in';
export type { SignInIntent } from './sign-in';

export { PLATFORM_SIGN_OUT_URL, platformSignOutHref } from './sign-out';
