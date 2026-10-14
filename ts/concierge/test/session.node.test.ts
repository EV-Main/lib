// The session body comes from evinvest.ltd across an origin boundary, so the
// parser is the only thing standing between a changed or broken backend and a
// host rendering "signed in as undefined". A mistyped kept field fails the
// whole body; everything else — platform role, admin and break-glass flags,
// the seat's permissions — is dropped, so a host never holds platform
// privileges it could be tempted to gate its own UI on.
import { afterEach, describe, expect, expectTypeOf, it, vi } from 'vitest';

import { fetchSession, parseSession, SESSION_URL, type Session, type SessionUser } from '../src/index';

/** What concierge's `SessionInfo` sends today, platform fields included. */
const WIRE_USER = {
  userId: 'u-1',
  email: 'valentina@example.com',
  emailVerified: true,
  username: 'valentina',
  status: 'active',
  role: 'investor',
  isAdmin: false,
  roleIsBreakGlass: false,
} as const;

const WIRE_SIGNED_IN = { authenticated: true, user: WIRE_USER, permissions: ['wallet.read'] } as const;

const USER: SessionUser = {
  userId: 'u-1',
  email: 'valentina@example.com',
  emailVerified: true,
  username: 'valentina',
  displayName: null,
};

const SIGNED_IN: Session = { authenticated: true, user: USER };

describe('SessionUser', () => {
  it('should_carry_only_the_five_fields_a_chip_shows', () => {
    expectTypeOf<keyof SessionUser>().toEqualTypeOf<'userId' | 'email' | 'emailVerified' | 'username' | 'displayName'>();
  });

  it('should_carry_no_permissions_on_the_session', () => {
    expectTypeOf<keyof Session>().toEqualTypeOf<'authenticated' | 'user'>();
  });
});

describe('parseSession: what passes', () => {
  it('should_accept_a_guest_with_a_null_user', () => {
    expect(parseSession({ authenticated: false, user: null, permissions: [] })).toStrictEqual({
      ok: true,
      value: { authenticated: false, user: null },
    });
  });

  it('should_read_a_guest_user_as_null_even_when_the_body_carries_one', () => {
    const result = parseSession({ authenticated: false, user: WIRE_USER, permissions: ['public.read'] });

    expect(result).toStrictEqual({ ok: true, value: { authenticated: false, user: null } });
  });

  it('should_accept_a_guest_body_without_permissions', () => {
    expect(parseSession({ authenticated: false, user: null })).toStrictEqual({
      ok: true,
      value: { authenticated: false, user: null },
    });
  });

  it('should_accept_a_signed_in_user_and_keep_only_the_five_chip_fields', () => {
    expect(parseSession(WIRE_SIGNED_IN)).toStrictEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_drop_permissions_role_isAdmin_and_roleIsBreakGlass', () => {
    const result = parseSession({
      authenticated: true,
      user: { ...WIRE_USER, role: 'admin', isAdmin: true, roleIsBreakGlass: true },
      permissions: ['platform.admin'],
    });

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_drop_unknown_fields_at_the_top_level_and_in_the_user', () => {
    const result = parseSession({
      ...WIRE_SIGNED_IN,
      csrf: 'leaked',
      user: { ...WIRE_USER, passwordHash: 'x', sessionId: 's-1' },
    });

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_accept_a_user_whose_username_is_null', () => {
    const result = parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, username: null } });

    expect(result).toStrictEqual({ ok: true, value: { authenticated: true, user: { ...USER, username: null } } });
  });

  it('should_read_an_absent_username_key_as_null', () => {
    const { username: _omitted, ...withoutUsername } = WIRE_USER;

    const result = parseSession({ ...WIRE_SIGNED_IN, user: withoutUsername });

    expect(result).toStrictEqual({ ok: true, value: { authenticated: true, user: { ...USER, username: null } } });
  });

  it('should_keep_a_display_name_when_concierge_sends_one', () => {
    const result = parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, displayName: 'Valentina K.' } });

    expect(result).toStrictEqual({ ok: true, value: { authenticated: true, user: { ...USER, displayName: 'Valentina K.' } } });
  });

  it('should_read_an_empty_display_name_as_null', () => {
    const result = parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, displayName: '' } });

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_read_a_null_display_name_as_null', () => {
    const result = parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, displayName: null } });

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
  });

  it('should_ignore_a_mistyped_platform_field_now_that_it_is_dropped', () => {
    const result = parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, isAdmin: 'false', role: null }, permissions: 'none' });

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
  });
});

describe('parseSession: what fails', () => {
  it.each([null, undefined, 'signed-in', 42])('should_reject_a_body_that_is_not_an_object: %j', body => {
    expect(parseSession(body)).toEqual({ ok: false, error: 'session is not an object' });
  });

  it.each(['true', 1, null, undefined])('should_reject_authenticated_that_is_not_a_boolean: %j', authenticated => {
    expect(parseSession({ ...WIRE_SIGNED_IN, authenticated })).toEqual({
      ok: false,
      error: 'session `authenticated` is not a boolean',
    });
  });

  it.each([null, undefined, 'u-1', []])('should_reject_a_signed_in_body_whose_user_is_%j', user => {
    expect(parseSession({ ...WIRE_SIGNED_IN, user })).toEqual({ ok: false, error: 'session `user` is missing or malformed' });
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
    ['displayName', 7],
    ['displayName', false],
  ])('should_reject_a_user_whose_%s_is_%j', (field, value) => {
    expect(parseSession({ ...WIRE_SIGNED_IN, user: { ...WIRE_USER, [field]: value } })).toEqual({
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
    const fetchMock = stubFetch(async () => Response.json(WIRE_SIGNED_IN));

    const result = await fetchSession();

    expect(result).toStrictEqual({ ok: true, value: SIGNED_IN });
    expect(requestOf(fetchMock).url).toBe('https://evinvest.ltd/api/auth/session');
    expect(SESSION_URL).toBe('https://evinvest.ltd/api/auth/session');
  });

  it('should_send_cookies_cross_origin_and_skip_the_http_cache', async () => {
    const fetchMock = stubFetch(async () => Response.json(WIRE_SIGNED_IN));

    await fetchSession();

    expect(requestOf(fetchMock).init?.credentials).toBe('include');
    expect(requestOf(fetchMock).init?.cache).toBe('no-store');
  });

  it('should_ask_for_json_when_the_caller_sets_no_accept', async () => {
    const fetchMock = stubFetch(async () => Response.json(WIRE_SIGNED_IN));

    await fetchSession(SESSION_URL, { headers: { 'x-trace': 't-1' } });

    const headers = new Headers(requestOf(fetchMock).init?.headers);
    expect(headers.get('accept')).toBe('application/json');
    expect(headers.get('x-trace')).toBe('t-1');
  });

  it('should_keep_the_accept_header_the_caller_passed', async () => {
    const fetchMock = stubFetch(async () => Response.json(WIRE_SIGNED_IN));

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
