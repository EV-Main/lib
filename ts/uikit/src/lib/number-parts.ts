/** A digit column or a fixed piece of a formatted number, with a stable identity across values. */
export type NumberPart =
  | { kind: "digit"; key: string; digit: number }
  | { kind: "text"; key: string; text: string };

export type RollDirection = 1 | -1 | 0;

/** The ten digit glyphs of the formatter's numbering system ("٠".."٩" in Arabic locales). */
export function digitGlyphs(formatter: Intl.NumberFormat): readonly string[] {
  const { numberingSystem } = formatter.resolvedOptions();
  const plain = new Intl.NumberFormat("en", { useGrouping: false, numberingSystem });
  return Array.from({ length: 10 }, (_, d) => plain.format(d));
}

/**
 * Splits a formatted number into parts keyed by what they are, not where they
 * sit in the string. Integer digits are keyed by place value counted from the
 * right, fraction digits by position after the point: adding a leading digit
 * (9 -> 10) leaves every existing column's key alone, so React keeps those
 * nodes and only the new place enters. Everything else is keyed by type, the
 * side of the integer it sits on, and its order among same-type parts there.
 */
export function splitNumber(
  formatter: Intl.NumberFormat,
  value: number,
  glyphs: readonly string[],
): NumberPart[] {
  const values = new Map(glyphs.map((glyph, digit) => [glyph, digit] as const));
  const raw = formatter.formatToParts(value);
  const integerDigits = raw
    .filter((p) => p.type === "integer")
    .reduce((n, p) => n + Array.from(p.value).length, 0);

  const parts: NumberPart[] = [];
  const seen = new Map<string, number>();
  const textKey = (type: string, side: string): string => {
    const id = `${type}:${side}`;
    const n = seen.get(id) ?? 0;
    seen.set(id, n + 1);
    return `${id}:${n}`;
  };
  let side = "pre";
  let placed = 0;
  let fractionAt = 0;

  for (const { type, value: text } of raw) {
    if (type === "integer") {
      side = "post";
      for (const char of Array.from(text)) {
        const digit = values.get(char);
        const place = integerDigits - 1 - placed;
        placed += 1;
        parts.push(
          digit === undefined
            ? { kind: "text", key: textKey("integer", side), text: char }
            : { kind: "digit", key: `int:${place}`, digit },
        );
      }
    } else if (type === "fraction") {
      for (const char of Array.from(text)) {
        const digit = values.get(char);
        const at = fractionAt;
        fractionAt += 1;
        parts.push(
          digit === undefined
            ? { kind: "text", key: textKey("fraction", side), text: char }
            : { kind: "digit", key: `frac:${at}`, digit },
        );
      }
    } else if (type === "group") {
      parts.push({ kind: "text", key: `group:${integerDigits - 1 - placed}`, text });
    } else if (type === "decimal") {
      side = "post";
      parts.push({ kind: "text", key: "decimal", text });
    } else {
      parts.push({ kind: "text", key: textKey(type, side), text });
    }
  }
  return parts;
}

/**
 * Signed number of steps a digit column travels. `1` always goes up (9 -> 0
 * wraps forward), `-1` always down, `0` takes the shorter way (a tie goes up).
 */
export function rollStep(from: number, to: number, direction: RollDirection): number {
  const up = (((to - from) % 10) + 10) % 10;
  const down = (10 - up) % 10;
  if (direction === 1) return up;
  if (direction === -1) return -down;
  return up <= down ? up : -down;
}

/** Which way the whole number moved: `trend` when given, else the sign of the change. */
export function resolveDirection(
  trend: RollDirection | undefined,
  from: number,
  to: number,
): RollDirection {
  if (trend !== undefined) return trend;
  return to > from ? 1 : to < from ? -1 : 0;
}

export interface NumberItem {
  part: NumberPart;
  /** Still on screen only to finish its exit; no longer in the value. */
  exiting: boolean;
  /** Enters from zero width; false for the first render and for reduced motion. */
  fresh: boolean;
}

/**
 * Carries the previous items into the next value. A key that is gone stays as
 * an exiting item in its old position (after the nearest surviving neighbour
 * to its left), so it can collapse instead of vanishing. A key that comes back
 * while exiting is reused, not remounted.
 */
export function mergeItems(
  previous: readonly NumberItem[],
  next: readonly NumberPart[],
  animate: boolean,
): NumberItem[] {
  const known = new Set(previous.map((item) => item.part.key));
  const result: NumberItem[] = next.map((part) => ({
    part,
    exiting: false,
    fresh: animate && !known.has(part.key),
  }));
  if (!animate) return result;

  const nextKeys = new Set(next.map((part) => part.key));
  let anchor = -1;
  for (const item of previous) {
    if (nextKeys.has(item.part.key)) {
      anchor = result.findIndex((r) => r.part.key === item.part.key);
    } else {
      anchor += 1;
      result.splice(anchor, 0, { part: item.part, exiting: true, fresh: false });
    }
  }
  return result;
}
