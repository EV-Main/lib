# Accents are ordered by significance

**Rule.** `accent-trace`, `accent-debug`, `accent-info`, `accent-warn`,
`accent-error` are a ladder from quiet to loud. Pick a rung by how loud the
thing should be on the page, never by what it means.

A faint background flourish and a trace line take `accent-trace` for the same
reason. A shout takes `accent-error` whether or not anything failed.

No tool can check this — "is this loud enough to be `accent-warn`?" is a
judgment at the call site, in four repos — so it is written down instead.

## Consequences

**Every consumer defines all five.** A palette with holes cannot be relied on,
which is what would stop a kit component from referencing an accent at all. A
design that does not visually distinguish two rungs binds them to the same
value; it does not leave one undefined.

**There is no separate severity vocabulary.** Colouring a message by importance
is not a second job — importance *is* the axis. `level-*` was the same five
values written a second time.

**Valence is not significance.** A success is low-significance (nothing to see)
but must read as good, so `positive` is its own role off the ladder. Adding it
as a sixth rung would break the ordering.

**Charts are neither.** `chart-1 … chart-6` are distinguishable hues carrying no
significance. A decorative colour that is not ordered belongs there.

## A fill is not an ink

`primary` is not an accent and not a rung of the ladder: it is the action role,
and it is the one role whose fill and ink are two values. A role is one colour
worn two ways — as a solid fill with `on-{role}` reading on it, and as ink on a
surface — and for every other role one value does both jobs: `positive` and the
five accents each hold their dark `on-*` label (≥ 4.5:1) and read as ink on
`background` and `card` at once. `primary` could too — the dark label reads on
the teal ink `#35a395` at 6.31:1 — so the split is a choice, not a necessity: the
call to action wanted a white label and a calmer plane than a bright teal
chip, and a teal dark enough for white (`#138377`, 4.62:1) is too dark to be
the link and eyebrow colour on a card. Hence `primary` / `primary-ink`, the
same hue and chroma one step apart in lightness; a role whose two values ever
diverge names its ink the same way, `{role}-ink`.

**The quiet status roles follow it.** `positive-ink` and `accent-info-ink`
are the colour for status *text* in those roles. On a light plane the fills
are shapes only, so a label, a chip or a message in a quiet status colour
wears the ink. They are derived — 60 % of the role toward `ink` — and EV pins
Figma's values. In Light, `positive` itself is `#1f7f48`, one step darker than
Figma's `#228a4e`, because neither label reached 4.5:1 on the Figma value.

**Attention colours have no ink.** `accent-warn` and `accent-error` exist to
grab attention, so they are exempt from the 4.5:1 text floor: their text,
icon and border — a form message, an invalid field, an alert, a destructive
menu row, a danger button, a warn badge — wear the role itself, held to the
non-text 3:1 against the surface they sit on. A lightened ink would clear AA
by going pastel, which is what the role must not do (Oldschool's mix read
`#ed8379` where the role is `#f05b52`). The floor is still met by every
palette: in Light, `accent-warn` is `#a77f12`, darkened toward Figma's warn
text from `#d8a523`, which read 1.89:1 on the page plane.

**Fill intensity is not text intensity.** A fill or a tint covers an area, a
glyph is a few pixels of stroke, so the same colour reads far quieter as text
than as a fill. Do not match a text colour to a fill by eye or by value: a
`bg-accent-error/10` tint behind `text-accent-error` is the intended pair, and a
tint is never measured against the 3:1 or 4.5:1 a text colour owes.

**Label polarity is deliberately split.** White on a fill marks the one call to
action, so it belongs to `primary` alone; `destructive`, `positive` and the five
accents keep the dark `on-*` label. A Primary and a Destructive button side by
side in a dialog footer read as two polarities on purpose — do not "fix" it by
whitening the semantic fills.

**What is the fill and what is the ink.** `bg-primary`, `text-on-primary`, the
checked state of a Checkbox and a Switch, a selected Calendar day, a Toggle
that is on and the `Primary` band are the fill. A link, an eyebrow, a checked Field outline, the
focus `ring`, a radio dot, a Slider range and a Progress indicator are the ink.
The radio dot, range and indicator are ink although they are painted shapes
because a shape with no label is identified by contrast alone, and the ink
holds more of it: on Dark card 5.92:1 against the fill's 3.94:1 for the 8px dot.
The range rides a 14 % ink track (`bg-ink/14`), 1.42:1 against card on Dark
and 1.35:1 on Light, and the indicator a 20 % tint of itself, 1.30:1 on
background at its tightest; the planes sit only 1.10:1 from `muted`, so a muted
track would vanish. A Checkbox and a
Switch stay the fill because their mark — the tick, the thumb — is
`on-primary`, so they identify themselves even where the fill does not clear
the surface.

**The floors.** The label on the fill ≥ 4.5:1. The fill against every surface
it is placed on ≥ 3:1. On Dark that is `background` (4.20:1), `secondary`
(3.67:1), `card` (3.94:1) and `popover` (3.30:1); Light is 3.88–4.62:1 and
Oldschool 3.01:1 on `popover`. `muted` is under the floor only in Oldschool
(2.55:1). It is the palette's well, not a plane a screen is laid out on — the
`TABS_LIST` behind the triggers, the `DRAWER_HANDLE` bar, a `Muted` band — and
a fill that does land there (a Switch or a Button in a `Muted` band) is never a
bare shape: its `on-primary` mark — a label, a tick, a thumb — identifies it,
which is what WCAG 1.4.11 asks of a control. The ink on `background` and `card` ≥ 4.5:1. `--ring` is the ink, and the test holds that
too: a focus indicator needs the 3:1 the ink gives on every surface, not the
2.54:1 the fill gives on muted.

**An outline follows what it touches.** A border adjoining its own fill takes
the fill's colour — `border-primary` on a checked Checkbox — and a border
adjoining an ink shape takes the ink — `border-primary-ink` on the Slider
thumb riding its ink range. An outline on a surface with no fill inside it is
ink: `border-primary-ink` on a checked Field.

**Planes are separated by a hairline.** The u2 planes sit 1.04–1.14:1 apart on
purpose: the design is flat, with no tinted fills and no shadow. The floor is
therefore stated for the `--border` hairline, not the planes: planes are separated by a
hairline of at least 1.3:1 against each plane it sits on, and adjacent planes may
sit below 1.2:1 (measured 1.37–1.46:1 on
Dark, 1.35:1 on Light, 1.37–1.45:1 on Oldschool). A text
tier is held at 4.5:1 on all four planes, and the disabled ink `ink-faint` is
exempt because nothing can be done with it. `border-strong` (about 2:1) is for
state and overlay edges, never for a control: a control edge stays `--input`.

**What lives where.** The measured values live beside the tokens in
`ev.css`; the track and indicator pairs beside their classes in
`rust/classes/src/{slider,progress}.rs`; every floor above is asserted in
`ts/uikit/test/tokens.test.ts`, so a retune of one value cannot quietly move
the other. The 3:1 the fill needs against `popover` pins `card` from above.
The destructive row's text is held at 3:1 on its tint and on `popover`, and
a Toggle's on-vs-off at the fill's 3:1, for every palette the test measures.
