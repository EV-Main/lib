"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { ChatStreamText } from "./chat-stream-text";
import { IconButton } from "./icon-button";
import { UiIcon } from "./ui-icons";

export type ChatBubbleVariant = "user" | "assistant" | "muted";

const BUBBLE: Record<ChatBubbleVariant, string> = {
  user: "max-w-105 rounded-lg border border-border bg-secondary px-3.5 py-2.5",
  assistant: "py-0.5",
  muted: "max-w-105 rounded-lg border border-border-subtle bg-muted px-3.5 py-2.5",
};

export interface ChatBubbleProps extends React.ComponentProps<"div"> {
  variant?: ChatBubbleVariant;
  /**
   * Still receiving text: a soft caret pulses after it, and string children
   * fade in word by word (`ChatStreamText`). Stays set until the text is complete.
   */
  streaming?: boolean;
}

/** One block of message text; `assistant` has no box so long answers read as a page. */
export function ChatBubble({ variant = "assistant", streaming = false, className, children, ...props }: ChatBubbleProps) {
  // Once a bubble has streamed it keeps its word wrappers and its caret slot, so
  // the last words finish landing and the caret fades out instead of vanishing.
  const [live, setLive] = React.useState(streaming);
  if (streaming && !live) setLive(true);
  return (
    <div
      data-slot="chat-bubble"
      data-variant={variant}
      data-streaming={streaming ? "" : undefined}
      data-caret={live ? "" : undefined}
      className={cn("min-w-0 break-words whitespace-pre-wrap text-body text-ink", BUBBLE[variant], className)}
      {...props}
    >
      {live && typeof children === "string" ? <ChatStreamText text={children} streaming={streaming} /> : children}
    </div>
  );
}

export interface ChatMessageProps extends Omit<React.ComponentProps<"div">, "role"> {
  role: "assistant" | "user";
  /** "Assistant", "You". */
  name?: React.ReactNode;
  /** Already formatted: "13:04". */
  time?: React.ReactNode;
  /** The assistant's tile; a default sparkle tile when omitted. */
  avatar?: React.ReactNode;
  /** Drops the name row and the avatar, for a narrow dock. */
  compact?: boolean;
}

/** A turn: who spoke, when, then the bubbles and markers. Fades in and rises once when it mounts. */
export function ChatMessage({ role, name, time, avatar, compact = false, className, children, ...props }: ChatMessageProps) {
  const user = role === "user";
  return (
    <div
      data-slot="chat-message"
      data-role={role}
      data-enter="rise"
      className={cn("flex gap-3", user && "justify-end", className)}
      {...props}
    >
      {!user && !compact && (
        <div className="shrink-0">
          {avatar ?? (
            <span className="flex size-7 items-center justify-center rounded-(--control-radius) border border-border bg-secondary text-ink-mid">
              <UiIcon name="sparkles" />
            </span>
          )}
        </div>
      )}
      <div className={cn("flex min-w-0 flex-col gap-1.5", user ? "max-w-105 items-end" : "flex-1")}>
        {!compact && (name != null || time != null) && (
          <div className="flex items-baseline gap-2">
            {name != null && <span className="text-body-s font-medium text-ink">{name}</span>}
            {time != null && <span className="text-caption text-ink-soft">{time}</span>}
          </div>
        )}
        {children}
      </div>
    </div>
  );
}

export type ChatMarkerProps = Omit<React.ComponentProps<"div">, "title"> &
  (
    | { type: "status"; state?: "running" | "done" }
    | {
        type: "tool";
        state?: "running" | "done" | "error";
        /** The tool's name. */
        title: React.ReactNode;
        detail?: React.ReactNode;
        /** The right-hand note: "Running…", "0.4s", "Failed · 1.2s". */
        meta?: React.ReactNode;
      }
    | { type: "separator" }
  );

