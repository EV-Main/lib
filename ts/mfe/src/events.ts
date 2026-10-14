// What an element tells its host, as DOM events dispatched from the element.
// All of them bubble and are `composed`, so a host that wraps the element in its
// own shadow root still hears them.
//
// `signout` keeps its unprefixed name: it is part of the frozen apiVersion-2
// chip contract, and renaming it to `ev:signout` would be apiVersion 3.

/**
 * The host-contract version this package speaks. A producer sets it as a static
 * on its element class (`static apiVersion = API_VERSION`); a host refuses an
 * older bundle rather than half-render it. Additive changes keep the number;
 * a renamed attribute or event bumps it.
 */
export const API_VERSION = 2;

/** Whether a defined element class speaks at least `min` of the host contract. */
export function speaksApi(ctor: unknown, min: number): boolean {
  if (typeof ctor !== 'function') return false;
  const version: unknown = Reflect.get(ctor, 'apiVersion');
  return typeof version === 'number' && version >= min;
}

/**
 * Cancelable: a plain same-origin link click inside the element, offered to the
 * host's router. The host calls `preventDefault()` to take it and pushes the
 * route itself; left alone, the browser follows the link.
 */
export const NAVIGATE_EVENT = 'ev:navigate';
/** The user finished signing in inside the element; the host starts its own RP login. */
export const SIGNED_IN_EVENT = 'ev:signed-in';
/** Sign out was picked in host mode; the host ends its own session. */
export const SIGNOUT_EVENT = 'signout';

/** `detail` of {@link NAVIGATE_EVENT}: the destination as path + query + hash. */
export interface NavigateDetail {
  href: string;
}

export interface MfeEventMap {
  [NAVIGATE_EVENT]: CustomEvent<NavigateDetail>;
  [SIGNED_IN_EVENT]: CustomEvent<null>;
  [SIGNOUT_EVENT]: CustomEvent<null>;
}

export type MfeEventName = keyof MfeEventMap;

const BASE = { bubbles: true, composed: true } as const;

/**
 * Offer `href` to the host from `source`. True when the host took it — the
 * producer then cancels its own click.
 */
export function dispatchNavigate(source: EventTarget, href: string): boolean {
  const event = new CustomEvent<NavigateDetail>(NAVIGATE_EVENT, {
    ...BASE,
    cancelable: true,
    detail: { href },
  });
  return !source.dispatchEvent(event);
}

export function dispatchSignedIn(source: EventTarget): void {
  source.dispatchEvent(new CustomEvent<null>(SIGNED_IN_EVENT, { ...BASE, detail: null }));
}

export function dispatchSignOut(source: EventTarget): void {
  source.dispatchEvent(new CustomEvent<null>(SIGNOUT_EVENT, { ...BASE, detail: null }));
}

/**
 * The `href` of an `ev:navigate` event, or null when the detail is missing or
 * malformed. `detail` comes from a bundle the host does not build — read it as
 * untrusted. Only root-relative paths pass: a host router cannot route
 * anything else, and `//evil` or `/\evil` would leave the origin.
 */
export function navigateHref(event: Event): string | null {
  const detail: unknown = event instanceof CustomEvent ? event.detail : undefined;
  if (typeof detail !== 'object' || detail === null) return null;
  const href: unknown = Reflect.get(detail, 'href');
  if (typeof href !== 'string' || !href.startsWith('/')) return null;
  if (href.startsWith('//') || /[\u0000-\u001F\u007F\\]/.test(href)) return null;
  return href;
}

/** The parts of a click {@link delegatedHref} reads (DOM `MouseEvent` or React's wrapper). */
export interface LinkClick {
  button: number;
  ctrlKey: boolean;
  metaKey: boolean;
  shiftKey: boolean;
  altKey: boolean;
  defaultPrevented: boolean;
}

/** The parts of the clicked `<a>` {@link delegatedHref} reads. */
export interface LinkAnchor {
  href: string;
  target: string;
  hasAttribute(name: string): boolean;
}

/**
 * Producer side: the path to offer the host for a link click, or null when the
 * click is the browser's — modified or non-primary (new tab, download), a
 * `target`/`download` link, already handled, or another origin, which no host
 * router can reach. Enter on a focused link arrives as a `button` 0 click.
 */
export function delegatedHref(
  click: LinkClick,
  anchor: LinkAnchor,
  page: { href: string; origin: string },
): string | null {
  if (click.defaultPrevented || click.button !== 0) return null;
  if (click.ctrlKey || click.metaKey || click.shiftKey || click.altKey) return null;
  if ((anchor.target && anchor.target !== '_self') || anchor.hasAttribute('download')) return null;
  let url: URL;
  try {
    url = new URL(anchor.href, page.href);
  } catch {
    return null;
  }
  if (url.origin !== page.origin) return null;
  return url.pathname + url.search + url.hash;
}
