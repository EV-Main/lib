// Wallet and PayInvoiceButton take a money action out of the element's click
// only when its href is a wallet action on evinvest.ltd; anything else is left
// to the element. `ev:navigate` details are untrusted: only a root-relative path
// reaches the host's router. Tags are defined up front, so the loader resolves
// without a network; jsdom has no popups, so `window.open` returns a fake.
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import type { WalletActionOutcome } from '../src/index';
import { PayInvoiceButton, Wallet, type NavigateHandler } from '../src/react/index';

const WALLET = 'mfe-banking-wallet';
const PAY = 'mfe-banking-pay-invoice';

interface FakePopup {
  closed: boolean;
  close: () => void;
}

// The one cast in this file: jsdom cannot open a real `Window`; the helper
// reads only `closed` and `close()` and compares the object by identity.
function asWindow(popup: FakePopup): Window {
  return popup as unknown as Window;
}

let container: HTMLDivElement;
let root: Root;

beforeAll(() => {
  customElements.define(WALLET, class extends HTMLElement {
    static apiVersion = 2;
  });
  customElements.define(PAY, class extends HTMLElement {
    static apiVersion = 2;
  });
});

beforeEach(() => {
  container = document.createElement('div');
  document.body.appendChild(container);
  root = createRoot(container);
});

afterEach(() => {
  act(() => root.unmount());
  container.remove();
  vi.restoreAllMocks();
});

async function render(node: ReactNode): Promise<void> {
  await act(async () => {
    root.render(node);
  });
}

function stubOpen() {
  const popup: FakePopup = { closed: false, close: () => {} };
  const open = vi.spyOn(window, 'open').mockReturnValue(asWindow(popup));
  return { popup, open };
}

function walletAction(target: Element | null, href: unknown): CustomEvent {
  const event = new CustomEvent('ev:wallet-action', { bubbles: true, composed: true, cancelable: true, detail: { href } });
  target?.dispatchEvent(event);
  return event;
}

function navigate(target: Element | null, href: unknown): CustomEvent {
  const event = new CustomEvent('ev:navigate', { bubbles: true, composed: true, cancelable: true, detail: { href } });
  target?.dispatchEvent(event);
  return event;
}

