# Architecture

`EV-invest/lib` is the organisation's **shared-library monorepo**. Its centre of
gravity is one thing expressed in two languages: a generic, I/O-free **DDD
tactical kernel**. The Rust crate is the source of truth; the TypeScript packages
mirror its _semantics_ idiomatically.

## Layout

```
lib/                 (repo: EV-invest/lib)
├── Cargo.toml       thin virtual workspace — anchors the crate at the repo root
├── tokens.css       the shared design tokens, one copy for both ports
├── rust/            the crate (sources); one library per Cargo feature
│   ├── Cargo.toml
│   ├── src/{lib.rs, architecture/, uikit/ (+ uikit_dioxus/, uikit_leptos/), analytics/, error_monitoring/, experiments/, settings/, otel/, i18n/, kitstart/}
│   └── tests/
├── ts/              TypeScript packages, one directory per library
│   ├── architecture/
│   ├── uikit/
│   ├── analytics/
│   ├── error-monitoring/
│   ├── experiments/
│   ├── settings/
│   ├── marketing/
│   └── kitstart/
├── tests/fixtures/  test vectors both ports read (kitstart)
├── nix/            lib.mkLanding — the flake every kitstart landing shares
├── docs/
│   ├── ARCHITECTURE.md          (this file)
│   └── .readme_assets/          README fragments (README.md is generated)
└── flake.nix        v_flakes dev shell + generated CI / README / configs
```

Each language owns a top-level directory (`rust/`, `ts/`) so neither toolchain
trips over the other. The shared tooling (CI, formatters, lint config) drives
`cargo` from the repo root, so a **thin virtual workspace** at `./Cargo.toml`
anchors the crate there while its sources stay in `rust/`. The crate is still a
single package: a consumer's git dependency resolves it **by name**, regardless
of the subdirectory.

```toml
# native
ev_lib = { git = "https://github.com/EV-invest/lib.git", default-features = false, features = ["architecture"] }
```

## Component map

_As of 2026-10-27._

