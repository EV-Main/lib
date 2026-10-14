// RemoteElement mounts a remote's custom element in light DOM. Pinned here:
// attributes are in place before the element's connectedCallback reads them;
// attribute changes and handler identity churn update the live element instead
// of remounting it; and awaitPaint holds the fallback until the element says it
// painted. Each test defines its own tag up front, so the loader resolves
// without a network (jsdom never fetches the injected script).
import { act, type ReactNode } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { RemoteElement, useElementBundle, type BundleState, type RemoteEventHandler } from '../src/react/index';

interface Probe {
  /** What `data-name` read at connect, the moment a remote reads its configuration. */
  nameAtConnect: (string | null)[];
  connects: number;
}

function defineProbe(tag: string): Probe {
  const probe: Probe = { nameAtConnect: [], connects: 0 };
  customElements.define(
    tag,
    class extends HTMLElement {
      static apiVersion = 2;
      connectedCallback() {
        probe.connects += 1;
        probe.nameAtConnect.push(this.getAttribute('data-name'));
      }
    },
  );
  return probe;
}

let container: HTMLDivElement;
let root: Root;

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

describe('RemoteElement', () => {
  it('should_set_attributes_before_connectedCallback_runs', async () => {
    const probe = defineProbe('ev-probe-connect');

    await render(<RemoteElement tag="ev-probe-connect" scriptUrl="/mfe/connect.js" attributes={{ 'data-name': 'Valentina' }} />);

    expect(probe.nameAtConnect).toEqual(['Valentina']);
  });

  it('should_encode_objects_as_json_numbers_as_strings_and_true_as_empty', async () => {
    defineProbe('ev-probe-encode');

    await render(
      <RemoteElement
        tag="ev-probe-encode"
        scriptUrl="/mfe/encode.js"
        attributes={{ 'data-groups': [{ id: 'g' }], 'data-count': 3, 'data-open': true }}
      />,
    );

    const element = container.querySelector('ev-probe-encode');
    expect(element?.getAttribute('data-groups')).toBe('[{"id":"g"}]');
    expect(element?.getAttribute('data-count')).toBe('3');
    expect(element?.getAttribute('data-open')).toBe('');
  });

  it('should_not_set_an_attribute_for_false_null_or_undefined', async () => {
    defineProbe('ev-probe-absent');

    await render(
      <RemoteElement
        tag="ev-probe-absent"
        scriptUrl="/mfe/absent.js"
        attributes={{ 'data-off': false, 'data-null': null, 'data-undefined': undefined }}
      />,
    );

    const element = container.querySelector('ev-probe-absent');
    expect(element?.getAttributeNames()).toEqual([]);
  });

  it('should_remove_an_attribute_whose_value_changes_to_false', async () => {
    defineProbe('ev-probe-to-false');
    await render(<RemoteElement tag="ev-probe-to-false" scriptUrl="/mfe/to-false.js" attributes={{ 'data-intent': 'x' }} />);

    await render(<RemoteElement tag="ev-probe-to-false" scriptUrl="/mfe/to-false.js" attributes={{ 'data-intent': false }} />);

    expect(container.querySelector('ev-probe-to-false')?.hasAttribute('data-intent')).toBe(false);
  });

  it('should_remove_an_attribute_whose_value_changes_to_null_or_undefined', async () => {
    defineProbe('ev-probe-to-null');
    await render(
      <RemoteElement tag="ev-probe-to-null" scriptUrl="/mfe/to-null.js" attributes={{ 'data-a': 'x', 'data-b': 'y' }} />,
    );

    await render(
      <RemoteElement tag="ev-probe-to-null" scriptUrl="/mfe/to-null.js" attributes={{ 'data-a': null, 'data-b': undefined }} />,
    );

    expect(container.querySelector('ev-probe-to-null')?.getAttributeNames()).toEqual([]);
  });

  it('should_mark_the_wrapper_ready_once_the_bundle_is_usable', async () => {
    defineProbe('ev-probe-state');

    await render(<RemoteElement tag="ev-probe-state" scriptUrl="/mfe/state.js" />);

    expect(container.querySelector('[data-mfe-state]')?.getAttribute('data-mfe-state')).toBe('ready');
  });

  it('should_update_and_remove_attributes_on_the_same_element', async () => {
    const probe = defineProbe('ev-probe-remove');
    await render(
      <RemoteElement tag="ev-probe-remove" scriptUrl="/mfe/remove.js" attributes={{ 'data-name': 'A', 'data-email': 'a@example.com' }} />,
    );
    const mounted = container.querySelector('ev-probe-remove');

    await render(<RemoteElement tag="ev-probe-remove" scriptUrl="/mfe/remove.js" attributes={{ 'data-name': 'B' }} />);

    const element = container.querySelector('ev-probe-remove');
    expect(element).toBe(mounted);
    expect(element?.getAttribute('data-name')).toBe('B');
    expect(element?.hasAttribute('data-email')).toBe(false);
    expect(probe.connects).toBe(1);
  });

  it('should_route_events_to_the_latest_handler_without_remounting', async () => {
    const probe = defineProbe('ev-probe-events');
    const first = vi.fn<RemoteEventHandler>();
    const second = vi.fn<RemoteEventHandler>();
    await render(<RemoteElement tag="ev-probe-events" scriptUrl="/mfe/events.js" events={{ 'ev:navigate': first }} />);
    const mounted = container.querySelector('ev-probe-events');

    await render(<RemoteElement tag="ev-probe-events" scriptUrl="/mfe/events.js" events={{ 'ev:navigate': second }} />);
    container.querySelector('ev-probe-events')?.dispatchEvent(new CustomEvent('ev:navigate', { detail: { href: '/x' } }));

    expect(container.querySelector('ev-probe-events')).toBe(mounted);
    expect(probe.connects).toBe(1);
    expect(first).not.toHaveBeenCalled();
    expect(second).toHaveBeenCalledTimes(1);
  });

  it('should_let_the_host_cancel_a_navigate_event', async () => {
    defineProbe('ev-probe-cancel');
    await render(
      <RemoteElement tag="ev-probe-cancel" scriptUrl="/mfe/cancel.js" events={{ 'ev:navigate': event => event.preventDefault() }} />,
    );

    const event = new CustomEvent('ev:navigate', { cancelable: true, detail: { href: '/x' } });
    container.querySelector('ev-probe-cancel')?.dispatchEvent(event);

    expect(event.defaultPrevented).toBe(true);
  });

  it('should_hide_the_fallback_once_ready_without_awaitPaint', async () => {
    defineProbe('ev-probe-nopaint');

    await render(<RemoteElement tag="ev-probe-nopaint" scriptUrl="/mfe/nopaint.js" fallback={<span>Loading account</span>} />);

    expect(container.textContent).not.toContain('Loading account');
  });

  it('should_hold_the_fallback_until_the_element_sets_data_ready_with_awaitPaint', async () => {
    defineProbe('ev-probe-paint');
    await render(
      <RemoteElement tag="ev-probe-paint" scriptUrl="/mfe/paint.js" awaitPaint fallback={<span>Loading account</span>} />,
    );
    expect(container.textContent).toContain('Loading account');

    await act(async () => {
      container.querySelector('ev-probe-paint')?.setAttribute('data-ready', '');
    });

    expect(container.textContent).not.toContain('Loading account');
  });

  it('should_remove_the_element_on_unmount', async () => {
    defineProbe('ev-probe-unmount');
    await render(<RemoteElement tag="ev-probe-unmount" scriptUrl="/mfe/unmount.js" />);

    await render(null);

    expect(container.querySelector('ev-probe-unmount')).toBeNull();
  });
});

