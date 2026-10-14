// A link click inside an element is offered to the host's router only when it
// is a plain same-origin navigation in this tab; the host reads the offered
// href as untrusted and takes only a root-relative path. delegatedHref vectors
// are carried over from banking `cabinet/frontend/features/account-chip/lib/navigate-event.test.ts`.
import { describe, expect, it } from 'vitest';

import {
  API_VERSION,
  delegatedHref,
  dispatchNavigate,
  dispatchSignedIn,
  dispatchSignOut,
  NAVIGATE_EVENT,
  navigateHref,
  SIGNED_IN_EVENT,
  SIGNOUT_EVENT,
  speaksApi,
  type LinkAnchor,
  type LinkClick,
} from '../src/index';

const PAGE = { href: 'https://cabinet.example/en/cabinet', origin: 'https://cabinet.example' };
const PLAIN: LinkClick = { button: 0, ctrlKey: false, metaKey: false, shiftKey: false, altKey: false, defaultPrevented: false };

/** An `<a>` as the DOM reports it: `href` resolved, `target` as written. */
function anchor(href: string, attrs: { target?: string; download?: boolean } = {}): LinkAnchor {
  return { href, target: attrs.target ?? '', hasAttribute: name => name === 'download' && attrs.download === true };
}

const SETTINGS = anchor('https://cabinet.example/en/cabinet/settings?tab=security#top');

function navigateEvent(detail: unknown): CustomEvent<unknown> {
  return new CustomEvent(NAVIGATE_EVENT, { detail });
}

describe('event names and apiVersion', () => {
  it('should_keep_the_frozen_v2_event_names', () => {
    expect([NAVIGATE_EVENT, SIGNED_IN_EVENT, SIGNOUT_EVENT]).toEqual(['ev:navigate', 'ev:signed-in', 'signout']);
    expect(API_VERSION).toBe(2);
  });

  it('should_speak_the_api_only_for_a_class_with_a_numeric_apiVersion_at_least_min', () => {
    class V2 {
      static apiVersion = 2;
    }
    class V1 {
      static apiVersion = 1;
    }
    class Bare {}
    class Stringly {
      static apiVersion = '2';
    }
    expect(speaksApi(V2, 2)).toBe(true);
    expect(speaksApi(V1, 2)).toBe(false);
    expect(speaksApi(Bare, 1)).toBe(false);
    expect(speaksApi(Stringly, 2)).toBe(false);
    expect(speaksApi(undefined, 0)).toBe(false);
  });
});

describe('dispatchNavigate', () => {
  it('should_dispatch_one_bubbling_composed_cancelable_event_with_the_href', () => {
    const source = new EventTarget();
    const seen: Event[] = [];
    source.addEventListener(NAVIGATE_EVENT, event => void seen.push(event));

    dispatchNavigate(source, '/en/cabinet/profile');

    expect(seen).toHaveLength(1);
    const event = seen[0];
    expect(event).toBeInstanceOf(CustomEvent);
    expect((event as CustomEvent<unknown>).detail).toEqual({ href: '/en/cabinet/profile' });
    expect([event?.bubbles, event?.composed, event?.cancelable]).toEqual([true, true, true]);
  });

  it('should_return_true_when_a_listener_calls_preventDefault', () => {
    const source = new EventTarget();
    source.addEventListener(NAVIGATE_EVENT, event => event.preventDefault());

    expect(dispatchNavigate(source, '/en/cabinet')).toBe(true);
  });

  it('should_return_false_with_no_listener', () => {
    expect(dispatchNavigate(new EventTarget(), '/en/cabinet')).toBe(false);
  });

  it('should_return_false_when_the_listener_only_looks', () => {
    const source = new EventTarget();
    source.addEventListener(NAVIGATE_EVENT, () => undefined);

    expect(dispatchNavigate(source, '/en/cabinet')).toBe(false);
  });
});

describe('dispatchSignedIn / dispatchSignOut', () => {
  it('should_dispatch_bubbling_composed_events_with_null_detail', () => {
    const source = new EventTarget();
    const seen: CustomEvent<unknown>[] = [];
    const record = (event: Event) => void seen.push(event as CustomEvent<unknown>);
    source.addEventListener(SIGNED_IN_EVENT, record);
    source.addEventListener(SIGNOUT_EVENT, record);

    dispatchSignedIn(source);
    dispatchSignOut(source);

    expect(seen.map(event => [event.type, event.detail, event.bubbles, event.composed])).toEqual([
      ['ev:signed-in', null, true, true],
      ['signout', null, true, true],
    ]);
  });
});

