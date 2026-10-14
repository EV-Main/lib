// navigateListener is the one `ev:navigate` handler every host wrapper uses:
// the detail comes from a bundle the host does not build, so only a
// root-relative path reaches the host's router, and the click is cancelled only
// when the router took it.
import { describe, expect, it, vi } from 'vitest';

import { navigateListener, type NavigateHandler } from '../src/react/index';

function navigateEvent(href: unknown): CustomEvent<unknown> {
  return new CustomEvent('ev:navigate', { cancelable: true, detail: { href } });
}

describe('navigateListener', () => {
  it('should_hand_a_root_relative_path_to_the_host_and_cancel_the_click_when_taken', () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    const event = navigateEvent('/account?tab=security#top');

    navigateListener(onNavigate)(event);

    expect(onNavigate).toHaveBeenCalledWith('/account?tab=security#top');
    expect(event.defaultPrevented).toBe(true);
  });

  it('should_leave_the_click_to_the_browser_when_the_host_declines', () => {
    const event = navigateEvent('/account');

    navigateListener(() => false)(event);

    expect(event.defaultPrevented).toBe(false);
  });

  it('should_leave_the_click_to_the_browser_without_a_handler', () => {
    const event = navigateEvent('/account');

    navigateListener(undefined)(event);

    expect(event.defaultPrevented).toBe(false);
  });

  it.each([
    ['protocol-relative', '//evil.example'],
    ['backslash', '/\\evil.example'],
    ['tab', '/\t/evil.example'],
    ['absolute', 'https://evil.example/x'],
    ['javascript', 'javascript:alert(1)'],
    ['not a string', 42],
  ])('should_not_ask_the_host_about_a_%s_href', (_label, href) => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);
    const event = navigateEvent(href);

    navigateListener(onNavigate)(event);

    expect(onNavigate).not.toHaveBeenCalled();
    expect(event.defaultPrevented).toBe(false);
  });

  it('should_not_ask_the_host_when_the_detail_is_missing', () => {
    const onNavigate = vi.fn<NavigateHandler>(() => true);

    navigateListener(onNavigate)(new CustomEvent('ev:navigate', { cancelable: true }));

    expect(onNavigate).not.toHaveBeenCalled();
  });
});
