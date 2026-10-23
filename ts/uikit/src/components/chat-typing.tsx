import * as React from "react";
import { cn } from "../lib/cn";

export interface ChatTypingProps extends Omit<React.ComponentProps<"span">, "children"> {
  /** Names the state for assistive tech. Default `"Assistant is thinking"`. */
  label?: string;
}

/**
 * The wait before the first token: three dots whose opacity ripples. Under
 * reduced motion CSS swaps them for a still "…". It is a status region, so a
 * screen reader hears the label once when it appears.
 */
export function ChatTyping({ label = "Assistant is thinking", className, ...props }: ChatTypingProps) {
  return (
    <span
      role="status"
      aria-label={label}
      data-slot="chat-typing"
      className={cn("inline-flex h-6 items-center gap-1 text-ink-soft", className)}
      {...props}
    >
      <span aria-hidden data-dot className="size-1.5 rounded-full bg-current" />
      <span aria-hidden data-dot className="size-1.5 rounded-full bg-current" />
      <span aria-hidden data-dot className="size-1.5 rounded-full bg-current" />
      <span aria-hidden data-still className="text-body leading-none">
        …
      </span>
    </span>
  );
}
