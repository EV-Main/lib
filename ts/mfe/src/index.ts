/**
 * @module @evinvest/mfe
 *
 * The host contract for EV microfrontends: the registry manifest, the bundle
 * loader, the events an element dispatches, `apiVersion`, and host-mode
 * attributes. Server-safe: nothing touches the DOM at import time. The React
 * `RemoteElement` lives in `./react`.
 *
 * Producer twin: the `mfe` module of the `ev_lib` Rust crate (`mfe!` emits the
 * `{name, tag, kind}` manifest this package parses).
 */

export {
  entryFor,
  entryFromManifest,
  fetchRegistry,
  findEntry,
  isBundleUrl,
  MFE_KINDS,
  parseEntry,
  parseManifest,
  parseRegistry,
  resolveEntries,
} from './manifest';
export type { MfeEntry, MfeKind, MfeManifest, ParseResult } from './manifest';

export {
  API_VERSION,
  delegatedHref,
  dispatchNavigate,
  dispatchSignedIn,
  dispatchSignOut,
  NAVIGATE_EVENT,
  navigateHref,
  rootRelative,
  SIGNED_IN_EVENT,
  SIGNOUT_EVENT,
  speaksApi,
} from './events';
export type { LinkAnchor, LinkClick, MfeEventMap, MfeEventName, NavigateDetail } from './events';

export {
  HOST_ATTRIBUTE_NAMES,
  HOST_ATTRS,
  hostAttributes,
  isHostMode,
  parseGroups,
  readHostProps,
  safeHref,
} from './host-attributes';
export type {
  AttributeReader,
  HostAttributeName,
  HostAttributes,
  HostProps,
  HostPropsInput,
  MenuGroup,
  MenuGroupInput,
  MenuItem,
  MenuItemInput,
} from './host-attributes';

export { awaitElement, containRemoteStyles, loadElement, REMOTE_LAYER } from './loader';
export type { ElementRegistry, LoadOptions } from './loader';
