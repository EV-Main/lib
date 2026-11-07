import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { frameKey, frames } from '../src/frames';

const list = JSON.parse(readFileSync(new URL('./fixtures/list.json', import.meta.url), 'utf8'));

describe('frames', () => {
  it('keys every @figma spec by project, in the list order', () => {
    expect(frames(list).map(frameKey)).toEqual(
      ['desktop', 'mobile', 'tablet'].flatMap(p =>
        ['/fr', '/fr/prices', '/fr/about', 'http://localhost:52826/fr', '/fr/nope', '/fr/thanks'].map(page => `${p} ${page}`),
      ),
    );
  });

  it('refuses a list that failed to load', () => {
    expect(() => frames({ suites: [], errors: [{ message: 'SyntaxError' }] })).toThrow(/SyntaxError/);
  });
});
