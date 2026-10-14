import { defineConfig } from 'tsup';

// The server-safe core only; the React subpath is `tsup.react.config.ts`, run
// after this one by `build`/`prepare`. Two sequential runs rather than one
// array: an entry with the `"use client"` banner must not share a run with `.`
// (the banner would reach the core), and tsup's `.d.ts` step deletes the
// declarations of a concurrent config when this one cleans `dist/`.
export default defineConfig({
  entry: { index: 'src/index.ts' },
  format: ['esm'],
  dts: true,
  clean: true,
  sourcemap: true,
  target: 'es2022',
});
