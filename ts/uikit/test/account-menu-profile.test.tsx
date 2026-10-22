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

  it("makes the card a button when it can open the settings", () => {
    const onOpenProfile = vi.fn();
    const menu = open({ onOpenProfile });
    fireEvent.click(menu.querySelector('[data-slot="account-menu-card"]') as HTMLElement);
    expect(onOpenProfile).toHaveBeenCalledTimes(1);
  });

  it("swaps the body for another tab's content and back", () => {
    const menu = open({
      tabs: [
        { id: "profile", label: "Profile" },
        { id: "notes", label: "Notifications", content: <p>No news</p> },
      ],
    });
    expect(within(menu).getByRole("tab", { name: "Profile" })).toHaveAttribute("aria-selected", "true");
    fireEvent.click(within(menu).getByRole("tab", { name: "Notifications" }));
    expect(within(menu).getByText("No news")).toBeInTheDocument();
    expect(within(menu).queryByText("Billing")).toBeNull();
    fireEvent.click(within(menu).getByRole("tab", { name: "Profile" }));
    expect(within(menu).getByText("Billing")).toBeInTheDocument();
  });

  it("moves between tabs with the arrow keys", () => {
    const menu = open({
      tabs: [
        { id: "profile", label: "Profile" },
        { id: "notes", label: "Notifications", content: <p>No news</p> },
      ],
    });
    fireEvent.keyDown(within(menu).getByRole("tab", { name: "Profile" }), { key: "ArrowRight" });
    expect(within(menu).getByRole("tab", { name: "Notifications" })).toHaveAttribute("aria-selected", "true");
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

  it("shows the language value as a badge and renders the colour-theme slot", () => {
    const onSelect = vi.fn();
    const menu = open({
      language: { label: "Language", value: "EN", onSelect },
      colorTheme: { label: "Color theme", control: <span>System</span> },
    });
    expect(within(menu).getByText("EN")).toBeInTheDocument();
    expect(within(menu).getByText("Color theme")).toBeInTheDocument();
    expect(within(menu).getByText("System")).toBeInTheDocument();
    fireEvent.click(within(menu).getByRole("menuitem", { name: /Language/ }));
    expect(onSelect).toHaveBeenCalledTimes(1);
  });

  it("marks sign out as the danger row", () => {
    const menu = open({});
    expect(within(menu).getByRole("menuitem", { name: "Sign out" })).toHaveAttribute("data-variant", "danger");
  });
});
