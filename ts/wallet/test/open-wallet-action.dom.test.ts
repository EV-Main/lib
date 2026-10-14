// openWalletAction opens evinvest.ltd in a popup and waits for its result.
// Only a message from that popup, from the wallet origin, counts; every other
// way out (closed, timeout, abort) is a value, and each one closes the popup.
// One action per page: while a popup is in flight, another call focuses it and
// resolves `busy`. That in-flight popup is module state, so each test imports a
// fresh copy of the module. jsdom has no popups and no `Window.closed`, so
// `window.open` returns a fake that only carries what the helper reads.
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import type { WalletActionOutcome } from '../src/index';

type WalletModule = typeof import('../src/index');

const ACTION = 'https://evinvest.ltd/wallet/withdraw?amount=5';
const RESULT = { type: 'ev:wallet-result', outcome: 'done', reference: 'payout-1' } as const;

interface FakePopup {
  closed: boolean;
  close: () => void;
  focus: () => void;
}

function fakePopup(): FakePopup {
  const popup: FakePopup = {
    closed: false,
    close: vi.fn(() => {
      popup.closed = true;
    }),
    focus: vi.fn(),
  };
  return popup;
}

// The one cast in this file: the fake stands in for a real `Window`, which jsdom
// cannot open. The helper reads only `closed`, `close()` and `focus()`, and
// compares the object by identity against `MessageEvent.source`.
function asWindow(popup: FakePopup): Window {
  return popup as unknown as Window;
}

function stubOpen(...popups: (FakePopup | null)[]) {
  const open = vi.spyOn(window, 'open');
  for (const popup of popups) open.mockReturnValueOnce(popup === null ? null : asWindow(popup));
  return open;
}

function post(data: unknown, from: { origin: string; source: FakePopup }): void {
  window.dispatchEvent(new MessageEvent('message', { data, origin: from.origin, source: asWindow(from.source) }));
}

let openWalletAction: WalletModule['openWalletAction'];

beforeEach(async () => {
  vi.resetModules();
  ({ openWalletAction } = await import('../src/index'));
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

  it('should_open_on_the_next_call_after_a_blocked_one', async () => {
    const popup = fakePopup();
    const open = stubOpen(null, popup);
    await openWalletAction(ACTION);

    void openWalletAction(ACTION);

    expect(open).toHaveBeenCalledTimes(2);
  });
});

describe('openWalletAction: the popup', () => {
  it('should_open_the_action_with_this_origin_as_opener_in_the_named_wallet_window', () => {
    const open = stubOpen(fakePopup());

    void openWalletAction(new URL(ACTION));

    expect(open).toHaveBeenCalledWith(
      'https://evinvest.ltd/wallet/withdraw?amount=5&opener=http%3A%2F%2Flocalhost%3A3000',
      'ev-wallet-action',
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

describe('openWalletAction: one action per page', () => {
  it('should_focus_the_open_popup_and_not_open_another_while_one_is_in_flight', async () => {
    const popup = fakePopup();
    const open = stubOpen(popup, fakePopup());
    void openWalletAction(ACTION);

    await expect(openWalletAction('https://evinvest.ltd/wallet/transfer')).resolves.toEqual({ status: 'busy' });
    expect(open).toHaveBeenCalledTimes(1);
    expect(popup.focus).toHaveBeenCalledTimes(1);
  });

  it('should_keep_the_first_action_answering_after_a_busy_call', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const first = openWalletAction(ACTION);
    await openWalletAction(ACTION);

    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });

    await expect(first).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
  });

  it('should_open_a_new_popup_once_the_previous_action_settled', async () => {
    const first = fakePopup();
    const second = fakePopup();
    const open = stubOpen(first, second);
    const done = openWalletAction(ACTION);
    post(RESULT, { origin: 'https://evinvest.ltd', source: first });
    await done;

    const next = openWalletAction('https://evinvest.ltd/wallet/transfer');
    post({ ...RESULT, reference: 'transfer-1' }, { origin: 'https://evinvest.ltd', source: second });

    await expect(next).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'transfer-1' } });
    expect(open).toHaveBeenCalledTimes(2);
  });

  it('should_free_the_page_on_settle_even_when_the_popup_stays_open', async () => {
    const stuck: FakePopup = { closed: false, close: vi.fn(), focus: vi.fn() };
    const open = stubOpen(stuck, fakePopup());
    const timedOut = openWalletAction(ACTION, { timeoutMs: 1_000 });
    vi.advanceTimersByTime(1_000);
    await timedOut;

    void openWalletAction(ACTION);

    expect(open).toHaveBeenCalledTimes(2);
    expect(stuck.focus).not.toHaveBeenCalled();
  });

  it('should_open_a_new_popup_when_the_previous_one_was_closed_but_not_yet_settled', async () => {
    const first = fakePopup();
    const open = stubOpen(first, fakePopup());
    void openWalletAction(ACTION);

    first.closed = true;
    void openWalletAction(ACTION);

    expect(open).toHaveBeenCalledTimes(2);
  });

  it('should_not_let_a_settling_old_action_free_the_new_one', async () => {
    const first = fakePopup();
    const second = fakePopup();
    const open = stubOpen(first, second, fakePopup());
    const old = openWalletAction(ACTION, { pollMs: 200 });
    first.closed = true;
    void openWalletAction(ACTION);
    vi.advanceTimersByTime(200);
    vi.advanceTimersToNextTimer();
    await expect(old).resolves.toEqual({ status: 'closed' });

    await expect(openWalletAction(ACTION)).resolves.toEqual({ status: 'busy' });
    expect(open).toHaveBeenCalledTimes(2);
    expect(second.focus).toHaveBeenCalledTimes(1);
  });
});

describe('openWalletAction: other ways out', () => {
  it('should_resolve_closed_once_the_poll_sees_the_popup_closed', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION, { pollMs: 200 });

    popup.closed = true;
    vi.advanceTimersByTime(200);
    // The poll defers `closed` by one more timer, so a result already on its way wins.
    vi.advanceTimersToNextTimer();

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

  it('should_let_a_result_posted_just_before_closing_win_over_closed', async () => {
    const popup = fakePopup();
    stubOpen(popup);
    const outcome = openWalletAction(ACTION, { pollMs: 200 });

    popup.closed = true;
    vi.advanceTimersToNextTimer();
    post(RESULT, { origin: 'https://evinvest.ltd', source: popup });
    vi.advanceTimersToNextTimer();

    await expect(outcome).resolves.toEqual({ status: 'result', result: { outcome: 'done', reference: 'payout-1' } });
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
