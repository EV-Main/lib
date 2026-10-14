// loadElement and containRemoteStyles against jsdom's real document and
// customElements. jsdom never fetches the injected script, so a load is
// simulated by defining the tag, and a failure by dispatching `error` on the
// injected `<script>`. The per-tag load cache is module state: each test uses
// its own tag.
import { afterEach, describe, expect, it, vi } from 'vitest';

import { containRemoteStyles, loadElement, REMOTE_LAYER } from '../src/index';

function injectedScripts(tag: string): HTMLScriptElement[] {
  return Array.from(document.querySelectorAll<HTMLScriptElement>('script[data-mfe]')).filter(script => script.dataset['mfe'] === tag);
}

function define(tag: string, apiVersion?: number): void {
  const ctor = class extends HTMLElement {};
  if (apiVersion !== undefined) Object.assign(ctor, { apiVersion });
  customElements.define(tag, ctor);
}

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
  document.head.replaceChildren();
});

describe('loadElement', () => {
  it('should_inject_one_module_script_marked_with_the_tag', () => {
    void loadElement('ev-inject-once', '/mfe/inject-once.js');
    void loadElement('ev-inject-once', '/mfe/inject-once.js');

    const scripts = injectedScripts('ev-inject-once');
    expect(scripts).toHaveLength(1);
    expect(scripts[0]?.type).toBe('module');
    expect(scripts[0]?.getAttribute('src')).toBe('/mfe/inject-once.js');
  });

  it('should_settle_true_once_the_bundle_defines_the_tag', async () => {
    const load = loadElement('ev-load-ok', '/mfe/load-ok.js', { minApiVersion: 2 });

    define('ev-load-ok', 2);

    await expect(load).resolves.toBe(true);
  });

  it('should_settle_false_and_remove_the_script_when_it_fails_to_load', async () => {
    const load = loadElement('ev-load-error', '/mfe/load-error.js', { timeoutMs: 60_000 });

    injectedScripts('ev-load-error')[0]?.dispatchEvent(new Event('error'));

    await expect(load).resolves.toBe(false);
    expect(injectedScripts('ev-load-error')).toHaveLength(0);
  });

  it('should_retry_with_a_fresh_script_after_a_failed_load', async () => {
    const first = loadElement('ev-load-retry', '/mfe/load-retry.js', { timeoutMs: 60_000 });
    injectedScripts('ev-load-retry')[0]?.dispatchEvent(new Event('error'));
    await first;

    const second = loadElement('ev-load-retry', '/mfe/load-retry.js');

    expect(second).not.toBe(first);
    expect(injectedScripts('ev-load-retry')).toHaveLength(1);
  });

  it('should_not_inject_when_another_loader_already_has_the_tag_in_flight', () => {
    const foreign = document.createElement('script');
    foreign.dataset['mfe'] = 'ev-foreign';
    document.head.appendChild(foreign);

    void loadElement('ev-foreign', '/mfe/foreign.js');

    expect(injectedScripts('ev-foreign')).toEqual([foreign]);
  });

  it('should_not_inject_for_a_tag_already_defined', async () => {
    define('ev-predefined');

    await expect(loadElement('ev-predefined', '/mfe/predefined.js')).resolves.toBe(true);
    expect(injectedScripts('ev-predefined')).toHaveLength(0);
  });

  // A load is shared per tag, the version verdict is per call: a host that
  // first loads a tag without a floor and later asks for apiVersion 2 must not
  // be handed the earlier `true` for a bundle it means to refuse.
  it('should_refuse_an_older_bundle_for_a_stricter_caller_after_a_lenient_load', async () => {
    define('ev-cache-floor', 1);
    await expect(loadElement('ev-cache-floor', '/mfe/cache-floor.js')).resolves.toBe(true);

    const strict = loadElement('ev-cache-floor', '/mfe/cache-floor.js', { minApiVersion: 2 });

    await expect(strict).resolves.toBe(false);
  });

  it('should_answer_a_strict_and_a_lenient_caller_of_one_load_each_on_its_own', async () => {
    const strict = loadElement('ev-shared-load', '/mfe/shared-load.js', { minApiVersion: 2 });
    const lenient = loadElement('ev-shared-load', '/mfe/shared-load.js');

    define('ev-shared-load', 1);

    await expect(strict).resolves.toBe(false);
    await expect(lenient).resolves.toBe(true);
  });

  it('should_warn_and_keep_one_script_when_a_second_url_asks_for_a_loading_tag', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});

    void loadElement('ev-two-urls', '/mfe/a/two-urls.js');
    void loadElement('ev-two-urls', '/mfe/b/two-urls.js');

    expect(warn).toHaveBeenCalledTimes(1);
    expect(warn.mock.calls[0]?.[0]).toContain('/mfe/b/two-urls.js');
    expect(injectedScripts('ev-two-urls').map(script => script.getAttribute('src'))).toEqual(['/mfe/a/two-urls.js']);
  });

  it('should_remove_a_timed_out_script_so_the_next_load_injects_a_fresh_one', async () => {
    vi.useFakeTimers({ toFake: ['setTimeout', 'clearTimeout'] });
    const first = loadElement('ev-timed-out', '/mfe/timed-out.js', { timeoutMs: 1000 });
    const stale = injectedScripts('ev-timed-out')[0];

    await vi.advanceTimersByTimeAsync(1000);
    await expect(first).resolves.toBe(false);
    expect(injectedScripts('ev-timed-out')).toHaveLength(0);

    void loadElement('ev-timed-out', '/mfe/timed-out.js', { timeoutMs: 1000 });

    const fresh = injectedScripts('ev-timed-out');
    expect(fresh).toHaveLength(1);
    expect(fresh[0]).not.toBe(stale);
  });
});

