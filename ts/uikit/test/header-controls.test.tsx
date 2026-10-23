import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { HeaderBalance, HeaderSearch, NotificationBell } from "../src/components/header-controls";

describe("HeaderBalance", () => {
  it("shows a skeleton while loading, then reveals the figure", () => {
    const { rerender, container } = render(<HeaderBalance value="$12.00" loading />);
    expect(screen.queryByText("$12.00")).toBeNull();
    expect(container.querySelector('[data-slot="skeleton"]')).not.toBeNull();
    rerender(<HeaderBalance value="$12.00" loading={false} />);
    expect(screen.getByText("$12.00")).toBeInTheDocument();
    expect(container.querySelector('[data-slot="settled"]')).toHaveAttribute("data-state", "revealed");
  });

  it("is a button only when it does something", () => {
    const onClick = vi.fn();
    const { rerender } = render(<HeaderBalance value="$0.00" />);
    expect(screen.queryByRole("button")).toBeNull();
    rerender(<HeaderBalance value="$0.00" onClick={onClick} />);
    fireEvent.click(screen.getByRole("button"));
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("colours the figure for the warn tone", () => {
    render(<HeaderBalance value="$0.00" tone="warn" />);
    expect(screen.getByText("$0.00")).toHaveClass("text-accent-warn-ink");
  });

  it("formats a numeric amount as USD by default and speaks it once", () => {
    const { container } = render(<HeaderBalance amount={1234.5} locales="en" />);
    expect(container.querySelector('[data-slot="animated-number-text"]')).toHaveTextContent("$1,234.50");
  });

  it("keeps the figure's colour class on the animated amount", () => {
    const { container } = render(<HeaderBalance amount={3} tone="warn" />);
    expect(container.querySelector(".text-accent-warn-ink")).toContainElement(
      container.querySelector('[data-slot="animated-number"]'),
    );
  });
});

describe("NotificationBell", () => {
  const items = [
    { id: "a", title: "Order filled", unread: true },
    { id: "b", title: "Statement ready" },
  ];

  it("names the bell with the unread count and draws the dot", () => {
    const { container } = render(<NotificationBell notifications={items} />);
    expect(screen.getByRole("button", { name: "Notifications, 1 unread" })).toBeInTheDocument();
    expect(container.querySelector('[data-slot="notification-dot"]')).not.toBeNull();
  });

  it("has no dot and a plain name when everything is read", () => {
    const { container } = render(<NotificationBell notifications={[{ id: "b", title: "Old" }]} />);
    expect(screen.getByRole("button", { name: "Notifications" })).toBeInTheDocument();
    expect(container.querySelector('[data-slot="notification-dot"]')).toBeNull();
  });

  it("lists notifications in a popover and reports the pick", () => {
    const onSelect = vi.fn();
    render(<NotificationBell notifications={items} onSelect={onSelect} />);
    expect(screen.queryByText("Order filled")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: /Notifications/ }));
    fireEvent.click(screen.getByRole("button", { name: /Order filled/ }));
    expect(onSelect).toHaveBeenCalledWith("a");
  });

  it("says so when the list is empty", () => {
    render(<NotificationBell notifications={[]} labels={{ empty: "Nothing new" }} />);
    fireEvent.click(screen.getByRole("button", { name: "Notifications" }));
    expect(screen.getByText("Nothing new")).toBeInTheDocument();
  });
});

describe("HeaderSearch", () => {
  it("is a button with the placeholder and the shortcut", () => {
    const onClick = vi.fn();
    render(<HeaderSearch onClick={onClick} />);
    const btn = screen.getByRole("button", { name: /Search or jump to…/ });
    expect(btn).toHaveTextContent("Ctrl K");
    fireEvent.click(btn);
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it("shows the Command key on Apple platforms", () => {
    vi.stubGlobal("navigator", { platform: "MacIntel" });
    render(<HeaderSearch />);
    expect(screen.getByRole("button")).toHaveTextContent("⌘K");
    vi.unstubAllGlobals();
  });

  it("lets the caller set the hint", () => {
    render(<HeaderSearch shortcut="/" />);
    expect(screen.getByRole("button")).toHaveTextContent("/");
  });
});

describe("HeaderBalance attributes", () => {
  it("keeps aria and data attributes when it is not a button", () => {
    render(<HeaderBalance value="$1" aria-label="Account balance" data-testid="bal" />);
    expect(screen.getByTestId("bal")).toHaveAttribute("aria-label", "Account balance");
  });
});
