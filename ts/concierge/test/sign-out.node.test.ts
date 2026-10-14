// The platform sign-out page lives on evinvest.ltd, so the address it sends
// the user back to crosses an origin: query and hash stay behind (they can
// carry tokens or ids), and an address that is not absolute https is left for
// concierge to decide rather than forwarded.
import { describe, expect, it } from 'vitest';

import { PLATFORM_SIGN_OUT_URL, platformSignOutHref } from '../src/index';

describe('platformSignOutHref', () => {
  it('should_point_at_the_concierge_confirm_page_on_evinvest_ltd', () => {
    expect(PLATFORM_SIGN_OUT_URL).toBe('https://evinvest.ltd/api/auth/logout/confirm');
  });

  it('should_carry_an_https_host_page_back_encoded', () => {
    expect(platformSignOutHref('https://invest.evinvest.ltd/funds')).toBe(
      'https://evinvest.ltd/api/auth/logout/confirm?return_to=https%3A%2F%2Finvest.evinvest.ltd%2Ffunds',
    );
  });

  it('should_carry_the_host_root_back_as_the_wrappers_send_it', () => {
    expect(platformSignOutHref('https://invest.evinvest.ltd/')).toBe(
      'https://evinvest.ltd/api/auth/logout/confirm?return_to=https%3A%2F%2Finvest.evinvest.ltd%2F',
    );
  });

  it('should_drop_the_query_and_hash_of_the_host_page', () => {
    expect(platformSignOutHref('https://invest.evinvest.ltd/funds?token=secret#row-7')).toBe(
      'https://evinvest.ltd/api/auth/logout/confirm?return_to=https%3A%2F%2Finvest.evinvest.ltd%2Ffunds',
    );
  });

  it('should_drop_userinfo_from_the_host_page', () => {
    expect(platformSignOutHref('https://user:pass@invest.evinvest.ltd/funds')).toBe(
      'https://evinvest.ltd/api/auth/logout/confirm?return_to=https%3A%2F%2Finvest.evinvest.ltd%2Ffunds',
    );
  });

  it('should_keep_plain_http_on_loopback_for_a_dev_stand', () => {
    expect(platformSignOutHref('http://localhost:3000/account')).toBe(
      'https://evinvest.ltd/api/auth/logout/confirm?return_to=http%3A%2F%2Flocalhost%3A3000%2Faccount',
    );
  });

  it.each([
    ['plain http to a real host', 'http://invest.evinvest.ltd/funds'],
    ['root-relative', '/funds'],
    ['protocol-relative', '//evil.example/x'],
    ['javascript', 'javascript:alert(1)'],
    ['empty', ''],
  ])('should_leave_return_to_out_for_a_%s_value', (_label, raw) => {
    expect(platformSignOutHref(raw)).toBe('https://evinvest.ltd/api/auth/logout/confirm');
  });
});
