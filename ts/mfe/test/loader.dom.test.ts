// loadElement and containRemoteStyles against jsdom's real document and
// customElements. jsdom never fetches the injected script, so a load is
// simulated by defining the tag, and a failure by dispatching `error` on the
// injected `<script>`. The per-tag load cache is module state: each test uses
// its own tag.
import { afterEach, describe, expect, it } from 'vitest';

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

  // The cache is keyed by tag only. A host that first loads a tag without a
  // version floor (or with a lower one) and later asks for apiVersion 2 is
  // handed the cached `true`, and mounts a bundle it meant to refuse.
  it('should_refuse_an_older_bundle_for_a_stricter_caller_after_a_lenient_load', async () => {
    define('ev-cache-floor', 1);
    await expect(loadElement('ev-cache-floor', '/mfe/cache-floor.js')).resolves.toBe(true);

    const strict = loadElement('ev-cache-floor', '/mfe/cache-floor.js', { minApiVersion: 2 });

    await expect(strict).resolves.toBe(false);
  });
});

describe('containRemoteStyles', () => {
  function stylesheet(href: string): HTMLLinkElement {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = href;
    return link;
  }

  it('should_demote_a_stylesheet_from_the_bundle_directory_into_the_layer', () => {
    document.head.appendChild(stylesheet('/mfe/chip/chip.css'));

    const stop = containRemoteStyles('/mfe/chip/chip.js');
    stop();

    const style = document.head.querySelector('style');
    expect(document.head.querySelector('link')).toBeNull();
    expect(style?.dataset['mfeLayer']).toBe(REMOTE_LAYER);
    expect(style?.textContent).toBe(`@import url("${new URL('/mfe/chip/chip.css', window.location.href).href}") layer(${REMOTE_LAYER});`);
  });

  it('should_leave_the_hosts_own_stylesheet_alone', () => {
    const own = stylesheet('/_next/static/app.css');
    document.head.appendChild(own);

    const stop = containRemoteStyles('/mfe/chip/chip.js');
    stop();

    expect(document.head.querySelector('link')).toBe(own);
  });

  it('should_demote_a_stylesheet_the_remote_injects_later', async () => {
    const stop = containRemoteStyles('/mfe/late/late.js', 'remote');

    document.head.appendChild(stylesheet('/mfe/late/late.css'));
    await Promise.resolve();
    stop();

    expect(document.head.querySelector('link')).toBeNull();
    expect(document.head.querySelector('style')?.dataset['mfeLayer']).toBe('remote');
  });

  it('should_stop_demoting_after_cleanup', async () => {
    const stop = containRemoteStyles('/mfe/stopped/x.js');
    stop();

    const late = stylesheet('/mfe/stopped/x.css');
    document.head.appendChild(late);
    await Promise.resolve();

    expect(document.head.querySelector('link')).toBe(late);
  });
});
