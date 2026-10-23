"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { AsideHeader } from "./aside";
import { IconButton } from "./icon-button";
import { UiIcon } from "./ui-icons";

// Within this many pixels of the end still counts as "at the end": sub-pixel
// scroll positions and a growing last line would otherwise unpin the reader.
const PIN_SLACK = 24;

export interface ChatPanelLabels {
  /** The scroll-to-end button. Default `"Jump to latest"`. */
  jump?: string;
  /** Names the log region. Default `"Conversation"`. */
  log?: string;
}

export interface ChatPanelProps extends Omit<React.ComponentProps<"div">, "children"> {
  /** `ChatMessage`s and `ChatMarker`s, oldest first. */
  children?: React.ReactNode;
  /** Shown instead of the log while there is nothing to read. */
  empty?: React.ReactNode;
  /** The composer, under the log. */
  composer?: React.ReactNode;
  labels?: ChatPanelLabels;
}

/**
 * The conversation: a scrolling log that follows new content while the reader
 * is at the end, and lets go the moment they scroll up. The jump button brings
 * them back. The log is a polite live region, so new turns are announced
 * without taking focus.
 */
export function ChatPanel({ children, empty, composer, labels = {}, className, ...props }: ChatPanelProps) {
  const scroller = React.useRef<HTMLDivElement>(null);
  const pinned = React.useRef(true);
  const [away, setAway] = React.useState(false);
  const isEmpty = React.Children.count(children) === 0 && empty != null;

  const toEnd = () => {
    const el = scroller.current;
    if (el) el.scrollTop = el.scrollHeight;
  };

  // Streaming grows the last node's text without adding nodes, so watch characters too.
  React.useEffect(() => {
    const el = scroller.current;
    if (!el) return;
    const follow = () => {
      if (pinned.current) toEnd();
    };
    const mo = new MutationObserver(follow);
    mo.observe(el, { childList: true, subtree: true, characterData: true });
    const ro = typeof ResizeObserver === "function" ? new ResizeObserver(follow) : null;
    ro?.observe(el);
    toEnd();
    return () => {
      mo.disconnect();
      ro?.disconnect();
    };
  }, [isEmpty]);

  return (
    <div data-slot="chat-panel" className={cn("flex min-h-0 flex-1 flex-col", className)} {...props}>
      <div className="relative flex min-h-0 flex-1 flex-col">
        {isEmpty ? (
          <div className="flex flex-1 flex-col justify-center px-4">{empty}</div>
        ) : (
          <div
            ref={scroller}
            role="log"
            aria-live="polite"
            aria-label={labels.log ?? "Conversation"}
            tabIndex={0}
            data-slot="chat-log"
            className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-4 outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
            onScroll={(e) => {
              const el = e.currentTarget;
              const atEnd = el.scrollHeight - el.scrollTop - el.clientHeight <= PIN_SLACK;
              pinned.current = atEnd;
              setAway(!atEnd);
            }}
          >
            {children}
          </div>
        )}
        {away && !isEmpty && (
          <IconButton
            variant="secondary"
            aria-label={labels.jump ?? "Jump to latest"}
            className="absolute bottom-3 left-1/2 -translate-x-1/2"
            onClick={() => {
              pinned.current = true;
              setAway(false);
              toEnd();
            }}
          >
            <UiIcon name="chevron-down" />
          </IconButton>
        )}
      </div>
      {composer != null && <div className="shrink-0 px-4 pb-4">{composer}</div>}
    </div>
  );
}

export interface ChatDockLabels extends ChatPanelLabels {
  /** The launcher bar's name when collapsed. Default `"Open assistant"`. */
  open?: string;
  /** Default `"Collapse"`. */
  collapse?: string;
}

export interface ChatDockProps extends Omit<React.ComponentProps<"section">, "title" | "children"> {
  title: React.ReactNode;
  expanded?: boolean;
  defaultExpanded?: boolean;
  onExpandedChange?: (expanded: boolean) => void;
  /** Header buttons before collapse: history. */
  actions?: React.ReactNode;
  /** The launcher bar's text when collapsed. */
  prompt?: string;
  children?: React.ReactNode;
  empty?: React.ReactNode;
  composer?: React.ReactNode;
  labels?: ChatDockLabels;
}

/** The assistant docked in a column: a launcher bar when collapsed, header + conversation + composer when open. */
export function ChatDock({
  title,
  expanded,
  defaultExpanded = true,
  onExpandedChange,
  actions,
  prompt = "Ask the assistant…",
  children,
  empty,
  composer,
  labels = {},
  className,
  ...props
}: ChatDockProps) {
  const [inner, setInner] = React.useState(defaultExpanded);
  const open = expanded ?? inner;
  const set = (next: boolean) => {
    setInner(next);
    onExpandedChange?.(next);
  };
  if (!open) {
    return (
      <section data-slot="chat-dock" data-state="collapsed" className={cn("bg-card", className)} {...props}>
        <button
          type="button"
          aria-label={labels.open ?? "Open assistant"}
          onClick={() => set(true)}
          className="flex h-11 w-full items-center gap-2 border-t border-border px-4 text-left text-body text-ink-soft outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-ring"
        >
          <UiIcon name="sparkles" />
          {prompt}
        </button>
      </section>
    );
  }
  return (
    <section data-slot="chat-dock" data-state="expanded" className={cn("flex h-full min-h-0 flex-col bg-card", className)} {...props}>
      <AsideHeader title={title} actions={actions} onClose={() => set(false)} labels={{ close: labels.collapse ?? "Collapse" }} />
      <ChatPanel empty={empty} composer={composer} labels={labels}>
        {children}
      </ChatPanel>
    </section>
  );
}
