import { useCallback, useEffect, useRef, useState } from "react";
import { useReducedMotion } from "./useReducedMotion";

const TICK_MS = 45;
// The pause before the first word, so the "thinking" state is visible.
const THINK_TICKS = 20;

/**
 * Simulates a model answering: appends `text` word by word on an interval.
 * With reduced motion on it appends in bigger chunks so the text still arrives
 * but nothing flickers word by word.
 */
export function useTokenStream(text: string): {
  shown: string;
  streaming: boolean;
  start: () => void;
  stop: () => void;
} {
  const reduced = useReducedMotion();
  const words = useRef<string[]>([]);
  const [count, setCount] = useState(0);
  const [streaming, setStreaming] = useState(false);

  const start = useCallback(() => {
    words.current = text.split(" ");
    setCount(0);
    setStreaming(true);
  }, [text]);
  const stop = useCallback(() => setStreaming(false), []);

  useEffect(() => {
    if (!streaming) return;
    const step = reduced ? 8 : 1;
    // the interval owns the running count; state only mirrors it for rendering
    let at = count;
    let wait = count === 0 ? THINK_TICKS : 0;
    const id = setInterval(() => {
      if (wait > 0) {
        wait -= 1;
        return;
      }
      at = Math.min(words.current.length, at + step);
      setCount(at);
      if (at >= words.current.length) {
        setStreaming(false);
        clearInterval(id);
      }
    }, TICK_MS);
    return () => clearInterval(id);
    // `count` is read once at (re)start; re-running on every tick would reset the interval
  },[streaming, reduced]);

  return { shown: words.current.slice(0, count).join(" "), streaming, start, stop };
}
