import * as React from "react";
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

const ENTRY =
  "h-8 w-full flex-none justify-start gap-2.5 rounded-(--control-radius) border-0 px-2.5 text-body font-normal text-ink-mid shadow-none data-[state=active]:bg-active data-[state=active]:text-ink data-[state=active]:shadow-none hover:bg-hover [&_svg]:text-ink-soft";

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
        className={cn(
          "flex max-h-[90dvh] max-w-[calc(100%-2rem)] flex-col gap-0 overflow-hidden bg-popover p-0 sm:max-w-260 md:h-170 md:flex-row",
          className,
        )}
      >
        <Tabs
          orientation="vertical"
          {...(value !== undefined ? { value } : { defaultValue: defaultValue ?? first })}
          {...(onValueChange ? { onValueChange } : {})}
          className="min-h-0 flex-1 flex-col gap-0 md:flex-row"
        >
          <div className="flex max-h-44 shrink-0 flex-col gap-2 overflow-y-auto border-b border-border-subtle bg-card p-2 md:max-h-none md:w-60 md:border-b-0 md:border-r">
            <TabsList className="h-auto w-full flex-1 flex-col items-stretch justify-start gap-0.5 bg-transparent p-0">
              {sections.map((s) => (
                <TabsTrigger key={s.id} value={s.id} className={ENTRY}>
                  {s.icon}
                  {s.label}
                </TabsTrigger>
              ))}
            </TabsList>
            {sidebarFooter != null && (
              <div className="flex items-center gap-1 border-t border-border-subtle pt-2">{sidebarFooter}</div>
            )}
          </div>
          <div className="relative flex min-h-0 min-w-0 flex-1 flex-col gap-5 overflow-y-auto p-6 md:p-8">
            <div className="flex flex-col gap-1 pr-10">
              <DialogTitle className="font-display text-h1 font-light text-ink">{title}</DialogTitle>
              {description != null && (
                <DialogDescription className="text-body text-ink-mid">{description}</DialogDescription>
              )}
            </div>
            {sections.map((s) => (
              <TabsContent key={s.id} value={s.id} className="flex flex-col gap-5">
                {s.content}
              </TabsContent>
            ))}
          </div>
        </Tabs>
        <DialogClose asChild>
          <IconButton aria-label={labels.close ?? "Close"} className="absolute right-4 top-4">
            <UiIcon name="x" />
          </IconButton>
        </DialogClose>
      </DialogContent>
    </Dialog>
  );
}
