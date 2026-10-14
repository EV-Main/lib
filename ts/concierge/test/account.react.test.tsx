// AccountChip and AccountPage: host mode is set by attributes, and an
// `ev:navigate` detail from the element is untrusted — only a root-relative
// path reaches the host's router. Sign-out ends the host's session and then the
// platform's, which a host cannot end itself: left alive, it signs the next
// visitor of a shared browser straight back in. Tags are defined up front, so
// the loader resolves without a network.
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
  vi.unstubAllGlobals();
});

async function render(node: ReactNode): Promise<void> {
  await act(async () => {
    root.render(node);
  });
}

const HOST_PAGE = 'https://invest.evinvest.ltd/funds?token=secret#row-7';
// The host's root, not the page: concierge returns only to a path in the
// scope's `return_paths`, and `/` is the one every scope with the chip lists.
const PLATFORM_SIGN_OUT = 'https://evinvest.ltd/api/auth/logout/confirm?return_to=https%3A%2F%2Finvest.evinvest.ltd%2F';

/**
 * jsdom cannot navigate to another origin, so `location` is swapped for a
 * record of where the page was sent. Stubbed after render: the loader resolves
 * bundle URLs against the real one.
 */
function stubLocation(href: string) {
  const url = new URL(href);
  const assign = vi.fn<(to: string) => void>();
  vi.stubGlobal('location', { href: url.href, origin: url.origin, pathname: url.pathname, search: url.search, hash: url.hash, assign });
  return assign;
}

type RejectionHandler = (reason: unknown) => void;

/** The slice of Node's `process` the rejection test needs; the package ships no Node types. */
interface RejectionEvents {
  listeners(event: 'unhandledRejection'): RejectionHandler[];
  removeAllListeners(event: 'unhandledRejection'): void;
  on(event: 'unhandledRejection', handler: RejectionHandler): void;
  once(event: 'unhandledRejection', handler: RejectionHandler): void;
}

const nodeProcess: RejectionEvents = Reflect.get(globalThis, 'process');

function signOut(target: Element | null): void {
  target?.dispatchEvent(new CustomEvent('signout', { bubbles: true, composed: true, detail: null }));
}

