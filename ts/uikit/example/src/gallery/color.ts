export interface Rgba {
  r: number;
  g: number;
  b: number;
  a: number;
}

const NUM = String.raw`(-?[\d.]+(?:e-?\d+)?)`;
const RGB = new RegExp(String.raw`^rgba?\(\s*${NUM}[\s,]+${NUM}[\s,]+${NUM}(?:\s*[,/]\s*${NUM}(%?))?\s*\)$`);
const SRGB = new RegExp(String.raw`^color\(srgb\s+${NUM}\s+${NUM}\s+${NUM}(?:\s*/\s*${NUM}(%?))?\s*\)$`);

const alphaOf = (v: string | undefined, pct: string | undefined): number => {
  if (v === undefined) return 1;
  return pct === "%" ? Number(v) / 100 : Number(v);
};

/**
 * Parses what `getComputedStyle` returns for a colour: `rgb()`/`rgba()` for
 * plain values, `color(srgb …)` for `color-mix()` results. Channels are 0..1.
 */
export function parseColor(css: string): Rgba | null {
  const s = css.trim();
  const rgb = RGB.exec(s);
  if (rgb) {
    const [, r, g, b, a, pct] = rgb;
    return { r: Number(r) / 255, g: Number(g) / 255, b: Number(b) / 255, a: alphaOf(a, pct) };
  }
  const srgb = SRGB.exec(s);
  if (srgb) {
    const [, r, g, b, a, pct] = srgb;
    return { r: Number(r), g: Number(g), b: Number(b), a: alphaOf(a, pct) };
  }
  return null;
}

/** `fg` laid over an opaque `bg`. */
export function over(fg: Rgba, bg: Rgba): Rgba {
  const mix = (f: number, b: number) => f * fg.a + b * (1 - fg.a);
  return { r: mix(fg.r, bg.r), g: mix(fg.g, bg.g), b: mix(fg.b, bg.b), a: 1 };
}

const lin = (c: number) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);

export function luminance(c: Rgba): number {
  return 0.2126 * lin(c.r) + 0.7152 * lin(c.g) + 0.0722 * lin(c.b);
}

/** WCAG 2.x contrast ratio of two opaque colours. */
export function contrast(a: Rgba, b: Rgba): number {
  const la = luminance(a);
  const lb = luminance(b);
  return (Math.max(la, lb) + 0.05) / (Math.min(la, lb) + 0.05);
}

const hex2 = (c: number) =>
  Math.round(Math.min(1, Math.max(0, c)) * 255)
    .toString(16)
    .padStart(2, "0");

export function toHex(c: Rgba): string {
  const base = `#${hex2(c.r)}${hex2(c.g)}${hex2(c.b)}`;
  return c.a < 0.995 ? `${base}${hex2(c.a)}` : base;
}
