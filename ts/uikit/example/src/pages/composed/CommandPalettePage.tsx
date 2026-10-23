import { useState } from "react";
import { Button, CommandPalette, toast, type CommandPaletteGroup } from "@evinvest/uikit";
import { CreditCardIcon, KeyboardIcon, LayersIcon, PlusIcon, SearchIcon, UserIcon } from "../../gallery/icons";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

const said = (what: string) => () => toast(what);

const GROUPS: CommandPaletteGroup[] = [
  {
    id: "suggestions",
    heading: "Suggestions",
    items: [
      { id: "new", label: "New workspace", icon: <PlusIcon />, shortcut: "⌘N", onSelect: said("New workspace") },
      { id: "symbol", label: "Search symbol", icon: <SearchIcon />, shortcut: "/", onSelect: said("Search symbol") },
      { id: "book", label: "Open order book", icon: <LayersIcon />, shortcut: "⌘O", onSelect: said("Open order book") },
    ],
  },
  {
    id: "settings",
    heading: "Settings",
    items: [
      { id: "profile", label: "Profile", icon: <UserIcon />, shortcut: "⌘,", onSelect: said("Profile") },
      { id: "billing", label: "Billing", icon: <CreditCardIcon />, onSelect: said("Billing") },
      { id: "keys", label: "Keyboard shortcuts", icon: <KeyboardIcon />, shortcut: "⌘/", onSelect: said("Keyboard shortcuts") },
      { id: "locked", label: "Danger zone (disabled)", disabled: true, onSelect: said("never") },
    ],
  },
];

export default function CommandPalettePage() {
  const [open, setOpen] = useState(false);
  return (
    <PageRoot
      title="Command palette"
      lede="One search over commands. The rows filter as you type; arrows move, Enter runs, Escape closes. The shell's own palette (the search in the header) jumps between gallery pages."
    >
      <Block title="CommandPalette" note="Groups, icons, shortcuts, a disabled row and the hint bar.">
        <Specimen>
          <Button variant="secondary" onClick={() => setOpen(true)}>
            Open palette
          </Button>
          <span className="text-body-s text-ink-soft">The header's search (⌘K) opens the shell's palette instead, so the shortcut is not bound here.</span>
        </Specimen>
      </Block>
      <CommandPalette open={open} onOpenChange={setOpen} groups={GROUPS} />
    </PageRoot>
  );
}
