// The session body comes from evinvest.ltd across an origin boundary, so the
// parser is the only thing standing between a changed or broken backend and a
// host rendering "signed in as undefined". A mistyped known field fails the
// whole body; unknown fields are dropped.
import { afterEach, describe, expect, it, vi } from 'vitest';

import { fetchSession, parseSession, SESSION_URL } from '../src/index';

const USER = {
  userId: 'u-1',
  email: 'valentina@example.com',
  emailVerified: true,
  username: 'valentina',
  status: 'active',
  role: 'investor',
  isAdmin: false,
  roleIsBreakGlass: false,
} as const;

const SIGNED_IN = { authenticated: true, user: USER, permissions: ['wallet.read'] } as const;

describe('parseSession: what passes', () => {
  it('should_accept_a_guest_with_a_null_user', () => {
    expect(parseSession({ authenticated: false, user: null, permissions: [] })).toEqual({
      ok: true,
      value: { authenticated: false, user: null, permissions: [] },
    });
  });

  it('should_read_a_guest_user_as_null_even_when_the_body_carries_one', () => {
    const result = parseSession({ authenticated: false, user: USER, permissions: ['public.read'] });

    expect(result).toEqual({ ok: true, value: { authenticated: false, user: null, permissions: ['public.read'] } });
  });

  it('should_accept_a_signed_in_user', () => {
    expect(parseSession(SIGNED_IN)).toEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_accept_a_user_whose_username_is_null', () => {
    const result = parseSession({ ...SIGNED_IN, user: { ...USER, username: null } });

    expect(result).toEqual({ ok: true, value: { ...SIGNED_IN, user: { ...USER, username: null } } });
  });

  it('should_read_an_absent_username_key_as_null', () => {
    const { username: _omitted, ...withoutUsername } = USER;

    const result = parseSession({ ...SIGNED_IN, user: withoutUsername });

    expect(result).toEqual({ ok: true, value: { ...SIGNED_IN, user: { ...USER, username: null } } });
  });

  it('should_drop_unknown_fields_at_the_top_level_and_in_the_user', () => {
    const result = parseSession({
      ...SIGNED_IN,
      csrf: 'leaked',
      user: { ...USER, passwordHash: 'x', sessionId: 's-1' },
    });

    expect(result).toStrictEqual({
      ok: true,
      value: {
        authenticated: true,
        user: {
          userId: 'u-1',
          email: 'valentina@example.com',
          emailVerified: true,
          username: 'valentina',
          status: 'active',
          role: 'investor',
          isAdmin: false,
          roleIsBreakGlass: false,
        },
        permissions: ['wallet.read'],
      },
    });
  });
});

describe('parseSession: what fails', () => {
  it.each([null, undefined, 'signed-in', 42])('should_reject_a_body_that_is_not_an_object: %j', body => {
    expect(parseSession(body)).toEqual({ ok: false, error: 'session is not an object' });
  });

  it.each(['true', 1, null, undefined])('should_reject_authenticated_that_is_not_a_boolean: %j', authenticated => {
    expect(parseSession({ ...SIGNED_IN, authenticated })).toEqual({
      ok: false,
      error: 'session `authenticated` is not a boolean',
    });
  });

  it.each([
    ['a string', 'wallet.read'],
    ['missing', undefined],
    ['null', null],
    ['an object', { 0: 'wallet.read' }],
    ['an array holding a number', ['wallet.read', 7]],
    ['an array holding null', [null]],
  ])('should_reject_permissions_that_are_%s', (_label, permissions) => {
    expect(parseSession({ ...SIGNED_IN, permissions })).toEqual({
      ok: false,
      error: 'session `permissions` is not a string array',
    });
  });

  it('should_reject_a_guest_whose_permissions_are_not_a_string_array', () => {
    expect(parseSession({ authenticated: false, user: null, permissions: 'none' }).ok).toBe(false);
  });

  it.each([null, undefined, 'u-1', []])('should_reject_a_signed_in_body_whose_user_is_%j', user => {
    expect(parseSession({ ...SIGNED_IN, user })).toEqual({ ok: false, error: 'session `user` is missing or malformed' });
  });

  it.each([
    ['userId', 1],
    ['userId', ''],
    ['userId', undefined],
    ['email', null],
    ['email', undefined],
    ['emailVerified', 'true'],
    ['emailVerified', undefined],
    ['username', 7],
    ['username', false],
    ['status', 0],
    ['status', undefined],
    ['role', null],
    ['role', undefined],
    ['isAdmin', 'false'],
    ['isAdmin', undefined],
    ['roleIsBreakGlass', 0],
    ['roleIsBreakGlass', undefined],
  ])('should_reject_a_user_whose_%s_is_%j', (field, value) => {
    expect(parseSession({ ...SIGNED_IN, user: { ...USER, [field]: value } })).toEqual({
      ok: false,
      error: 'session `user` is missing or malformed',
    });
  });
});

