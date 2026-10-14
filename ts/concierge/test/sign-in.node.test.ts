// `returnTo` ends up as the redirect target of the host's RP login: anything a
// browser would resolve off the host's origin collapses to `/`.
import { describe, expect, it } from 'vitest';

import { rootRelative, rpLoginHref } from '../src/index';

describe('rpLoginHref', () => {
  it('should_carry_a_root_relative_path_back_encoded', () => {
    expect(rpLoginHref('/invest/funds')).toBe('/auth/login?return_to=%2Finvest%2Ffunds');
  });

  it('should_encode_the_query_and_hash_of_the_return_path', () => {
    expect(rpLoginHref('/x?a=1&b=two words#top')).toBe('/auth/login?return_to=%2Fx%3Fa%3D1%26b%3Dtwo%20words%23top');
  });

  it('should_use_the_given_login_path', () => {
    expect(rpLoginHref('/x', '/rp/login')).toBe('/rp/login?return_to=%2Fx');
  });

  it('should_fall_back_to_root_for_a_protocol_relative_url', () => {
    expect(rpLoginHref('//evil.example/steal')).toBe('/auth/login?return_to=%2F');
  });

  it('should_fall_back_to_root_for_a_backslash_after_the_slash', () => {
    expect(rpLoginHref('/\\evil.example')).toBe('/auth/login?return_to=%2F');
  });

  it('should_fall_back_to_root_for_a_backslash_anywhere_in_the_path', () => {
    expect(rpLoginHref('/x\\..\\y')).toBe('/auth/login?return_to=%2F');
  });

  it.each([
    ['tab', '/\t/evil.example'],
    ['newline', '/\n/evil.example'],
    ['carriage return', '/x\r'],
    ['nul', '/x\u0000'],
    ['delete', '/x\u007F'],
  ])('should_fall_back_to_root_for_a_%s_in_the_path', (_label, returnTo) => {
    expect(rpLoginHref(returnTo)).toBe('/auth/login?return_to=%2F');
  });

  it('should_fall_back_to_root_for_an_absolute_url', () => {
    expect(rpLoginHref('https://evil.example/x')).toBe('/auth/login?return_to=%2F');
  });

  it('should_fall_back_to_root_for_a_javascript_url', () => {
    expect(rpLoginHref('javascript:alert(1)')).toBe('/auth/login?return_to=%2F');
  });

  it('should_fall_back_to_root_for_a_bare_word_and_an_empty_string', () => {
    expect(rpLoginHref('dashboard')).toBe('/auth/login?return_to=%2F');
    expect(rpLoginHref('')).toBe('/auth/login?return_to=%2F');
  });
});

describe('rootRelative', () => {
  it('should_return_a_root_relative_path_unchanged', () => {
    expect(rootRelative('/a/b?c=d#e')).toBe('/a/b?c=d#e');
  });

  it('should_return_null_for_a_protocol_relative_url', () => {
    expect(rootRelative('//evil.example')).toBeNull();
  });
});
