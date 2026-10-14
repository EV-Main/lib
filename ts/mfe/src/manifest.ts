// The registry contract: which microfrontends a host may mount and where their
// bundles live. A producer's build emits `{name, tag, kind}` (the Rust `mfe!`
// macro's `MFE_MANIFEST`, shipped as `mfe.json`); the host — or the producer's
// published manifest — adds `scriptUrl`.
//
// Trust is the host's CSP (`script-src`, scoped to the bundle directory), not
// a hash pinned here (D20): a pin had to be re-taken by hand after every
// producer release. The registry still refuses URLs no CSP should ever have to
// judge (`javascript:`, `data:`, protocol-relative, plain http off loopback),
// and `fetchRegistry` refuses a bundle outside the directory the caller names,
// because a bad entry is easier to read here than in a console full of CSP
// reports — and a CSP allowing the whole origin would let it run.

import { isSecureTransport } from './transport';

/** Inline widget inside a host page, or the owner of a whole route. */
export type MfeKind = 'component' | 'page';

export const MFE_KINDS: readonly MfeKind[] = ['component', 'page'];

/** What a producer's build emits (`mfe.json`), before it has a URL. */
export interface MfeManifest {
  /** Logical name, `{service}.{name}` for bundles built with the Rust `mfe!`. */
  name: string;
  /** Globally unique custom-element tag the bundle defines. */
  tag: string;
  kind: MfeKind;
}

/** One registry entry: a manifest plus where its self-registering ESM bundle is. */
export interface MfeEntry extends MfeManifest {
  /** Root-relative (`/cabinet/mfe/x.js`) or absolute https URL of the bundle (http only to loopback). */
  scriptUrl: string;
}

/** Expected failures are values here; only the caller knows whether one is fatal. */
export type ParseResult<T> =
  | { ok: true; value: T }
  | { ok: false; error: string };

// A valid custom-element name: lowercase ASCII start, at least one hyphen. The
// spec allows more (PCENChar), but no tag of ours needs it and a narrower rule
// catches a typo before `customElements.define` throws in someone's browser.
const TAG = /^[a-z][a-z0-9._]*(-[a-z0-9._]*)+$/;

const MAX_URL = 2048;

function field(source: object, key: string): unknown {
  return Reflect.get(source, key);
}

function isKind(value: unknown): value is MfeKind {
  return MFE_KINDS.some(kind => kind === value);
}

/**
 * Whether `url` may be a bundle URL: root-relative, or absolute `https:` —
 * `http:` only to a loopback host, for a dev stand. Tabs, newlines and
 * backslashes are refused outright — browsers strip the first two and read `\`
 * as `/`, so `/\evil` would load from another host.
 */
export function isBundleUrl(url: string): boolean {
  if (url.length === 0 || url.length > MAX_URL) return false;
  if (/[\u0000-\u001F\u007F\\\s]/.test(url)) return false;
  if (url.startsWith('/')) return !url.startsWith('//');
  try {
    return isSecureTransport(new URL(url));
  } catch {
    return false;
  }
}

/** Validate a producer manifest (`mfe.json`). */
export function parseManifest(raw: unknown): ParseResult<MfeManifest> {
  if (typeof raw !== 'object' || raw === null) {
    return { ok: false, error: 'manifest is not an object' };
  }
  const name = field(raw, 'name');
  const tag = field(raw, 'tag');
  const kind = field(raw, 'kind');
  if (typeof name !== 'string' || name.length === 0) {
    return { ok: false, error: 'manifest `name` is missing or empty' };
  }
  if (typeof tag !== 'string' || !TAG.test(tag)) {
    return { ok: false, error: `${name}: \`tag\` is not a custom-element name` };
  }
  if (!isKind(kind)) {
    return { ok: false, error: `${name}: \`kind\` is not "component" or "page"` };
  }
  return { ok: true, value: { name, tag, kind } };
}

