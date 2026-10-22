import * as React from "react";
import { cn } from "../lib/cn";

export type StatusTone = "positive" | "neutral" | "warn" | "error" | "muted";

const DOT: Record<StatusTone, string> = {
  positive: "bg-positive",
  neutral: "bg-ink-soft",
  warn: "bg-accent-warn",
  error: "bg-accent-error",
  muted: "bg-ink-faint",
};

export interface StatusDotProps extends React.ComponentProps<"span"> {
  tone?: StatusTone;
}

/** The 6 px dot alone; decorative, so the text next to it must carry the meaning. */
export function StatusDot({ tone = "neutral", className, ...props }: StatusDotProps) {
  return (
    <span
      data-slot="status-dot"
      data-tone={tone}
      aria-hidden
      className={cn("size-1.5 shrink-0 rounded-full", DOT[tone], className)}
      {...props}
    />
  );
}

export interface StatusItemProps extends React.ComponentProps<"span"> {
  tone?: StatusTone;
  /** A figure after the label, e.g. a latency. */
  value?: React.ReactNode;
}

/** A dot and a short label: connection, sync, environment. */
export function StatusItem({ tone = "neutral", value, className, children, ...props }: StatusItemProps) {
  return (
    <span
      data-slot="status-item"
      data-tone={tone}
      className={cn("inline-flex h-5 items-center gap-1.5 px-2 text-caption text-ink-mid", className)}
      {...props}
    >
      <StatusDot tone={tone} />
      <span>{children}</span>
      {value != null && <span className="tabular-nums text-ink">{value}</span>}
    </span>
  );
}
