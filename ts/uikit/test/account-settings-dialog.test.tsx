import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { AccountSettingsDialog } from "../src/components/account-settings-dialog";

const sections = [
  { id: "profile", label: "Profile", content: <p>Profile pane</p> },
  { id: "security", label: "Security", content: <p>Security pane</p> },
];

describe("AccountSettingsDialog", () => {
  it("is a named modal with the first section shown", () => {
    render(<AccountSettingsDialog open title="Account settings" sections={sections} />);
    expect(screen.getByRole("dialog", { name: "Account settings" })).toBeInTheDocument();
    expect(screen.getByText("Profile pane")).toBeInTheDocument();
    expect(screen.queryByText("Security pane")).toBeNull();
  });

  it("switches the pane from the vertical tab rail", () => {
    render(<AccountSettingsDialog open title="Account settings" sections={sections} />);
    expect(screen.getByRole("tablist")).toHaveAttribute("aria-orientation", "vertical");
    fireEvent.click(screen.getByRole("tab", { name: "Security" }));
    expect(screen.getByText("Security pane")).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: "Security" })).toHaveAttribute("aria-selected", "true");
  });

  it("moves between tabs with the arrow keys", () => {
    render(<AccountSettingsDialog open title="Account settings" sections={sections} />);
    const profile = screen.getByRole("tab", { name: "Profile" });
    profile.focus();
    fireEvent.keyDown(profile, { key: "ArrowDown" });
    expect(screen.getByRole("tab", { name: "Security" })).toHaveAttribute("tabindex", "0");
  });

  it("closes through the labelled button and reports it", () => {
    const onOpenChange = vi.fn();
    render(
      <AccountSettingsDialog
        open
        onOpenChange={onOpenChange}
        title="Account settings"
        sections={sections}
        labels={{ close: "Close dialog" }}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Close dialog" }));
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("renders the sidebar footer", () => {
    render(
      <AccountSettingsDialog
        open
        title="Account settings"
        sections={sections}
        sidebarFooter={<button type="button">Sign out</button>}
      />,
    );
    expect(screen.getByRole("button", { name: "Sign out" })).toBeInTheDocument();
  });
});
