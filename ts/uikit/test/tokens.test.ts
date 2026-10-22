import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it, expect } from "vitest";

import { buttonVariants } from "../src/components/button";
import { CHECKBOX_BASE } from "../src/generated/checkbox";
import { CONTEXT_MENU_CHECK_ITEM, CONTEXT_MENU_ITEM, CONTEXT_MENU_SUB_TRIGGER } from "../src/generated/context-menu";
import { DROPDOWN_MENU_CHECK_ITEM, DROPDOWN_MENU_ITEM, DROPDOWN_MENU_SUB_TRIGGER } from "../src/generated/dropdown-menu";
import { OPTION_FOCUS_RING } from "../src/generated/focus";
import { INPUT_BASE } from "../src/generated/input";
import { INPUT_GROUP_BASE } from "../src/generated/input-group";
import { INPUT_OTP_SLOT } from "../src/generated/input-otp";
import {
  MENUBAR_CHECKBOX_ITEM,
  MENUBAR_ITEM,
  MENUBAR_RADIO_ITEM,
  MENUBAR_SUB_TRIGGER,
  MENUBAR_TRIGGER,
} from "../src/generated/menubar";
import { PROGRESS_INDICATOR, PROGRESS_TRACK } from "../src/generated/progress";
import { SLIDER_RANGE, SLIDER_TRACK } from "../src/generated/slider";
import { SWITCH_BASE, SWITCH_THUMB } from "../src/generated/switch";
import { TEXTAREA_BASE } from "../src/generated/textarea";
import { RADIO_GROUP_ITEM } from "../src/generated/radio-group";
import { SELECT_ITEM } from "../src/generated/select";
import { TOGGLE_BASE, toggleVariantClasses } from "../src/generated/toggle";
import { brandFromToml, DERIVED_SCOPE, readContract, readRules, renderPalette, type CssRule } from "../src/palette";

// The shipped sheet, not the repo-root source: a consumer imports this one.
// Resolved from cwd rather than import.meta.url: jsdom rewrites the module URL
// to http://, which `new URL(...)` then refuses to read from disk. The cwd is
// `ts/uikit` — vitest runs from the package root (`npm test`), which is where
// `styles/tokens.css` resolves; run it from anywhere else and the read fails
// loudly rather than measuring the wrong sheet.
const sheet = readFileSync(join(process.cwd(), "styles/tokens.css"), "utf8");
const contract = readContract(sheet);

/** One set of values a surface can be painted with: a palette in one polarity. */
interface Scope {
  label: string;
  rule: CssRule;
}

function scopeOf(rules: CssRule[], selector: string, label: string): Scope {
  const rule = rules.find((r) => r.selector === selector);
  if (!rule) throw new Error(`no rule \`${selector}\` for ${label}`);
  return { label, rule };
}

const demoBrand = "aquafix-demo";
const demoRules = readRules(
  renderPalette(
    demoBrand,
    brandFromToml(readFileSync(join(process.cwd(), "test/fixtures/aquafix-demo.brand.toml"), "utf8")),
    contract,
  ),
);
const at = `[data-brand="${demoBrand}"]`;

const sheetRules = readRules(sheet);
// EV's rules do not match inside another brand's scope or on its element.
const NOT_A_BRAND = ':not([data-brand]:not([data-brand="ev"]), [data-brand]:not([data-brand="ev"]) *)';
const EV_DARK_SELECTOR = `:where(:is(:root, [data-brand="ev"], .dark)${NOT_A_BRAND})`;
const EV_LIGHT_SELECTOR = `:where(.light${NOT_A_BRAND})`;
const EV_OLDSCHOOL_SELECTOR = ':where([data-theme="oldschool"], [data-theme="oldschool"] :is(.light, .dark))';
const FACES_SELECTOR = ":where([data-theme], [data-brand], .light, .dark, :root)";

const EV_DARK = scopeOf(sheetRules, EV_DARK_SELECTOR, "ev dark");
const EV_LIGHT = scopeOf(sheetRules, EV_LIGHT_SELECTOR, "ev light");
const EV_OLDSCHOOL = scopeOf(sheetRules, EV_OLDSCHOOL_SELECTOR, "ev oldschool");
// Every palette the kit measures, in every polarity it declares. Oldschool is
// single-polarity: one scope serves both classes.
const scopes: Scope[] = [
  EV_DARK,
  EV_LIGHT,
  EV_OLDSCHOOL,
  scopeOf(demoRules, `${at}, ${at}.light, ${at} .light`, `${demoBrand} light`),
  scopeOf(demoRules, `${at}.dark, ${at} .dark`, `${demoBrand} dark`),
];

