import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SystemBanner } from "../src/components/shell/system-banner";
import { SectionLabel } from "../src/components/shell/page-frame";

describe("SystemBanner tones", () => {
  it("tints a warn strip and its icon", () => {
    const { container } = render(
      <SystemBanner tone="warn" icon={<svg data-testid="glyph" />}>
        Maintenance tonight
      </SystemBanner>,
    );
    expect(screen.getByRole("status")).toHaveClass("rounded-panel", "border", "border-accent-warn/40", "bg-accent-warn/10");
    expect(screen.getByRole("status")).not.toHaveClass("bg-card");
    expect(screen.getByTestId("glyph").parentElement).toHaveClass("text-accent-warn");
    expect(container.querySelector('[data-tone="warn"]')).not.toBeNull();
  });

  it.each([
    ["neutral", "text-ink-soft"],
    ["primary", "text-primary-ink"],
    ["info", "text-accent-info-ink"],
  ] as const)("tints the %s icon", (tone, cls) => {
    render(
      <SystemBanner tone={tone} icon={<svg data-testid="glyph" />}>
        Hello
      </SystemBanner>,
    );
    expect(screen.getByTestId("glyph").parentElement).toHaveClass(cls);
  });

  it("renders an action after the message", () => {
    render(<SystemBanner action={<a href="#d">Details</a>}>Hello</SystemBanner>);
    expect(screen.getByRole("link", { name: "Details" })).toBeInTheDocument();
  });
});

describe("SectionLabel", () => {
  it("is tracked uppercase in the muted tone", () => {
    render(<SectionLabel>General</SectionLabel>);
    const el = screen.getByText("General");
    expect(el).toHaveClass("uppercase", "tracking-widest", "text-ink-soft");
  });
});