```
                         ┌───────────────────────── repo root ─────────────────────────┐
                         │ theme.css + ev.css ──concat──▶ tokens.css      motion.css   │
                         │ (hand-edited design tokens, the shared contract)             │
                         └───────────────┬─────────────────────────────────────────────┘
                                         │ read by
┌─ crate ev_lib_classes ─┐               │
│ class tables:          │               │
│ ButtonVariant, Size,   │               │
│ Polarity, Accent, …    │               │
│ cn!/fuse  (tailwind_   │               │
│ fuse), CLASS_INVENTORY,│               │
│ TOKENS_CSS             │               │
└───────┬────────────────┘               │
        │ used by                        │
        ▼                                ▼
┌──────────────────── crate ev_lib (one library per Cargo feature) ────────────────────┐
│                                                                                      │
│  KERNEL (I/O-free, zero-dep)       UI                     I/O libraries (wasm-safe)  │
│  ┌──────────────┐          ┌───────────────────┐   ┌───────────┐ ┌────────────────┐ │
│  │ architecture │          │ uikit core: no DOM│   │ analytics │ │error_monitoring│ │
│  │ Id, Entity,  │          │ ├ uikit_dioxus    │   │ (PostHog) │ │ (Sentry)       │ │
│  │ Repository,  │          │ └ uikit_leptos    │   └───────────┘ └───────┬────────┘ │
│  │ UnitOfWork ✗─┼─Gateway  └───────────────────┘   ┌───────────┐         │ used by │
│  │ DomainEvent, │          (Gateway can't be       │experiments│ ┌───────▼──────┐  │
│  │ Specification│           enrolled in UoW)       │ ─sink▶    │ │ otel (native)│  │
│  └──────────────┘                                  │ no import │ └──────────────┘  │
│  ┌───────┐ ┌──────────┐                            │ of        │ ┌──────────────┐  │
│  │ types │ │ settings │─(+settings_drift: tokio)   │ analytics │ │ alerts       │  │
│  │ Phone,│ │ settings!│                            └───────────┘ │ (native,     │  │
│  │ Email │ └──────────┘                                          │  Discord)    │  │
│  └───────┘                                                       └──────────────┘  │
│              ┌───────────────┐   LocaleRegistry   ┌──────────────────────────────┐ │
│              │ i18n          │◀───────────────────│ kitstart (pure, no I/O/clock)│ │
│              │ locales, t!,  │                    │ place · routing · site · ld  │ │
│              │ ICU fmt,policy│◀──┐                │ meta · sitemap · antispam ·  │ │
│              └───────────────┘   │ requires       │ contact · query · json       │ │
│                          ┌───────┴──────┐         └──────────────┬───────────────┘ │
│                          │ mfe (wasm)   │                        │                 │
│                          │ mfe! remotes │       tests/fixtures/kitstart/*.json ◀─┐ │
│                          └──────────────┘       (vectors both ports run)         │ │
│  ┌───────────────────────────────────┐                                           │ │
│  │ ts_gen: Ts::{Const,Value,Table,   │                                           │ │
│  │ Union,Types} via serde + ts-rs    │                                           │ │
│  └─────────────┬─────────────────────┘                                           │ │
└────────────────┼──────────────────────────────────────────────────────────────────┼─┘
                 ▼                                                                 │
┌─ crate ev_lib_gen (`nix run .#gen`, re-run by pre-commit hook) ──┐               │
│  manifest(): uikit class tables ─▶ ts/uikit/src/generated/*.ts   │ ts/kitstart ──┘
│  shared():   i18n/locales, types/e164, experiments/contract,     │
│              settings/contract, analytics/host, kitstart/contract│
│              ─▶ ts/<pkg>/src/generated/*.ts                      │
│  css:        tokens/theme/ev ─▶ rust/classes/css + ts/uikit/styles
│  inventory:  ─▶ rust/classes/uikit-classes.txt                   │
└──────────────────────────────────────────────────────────────────┘
  uikit-viewer (Dioxus web app) ─uses─▶ ev_lib[uikit_dioxus,wasm]   ≙ ts/uikit/example
```

What of `ts/` is hand-written rather than generated, and why:
[`ts/README.md`](../ts/README.md#what-is-not-generated).

## The `architecture` kernel

Generic DDD building blocks every bounded context implements. Deliberately
I/O-free and `wasm32`-safe; concrete adapters (sqlx, an external ledger, …) live
in the consuming service.

| Trait / type (Rust)           | TypeScript form                   | Role                                                                             | Object-safe (Rust)?  |
| ----------------------------- | --------------------------------- | -------------------------------------------------------------------------------- | -------------------- |
| `Id<Tag, U>` / `Identifier`   | branded primitive `Id<Tag, U>`    | typed identity — a `TransferId` can't be passed where an `AccountId` is expected | no (always concrete) |
| `Entity` / `AggregateRoot`    | `interface`                       | stable identity; the transactional consistency boundary                          | no                   |
| `Repository` / `Reader`       | `interface` (phantom-bound)       | marker ports tying a port to one aggregate (CRUD lives on the leaf)              | yes                  |
| `Gateway`                     | `interface` (marker)              | anti-corruption boundary to an external transactional system                     | yes                  |
| `UnitOfWork`                  | `interface` (`Promise`)           | one atomic transaction; `commit`/`rollback` are terminal                         | yes                  |
| `DomainEvent` / `EmitsEvents` | discriminated union / `interface` | past-tense facts; defined, not yet wired (future outbox)                         | no                   |
| `Specification<T>`            | `interface` + combinators         | composable in-memory predicate                                                   | yes (core method)    |

## Consistency boundaries — Repository vs Gateway

Two independent consistency boundaries, encoded in the type system:

- A `UnitOfWork` is exactly one SQL (e.g. Postgres) transaction; repositories
  enroll in it.
- A `Gateway` is an external system (a ledger, a payment API). It is **never** a
  `Repository` and is **unreachable** from a `UnitOfWork` (there is no
  `UnitOfWork::gateway()`), so the type system forbids enrolling it in a local
  transaction. It owns its own identity, invariants, and atomicity.

Any operation spanning both is an explicit application-layer saga, not an atomic
transaction. The intended future mechanism is a `DomainEvent` written to an
outbox **inside the same `UnitOfWork`** as the state change — which is why
`DomainEvent` exists but is unwired today.

## The `uikit` library

A **dep-light** UI kit — 63 shadcn-semantics components — shipped as two Rust
ports over one core, `uikit_dioxus` (`ev_lib::uikit_dioxus`) and `uikit_leptos`
(`ev_lib::uikit_leptos`), and as the TS package `@evinvest/uikit` (React). It is
the kit consolidated out of the EV-invest apps (`cabinet`'s Dioxus components and
`landing`'s React bricks), so every app depends on one versioned source.

```
ev_lib_classes      class strings · variants · cn!/fuse · tokens.css · CLASS_INVENTORY
      │
ev_lib::uikit       crate-private core, compiled when either port is on; no dioxus,
      │             no leptos, no web_sys. One plain struct per stateful component,
      │             `&mut self` methods over plain values (KeyInput, now_ms, Placement),
      │             read back as view data
  ┌───┴──────────────────────┐
uikit_dioxus             uikit_leptos       one reactive cell per instance + glue:
feature `uikit_dioxus`   feature `uikit_leptos`   events → core inputs, view data → markup
```

The core is `pub(crate)`: both ports live in this crate, so it never joins the
public interface. A port re-exports only the types a caller has to name to build
a prop (the `ev_lib_classes` variants, `Side`, `DismissReason`, `FormControlContext`,
…). The two features may be on together; neither pulls in the other's framework —
`wasm` reaches Dioxus only through the weak `dioxus?/web`. The Leptos port never
enables `ssr`, `hydrate` or `csr`; the consumer's own `leptos` picks one.

Unlike the kernel, `uikit` is **not** zero-dep and **not** I/O-free — a UI kit
inherently carries a renderer and styling helpers. What it keeps from the repo's
discipline:

- **Dep-light:** no `@radix-ui`, no `class-variance-authority`, no icon/charting/
  date/toast libraries. Variants are `enum`+`match` (Rust) / `as const` maps (TS)
  fused by the `cn!`/`cn` helper; overlay behaviour is hand-rolled; icons are
  inline SVG.
- **Rust core, two Rust ports, TS mirror:** behaviour lives once, in the core;
  each Rust port is glue over it and renders the same markup from its own
  template (nothing in Rust can share a view across Dioxus and Leptos, so
  `rust/tests/parity.rs` is what holds them together). The TS port mirrors the
  semantics; canonical Tailwind class strings are identical per element across
  all three. Variants are the **superset** of the two original sources.
- **`tokens.css` is the contract:** the design tokens (CSS custom properties +
  Tailwind `@theme inline`) live once at the repo root. `nix run .#gen`
  flattens them into both languages — `ts/uikit/styles/` for npm and
  `rust/classes/css/` for the crate, the latter because Tailwind can neither
  scan nor `@import` a crates.io checkout. Every component class references a
  token; a consumer must `@import` the sheet.
- **Nothing derived is hand-written or copied:** `nix run .#gen` is the only
  producer of the TS class tables, the Tailwind class inventory (the tables plus
  every literal in the core and both ports) and the two token sheets, and the
  `generated` pre-commit hook re-runs it and re-stages. A committed artefact
  therefore cannot disagree with its source — which matters because the failure
  is silent: Tailwind answers an undefined token by emitting no rule, so a
  drifted sheet is a colour quietly going missing on a consumer, not a build
  error.
- **Class strings may be composed at runtime** (`format!("h-{}", n)`), e.g. the
  shared `Size` whose `scale()` magnitude each component applies on its own axis
  (`h-`/`size-`/`min-w-`). A consumer on the v4 browser CDN gets them by its DOM
  scan; one on a *static* Tailwind build `@source`s `CLASS_INVENTORY` and must
  likewise safelist the kit's magnitudes.

Because there is no renderer-agnostic portal and layout measuring is host-only,
the Rust overlays/engines carry documented fidelity gaps (inline positioning
instead of portals, keyboard instead of pointer-drag, no recharts/embla/vaul) —
the full list is in [`ts/uikit/README.md`](../ts/uikit/README.md#limitations).

### Invariants

- **The core is DOM-free and framework-free.** No `dioxus`, `leptos` or `web_sys`
  under `rust/src/uikit/`. Keys arrive as a core `KeyInput` (each port has
  `From<&its event>`); anything needing a clock takes `now_ms`; document order
  is an input (`Placement`, which the port answers from its DOM). The core never
  stores elements or callbacks — it keeps ids, and the port maps id → element /
  handler and carries out the effects the core returns (focus, scroll, select).
- **Each port is a reactive cell plus glue.** One reactive cell holds a
  component instance's core state; the port turns its events into core inputs
  and renders markup from the view data, deriving per-part reads so a change
  re-renders only the parts whose state changed. Behaviour a port re-implements
  is a bug in the split.
- **Both ports render the same HTML.** `rust/tests/parity.rs` renders every
  gallery entry through dioxus-ssr and Leptos SSR and compares them after
  canonicalising (attribute order and spelling, comments and hydration markers,
  whitespace-only text, and ids renamed by first appearance).
- **A component exists in both ports or neither.** The same test fails on a
  gallery label present on one side only, and on a `#[component]` exported by
  one port only; every exported component appears in a gallery entry.

## The I/O libraries — analytics, error_monitoring, experiments

Three frontend-facing libraries that, unlike the kernel, **do network I/O**.
Each is a Cargo feature on the crate and an npm package under `ts/`, mirroring the
other's semantics.

| Cargo feature      | npm package                  | Purpose                        | Network I/O? | wasm-safe? |
| ------------------ | ---------------------------- | ------------------------------ | ------------ | ---------- |
| `analytics`        | `@evinvest/analytics`        | PostHog product analytics      | yes          | yes        |
| `error_monitoring` | `@evinvest/error-monitoring` | Sentry error monitoring        | yes          | yes        |
| `experiments`      | `@evinvest/experiments`      | A/B testing (frontend-only)    | via the host | yes        |

The property taxonomy is deliberate:

- **I/O-free** is a property of the **kernel only**. The kernel ships zero
  runtime deps and touches no host.
- **wasm-safe** holds for the kernel **and** both `uikit` ports **and** these three.
- **"no I/O" is *not*** a property of these three. Network I/O is their purpose:
  `analytics` POSTs PostHog events, `error_monitoring` reports to Sentry. Asking
  them to be I/O-free would defeat them.

Doing I/O while staying wasm-safe is reconciled by **per-target dep gating**.
Native builds use `reqwest`(rustls) / the `sentry` crate; wasm builds use
pure-Rust HTTP (the `reqwest` fetch backend plus `web-sys`/`js-sys`/`wasm-bindgen`),
selected through `[target.'cfg(target_arch="wasm32")']` tables behind the `wasm`
feature. `error_monitoring`'s native `sentry` crate is native-only and is never
linked into a wasm build. `experiments` is frontend-only and reports exposure
through an **injected sink** — it never imports `analytics`, so there is no
cross-library coupling. None of this touches the kernel, which stays I/O-free.

The TS packages expose backends through subpath exports — analytics
`.`/`./react`/`./node`; error-monitoring `.`/`./react`/`./node`/`./next`;
experiments `.`/`./react`/`./next` — with the vendor SDKs as optional
`peerDependencies`.

## The `settings` library

Typed env settings — the `settings` Cargo feature (`ev_lib::settings`, a
`settings!` macro) and `@evinvest/settings` (`createSettings`). Zero runtime
deps on both sides, like the kernel — but not I/O-free: reading the process
environment is its purpose (no files, no network). Env-only by design: no
config-file layer, no hot reload (a process's environment is fixed at `exec`).
The shared semantics — SCREAMING_SNAKE naming, required-by-default,
empty-string-is-unset, the bool/list parsing rules, aggregate error reporting,
secret redaction — are pinned by mirrored contract-test vectors in both suites.
`presets` fixes the org-canonical shared names (`POSTHOG_KEY`, `SENTRY_DSN`,
`APP_ENV`) once, on both sides.

Drift watching is the one exception to zero-dep, and so sits behind its own
native-only feature, `settings_drift` (`tokio` + `tracing`): with it `settings!`
also generates `watch_drift()`, a never-returning future that polls the k8s
Secret mounted at `$SETTINGS_DRIFT_MOUNT` and warns when it no longer matches
the boot snapshot. Core `settings` is unchanged by it. `settings!` reaches the
method through a `#[doc(hidden)]` helper macro whose two cfg'd bodies are
resolved when ev_lib compiles, so it follows ev_lib's features, not the
consumer's. Rust-only: a browser bundle has no Secret to drift from.

Secrets deliberately live **outside** the library: sops-encrypted env files
(age keys) are committed to each consuming repo and decrypted only at the
boundary — direnv/`sops exec-env` in dev shells, `SOPS_AGE_KEY` in CI — so
apps stay sops-unaware and the library never links a decryption stack. The
workflow is documented in
[`rust/src/settings/GUIDE.md`](../rust/src/settings/GUIDE.md#secrets-the-sops-boundary).

## Generated TypeScript — `ts_gen`

`ev_lib::ts_gen` is the machinery `ev_lib_gen` writes this repo's TS with, opened
to every repo: a downstream mirror of a Rust type or value is generated from the
Rust, never re-typed. A repo keeps one `gen` binary that lists its modules as
`Ts` exports, exposes it as `nix run .#gen` (the header every module carries),
commits the output and re-runs it from a `generated` pre-commit hook.

- **Types** go through `ts-rs`: `#[derive(TS)]` beside `Serialize`, and
  `Ts::types::<T>()` declares `T` and every type it names, as serde writes them.
  `ts-rs` keeps `u64`/`i64` as `bigint` (see Cross-language parity): a wire type
  that wants `number` says so with `#[ts(type = "number")]`.
- **Values** — the variants of an enum, a role/permission table — go through
  serde (`Ts::Value`), so their words are the wire's and the logic that computes
  them (`Role::may`) stays in Rust; TS gets the table, not a second predicate.
- **Closed word sets** — `Ts::Union` emits the array and its type together,
  from the Rust words (`as_str`), so a TS parser's `oneOf` and the type agree.

## Cross-language parity

The TS packages are not line-by-line translations; they preserve the _semantics_
of the Rust public API while reading like idiomatic TS. The full mapping and the
intentional divergences (errors, numbers, serialization) are the source of truth
in [`ts/architecture/README.md`](../ts/architecture/README.md). The load-bearing
ones:

- **Numbers** — `u64`/`u128` and anything precision-sensitive map to `bigint`,
  never `number` (an f64 loses precision past 2⁵³). Numeric ids are `bigint`.
- **Errors** — `Option<T>` → `T | undefined`; the one fallible async op
  (`UnitOfWork`) rejects its `Promise` (throws) rather than returning a `Result`.
- **Typed ids** — branded primitives, so `===` and `Map`/`Set` keys work
  natively; no `underlying()` indirection.

## Source of truth

- The **Rust crate is canonical**. When the kernel's behaviour changes, change
  Rust first, then bring the TS port back into parity. The TS README's mapping
  table is the contract between the two.
- A new library = a Rust module behind a Cargo feature **and** (if it has a TS
  consumer) a package under `ts/`. Keep the feature name and the package name in
  step, following the naming convention: **snake_case in Rust, kebab-case in TS;
  identical when single-token** (`architecture` ↔ `ts/architecture`;
  `error_monitoring` ↔ `ts/error-monitoring`).

