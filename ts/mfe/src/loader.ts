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

// What is shared per tag across mounts (and React's dev double effect): the
// script and its failure signal. The verdict — defined in time, new enough — is
// taken per call, so a strict caller and a lenient one sharing a load each get
// their own answer. A failed load is forgotten so the next mount retries.
interface Load {
  scriptUrl: string;
  failure: Promise<void>;
}

const loads = new Map<string, Load>();

const NEVER = new Promise<void>(() => {});

function scriptsFor(tag: string): HTMLScriptElement[] {
  return Array.from(document.querySelectorAll<HTMLScriptElement>('script[data-mfe]')).filter(
    script => script.dataset['mfe'] === tag,
  );
}

function inject(tag: string, scriptUrl: string): Promise<void> {
  // `data-mfe` marks the tag's script for every loader on the page, including
  // the hand-rolled ones hosts carried before this package: one already in
  // flight is waited on, not injected twice.
  if (scriptsFor(tag).length > 0) return NEVER;
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

function forget(tag: string, load: Load): void {
  if (loads.get(tag) !== load) return;
  loads.delete(tag);
  // A script that timed out stays in the DOM with its marker; left there, every
  // later mount would take it for "in flight" and sit out the full timeout.
  for (const script of scriptsFor(tag)) script.remove();
}

/**
 * Inject `scriptUrl` (once per tag) and resolve whether `tag` became usable for
 * this caller's `minApiVersion`. A 404 served as HTML/JSON (a MIME error), an
 * older bundle and a timeout all resolve false — never an endless wait. A
 * second URL for a tag already loading is reported, not loaded: a tag can be
 * defined once per page, so it could never take effect. Browser only.
 */
export function loadElement(tag: string, scriptUrl: string, options: LoadOptions = {}): Promise<boolean> {
  let load = loads.get(tag);
  if (load && load.scriptUrl !== scriptUrl) {
    console.warn(`@evinvest/mfe: <${tag}> is already loading from ${load.scriptUrl}; ignoring ${scriptUrl}`);
  }
  if (!load) {
    load = { scriptUrl, failure: customElements.get(tag) ? NEVER : inject(tag, scriptUrl) };
    loads.set(tag, load);
  }
  const current = load;
  const verdict = awaitElement(
    customElements,
    tag,
    options.minApiVersion ?? 0,
    options.timeoutMs ?? DEFAULT_TIMEOUT_MS,
    current.failure,
  );
  void verdict.then(ok => {
    // Defined but too old is a final answer for this caller, not a failed
    // load: the definition stays and a more lenient caller may still use it.
    if (!ok && !customElements.get(tag)) forget(tag, current);
  });
  return verdict;
}

/**
 * The cascade layer remote stylesheets are moved into. The account chip bakes
 * the same name into its own inlined `<style>`, so a host declares this layer
 * first (below all of its own) once, and every remote sits under the host.
 */
export const REMOTE_LAYER = 'reamfe';

// How long a demoted `<link>` may stay beside its layered replacement when the
// replacement's load event never comes (a browser that does not fire it).
const SWAP_GRACE_MS = 10_000;

/**
 * Demote the stylesheets a remote injects into `layer`. A light-DOM remote
 * adds its Tailwind sheet to `<head>`; loading last, its unlayered utilities
 * beat the host's (a non-responsive `.grid-cols-1` once collapsed the host's
 * `lg:grid-cols-3`).
 *
 * Only `<link>`s added after this call, served from the bundle's directory,
 * are touched — never one already on the page, which is the host's own. Keyed
 * on the path, not the origin, because a remote is often proxied through the
 * host's own origin. A bundle at the origin root has no directory of its own
 * to tell its sheets from the host's, so nothing is demoted for it. Returns the
 * cleanup. Browser only; a no-op on a server.
 */
export function containRemoteStyles(scriptUrl: string, layer: string = REMOTE_LAYER): () => void {
  if (typeof document === 'undefined') return () => {};
  const base = new URL(scriptUrl, window.location.href);
  if (base.pathname.lastIndexOf('/') === 0) return () => {};
  const assetDir = base.origin + base.pathname.slice(0, base.pathname.lastIndexOf('/') + 1);

  const demote = (link: HTMLLinkElement) => {
    const style = document.createElement('style');
    style.dataset['mfeLayer'] = layer;
    style.textContent = `@import url(${JSON.stringify(link.href)}) layer(${layer});`;
    // The link goes only once its layered copy has loaded: removed first, the
    // remote would paint unstyled until the import arrived.
    let timer: ReturnType<typeof setTimeout> | undefined;
    const drop = () => {
      clearTimeout(timer);
      link.remove();
    };
    style.addEventListener('load', drop, { once: true });
    style.addEventListener('error', drop, { once: true });
    timer = setTimeout(drop, SWAP_GRACE_MS);
    link.before(style);
  };

  const observer = new MutationObserver(records => {
    for (const record of records) {
      record.addedNodes.forEach(node => {
        if (node instanceof HTMLLinkElement && node.rel === 'stylesheet' && node.href.startsWith(assetDir)) {
          demote(node);
        }
      });
    }
  });
  observer.observe(document.head, { childList: true });
  return () => observer.disconnect();
}
