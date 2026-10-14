import { defineConfig } from 'vitest/config';

// Shared config; the node/jsdom split lives in `vitest.workspace.ts` (Vitest
// 2.x workspaces), as in @evinvest/experiments. The core is verified without
// ever loading a DOM, mirroring how it ships server-safe.
export default defineConfig({
  esbuild: {
    jsx: 'automatic',
  },
});
