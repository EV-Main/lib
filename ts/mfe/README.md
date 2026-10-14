# @evinvest/mfe

The **host contract** for EV microfrontends: what a page needs to mount a
platform element (`<mfe-concierge-sign-in>`, `<mfe-concierge-account-chip>`,
`<mfe-banking-wallet>`, …) and
talk to it. The producer side is the `mfe` module of the `ev_lib` Rust crate
(`mfe!`), or a React bundle built to the same contract.

- `.` — server-safe core, no peers: registry manifest, `fetchRegistry`, events,
  `apiVersion`, host-mode attributes, the bundle loader (touches the DOM only
  when called).
- `./react` — `RemoteElement`, `useElementBundle` and `navigateListener` (a
  `"use client"` bundle; `react` is an optional peer).

## Trust

No hash pins (D20). A host allows the producer's **bundle directories** in its
CSP — paths, not the bare origin, so nothing else served by evinvest.ltd can
run as a script on the host:

```
script-src 'self' https://evinvest.ltd/api/auth/mfe/ https://evinvest.ltd/wallet/mfe/
```

and loads whatever the manifest names; a script from there has exactly the
trust of the service that serves it. The registry backs the CSP up:

- `fetchRegistry(url, { allowedPrefix })` refuses the whole registry when a
  resolved `scriptUrl` is outside `allowedPrefix` (the same directory the CSP
  names), and refuses redirects (`redirect: 'error'`, and a `response.url` on
  another origin). `allowedPrefix` is required and must be a directory below
  the origin root: without it, or with `https://host/`, the call returns
  `{ ok: false }` — the check would otherwise shrink to comparing origins;
- `isBundleUrl` and `safeHref` accept absolute
  URLs only over `https:` — `http:` only to `localhost`, `127.x.x.x` or `[::1]`
  for a dev stand (`isSecureTransport`);
- the parser refuses `javascript:`, `data:` and protocol-relative URLs, and
  ignores a leftover `integrity` field.

## Registry

```ts
import { fetchRegistry, findEntry, parseRegistry } from '@evinvest/mfe';

// A producer's published registry; relative scriptUrls resolve against it,
// and every one must land under allowedPrefix.
const result = await fetchRegistry('https://evinvest.ltd/api/auth/mfe/registry.json', {
  allowedPrefix: 'https://evinvest.ltd/api/auth/mfe/',
});
if (!result.ok) return fallback; // expected failure, not a throw
const chip = findEntry(result.value, 'concierge.account-chip');

// Or an in-repo mfe-registry.json, read server-side by the host.
const local = parseRegistry(JSON.parse(raw));
```

A host wrapper that knows its element by name and tag takes it with
`entryFor(entries, { name, tag })`, which refuses a missing entry and one whose
tag differs.

An entry is `{ name, tag, scriptUrl, kind: "component" | "page" }`. The Rust
`mfe!` emits the same object without `scriptUrl` (`mfe.json`):
`entryFromManifest(manifest, url)` joins them. Rust producers name an entry
`{service}.{name}` with the tag `mfe-{service}-{name}`, and the platform's React
bundles follow the same scheme; any tag must be a valid custom-element name, and
names and tags are unique in a registry.

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
a producer reads its configuration in `connectedCallback`. A Rust `mfe!` element reads
its locale from the nearest `[lang]` once, at mount; to remount on a language
change, change the component's `key`.

The wrapper carries `ph-no-capture`, so the host's PostHog
neither autocaptures inside it nor records it in session replay: a remote shows
platform data — email, balances, addresses — the host's analytics must not copy
out. `noCapture={false}` turns it off for an element that shows nothing
personal.

An element built with the Rust `mfe!` producer adds `ph-no-capture` to itself
when it connects, so an element appended straight to `<body>` is covered too.
Text the host renders around it is not: a host without React marks its own
container.

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

`navigateHref(event)` reads an `ev:navigate` destination as untrusted input;
`navigateListener(onNavigate)` from `./react` wraps it for `RemoteElement`, with
`onNavigate(href)` returning whether the host took the link. Both sit on
`rootRelative(raw)` — a path on this origin, never `//evil` or `/\evil` — which
is also the rule for an RP-login `return_to`.

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

`HOST_ATTRS` also names two default-mode attributes that neither function
touches: `data-intent` (`signin` | `signup`, the first auth screen) and
`data-return-to` (an absolute URL on the host to come back to after sign-in).
`data-layout="menu"` is written by `hostAttributes` but read only by the
element's CSS.