function deferred(): { promise: Promise<void>; resolve: () => void } {
  let resolve = () => {};
  const promise = new Promise<void>(done => {
    resolve = done;
  });
  return { promise, resolve };
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

  it('should_default_return_to_to_the_current_page_without_query_or_hash', async () => {
    stubLocation(HOST_PAGE);

    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" />);

    expect(container.querySelector(CHIP)?.getAttribute('data-return-to')).toBe('https://invest.evinvest.ltd/funds');
  });

  it('should_cut_an_explicit_return_to_to_origin_and_path', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" returnTo="https://user:pass@invest.evinvest.ltd/funds?token=secret#row-7" />);

    expect(container.querySelector(CHIP)?.getAttribute('data-return-to')).toBe('https://invest.evinvest.ltd/funds');
  });

  it('should_not_set_return_to_in_host_mode', async () => {
    stubLocation(HOST_PAGE);

    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} />);

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

  it('should_call_onSignOut_then_send_the_page_to_the_platform_sign_out_back_to_the_host_root', async () => {
    const order: string[] = [];
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} onSignOut={() => void order.push('host')} />);
    const assign = stubLocation(HOST_PAGE);
    assign.mockImplementation(() => void order.push('platform'));

    signOut(container.querySelector(CHIP));

    await vi.waitFor(() => expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT));
    expect(order).toEqual(['host', 'platform']);
  });

  it('should_wait_for_an_async_onSignOut_before_leaving_the_page', async () => {
    const hostDone = deferred();
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} onSignOut={() => hostDone.promise} />);
    const assign = stubLocation(HOST_PAGE);

    signOut(container.querySelector(CHIP));
    await Promise.resolve();

    expect(assign).not.toHaveBeenCalled();
    hostDone.resolve();
    await vi.waitFor(() => expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT));
  });

  it('should_still_end_the_platform_session_when_onSignOut_rejects', async () => {
    // The listener lets the rejection reach the page's error handlers on
    // purpose; vitest's own handler would fail the run, so it steps aside for
    // this one test and the rejection is checked here instead.
    const vitestHandlers = nodeProcess.listeners('unhandledRejection');
    nodeProcess.removeAllListeners('unhandledRejection');
    const surfaced = new Promise<unknown>(resolve => nodeProcess.once('unhandledRejection', resolve));
    try {
      const failure = new Error('host sign-out failed');
      await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} onSignOut={() => Promise.reject(failure)} />);
      const assign = stubLocation(HOST_PAGE);

      signOut(container.querySelector(CHIP));

      await expect(surfaced).resolves.toBe(failure);
      expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT);
    } finally {
      for (const handler of vitestHandlers) nodeProcess.on('unhandledRejection', handler);
    }
  });

  it('should_still_end_the_platform_session_when_onSignOut_throws_synchronously', async () => {
    // A synchronous throw surfaces as the same unhandled rejection as above.
    const vitestHandlers = nodeProcess.listeners('unhandledRejection');
    nodeProcess.removeAllListeners('unhandledRejection');
    const surfaced = new Promise<unknown>(resolve => nodeProcess.once('unhandledRejection', resolve));
    try {
      const failure = new Error('host sign-out failed');
      await render(
        <AccountChip
          scriptUrl="/auth/mfe/chip.js"
          host={{ name: 'V' }}
          onSignOut={() => {
            throw failure;
          }}
        />,
      );
      const assign = stubLocation(HOST_PAGE);

      signOut(container.querySelector(CHIP));

      await expect(surfaced).resolves.toBe(failure);
      expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT);
    } finally {
      for (const handler of vitestHandlers) nodeProcess.on('unhandledRejection', handler);
    }
  });

  it('should_end_the_platform_session_without_an_onSignOut', async () => {
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} />);
    const assign = stubLocation(HOST_PAGE);

    signOut(container.querySelector(CHIP));

    await vi.waitFor(() => expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT));
  });

  it('should_stay_on_the_page_when_platformSignOut_is_false', async () => {
    const hostDone = Promise.resolve();
    const onSignOut = vi.fn(() => hostDone);
    await render(<AccountChip scriptUrl="/auth/mfe/chip.js" host={{ name: 'V' }} onSignOut={onSignOut} platformSignOut={false} />);
    const assign = stubLocation(HOST_PAGE);

    signOut(container.querySelector(CHIP));
    // The listener chains on the host's promise before this await does.
    await hostDone;

    expect(onSignOut).toHaveBeenCalledTimes(1);
    expect(assign).not.toHaveBeenCalled();
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

  it('should_call_onSignOut_then_send_the_page_to_the_platform_sign_out', async () => {
    const onSignOut = vi.fn<() => void>();
    await render(<AccountPage scriptUrl="/auth/mfe/account.js" onSignOut={onSignOut} />);
    const assign = stubLocation(HOST_PAGE);

    signOut(container.querySelector(PAGE));

    await vi.waitFor(() => expect(assign).toHaveBeenCalledWith(PLATFORM_SIGN_OUT));
    expect(onSignOut).toHaveBeenCalledTimes(1);
  });

  it('should_stay_on_the_page_when_platformSignOut_is_false', async () => {
    const hostDone = Promise.resolve();
    const onSignOut = vi.fn(() => hostDone);
    await render(<AccountPage scriptUrl="/auth/mfe/account.js" onSignOut={onSignOut} platformSignOut={false} />);
    const assign = stubLocation(HOST_PAGE);

    signOut(container.querySelector(PAGE));
    await hostDone;

    expect(onSignOut).toHaveBeenCalledTimes(1);
    expect(assign).not.toHaveBeenCalled();
  });
});
