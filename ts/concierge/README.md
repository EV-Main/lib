# @evinvest/concierge

Host wrappers for the **concierge microfrontends** — the sign-in dialog, the
account page and the account chip — over [`@evinvest/mfe`](../mfe). A host
(invest cabinet, the SA panel, admin) mounts them; the elements themselves are
built in concierge's `web/` and served from `https://evinvest.ltd/api/auth/mfe/` (the site's `/api/auth/*` rewrite to concierge).

- `.` — server-safe core: registry constants, tags, the session shape, the
  RP-login href.
- `./react` — `SignInDialog`, `AccountPage`, `AccountChip`, `useSignInRequired`
  (a `"use client"` bundle; `react` is an optional peer).

`@evinvest/mfe` is a **peer**: its loader caches loads per tag, so the host and
this package must share one copy.

No network calls beyond the registry and the session; the elements do the rest.

## Elements

| Element | Tag | Registry name |
| --- | --- | --- |
| Sign-in dialog | `mfe-concierge-sign-in` | `concierge.sign-in` |
| Account page | `mfe-concierge-account` | `concierge.account` |
| Account chip | `mfe-concierge-account-chip` | `concierge.account-chip` |

```ts
import { conciergeEntry, fetchConciergeRegistry } from '@evinvest/concierge';

const registry = await fetchConciergeRegistry(); // CONCIERGE_REGISTRY_URL by default
if (!registry.ok) return fallback;
const signIn = conciergeEntry(registry.value, 'signIn'); // refused if missing or another tag
```

The host's CSP allows `script-src https://evinvest.ltd` (no hash pins, D20).

## Sign in when the host gets a 401

```tsx
'use client';
import { SignInDialog, useSignInRequired } from '@evinvest/concierge/react';

const { guardResponse, dialogProps } = useSignInRequired();
// …
const response = await fetch('/api/thing');
if (guardResponse(response)) return; // a 401 opened the dialog

<SignInDialog scriptUrl={signIn.scriptUrl} {...dialogProps} />; // returnTo: this page by default
```

The element draws its own backdrop and traps focus; the host only drives
`open`. The dialog is mounted on the first open and kept, so a signed-in visitor
never loads the bundle. When the element reports `ev:signed-in`, the dialog
closes and `onSignedIn` runs — by default a full navigation to the host's RP
login, `/auth/login?return_to=<this page>` (`rpLoginHref`), which completes
silently because the evinvest.ltd session now exists. Pass
`useSignInRequired({ onSignedIn })` for another login path.

Element contract, host side:

| | Name | Meaning |
| --- | --- | --- |
| attribute | `open` | present while the dialog is open |
| attribute | `data-intent` | `signin` or `signup`, the first screen |
| attribute | `data-return-to` | absolute http(s) URL on the host an OAuth round trip lands on; concierge accepts it only for an origin in a scope's registry. Relative values are left out (`hostReturnTo`) |
| event | `ev:signed-in` | sign-in finished |
| event | `ev:close` | dismissed by the user (`CLOSE_EVENT`) |

## Account page and chip

```tsx
<AccountPage scriptUrl={account.scriptUrl} onNavigate={href => (router.push(href), true)} onSignOut={signOut} />

<AccountChip
  scriptUrl={chip.scriptUrl}
  host={{ name, email, groups }} // host mode; omit for default mode
  onNavigate={href => isMyRoute(href) && (router.push(href), true)}
  onSignOut={signOut}
  fallback={<ChipPlaceholder />}
/>
```

`onNavigate` (`NavigateHandler`, from `@evinvest/mfe/react`) gets an already
validated root-relative path and returns whether the host took it. In default
mode the chip takes the same `intent` (`signin` | `signup`) and an absolute
`returnTo` as the dialog. The chip keeps `apiVersion` 2, `ev:navigate` and `signout`;
host-mode attributes come from `hostAttributes` in `@evinvest/mfe`.

## Session

```ts
import { fetchSession } from '@evinvest/concierge';

const session = await fetchSession(); // GET https://evinvest.ltd/api/auth/session, credentials: include
if (session.ok && session.value.authenticated) session.value.user.email;
```

`{ authenticated, user, permissions }`, as concierge's `SessionInfo`. Browser
only in practice: the session cookie is first-party to evinvest.ltd and never
reaches a host's server. It says who holds the evinvest.ltd session, not what
the host's backend lets them do — that stays with the host's RP session.

## Develop

```sh
(cd ../mfe && npm ci && npm run build)  # the peer is linked from ../mfe
npm install
npm run typecheck && npm test && npm run build
```