describe('navigateHref', () => {
  it('should_return_a_root_relative_href', () => {
    expect(navigateHref(navigateEvent({ href: '/en/cabinet?tab=1#a' }))).toBe('/en/cabinet?tab=1#a');
  });

  it('should_refuse_a_protocol_relative_href', () => {
    expect(navigateHref(navigateEvent({ href: '//evil.example/x' }))).toBeNull();
  });

  it('should_refuse_a_backslash_href', () => {
    expect(navigateHref(navigateEvent({ href: '/\\evil.example/x' }))).toBeNull();
  });

  it('should_refuse_a_tab_the_browser_would_strip_into_protocol_relative', () => {
    expect(navigateHref(navigateEvent({ href: '/\t/evil.example/x' }))).toBeNull();
  });

  it('should_refuse_an_absolute_url', () => {
    expect(navigateHref(navigateEvent({ href: 'https://cabinet.example/en' }))).toBeNull();
  });

  it('should_refuse_a_non_string_href', () => {
    expect(navigateHref(navigateEvent({ href: 42 }))).toBeNull();
    expect(navigateHref(navigateEvent({ href: { toString: () => '/x' } }))).toBeNull();
  });

  it('should_refuse_a_missing_or_non_object_detail', () => {
    expect(navigateHref(navigateEvent(null))).toBeNull();
    expect(navigateHref(navigateEvent(undefined))).toBeNull();
    expect(navigateHref(navigateEvent('/en/cabinet'))).toBeNull();
    expect(navigateHref(navigateEvent({}))).toBeNull();
  });

  it('should_refuse_a_plain_event_without_detail', () => {
    expect(navigateHref(new Event(NAVIGATE_EVENT))).toBeNull();
  });
});

describe('delegatedHref: offered', () => {
  it('should_offer_path_query_and_hash_without_the_origin', () => {
    expect(delegatedHref(PLAIN, SETTINGS, PAGE)).toBe('/en/cabinet/settings?tab=security#top');
  });

  it('should_resolve_a_relative_href_against_the_page', () => {
    expect(delegatedHref(PLAIN, anchor('profile?x=1'), PAGE)).toBe('/en/profile?x=1');
  });

  it('should_offer_a_target_self_link', () => {
    expect(delegatedHref(PLAIN, anchor('https://cabinet.example/en/cabinet', { target: '_self' }), PAGE)).toBe('/en/cabinet');
  });
});

describe('delegatedHref: left to the browser', () => {
  it('should_leave_a_middle_or_right_click', () => {
    expect(delegatedHref({ ...PLAIN, button: 1 }, SETTINGS, PAGE)).toBeNull();
    expect(delegatedHref({ ...PLAIN, button: 2 }, SETTINGS, PAGE)).toBeNull();
  });

  it('should_leave_a_click_with_any_modifier', () => {
    expect(delegatedHref({ ...PLAIN, ctrlKey: true }, SETTINGS, PAGE)).toBeNull();
    expect(delegatedHref({ ...PLAIN, metaKey: true }, SETTINGS, PAGE)).toBeNull();
    expect(delegatedHref({ ...PLAIN, shiftKey: true }, SETTINGS, PAGE)).toBeNull();
    expect(delegatedHref({ ...PLAIN, altKey: true }, SETTINGS, PAGE)).toBeNull();
  });

  it('should_leave_a_target_blank_or_named_window_link', () => {
    expect(delegatedHref(PLAIN, anchor(SETTINGS.href, { target: '_blank' }), PAGE)).toBeNull();
    expect(delegatedHref(PLAIN, anchor(SETTINGS.href, { target: 'docs' }), PAGE)).toBeNull();
  });

  it('should_leave_a_download_link', () => {
    expect(delegatedHref(PLAIN, anchor(SETTINGS.href, { download: true }), PAGE)).toBeNull();
  });

  it('should_leave_a_click_already_prevented', () => {
    expect(delegatedHref({ ...PLAIN, defaultPrevented: true }, SETTINGS, PAGE)).toBeNull();
  });

  it('should_leave_another_origin_whatever_its_path', () => {
    expect(delegatedHref(PLAIN, anchor('https://evil.example/en/cabinet'), PAGE)).toBeNull();
    expect(delegatedHref(PLAIN, anchor('http://cabinet.example/en/cabinet'), PAGE)).toBeNull();
    expect(delegatedHref(PLAIN, anchor('https://cabinet.example:8443/en/cabinet'), PAGE)).toBeNull();
    expect(delegatedHref(PLAIN, anchor('//evil.example/x'), PAGE)).toBeNull();
  });

  it('should_leave_an_unparseable_href_without_throwing', () => {
    expect(delegatedHref(PLAIN, anchor('http://[::1'), PAGE)).toBeNull();
  });
});
