// The registry is read from a producer the host does not build, so every entry
// is parsed: all or nothing, and no bundle URL that leaves the producer's
// control (`javascript:`, `data:`, protocol-relative, plain http off loopback)
// ever reaches a loader; a fetched registry is also held to one directory.
import { afterEach, describe, expect, it, vi } from 'vitest';

import { entryFor, fetchRegistry, parseRegistry, resolveEntries, type MfeEntry } from '../src/index';

const CHIP = {
  name: 'concierge.account-chip',
  tag: 'ev-account-chip',
  kind: 'component',
  scriptUrl: '/auth/mfe/account-chip.js',
} as const;

const REGISTRY_URL = 'https://evinvest.ltd/auth/mfe/registry.json';

describe('parseRegistry: what passes', () => {
  it('should_accept_a_valid_entry_as_written', () => {
    expect(parseRegistry([CHIP])).toEqual({ ok: true, value: [CHIP] });
  });

  it('should_accept_an_absolute_https_bundle_url', () => {
    const entry = { ...CHIP, scriptUrl: 'https://cdn.evinvest.ltd/mfe/chip.js' };
    expect(parseRegistry([entry])).toEqual({ ok: true, value: [entry] });
  });

  it('should_accept_a_plain_http_bundle_url_on_loopback_for_a_dev_stand', () => {
    const entry = { ...CHIP, scriptUrl: 'http://127.0.0.1:5173/mfe/chip.js' };
    expect(parseRegistry([entry])).toEqual({ ok: true, value: [entry] });
  });

  it('should_accept_an_empty_registry', () => {
    expect(parseRegistry([])).toEqual({ ok: true, value: [] });
  });

  it('should_drop_a_leftover_integrity_field_instead_of_failing', () => {
    const result = parseRegistry([{ ...CHIP, integrity: 'sha384-abc' }]);
    expect(result).toEqual({ ok: true, value: [CHIP] });
  });
});

describe('parseRegistry: what fails the whole registry', () => {
  it('should_reject_a_plain_http_bundle_url_on_a_real_host', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: 'http://cdn.evinvest.ltd/mfe/chip.js' }])).toEqual({
      ok: false,
      error: 'entry 0: concierge.account-chip: `scriptUrl` is not a root-relative or https URL',
    });
  });

  it('should_reject_an_entry_with_an_empty_name', () => {
    const result = parseRegistry([{ ...CHIP, name: '' }]);
    expect(result).toEqual({ ok: false, error: 'entry 0: manifest `name` is missing or empty' });
  });

  it('should_reject_a_tag_without_a_hyphen', () => {
    expect(parseRegistry([{ ...CHIP, tag: 'evchip' }]).ok).toBe(false);
  });

  it('should_reject_a_tag_with_uppercase_letters', () => {
    expect(parseRegistry([{ ...CHIP, tag: 'ev-Account-chip' }]).ok).toBe(false);
  });

  it('should_reject_a_kind_outside_component_and_page', () => {
    const result = parseRegistry([{ ...CHIP, kind: 'widget' }]);
    expect(result).toEqual({
      ok: false,
      error: 'entry 0: concierge.account-chip: `kind` is not "component" or "page"',
    });
  });

  it('should_reject_a_javascript_bundle_url', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: 'javascript:alert(1)' }]).ok).toBe(false);
  });

  it('should_reject_a_data_bundle_url', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: 'data:text/javascript,alert(1)' }]).ok).toBe(false);
  });

  it('should_reject_a_protocol_relative_bundle_url', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: '//evil.example/x.js' }]).ok).toBe(false);
  });

  it('should_reject_a_backslash_bundle_url_that_browsers_read_as_protocol_relative', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: '/\\evil.example/x.js' }]).ok).toBe(false);
  });

  it('should_reject_a_tab_inside_a_bundle_url', () => {
    expect(parseRegistry([{ ...CHIP, scriptUrl: '/\t/evil.example/x.js' }]).ok).toBe(false);
  });

  it('should_reject_a_duplicate_name_and_name_the_second_entry', () => {
    const other = { ...CHIP, tag: 'ev-account-chip-two' };
    expect(parseRegistry([CHIP, other])).toEqual({
      ok: false,
      error: 'entry 1: duplicate name concierge.account-chip',
    });
  });

  it('should_reject_a_duplicate_tag_and_name_the_second_entry', () => {
    const other = { ...CHIP, name: 'banking.account-chip' };
    expect(parseRegistry([CHIP, other])).toEqual({
      ok: false,
      error: 'entry 1: duplicate tag ev-account-chip',
    });
  });

  it('should_fail_all_entries_when_one_is_bad', () => {
    const good = { ...CHIP, name: 'banking.wallet', tag: 'ev-wallet' };
    expect(parseRegistry([good, { ...CHIP, kind: 'widget' }]).ok).toBe(false);
  });

  it('should_reject_a_registry_that_is_not_an_array', () => {
    expect(parseRegistry({ entries: [CHIP] })).toEqual({ ok: false, error: 'registry is not an array' });
    expect(parseRegistry(null)).toEqual({ ok: false, error: 'registry is not an array' });
  });

  it('should_reject_a_null_entry_without_throwing', () => {
    expect(parseRegistry([null])).toEqual({ ok: false, error: 'entry 0: manifest is not an object' });
  });
});

