import { useEffect, useRef, useState, type ReactNode } from 'react';

import { useElementBundle, type ElementBundleOptions } from './use-element-bundle';

/**
 * `false`, `null` and `undefined` mean "no attribute" and `true` an empty one,
 * as for a boolean HTML attribute — never the strings "false" or "null", which
 * an element checking `hasAttribute` would read as set.
 */
export type AttributeValue = string | number | boolean | object | null | undefined;

/**
 * Called with the event itself so a host can `preventDefault()` a cancelable
 * one (`ev:navigate`). `detail` comes from a bundle the host does not build:
 * validate it (`navigateHref`) before it reaches host state.
 */
export type RemoteEventHandler = (event: CustomEvent<unknown>) => void;

export interface RemoteElementProps extends ElementBundleOptions {
  /** Custom-element tag the bundle defines. */
  tag: string;
  /** URL of the self-registering ESM bundle. */
  scriptUrl: string;
  /** Set on the element; objects are JSON-encoded. A removed or `false` key removes the attribute. */
  attributes?: Readonly<Record<string, AttributeValue>>;
  /** Event name → handler. Identities may change freely; the set of names must not. */
  events?: Readonly<Record<string, RemoteEventHandler>>;
  /**
   * Keep the fallback until the element sets `data-ready` (it has painted with
   * its styles), not just until it is defined — no blank frame between them.
   * Only for elements that set the attribute; others would never leave it.
   */
  awaitPaint?: boolean;
  className?: string;
  /**
   * Mark the wrapper `ph-no-capture` (default true): the host's PostHog then
   * neither autocaptures clicks inside it (the class counts on any ancestor)
   * nor records its subtree in session replay. On the wrapper, not the remote's
   * node, so the remote's own attributes stay exactly what the host set.
   * A remote renders platform data the host does not own — an email, a
   * balance, a payout address — and the host's analytics must not copy it
   * out. Set false only for an element that shows nothing personal.
   */
  noCapture?: boolean;
  /** Shown while loading, and for good if the bundle fails. */
  fallback?: ReactNode;
}

const NO_ATTRS: Readonly<Record<string, AttributeValue>> = {};
const NO_EVENTS: Readonly<Record<string, RemoteEventHandler>> = {};
const READY_ATTR = 'data-ready';
const NO_CAPTURE_CLASS = 'ph-no-capture';

function encode(value: AttributeValue): string | null {
  if (value === false || value === null || value === undefined) return null;
  if (value === true) return '';
  return typeof value === 'object' ? JSON.stringify(value) : String(value);
}

function sync(element: HTMLElement, attrs: Readonly<Record<string, AttributeValue>>, previous: readonly string[]): string[] {
  const set: string[] = [];
  for (const [name, value] of Object.entries(attrs)) {
    const encoded = encode(value);
    if (encoded === null) continue;
    if (element.getAttribute(name) !== encoded) element.setAttribute(name, encoded);
    set.push(name);
  }
  for (const name of previous) if (!set.includes(name)) element.removeAttribute(name);
  return set;
}

/**
 * Mount a remote's custom element in light DOM — never an iframe, never a
 * shadow root: the host's tokens, fonts and Tailwind `@property` registrations
 * only cascade into light DOM. Created imperatively, not via JSX, so any
 * framework's element works without React's attribute/property quirks. To
 * remount (e.g. on a locale change the element reads only at connect), change
 * the component's `key`.
 */
export function RemoteElement({
  tag,
  scriptUrl,
  attributes = NO_ATTRS,
  events = NO_EVENTS,
  awaitPaint = false,
  className,
  noCapture = true,
  fallback = null,
  ...bundle
}: RemoteElementProps) {
  const hostRef = useRef<HTMLDivElement>(null);
  const elementRef = useRef<HTMLElement | null>(null);
  const applied = useRef<string[]>([]);
  const latest = useRef({ attributes, events });
  const [painted, setPainted] = useState(false);
  const state = useElementBundle(tag, scriptUrl, bundle);
  const ready = state === 'ready';

  useEffect(() => {
    latest.current = { attributes, events };
  });

  // Keyed on [ready, tag] only, so the element survives attribute/handler churn.
  useEffect(() => {
    const host = hostRef.current;
    if (!host || !ready) return;
    const element = document.createElement(tag);
    elementRef.current = element;
    const bound = Object.keys(latest.current.events).map((name): [string, EventListener] => {
      const listener: EventListener = event => {
        if (event instanceof CustomEvent) latest.current.events[name]?.(event);
      };
      element.addEventListener(name, listener);
      return [name, listener];
    });
    // Before appendChild: the upgrade runs `connectedCallback` synchronously,
    // which is where a remote reads its configuration.
    applied.current = sync(element, latest.current.attributes, []);
    const observer = new MutationObserver(() => setPainted(element.hasAttribute(READY_ATTR)));
    observer.observe(element, { attributes: true, attributeFilter: [READY_ATTR] });
    host.appendChild(element);
    setPainted(element.hasAttribute(READY_ATTR));
    return () => {
      observer.disconnect();
      for (const [name, listener] of bound) element.removeEventListener(name, listener);
      element.remove();
      elementRef.current = null;
      applied.current = [];
      setPainted(false);
    };
  }, [ready, tag]);

  // Serialised, so a fresh but equal object each render does not touch the element.
  const attributesKey = JSON.stringify(attributes);
  useEffect(() => {
    const element = elementRef.current;
    if (element) applied.current = sync(element, latest.current.attributes, applied.current);
  }, [attributesKey]);

  const showFallback = !ready || (awaitPaint && !painted);
  const wrapperClass = noCapture ? (className ? `${className} ${NO_CAPTURE_CLASS}` : NO_CAPTURE_CLASS) : className;
  return (
    <div ref={hostRef} className={wrapperClass} data-mfe-state={state}>
      {showFallback ? fallback : null}
    </div>
  );
}
