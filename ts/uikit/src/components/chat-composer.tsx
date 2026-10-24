"use client";

import * as React from "react";
import {
  CHAT_COMPOSER_ATTACHMENTS,
  CHAT_COMPOSER_BASE,
  CHAT_COMPOSER_FIELD,
  CHAT_COMPOSER_HINT,
  CHAT_COMPOSER_IDLE,
  CHAT_COMPOSER_SPACER,
  CHAT_COMPOSER_STREAMING,
  CHAT_COMPOSER_TOOLBAR,
  MODEL_CHIP_BASE,
} from "../generated/chat-composer";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuTrigger,
} from "./dropdown-menu";
import { IconButton } from "./icon-button";
import { Textarea } from "./textarea";
import { UiIcon } from "./ui-icons";

export interface ModelOption {
  id: string;
  label: string;
}

export interface ModelChipProps {
  models: readonly ModelOption[];
  value: string;
  onValueChange: (id: string) => void;
  /** Names the menu. Default `"Model"`. */
  "aria-label"?: string;
  className?: string;
}

/** The small bordered button naming the model; opens a radio menu of the options. */
export function ModelChip({ models, value, onValueChange, "aria-label": ariaLabel = "Model", className }: ModelChipProps) {
  const current = models.find((m) => m.id === value);
  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        aria-label={`${ariaLabel}: ${current?.label ?? value}`}
        data-slot="model-chip"
        className={cn(MODEL_CHIP_BASE, className)}
      >
        {current?.label ?? value}
        <UiIcon name="chevron-down" />
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start">
        <DropdownMenuRadioGroup value={value} onValueChange={onValueChange}>
          {models.map((m) => (
            <DropdownMenuRadioItem key={m.id} value={m.id}>
              {m.label}
            </DropdownMenuRadioItem>
          ))}
        </DropdownMenuRadioGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export interface ChatComposerLabels {
  /** Names the message field. Default `"Message"`. */
  field?: string;
  send?: string;
  stop?: string;
  attach?: string;
}

export interface ChatComposerProps extends Omit<React.ComponentProps<"div">, "onChange" | "defaultValue"> {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  /** Enter or the send button, with the trimmed text. The field clears afterwards. */
  onSend: (text: string) => void;
  /** The assistant is answering: the button becomes Stop. Enter still queues a follow-up. */
  streaming?: boolean;
  onStop?: () => void;
  placeholder?: string;
  /** Shows the attach button. */
  onAttach?: () => void;
  /** A row of `ChatAttachment`s above the field. */
  attachments?: React.ReactNode;
  /** Model chips and other toolbar controls, after the attach button. */
  tools?: React.ReactNode;
  /** A caption in the toolbar: "Enter to send · Shift+Enter for a new line". */
  hint?: React.ReactNode;
  disabled?: boolean;
  labels?: ChatComposerLabels;
}

export function ChatComposer({
  value,
  defaultValue = "",
  onValueChange,
  onSend,
  streaming = false,
  onStop,
  placeholder = "Ask the assistant…",
  onAttach,
  attachments,
  tools,
  hint,
  disabled = false,
  labels = {},
  className,
  ...props
}: ChatComposerProps) {
  const [text, setText] = useControllableState<string>({
    ...(value !== undefined ? { value } : {}),
    defaultValue,
    ...(onValueChange ? { onChange: onValueChange } : {}),
  });
  const trimmed = text.trim();
  const root = React.useRef<HTMLDivElement>(null);

  const send = () => {
    if (trimmed === "" || disabled) return;
    onSend(trimmed);
    setText("");
  };

  return (
    <div
      ref={root}
      data-slot="chat-composer"
      data-state={streaming ? "streaming" : "default"}
      className={cn(
        CHAT_COMPOSER_BASE,
        streaming ? CHAT_COMPOSER_STREAMING : CHAT_COMPOSER_IDLE,
        className,
      )}
      {...props}
    >
      {attachments != null && <div className={CHAT_COMPOSER_ATTACHMENTS}>{attachments}</div>}
      <Textarea
        rows={1}
        value={text}
        disabled={disabled}
        placeholder={placeholder}
        aria-label={labels.field ?? "Message"}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={(e) => {
          // Enter during IME composition confirms the candidate; it must not send. Safari reports the
          // confirming Enter after compositionend, with isComposing already false and keyCode 229.
          if (e.key === "Enter" && !e.shiftKey && !e.nativeEvent.isComposing && e.keyCode !== 229) {
            e.preventDefault();
            send();
          }
        }}
        className={CHAT_COMPOSER_FIELD}
      />
      <div className={CHAT_COMPOSER_TOOLBAR}>
        {onAttach && (
          <IconButton aria-label={labels.attach ?? "Attach file"} size="sm" onClick={onAttach} disabled={disabled}>
            <UiIcon name="plus" />
          </IconButton>
        )}
        {tools}
        <span className={CHAT_COMPOSER_SPACER} />
        {hint != null && <span className={CHAT_COMPOSER_HINT}>{hint}</span>}
        {streaming ? (
          <IconButton
            variant="secondary"
            aria-label={labels.stop ?? "Stop"}
            onClick={() => {
              onStop?.();
              // The Stop button turns into Send, so focus would otherwise stay on a swapped-out element.
              root.current?.querySelector("textarea")?.focus();
            }}
          >
            <UiIcon name="stop" />
          </IconButton>
        ) : (
          <IconButton
            variant="secondary"
            aria-label={labels.send ?? "Send"}
            onClick={send}
            disabled={disabled || trimmed === ""}
          >
            <UiIcon name="arrow-up" />
          </IconButton>
        )}
      </div>
    </div>
  );
}