/** The quiet lines between messages: progress, a tool call, or a day break. */
export function ChatMarker(allProps: ChatMarkerProps) {
  if (allProps.type === "separator") {
    const { type: _type, className, children, ...rest } = allProps;
    return (
      <div data-slot="chat-marker" data-type="separator" className={cn("flex items-center gap-3", className)} {...rest}>
        <span className="h-px flex-1 bg-border" />
        <span className="text-caption text-ink-soft">{children}</span>
        <span className="h-px flex-1 bg-border" />
      </div>
    );
  }
  if (allProps.type === "status") {
    const { type: _type, state = "done", className, children, ...rest } = allProps;
    return (
      <div
        data-slot="chat-marker"
        data-type="status"
        data-state={state}
        role={state === "running" ? "status" : undefined}
        className={cn("flex items-center gap-2 text-body-s text-ink-soft", className)}
        {...rest}
      >
        <span
          aria-hidden
          className={cn("size-1.5 rounded-full", state === "running" ? "bg-primary-ink motion-safe:animate-pulse" : "bg-ink-soft")}
        />
        {children}
      </div>
    );
  }
  const { type: _type, state = "done", title, detail, meta, className, children, ...rest } = allProps;
  return (
    <div
      data-slot="chat-marker"
      data-type="tool"
      data-state={state}
      className={cn(
        "flex items-center gap-2.5 rounded-(--control-radius) border bg-card px-2.5 py-2",
        state === "error" ? "border-chip-error-border" : "border-border",
        className,
      )}
      {...rest}
    >
      <UiIcon name={state === "error" ? "alert" : "sparkles"} className="size-4 shrink-0 text-ink-mid" />
      <div className="flex min-w-0 flex-1 flex-col gap-px">
        <span className="truncate text-body-s font-medium text-ink">{title}</span>
        {detail != null && <span className="truncate text-body-s tabular-nums text-ink-soft">{detail}</span>}
        {children}
      </div>
      {meta != null && (
        <span
          className={cn(
            "shrink-0 text-caption",
            state === "running" ? "text-primary-ink" : state === "error" ? "text-accent-error-ink" : "text-ink-soft",
          )}
        >
          {meta}
        </span>
      )}
      {state === "done" && <UiIcon name="check" className="size-4 shrink-0 text-ink-soft" />}
    </div>
  );
}

export interface ChatAttachmentProps extends Omit<React.ComponentProps<"div">, "children"> {
  type?: "file" | "image";
  state?: "ready" | "uploading";
  name: string;
  /** "PDF · 1.2 MB"; while uploading, defaults to the percentage. */
  meta?: React.ReactNode;
  /** 0 to 100, while uploading. */
  progress?: number;
  /** Shows the remove button. */
  onRemove?: () => void;
  /** Default `"Remove {name}"`. */
  removeLabel?: string;
}

/** A card for a file in a message or in the composer. */
export function ChatAttachment({
  type = "file",
  state = "ready",
  name,
  meta,
  progress = 0,
  onRemove,
  removeLabel,
  className,
  ...props
}: ChatAttachmentProps) {
  const pct = Math.min(100, Math.max(0, Math.round(progress)));
  return (
    <div
      data-slot="chat-attachment"
      data-type={type}
      data-state={state}
      className={cn("flex w-60 max-w-full flex-col overflow-hidden rounded-(--control-radius) border border-border bg-card", className)}
      {...props}
    >
      <div className="flex items-center gap-2.5 p-2">
        <span className="flex size-9 shrink-0 items-center justify-center rounded-(--control-radius) border border-border-subtle bg-muted text-ink-mid">
          <UiIcon name={type === "image" ? "image" : "file"} />
        </span>
        <div className="flex min-w-0 flex-1 flex-col gap-px">
          <span className="truncate text-body-s font-medium text-ink">{name}</span>
          <span className="truncate text-caption text-ink-soft">{meta ?? (state === "uploading" ? `Uploading… ${pct}%` : null)}</span>
        </div>
        {onRemove && (
          <IconButton aria-label={removeLabel ?? `Remove ${name}`} size="sm" onClick={onRemove}>
            <UiIcon name="x" />
          </IconButton>
        )}
      </div>
      {state === "uploading" && (
        <div
          role="progressbar"
          aria-label={name}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={pct}
          className="h-0.5 bg-border-subtle"
        >
          <div className="h-full bg-primary" style={{ width: `${pct}%` }} />
        </div>
      )}
    </div>
  );
}
