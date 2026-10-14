// Host mode (`data-mode="host"`): a host on another origin, with its own
// session, tells the element who is signed in and what to link to, and the
// element fetches nothing. Both sides live here so they cannot drift: the host
// writes with `hostAttributes`, the element reads with `readHostProps`, and a
// value the reader would refuse is never written.
//
// Every attribute is outside input to the element — a host on another release,
// a typo, a stale template — so the reader parses each one, and anything that
// does not parse reads as absent rather than half-rendering.

/**
 * Attribute names. Frozen with apiVersion 2; a rename is apiVersion 3.
 * `intent` and `returnTo` belong to default mode: {@link hostAttributes} does
 * not write them and {@link readHostProps} does not read them. `layout` is
 * written but read by no function here.
 */
export const HOST_ATTRS = {
  mode: 'data-mode',
  name: 'data-name',
  email: 'data-email',
  manageHref: 'data-manage-href',
  profileHref: 'data-profile-href',
  switchHref: 'data-switch-href',
  groups: 'data-groups',
  /** Which auth screen the sign-in flow opens on: `signin` or `signup`. */
  intent: 'data-intent',
  /** Where to come back to after sign-in: an absolute URL on the host, checked by concierge against the scope registry. */
  returnTo: 'data-return-to',
  /** Either mode; `menu` keeps the name below `sm` (a drawer row). CSS only. */
  layout: 'data-layout',
} as const;

export type HostAttributeName = (typeof HOST_ATTRS)[keyof typeof HOST_ATTRS];

/** The attributes a host may set; an absent key is an absent attribute. */
export type HostAttributes = Partial<Record<HostAttributeName, string>>;

/** Every name the reader observes, for `static observedAttributes`. */
export const HOST_ATTRIBUTE_NAMES: readonly HostAttributeName[] = Object.values(HOST_ATTRS);

export interface MenuItem {
  id: string;
  href: string;
  label: string;
  /** Opens in a new tab. */
  external: boolean;
  /** The page the reader is on: `aria-current="page"`. */
  current: boolean;
}

export interface MenuGroup {
  id: string;
  /** Heading above the group, already localised by the host. */
  label: string | null;
  items: MenuItem[];
}

/** What the host writes for a menu: flags default to false, a label to none. */
export interface MenuItemInput {
  id: string;
  href: string;
  label: string;
  external?: boolean;
  current?: boolean;
}

export interface MenuGroupInput {
  id: string;
  label?: string;
  items: MenuItemInput[];
}

/** What {@link readHostProps} returns: a field absent or refused reads as null, `groups` as empty. */
export interface HostProps {
  name: string | null;
  email: string | null;
  manageHref: string | null;
  profileHref: string | null;
  switchHref: string | null;
  groups: MenuGroup[];
}

/** What a host passes to {@link hostAttributes}. */
export interface HostPropsInput {
  name?: string;
  email?: string;
  manageHref?: string;
  profileHref?: string;
  switchHref?: string;
  groups?: MenuGroupInput[];
  layout?: 'menu';
}

// Bounds on what a header menu can sensibly hold, so a runaway attribute cannot
// lay out a thousand rows on every page of the host.
const MAX_GROUPS = 8;
const MAX_ITEMS = 16;
// The longest valid email address; no name or label legitimately runs longer.
const MAX_TEXT = 254;
const MAX_HREF = 2048;
// Only resolves relative hrefs; never fetched.
const PROBE = 'https://h.invalid';

function text(raw: unknown): string | null {
  if (typeof raw !== 'string') return null;
  const value = raw.trim();
  // Code points, as the writer cuts: UTF-16 units would refuse a cut name of astral characters.
  return value && Array.from(value).length <= MAX_TEXT ? value : null;
}

/**
 * An href the element may render: root-relative or absolute http(s). Refused:
 * `javascript:`, `data:`, protocol-relative `//evil`, control characters,
 * backslashes, bare words — the value lands in an `<a href>` on a page that
 * holds a session. Browsers strip tabs and newlines and read `\` as `/`, so
 * `/<TAB>/evil` and `/\evil` are refused before any trimming.
 */
export function safeHref(raw: unknown): string | null {
  if (typeof raw !== 'string') return null;
  if (/[\u0000-\u001F\u007F\\]/.test(raw)) return null;
  const value = raw.trim();
  if (!value || value.length > MAX_HREF) return null;
  try {
    if (value.startsWith('/')) {
      if (value.startsWith('//')) return null;
      return new URL(value, PROBE).origin === PROBE ? value : null;
    }
    const { protocol } = new URL(value);
    return protocol === 'https:' || protocol === 'http:' ? value : null;
  } catch {
    return null;
  }
}

function optionalFlag(source: object, key: string): boolean | null {
  const value: unknown = Reflect.get(source, key);
  if (value === undefined) return false;
  return typeof value === 'boolean' ? value : null;
}

