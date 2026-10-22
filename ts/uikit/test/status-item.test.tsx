import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { StatusItem, StatusDot } from "../src/components/status-item";

describe("StatusItem", () => {
  it("shows the label with a hidden dot of the tone's colour", () => {
    const { container } = render(<StatusItem tone="positive">Connected</StatusItem>);
    expect(screen.getByText("Connected")).toBeInTheDocument();
    const dot = container.querySelector('[data-slot="status-dot"]');
    expect(dot).toHaveClass("bg-positive");
    expect(dot).toHaveAttribute("aria-hidden", "true");
  });

  it("defaults to the neutral tone", () => {
    const { container } = render(<StatusItem>Idle</StatusItem>);
    expect(container.firstChild).toHaveAttribute("data-tone", "neutral");
  });

  it("renders the optional value after the label", () => {
    render(
      <StatusItem tone="warn" value="49ms">
        Latency
      </StatusItem>,
    );
    expect(screen.getByText("49ms")).toHaveClass("tabular-nums");
  });

  it("exposes the dot on its own", () => {
    const { container } = render(<StatusDot tone="error" />);
    expect(container.firstChild).toHaveClass("bg-accent-error");
  });
});
