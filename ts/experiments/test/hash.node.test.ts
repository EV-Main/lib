import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  applyOverrides,
  fnv1a32,
  forcedVariant,
  hashRng,
  hashToUnit,
  pickVariant,
  pickVariantFor,
  resolveVariant,
  type ExperimentConfig,
  type ExperimentSpec,
} from '../src/index';

// Shared with ev_lib::experiments (rust/tests/experiments.rs).
const HASH = JSON.parse(readFileSync(join(import.meta.dirname, '../../../tests/fixtures/experiments/hash.json'), 'utf8')) as {
  fnv1a32: [string, number][];
  pick_variant_for: [ExperimentSpec, string, string, string][];
};

describe('hash vectors', () => {
  it.each(HASH.fnv1a32)('fnv1a32(%j) is %d, hashToUnit its share of 2^32', (seed, hash) => {
    expect(fnv1a32(seed)).toBe(hash);
    expect(hashToUnit(seed)).toBe(hash / 2 ** 32);
  });

  it.each(HASH.pick_variant_for)('pickVariantFor(%j, %j, %j) is %j', (spec, key, subject, variant) => {
    expect(pickVariantFor({ [key]: spec }, key, subject)).toBe(variant);
  });

  it('encodes a lone surrogate as U+FFFD, like TextEncoder', () => {
    expect(fnv1a32('\ud800')).toBe(fnv1a32('\ufffd'));
    expect(fnv1a32('x\udc00y')).toBe(fnv1a32('x\ufffdy'));
  });
});

describe('hashToUnit', () => {
  it('stays in [0, 1)', () => {
    for (let i = 0; i < 1000; i++) {
      const u = hashToUnit(`seed-${i}`);
      expect(u).toBeGreaterThanOrEqual(0);
      expect(u).toBeLessThan(1);
    }
  });
});

describe('hashRng', () => {
  it('returns hashToUnit(`${seed}#${n}`) for the n-th call', () => {
    const rng = hashRng('exp:loc-1');
    expect(rng()).toBe(hashToUnit('exp:loc-1#0'));
    expect(rng()).toBe(hashToUnit('exp:loc-1#1'));
    expect(rng()).toBe(hashToUnit('exp:loc-1#2'));
  });

  it('gives each generator its own counter', () => {
    const a = hashRng('s');
    a();
    expect(hashRng('s')()).toBe(hashToUnit('s#0'));
  });
});

const config = {
  hero: { variants: ['a', 'b'] },
  team: { variants: ['a', 'b', 'c'] },
} as const satisfies ExperimentConfig;

describe('pickVariantFor', () => {
  it('equals pickVariant with hashRng(`${key}:${subject}`)', () => {
    for (const subject of ['loc-1', 'loc-2', 'loc-3', 'h\u00e9llo']) {
      expect(pickVariantFor(config, 'team', subject)).toBe(
        pickVariant(config, 'team', hashRng(`team:${subject}`)),
      );
    }
  });

  it('is deterministic per subject', () => {
    for (let i = 0; i < 200; i++) {
      const subject = `loc-${i}`;
      expect(pickVariantFor(config, 'team', subject)).toBe(pickVariantFor(config, 'team', subject));
    }
  });

  it('distributes 10k subjects in equal thirds', () => {
    const counts: Record<string, number> = { a: 0, b: 0, c: 0 };
    const n = 10_000;
    for (let i = 0; i < n; i++) {
      const v = pickVariantFor(config, 'team', `location-${i}`);
      counts[v] = (counts[v] ?? 0) + 1;
    }
    // ±2 pp is > 4 sigma at n = 10k.
    for (const v of ['a', 'b', 'c']) {
      expect(Math.abs((counts[v] ?? 0) / n - 1 / 3)).toBeLessThan(0.02);
    }
  });

  it('distributes 10k subjects close to operator weights', () => {
    const live = applyOverrides(config, { team: { weights: [2, 1, 1] } });
    const counts: Record<string, number> = { a: 0, b: 0, c: 0 };
    const n = 10_000;
    for (let i = 0; i < n; i++) {
      const v = pickVariantFor(live, 'team', `location-${i}`);
      counts[v] = (counts[v] ?? 0) + 1;
    }
    // weights 2:1:1 → 50/25/25 %; ±2 pp is > 4 sigma at n = 10k.
    expect(Math.abs((counts['a'] ?? 0) / n - 0.5)).toBeLessThan(0.02);
    expect(Math.abs((counts['b'] ?? 0) / n - 0.25)).toBeLessThan(0.02);
    expect(Math.abs((counts['c'] ?? 0) / n - 0.25)).toBeLessThan(0.02);
  });

  it('buckets the same subject independently per experiment key', () => {
    const differs = Array.from({ length: 100 }, (_, i) => `loc-${i}`).some(
      (s) => pickVariantFor(config, 'hero', s) !== pickVariantFor(config, 'team', s),
    );
    expect(differs).toBe(true);
  });
});

