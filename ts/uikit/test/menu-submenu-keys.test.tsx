import { describe, it, expect, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from "../src/components/context-menu";
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "../src/components/menubar";

describe("ContextMenu submenu and checkbox", () => {
  function setup(onToggle = vi.fn()) {
    render(
      <ContextMenu>
        <ContextMenuTrigger>area</ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuCheckboxItem checked={false} onClick={onToggle}>
            Grid
          </ContextMenuCheckboxItem>
          <ContextMenuSub>
            <ContextMenuSubTrigger>More</ContextMenuSubTrigger>
            <ContextMenuSubContent>
              <button role="menuitem" type="button">
                Deep
              </button>
            </ContextMenuSubContent>
          </ContextMenuSub>
        </ContextMenuContent>
      </ContextMenu>,
    );
    fireEvent.contextMenu(screen.getByText("area"));
  }

  it("keeps the menu open when a checkbox item is toggled", () => {
    const onToggle = vi.fn();
    setup(onToggle);
    fireEvent.click(screen.getByRole("menuitemcheckbox", { name: "Grid" }));
    expect(onToggle).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("menu")).toBeInTheDocument();
  });

  it("opens the flyout with ArrowRight onto its first item and closes it with ArrowLeft", () => {
    setup();
    const trigger = screen.getByRole("menuitem", { name: "More" });
    fireEvent.keyDown(trigger, { key: "ArrowRight" });
    expect(screen.getByRole("menuitem", { name: "Deep" })).toHaveFocus();
    fireEvent.keyDown(screen.getByRole("menuitem", { name: "Deep" }), { key: "ArrowLeft" });
    expect(screen.queryByRole("menuitem", { name: "Deep" })).toBeNull();
    expect(trigger).toHaveFocus();
  });
});

describe("Menubar submenu", () => {
  it("opens with ArrowRight onto its first item and closes with Escape, refocusing the trigger", () => {
    render(
      <Menubar>
        <MenubarMenu>
          <MenubarTrigger>File</MenubarTrigger>
          <MenubarContent>
            <MenubarSub>
              <MenubarSubTrigger>Share</MenubarSubTrigger>
              <MenubarSubContent>
                <MenubarItem>Email</MenubarItem>
              </MenubarSubContent>
            </MenubarSub>
          </MenubarContent>
        </MenubarMenu>
      </Menubar>,
    );
    fireEvent.click(screen.getByText("File"));
    const trigger = screen.getByRole("menuitem", { name: "Share" });
    fireEvent.keyDown(trigger, { key: "ArrowRight" });
    expect(screen.getByRole("menuitem", { name: "Email" })).toHaveFocus();
    fireEvent.keyDown(screen.getByRole("menuitem", { name: "Email" }), { key: "Escape" });
    expect(trigger).toHaveFocus();
  });
});
