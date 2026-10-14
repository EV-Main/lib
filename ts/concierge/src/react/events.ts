import { navigateHref } from '@evinvest/mfe';
import type { RemoteEventHandler } from '@evinvest/mfe/react';

/**
 * Offered an in-element link: return true to take it into the host's router
 * (the element then leaves the click alone), false to let the browser follow it.
 * `href` is already validated as a root-relative path.
 */
export type NavigateHandler = (href: string) => boolean;

/** The `ev:navigate` listener: validates the untrusted detail, then asks the host. */
export function navigateListener(onNavigate: NavigateHandler | undefined): RemoteEventHandler {
  return event => {
    const href = navigateHref(event);
    if (href !== null && onNavigate?.(href) === true) event.preventDefault();
  };
}
