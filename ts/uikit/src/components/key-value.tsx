"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { UiIcon } from "./ui-icons";

export interface KeyValueLabels {
  /** The copy button. Default `"Copy"`. */
  copy?: string;
  /** The polite announcement after a copy. Default `"Copied"`. */
  copied?: string;
}

export interface KeyValueProps extends Omit<React.ComponentProps<"div">, "children"> {
  label: React.ReactNode;
  value: React.ReactNode;
  /** Adds a copy button for this text. */
  copyText?: string;
  labels?: KeyValueLabels;
}

const COPIED_MS = 2000;

// The async clipboard API needs a secure context and a user gesture; an
// insecure origin or an old webview falls back to a selection + execCommand.
async function writeClipboard(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text);
      return true;
    }
  } catch {
    // Denied permission: try the selection route below.
  }
  const area = document.createElement("textarea");
  area.value = text;
  area.setAttribute("readonly", "");
  area.style.position = "fixed";
  area.style.opacity = "0";
  document.body.append(area);
  area.select();
  try {
    return document.execCommand("copy");
  } finally {
    area.remove();
  }
}

/** A small label over its value, with an optional copy button. */
export function KeyValue({ label, value, copyText, labels = {}, className, ...props }: KeyValueProps) {
  const [copied, setCopied] = React.useState(false);
  const timer = React.useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  React.useEffect(() => () => clearTimeout(timer.current), []);

  const copy = async () => {
    if (copyText === undefined || !(await writeClipboard(copyText))) return;
    setCopied(true);
    clearTimeout(timer.current);
    timer.current = setTimeout(() => setCopied(false), COPIED_MS);
  };

  return (
    <div data-slot="key-value" className={cn("flex min-w-0 flex-col gap-0.5", className)} {...props}>
      <span className="text-body-s text-ink-mid">{label}</span>
      <div className="flex min-w-0 items-center gap-1.5">
        <span className="min-w-0 truncate text-body-l text-ink">{value}</span>
        {copyText !== undefined && (
          <button
            type="button"
            aria-label={labels.copy ?? "Copy"}
            onClick={copy}
            className="inline-flex size-5 shrink-0 items-center justify-center rounded-(--control-radius) text-ink-soft outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring"
          >
            <UiIcon name={copied ? "check" : "copy"} />
          </button>
        )}
      </div>
      <span role="status" aria-live="polite" className="sr-only">
        {copied ? (labels.copied ?? "Copied") : ""}
      </span>
    </div>
  );
}