/** `--name` as a flat hex in `scope`, following `var(--other)` within it. */
function token(name: string, scope: Scope): string {
  const value = scope.rule.declarations.get(name);
  const ref = value && /^var\(--([a-z0-9-]+)\)$/.exec(value);
  if (ref?.[1]) return token(ref[1], scope);
  if (!value || !/^#[0-9a-f]{6}$/i.test(value)) throw new Error(`--${name} is not a plain hex token in ${scope.label}`);
  return value.toLowerCase();
}

const derivedRule = sheetRules.find((r) => r.selector === DERIVED_SCOPE);
const facesRule = sheetRules.find((r) => r.selector === FACES_SELECTOR);

/** The value a scope gives `--name`: its own, else the contract's formula. */
function declared(name: string, scope: Scope): string {
  const value = scope.rule.declarations.get(name) ?? derivedRule?.declarations.get(name) ?? facesRule?.declarations.get(name);
  if (value === undefined) throw new Error(`--${name} is neither pinned in ${scope.label} nor derived by the contract`);
  return value;
}

/**
 * `--name` as it is seen over `surface` in `scope`: a flat hex, an 8-digit hex
 * or an sRGB `color-mix` of tokens, resolved the way the browser does and
 * composited onto the plane under it. A translucent line is only ever seen over
 * whatever is below, so measuring it without the plane would lie.
 */
function paint(name: string, scope: Scope, surface: string): string {
  return paintValue(declared(name, scope), scope, surface);
}

function paintValue(value: string, scope: Scope, surface: string): string {
  const ref = /^var\(--([a-z0-9-]+)\)$/.exec(value);
  if (ref?.[1]) return paint(ref[1], scope, surface);
  if (value === "black") return "#000000";
  if (value === "transparent") return surface;
  if (/^#[0-9a-f]{6}$/i.test(value)) return value.toLowerCase();
  const rgba = /^(#[0-9a-f]{6})([0-9a-f]{2})$/i.exec(value);
  if (rgba?.[1] && rgba[2]) return composite(rgba[1].toLowerCase(), parseInt(rgba[2], 16) / 255, surface);
  const mix = /^color-mix\(in srgb, (.+?) (\d+(?:\.\d+)?%|var\(--[a-z0-9-]+\)), (.+)\)$/.exec(value);
  if (!mix?.[1] || !mix[2] || !mix[3]) throw new Error(`cannot paint ${value} in ${scope.label}`);
  const amount = mix[2].startsWith("var(") ? declared(/--([a-z0-9-]+)/.exec(mix[2])?.[1] ?? "", scope) : mix[2];
  const share = Number.parseFloat(amount) / 100;
  const under = mix[3] === "transparent" ? surface : paintValue(mix[3], scope, surface);
  return composite(paintValue(mix[1], scope, surface), share, under);
}

/**
 * `--ring` as painted in `scope`: the palette's own value when it pins one,
 * else the contract's formula resolved against the palette (`var(--primary-ink)`
 * today) — so a retune of either side is what gets measured.
 */
function ring(scope: Scope): string {
  return paint("ring", scope, token("background", scope));
}

/** A line or a tint (`--input`, `--border`, `--hover`) over `surface`. */
function line(name: string, scope: Scope, surface: string): string {
  return paint(name, scope, surface);
}

/** A derived ink (`--accent-error-ink`): two opaque colours, so no plane shows through. */
function derivedInk(name: string, scope: Scope): string {
  return paint(name, scope, token("background", scope));
}

function luminance(hex: string): number {
  const channel = (i: number) => {
    const c = parseInt(hex.slice(i, i + 2), 16) / 255;
    return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5);
}

function contrast(a: string, b: string): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x) as [number, number];
  return (hi + 0.05) / (lo + 0.05);
}

// `bg-x/N` painted over an opaque surface, the way the browser composites it
// in sRGB. Naive alpha lies about contrast; this is the colour that is measured.
function composite(fg: string, alpha: number, bg: string): string {
  const channel = (i: number) => {
    const f = parseInt(fg.slice(i, i + 2), 16);
    const b = parseInt(bg.slice(i, i + 2), 16);
    return Math.round(f * alpha + b * (1 - alpha))
      .toString(16)
      .padStart(2, "0");
  };
  return `#${channel(1)}${channel(3)}${channel(5)}`;
}

describe.each(scopes)("palette $label", (scope) => {
  const t = (name: string) => token(name, scope);

  it("declares every name the contract requires", () => {
    const missing = contract.required.filter((name) => !scope.rule.declarations.has(name));
    expect(missing).toEqual([]);
  });

  // The floors docs/spec/accents.md commits to for a role whose fill and ink
  // diverge. Measured, because a retune of one value quietly moves the other.
  describe("primary fill vs ink", () => {
    it("the label reads on the fill (AA text)", () => {
      expect(contrast(t("primary"), t("on-primary"))).toBeGreaterThanOrEqual(4.5);
    });

    it.each(["background", "secondary", "card", "popover"])("the fill reads as a shape on %s (non-text 3:1)", (surface) => {
      expect(contrast(t("primary"), t(surface))).toBeGreaterThanOrEqual(3);
    });

    it.each(["background", "card"])("the ink reads as text on %s (AA text)", (surface) => {
      expect(contrast(t("primary-ink"), t(surface))).toBeGreaterThanOrEqual(4.5);
    });

    it.each(["background", "card"])("body text reads on %s (AA text)", (surface) => {
      expect(contrast(t("ink"), t(surface))).toBeGreaterThanOrEqual(4.5);
    });
  });

  // lib#129. A filled control's focus indicator is the solid ring 2px off the
  // control, so both of its edges are against the surface: ring | surface gap |
  // fill. The fill-vs-surface edge is the "fill reads as a shape" floor above;
  // this is the other one. The 50 % halo it replaced measured ~2:1 here.
  describe("focus ring on a filled control", () => {
    it.each(["background", "secondary", "card", "popover"])("the ring reads against %s (non-text 3:1)", (surface) => {
      expect(contrast(ring(scope), t(surface))).toBeGreaterThanOrEqual(3);
    });

    it("stands off the fill, so the ring is never measured against it", () => {
      for (const classes of [
        buttonVariants({ variant: "primary" }),
        buttonVariants({ variant: "destructive" }),
        buttonVariants({ variant: "ghost", accent: "warn" }),
      ]) {
        expect(classes).toMatch(/\bfocus-visible:outline-offset-2\b/);
        expect(classes).toMatch(/\bfocus-visible:outline-ring\b/);
        expect(classes).not.toMatch(/focus-visible:ring-\[3px\]/);
      }
    });
  });

  // lib#154. `--input` is the visible boundary of a control that has no fill
  // (outline button, input, select, checkbox, radio), so it owes the non-text
  // 3:1 on every surface a control sits on. `--border` stays a divider: it is
  // decoration, and at 3:1 every card edge would shout.
  describe("control boundary", () => {
    it.each(["background", "secondary", "card", "popover"])("--input reads against %s (non-text 3:1)", (surface) => {
      expect(contrast(line("input", scope, t(surface)), t(surface))).toBeGreaterThanOrEqual(3);
    });

    it("the unchecked switch thumb still reads on its --input track", () => {
      expect(SWITCH_BASE).toMatch(/\bdata-\[state=unchecked\]:bg-input\b/);
      expect(SWITCH_THUMB).toMatch(/\bdata-\[state=unchecked\]:bg-ink\b/);
      for (const surface of ["background", "card"]) {
        expect(contrast(t("ink"), line("input", scope, t(surface)))).toBeGreaterThanOrEqual(3);
      }
    });
  });

  // An open list's focused row: the `bg-hover` tint alone is a surface step
  // (1.22:1 on EV's popover, 1.14:1 and 1.00:1 where a brand pins --hover), so
  // the indicator is the inset --ring, which sits between the tint inside the
  // row and the popover around it and owes 3:1 to both.
  describe("focused option in an open list", () => {
    const tint = () => line("hover", scope, t("popover"));

    it("the ring reads against the popover and against the tint (non-text 3:1)", () => {
      expect(contrast(ring(scope), t("popover"))).toBeGreaterThanOrEqual(3);
      expect(contrast(ring(scope), tint())).toBeGreaterThanOrEqual(3);
    });

    it("the label reads on the tint (AA text)", () => {
      expect(contrast(t("ink"), tint())).toBeGreaterThanOrEqual(4.5);
    });

    // A destructive menu row swaps the tint for 10 % of --accent-error, so the
    // ring's inner edge sits on that instead.
    it("the ring reads against a destructive row's tint (non-text 3:1)", () => {
      for (const classes of [DROPDOWN_MENU_ITEM, CONTEXT_MENU_ITEM, MENUBAR_ITEM]) {
        expect(classes).toMatch(/(^|\s)data-\[variant=destructive\]:focus:bg-accent-error\/10(\s|$)/);
      }
      const destructive = composite(t("accent-error"), 0.1, t("popover"));
      expect(contrast(ring(scope), destructive)).toBeGreaterThanOrEqual(3);
    });
  });

  // lib#166. A destructive row is text on 10 % of the error role, so it reads
  // in the role's ink: the fill itself measured 3.76:1 there on EV and 2.48:1
  // on the demo's dark side. The row sits on the popover unfocused, so it owes
  // AA there too.
  describe("destructive menu row", () => {
    const ink = () => derivedInk("accent-error-ink", scope);

    it("the text reads on the row's tint (AA text)", () => {
      expect(contrast(ink(), composite(t("accent-error"), 0.1, t("popover")))).toBeGreaterThanOrEqual(4.5);
    });

    it("the text reads on the popover at rest (AA text)", () => {
      expect(contrast(ink(), t("popover"))).toBeGreaterThanOrEqual(4.5);
    });
  });

  // lib#175. "On" is the primary fill and "off" leaves the surface showing, so
  // the state is told apart by the fill against each plane a toggle sits on.
  // Hover paints a surface tint (bg-muted, or bg-hover on outline), never the
  // fill, so a pointed-at item cannot pass for a selected one.
  describe("toggle on vs off", () => {
    it.each(["background", "secondary", "card", "popover"])("on reads against off on %s (non-text 3:1)", (surface) => {
      expect(TOGGLE_BASE).toMatch(/(^|\s)data-\[state=on\]:bg-primary(\s|$)/);
      expect(contrast(t("primary"), t(surface))).toBeGreaterThanOrEqual(3);
    });

    it("the on label reads on the fill (AA text)", () => {
      expect(TOGGLE_BASE).toMatch(/(^|\s)data-\[state=on\]:text-on-primary(\s|$)/);
      expect(contrast(t("on-primary"), t("primary"))).toBeGreaterThanOrEqual(4.5);
    });
  });

  // A track is a shape under a shape, so it owes two floors at once: the moving
  // part reads on it (3:1) and it reads against the plane it sits on (~1.3:1, or
  // the UI goes flat). Both are painted by the class tables, so the classes are
  // asserted alongside the numbers they were chosen for. The u2 planes sit only
  // 1.10:1 apart, so a `bg-muted` track would vanish: it is 14 % of the ink.
  describe("slider and progress tracks", () => {
    const track = (surface: string) => composite(t("ink"), 0.14, t(surface));

    it("the slider range is the ink on a 14% ink track", () => {
      expect(SLIDER_RANGE).toMatch(/\bbg-primary-ink\b/);
      expect(SLIDER_TRACK).toMatch(/\bbg-ink\/14\b/);
      for (const surface of ["background", "card"]) {
        expect(contrast(t("primary-ink"), track(surface))).toBeGreaterThanOrEqual(3);
        expect(contrast(track(surface), t(surface))).toBeGreaterThanOrEqual(1.3);
      }
    });

    it("the progress indicator is the ink on the same track", () => {
      expect(PROGRESS_INDICATOR).toMatch(/\bbg-primary-ink\b/);
      expect(PROGRESS_TRACK).toMatch(/\bbg-ink\/14\b/);
      for (const surface of ["background", "card"]) {
        expect(contrast(t("primary-ink"), track(surface))).toBeGreaterThanOrEqual(3);
        expect(contrast(track(surface), t(surface))).toBeGreaterThanOrEqual(1.3);
      }
    });
  });

  // 1.0: the planes are flat steps (1.04-1.14:1), so what separates them is the
  // hairline. AGENTS.md states the floor as the hairline's, not the planes'.
  describe("planes are separated by a hairline", () => {
    it.each(["background", "secondary", "card", "popover"])("--border reads against %s (>= 1.3:1)", (surface) => {
      expect(contrast(line("border", scope, t(surface)), t(surface))).toBeGreaterThanOrEqual(1.3);
    });
  });

  describe("text tiers", () => {
    it.each(["background", "secondary", "card", "popover"])("ink-mid and ink-soft read on %s (AA text)", (surface) => {
      expect(contrast(line("ink-mid", scope, t(surface)), t(surface))).toBeGreaterThanOrEqual(4.5);
      expect(contrast(line("ink-soft", scope, t(surface)), t(surface))).toBeGreaterThanOrEqual(4.5);
    });

    it.each(["card", "popover"])("the label reads on a hovered, pressed or selected row of %s (AA text)", (surface) => {
      for (const state of ["hover", "active", "selected"]) {
        expect(contrast(t("ink"), line(state, scope, t(surface)))).toBeGreaterThanOrEqual(4.5);
      }
    });

    it.each(["positive-ink", "accent-info-ink", "accent-warn-ink", "accent-error-ink"])("%s reads as text on the planes (AA text)", (name) => {
      for (const surface of ["background", "card", "popover"]) {
        expect(contrast(paint(name, scope, t(surface)), t(surface))).toBeGreaterThanOrEqual(4.5);
      }
    });

    it("the solid primary hover still carries the label (AA text)", () => {
      expect(contrast(t("on-primary"), paint("primary-hover", scope, t("card")))).toBeGreaterThanOrEqual(4.5);
    });
  });

  // 1.0 Button faces. Each is a background that may be translucent, a hover
  // painted over it, an edge and a label; all four are resolved over the plane
  // the button sits on.
  describe("button faces", () => {
    const faces = ["primary", "danger"] as const;

    it.each(faces)("the %s label reads on the face and on its hover (AA text)", (face) => {
      for (const surface of ["background", "secondary", "card", "popover"]) {
        const plane = t(surface);
        const bg = paint(`button-${face}-bg`, scope, plane);
        const hover = paint(`button-${face}-hover`, scope, bg);
        const ink = paint(`button-${face}-ink`, scope, bg);
        expect(contrast(ink, bg)).toBeGreaterThanOrEqual(4.5);
        expect(contrast(paint(`button-${face}-ink`, scope, hover), hover)).toBeGreaterThanOrEqual(4.5);
      }
    });

    it.each(faces)("the %s edge reads against the plane (non-text 3:1)", (face) => {
      for (const surface of ["background", "secondary", "card", "popover"]) {
        const plane = t(surface);
        expect(contrast(paint(`button-${face}-border`, scope, plane), plane)).toBeGreaterThanOrEqual(3);
      }
    });
  });

  // 1.0 Chips. A tinted fill (or none) and an edge in the role's hue, with the
  // role's ink as the label. Only the label owes AA; the edge is decoration.
  describe("chip tints", () => {
    const tones = [
      ["neutral", "ink-mid"],
      ["primary", "primary-ink"],
      ["positive", "positive-ink"],
      ["info", "accent-info-ink"],
      ["warn", "accent-warn-ink"],
      ["error", "accent-error-ink"],
    ] as const;

    // background and card, as for the other text inks: Oldschool's teal ink is
    // 4.52:1 on its popover bare, so no tint can sit under it.
    it.each(tones)("the %s label reads on its fill over the planes (AA text)", (tone, ink) => {
      for (const surface of ["background", "card"]) {
        const plane = t(surface);
        const fill = paint(`chip-${tone}-bg`, scope, plane);
        expect(contrast(paint(ink, scope, fill), fill)).toBeGreaterThanOrEqual(4.5);
      }
    });
  });

  describe("scrim", () => {
    it("darkens the page behind a modal", () => {
      const page = t("background");
      expect(luminance(paint("scrim", scope, page))).toBeLessThan(luminance(page));
    });
  });
});

// The floor above is only worth something if the controls draw their frame with
// the token it measures, not with the divider.
describe("outlined controls frame with --input", () => {
  it.each([
    ["button outline", buttonVariants({ variant: "outline" })],
    ["toggle outline", toggleVariantClasses.outline],
    ["input", INPUT_BASE],
    ["textarea", TEXTAREA_BASE],
    ["checkbox", CHECKBOX_BASE],
    ["radio", RADIO_GROUP_ITEM],
    ["input group", INPUT_GROUP_BASE],
    ["otp slot", INPUT_OTP_SLOT],
  ])("%s", (_, classes) => {
    expect(classes).toMatch(/(^|\s)border-input(\s|$)/);
    expect(classes).not.toMatch(/(^|\s)border-border(\s|$)/);
  });
});

// The destructive-row floor measures --accent-error-ink, so the rows have to
// paint their text and icon with it rather than with the fill.
describe("destructive menu rows read in the error ink", () => {
  it.each([
    ["dropdown menu item", DROPDOWN_MENU_ITEM],
    ["context menu item", CONTEXT_MENU_ITEM],
    ["menubar item", MENUBAR_ITEM],
  ])("%s", (_, classes) => {
    expect(classes).toMatch(/(^|\s)data-\[variant=destructive\]:text-accent-error-ink(\s|$)/);
    expect(classes).toMatch(/(^|\s)data-\[variant=destructive\]:focus:text-accent-error-ink(\s|$)/);
    expect(classes).toMatch(/(^|\s)data-\[variant=destructive\]:\*:\[svg\]:!text-accent-error-ink(\s|$)/);
    expect(classes).not.toMatch(/text-accent-error(\s|$)/);
  });
});

// The on-vs-off floor holds only while hover stays a surface tint.
describe("toggle hover is not the on state", () => {
  it.each([
    ["bare", `${TOGGLE_BASE} ${toggleVariantClasses.bare}`],
    ["outline", `${TOGGLE_BASE} ${toggleVariantClasses.outline}`],
  ])("%s", (_, classes) => {
    const hover = classes.split(/\s+/).filter((c) => c.startsWith("hover:bg-"));
    expect(hover.length).toBeGreaterThan(0);
    expect(hover).not.toContain("hover:bg-primary");
    expect(classes).not.toMatch(/(^|\s)data-\[state=on\]:bg-hover(\s|$)/);
  });

  it("an outline toggle's frame follows its fill when on", () => {
    expect(toggleVariantClasses.outline).toMatch(/(^|\s)data-\[state=on\]:border-primary(\s|$)/);
  });
});

// The floor above is only worth something if the rows draw their focus with
// the ring it measures, not with the tint alone.
describe("list rows ring their keyboard focus", () => {
  it("is the inset --ring, on focus-visible", () => {
    expect(OPTION_FOCUS_RING).toMatch(/\bfocus-visible:inset-ring-2\b/);
    expect(OPTION_FOCUS_RING).toMatch(/\bfocus-visible:inset-ring-ring\b/);
  });

  it.each([
    ["select item", SELECT_ITEM],
    ["dropdown menu item", DROPDOWN_MENU_ITEM],
    ["dropdown menu check item", DROPDOWN_MENU_CHECK_ITEM],
    ["dropdown menu sub trigger", DROPDOWN_MENU_SUB_TRIGGER],
    ["context menu item", CONTEXT_MENU_ITEM],
    ["context menu check item", CONTEXT_MENU_CHECK_ITEM],
    ["context menu sub trigger", CONTEXT_MENU_SUB_TRIGGER],
    ["menubar trigger", MENUBAR_TRIGGER],
    ["menubar item", MENUBAR_ITEM],
    ["menubar checkbox item", MENUBAR_CHECKBOX_ITEM],
    ["menubar radio item", MENUBAR_RADIO_ITEM],
    ["menubar sub trigger", MENUBAR_SUB_TRIGGER],
  ])("%s", (_, classes) => {
    expect(classes).toContain(OPTION_FOCUS_RING);
  });
});

describe("the contract half of the sheet", () => {
  const rules = readRules(sheet);

  it("derives the states, lines, inks and status inks rather than asking a palette for them", () => {
    expect([...contract.derived].sort()).toEqual([
      "accent-error-ink",
      "accent-info-ink",
      "accent-warn-ink",
      "active",
      "border",
      "border-strong",
      "border-subtle",
      "hover",
      "ink-faint",
      "ink-mid",
      "ink-soft",
      "input",
      "positive-ink",
      "primary-border",
      "primary-hover",
      "ring",
      "scrim",
      "selected",
    ]);
  });

  it("derives the button faces and chips for a brand that pins none", () => {
    const names = [...(facesRule?.declarations.keys() ?? [])];
    expect(names).toEqual(
      expect.arrayContaining([
        "button-primary-bg",
        "button-primary-hover",
        "button-primary-border",
        "button-primary-ink",
        "button-danger-bg",
        "button-danger-hover",
        "button-danger-border",
        "button-danger-ink",
        "chip-neutral-bg",
        "chip-primary-border",
        "chip-error-bg",
      ]),
    );
  });

  it("the focus ring is the ink", () => {
    expect(sheet).toMatch(/^\s*--ring:\s*var\(--primary-ink\);/m);
  });

  // EV binds both polarity classes now, so its pins would reach a brand's own
  // `.light` / `.dark` subtree (and a brand element that carries the class
  // itself) unless the rules stay out of every brand but EV.
  it("keeps EV's polarity rules out of a brand's subtree and off its element", () => {
    const palettes = rules.filter((r) => r.declarations.has("background"));
    expect(palettes.map((r) => r.selector)).toEqual([EV_DARK_SELECTOR, EV_LIGHT_SELECTOR, EV_OLDSCHOOL_SELECTOR]);
    for (const selector of [EV_DARK_SELECTOR, EV_LIGHT_SELECTOR]) {
      expect(selector).toContain('[data-brand]:not([data-brand="ev"]) *');
      expect(selector).toContain('[data-brand]:not([data-brand="ev"])');
      expect(selector.startsWith(":where(")).toBe(true);
    }
  });

  // A nested wrapper replaces its parent's theme only if it sets everything the
  // parent set; a token it leaves out would show through from the wrapper above.
  it("pins the same tokens in Dark and Light, and all of them in Oldschool", () => {
    const dark = [...EV_DARK.rule.declarations.keys()].sort();
    expect([...EV_LIGHT.rule.declarations.keys()].sort()).toEqual(dark);
    expect(dark.filter((n) => !EV_OLDSCHOOL.rule.declarations.has(n))).toEqual([]);
  });

  it("gives Oldschool its shapes and depth, and leaves Dark and Light flat and square", () => {
    for (const shape of ["radius", "control-radius", "chip-radius", "depth-card", "depth-elevated", "depth-overlay"]) {
      expect(EV_OLDSCHOOL.rule.declarations.has(shape)).toBe(true);
      expect(EV_DARK.rule.declarations.has(shape)).toBe(false);
    }
    expect(EV_OLDSCHOOL.rule.declarations.get("chip-radius")).toBe("9999px");
  });

  // `none` inside Tailwind's comma-separated box-shadow list invalidates the
  // whole declaration, and the focus ring is in that list.
  it("makes the flat depth a zero shadow, never `none`", () => {
    const geometry = rules.find((r) => r.selector === ":where(:root, [data-brand])");
    for (const tier of ["card", "elevated", "overlay"]) {
      expect(geometry?.declarations.get(`depth-${tier}`)).toBe("0 0 #0000");
    }
    expect(sheet).toMatch(/--shadow-overlay:\s*var\(--depth-overlay\);/);
  });

  it("styles the text scale with a line height each", () => {
    for (const step of ["caption", "body-s", "body", "body-l", "h3", "h2", "h1", "figure-m", "figure-l"]) {
      expect(sheet).toMatch(new RegExp(`--text-${step}:\\s*[\\d.]+rem;`));
      expect(sheet).toMatch(new RegExp(`--text-${step}--line-height:\\s*[\\d.]+rem;`));
    }
  });

  it("scales the geometry up on a brand scope as it does on the root", () => {
    const bumps = rules.filter((r) => r.declarations.has("page-px") && r.declarations.get("page-px") !== "1rem");
    expect(bumps.length).toBeGreaterThan(0);
    for (const bump of bumps) expect(bump.selector).toBe(":where(:root, [data-brand])");
  });
});