describe('enabled', () => {
  const off = {
    hero: { variants: ['a', 'b'], enabled: false },
  } as const satisfies ExperimentConfig;

  it('a disabled experiment picks the control without drawing', () => {
    let calls = 0;
    const rng = () => {
      calls++;
      return 0.99;
    };
    expect(pickVariant(off, 'hero', rng)).toBe('a');
    expect(calls).toBe(0);
    expect(pickVariantFor(off, 'hero', 'loc-1')).toBe('a');
  });

  it('a disabled experiment resolves any cookie to the control', () => {
    expect(resolveVariant(off, 'hero', 'b')).toBe('a');
  });

  it('enabled: true behaves like an omitted flag', () => {
    const on = { hero: { variants: ['a', 'b'], enabled: true } } as const;
    expect(pickVariant(on, 'hero', () => 0.5)).toBe('b');
    expect(resolveVariant(on, 'hero', 'b')).toBe('b');
  });
});

describe('holdout', () => {
  const held = {
    hero: { variants: ['a', 'b'], holdout: 0.2 },
  } as const satisfies ExperimentConfig;
  // An operator override pinning every non-held-out draw to "b", so the control
  // count below is exactly the holdout.
  const heldAllB = applyOverrides(held, { hero: { weights: [0, 1] } });

  it('pins draws below the holdout to the control', () => {
    expect(pickVariant(heldAllB, 'hero', () => 0)).toBe('a');
    expect(pickVariant(heldAllB, 'hero', () => 0.1999)).toBe('a');
    expect(pickVariant(heldAllB, 'hero', () => 0.2)).toBe('b');
  });

  it('rescales the remaining draw onto the equal split with a single rng call', () => {
    const split = {
      hero: { variants: ['a', 'b'], holdout: 0.5 },
    } as const satisfies ExperimentConfig;
    // u = 0.7 → (0.7 - 0.5) / 0.5 = 0.4 → "a"; u = 0.8 → 0.6 → "b".
    expect(pickVariant(split, 'hero', () => 0.7)).toBe('a');
    expect(pickVariant(split, 'hero', () => 0.8)).toBe('b');
    let calls = 0;
    pickVariant(split, 'hero', () => {
      calls++;
      return 0.8;
    });
    expect(calls).toBe(1);
  });

  it('holdout 1 is all control, out-of-range values are clamped, NaN is none', () => {
    const all = { hero: { variants: ['a', 'b'], holdout: 1 } } as const;
    expect(pickVariant(all, 'hero', () => 0.999)).toBe('a');
    const over = { hero: { variants: ['a', 'b'], holdout: 7 } } as const;
    expect(pickVariant(over, 'hero', () => 0.999)).toBe('a');
    // No holdout: u = 0.6 is the second half → "b" (a 0.6 holdout would pin it).
    const neg = { hero: { variants: ['a', 'b'], holdout: -1 } } as const;
    expect(pickVariant(neg, 'hero', () => 0.6)).toBe('b');
    const nan = { hero: { variants: ['a', 'b'], holdout: Number.NaN } };
    expect(pickVariant(nan, 'hero', () => 0.6)).toBe('b');
  });

  it('holds out close to the configured share across 10k subjects', () => {
    let control = 0;
    const n = 10_000;
    for (let i = 0; i < n; i++) {
      if (pickVariantFor(heldAllB, 'hero', `location-${i}`) === 'a') control++;
    }
    expect(Math.abs(control / n - 0.2)).toBeLessThan(0.02);
  });
});

describe('forcedVariant', () => {
  it('accepts a declared variant and rejects anything else', () => {
    expect(forcedVariant(config, 'team', 'c')).toBe('c');
    expect(forcedVariant(config, 'team', 'zzz')).toBeUndefined();
    expect(forcedVariant(config, 'team', '')).toBeUndefined();
    expect(forcedVariant(config, 'team', null)).toBeUndefined();
    expect(forcedVariant(config, 'team', undefined)).toBeUndefined();
  });

  it('is refused for a disabled experiment', () => {
    const off = { hero: { variants: ['a', 'b'], enabled: false } } as const;
    expect(forcedVariant(off, 'hero', 'b')).toBeUndefined();
  });
});
