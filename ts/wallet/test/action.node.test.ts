// A money action leaves the scope host for evinvest.ltd, so the action URL is
// checked the way a browser will read it: another origin, a path outside
// `/wallet/`, credentials, backslashes or control characters never get opened.
// The result message comes back over a shared channel and is parsed as untrusted.
import { describe, expect, it } from 'vitest';

import { parseWalletMessage, walletActionFromEvent, walletActionUrl, walletNavigationHref } from '../src/index';

describe('walletActionUrl: what passes', () => {
  it('should_accept_a_url_under_wallet_on_evinvest_ltd', () => {
    expect(walletActionUrl('https://evinvest.ltd/wallet/withdraw?amount=5')?.href).toBe('https://evinvest.ltd/wallet/withdraw?amount=5');
  });

  it('should_accept_a_staging_origin_when_given_one', () => {
    const url = walletActionUrl('https://stage.evinvest.ltd/wallet/pay', 'https://stage.evinvest.ltd');

    expect(url?.href).toBe('https://stage.evinvest.ltd/wallet/pay');
  });

  it('should_accept_a_url_of_exactly_2048_characters', () => {
    const raw = `https://evinvest.ltd/wallet/${'a'.repeat(2020)}`;
    expect(raw).toHaveLength(2048);

    expect(walletActionUrl(raw)?.href).toBe(raw);
  });
});

describe('walletActionUrl: what is refused', () => {
  it.each([
    ['another host', 'https://evil.example/wallet/withdraw'],
    ['a lookalike host', 'https://evinvest.ltd.evil.example/wallet/withdraw'],
    ['a subdomain', 'https://invest.evinvest.ltd/wallet/withdraw'],
    ['plain http', 'http://evinvest.ltd/wallet/withdraw'],
    ['another port', 'https://evinvest.ltd:8443/wallet/withdraw'],
  ])('should_refuse_%s', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });

  it('should_refuse_the_production_origin_when_a_staging_origin_is_expected', () => {
    expect(walletActionUrl('https://evinvest.ltd/wallet/pay', 'https://stage.evinvest.ltd')).toBeNull();
  });

  it.each([
    ['the root', 'https://evinvest.ltd/'],
    ['another section', 'https://evinvest.ltd/admin/users'],
    ['wallet without the trailing slash', 'https://evinvest.ltd/wallet'],
    ['a sibling prefix', 'https://evinvest.ltd/wallets/x'],
  ])('should_refuse_a_path_outside_wallet: %s', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });

  it('should_refuse_dot_segments_that_resolve_outside_wallet', () => {
    expect(walletActionUrl('https://evinvest.ltd/wallet/../admin')).toBeNull();
  });

  it('should_refuse_percent_encoded_dot_segments_that_resolve_outside_wallet', () => {
    expect(walletActionUrl('https://evinvest.ltd/wallet/%2e%2e/admin')).toBeNull();
  });

  it.each([
    ['user and password', 'https://user:pass@evinvest.ltd/wallet/withdraw'],
    ['user only', 'https://user@evinvest.ltd/wallet/withdraw'],
    ['password only', 'https://:pass@evinvest.ltd/wallet/withdraw'],
  ])('should_refuse_credentials_in_the_url: %s', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });

  it.each([
    ['in the path', 'https://evinvest.ltd/wallet/\\..\\admin'],
    ['before the path', 'https://evinvest.ltd\\wallet\\withdraw'],
    ['in the query', 'https://evinvest.ltd/wallet/withdraw?to=a\\b'],
  ])('should_refuse_a_backslash_%s', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });

  it.each([
    ['tab', 'https://evinvest.ltd/wal\tlet/withdraw'],
    ['newline', 'https://evinvest.ltd/wallet/\n../admin'],
    ['nul', 'https://evinvest.ltd/wallet/x\u0000'],
    ['delete', 'https://evinvest.ltd/wallet/x\u007F'],
  ])('should_refuse_a_%s_character', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });

  it('should_refuse_a_url_longer_than_2048_characters', () => {
    const raw = `https://evinvest.ltd/wallet/${'a'.repeat(2021)}`;
    expect(raw).toHaveLength(2049);

    expect(walletActionUrl(raw)).toBeNull();
  });

  it.each([
    ['empty', ''],
    ['relative', '/wallet/withdraw'],
    ['javascript', 'javascript:alert(1)'],
    ['a number', 42],
    ['undefined', undefined],
    ['an object', { href: 'https://evinvest.ltd/wallet/x' }],
  ])('should_refuse_input_that_is_%s', (_label, raw) => {
    expect(walletActionUrl(raw)).toBeNull();
  });
});

