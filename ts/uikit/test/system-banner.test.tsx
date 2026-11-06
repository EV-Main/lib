import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SystemBanner } from "../src/components/shell/system-banner";
import { toneClasses } from "../src/generated/tone";
import { SectionLabel } from "../src/components/shell/page-frame";

describe("SystemBanner tones", () => {
  it("fuses the tone onto the strip", () => {
    const { container } = render(
      <SystemBanner tone="warn" icon={<svg data-testid="glyph" />}>
        Maintenance tonight
      </SystemBanner>,
    );
    expect(screen.getByRole("status")).toHaveClass("rounded-panel", "border", toneClasses.warn);
    expect(container.querySelector('[data-tone="warn"]')).not.toBeNull();
  });

  it("defaults to the neutral tone", () => {
    render(<SystemBanner>Hello</SystemBanner>);
    expect(screen.getByRole("status")).toHaveClass(toneClasses.neutral);
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