function parseItem(raw: unknown): MenuItem | null {
  if (typeof raw !== 'object' || raw === null) return null;
  const id = text(Reflect.get(raw, 'id'));
  const href = safeHref(Reflect.get(raw, 'href'));
  const label = text(Reflect.get(raw, 'label'));
  const external = optionalFlag(raw, 'external');
  const current = optionalFlag(raw, 'current');
  if (!id || !href || !label || external === null || current === null) return null;
  return { id, href, label, external, current };
}

function parseGroup(raw: unknown): MenuGroup | null {
  if (typeof raw !== 'object' || raw === null) return null;
  const id = text(Reflect.get(raw, 'id'));
  if (!id) return null;
  const rawLabel: unknown = Reflect.get(raw, 'label');
  const label = rawLabel === undefined ? null : text(rawLabel);
  if (rawLabel !== undefined && label === null) return null;
  const rawItems: unknown = Reflect.get(raw, 'items');
  if (!Array.isArray(rawItems) || rawItems.length === 0 || rawItems.length > MAX_ITEMS) return null;
  const items: MenuItem[] = [];
  for (const entry of rawItems) {
    const item = parseItem(entry);
    if (!item) return null;
    items.push(item);
  }
  return { id, label, items };
}

/**
 * `data-groups`: JSON `[{ id, label?, items: [{ id, href, label, external?,
 * current? }] }]`. All or nothing — one malformed entry empties the list, so a
 * host bug shows up as a missing section rather than a menu with holes.
 */
export function parseGroups(raw: string | null): MenuGroup[] {
  if (!raw) return [];
  let value: unknown;
  try {
    value = JSON.parse(raw);
  } catch {
    return [];
  }
  if (!Array.isArray(value) || value.length > MAX_GROUPS) return [];
  const groups: MenuGroup[] = [];
  for (const entry of value) {
    const group = parseGroup(entry);
    if (!group) return [];
    groups.push(group);
  }
  return groups;
}

export type AttributeReader = (name: string) => string | null;

/** Whether `data-mode` is exactly `host`; anything else is default mode. */
export function isHostMode(read: AttributeReader): boolean {
  return read(HOST_ATTRS.mode) === 'host';
}

/** Everything host mode renders, from `el.getAttribute.bind(el)`. */
export function readHostProps(read: AttributeReader): HostProps {
  return {
    name: text(read(HOST_ATTRS.name)),
    email: text(read(HOST_ATTRS.email)),
    manageHref: safeHref(read(HOST_ATTRS.manageHref)),
    profileHref: safeHref(read(HOST_ATTRS.profileHref)),
    switchHref: safeHref(read(HOST_ATTRS.switchHref)),
    groups: parseGroups(read(HOST_ATTRS.groups)),
  };
}

// A name over the limit is cut so it still shows; an email is never cut — a cut
// address is someone else's — so one over the limit is left out instead.
function clip(value: string): string {
  const chars = Array.from(value.trim());
  // Trim after the cut too: the reader trims, and the host must see what renders.
  return chars.length > MAX_TEXT ? chars.slice(0, MAX_TEXT).join('').trim() : chars.join('');
}

// A parsed group back in the wire shape: no label is an absent key, because
// the reader refuses `"label": null`.
function toInput({ id, label, items }: MenuGroup): MenuGroupInput {
  return label === null ? { id, items } : { id, label, items };
}

/**
 * Host side: the host-mode attribute set for `props`. Values the element would
 * refuse are left out here, so the host sees in this record exactly what the
 * element will render.
 */
export function hostAttributes(props: HostPropsInput): HostAttributes {
  const attrs: HostAttributes = { [HOST_ATTRS.mode]: 'host' };
  const name = props.name === undefined ? '' : clip(props.name);
  if (name) attrs[HOST_ATTRS.name] = name;
  const email = text(props.email);
  if (email) attrs[HOST_ATTRS.email] = email;
  const hrefs = [
    [HOST_ATTRS.manageHref, props.manageHref],
    [HOST_ATTRS.profileHref, props.profileHref],
    [HOST_ATTRS.switchHref, props.switchHref],
  ] as const;
  for (const [attr, value] of hrefs) {
    const href = safeHref(value);
    if (href) attrs[attr] = href;
  }
  if (props.groups && props.groups.length > 0) {
    // Written as the reader parses it — trimmed, flags spelled out, unknown
    // fields gone — so the attribute carries nothing the element would not read.
    const groups = parseGroups(JSON.stringify(props.groups));
    if (groups.length > 0) attrs[HOST_ATTRS.groups] = JSON.stringify(groups.map(toInput));
  }
  if (props.layout) attrs[HOST_ATTRS.layout] = props.layout;
  return attrs;
}
