"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { AnimatedNumber } from "./animated-number";
import { IconButton } from "./icon-button";
import { Kbd } from "./kbd";
import { Popover, PopoverContent, PopoverTrigger } from "./popover";
import { Settled } from "./settled";
import { Skeleton } from "./skeleton";
import { UiIcon } from "./ui-icons";

export interface HeaderBalanceProps extends Omit<React.HTMLAttributes<HTMLElement>, "children"> {
  /** Only meaningful with `onClick`. */
  disabled?: boolean;
  label?: React.ReactNode;
  /** The figure, already formatted: "$0.00". Ignored when `amount` is given. */
  value?: React.ReactNode;
  /** The figure as a number: it rolls to each new amount, formatted by `format` and `locales`. */
  amount?: number;
  /** Formatting for `amount`. Default: USD currency. */
  format?: Intl.NumberFormatOptions;
  locales?: Intl.LocalesArgument;
  /** True until the balance is known; the figure then reveals in place of the skeleton. */
  loading?: boolean;
  /** A glyph before the label. */
  icon?: React.ReactNode;
  /** `warn` colours the figure as a low-funds signal. */
  tone?: "default" | "warn";
}

const USD: Intl.NumberFormatOptions = { style: "currency", currency: "USD" };

/** "Balance: $0.00" in the header; a button only when given `onClick`. */
export function HeaderBalance({
  label = "Balance:",
  value,
  amount,
  format = USD,
  locales,
  loading = false,
  icon,
  tone = "default",
  onClick,
  disabled,
  className,
  ...props
}: HeaderBalanceProps) {
  const face = cn(
    "inline-flex h-8 items-center gap-2 rounded-(--control-radius) border border-border bg-secondary pl-2.5 pr-2 text-body-s",
    onClick && "cursor-pointer outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring",
    className,
  );
  const body = (
    <>
      {icon}
      <span className="text-ink-soft">{label}</span>
      <Settled
        loading={loading}
        skeleton={<Skeleton className="h-3 w-12" />}
        className="inline-flex"
      >
        <span className={cn("font-medium tabular-nums", tone === "warn" ? "text-accent-warn-ink" : "text-ink")}>
          {amount === undefined ? value : <AnimatedNumber value={amount} format={format} locales={locales} />}
        </span>
      </Settled>
      {onClick && <UiIcon name="chevron-down" className="size-4 text-ink-soft" />}
    </>
  );
  if (!onClick) {
    return (
      <div data-slot="header-balance" className={face} {...props}>
        {body}
      </div>
    );
  }
  return (
    <button type="button" data-slot="header-balance" className={face} onClick={onClick} disabled={disabled} {...props}>
      {body}
    </button>
  );
}

export interface HeaderNotification {
  id: string;
  title: React.ReactNode;
  description?: React.ReactNode;
  /** Already formatted: "2m ago". */
  time?: React.ReactNode;
  unread?: boolean;
}

export interface NotificationBellLabels {
  /** The bell's name. Default `"Notifications"`. */
  trigger?: string;
  /** Spoken after the name when something is unread; gets the count. Default `"{n} unread"`. */
  unread?: (count: number) => string;
  /** Default `"No notifications"`. */
  empty?: string;
}

export interface NotificationBellProps {
  notifications: readonly HeaderNotification[];
  onSelect?: (id: string) => void;
  labels?: NotificationBellLabels;
  /** Under the list: "View all". */
  footer?: React.ReactNode;
  className?: string;
}

/** The bell with an unread dot; the list opens in a popover. */
export function NotificationBell({ notifications, onSelect, labels = {}, footer, className }: NotificationBellProps) {
  const unread = notifications.filter((n) => n.unread).length;
  const name = labels.trigger ?? "Notifications";
  const spoken = unread > 0 ? `${name}, ${(labels.unread ?? ((n) => `${n} unread`))(unread)}` : name;
  return (
    <Popover>
      <PopoverTrigger asChild>
        <IconButton aria-label={spoken} variant="secondary" className={cn("relative", className)}>
          <UiIcon name="bell" />
          {unread > 0 && (
            <span
              data-slot="notification-dot"
              aria-hidden
              className="absolute right-1 top-1 size-1.5 rounded-full bg-accent-error"
            />
          )}
        </IconButton>
      </PopoverTrigger>
      <PopoverContent align="end" className="w-80 p-1">
        {notifications.length === 0 ? (
          <p className="px-2 py-3 text-center text-body text-ink-soft">{labels.empty ?? "No notifications"}</p>
        ) : (
          <ul className="flex max-h-80 flex-col gap-0.5 overflow-y-auto">
            {notifications.map((n) => (
              <li key={n.id}>
                <button
                  type="button"
                  onClick={() => onSelect?.(n.id)}
                  className="flex w-full items-start gap-2 rounded-(--control-radius) px-2 py-1.5 text-left outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring"
                >
                  <span
                    aria-hidden
                    className={cn("mt-1.5 size-1.5 shrink-0 rounded-full", n.unread ? "bg-primary" : "bg-transparent")}
                  />
                  <span className="flex min-w-0 flex-1 flex-col gap-0.5">
                    <span className={cn("text-body text-ink", n.unread && "font-medium")}>{n.title}</span>
                    {n.description != null && <span className="text-body-s text-ink-mid">{n.description}</span>}
                  </span>
                  {n.time != null && <span className="shrink-0 text-caption text-ink-soft">{n.time}</span>}
                </button>
              </li>
            ))}
          </ul>
        )}
        {footer != null && <div className="border-t border-border-subtle p-1 pt-1.5">{footer}</div>}
      </PopoverContent>
    </Popover>
  );
}

export interface HeaderSearchProps extends React.ComponentProps<"button"> {
  placeholder?: string;
  /** The shortcut hint at the end. Default: `⌘K` on Apple platforms, `Ctrl K` elsewhere. */
  shortcut?: string;
}

const APPLE = /Mac|iPhone|iPad|iPod/;

// Read after mount, so the server render and the first client render agree.
function usePlatformShortcut(): string {
  const [shortcut, setShortcut] = React.useState("Ctrl K");
  React.useEffect(() => {
    const nav = navigator as Navigator & { userAgentData?: { platform?: string } };
    if (APPLE.test(nav.userAgentData?.platform ?? nav.platform ?? "")) setShortcut("⌘K");
  }, []);
  return shortcut;
}

/** A button dressed as the search field; it opens the palette rather than taking input. */
export function HeaderSearch({
  placeholder = "Search or jump to…",
  shortcut,
  className,
  ...props
}: HeaderSearchProps) {
  const platformShortcut = usePlatformShortcut();
  return (
    <button
      type="button"
      data-slot="header-search"
      className={cn(
        "flex h-8 w-55 items-center gap-2 rounded-(--control-radius) border border-border bg-muted pl-2.5 pr-1.5 text-left text-body text-ink-mid outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring",
        className,
      )}
      {...props}
    >
      <UiIcon name="search" />
      <span className="min-w-0 flex-1 truncate">{placeholder}</span>
      <Kbd>{shortcut ?? platformShortcut}</Kbd>
    </button>
  );
}