describe('walletActionFromEvent', () => {
  it('should_read_the_action_url_from_the_event_detail', () => {
    const event = new CustomEvent('ev:wallet-action', { detail: { href: 'https://evinvest.ltd/wallet/transfer' } });

    expect(walletActionFromEvent(event)?.href).toBe('https://evinvest.ltd/wallet/transfer');
  });

  it('should_return_null_for_a_detail_with_a_foreign_href', () => {
    const event = new CustomEvent('ev:wallet-action', { detail: { href: 'https://evil.example/wallet/transfer' } });

    expect(walletActionFromEvent(event)).toBeNull();
  });

  it('should_return_null_for_a_missing_detail_or_a_plain_event', () => {
    expect(walletActionFromEvent(new CustomEvent('ev:wallet-action'))).toBeNull();
    expect(walletActionFromEvent(new Event('ev:wallet-action'))).toBeNull();
  });
});

describe('walletNavigationHref', () => {
  it('should_add_the_return_address_as_return_to', () => {
    const href = walletNavigationHref(new URL('https://evinvest.ltd/wallet/pay?invoice=i-1'), 'https://sa.evinvest.ltd/billing?x=1');

    expect(href).toBe('https://evinvest.ltd/wallet/pay?invoice=i-1&return_to=https%3A%2F%2Fsa.evinvest.ltd%2Fbilling%3Fx%3D1');
  });
});

describe('parseWalletMessage: what passes', () => {
  it('should_read_a_done_result_with_a_reference', () => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'done', reference: 'payout-1' })).toEqual({
      outcome: 'done',
      reference: 'payout-1',
    });
  });

  it.each(['failed', 'cancelled'])('should_read_the_%s_outcome', outcome => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome })).toEqual({ outcome, reference: null });
  });

  it('should_read_a_null_reference_as_null', () => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'done', reference: null })).toEqual({ outcome: 'done', reference: null });
  });

  it('should_accept_a_reference_of_exactly_128_characters', () => {
    const reference = 'r'.repeat(128);

    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'done', reference })).toEqual({ outcome: 'done', reference });
  });

  // The outcome is what matters — money may already have moved — so an unusable
  // reference reads as none instead of dropping the whole result.
  it.each([
    ['empty', ''],
    ['129 characters long', 'r'.repeat(129)],
    ['a number', 7],
    ['an object', { id: 'p-1' }],
  ])('should_keep_the_outcome_and_drop_a_reference_that_is_%s', (_label, reference) => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'done', reference })).toEqual({ outcome: 'done', reference: null });
  });

  it('should_keep_a_long_reference_out_rather_than_cut_it', () => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'failed', reference: 'p'.repeat(200) })).toEqual({
      outcome: 'failed',
      reference: null,
    });
  });

  it('should_ignore_unknown_fields', () => {
    expect(parseWalletMessage({ type: 'ev:wallet-result', outcome: 'done', reference: 'p-1', amount: 5 })).toStrictEqual({
      outcome: 'done',
      reference: 'p-1',
    });
  });
});

describe('parseWalletMessage: what is ignored', () => {
  it.each([
    ['another type', { type: 'ev:wallet-progress', outcome: 'done' }],
    ['no type', { outcome: 'done' }],
    ['an unknown outcome', { type: 'ev:wallet-result', outcome: 'success' }],
    ['no outcome', { type: 'ev:wallet-result' }],
  ])('should_return_null_for_%s', (_label, data) => {
    expect(parseWalletMessage(data)).toBeNull();
  });

  it.each([null, undefined, 'ev:wallet-result', 1])('should_return_null_for_a_non_object: %j', data => {
    expect(parseWalletMessage(data)).toBeNull();
  });
});
