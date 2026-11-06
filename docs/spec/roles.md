# A class table names a role, never a magnitude

**Rule.** A height, a radius, a state fill, a tint or a surface in a class
table is a theme token named for its role. A literal magnitude there is a
token that is missing.

A theme reshapes the kit by writing a few inputs. A literal in a table is a
place the theme cannot reach, and a sibling that spells the same role in
another way drifts from it without anything noticing. `rust/tests/roles.rs`
fails on a literal; an exemption there says why its literal is no role.

## Control height

`--control-h` and `--control-step` are the inputs. The ramp is derived from
them:

```
xs = h − 2·step   sm = h − step   md = h   lg = h + step   xl = h + 3·step
touch = max(lg, 44 px)                       (the WCAG 2.5.5 pointer target)
```

A `Size` step names its rung and nothing else. A theme bends the ramp by
pinning one rung (`--control-h-xl`), never by giving a component its own
height. A component that sits off the ramp by design (a lone glyph two steps
under its labelled sibling) names a rung too.

A step names its text size with its rung (`xs` 12 px, `sm`/`md` 14 px,
`lg`/`xl`/`touch` 16 px). A glyph follows the text, not the box: it is
`1em + 2px`, so a lone glyph in a small square and a label's glyph in a tall
one stay the same pair.

## Radius

`--control-radius`, `--radius` and `--chip-radius` are the inputs; the kit
names four roles off them and the `full` shape:

| role | reads | wears it |
|---|---|---|
| `rounded-control` | `--control-radius` | anything pressed or typed into, and a glyph tile |
| `rounded-panel` | `--radius` | a surface: card, dialog, menu, popover, toast, banner |
| `rounded-row` | `--radius` − 4 px | a row inset in a panel's `p-1`, so the corners stay concentric |
| `rounded-chip` | `--chip-radius` | a label pill |

The `--radius-sm…2xl` steps stay in the sheet for a consumer's own markup.

## State on a surface

A state on a surface is the ladder, never a surface colour at an alpha:
`bg-hover` pointed at or open, `bg-active` pressed, `bg-selected` chosen or
current. A face with a fill of its own (a Secondary button, a tinted chip)
takes the hover as a layer over the fill
(`bg-[image:linear-gradient(var(--hover),var(--hover))]`), so the fill
survives it. A state on a role's solid fill (primary, danger, an accent)
belongs to that role.

## Surface

A fill names where the element sits, not how light it should look:

| role | fill | what sits there |
|---|---|---|
| page | `bg-background` | the plane a screen is laid out on, and a panel docked to its edge (sheet, drawer): part of the frame, one step off the scrim |
| raised | `bg-card` | a surface in flow: card, rail, bar, a chosen tab, the user's bubble |
| floating | `bg-popover` | an overlay: dialog, menu, popover, toast |
| well | `bg-muted` | a recess: a track, a muted band, a read-out |
| control | transparent, `--input` edge | an empty slot the user fills: input, textarea, select |

A thing holding a value of its own (a balance, a search trigger, an addon, an
account) keeps a fill: an empty inside reads as a slot to put something in. A
skeleton is a `bg-hover` tint over whatever it stands in for.

`secondary` is a role fill, the face of the Secondary and Outline buttons, and
never a plane.