/**
 * Validate one registry entry. Unknown fields are ignored, so a registry that
 * still carries the retired `integrity` pin parses; the pin is simply dropped.
 */
export function parseEntry(raw: unknown): ParseResult<MfeEntry> {
  const manifest = parseManifest(raw);
  if (!manifest.ok) return manifest;
  const scriptUrl = typeof raw === 'object' && raw !== null ? field(raw, 'scriptUrl') : undefined;
  if (typeof scriptUrl !== 'string' || !isBundleUrl(scriptUrl)) {
    return { ok: false, error: `${manifest.value.name}: \`scriptUrl\` is not a root-relative or https URL` };
  }
  return { ok: true, value: { ...manifest.value, scriptUrl } };
}

/**
 * Validate a whole registry. All or nothing: one bad entry fails the lot, so a
 * broken registry surfaces at load instead of as one silently missing widget.
 * Duplicate names and tags are refused — a second `define` of a tag is a no-op
 * in every loader, so the second bundle would never run.
 */
export function parseRegistry(raw: unknown): ParseResult<MfeEntry[]> {
  if (!Array.isArray(raw)) return { ok: false, error: 'registry is not an array' };
  const entries: MfeEntry[] = [];
  const names = new Set<string>();
  const tags = new Set<string>();
  for (const [index, item] of raw.entries()) {
    const entry = parseEntry(item);
    if (!entry.ok) return { ok: false, error: `entry ${index}: ${entry.error}` };
    const { name, tag } = entry.value;
    if (names.has(name)) return { ok: false, error: `entry ${index}: duplicate name ${name}` };
    if (tags.has(tag)) return { ok: false, error: `entry ${index}: duplicate tag ${tag}` };
    names.add(name);
    tags.add(tag);
    entries.push(entry.value);
  }
  return { ok: true, value: entries };
}

/** Attach a URL to a producer manifest, making it a registry entry. */
export function entryFromManifest(manifest: MfeManifest, scriptUrl: string): ParseResult<MfeEntry> {
  return parseEntry({ ...manifest, scriptUrl });
}

/** The entry named `name`, or undefined. The tag is not checked; {@link entryFor} checks it. */
export function findEntry(entries: readonly MfeEntry[], name: string): MfeEntry | undefined {
  return entries.find(entry => entry.name === name);
}

/**
 * The entry for an element a host wrapper knows by `{name, tag}`. Refused when
 * missing, or when the registry gives that name another tag: the wrapper mounts
 * the tag it knows, so it would load a bundle that never defines it.
 */
export function entryFor(entries: readonly MfeEntry[], known: { name: string; tag: string }): ParseResult<MfeEntry> {
  const entry = findEntry(entries, known.name);
  if (!entry) return { ok: false, error: `registry has no ${known.name}` };
  if (entry.tag !== known.tag) return { ok: false, error: `${known.name}: tag ${entry.tag}, expected ${known.tag}` };
  return { ok: true, value: entry };
}

/**
 * Resolve every root-relative `scriptUrl` against `base` — the URL the registry
 * was fetched from. A producer publishes `/auth/mfe/x.js` meaning *its* origin;
 * left relative, a host on another origin would load it from itself.
 */
export function resolveEntries(entries: readonly MfeEntry[], base: string | URL): MfeEntry[] {
  return entries.map(entry => ({ ...entry, scriptUrl: new URL(entry.scriptUrl, base).href }));
}

export interface FetchRegistryOptions {
  /**
   * The directory every bundle must live in, as an absolute URL ending in `/`
   * (`https://evinvest.ltd/api/auth/mfe/`). Required: the host's CSP names the
   * same directory, and a registry entry pointing elsewhere on the producer's
   * origin — an upload, a user page — must not become a script on the host.
   */
  allowedPrefix: string;
  /** Passed to `fetch`; its `redirect` is always overridden with `'error'`. */
  init?: RequestInit;
}

