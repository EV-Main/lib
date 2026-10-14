// SignInDialog mounts the concierge sign-in element on the first open and keeps
// it, so a signed-in visitor never loads the bundle and a reopen does not
// reload it. The tag is defined up front, so the loader resolves without a
// network (jsdom never fetches the injected script).
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';

import { SignInDialog } from '../src/react/index';

const TAG = 'mfe-concierge-sign-in';
const SCRIPT = '/auth/mfe/sign-in.js';

let container: HTMLDivElement;
let root: Root;
let connects = 0;

beforeAll(() => {
  customElements.define(
    TAG,
    class extends HTMLElement {
      static apiVersion = 2;
      connectedCallback() {
        connects += 1;
      }
    },
  );
});

beforeEach(() => {
  connects = 0;
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

function dialog(open: boolean, handlers: { onOpenChange?: (open: boolean) => void; onSignedIn?: () => void } = {}) {
  return (
    <SignInDialog scriptUrl={SCRIPT} open={open} onOpenChange={handlers.onOpenChange ?? (() => {})} onSignedIn={handlers.onSignedIn ?? (() => {})} />
  );
}

function element(): HTMLElement | null {
  return container.querySelector(TAG);
}

describe('SignInDialog mounting', () => {
  it('should_render_nothing_while_open_has_never_been_true', async () => {
    await render(dialog(false));
    await render(dialog(false));

    expect(container.childElementCount).toBe(0);
    expect(document.querySelector(`script[data-mfe="${TAG}"]`)).toBeNull();
  });

  it('should_mount_the_element_with_the_open_attribute_on_the_first_open', async () => {
    await render(dialog(false));

    await render(dialog(true));

    expect(element()?.hasAttribute('open')).toBe(true);
  });

  it('should_keep_the_same_element_after_closing_and_drop_the_open_attribute', async () => {
    await render(dialog(true));
    const mounted = element();

    await render(dialog(false));

    expect(element()).toBe(mounted);
    expect(element()?.hasAttribute('open')).toBe(false);
  });

  it('should_reuse_the_element_on_a_second_open', async () => {
    await render(dialog(true));
    await render(dialog(false));

    await render(dialog(true));

    expect(connects).toBe(1);
    expect(element()?.hasAttribute('open')).toBe(true);
  });

  it('should_pass_intent_and_a_safe_return_to', async () => {
    await render(
      <SignInDialog scriptUrl={SCRIPT} open intent="signup" returnTo="https://invest.evinvest.ltd/funds" onOpenChange={() => {}} onSignedIn={() => {}} />,
    );

    expect(element()?.getAttribute('data-intent')).toBe('signup');
    expect(element()?.getAttribute('data-return-to')).toBe('https://invest.evinvest.ltd/funds');
  });

  it('should_drop_a_javascript_return_to', async () => {
    await render(<SignInDialog scriptUrl={SCRIPT} open returnTo="javascript:alert(1)" onOpenChange={() => {}} onSignedIn={() => {}} />);

    expect(element()?.hasAttribute('data-return-to')).toBe(false);
  });

  it('should_drop_a_root_relative_return_to_instead_of_falling_back_to_the_page', async () => {
    await render(<SignInDialog scriptUrl={SCRIPT} open returnTo="/funds" onOpenChange={() => {}} onSignedIn={() => {}} />);

    expect(element()?.hasAttribute('data-return-to')).toBe(false);
  });

  it('should_return_to_the_current_page_when_no_return_to_is_given', async () => {
    await render(dialog(true));

    expect(element()?.getAttribute('data-return-to')).toBe('http://localhost:3000/');
  });

  it('should_leave_the_query_and_hash_of_the_current_page_behind', async () => {
    window.history.replaceState(null, '', '/funds?token=secret#row-7');
    try {
      await render(dialog(true));

      expect(element()?.getAttribute('data-return-to')).toBe('http://localhost:3000/funds');
    } finally {
      window.history.replaceState(null, '', '/');
    }
  });

  it('should_cut_an_explicit_return_to_to_origin_and_path', async () => {
    // Concierge answers the OAuth start with 400 for a returnTo carrying `?` or `#`.
    await render(
      <SignInDialog scriptUrl={SCRIPT} open returnTo="https://invest.evinvest.ltd/funds?x=1#top" onOpenChange={() => {}} onSignedIn={() => {}} />,
    );

    expect(element()?.getAttribute('data-return-to')).toBe('https://invest.evinvest.ltd/funds');
  });

  it('should_drop_a_plain_http_return_to_on_a_real_host', async () => {
    await render(<SignInDialog scriptUrl={SCRIPT} open returnTo="http://invest.evinvest.ltd/funds" onOpenChange={() => {}} onSignedIn={() => {}} />);

    expect(element()?.hasAttribute('data-return-to')).toBe(false);
  });
});

describe('SignInDialog events', () => {
  it('should_close_before_starting_the_rp_login_on_signed_in', async () => {
    const calls: string[] = [];
    await render(
      dialog(true, {
        onOpenChange: open => calls.push(`onOpenChange(${open})`),
        onSignedIn: () => calls.push('onSignedIn'),
      }),
    );

    element()?.dispatchEvent(new CustomEvent('ev:signed-in', { bubbles: true, composed: true, detail: null }));

    expect(calls).toEqual(['onOpenChange(false)', 'onSignedIn']);
  });

  it('should_ask_to_close_on_ev_close_without_signing_in', async () => {
    const onOpenChange = vi.fn<(open: boolean) => void>();
    const onSignedIn = vi.fn<() => void>();
    await render(dialog(true, { onOpenChange, onSignedIn }));

    element()?.dispatchEvent(new CustomEvent('ev:close', { bubbles: true, composed: true }));

    expect(onOpenChange).toHaveBeenCalledWith(false);
    expect(onSignedIn).not.toHaveBeenCalled();
  });
});
