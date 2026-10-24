import * as React from "react";
import {
  ACCOUNT_SETTINGS_DIALOG_CLOSE,
  ACCOUNT_SETTINGS_DIALOG_CONTENT,
  ACCOUNT_SETTINGS_DIALOG_DESCRIPTION,
  ACCOUNT_SETTINGS_DIALOG_ENTRY,
  ACCOUNT_SETTINGS_DIALOG_FOOTER,
  ACCOUNT_SETTINGS_DIALOG_HEADING,
  ACCOUNT_SETTINGS_DIALOG_LIST,
  ACCOUNT_SETTINGS_DIALOG_PANE,
  ACCOUNT_SETTINGS_DIALOG_PANEL,
  ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
  ACCOUNT_SETTINGS_DIALOG_TABS,
  ACCOUNT_SETTINGS_DIALOG_TITLE,
} from "../generated/account-settings-dialog";
import { cn } from "../lib/cn";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogTitle } from "./dialog";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./tabs";
import { IconButton } from "./icon-button";
import { UiIcon } from "./ui-icons";

export interface AccountSettingsSection {
  id: string;
  label: React.ReactNode;
  icon?: React.ReactNode;
  content: React.ReactNode;
}

export interface AccountSettingsDialogLabels {
  /** The close button. Default `"Close"`. */
  close?: string;
}

export interface AccountSettingsDialogProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  /** The heading of the content pane. */
  title: React.ReactNode;
  description?: React.ReactNode;
  /** One sidebar entry each; the active one's `content` fills the pane. */
  sections: readonly AccountSettingsSection[];
  value?: string;
  defaultValue?: string;
  onValueChange?: (id: string) => void;
  /** Pinned under the sidebar entries: sign out, language. */
  sidebarFooter?: React.ReactNode;
  labels?: AccountSettingsDialogLabels;
  className?: string;
}

/**
 * The account modal: a vertical tab rail on the left, the active section on
 * the right. Sections are the caller's, so Profile and Security (or any other)
 * are built from `SettingsSection` and `SettingsRow` inside `content`.
 */
export function AccountSettingsDialog({
  open,
  defaultOpen,
  onOpenChange,
  title,
  description,
  sections,
  value,
  defaultValue,
  onValueChange,
  sidebarFooter,
  labels = {},
  className,
}: AccountSettingsDialogProps) {
  const first = sections[0]?.id ?? "";
  return (
    <Dialog
      {...(open !== undefined ? { open } : {})}
      {...(defaultOpen !== undefined ? { defaultOpen } : {})}
      {...(onOpenChange ? { onOpenChange } : {})}
    >
      <DialogContent
        showCloseButton={false}
        className={cn(ACCOUNT_SETTINGS_DIALOG_CONTENT, className)}
      >
        <Tabs
          orientation="vertical"
          {...(value !== undefined ? { value } : { defaultValue: defaultValue ?? first })}
          {...(onValueChange ? { onValueChange } : {})}
          className={ACCOUNT_SETTINGS_DIALOG_TABS}
        >
          <div className={ACCOUNT_SETTINGS_DIALOG_SIDEBAR}>
            <TabsList className={ACCOUNT_SETTINGS_DIALOG_LIST}>
              {sections.map((s) => (
                <TabsTrigger key={s.id} value={s.id} className={ACCOUNT_SETTINGS_DIALOG_ENTRY}>
                  {s.icon}
                  {s.label}
                </TabsTrigger>
              ))}
            </TabsList>
            {sidebarFooter != null && (
              <div className={ACCOUNT_SETTINGS_DIALOG_FOOTER}>{sidebarFooter}</div>
            )}
          </div>
          <div className={ACCOUNT_SETTINGS_DIALOG_PANE}>
            <div className={ACCOUNT_SETTINGS_DIALOG_HEADING}>
              <DialogTitle className={ACCOUNT_SETTINGS_DIALOG_TITLE}>{title}</DialogTitle>
              {description != null && (
                <DialogDescription className={ACCOUNT_SETTINGS_DIALOG_DESCRIPTION}>{description}</DialogDescription>
              )}
            </div>
            {sections.map((s) => (
              <TabsContent key={s.id} value={s.id} className={ACCOUNT_SETTINGS_DIALOG_PANEL}>
                {s.content}
              </TabsContent>
            ))}
          </div>
        </Tabs>
        <DialogClose asChild>
          <IconButton aria-label={labels.close ?? "Close"} className={ACCOUNT_SETTINGS_DIALOG_CLOSE}>
            <UiIcon name="x" />
          </IconButton>
        </DialogClose>
      </DialogContent>
    </Dialog>
  );
}
