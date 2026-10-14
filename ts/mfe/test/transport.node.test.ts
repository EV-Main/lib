// Plain http is tolerated only where nobody sits on the path: a dev stand on
// loopback. Everywhere else a bundle, link or return address over http can be
// rewritten in flight, so it is refused like any non-web scheme.
import { describe, expect, it } from 'vitest';

import { isSecureTransport } from '../src/index';

describe('isSecureTransport', () => {
  it('should_accept_https_to_any_host', () => {
    expect(isSecureTransport(new URL('https://evinvest.ltd/x'))).toBe(true);
  });

  it('should_accept_plain_http_to_localhost', () => {
    expect(isSecureTransport(new URL('http://localhost:3000/'))).toBe(true);
  });

  it('should_accept_plain_http_to_ipv4_loopback', () => {
    expect(isSecureTransport(new URL('http://127.0.0.1:8080/'))).toBe(true);
  });

  it('should_accept_plain_http_to_ipv6_loopback', () => {
    expect(isSecureTransport(new URL('http://[::1]:8080/'))).toBe(true);
  });

  it('should_refuse_plain_http_to_a_real_host', () => {
    expect(isSecureTransport(new URL('http://evinvest.ltd/'))).toBe(false);
  });

  it('should_refuse_a_host_that_only_starts_with_localhost', () => {
    expect(isSecureTransport(new URL('http://localhost.evil.example/'))).toBe(false);
  });

  it('should_refuse_javascript_and_data_urls', () => {
    expect(isSecureTransport(new URL('javascript:alert(1)'))).toBe(false);
    expect(isSecureTransport(new URL('data:text/html,x'))).toBe(false);
  });
});
