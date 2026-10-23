import { useEffect, useRef, useState, type RefObject } from "react";
import { useScopeTheme } from "./ThemeScope";

/**
 * The computed value of one CSS property on the element `ref` is attached to,
 * re-read when the theme changes (radius, depth and colour all follow it).
 * `prop` is the kebab-case CSS name.
 */
export function useComputedProp<E extends HTMLElement>(prop: string): [RefObject<E | null>, string] {
  const ref = useRef<E | null>(null);
  const theme = useScopeTheme();
  const [value, setValue] = useState("");
  useEffect(() => {
    if (ref.current !== null) setValue(getComputedStyle(ref.current).getPropertyValue(prop));
  }, [theme, prop]);
  return [ref, value];
}
