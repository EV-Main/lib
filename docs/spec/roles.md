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
