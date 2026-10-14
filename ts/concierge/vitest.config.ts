import { defineConfig } from 'vitest/config';

// Shared config; the node/jsdom split lives in `vitest.workspace.ts`, as in @evinvest/mfe.
export default defineConfig({
  esbuild: {
    jsx: 'automatic',
  },
  resolve: {
    // `@evinvest/mfe` is linked from `file:../mfe`, whose own node_modules holds
    // a second React; without this its RemoteElement calls hooks on that copy
    // and every render fails. A consumer installing from npm has one React.
    dedupe: ['react', 'react-dom'],
  },
});
