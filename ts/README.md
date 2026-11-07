# ts

TypeScript libraries — one directory per package, each self-contained with its
own `package.json`, built ESM-only with `tsup` and tested with `vitest`.

```
ts/
├── architecture/      port of the `architecture` Cargo feature (DDD kernel)
├── uikit/             port of the `uikit` Cargo feature (dep-light React UI kit)
├── analytics/         port of the `analytics` Cargo feature (PostHog product analytics)
├── error-monitoring/  port of the `error_monitoring` Cargo feature (Sentry error monitoring)
├── experiments/       port of the `experiments` Cargo feature (frontend-only A/B testing)
├── mfe/               host side of the `mfe` Cargo feature (registry, loader, events, RemoteElement)
├── settings/          port of the `settings` Cargo feature (typed env settings)
├── marketing/         landing-page layer over uikit: motion, form harness, click-to-load
├── figma-render/      a Playwright suite's pages into Figma frames (the `@figma` suite + its bin)
└── i18n/              five-locale i18n: registry, URL contract, ICU-subset formatter
```

Each package mirrors the _semantics_ of its Rust counterpart in
[`../rust`](../rust); see the package's own README for the Rust↔TS mapping.
Two exceptions. `i18n` has no Cargo feature yet; the planned one will read the
*same* `messages/<locale>/*.json` catalogues to localise transactional
email, so the two sides share one translation source rather than drifting.
`marketing` has none by design: it is React-only glue over the kit (motion,
forms, click-to-load) for landing pages; nor has `figma-render`, test tooling.
`node_modules/`, `dist/`, and `*.tsbuildinfo` are git-ignored.

## What is not generated

_As of 2026-11-06._

`ev_lib_gen` emits only **data** (consts, values, unions, tables, types; see
[`docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md#generated-typescript--ts_gen)):
~1.8k of ~37k lines. For the uikit that is every class string, word set
(`Side`, `Orientation`, `MenuItemVariant`, …), Labels default, line of fixed
copy, timing and icon shape; elsewhere the i18n locales and `RejectionReason`,
`ExperimentSpec` and its hash constants, the settings presets, the mfe
`MfeKind`/`MfeManifest` contract, and the analytics host and `$lib` marker.
The rest is hand-written for one of four reasons:

- **A** — logic, not data: `ts_gen` cannot emit a function
- **B** — React / Next / vendor-SDK glue, with no renderer-neutral form
- **C** — no Rust twin at all
- **D** — hand-ported mirror of Rust logic, pinned by vectors in `tests/fixtures/<pkg>/` that both suites read

```
 package      lines  what                                                   why
 ───────────────────────────────────────────────────────────────────────────────────
 uikit        14.7k  components/{atomic,derived,shell}/* (Dioxus twin; data shared) A B
                     the <svg> frames around the shared icon shapes         B
                     TS-only components (app-shell, nav, chat-panel, …)     C
                     primitives/ (portal, floating, focus-scope)            B C
                     palette/ (brand palette tool)                          C
 architecture  0.3k  kernel ports + combinators                             A  README mapping table
 settings      0.7k  createSettings, redaction                              A
                     validators (std FromStr grammar as regexes)            D  fixtures/settings
 i18n          3.0k  format, policy, registry/URL                           D  fixtures/i18n
                     extract CLI, react, next                               B
                     custom registry                                        C
 experiments   1.2k  bucketing                                              A
                     hash (FNV-1a)                                          D  fixtures/experiments
                     overrides                                              C
                     react, next                                            B
 mfe           0.9k  manifest parsing, URL/tag checks, events, loader       A C
                     RemoteElement                                          B
 analytics     1.4k  beacon body (captureBody)                              D  fixtures/analytics
                     sink / consent                                         A
                     posthog, react, next                                   B
 error-monitoring 0.6k  Sentry / React / Next wiring                        B
 marketing     1.8k  motion, form harness, react                            C
 figma-render  0.2k  @figma suite, settle, frames bin                       C
```
