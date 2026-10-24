"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  HEADER_BALANCE_BASE,
  HEADER_BALANCE_CHEVRON,
  HEADER_BALANCE_FIGURE,
  HEADER_BALANCE_INTERACTIVE,
  HEADER_BALANCE_LABEL,
  HEADER_BALANCE_SETTLED,
  HEADER_BALANCE_SKELETON,
  HEADER_SEARCH_BASE,
  HEADER_SEARCH_PLACEHOLDER,
  NOTIFICATION_BELL_CONTENT,
  NOTIFICATION_BELL_EMPTY,
  NOTIFICATION_BELL_FOOTER,
  NOTIFICATION_BELL_ITEM,
  NOTIFICATION_BELL_ITEM_BODY,
  NOTIFICATION_BELL_ITEM_DESCRIPTION,
  NOTIFICATION_BELL_ITEM_DOT,
  NOTIFICATION_BELL_ITEM_DOT_READ,
  NOTIFICATION_BELL_ITEM_DOT_UNREAD,
  NOTIFICATION_BELL_ITEM_TIME,
  NOTIFICATION_BELL_ITEM_TITLE,
  NOTIFICATION_BELL_ITEM_TITLE_UNREAD,
  NOTIFICATION_BELL_LIST,
  NOTIFICATION_BELL_TRIGGER,
  NOTIFICATION_DOT,
  headerBalanceToneClasses,
  type HeaderBalanceTone,
} from "../generated/header-controls";
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
  tone?: HeaderBalanceTone;
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
  const face = cn(HEADER_BALANCE_BASE, onClick && HEADER_BALANCE_INTERACTIVE, className);
  const body = (
    <>
      {icon}
      <span className={HEADER_BALANCE_LABEL}>{label}</span>
      <Settled
        loading={loading}
        skeleton={<Skeleton className={HEADER_BALANCE_SKELETON} />}
        className={HEADER_BALANCE_SETTLED}
      >
        <span className={cn(HEADER_BALANCE_FIGURE, headerBalanceToneClasses[tone])}>
          {amount === undefined ? value : <AnimatedNumber value={amount} format={format} locales={locales} />}
        </span>
      </Settled>
      {onClick && <UiIcon name="chevron-down" className={HEADER_BALANCE_CHEVRON} />}
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
        <IconButton aria-label={spoken} variant="secondary" className={cn(NOTIFICATION_BELL_TRIGGER, className)}>
          <UiIcon name="bell" />
          {unread > 0 && (
            <span data-slot="notification-dot" aria-hidden className={NOTIFICATION_DOT} />
          )}
        </IconButton>
      </PopoverTrigger>
      <PopoverContent align="end" className={NOTIFICATION_BELL_CONTENT}>
        {notifications.length === 0 ? (
          <p className={NOTIFICATION_BELL_EMPTY}>{labels.empty ?? "No notifications"}</p>
        ) : (
          <ul className={NOTIFICATION_BELL_LIST}>
            {notifications.map((n) => (
              <li key={n.id}>
                <button
                  type="button"
                  onClick={() => onSelect?.(n.id)}
                  className={NOTIFICATION_BELL_ITEM}
                >
                  <span
                    aria-hidden
                    className={cn(
                      NOTIFICATION_BELL_ITEM_DOT,
                      n.unread ? NOTIFICATION_BELL_ITEM_DOT_UNREAD : NOTIFICATION_BELL_ITEM_DOT_READ,
                    )}
                  />
                  <span className={NOTIFICATION_BELL_ITEM_BODY}>
                    <span className={cn(NOTIFICATION_BELL_ITEM_TITLE, n.unread && NOTIFICATION_BELL_ITEM_TITLE_UNREAD)}>{n.title}</span>
                    {n.description != null && <span className={NOTIFICATION_BELL_ITEM_DESCRIPTION}>{n.description}</span>}
                  </span>
                  {n.time != null && <span className={NOTIFICATION_BELL_ITEM_TIME}>{n.time}</span>}
                </button>
              </li>
            ))}
          </ul>
        )}
        {footer != null && <div className={NOTIFICATION_BELL_FOOTER}>{footer}</div>}
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
      className={cn(HEADER_SEARCH_BASE, className)}
      {...props}
    >
      <UiIcon name="search" />
      <span className={HEADER_SEARCH_PLACEHOLDER}>{placeholder}</span>
      <Kbd>{shortcut ?? platformShortcut}</Kbd>
    </button>
  );
}
