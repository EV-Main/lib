"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  SYSTEM_BANNER_ACTION,
  SYSTEM_BANNER_BASE,
  SYSTEM_BANNER_BODY_AFTER_TITLE,
  SYSTEM_BANNER_DISMISS,
  SYSTEM_BANNER_GLYPH,
  SYSTEM_BANNER_ICON,
  SYSTEM_BANNER_MESSAGE,
  SYSTEM_BANNER_TITLE,
  systemBannerIconToneClasses,
  type SystemBannerTone,
} from "../generated/system-banner";
import { CloseIcon, InfoIcon, TriangleAlertIcon } from "./shell-icons";

export type { SystemBannerTone };

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
    icon !== undefined ? icon : tone === "warn" ? <TriangleAlertIcon className={SYSTEM_BANNER_GLYPH} /> : <InfoIcon className={SYSTEM_BANNER_GLYPH} />;
  return (
    <div
      data-slot="system-banner"
      data-tone={tone}
      role="status"
      className={cn(SYSTEM_BANNER_BASE, className)}
      {...props}
    >
      {glyph !== null && <span className={cn(SYSTEM_BANNER_ICON, systemBannerIconToneClasses[tone])}>{glyph}</span>}
      <div className={SYSTEM_BANNER_MESSAGE}>
        {title !== undefined && <span className={SYSTEM_BANNER_TITLE}>{title}</span>}
        {title !== undefined && children !== undefined && " — "}
        {children !== undefined && <span className={cn(title !== undefined && SYSTEM_BANNER_BODY_AFTER_TITLE)}>{children}</span>}
      </div>
      {action !== undefined && <span className={SYSTEM_BANNER_ACTION}>{action}</span>}
      {onDismiss && (
        <button
          type="button"
          aria-label={labels.dismiss ?? "Dismiss"}
          onClick={onDismiss}
          className={SYSTEM_BANNER_DISMISS}
        >
          <CloseIcon className={SYSTEM_BANNER_GLYPH} />
        </button>
      )}
    </div>
  );
}
