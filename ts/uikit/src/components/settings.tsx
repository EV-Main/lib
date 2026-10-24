"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  SETTINGS_LAYOUT,
  SETTINGS_MAIN,
  SETTINGS_NAV,
  SETTINGS_ROW_INLINE,
  SETTINGS_SECTION_ACTION,
  SETTINGS_SECTION_BASE,
  SETTINGS_SECTION_BODY,
  SETTINGS_SECTION_BODY_FLUSH,
  SETTINGS_SECTION_BODY_PADDED,
  SETTINGS_SECTION_DESCRIPTION,
  SETTINGS_SECTION_HEADER,
  SETTINGS_SECTION_HEADER_FLUSH,
  SETTINGS_SECTION_HEADING,
  SETTINGS_SECTION_TITLE,
  settingsSectionToneClasses,
  type SettingsSectionTone,
} from "../generated/settings";
import { FormRow, type FormRowProps } from "./form-row";

export interface SettingsLayoutProps extends Omit<React.ComponentProps<"div">, "title"> {
  /** The section rail (`SectionNav`), 256 wide from `lg` up and on top below it. */
  nav: React.ReactNode;
}

/**
 * Rail on the left, cards in the main column; centres a 1408 px measure. The column is a
 * div: `AppShell` already renders the page's `<main>`.
 */
export function SettingsLayout({ nav, className, children, ...props }: SettingsLayoutProps) {
  return (
    <div
      data-slot="settings-layout"
      className={cn(
        SETTINGS_LAYOUT,
        className,
      )}
      {...props}
    >
      <div data-slot="settings-nav" className={SETTINGS_NAV}>
        {nav}
      </div>
      <div data-slot="settings-main" className={SETTINGS_MAIN}>
        {children}
      </div>
    </div>
  );
}

export interface SettingsSectionProps extends Omit<React.ComponentProps<"section">, "title"> {
  title: React.ReactNode;
  description?: React.ReactNode;
  /** Top-right control, e.g. a "Create team" button. */
  action?: React.ReactNode;
  /** `danger` draws the destructive-zone edge. */
  tone?: SettingsSectionTone;
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
        SETTINGS_SECTION_BASE,
        settingsSectionToneClasses[tone],
        className,
      )}
      {...props}
    >
      <header className={cn(SETTINGS_SECTION_HEADER, flush && SETTINGS_SECTION_HEADER_FLUSH)}>
        <div className={SETTINGS_SECTION_HEADING}>
          <h3 id={headingId} className={SETTINGS_SECTION_TITLE}>
            {title}
          </h3>
          {description != null && <p className={SETTINGS_SECTION_DESCRIPTION}>{description}</p>}
        </div>
        {action != null && <div className={SETTINGS_SECTION_ACTION}>{action}</div>}
      </header>
      {children != null && (
        <div className={cn(SETTINGS_SECTION_BODY, flush ? SETTINGS_SECTION_BODY_FLUSH : SETTINGS_SECTION_BODY_PADDED)}>
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
        inline && SETTINGS_ROW_INLINE,
        className,
      )}
      {...props}
    />
  );
}
