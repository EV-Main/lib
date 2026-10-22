import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SettingsLayout, SettingsSection, SettingsRow } from "../src/components/settings";
import { SectionNav } from "../src/components/section-nav";

describe("Settings", () => {
  it("puts the rail and the main column side by side", () => {
    render(
      <SettingsLayout nav={<SectionNav groups={[{ id: "g", items: [{ id: "a", label: "Account" }] }]} value="a" />}>
        <p>Body</p>
      </SettingsLayout>,
    );
    expect(screen.getByRole("navigation", { name: "Sections" })).toBeInTheDocument();
    expect(screen.getByRole("main")).toHaveTextContent("Body");
    expect(screen.getByRole("button", { name: "Account" })).toHaveAttribute("aria-current", "true");
    expect(screen.getByRole("button", { name: "Account" })).toHaveClass("bg-active");
  });

  it("names a section by its title and shows description and action", () => {
    render(
      <SettingsSection title="Accounts" description="Switch accounts" action={<button type="button">Create team</button>}>
        <span>row</span>
      </SettingsSection>,
    );
    const region = screen.getByRole("region", { name: "Accounts" });
    expect(region).toHaveTextContent("Switch accounts");
    expect(screen.getByRole("button", { name: "Create team" })).toBeInTheDocument();
  });

  it("marks the danger zone with the error edge", () => {
    render(<SettingsSection title="Delete account" tone="danger" />);
    expect(screen.getByRole("region", { name: "Delete account" })).toHaveClass("border-accent-error");
  });

  it("lays settings rows out like form rows, inline for toggles", () => {
    const { container } = render(
      <SettingsSection title="Notifications" flush>
        <SettingsRow label="Email alerts" inline>
          <input type="checkbox" aria-label="Email alerts toggle" />
        </SettingsRow>
      </SettingsSection>,
    );
    expect(container.querySelector('[data-slot="form-row"]')).toHaveAttribute("data-inline", "true");
    expect(screen.getByLabelText("Email alerts toggle")).toBeInTheDocument();
  });
});
