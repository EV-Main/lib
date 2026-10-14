// Browser-side loading of a self-registering ESM bundle. Nothing here touches
// the DOM at import time, so the module is safe to import on a server.

import { speaksApi } from './events';

/** The slice of `customElements` the wait needs; a parameter so it runs without a DOM. */
export interface ElementRegistry {
  get(tag: string): CustomElementConstructor | undefined;
  whenDefined(tag: string): Promise<CustomElementConstructor>;
}

export interface LoadOptions {
  /** Refuse a bundle whose element class has an older `static apiVersion`. 0 accepts any. */
  minApiVersion?: number;
  /** A bundle that neither loads nor fails (a hung proxy) must not hold the fallback forever. */
  timeoutMs?: number;
}

const DEFAULT_TIMEOUT_MS = 8000;

/**
 * Settles true once `tag` is defined by a class speaking `minApiVersion`; false
 * when `failure` settles (the script did not load), on timeout, or on an older
 * bundle. Never rejects.
 */
export function awaitElement(
  registry: ElementRegistry,
  tag: string,
  minApiVersion: number,
  timeoutMs: number,
  failure: Promise<void>,
): Promise<boolean> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  const timeout = new Promise<false>(resolve => {
    timer = setTimeout(() => resolve(false), timeoutMs);
  });
  const defined = registry.whenDefined(tag).then(
    ctor => minApiVersion <= 0 || speaksApi(ctor, minApiVersion),
    () => false,
  );
  return Promise.race([defined, timeout, failure.then(() => false)]).finally(() => clearTimeout(timer));
}

// One load per tag across mounts (and React's dev double effect): a good load
// is shared, a failed one is forgotten so the next mount retries.
const loads = new Map<string, Promise<boolean>>();

const NEVER = new Promise<void>(() => {});

function inject(tag: string, scriptUrl: string): Promise<void> {
  // `data-mfe` marks the tag's script for every loader on the page, including
  // the hand-rolled ones hosts carried before this package: one already in
  // flight is waited on, not injected twice.
  const existing = Array.from(document.querySelectorAll<HTMLScriptElement>('script[data-mfe]')).some(
    script => script.dataset['mfe'] === tag,
  );
  if (existing) return NEVER;
  const script = document.createElement('script');
  script.type = 'module';
  script.src = scriptUrl;
  script.dataset['mfe'] = tag;
  return new Promise<void>(resolve => {
    script.addEventListener(
      'error',
      () => {
        // Left in the DOM, the marker above would read as "in flight" forever.
        script.remove();
        resolve();
      },
      { once: true },
    );
    document.head.appendChild(script);
  });
}

/**
 * Inject `scriptUrl` (once per tag) and resolve whether `tag` became usable.
 * A 404 served as HTML/JSON (a MIME error), an older bundle and a timeout all
 * resolve false — never an endless wait. Browser only.
 */
export function loadElement(tag: string, scriptUrl: string, options: LoadOptions = {}): Promise<boolean> {
  const min = options.minApiVersion ?? 0;
  const pending = loads.get(tag);
  // The cache is per tag, not per threshold: a stricter caller after a lenient
  // load must still be refused an older bundle.
  if (pending) return pending.then(ok => ok && (min === 0 || speaksApi(customElements.get(tag), min)));
  const failure = customElements.get(tag) ? NEVER : inject(tag, scriptUrl);
  const load = awaitElement(
    customElements,
    tag,
    min,
    options.timeoutMs ?? DEFAULT_TIMEOUT_MS,
    failure,
  );
  loads.set(tag, load);
  void load.then(ok => {
    if (!ok) loads.delete(tag);
  });
  return load;
}

/**
 * The cascade layer remote stylesheets are moved into. The account chip bakes
 * the same name into its own inlined `<style>`, so a host declares this layer
 * first (below all of its own) once, and every remote sits under the host.
 */
export const REMOTE_LAYER = 'reamfe';

/**
 * Demote every `<link rel="stylesheet">` served from the bundle's directory
 * into `layer`. A light-DOM remote injects its Tailwind sheet into `<head>`;
 * loading last, its unlayered utilities beat the host's (a non-responsive
 * `.grid-cols-1` once collapsed the host's `lg:grid-cols-3`). Keyed on the
 * asset path, not the origin, because a remote is often proxied through the
 * host's own origin. Returns the cleanup. Browser only; a no-op on a server.
 */
export function containRemoteStyles(scriptUrl: string, layer: string = REMOTE_LAYER): () => void {
  if (typeof document === 'undefined') return () => {};
  const base = new URL(scriptUrl, window.location.href);
  const assetDir = base.href.slice(0, base.href.lastIndexOf('/') + 1);

  const demote = (link: HTMLLinkElement) => {
    const style = document.createElement('style');
    style.dataset['mfeLayer'] = layer;
    style.textContent = `@import url(${JSON.stringify(link.href)}) layer(${layer});`;
    link.replaceWith(style);
  };
  const scan = () => {
    document.head.querySelectorAll<HTMLLinkElement>('link[rel="stylesheet"]').forEach(link => {
      if (link.href.startsWith(assetDir)) demote(link);
    });
  };

  scan();
  const observer = new MutationObserver(scan);
  observer.observe(document.head, { childList: true });
  return () => observer.disconnect();
}
