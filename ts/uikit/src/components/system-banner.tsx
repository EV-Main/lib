"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { CloseIcon, InfoIcon, TriangleAlertIcon } from "./shell-icons";

export type SystemBannerTone = "neutral" | "primary" | "info" | "warn";

export interface SystemBannerLabels {
  /** The close button. Default `"Dismiss"`. */
  dismiss?: string;
}

export interface SystemBannerProps extends Omit<React.ComponentProps<"div">, "title"> {
  /** `warn` — a platform state (maintenance, read-only); `info` — an announcement. Only the icon carries the tone. */
  tone?: SystemBannerTone;
  /** Bold lead before the message. */
  title?: React.ReactNode;
  /** Replaces the tone's glyph; `null` drops it. */
  icon?: React.ReactNode;
  /** Shows a close button; remembering the dismissal is the caller's. */
  onDismiss?: () => void;
  /** A link or button after the message. */
  action?: React.ReactNode;
  labels?: SystemBannerLabels;
}

const ICON_TONE: Record<SystemBannerTone, string> = {
  neutral: "text-ink-soft",
  primary: "text-primary-ink",
  info: "text-accent-info-ink",
  warn: "text-accent-warn-ink",
};

/**
 * A slim full-width strip above the page — presentation only: where its state
 * comes from, and whether a dismissal sticks, are the caller's. Put it in
 * `AppShell`'s `banner` slot; several stack with a gap.
 */
export function SystemBanner({
  tone = "info",
  title,
  icon,
  onDismiss,
  action,
  labels = {},
  className,
  children,
  ...props
}: SystemBannerProps) {
  const glyph =
    icon !== undefined ? icon : tone === "warn" ? <TriangleAlertIcon className="size-4" /> : <InfoIcon className="size-4" />;
  return (
    <div
      data-slot="system-banner"
      data-tone={tone}
      role="status"
      className={cn(
        "relative flex min-h-8 items-center justify-center gap-2 border-b border-border bg-card py-1.5 pl-4 pr-10 text-center text-body text-ink",
        className,
      )}
      {...props}
    >
      {glyph !== null && <span className={cn("flex shrink-0", ICON_TONE[tone])}>{glyph}</span>}
      <div className="min-w-0">
        {title !== undefined && <span className="font-medium">{title}</span>}
        {title !== undefined && children !== undefined && " — "}
        {children !== undefined && <span className={cn(title !== undefined && "text-ink-mid")}>{children}</span>}
      </div>
      {action !== undefined && <span className="shrink-0">{action}</span>}
      {onDismiss && (
        <button
          type="button"
          aria-label={labels.dismiss ?? "Dismiss"}
          onClick={onDismiss}
          className="absolute right-3 top-1/2 flex size-6 -translate-y-1/2 items-center justify-center rounded-(--control-radius) text-ink-mid outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring"
        >
          <CloseIcon className="size-4" />
        </button>
      )}
    </div>
  );
}
