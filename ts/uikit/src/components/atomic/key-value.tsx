"use client";

import * as React from "react";
import { cn } from "../../lib/cn";
import {
  KEY_VALUE_BASE,
  KEY_VALUE_COPY,
  KEY_VALUE_LABEL,
  KEY_VALUE_ROW,
  KEY_VALUE_STATUS,
  KEY_VALUE_VALUE,
  KEY_VALUE_LABELS,
  KEY_VALUE_COPIED_MS,
} from "../../generated/key-value";
import { UiIcon } from "../../icons/ui-icons";

export interface KeyValueLabels {
  /** The copy button. Default `"Copy {label}"` (just `"Copy"` when the label is not text). */
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
  // Inside a modal the focus scope pulls focus back to the dialog, which would drop a selection
  // made in <body>; the helper has to live inside the dialog the user is in.
  const host = document.activeElement?.closest('[role="dialog"]') ?? document.body;
  const area = document.createElement("textarea");
  area.value = text;
  area.setAttribute("readonly", "");
  area.style.position = "fixed";
  area.style.opacity = "0";
  host.append(area);
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
    timer.current = setTimeout(() => setCopied(false), KEY_VALUE_COPIED_MS);
  };

  return (
    <div data-slot="key-value" className={cn(KEY_VALUE_BASE, className)} {...props}>
      <span className={KEY_VALUE_LABEL}>{label}</span>
      <div className={KEY_VALUE_ROW}>
        <span className={KEY_VALUE_VALUE}>{value}</span>
        {copyText !== undefined && (
          <button
            type="button"
            aria-label={labels.copy ?? (typeof label === "string" ? `Copy ${label}` : "Copy")}
            onClick={copy}
            className={KEY_VALUE_COPY}
          >
            <UiIcon name={copied ? "check" : "copy"} />
          </button>
        )}
      </div>
      <span role="status" aria-live="polite" className={KEY_VALUE_STATUS}>
        {copied ? (labels.copied ?? KEY_VALUE_LABELS.copied) : ""}
      </span>
    </div>
  );
}
