# rust

The `ev_lib` crate's sources — one library per Cargo feature, so a consumer compiles
only what it enables. The thin virtual workspace that anchors the crate for
repo-root tooling is [`../Cargo.toml`](../Cargo.toml).

```
rust/
├── Cargo.toml          the `ev_lib` package
├── src/                one module per Cargo feature, grouped by scope (docs/ARCHITECTURE.md, "Scopes")
│   ├── kernel/         architecture · i18n · types
│   ├── product/        analytics · experiments
│   ├── ops/            settings · error_monitoring · otel · alerts
│   ├── ui/             uikit (core + dioxus + leptos ports) · active (mfe)
│   └── codegen/        ts_gen
├── classes/            ev_lib_classes: the class tables and token sheets
├── gen/                ev_lib_gen: `nix run .#gen`
└── tests/              integration tests
```

Unlike `architecture`, the two `uikit_*` features carry runtime deps (`dioxus` or
`leptos`, and `tailwind_fuse`) — a UI kit can't be zero-dep. Both ports run over
one crate-private core and render the same HTML; they mirror the `@evinvest/uikit`
TypeScript package and ship the shared design tokens; see their rustdoc and
[`../ts/uikit/README.md`](../ts/uikit/README.md), which carries the token table.

The class strings live once, in the framework-free `ev_lib_classes` crate
([`classes/`](classes)); `ev_lib_gen` emits the TypeScript half from them, so the
ports cannot drift. Each port re-exports only the types a caller has to *name*
to build a prop (`ButtonVariant`, `Size`, `Polarity`, `Surface`, …) — the class
constants are an implementation detail.

`tokens.css` is the token stylesheet: the brand-neutral contract (`theme.css`) followed by EV's palette (`ev.css`), both also shipped on their own as `ev_lib_classes::THEME_CSS` / `EV_CSS` for a consumer bringing its own palette. Tailwind can neither scan nor `@import` a crate unpacked from crates.io, so `ev_lib_classes` carries the sheets as data. Write them out from `build.rs`:

```rust
std::fs::write("uikit-classes.txt", ev_lib_classes::CLASS_INVENTORY).unwrap();
std::fs::write("assets/tokens.css", ev_lib_classes::TOKENS_CSS).unwrap();
```

then `@source` the first and `@import` the second. A hand-kept copy is a file that drifts silently on the next bump.

`analytics`, `error_monitoring`, and `experiments` likewise carry runtime deps
and **do network I/O** (PostHog / Sentry), gated per-target so native and browser
backends stay separate — native uses `reqwest`(rustls)/`sentry`, wasm uses
pure-Rust HTTP behind the `wasm` feature. Each mirrors its TS package
(`@evinvest/analytics`, `@evinvest/error-monitoring`, `@evinvest/experiments`);
see their rustdoc and READMEs.

`settings` is zero-dep like the kernel but reads host state (the process
environment — no files, no network): the `settings!` macro builds validated
settings structs with aggregate error reporting; sops/age decrypt at the
shell/CI boundary, never in the library. It mirrors `@evinvest/settings`.

`i18n` is zero-dep and wasm-safe: the locale registry, the `/<locale>` URL
contract, `Accept-Language` negotiation, an ICU-subset message formatter, and
the translation policy. It exists because a Dioxus zone cannot import an npm
package, and both halves read **the same `messages/<locale>/*.json`** so a
catalogue is portable between them and neither can drift alone. The CLDR plural
rules and number grouping are hand-written for exactly EV's five locales rather
than pulling ICU4X into a wasm bundle — see the module note on why that is a
deliberate ceiling, not a shortcut. It mirrors `@evinvest/i18n`.

Each feature mirrors a TypeScript package in [`../ts`](../ts). cargo runs from the
repo root — pass `-p ev` for feature flags. See
[`../docs/ARCHITECTURE.md`](../docs/ARCHITECTURE.md).
