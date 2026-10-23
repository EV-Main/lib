import * as React from "react";
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/react";
import { AccountMenu, type AccountMenuProfile } from "../src/components/account-menu";

function open(profile: AccountMenuProfile, onSignOut = () => {}) {
  render(
    <AccountMenu
      account={{ name: "Jane Doe", email: "jane@example.com" }}
      switchHref="/switch"
      onSignOut={onSignOut}
      groups={[{ id: "g", items: [{ id: "billing", href: "/billing", label: "Billing" }] }]}
      profile={profile}
    />,
  );
  fireEvent.click(screen.getByRole("button", { name: "Account" }));
  return screen.getByRole("menu");
}

describe("AccountMenu profile form", () => {
  it("shows the profile card with the plan instead of the plain label", () => {
    const menu = open({ plan: "Free" });
    expect(within(menu).getByText("Jane Doe")).toBeInTheDocument();
    expect(within(menu).getByText("Free")).toBeInTheDocument();
    expect(menu.querySelector('[data-slot="account-menu-card"]')).not.toBeNull();
  });

  it("makes the card a menu item that opens the settings and closes the menu", () => {
    const onOpenProfile = vi.fn();
    const menu = open({ onOpenProfile });
    const card = menu.querySelector('[data-slot="account-menu-card"]') as HTMLElement;
    expect(card).toHaveAttribute("role", "menuitem");
    fireEvent.click(card);
    expect(onOpenProfile).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("swaps the body for another tab's content and back", () => {
    const menu = open({
      tabs: [
        { id: "profile", label: "Profile" },
        { id: "notes", label: "Notifications", content: <p>No news</p> },
      ],
    });
    expect(within(menu).getByRole("menuitemradio", { name: "Profile" })).toHaveAttribute("aria-checked", "true");
    fireEvent.click(within(menu).getByRole("menuitemradio", { name: "Notifications" }));
    expect(screen.getByRole("menu")).toBeInTheDocument();
    expect(within(menu).getByText("No news")).toBeInTheDocument();
    expect(within(menu).queryByText("Billing")).toBeNull();
    fireEvent.click(within(menu).getByRole("menuitemradio", { name: "Profile" }));
    expect(within(menu).getByText("Billing")).toBeInTheDocument();
  });

  it("keeps the tabs inside the menu's own item order", () => {
    const menu = open({
      tabsLabel: "Menu sections",
      tabs: [
        { id: "profile", label: "Profile" },
        { id: "notes", label: "Notifications", content: <p>No news</p> },
      ],
    });
    expect(within(menu).getByRole("group", { name: "Menu sections" })).toBeInTheDocument();
    expect(within(menu).queryByRole("tablist")).toBeNull();
    // The same selector the menu's roving focus uses: the first stop is the first tab.
    const first = menu.querySelector<HTMLElement>("[role^='menuitem']");
    expect(first).toHaveAccessibleName("Profile");
  });

  it("moves selection and focus together with the left and right arrows", () => {
    const menu = open({
      tabs: [
        { id: "profile", label: "Profile" },
        { id: "notes", label: "Notifications", content: <p>No news</p> },
      ],
    });
    const profile = within(menu).getByRole("menuitemradio", { name: "Profile" });
    profile.focus();
    fireEvent.keyDown(profile, { key: "ArrowRight" });
    const notes = within(menu).getByRole("menuitemradio", { name: "Notifications" });
    expect(notes).toHaveAttribute("aria-checked", "true");
    expect(notes).toHaveFocus();
  });

  it("toggles the dark theme through its row and closes", () => {
    const onCheckedChange = vi.fn();
    const menu = open({ darkTheme: { label: "Dark theme", checked: false, onCheckedChange } });
    const row = within(menu).getByRole("menuitemcheckbox", { name: "Dark theme" });
    expect(row).toHaveAttribute("aria-checked", "false");
    fireEvent.click(row);
    expect(onCheckedChange).toHaveBeenCalledWith(true);
    expect(screen.queryByRole("menu")).toBeNull();
  });

  it("shows the language value as a badge and runs its action", () => {
    const onSelect = vi.fn();
    const menu = open({ language: { label: "Language", value: "EN", onSelect } });
    expect(within(menu).getByText("EN")).toBeInTheDocument();
    fireEvent.click(within(menu).getByRole("menuitem", { name: /Language/ }));
    expect(onSelect).toHaveBeenCalledTimes(1);
  });

  it("puts the colour theme behind a submenu of radio options", () => {
    const onValueChange = vi.fn();
    const menu = open({
      colorTheme: {
        label: "Color theme",
        value: "system",
        options: [
          { id: "system", label: "System" },
          { id: "teal", label: "Teal" },
        ],
        onValueChange,
      },
    });
    expect(within(menu).queryByRole("menuitemradio", { name: "Teal" })).toBeNull();
    fireEvent.click(within(menu).getByRole("menuitem", { name: /Color theme/ }));
    expect(within(menu).getByRole("menuitemradio", { name: "System" })).toHaveAttribute("aria-checked", "true");
    fireEvent.click(within(menu).getByRole("menuitemradio", { name: "Teal" }));
    expect(onValueChange).toHaveBeenCalledWith("teal");
  });

  it("marks sign out as the danger row", () => {
    const menu = open({});
    expect(within(menu).getByRole("menuitem", { name: "Sign out" })).toHaveAttribute("data-variant", "danger");
  });
});
