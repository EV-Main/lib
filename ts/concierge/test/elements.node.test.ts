// The wrappers mount the tag they know, so a registry entry is usable only
// when its name is there and points at that same tag.
import type { MfeEntry } from '@evinvest/mfe';
import { describe, expect, it } from 'vitest';

import { conciergeEntry, CONCIERGE_REGISTRY_URL } from '../src/index';

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
});
