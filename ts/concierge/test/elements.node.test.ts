// The wrappers mount the tag they know, so a registry entry is usable only
// when its name is there and points at that same tag.
import type { MfeEntry } from '@evinvest/mfe';
import { afterEach, describe, expect, it, vi } from 'vitest';

import { conciergeEntry, CONCIERGE_MFE_PREFIX, CONCIERGE_REGISTRY_URL, fetchConciergeRegistry } from '../src/index';

const SIGN_IN: MfeEntry = {
  name: 'concierge.sign-in',
  tag: 'mfe-concierge-sign-in',
  kind: 'component',
  scriptUrl: '/auth/mfe/sign-in.js',
};

const ACCOUNT_CHIP: MfeEntry = {
  name: 'concierge.account-chip',
  tag: 'mfe-concierge-account-chip',
  kind: 'component',
  scriptUrl: '/auth/mfe/account-chip.js',
};

describe('conciergeEntry', () => {
  it('should_return_the_entry_whose_name_and_tag_match', () => {
    expect(conciergeEntry([ACCOUNT_CHIP, SIGN_IN], 'signIn')).toEqual({ ok: true, value: SIGN_IN });
  });

  it('should_refuse_when_the_registry_has_no_such_entry', () => {
    expect(conciergeEntry([ACCOUNT_CHIP], 'signIn')).toEqual({ ok: false, error: 'registry has no concierge.sign-in' });
  });

  it('should_refuse_an_empty_registry', () => {
    expect(conciergeEntry([], 'account').ok).toBe(false);
  });

  it('should_refuse_an_entry_whose_tag_differs_from_the_wrapper_tag', () => {
    const renamed: MfeEntry = { ...SIGN_IN, tag: 'mfe-concierge-sign-in-v3' };

    expect(conciergeEntry([renamed], 'signIn')).toEqual({
      ok: false,
      error: 'concierge.sign-in: tag mfe-concierge-sign-in-v3, expected mfe-concierge-sign-in',
    });
  });
});

describe('registry location', () => {
  it('should_point_at_the_concierge_registry_on_evinvest_ltd', () => {
    expect(CONCIERGE_REGISTRY_URL).toBe('https://evinvest.ltd/api/auth/mfe/registry.json');
  });

  it('should_hold_bundles_to_the_concierge_mfe_directory', () => {
    expect(CONCIERGE_MFE_PREFIX).toBe('https://evinvest.ltd/api/auth/mfe/');
  });
});

describe('fetchConciergeRegistry', () => {
  // As concierge publishes it: bundles next to the registry.
  const PUBLISHED: MfeEntry = { ...SIGN_IN, scriptUrl: '/api/auth/mfe/sign-in.js' };

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('should_resolve_bundles_inside_the_concierge_mfe_directory', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([PUBLISHED])));

    const result = await fetchConciergeRegistry();

    expect(result).toEqual({ ok: true, value: [{ ...PUBLISHED, scriptUrl: 'https://evinvest.ltd/api/auth/mfe/sign-in.js' }] });
  });

  it('should_refuse_a_registry_with_a_bundle_elsewhere_on_evinvest_ltd', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([{ ...PUBLISHED, scriptUrl: '/uploads/sign-in.js' }])));

    const result = await fetchConciergeRegistry();

    expect(result).toEqual({
      ok: false,
      error: `registry ${CONCIERGE_REGISTRY_URL}: entry 0: concierge.sign-in: scriptUrl https://evinvest.ltd/uploads/sign-in.js is outside https://evinvest.ltd/api/auth/mfe/`,
    });
  });

  it('should_refuse_redirects_and_pass_the_init_through', async () => {
    const fetchMock = vi.fn(async (_url: string | URL | Request, _init?: RequestInit) => Response.json([]));
    vi.stubGlobal('fetch', fetchMock);

    await fetchConciergeRegistry(CONCIERGE_REGISTRY_URL, { init: { cache: 'no-store' } });

    expect(fetchMock).toHaveBeenCalledWith(new URL(CONCIERGE_REGISTRY_URL), { cache: 'no-store', redirect: 'error' });
  });

  it('should_hold_a_staging_registry_to_the_prefix_the_caller_names', async () => {
    const staging = 'https://staging.evinvest.ltd/api/auth/mfe/';
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([PUBLISHED])));

    const result = await fetchConciergeRegistry(`${staging}registry.json`, { allowedPrefix: staging });

    expect(result).toEqual({ ok: true, value: [{ ...PUBLISHED, scriptUrl: `${staging}sign-in.js` }] });
  });

  it('should_refuse_a_staging_registry_held_to_the_production_prefix', async () => {
    vi.stubGlobal('fetch', vi.fn(async () => Response.json([PUBLISHED])));

    const result = await fetchConciergeRegistry('https://staging.evinvest.ltd/api/auth/mfe/registry.json');

    expect(result.ok).toBe(false);
  });
});
