// The wrappers mount the tag they know, so a registry entry is usable only
// when its name is there and points at that same tag.
import type { MfeEntry } from '@evinvest/mfe';
import { describe, expect, it } from 'vitest';

import { walletEntry, WALLET_REGISTRY_URL } from '../src/index';

const WALLET: MfeEntry = {
  name: 'banking.wallet',
  tag: 'mfe-banking-wallet',
  kind: 'component',
  scriptUrl: '/wallet/mfe/wallet.js',
};

const PAY_INVOICE: MfeEntry = {
  name: 'banking.pay-invoice',
  tag: 'mfe-banking-pay-invoice',
  kind: 'component',
  scriptUrl: '/wallet/mfe/pay-invoice.js',
};

describe('walletEntry', () => {
  it('should_return_the_entry_whose_name_and_tag_match', () => {
    expect(walletEntry([WALLET, PAY_INVOICE], 'payInvoice')).toEqual({ ok: true, value: PAY_INVOICE });
  });

  it('should_refuse_when_the_registry_has_no_such_entry', () => {
    expect(walletEntry([PAY_INVOICE], 'wallet')).toEqual({ ok: false, error: 'registry has no banking.wallet' });
  });

  it('should_refuse_an_entry_whose_tag_differs_from_the_wrapper_tag', () => {
    const renamed: MfeEntry = { ...WALLET, tag: 'mfe-banking-wallet-next' };

    expect(walletEntry([renamed], 'wallet')).toEqual({
      ok: false,
      error: 'banking.wallet: tag mfe-banking-wallet-next, expected mfe-banking-wallet',
    });
  });
});

describe('registry location', () => {
  it('should_point_at_the_wallet_registry_on_evinvest_ltd', () => {
    expect(WALLET_REGISTRY_URL).toBe('https://evinvest.ltd/wallet/mfe/registry.json');
  });
});
