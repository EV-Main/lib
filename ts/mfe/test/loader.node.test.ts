// awaitElement decides whether a remote is usable: it settles true only for a
// class speaking the required apiVersion, false for everything else (an older
// bundle, a script that failed, a hung load), and never rejects. Driven with a
// fake registry and fake timers: no DOM, no real clock.
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { awaitElement, type ElementRegistry } from '../src/index';

const TAG = 'ev-test-chip';
const NEVER = new Promise<void>(() => {});

/** A registry whose `whenDefined` settles only when the test says so. */
function deferredRegistry() {
  let define: (ctor: CustomElementConstructor) => void = () => {};
  let reject: (reason: unknown) => void = () => {};
  const defined = new Promise<CustomElementConstructor>((resolve, fail) => {
    define = resolve;
    reject = fail;
  });
  const registry: ElementRegistry = { get: () => undefined, whenDefined: () => defined };
  return { registry, define, reject };
}

/** The settled value so far, or 'pending'. */
function track(promise: Promise<boolean>) {
  const state: { value: boolean | 'pending' } = { value: 'pending' };
  void promise.then(value => {
    state.value = value;
  });
  return state;
}

function elementClass(apiVersion?: number): CustomElementConstructor {
  const ctor = class {};
  if (apiVersion !== undefined) Object.assign(ctor, { apiVersion });
  return ctor as unknown as CustomElementConstructor;
}

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe('awaitElement', () => {
  it('should_settle_true_for_a_class_speaking_the_required_api_version', async () => {
    const { registry, define } = deferredRegistry();
    const result = awaitElement(registry, TAG, 2, 1000, NEVER);

    define(elementClass(2));

    await expect(result).resolves.toBe(true);
  });

  it('should_settle_false_for_a_class_with_an_older_api_version', async () => {
    const { registry, define } = deferredRegistry();
    const result = awaitElement(registry, TAG, 2, 1000, NEVER);

    define(elementClass(1));

    await expect(result).resolves.toBe(false);
  });

  it('should_settle_false_for_a_class_without_api_version_when_one_is_required', async () => {
    const { registry, define } = deferredRegistry();
    const result = awaitElement(registry, TAG, 1, 1000, NEVER);

    define(elementClass());

    await expect(result).resolves.toBe(false);
  });

  it('should_accept_any_class_when_min_api_version_is_zero', async () => {
    const { registry, define } = deferredRegistry();
    const result = awaitElement(registry, TAG, 0, 1000, NEVER);

    define(elementClass());

    await expect(result).resolves.toBe(true);
  });

  it('should_stay_pending_until_the_timeout_and_then_settle_false', async () => {
    const { registry } = deferredRegistry();
    const state = track(awaitElement(registry, TAG, 0, 1000, NEVER));

    await vi.advanceTimersByTimeAsync(999);
    expect(state.value).toBe('pending');

    await vi.advanceTimersByTimeAsync(1);
    expect(state.value).toBe(false);
  });

  it('should_settle_false_when_the_script_fails_before_the_tag_is_defined', async () => {
    const { registry } = deferredRegistry();
    const result = awaitElement(registry, TAG, 0, 1000, Promise.resolve());

    await expect(result).resolves.toBe(false);
  });

  it('should_settle_false_instead_of_rejecting_when_whenDefined_rejects', async () => {
    const { registry, reject } = deferredRegistry();
    const result = awaitElement(registry, TAG, 0, 1000, NEVER);

    reject(new SyntaxError('not a valid custom element name'));

    await expect(result).resolves.toBe(false);
  });

  it('should_clear_its_timer_once_settled', async () => {
    const { registry, define } = deferredRegistry();
    const result = awaitElement(registry, TAG, 0, 1000, NEVER);

    define(elementClass());
    await result;

    expect(vi.getTimerCount()).toBe(0);
  });
});
