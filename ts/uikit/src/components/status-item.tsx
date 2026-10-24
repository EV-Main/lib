import * as React from "react";
import { cn } from "../lib/cn";
import {
  STATUS_DOT_BASE,
  STATUS_ITEM_BASE,
  STATUS_ITEM_VALUE,
  statusDotClasses,
  type StatusTone,
} from "../generated/status-item";

export type { StatusTone };

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
      className={cn(STATUS_DOT_BASE, statusDotClasses[tone], className)}
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
      className={cn(STATUS_ITEM_BASE, className)}
      {...props}
    >
      <StatusDot tone={tone} />
      <span>{children}</span>
      {value != null && <span className={STATUS_ITEM_VALUE}>{value}</span>}
    </span>
  );
}