describe('Wallet: ev:wallet-action', () => {
  it('should_take_a_wallet_action_into_a_popup_and_report_its_result', async () => {
    const { popup, open } = stubOpen();
    const onResult = vi.fn<(outcome: WalletActionOutcome) => void>();
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" scope="sa" onResult={onResult} />);

    const event = walletAction(container.querySelector(WALLET), 'https://evinvest.ltd/wallet/withdraw');
    window.dispatchEvent(
      new MessageEvent('message', {
        origin: 'https://evinvest.ltd',
        source: asWindow(popup),
        data: { type: 'ev:wallet-result', outcome: 'done', reference: 'payout-1' },
      }),
    );

    expect(event.defaultPrevented).toBe(true);
    expect(open).toHaveBeenCalledTimes(1);
    await vi.waitFor(() => {
      expect(onResult).toHaveBeenCalledWith({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
    });
  });

  it.each([
    ['another origin', 'https://evil.example/wallet/withdraw'],
    ['a path outside wallet', 'https://evinvest.ltd/admin/users'],
    ['a dot-segment escape', 'https://evinvest.ltd/wallet/../admin'],
    ['a relative path', '/wallet/withdraw'],
    ['not a string', 42],
  ])('should_leave_an_action_to_the_element_when_its_href_is_%s', async (_label, href) => {
    const { open } = stubOpen();
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" />);

    const event = walletAction(container.querySelector(WALLET), href);

    expect(event.defaultPrevented).toBe(false);
    expect(open).not.toHaveBeenCalled();
  });

  it('should_refuse_a_production_href_when_a_staging_origin_is_set', async () => {
    const { open } = stubOpen();
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" origin="https://stage.evinvest.ltd" />);

    const event = walletAction(container.querySelector(WALLET), 'https://evinvest.ltd/wallet/withdraw');

    expect(event.defaultPrevented).toBe(false);
    expect(open).not.toHaveBeenCalled();
  });

  it('should_take_a_staging_action_and_its_staging_result_when_a_staging_origin_is_set', async () => {
    const { popup, open } = stubOpen();
    const onResult = vi.fn<(outcome: WalletActionOutcome) => void>();
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" origin="https://stage.evinvest.ltd" onResult={onResult} />);

    walletAction(container.querySelector(WALLET), 'https://stage.evinvest.ltd/wallet/withdraw');
    window.dispatchEvent(
      new MessageEvent('message', {
        origin: 'https://stage.evinvest.ltd',
        source: asWindow(popup),
        data: { type: 'ev:wallet-result', outcome: 'failed' },
      }),
    );

    expect(open).toHaveBeenCalledTimes(1);
    await vi.waitFor(() => {
      expect(onResult).toHaveBeenCalledWith({ status: 'result', result: { outcome: 'failed', reference: null } });
    });
  });

  it('should_set_the_scope_attribute', async () => {
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" scope="sa" />);

    expect(container.querySelector(WALLET)?.getAttribute('scope')).toBe('sa');
  });
});

describe('Wallet: ev:navigate', () => {
  it('should_hand_a_valid_path_to_onNavigate_and_cancel_the_click_when_taken', async () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" onNavigate={onNavigate} />);

    const event = navigate(container.querySelector(WALLET), '/wallet/history?page=2');

    expect(onNavigate).toHaveBeenCalledWith('/wallet/history?page=2');
    expect(event.defaultPrevented).toBe(true);
  });

  it.each([
    ['protocol-relative', '//evil.example'],
    ['backslash', '/\\evil.example'],
    ['absolute', 'https://evil.example/'],
    ['not a string', null],
  ])('should_not_call_onNavigate_for_a_%s_href', async (_label, href) => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<Wallet scriptUrl="/wallet/mfe/wallet.js" onNavigate={onNavigate} />);

    const event = navigate(container.querySelector(WALLET), href);

    expect(onNavigate).not.toHaveBeenCalled();
    expect(event.defaultPrevented).toBe(false);
  });
});

describe('PayInvoiceButton', () => {
  it('should_set_the_invoice_and_scope_attributes', async () => {
    await render(<PayInvoiceButton scriptUrl="/wallet/mfe/pay.js" invoice="inv-1" scope="sa" onResult={() => {}} />);

    const button = container.querySelector(PAY);
    expect(button?.getAttribute('invoice')).toBe('inv-1');
    expect(button?.getAttribute('scope')).toBe('sa');
  });

  it('should_take_a_wallet_action_into_a_popup', async () => {
    const { popup, open } = stubOpen();
    const onResult = vi.fn<(outcome: WalletActionOutcome) => void>();
    await render(<PayInvoiceButton scriptUrl="/wallet/mfe/pay.js" invoice="inv-1" onResult={onResult} />);

    const event = walletAction(container.querySelector(PAY), 'https://evinvest.ltd/wallet/pay?invoice=inv-1');
    window.dispatchEvent(
      new MessageEvent('message', {
        origin: 'https://evinvest.ltd',
        source: asWindow(popup),
        data: { type: 'ev:wallet-result', outcome: 'cancelled' },
      }),
    );

    expect(event.defaultPrevented).toBe(true);
    expect(open).toHaveBeenCalledTimes(1);
    await vi.waitFor(() => {
      expect(onResult).toHaveBeenCalledWith({ status: 'result', result: { outcome: 'cancelled', reference: null } });
    });
  });

  it('should_leave_an_action_to_the_element_when_its_href_is_not_a_wallet_action', async () => {
    const { open } = stubOpen();
    await render(<PayInvoiceButton scriptUrl="/wallet/mfe/pay.js" invoice="inv-1" onResult={() => {}} />);

    const event = walletAction(container.querySelector(PAY), 'https://evil.example/wallet/pay');

    expect(event.defaultPrevented).toBe(false);
    expect(open).not.toHaveBeenCalled();
  });
});