describe('entryFor', () => {
  const SIGN_IN: MfeEntry = { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in', kind: 'component', scriptUrl: '/auth/mfe/sign-in.js' };

  it('should_return_the_entry_whose_name_and_tag_match', () => {
    expect(entryFor([CHIP, SIGN_IN], { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in' })).toEqual({ ok: true, value: SIGN_IN });
  });

  it('should_refuse_when_the_registry_has_no_such_entry', () => {
    expect(entryFor([CHIP], { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in' })).toEqual({
      ok: false,
      error: 'registry has no concierge.sign-in',
    });
  });

  it('should_refuse_an_empty_registry', () => {
    expect(entryFor([], { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in' }).ok).toBe(false);
  });

  it('should_refuse_an_entry_whose_tag_differs_from_the_known_tag', () => {
    expect(entryFor([SIGN_IN], { name: 'concierge.sign-in', tag: 'mfe-concierge-sign-in-v3' })).toEqual({
      ok: false,
      error: 'concierge.sign-in: tag mfe-concierge-sign-in, expected mfe-concierge-sign-in-v3',
    });
  });
});

describe('resolveEntries', () => {
  const entry: MfeEntry = { ...CHIP };

  it('should_resolve_a_root_relative_url_against_the_registry_url', () => {
    const [resolved] = resolveEntries([entry], REGISTRY_URL);
    expect(resolved?.scriptUrl).toBe('https://evinvest.ltd/auth/mfe/account-chip.js');
  });

  it('should_leave_an_absolute_url_unchanged', () => {
    const absolute = { ...entry, scriptUrl: 'https://cdn.example/mfe/chip.js?v=3' };
    const [resolved] = resolveEntries([absolute], REGISTRY_URL);
    expect(resolved?.scriptUrl).toBe('https://cdn.example/mfe/chip.js?v=3');
  });

  it('should_keep_every_other_field_of_the_entry', () => {
    const [resolved] = resolveEntries([entry], REGISTRY_URL);
    expect(resolved).toEqual({ ...entry, scriptUrl: 'https://evinvest.ltd/auth/mfe/account-chip.js' });
  });
});

describe('fetchRegistry', () => {
  // The directory the host's CSP names; every resolved bundle must sit under it.
  const PREFIX = 'https://evinvest.ltd/auth/mfe/';
  const OPTIONS = { allowedPrefix: PREFIX };

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('should_return_entries_with_urls_resolved_against_the_registry_url', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({
      ok: true,
      value: [{ ...CHIP, scriptUrl: 'https://evinvest.ltd/auth/mfe/account-chip.js' }],
    });
  });

  it('should_return_a_failure_on_http_500_without_throwing', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => new Response('oops', { status: 500 })));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: HTTP 500` });
  });

  it('should_return_a_failure_on_a_network_error_without_throwing', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        throw new TypeError('fetch failed');
      }),
    );

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: fetch failed` });
  });

  it('should_return_a_failure_on_invalid_json_without_throwing', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => new Response('<!doctype html><title>404</title>', { status: 200 })));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_return_the_parse_failure_for_a_well_formed_but_invalid_registry', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json({ entries: [] })));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({ ok: false, error: 'registry is not an array' });
  });

  it('should_return_a_failure_for_a_relative_registry_url_without_rejecting', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry('/auth/mfe/registry.json', OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_pass_the_request_init_through_to_fetch_with_redirects_refused', async () => {
    const fetchMock = vi.fn(async (_url: string | URL | Request, _init?: RequestInit) => Response.json([]));
    vi.stubGlobal('fetch', fetchMock);

    await fetchRegistry(REGISTRY_URL, { ...OPTIONS, init: { cache: 'no-store', redirect: 'follow' } });

    expect(fetchMock).toHaveBeenCalledWith(new URL(REGISTRY_URL), { cache: 'no-store', redirect: 'error' });
  });

  it('should_refuse_redirects_even_without_a_request_init', async () => {
    const fetchMock = vi.fn(async (_url: string | URL | Request, _init?: RequestInit) => Response.json([]));
    vi.stubGlobal('fetch', fetchMock);

    await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(fetchMock).toHaveBeenCalledWith(new URL(REGISTRY_URL), { redirect: 'error' });
  });

  it('should_return_a_failure_when_fetch_rejects_a_redirect', async () => {
    // What `redirect: 'error'` does in a real fetch when the server answers 3xx.
    vi.stubGlobal(
      'fetch',
      vi.fn(async () => {
        throw new TypeError('unexpected redirect');
      }),
    );

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: unexpected redirect` });
  });

  it('should_return_a_failure_when_the_response_was_served_from_another_origin', async () => {
    const moved = Response.json([CHIP]);
    Object.defineProperty(moved, 'url', { value: 'https://evil.example/auth/mfe/registry.json' });
    vi.stubGlobal('fetch', vi.fn(async () => moved));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({
      ok: false,
      error: `registry ${REGISTRY_URL}: served from another origin, https://evil.example/auth/mfe/registry.json`,
    });
  });

  it('should_accept_a_response_url_on_the_requested_origin', async () => {
    const served = Response.json([CHIP]);
    Object.defineProperty(served, 'url', { value: REGISTRY_URL });
    vi.stubGlobal('fetch', vi.fn(async () => served));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({
      ok: true,
      value: [{ ...CHIP, scriptUrl: 'https://evinvest.ltd/auth/mfe/account-chip.js' }],
    });
  });

  it('should_fail_the_whole_registry_when_one_bundle_is_outside_the_allowed_prefix', async () => {
    const upload = { ...CHIP, name: 'concierge.upload', tag: 'ev-upload', scriptUrl: '/uploads/x.js' };
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP, upload])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result).toEqual({
      ok: false,
      error: `registry ${REGISTRY_URL}: entry 1: concierge.upload: scriptUrl https://evinvest.ltd/uploads/x.js is outside ${PREFIX}`,
    });
  });

  it('should_refuse_a_sibling_directory_that_only_shares_the_prefix_text', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([{ ...CHIP, scriptUrl: '/auth/mfe-uploads/x.js' }])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_refuse_a_dot_dot_path_that_resolves_outside_the_prefix', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([{ ...CHIP, scriptUrl: '/auth/mfe/../uploads/x.js' }])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_refuse_an_encoded_slash_that_a_server_could_decode_into_a_traversal', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([{ ...CHIP, scriptUrl: '/auth/mfe/..%2F..%2Fuploads/x.js' }])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_refuse_an_absolute_bundle_on_another_origin', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([{ ...CHIP, scriptUrl: 'https://cdn.example/auth/mfe/x.js' }])));

    const result = await fetchRegistry(REGISTRY_URL, OPTIONS);

    expect(result.ok).toBe(false);
  });

  it('should_refuse_an_allowed_prefix_without_a_trailing_slash_before_fetching', async () => {
    const fetchMock = vi.fn(async () => Response.json([CHIP]));
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchRegistry(REGISTRY_URL, { allowedPrefix: 'https://evinvest.ltd/auth/mfe' });

    expect(result).toEqual({
      ok: false,
      error: `registry ${REGISTRY_URL}: allowedPrefix https://evinvest.ltd/auth/mfe is not a canonical directory URL ending in /`,
    });
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('should_refuse_a_call_without_options_before_fetching', async () => {
    const fetchMock = vi.fn(async () => Response.json([CHIP]));
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchRegistry(REGISTRY_URL);

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: allowedPrefix is required` });
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('should_refuse_empty_options_before_fetching', async () => {
    const fetchMock = vi.fn(async () => Response.json([CHIP]));
    vi.stubGlobal('fetch', fetchMock);

    // What a plain-JS host or a caller built against the old signature can pass.
    const result = await fetchRegistry(REGISTRY_URL, JSON.parse('{}'));

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: allowedPrefix is required` });
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('should_refuse_the_origin_root_as_allowed_prefix_before_fetching', async () => {
    const fetchMock = vi.fn(async () => Response.json([CHIP]));
    vi.stubGlobal('fetch', fetchMock);

    const result = await fetchRegistry(REGISTRY_URL, { allowedPrefix: 'https://evinvest.ltd/' });

    expect(result).toEqual({
      ok: false,
      error: `registry ${REGISTRY_URL}: allowedPrefix https://evinvest.ltd/ is the origin root, not a bundle directory`,
    });
    expect(fetchMock).not.toHaveBeenCalled();
  });

  it('should_refuse_a_relative_allowed_prefix', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry(REGISTRY_URL, { allowedPrefix: '/auth/mfe/' });

    expect(result).toEqual({ ok: false, error: `registry ${REGISTRY_URL}: allowedPrefix /auth/mfe/ is not an absolute URL` });
  });

  it('should_refuse_a_non_canonical_allowed_prefix', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry(REGISTRY_URL, { allowedPrefix: 'https://EVINVEST.ltd/auth/mfe/' });

    expect(result.ok).toBe(false);
  });

  it('should_refuse_a_plain_http_allowed_prefix_on_a_real_host', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry('http://evinvest.ltd/auth/mfe/registry.json', { allowedPrefix: 'http://evinvest.ltd/auth/mfe/' });

    expect(result).toEqual({
      ok: false,
      error: 'registry http://evinvest.ltd/auth/mfe/registry.json: allowedPrefix http://evinvest.ltd/auth/mfe/ is not https',
    });
  });

  it('should_accept_a_plain_http_allowed_prefix_on_loopback_for_a_dev_stand', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([CHIP])));

    const result = await fetchRegistry('http://localhost:8080/auth/mfe/registry.json', {
      allowedPrefix: 'http://localhost:8080/auth/mfe/',
    });

    expect(result).toEqual({
      ok: true,
      value: [{ ...CHIP, scriptUrl: 'http://localhost:8080/auth/mfe/account-chip.js' }],
    });
  });
});
