"use client";

import * as React from "react";

// Words in one chunk start this far apart, up to STAGGER_CAP of them, so a burst
// of text reads as a ripple instead of landing at once.
const STAGGER_STEP_MS = 28;
const STAGGER_CAP = 6;
// Safety net for when `animationend` never fires (hidden tab, display: none).
const SETTLE_FALLBACK_MS = 900;

interface Unit {
  word: string;
  /** The whitespace that follows the word, kept outside the animated span. */
  gap: string;
}

function split(text: string): { lead: string; units: Unit[] } {
  const lead = /^\s*/.exec(text)?.[0] ?? "";
  const units: Unit[] = [];
  for (const m of text.slice(lead.length).matchAll(/(\S+)(\s*)/g)) units.push({ word: m[1] ?? "", gap: m[2] ?? "" });
  return { lead, units };
}

interface Progress {
  text: string;
  /** Units before this index are plain text; the rest are animating. */
  settled: number;
  /** Start delay per unit, fixed when the unit appears so re-renders never restart it. */
  delays: number[];
}

function delaysFor(from: number, to: number): number[] {
  return Array.from({ length: to - from }, (_, j) => Math.min(j, STAGGER_CAP) * STAGGER_STEP_MS);
}

export interface ChatStreamTextProps extends Omit<React.ComponentProps<"span">, "children"> {
  /** The text so far. Pass the whole growing string; only the appended tail animates. */
  text: string;
  /**
   * The text is still growing. A string that is already complete when this
   * mounts as `false` shows at once, with no entrance.
   */
  streaming?: boolean;
}

/**
 * Text that arrives word by word. New words fade in from a soft blur and settle
 * (motion.css); words already on screen are never touched again. Once a word has
 * landed it folds into one plain text node, so a long reply costs a handful of
 * elements, not thousands. The text content stays exactly `text`.
 */
export function ChatStreamText({ text, streaming = false, ...props }: ChatStreamTextProps) {
  const parsed = split(text);
  const [progress, setProgress] = React.useState<Progress>(() => ({
    text,
    settled: streaming ? 0 : parsed.units.length,
    delays: delaysFor(0, parsed.units.length),
  }));

  let cur = progress;
  if (text !== progress.text) {
    // Anything but an append (edit, regenerate) starts over rather than patching words in place.
    const appended = text.startsWith(progress.text);
    const known = appended ? split(progress.text).units.length : 0;
    cur = {
      text,
      settled: appended ? progress.settled : 0,
      delays: [...(appended ? progress.delays.slice(0, known) : []), ...delaysFor(known, parsed.units.length)],
    };
    setProgress(cur);
  }

  const settle = React.useCallback((upTo: number) => {
    setProgress((p) => (upTo > p.settled ? { ...p, settled: upTo } : p));
  }, []);

  const total = parsed.units.length;
  React.useEffect(() => {
    if (streaming) return;
    const id = setTimeout(() => settle(total), SETTLE_FALLBACK_MS);
    return () => clearTimeout(id);
  }, [streaming, total, settle]);

  const settledText = parsed.lead + parsed.units.slice(0, cur.settled).map((u) => u.word + u.gap).join("");
  return (
    <span
      data-slot="chat-stream-text"
      onAnimationEnd={(e) => {
        const i = Number((e.target as HTMLElement).dataset["word"]);
        if (Number.isInteger(i)) settle(i + 1);
      }}
      {...props}
    >
      {settledText}
      {parsed.units.slice(cur.settled).map((u, k) => {
        const i = cur.settled + k;
        return (
          <React.Fragment key={i}>
            <span data-slot="chat-word" data-word={i} style={{ animationDelay: `${cur.delays[i] ?? 0}ms` }}>
              {u.word}
            </span>
            {u.gap}
          </React.Fragment>
        );
      })}
    </span>
  );
}
