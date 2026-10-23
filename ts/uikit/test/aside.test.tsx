import { describe, it, expect, vi, afterEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { AsideHeader, AsidePanel, AsideRail } from "../src/components/aside";
import { AppShell } from "../src/components/app-shell";

afterEach(() => {
  vi.unstubAllGlobals();
});

function stubWidth(wide: boolean) {
  vi.stubGlobal("matchMedia", () => ({
    matches: wide,
    addEventListener: () => {},
    removeEventListener: () => {},
  }));
}

describe("AsideHeader", () => {
  it("shows the title and only the buttons it was given handlers for", () => {
    const onClose = vi.fn();
    render(<AsideHeader title="Watchlist" onClose={onClose} />);
    expect(screen.getByRole("heading", { name: "Watchlist" })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: "Back" })).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Close" }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("puts the back button first", () => {
    const onBack = vi.fn();
    render(<AsideHeader title="Edit alert" onBack={onBack} onClose={() => {}} />);
    const buttons = screen.getAllByRole("button");
    expect(buttons[0]).toHaveAccessibleName("Back");
    fireEvent.click(buttons[0]!);
    expect(onBack).toHaveBeenCalledTimes(1);
  });
});

describe("AsideRail", () => {
  const items = [
    { id: "list", label: "Watchlist", icon: <svg /> },
    { id: "chat", label: "Assistant", icon: <svg /> },
  ];

  it("marks the open aside's button as pressed", () => {
    render(<AsideRail items={items} value="chat" onValueChange={() => {}} />);
    expect(screen.getByRole("group", { name: "Panels" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Assistant" })).toHaveAttribute("aria-pressed", "true");
    expect(screen.getByRole("button", { name: "Watchlist" })).toHaveAttribute("aria-pressed", "false");
  });

  it("opens another aside, and closes the open one when pressed again", () => {
    const onValueChange = vi.fn();
    render(<AsideRail items={items} value="chat" onValueChange={onValueChange} />);
    fireEvent.click(screen.getByRole("button", { name: "Watchlist" }));
    expect(onValueChange).toHaveBeenLastCalledWith("list");
    fireEvent.click(screen.getByRole("button", { name: "Assistant" }));
    expect(onValueChange).toHaveBeenLastCalledWith(null);
  });

  it("names the buttons by tooltip on focus and keeps end items after the spacer", () => {
    render(<AsideRail items={items} endItems={[{ id: "set", label: "Settings", icon: <svg /> }]} value={null} onValueChange={() => {}} />);
    const names = screen.getAllByRole("button").map((b) => b.getAttribute("aria-label"));
    expect(names).toEqual(["Watchlist", "Assistant", "Settings"]);
    fireEvent.focus(screen.getByRole("button", { name: "Settings" }));
    expect(screen.getByRole("tooltip")).toHaveTextContent("Settings");
  });
});

describe("AsidePanel", () => {
  it("docked: sits in the layout with the border on the inner edge", () => {
    render(
      <AsidePanel aria-label="Watchlist" header={<AsideHeader title="Watchlist" />} footer={<button type="button">Save</button>}>
        <p>Body</p>
      </AsidePanel>,
    );
    const panel = screen.getByRole("complementary", { name: "Watchlist" });
    expect(panel).toHaveClass("border-l");
    expect(panel).toHaveTextContent("Body");
    expect(screen.getByRole("button", { name: "Save" })).toBeInTheDocument();
  });

  it("docked below md falls back to a drawer so the content keeps its room", () => {
    stubWidth(false);
    render(
      <AsidePanel aria-label="Watchlist">
        <p>Body</p>
      </AsidePanel>,
    );
    expect(screen.queryByRole("complementary")).toBeNull();
    expect(document.querySelector('[data-slot="drawer-content"]')).not.toBeNull();
  });

  it("collapseBelow={false} keeps a docked panel docked on a narrow screen", () => {
    stubWidth(false);
    render(
      <AsidePanel aria-label="Watchlist" collapseBelow={false}>
        <p>Body</p>
      </AsidePanel>,
    );
    expect(screen.getByRole("complementary", { name: "Watchlist" })).toBeInTheDocument();
    expect(document.querySelector('[data-slot="drawer-content"]')).toBeNull();
  });

  it("the drawer path reports Escape and a scrim press as a close", () => {
    stubWidth(false);
    const onOpenChange = vi.fn();
    render(
      <AsidePanel aria-label="Watchlist" onOpenChange={onOpenChange}>
        <p>Body</p>
      </AsidePanel>,
    );
    fireEvent.keyDown(document, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);
    onOpenChange.mockClear();
    const scrim = document.querySelector('[data-slot="drawer-overlay"]');
    expect(scrim).not.toBeNull();
    fireEvent.pointerDown(scrim as Element);
    fireEvent.click(scrim as Element);
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("docked on the left borders the right edge, and renders nothing when closed", () => {
    const { rerender } = render(
      <AsidePanel aria-label="Nav" side="left">
        x
      </AsidePanel>,
    );
    expect(screen.getByRole("complementary")).toHaveClass("border-r");
    rerender(
      <AsidePanel aria-label="Nav" side="left" open={false}>
        x
      </AsidePanel>,
    );
    expect(screen.queryByRole("complementary")).toBeNull();
  });

  it("overlay on a wide screen is a modal sheet that reports dismissal", () => {
    stubWidth(true);
    const onOpenChange = vi.fn();
    render(
      <AsidePanel aria-label="Delete alert" mode="overlay" onOpenChange={onOpenChange} header={<AsideHeader title="Delete alert" />}>
        <p>Sure?</p>
      </AsidePanel>,
    );
    const dialog = screen.getByRole("dialog", { name: "Delete alert" });
    expect(dialog).toHaveAttribute("data-slot", "sheet-content");
    expect(dialog).toHaveTextContent("Sure?");
    fireEvent.keyDown(document, { key: "Escape" });
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });

  it("overlay below md uses a drawer", () => {
    stubWidth(false);
    render(
      <AsidePanel aria-label="Delete alert" mode="overlay">
        <p>Sure?</p>
      </AsidePanel>,
    );
    expect(document.querySelector('[data-slot="drawer-content"]')).not.toBeNull();
    expect(document.querySelector('[data-slot="sheet-content"]')).toBeNull();
  });

  it("closed overlay renders nothing", () => {
    stubWidth(true);
    render(
      <AsidePanel aria-label="Delete alert" mode="overlay" open={false}>
        <p>Sure?</p>
      </AsidePanel>,
    );
    expect(screen.queryByText("Sure?")).toBeNull();
  });
});

describe("AppShell aside slot", () => {
  it("renders the aside beside the content column", () => {
    const { container } = render(<AppShell aside={<span>Aside</span>}>Page</AppShell>);
    const slot = container.querySelector('[data-slot="app-shell-aside"]');
    expect(slot).toHaveTextContent("Aside");
    expect(slot?.previousElementSibling).toHaveAttribute("data-slot", "app-shell-content");
  });

  it("draws no slot without an aside", () => {
    const { container } = render(<AppShell>Page</AppShell>);
    expect(container.querySelector('[data-slot="app-shell-aside"]')).toBeNull();
  });
});
