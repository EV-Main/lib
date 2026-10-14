import { defineConfig } from 'tsup';

import { coreAsSibling } from './tsup.react.config';

// Same split as @evinvest/experiments: only the React subpath carries the
// `"use client"` banner — on `.` it would pull the registry parser and
// `fetchRegistry` into the client bundle of every Server Component importing
// them. Config #1 alone cleans `dist/`; tsup's `.d.ts` step races a concurrent
// clean, so `build` re-runs config #2 alone (`tsup.react.config.ts`) last.
export default defineConfig([
  {
    entry: { index: 'src/index.ts' },
    format: ['esm'],
    dts: true,
    clean: true,
    sourcemap: true,
    target: 'es2022',
  },
  {
    entry: { react: 'src/react/index.ts' },
    format: ['esm'],
    dts: true,
    clean: false,
    sourcemap: true,
    target: 'es2022',
    external: ['react', 'react-dom', 'react/jsx-runtime'],
    banner: { js: '"use client";' },
    esbuildPlugins: [coreAsSibling],
  },
]);
