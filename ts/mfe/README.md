# @evinvest/mfe

The **host contract** for EV microfrontends: what a page needs to mount a
platform element (`<ev-sign-in>`, `<ev-account-chip>`, `<ev-wallet>`, …) and
talk to it. The producer side is the `mfe` module of the `ev_lib` Rust crate
(`mfe!`), or a React bundle built to the same contract.

- `.` — server-safe core, no peers: registry manifest, `fetchRegistry`, events,
  `apiVersion`, host-mode attributes, the bundle loader (touches the DOM only
  when called).
- `./react` — `RemoteElement` and `useElementBundle` (a `"use client"` bundle;
  `react` is an optional peer).

## Trust

No hash pins (D20). A host allows the producer's origin in its CSP
(`script-src https://evinvest.ltd`) and loads whatever the manifest names; a
script from there has exactly the trust of the service that serves it. The
registry parser still refuses `javascript:`, `data:` and protocol-relative
URLs, and ignores a leftover `integrity` field.

## Registry

```ts
import { fetchRegistry, findEntry, parseRegistry } from '@evinvest/mfe';

// A producer's published registry; relative scriptUrls resolve against it.
const result = await fetchRegistry('https://evinvest.ltd/auth/mfe/registry.json');
if (!result.ok) return fallback; // expected failure, not a throw
const chip = findEntry(result.value, 'concierge.account-chip');

// Or an in-repo mfe-registry.json, read server-side by the host.
const local = parseRegistry(JSON.parse(raw));
```

An entry is `{ name, tag, scriptUrl, kind: "component" | "page" }`. The Rust
`mfe!` emits the same object without `scriptUrl` (`mfe.json`):
`entryFromManifest(manifest, url)` joins them. Names are `{service}.{name}` for
Rust producers; tags must be valid custom-element names and unique in a
registry.

## Mount (React)

```tsx
'use client';
import { API_VERSION, NAVIGATE_EVENT, SIGNOUT_EVENT, hostAttributes, navigateHref } from '@evinvest/mfe';
import { RemoteElement } from '@evinvest/mfe/react';

<RemoteElement
  tag={chip.tag}
  scriptUrl={chip.scriptUrl}
  minApiVersion={API_VERSION}
  awaitPaint
  attributes={hostAttributes({ name, email, groups })}
  events={{
    [NAVIGATE_EVENT]: event => {
      const href = navigateHref(event);
      if (href && isMyRoute(href)) {
        event.preventDefault();
        router.push(href);
      }
    },
    [SIGNOUT_EVENT]: () => signOut(),
  }}
  fallback={<ChipPlaceholder />}
/>;
```

Light DOM only: host tokens, fonts and Tailwind `@property` registrations do not
cross a shadow root. Attributes are set **before** the element connects, because
a producer reads its configuration in `connectedCallback`. The element reads its
locale from the nearest `[lang]`; to remount on a language change, change the
component's `key`.

Remote stylesheets are moved into the `reamfe` cascade layer
(`containRemoteStyles`). Declare it first in the host's CSS, below every layer of
its own: `@layer reamfe, theme, base, components, utilities;`.

## Events

All bubble and are `composed`.

| Event | `detail` | Cancelable | Meaning |
| --- | --- | --- | --- |
| `ev:navigate` | `{ href }` (path + query + hash) | yes | A plain same-origin link click; `preventDefault()` takes it into the host router |
| `ev:signed-in` | `null` | no | Sign-in finished inside the element; the host starts its own RP login |
| `signout` | `null` | no | Sign out picked in host mode; the host ends its session |

Producers dispatch with `dispatchNavigate` (with `delegatedHref` to decide which
clicks to offer), `dispatchSignedIn`, `dispatchSignOut`.

## apiVersion

`API_VERSION` is `2`. A producer sets `static apiVersion = API_VERSION` on its
element class; a host passes `minApiVersion` and an older bundle reads as
`failed`. Additive changes keep the number; renaming an attribute or event is
version 3.

## Host mode

`data-mode="host"`: the host has its own session and tells the element who is
signed in. `hostAttributes(props)` writes the set; `readHostProps(read)` is the
element's side. Both live here so a value the reader would refuse is never
written. Names: `HOST_ATTRS`.
