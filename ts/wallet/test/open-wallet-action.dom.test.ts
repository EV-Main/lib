// openWalletAction opens evinvest.ltd in a popup and waits for its result.
// Only a message from that popup, from the wallet origin, counts; every other
// way out (closed, timeout, abort) is a value, and each one closes the popup.
// jsdom has no popups and no `Window.closed`, so `window.open` returns a fake
// that only carries what the helper reads: `closed` and `close()`.
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { openWalletAction, type WalletActionOutcome } from '../src/index';

const ACTION = 'https://evinvest.ltd/wallet/withdraw?amount=5';

interface FakePopup {
  closed: boolean;
  close: () => void;
}

function fakePopup(): FakePopup {
  const popup: FakePopup = {
    closed: false,
    close: vi.fn(() => {
      popup.closed = true;
    }),
  };
  return popup;
}

// The one cast in this file: the fake stands in for a real `Window`, which jsdom
// cannot open. The helper reads only `closed` and `close()`, and compares the
// object by identity against `MessageEvent.source`.
function asWindow(popup: FakePopup): Window {
  return popup as unknown as Window;
}

function stubOpen(popup: FakePopup | null) {
  return vi.spyOn(window, 'open').mockReturnValue(popup === null ? null : asWindow(popup));
}

function post(data: unknown, from: { origin: string; source: FakePopup }): void {
  window.dispatchEvent(new MessageEvent('message', { data, origin: from.origin, source: asWindow(from.source) }));
}

const RESULT = { type: 'ev:wallet-result', outcome: 'done', reference: 'payout-1' } as const;

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe('openWalletAction: refused before opening', () => {
  it('should_return_invalid_url_and_not_open_a_foreign_url', async () => {
    const open = stubOpen(fakePopup());

    await expect(openWalletAction('https://evil.example/wallet/withdraw')).resolves.toEqual({ status: 'invalid_url' });
    expect(open).not.toHaveBeenCalled();
  });

  it('should_return_aborted_and_not_open_when_the_signal_is_already_aborted', async () => {
    const open = stubOpen(fakePopup());
    const controller = new AbortController();
    controller.abort();

    await expect(openWalletAction(ACTION, { signal: controller.signal })).resolves.toEqual({ status: 'aborted' });
    expect(open).not.toHaveBeenCalled();
  });

  it('should_return_blocked_when_window_open_is_refused', async () => {
    stubOpen(null);

    await expect(openWalletAction(ACTION)).resolves.toEqual({ status: 'blocked' });
  });
});

describe('openWalletAction: the popup', () => {
  it('should_open_the_action_with_this_origin_as_opener_in_a_popup', () => {
    const open = stubOpen(fakePopup());

    void openWalletAction(new URL(ACTION));

    expect(open).toHaveBeenCalledWith(
      'https://evinvest.ltd/wallet/withdraw?amount=5&opener=http%3A%2F%2Flocalhost%3A3000',
      '_blank',
      'popup=yes,width=480,height=720',
    );
  });

  it('should_resolve_with_the_result_and_close_the_popup', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION);

    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
    expect(popup.close).toHaveBeenCalledTimes(1);
  });

  it('should_ignore_a_result_from_another_origin', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION);

    post({ ...RESULT, reference: 'forged' }, { origin: 'https://evil.example', source: popup });
    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
  });

  it('should_ignore_a_result_from_another_evinvest_window', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION);

    post({ ...RESULT, reference: 'other-tab' }, { origin: 'https://evinvest.ltd', source: fakePopup() });
    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
  });

  it('should_ignore_a_message_from_the_popup_that_is_not_a_result', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION, { timeoutMs: 1_000 });

    post({ type: 'ev:wallet-progress' }, { origin: 'https://evinvest.ltd', source: popup });
    vi.advanceTimersByTime(1_000);

    await expect(outcome).resolves.toEqual({ status: 'timeout' });
  });

  it('should_accept_a_result_from_a_staging_origin_when_given_one', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction('https://stage.evinvest.ltd/wallet/pay', { origin: 'https://stage.evinvest.ltd' });

    post(RESULT, { origin: 'https://stage.evinvest.ltd', source: popup });

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
  });
});

describe('openWalletAction: other ways out', () => {
  it('should_resolve_closed_once_the_poll_sees_the_popup_closed', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION, { pollMs: 200 });

    popup.closed = true;
    vi.advanceTimersByTime(200);

    await expect(outcome).resolves.toEqual({ status: 'closed' });
    expect(popup.close).not.toHaveBeenCalled();
  });

  it('should_not_resolve_closed_before_the_poll_runs', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const settled: WalletActionOutcome[] = [];
    void openWalletAction(ACTION, { pollMs: 200 }).then(outcome => settled.push(outcome));

    popup.closed = true;
    await vi.advanceTimersByTimeAsync(199);

    expect(settled).toEqual([]);
  });

  it('should_resolve_timeout_and_close_the_popup_after_timeoutMs', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION, { timeoutMs: 5_000 });

    vi.advanceTimersByTime(5_000);

    await expect(outcome).resolves.toEqual({ status: 'timeout' });
    expect(popup.close).toHaveBeenCalledTimes(1);
  });

  it('should_time_out_after_fifteen_minutes_by_default', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const settled: WalletActionOutcome[] = [];
    const outcome = openWalletAction(ACTION);
    void outcome.then(value => settled.push(value));

    await vi.advanceTimersByTimeAsync(15 * 60_000 - 1);
    expect(settled).toEqual([]);
    vi.advanceTimersByTime(1);

    await expect(outcome).resolves.toEqual({ status: 'timeout' });
  });

  it('should_resolve_aborted_and_close_the_popup_on_abort', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const controller = new AbortController();
    const outcome = openWalletAction(ACTION, { signal: controller.signal });

    controller.abort();

    await expect(outcome).resolves.toEqual({ status: 'aborted' });
    expect(popup.close).toHaveBeenCalledTimes(1);
  });

  it('should_leave_no_timers_running_once_settled', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION);

    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });
    await outcome;

    expect(vi.getTimerCount()).toBe(0);
  });

  it('should_keep_the_first_outcome_when_a_second_result_arrives', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION);

    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });
    post({ ...RESULT, outcome: 'failed' }, { origin: 'https://evinvest.ltd', source: popup });

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
    expect(popup.close).toHaveBeenCalledTimes(1);
  });
});
