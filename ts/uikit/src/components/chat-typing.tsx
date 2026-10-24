import * as React from "react";
import { CHAT_TYPING_BASE, CHAT_TYPING_DOT, CHAT_TYPING_STILL } from "../generated/chat-typing";
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
      className={cn(CHAT_TYPING_BASE, className)}
      {...props}
    >
      <span aria-hidden data-dot className={CHAT_TYPING_DOT} />
      <span aria-hidden data-dot className={CHAT_TYPING_DOT} />
      <span aria-hidden data-dot className={CHAT_TYPING_DOT} />
      <span aria-hidden data-still className={CHAT_TYPING_STILL}>
        …
      </span>
    </span>
  );
}
