# A variant names the state

**Rule.** A variant names the **state**, never its appearance and never its
selection.

A name that would become a lie if the palette changed (`Red`, `Teal`) is wrong.
A name that says only "this is the one you get if you say nothing" (`Default`)
is wrong — that fact lives in a `Default` impl. `lg / md / sm`, never
`lg / default / sm`.

No tool can check this — it is decided once per new enum, in four repos — so it
is written down instead.

## Where the line falls

A button is `Primary`, `Secondary`, `Ghost`, `Solid`, `Danger` or `Link`, from
Uikit 2.0 on. `Ghost` and `Link` name an **affordance pattern**, not a raw
visual property: they survive a repalette, and they carry information an
emphasis ladder loses. `Danger` names a consequence, and it replaces
`Destructive` so menu rows and buttons say the same word. `Primary` is the
outlined action and `Solid` the one filled call to action; whether either paints
an outline or a fill is the theme's job (the `--button-*` face tokens), so the
name stays true when Oldschool fills `Primary`. `Outline` is gone: `Secondary`
is the raised face with a hairline.

`Red` names a hue. Change the palette and the name lies.
`Default` names nothing at all.

## Consequences

**The selected-by-default fact lives in the trait.** `#[tw(default, …)]` already
emits `impl Default`, so a variant marked with it needs no name saying so.

**The generated key is the variant name.** A table keyed `"default"` beside
`"sm"` and `"lg"` cannot be read: `sidebarMenuButtonSizeClasses["default"]` was
`Sm` while `buttonSizeClasses["default"]` was `Md`, and nothing surfaced the
discrepancy because the key carried no information to contradict.

**A table key names one axis.** A multi-axis table is multiple tables, never a
product flattened into strings: `buttonSizeClasses` over the sizes and
`buttonIconSizeClasses` over the same sizes, not one table keyed `"icon-sm"` —
which fuses magnitude with shape and, in `"icon"`, elides the default as well.

**There is one severity vocabulary.** A toast is `{Neutral, Positive, Info,
Warn, Error}` — the [accent rungs](./accents.md) plus the valence — not a second
ladder written in different words. A badge adds `Primary`: `{Neutral, Primary,
Positive, Info, Warn, Error}`. Figma's own names (accent, negative, warning) map
onto these; the chip tokens `--chip-{tone}-{bg,border}` carry the same six
tones.
