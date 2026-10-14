// The registry contract: which microfrontends a host may mount and where their
// bundles live. A producer's build emits `{name, tag, kind}` (the Rust `mfe!`
// macro's `MFE_MANIFEST`, shipped as `mfe.json`); the host — or the producer's
// published manifest — adds `scriptUrl`.
//
// Trust is the host's CSP (`script-src`), not a hash pinned here (D20): a pin
// had to be re-taken by hand after every producer release. The registry still
// refuses URLs no CSP should ever have to judge (`javascript:`, `data:`,
// protocol-relative), because a bad entry is easier to read here than in a
// console full of CSP reports.

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
  /** Root-relative (`/cabinet/mfe/x.js`) or absolute http(s) URL of the bundle. */
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
 * Whether `url` may be a bundle URL: root-relative, or absolute http(s). Tabs,
 * newlines and backslashes are refused outright — browsers strip the first two
 * and read `\` as `/`, so `/\evil` would load from another host.
 */
export function isBundleUrl(url: string): boolean {
  if (url.length === 0 || url.length > MAX_URL) return false;
  if (/[\u0000-\u001F\u007F\\\s]/.test(url)) return false;
  if (url.startsWith('/')) return !url.startsWith('//');
  try {
    const { protocol } = new URL(url);
    return protocol === 'https:' || protocol === 'http:';
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
    return { ok: false, error: `${manifest.value.name}: \`scriptUrl\` is not a root-relative or http(s) URL` };
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

export function findEntry(entries: readonly MfeEntry[], name: string): MfeEntry | undefined {
  return entries.find(entry => entry.name === name);
}

/**
 * Resolve every root-relative `scriptUrl` against `base` — the URL the registry
 * was fetched from. A producer publishes `/auth/mfe/x.js` meaning *its* origin;
 * left relative, a host on another origin would load it from itself.
 */
export function resolveEntries(entries: readonly MfeEntry[], base: string | URL): MfeEntry[] {
  return entries.map(entry => ({ ...entry, scriptUrl: new URL(entry.scriptUrl, base).href }));
}

/**
 * Fetch and validate a producer's published registry, with its relative URLs
 * resolved against the URL the registry was finally served from (after
 * redirects). Network, HTTP and URL failures are results, not throws: a host
 * renders its fallback either way.
 */
export async function fetchRegistry(url: string | URL, init?: RequestInit): Promise<ParseResult<MfeEntry[]>> {
  try {
    const response = await fetch(url, init);
    if (!response.ok) return { ok: false, error: `registry ${String(url)}: HTTP ${response.status}` };
    const parsed = parseRegistry(await response.json());
    if (!parsed.ok) return parsed;
    // `response.url` is empty for a hand-built Response; the request URL is
    // then the best base there is, and a relative one throws into the catch.
    return { ok: true, value: resolveEntries(parsed.value, response.url || url) };
  } catch (cause) {
    const reason = cause instanceof Error ? cause.message : String(cause);
    return { ok: false, error: `registry ${String(url)}: ${reason}` };
  }
}
