import { defineWorkspace } from 'vitest/config';

// Split by file suffix: `*.node.test.ts` runs in node (the server-safe core),
// `*.dom.test.ts` and `*.react.test.tsx` run in jsdom (the loader's DOM path
// and the `./react` subpath). The node project never loads a DOM.
export default defineWorkspace([
  {
    extends: './vitest.config.ts',
    test: {
      name: 'node',
      environment: 'node',
      include: ['test/**/*.node.test.ts'],
    },
  },
  {
    extends: './vitest.config.ts',
    test: {
      name: 'dom',
      environment: 'jsdom',
      setupFiles: ['./test/setup.dom.ts'],
      include: ['test/**/*.dom.test.ts', 'test/**/*.react.test.tsx'],
    },
  },
]);