// Encoded slashes survive URL normalisation, so a server that decodes them
// could still serve `/mfe/..%2f..%2fx.js` from outside the directory.
const ENCODED_SEPARATOR = /%2f|%5c/i;

function prefixError(prefix: string): string | null {
  let parsed: URL;
  try {
    parsed = new URL(prefix);
  } catch {
    return `allowedPrefix ${prefix} is not an absolute URL`;
  }
  // Without the trailing slash `https://evinvest.ltd/mfe` would also admit
  // `/mfe-uploads/`; a non-canonical spelling would never match a resolved URL.
  if (!prefix.endsWith('/') || parsed.href !== prefix || parsed.search !== '' || parsed.hash !== '') {
    return `allowedPrefix ${prefix} is not a canonical directory URL ending in /`;
  }
  // The origin root admits every path on it, so the check would shrink to an
  // origin comparison and an upload or a user page would pass as a bundle.
  if (parsed.pathname === '/') return `allowedPrefix ${prefix} is the origin root, not a bundle directory`;
  return isSecureTransport(parsed) ? null : `allowedPrefix ${prefix} is not https`;
}

function outsidePrefix(entries: readonly MfeEntry[], prefix: string): string | null {
  for (const [index, entry] of entries.entries()) {
    const { pathname } = new URL(entry.scriptUrl);
    if (!entry.scriptUrl.startsWith(prefix) || ENCODED_SEPARATOR.test(pathname)) {
      return `entry ${index}: ${entry.name}: scriptUrl ${entry.scriptUrl} is outside ${prefix}`;
    }
  }
  return null;
}

/**
 * Fetch and validate a producer's published registry, with its relative URLs
 * resolved against the registry URL. Redirects are refused (`redirect:
 * 'error'`, and a `response.url` on another origin fails too): a registry that
 * moved could otherwise hand its entries' base to whoever it moved to. The
 * whole registry fails when any `scriptUrl`, once resolved, does not start
 * with `allowedPrefix`. Network, HTTP and URL failures are results, not
 * throws: a host renders its fallback either way — and so is a call without
 * `allowedPrefix`, which a plain-JS host or a stale caller can still make.
 */
export async function fetchRegistry(url: string | URL, options?: FetchRegistryOptions): Promise<ParseResult<MfeEntry[]>> {
  // Read as unknown: plain-JS callers reach here past the type, with any value or none.
  const allowedPrefix: unknown = options?.allowedPrefix;
  if (typeof allowedPrefix !== 'string') return { ok: false, error: `registry ${String(url)}: allowedPrefix is required` };
  const init = options?.init;
  const badPrefix = prefixError(allowedPrefix);
  if (badPrefix) return { ok: false, error: `registry ${String(url)}: ${badPrefix}` };
  try {
    // A relative registry URL means this page's origin; on a server there is
    // none, and the throw lands in the catch.
    const requested = new URL(url, typeof location === 'undefined' ? undefined : location.href);
    const response = await fetch(requested, { ...init, redirect: 'error' });
    if (!response.ok) return { ok: false, error: `registry ${String(url)}: HTTP ${response.status}` };
    // `response.url` is empty for a hand-built Response; then there is nothing to compare.
    if (response.url !== '' && new URL(response.url).origin !== requested.origin) {
      return { ok: false, error: `registry ${String(url)}: served from another origin, ${response.url}` };
    }
    const parsed = parseRegistry(await response.json());
    if (!parsed.ok) return parsed;
    const entries = resolveEntries(parsed.value, requested);
    const outside = outsidePrefix(entries, allowedPrefix);
    if (outside) return { ok: false, error: `registry ${String(url)}: ${outside}` };
    return { ok: true, value: entries };
  } catch (cause) {
    const reason = cause instanceof Error ? cause.message : String(cause);
    return { ok: false, error: `registry ${String(url)}: ${reason}` };
  }
}
