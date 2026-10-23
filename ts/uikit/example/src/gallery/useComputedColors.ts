import { useEffect, useRef, useState, type RefObject } from "react";
import { parseColor, type Rgba } from "./color";
import { useScopeTheme } from "./ThemeScope";

/**
 * Resolves contract tokens to real colours by asking the browser, so the page
 * shows what the theme computes (including `color-mix()` and alpha) and not a
 * second copy of the table. Attach `probe` to a hidden element inside the
 * scope being measured; a theme change re-reads, because the tokens are
 * inherited from the scope's wrapper or from `<html>`.
 */
export function useComputedColors(names: readonly string[]): {
  probe: RefObject<HTMLSpanElement | null>;
  colors: ReadonlyMap<string, Rgba>;
} {
  const probe = useRef<HTMLSpanElement | null>(null);
  const theme = useScopeTheme();
  const [colors, setColors] = useState<ReadonlyMap<string, Rgba>>(new Map());

  useEffect(() => {
    const el = probe.current;
    if (el === null) return;
    const next = new Map<string, Rgba>();
    for (const name of names) {
      el.style.color = `var(--${name})`;
      const parsed = parseColor(getComputedStyle(el).color);
      if (parsed) next.set(name, parsed);
    }
    el.style.color = "";
    // reading computed style is the sync with the outside world; the theme is the trigger
    setColors(next);
  }, [theme, names]);

  return { probe, colors };
}
