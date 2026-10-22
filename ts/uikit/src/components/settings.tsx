"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { FormRow, type FormRowProps } from "./form-row";

export interface SettingsLayoutProps extends Omit<React.ComponentProps<"div">, "title"> {
  /** The section rail (`SectionNav`), 256 wide from `lg` up and on top below it. */
  nav: React.ReactNode;
}

/** Rail on the left, cards in the main column; centres a 1408 px measure. */
export function SettingsLayout({ nav, className, children, ...props }: SettingsLayoutProps) {
  return (
    <div
      data-slot="settings-layout"
      className={cn(
        "mx-auto flex w-full max-w-[1408px] flex-col gap-6 px-4 py-8 lg:flex-row lg:gap-8",
        className,
      )}
      {...props}
    >
      <div data-slot="settings-nav" className="shrink-0 lg:w-64">
        {nav}
      </div>
      <main data-slot="settings-main" className="flex min-w-0 flex-1 flex-col gap-6">
        {children}
      </main>
    </div>
  );
}

export interface SettingsSectionProps extends Omit<React.ComponentProps<"section">, "title"> {
  title: React.ReactNode;
  description?: React.ReactNode;
  /** Top-right control, e.g. a "Create team" button. */
  action?: React.ReactNode;
  /** `danger` draws the destructive-zone edge. */
  tone?: "default" | "danger";
  /** Rows run edge to edge with their hairlines; otherwise the body is padded. */
  flush?: boolean;
}

/** A settings card: title, a line saying what it is for, then the body. */
export function SettingsSection({
  title,
  description,
  action,
  tone = "default",
  flush = false,
  className,
  children,
  ...props
}: SettingsSectionProps) {
  const headingId = React.useId();
  return (
    <section
      data-slot="settings-section"
      data-tone={tone}
      aria-labelledby={headingId}
      className={cn(
        "flex flex-col rounded-lg border bg-card",
        tone === "danger" ? "border-accent-error" : "border-border",
        className,
      )}
      {...props}
    >
      <header className={cn("flex items-start justify-between gap-4 p-4", flush && "pb-3")}>
        <div className="flex min-w-0 flex-col gap-0.5">
          <h3 id={headingId} className="text-h3 font-medium text-ink">
            {title}
          </h3>
          {description != null && <p className="text-body text-ink-mid">{description}</p>}
        </div>
        {action != null && <div className="shrink-0">{action}</div>}
      </header>
      {children != null && (
        <div className={cn("flex flex-col", flush ? "border-t border-border-subtle" : "gap-4 px-4 pb-4")}>
          {children}
        </div>
      )}
    </section>
  );
}

export interface SettingsRowProps extends FormRowProps {
  /** Keeps label and control on one line at every width, control at the end: for a switch or a button. */
  inline?: boolean;
}

/** A `FormRow` for settings; `inline` suits toggles. */
export function SettingsRow({ inline = false, className, ...props }: SettingsRowProps) {
  return (
    <FormRow
      data-inline={inline}
      className={cn(
        inline && "flex-row items-center justify-between sm:gap-6 [&>[data-slot=form-row-control]]:flex-none [&>[data-slot=form-row-control]]:items-end",
        className,
      )}
      {...props}
    />
  );
}
