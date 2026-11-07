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

`Outline`, `Ghost` and `Link` stay. They name an **affordance pattern**, not a
raw visual property: "outlined button" is a term of art that survives a
repalette, and it carries information an emphasis ladder loses — `Tertiary` does
not tell you the thing has a border. `Destructive` stays for the same reason
inverted: it names a consequence.

`Red` names a hue. Change the palette and the name lies.
`Default` names nothing at all.

## Consequences

**The selected-by-default fact lives in the trait.** `#[tw(default, …)]` already
emits `impl Default`, so a variant marked with it needs no name saying so.

**The generated key is the variant name.** A table keyed `"default"` beside
`"sm"` and `"lg"` cannot be read: two tables can mean different steps by it, and
nothing surfaces the discrepancy because the key carries no information to
contradict.

**A table key names one axis.** A multi-axis table is multiple tables, never a
product flattened into strings: `sizeClasses` over the sizes and
`BUTTON_ICON` for the shape at any of them, not one table keyed `"icon-sm"` —
which fuses magnitude with shape and, in `"icon"`, elides the default as well.

**There is one severity vocabulary.** `Tone` is `{Neutral, Primary, Positive,
Info, Warn, Error}`: the [accent rungs](./accents.md) a status needs, the
valence and the action role. Every toned component takes it, never a second
ladder written in different words. Figma's own names (accent, negative,
warning) map onto these; the tint tokens `--tint-{tone}-{fill,edge}` carry the
same six tones.
