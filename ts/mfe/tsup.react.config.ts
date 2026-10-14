import type { Plugin } from 'esbuild';
import { defineConfig } from 'tsup';

/**
 * The React subpath imports the core as `../index`; this keeps that import
 * external, pointing at the sibling `index.js`, instead of bundling a second
 * copy. A copy would split the loader's per-tag load cache in two, so a host
 * calling `loadElement` and rendering `RemoteElement` would load twice.
 */
const coreAsSibling: Plugin = {
  name: 'core-as-sibling',
  setup(build) {
    build.onResolve({ filter: /^\.\.\/index$/ }, () => ({ path: './index.js', external: true }));
  },
};

// The client subpath, built after `tsup.config.ts` and never cleaning, so the
// core's output survives. `"use client"` so it can be imported from React Server
// Components without its hooks running on the server.
export default defineConfig({
  entry: { react: 'src/react/index.ts' },
  format: ['esm'],
  dts: true,
  clean: false,
  sourcemap: true,
  target: 'es2022',
  external: ['react', 'react-dom', 'react/jsx-runtime'],
  banner: { js: '"use client";' },
  esbuildPlugins: [coreAsSibling],
});
