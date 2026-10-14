# @evinvest/wallet

Host wrappers for the **banking wallet microfrontends** — the wallet and the
pay-invoice button — over [`@evinvest/mfe`](../mfe), plus the popup round trip
that carries a money action to evinvest.ltd and brings its result back. Hosts:
the invest cabinet, the SA panel, admin. The elements are banking's, served from
`https://evinvest.ltd/wallet/mfe/`.

- `.` — server-safe core: registry constants, tags, `openWalletAction` and its
  URL and message checks (they touch `window` only when called).
- `./react` — `Wallet`, `PayInvoiceButton` (a `"use client"` bundle; `react` is
  an optional peer).

`@evinvest/mfe` is a **peer**, shared with the host so each element loads once.

## Elements

| Element | Tag | Registry name |
| --- | --- | --- |
| Wallet | `mfe-banking-wallet` | `banking.wallet` |
| Pay invoice | `mfe-banking-pay-invoice` | `banking.pay-invoice` |

```ts
import { fetchWalletRegistry, walletEntry } from '@evinvest/wallet';

const registry = await fetchWalletRegistry(); // WALLET_REGISTRY_URL by default
if (!registry.ok) return fallback;
const walletResult = walletEntry(registry.value, 'wallet');
const payResult = walletEntry(registry.value, 'payInvoice');
if (!walletResult.ok || !payResult.ok) return fallback;
const wallet = walletResult.value;
const pay = payResult.value;
```

## Money actions stay on evinvest.ltd

With `scope` (a scope host) the wallet shows balances and history only.
Without it the wallet is mounted on evinvest.ltd itself and the element carries
out its actions in place; the host does not intercept them. A withdrawal,
transfer, payment or mandate happens on `https://evinvest.ltd/wallet/…`: in a
popup, whose page posts the result back to its opener, or — when the popup is
blocked — as a full navigation that returns to the host. Banking's mutating
routes accept only `Origin: https://evinvest.ltd`, so nothing on a host can move
money by itself.

```tsx
'use client';
import { PayInvoiceButton, Wallet } from '@evinvest/wallet/react';

<Wallet scriptUrl={wallet.scriptUrl} scope="sa" onResult={() => refresh()} />;

<PayInvoiceButton
  scriptUrl={pay.scriptUrl}
  invoice={invoiceId}
  scope="sa"
  onResult={outcome => {
    if (outcome.status === 'result' && outcome.result.outcome === 'done') markPaid(outcome.result.reference);
  }}
/>;
```

Contract, host side:

| | Name | Meaning |
| --- | --- | --- |
| attribute | `scope` | the scope host's id; absent = the full wallet, actions in place |
| attribute | `invoice` | the invoice id (pay-invoice only) |
| event | `ev:wallet-action` | cancelable, `detail: { href }` under `https://evinvest.ltd/wallet/`; the host opens it |
| event | `ev:navigate` | `Wallet` only: an in-element link, as in `@evinvest/mfe` (`onNavigate`) |
| query | `opener` | the host's origin, added to the popup URL — where the page posts |
| query | `return_to` | where the page sends the user back when navigated, not popped up |
| message | `{ type: 'ev:wallet-result', outcome: 'done' \| 'failed' \| 'cancelled', reference? }` | posted to the opener; a reference that is empty, not a string or over 128 characters reads as none — never cut — and the outcome stands |

### `openWalletAction(url, options?)`

Opens the popup — one window named `ev-wallet-action` per page — and resolves
how it ended — never rejects:
`result` · `closed` (closed before reporting; settled one macrotask after the
poll sees it closed, so a result posted just before closing still wins) ·
`blocked` (use `walletNavigationHref`) · `busy` (another action's popup is open
on this page: it is focused, and this call reports nothing) · `timeout`
(default 15 min; the popup is closed) · `aborted` (`signal`) · `invalid_url`
(not under `https://evinvest.ltd/wallet/`). The React wrappers ignore `busy`,
so a double click gives one popup and one `onResult`.
A message counts only if `event.origin` is `https://evinvest.ltd` **and** its
source is this popup. Call it inside the user's click, or the browser blocks the
popup. `origin` is overridable for a staging stand only.

## Develop

```sh
(cd ../mfe && npm ci && npm run build)  # the peer is linked from ../mfe
npm install
npm run typecheck && npm test && npm run build
```
