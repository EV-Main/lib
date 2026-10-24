"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  CHAT_ATTACHMENT_BASE,
  CHAT_ATTACHMENT_BODY,
  CHAT_ATTACHMENT_ICON,
  CHAT_ATTACHMENT_META,
  CHAT_ATTACHMENT_NAME,
  CHAT_ATTACHMENT_PROGRESS,
  CHAT_ATTACHMENT_PROGRESS_FILL,
  CHAT_ATTACHMENT_ROW,
  CHAT_BUBBLE_BASE,
  CHAT_MARKER_DOT,
  CHAT_MARKER_RULE,
  CHAT_MARKER_SEPARATOR,
  CHAT_MARKER_SEPARATOR_LABEL,
  CHAT_MARKER_STATUS,
  CHAT_MARKER_TOOL,
  CHAT_MARKER_TOOL_BODY,
  CHAT_MARKER_TOOL_BORDER,
  CHAT_MARKER_TOOL_CHECK,
  CHAT_MARKER_TOOL_DETAIL,
  CHAT_MARKER_TOOL_ERROR_BORDER,
  CHAT_MARKER_TOOL_ICON,
  CHAT_MARKER_TOOL_META,
  CHAT_MARKER_TOOL_TITLE,
  CHAT_MESSAGE_AVATAR,
  CHAT_MESSAGE_AVATAR_TILE,
  CHAT_MESSAGE_BASE,
  CHAT_MESSAGE_BODY,
  CHAT_MESSAGE_BODY_ASSISTANT,
  CHAT_MESSAGE_BODY_USER,
  CHAT_MESSAGE_HEADER,
  CHAT_MESSAGE_NAME,
  CHAT_MESSAGE_TIME,
  CHAT_MESSAGE_USER,
  chatBubbleVariantClasses,
  chatStatusDotClasses,
  chatToolMetaClasses,
} from "../generated/chat-message";
import type { ChatBubbleVariant, ChatStatusState, ChatToolState } from "../generated/chat-message";
import { ChatStreamText } from "./chat-stream-text";
import { IconButton } from "./icon-button";
import { UiIcon } from "./ui-icons";

export type { ChatBubbleVariant };

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
      className={cn(CHAT_BUBBLE_BASE, chatBubbleVariantClasses[variant], className)}
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
      className={cn(CHAT_MESSAGE_BASE, user && CHAT_MESSAGE_USER, className)}
      {...props}
    >
      {!user && !compact && (
        <div className={CHAT_MESSAGE_AVATAR}>
          {avatar ?? (
            <span className={CHAT_MESSAGE_AVATAR_TILE}>
              <UiIcon name="sparkles" />
            </span>
          )}
        </div>
      )}
      <div className={cn(CHAT_MESSAGE_BODY, user ? CHAT_MESSAGE_BODY_USER : CHAT_MESSAGE_BODY_ASSISTANT)}>
        {!compact && (name != null || time != null) && (
          <div className={CHAT_MESSAGE_HEADER}>
            {name != null && <span className={CHAT_MESSAGE_NAME}>{name}</span>}
            {time != null && <span className={CHAT_MESSAGE_TIME}>{time}</span>}
          </div>
        )}
        {children}
      </div>
    </div>
  );
}

export type ChatMarkerProps = Omit<React.ComponentProps<"div">, "title"> &
  (
    | { type: "status"; state?: ChatStatusState }
    | {
        type: "tool";
        state?: ChatToolState;
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
      <div data-slot="chat-marker" data-type="separator" className={cn(CHAT_MARKER_SEPARATOR, className)} {...rest}>
        <span className={CHAT_MARKER_RULE} />
        <span className={CHAT_MARKER_SEPARATOR_LABEL}>{children}</span>
        <span className={CHAT_MARKER_RULE} />
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
        className={cn(CHAT_MARKER_STATUS, className)}
        {...rest}
      >
        <span
          aria-hidden
          className={cn(CHAT_MARKER_DOT, chatStatusDotClasses[state])}
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
        CHAT_MARKER_TOOL,
        state === "error" ? CHAT_MARKER_TOOL_ERROR_BORDER : CHAT_MARKER_TOOL_BORDER,
        className,
      )}
      {...rest}
    >
      <UiIcon name={state === "error" ? "alert" : "sparkles"} className={CHAT_MARKER_TOOL_ICON} />
      <div className={CHAT_MARKER_TOOL_BODY}>
        <span className={CHAT_MARKER_TOOL_TITLE}>{title}</span>
        {detail != null && <span className={CHAT_MARKER_TOOL_DETAIL}>{detail}</span>}
        {children}
      </div>
      {meta != null && (
        <span className={cn(CHAT_MARKER_TOOL_META, chatToolMetaClasses[state])}>{meta}
        </span>
      )}
      {state === "done" && <UiIcon name="check" className={CHAT_MARKER_TOOL_CHECK} />}
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
      className={cn(CHAT_ATTACHMENT_BASE, className)}
      {...props}
    >
      <div className={CHAT_ATTACHMENT_ROW}>
        <span className={CHAT_ATTACHMENT_ICON}>
          <UiIcon name={type === "image" ? "image" : "file"} />
        </span>
        <div className={CHAT_ATTACHMENT_BODY}>
          <span className={CHAT_ATTACHMENT_NAME}>{name}</span>
          <span className={CHAT_ATTACHMENT_META}>{meta ?? (state === "uploading" ? `Uploading… ${pct}%` : null)}</span>
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
          className={CHAT_ATTACHMENT_PROGRESS}
        >
          <div className={CHAT_ATTACHMENT_PROGRESS_FILL} style={{ width: `${pct}%` }} />
        </div>
      )}
    </div>
  );
}