describe('useElementBundle', () => {
  function StateLog({ tag, scriptUrl, minApiVersion, log }: { tag: string; scriptUrl: string; minApiVersion: number; log: BundleState[] }) {
    log.push(useElementBundle(tag, scriptUrl, { minApiVersion, layer: false }));
    return null;
  }

  it('should_read_loading_at_once_when_the_script_url_changes_for_the_same_tag', async () => {
    defineProbe('ev-probe-url-switch');
    vi.spyOn(console, 'warn').mockImplementation(() => {});
    const log: BundleState[] = [];
    await render(<StateLog tag="ev-probe-url-switch" scriptUrl="/mfe/a/x.js" minApiVersion={0} log={log} />);
    expect(log.at(-1)).toBe('ready');
    log.length = 0;

    await render(<StateLog tag="ev-probe-url-switch" scriptUrl="/mfe/b/x.js" minApiVersion={0} log={log} />);

    expect(log[0]).toBe('loading');
  });

  it('should_read_loading_at_once_when_the_version_floor_changes_for_the_same_tag', async () => {
    defineProbe('ev-probe-floor-switch');
    const log: BundleState[] = [];
    await render(<StateLog tag="ev-probe-floor-switch" scriptUrl="/mfe/floor.js" minApiVersion={0} log={log} />);
    expect(log.at(-1)).toBe('ready');
    log.length = 0;

    await render(<StateLog tag="ev-probe-floor-switch" scriptUrl="/mfe/floor.js" minApiVersion={3} log={log} />);

    expect(log[0]).toBe('loading');
    expect(log.at(-1)).toBe('failed');
  });
});
