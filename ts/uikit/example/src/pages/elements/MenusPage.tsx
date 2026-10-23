import { useState } from "react";
import {
  Button,
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuTrigger,
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@evinvest/uikit";
import { CopyIcon, TrashIcon } from "../../gallery/icons";
import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { CommandSearch } from "./command-search";

function Dropdown() {
  const [alerts, setAlerts] = useState(true);
  const [sort, setSort] = useState("recent");
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="secondary">Actions</Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
        <DropdownMenuLabel>Position</DropdownMenuLabel>
        <DropdownMenuItem>
          <CopyIcon />
          Duplicate
          <DropdownMenuShortcut>⌘D</DropdownMenuShortcut>
        </DropdownMenuItem>
        <DropdownMenuItem disabled>Move (locked)</DropdownMenuItem>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>Export</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>CSV</DropdownMenuItem>
            <DropdownMenuItem>PDF</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuSeparator />
        <DropdownMenuCheckboxItem checked={alerts} onClick={() => setAlerts((v) => !v)}>
          Price alerts
        </DropdownMenuCheckboxItem>
        <DropdownMenuRadioGroup value={sort} onValueChange={setSort}>
          <DropdownMenuRadioItem value="recent">Most recent</DropdownMenuRadioItem>
          <DropdownMenuRadioItem value="size">Largest</DropdownMenuRadioItem>
        </DropdownMenuRadioGroup>
        <DropdownMenuSeparator />
        <DropdownMenuItem variant="danger">
          <TrashIcon />
          Delete
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function Context() {
  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div className="border-input text-body text-ink-soft flex h-24 w-64 items-center justify-center rounded-lg border border-dashed">
          Right-click here
        </div>
      </ContextMenuTrigger>
      <ContextMenuContent>
        <ContextMenuItem>
          Copy
          <ContextMenuShortcut>⌘C</ContextMenuShortcut>
        </ContextMenuItem>
        <ContextMenuItem disabled>Paste</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem variant="danger">Delete</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

function MenuBar() {
  return (
    <Menubar>
      {["File", "Edit"].map((name) => (
        <MenubarMenu key={name}>
          <MenubarTrigger>{name}</MenubarTrigger>
          <MenubarContent>
            <MenubarItem>
              New
              <MenubarShortcut>⌘N</MenubarShortcut>
            </MenubarItem>
            <MenubarSeparator />
            <MenubarItem variant="danger">Close all</MenubarItem>
          </MenubarContent>
        </MenubarMenu>
      ))}
    </Menubar>
  );
}

export default function MenusPage() {
  return (
    <PageRoot
      title="Menus"
      lede="Menus open in a layer above the page and close one layer at a time. A destructive item is marked danger, never only by position."
    >
      <Block title="DropdownMenu" note="Icons, shortcuts, a submenu, a checkbox item, a radio group, a disabled item and a danger item.">
        <Specimen>
          <Dropdown />
        </Specimen>
      </Block>
      <Block title="ContextMenu">
        <Specimen>
          <Context />
        </Specimen>
      </Block>
      <Block title="Menubar">
        <Specimen>
          <MenuBar />
        </Specimen>
      </Block>
      <Block title="Command" note="Server-driven: rows arrive 600 ms after typing; arrows and Enter pick.">
        <Specimen>
          <CommandSearch />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