describe('fetchSession', () => {
  afterEach(() => {
    vi.unstubAllGlobals();
  });

  function stubFetch(answer: () => Promise<Response>) {
    const fetchMock = vi.fn<typeof fetch>(answer);
    vi.stubGlobal('fetch', fetchMock);
    return fetchMock;
  }

  function requestOf(fetchMock: ReturnType<typeof stubFetch>): { url: unknown; init: RequestInit | undefined } {
    const [url, init] = fetchMock.mock.calls[0] ?? [];
    return { url, init };
  }

  it('should_parse_a_2xx_body_from_the_default_url', async () => {
    const fetchMock = stubFetch(async () => Response.json(SIGNED_IN));

    const result = await fetchSession();

    expect(result).toEqual({ ok: true, value: SIGNED_IN });
    expect(requestOf(fetchMock).url).toBe('https://evinvest.ltd/api/auth/session');
    expect(SESSION_URL).toBe('https://evinvest.ltd/api/auth/session');
  });

  it('should_send_cookies_cross_origin_and_skip_the_http_cache', async () => {
    const fetchMock = stubFetch(async () => Response.json(SIGNED_IN));

    await fetchSession();

    expect(requestOf(fetchMock).init?.credentials).toBe('include');
    expect(requestOf(fetchMock).init?.cache).toBe('no-store');
  });

  it('should_ask_for_json_when_the_caller_sets_no_accept', async () => {
    const fetchMock = stubFetch(async () => Response.json(SIGNED_IN));

    await fetchSession(SESSION_URL, { headers: { 'x-trace': 't-1' } });

    const headers = new Headers(requestOf(fetchMock).init?.headers);
    expect(headers.get('accept')).toBe('application/json');
    expect(headers.get('x-trace')).toBe('t-1');
  });

  it('should_keep_the_accept_header_the_caller_passed', async () => {
    const fetchMock = stubFetch(async () => Response.json(SIGNED_IN));

    await fetchSession(SESSION_URL, { headers: { Accept: 'application/vnd.ev.session+json' } });

    expect(new Headers(requestOf(fetchMock).init?.headers).get('accept')).toBe('application/vnd.ev.session+json');
  });

  it('should_return_a_failure_for_a_non_2xx_status', async () => {
    stubFetch(async () => new Response('down', { status: 503 }));

    await expect(fetchSession()).resolves.toEqual({
      ok: false,
      error: 'session https://evinvest.ltd/api/auth/session: HTTP 503',
    });
  });

  it('should_return_a_failure_for_a_401_instead_of_reading_its_body', async () => {
    stubFetch(async () => Response.json({ authenticated: false, user: null, permissions: [] }, { status: 401 }));

    await expect(fetchSession()).resolves.toEqual({
      ok: false,
      error: 'session https://evinvest.ltd/api/auth/session: HTTP 401',
    });
  });

  it('should_return_a_failure_instead_of_throwing_on_a_network_error', async () => {
    stubFetch(async () => {
      throw new TypeError('fetch failed');
    });

    await expect(fetchSession()).resolves.toEqual({
      ok: false,
      error: 'session https://evinvest.ltd/api/auth/session: fetch failed',
    });
  });

  it('should_return_a_failure_instead_of_throwing_on_a_body_that_is_not_json', async () => {
    stubFetch(async () => new Response('<html>maintenance</html>', { status: 200 }));

    const result = await fetchSession();

    expect(result.ok).toBe(false);
  });

  it('should_return_a_failure_for_a_json_body_of_the_wrong_shape', async () => {
    stubFetch(async () => Response.json({ authenticated: 'yes' }));

    await expect(fetchSession()).resolves.toEqual({ ok: false, error: 'session `authenticated` is not a boolean' });
  });
});
