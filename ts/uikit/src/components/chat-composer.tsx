"use client";

import * as React from "react";
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
        className={cn(
          "inline-flex h-6.5 items-center gap-1 rounded-(--control-radius) border border-border py-1 pl-2 pr-1.5 text-body-s text-ink-mid outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring",
          className,
        )}
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

  const send = () => {
    if (trimmed === "" || disabled) return;
    onSend(trimmed);
    setText("");
  };

  return (
    <div
      data-slot="chat-composer"
      data-state={streaming ? "streaming" : "default"}
      className={cn(
        "flex flex-col gap-2 rounded-lg border bg-card px-3 pb-2.5 pt-3 focus-within:ring-1 focus-within:ring-ring",
        streaming ? "border-border" : "border-border-strong",
        className,
      )}
      {...props}
    >
      {attachments != null && <div className="flex flex-wrap gap-2">{attachments}</div>}
      <Textarea
        rows={1}
        value={text}
        disabled={disabled}
        placeholder={placeholder}
        onChange={(e) => setText(e.target.value)}
        onKeyDown={(e) => {
          // Enter during IME composition confirms the candidate; it must not send.
          if (e.key === "Enter" && !e.shiftKey && !e.nativeEvent.isComposing) {
            e.preventDefault();
            send();
          }
        }}
        className="max-h-40 min-h-6 resize-none border-0 bg-transparent p-0 focus-visible:ring-0"
      />
      <div className="flex items-center gap-1.5">
        {onAttach && (
          <IconButton aria-label={labels.attach ?? "Attach file"} size="sm" onClick={onAttach} disabled={disabled}>
            <UiIcon name="plus" />
          </IconButton>
        )}
        {tools}
        <span className="flex-1" />
        {hint != null && <span className="hidden text-caption text-ink-soft sm:inline">{hint}</span>}
        {streaming ? (
          <IconButton variant="secondary" aria-label={labels.stop ?? "Stop"} onClick={onStop}>
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