describe('containRemoteStyles', () => {
  function stylesheet(href: string): HTMLLinkElement {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = href;
    return link;
  }

  /** Lets the MutationObserver deliver the records of the last DOM change. */
  async function observed(): Promise<void> {
    await Promise.resolve();
  }

  function absolute(path: string): string {
    return new URL(path, window.location.href).href;
  }

  it('should_leave_a_stylesheet_already_on_the_page_alone', async () => {
    const own = stylesheet('/mfe/chip/chip.css');
    document.head.appendChild(own);

    const stop = containRemoteStyles('/mfe/chip/chip.js');
    await observed();
    stop();

    expect(Array.from(document.head.children)).toEqual([own]);
  });

  it('should_put_a_layered_copy_before_a_stylesheet_the_remote_injects_later', async () => {
    const stop = containRemoteStyles('/mfe/chip/chip.js');

    const link = stylesheet('/mfe/chip/chip.css');
    document.head.appendChild(link);
    await observed();
    stop();

    const style = document.head.querySelector('style');
    expect(style?.dataset['mfeLayer']).toBe(REMOTE_LAYER);
    expect(style?.textContent).toBe(`@import url("${absolute('/mfe/chip/chip.css')}") layer(${REMOTE_LAYER});`);
    expect(Array.from(document.head.children)).toEqual([style, link]);
  });

  it('should_drop_the_link_once_the_layered_copy_loads', async () => {
    const stop = containRemoteStyles('/mfe/loaded/x.js', 'remote');
    document.head.appendChild(stylesheet('/mfe/loaded/x.css'));
    await observed();
    stop();

    document.head.querySelector('style')?.dispatchEvent(new Event('load'));

    expect(document.head.querySelector('link')).toBeNull();
    expect(document.head.querySelector('style')?.dataset['mfeLayer']).toBe('remote');
  });

  it('should_drop_the_link_when_the_layered_copy_fails', async () => {
    const stop = containRemoteStyles('/mfe/failed/x.js');
    document.head.appendChild(stylesheet('/mfe/failed/x.css'));
    await observed();
    stop();

    document.head.querySelector('style')?.dispatchEvent(new Event('error'));

    expect(document.head.querySelector('link')).toBeNull();
  });

  it('should_drop_the_link_after_10_seconds_when_no_load_event_comes', async () => {
    vi.useFakeTimers({ toFake: ['setTimeout', 'clearTimeout'] });
    const stop = containRemoteStyles('/mfe/silent/x.js');
    const link = stylesheet('/mfe/silent/x.css');
    document.head.appendChild(link);
    await observed();
    stop();

    vi.advanceTimersByTime(9_999);
    expect(document.head.querySelector('link')).toBe(link);

    vi.advanceTimersByTime(1);
    expect(document.head.querySelector('link')).toBeNull();
  });

  it('should_demote_nothing_for_a_bundle_at_the_origin_root', async () => {
    const stop = containRemoteStyles('/x.js');

    const link = stylesheet('/x.css');
    document.head.appendChild(link);
    await observed();
    stop();

    expect(Array.from(document.head.children)).toEqual([link]);
  });

  it('should_take_the_directory_from_the_path_not_the_query', async () => {
    const stop = containRemoteStyles('/mfe/versioned/x.js?v=1/2');

    document.head.appendChild(stylesheet('/mfe/versioned/x.css'));
    await observed();
    stop();

    expect(document.head.querySelector('style')?.textContent).toContain(absolute('/mfe/versioned/x.css'));
  });

  it('should_leave_a_stylesheet_from_another_directory_alone', async () => {
    const stop = containRemoteStyles('/mfe/chip/chip.js');

    const own = stylesheet('/_next/static/app.css');
    document.head.appendChild(own);
    await observed();
    stop();

    expect(Array.from(document.head.children)).toEqual([own]);
  });

  it('should_stop_demoting_after_cleanup', async () => {
    const stop = containRemoteStyles('/mfe/stopped/x.js');
    stop();

    const late = stylesheet('/mfe/stopped/x.css');
    document.head.appendChild(late);
    await observed();

    expect(Array.from(document.head.children)).toEqual([late]);
  });
});
