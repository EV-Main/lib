// AccountChip and AccountPage: host mode is set by attributes, and an
// `ev:navigate` detail from the element is untrusted — only a root-relative
// path reaches the host's router. Tags are defined up front, so the loader
// resolves without a network.
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import { AccountChip, AccountPage, type NavigateHandler } from '../src/react/index';

const CHIP = 'mfe-concierge-account-chip';
const PAGE = 'mfe-concierge-account';

let container: HTMLDivElement;
let root: Root;

beforeAll(() => {
  customElements.define(CHIP, class extends HTMLElement {
    static apiVersion = 2;
  });
  customElements.define(PAGE, class extends HTMLElement {
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
});

async function render(node: ReactNode): Promise<void> {
  await act(async () => {
    root.render(node);
  });
}

function navigate(target: Element | null, href: unknown): CustomEvent {
  const event = new CustomEvent('ev:navigate', { bubbles: true, composed: true, cancelable: true, detail: { href } });
  target?.dispatchEvent(event);
  return event;
}

describe('AccountChip attributes', () => {
  it('should_set_data_mode_host_and_the_host_props_when_host_is_given', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'Valentina', email: 'v@example.com' }} />);

    const chip = container.querySelector(CHIP);
    expect(chip?.getAttribute('data-mode')).toBe('host');
    expect(chip?.getAttribute('data-name')).toBe('Valentina');
    expect(chip?.getAttribute('data-email')).toBe('v@example.com');
  });

  it('should_not_set_data_mode_without_host', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" intent="signin" returnTo="https://invest.evinvest.ltd/funds" />);

    const chip = container.querySelector(CHIP);
    expect(chip?.hasAttribute('data-mode')).toBe(false);
    expect(chip?.getAttribute('data-intent')).toBe('signin');
    expect(chip?.getAttribute('data-return-to')).toBe('https://invest.evinvest.ltd/funds');
  });

  it('should_pass_the_signup_intent', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" intent="signup" />);

    expect(container.querySelector(CHIP)?.getAttribute('data-intent')).toBe('signup');
  });

  it('should_drop_a_root_relative_return_to', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" returnTo="/funds" />);

    expect(container.querySelector(CHIP)?.hasAttribute('data-return-to')).toBe(false);
  });

  it('should_not_default_return_to_to_the_current_page', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" />);

    expect(container.querySelector(CHIP)?.hasAttribute('data-return-to')).toBe(false);
  });

  it('should_drop_a_return_to_that_leaves_the_origin', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" returnTo="//evil.example" />);

    expect(container.querySelector(CHIP)?.hasAttribute('data-return-to')).toBe(false);
  });
});

describe('AccountChip events', () => {
  it('should_hand_a_valid_path_to_onNavigate_and_cancel_the_click_when_taken', async () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" onNavigate={onNavigate} />);

    const event = navigate(container.querySelector(CHIP), '/account?tab=security');

    expect(onNavigate).toHaveBeenCalledWith('/account?tab=security');
    expect(event.defaultPrevented).toBe(true);
  });

  it('should_leave_the_click_to_the_browser_when_onNavigate_declines', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" onNavigate={() => false} />);

    const event = navigate(container.querySelector(CHIP), '/account');

    expect(event.defaultPrevented).toBe(false);
  });

  it.each([
    ['protocol-relative', '//evil.example'],
    ['backslash', '/\\evil.example'],
    ['absolute', 'https://evil.example/x'],
    ['javascript', 'javascript:alert(1)'],
    ['not a string', 42],
  ])('should_not_call_onNavigate_for_a_%s_href', async (_label, href) => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" onNavigate={onNavigate} />);

    const event = navigate(container.querySelector(CHIP), href);

    expect(onNavigate).not.toHaveBeenCalled();
    expect(event.defaultPrevented).toBe(false);
  });

  it('should_call_onSignOut_on_signout', async () => {
    const onSignOut = vi.fn<() => void>();
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} onSignOut={onSignOut} />);

    container.querySelector(CHIP)?.dispatchEvent(new CustomEvent('signout', { bubbles: true, composed: true, detail: null }));

    expect(onSignOut).toHaveBeenCalledTimes(1);
  });
});

describe('AccountPage events', () => {
  it('should_hand_a_valid_path_to_onNavigate', async () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<AccountPage scriptUrl="/auth/mfe/account.js" onNavigate={onNavigate} />);

    const event = navigate(container.querySelector(PAGE), '/funds');

    expect(onNavigate).toHaveBeenCalledWith('/funds');
    expect(event.defaultPrevented).toBe(true);
  });

  it('should_not_call_onNavigate_for_a_protocol_relative_href', async () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    await render(<AccountPage scriptUrl="/auth/mfe/account.js" onNavigate={onNavigate} />);

    navigate(container.querySelector(PAGE), '//evil.example');

    expect(onNavigate).not.toHaveBeenCalled();
  });

  it('should_call_onSignOut_on_signout', async () => {
    const onSignOut = vi.fn<() => void>();
    await render(<AccountPage scriptUrl="/auth/mfe/account.js" onSignOut={onSignOut} />);

    container.querySelector(PAGE)?.dispatchEvent(new CustomEvent('signout', { bubbles: true, composed: true, detail: null }));

    expect(onSignOut).toHaveBeenCalledTimes(1);
  });
});
