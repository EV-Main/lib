import { describe, it, expect, vi } from "vitest";
import { act, render, screen, fireEvent } from "@testing-library/react";
import { ChatDock, ChatPanel } from "../src/components/chat-panel";

// jsdom does no layout, so the scroll geometry is set by hand.
function geometry(el: HTMLElement, g: { scrollHeight: number; clientHeight: number; scrollTop: number }) {
  Object.defineProperty(el, "scrollHeight", { value: g.scrollHeight, configurable: true });
  Object.defineProperty(el, "clientHeight", { value: g.clientHeight, configurable: true });
  el.scrollTop = g.scrollTop;
}

const log = () => screen.getByRole("log");

describe("ChatPanel", () => {
  it("is a polite, named, keyboard-scrollable log", () => {
    render(
      <ChatPanel>
        <p>hello</p>
      </ChatPanel>,
    );
    expect(log()).toHaveAttribute("aria-live", "polite");
    expect(log()).toHaveAccessibleName("Conversation");
    expect(log()).toHaveAttribute("tabindex", "0");
  });

  it("shows the empty state, and no log, while there is nothing to read", () => {
    render(<ChatPanel empty={<p>How can I help?</p>} />);
    expect(screen.getByText("How can I help?")).toBeInTheDocument();
    expect(screen.queryByRole("log")).toBeNull();
  });

  it("follows new content while pinned to the end", async () => {
    const { rerender } = render(
      <ChatPanel>
        <p>one</p>
      </ChatPanel>,
    );
    geometry(log(), { scrollHeight: 500, clientHeight: 200, scrollTop: 300 });
    rerender(
      <ChatPanel>
        <p>one</p>
        <p>two</p>
      </ChatPanel>,
    );
    await act(async () => {
      await Promise.resolve();
    });
    expect(log().scrollTop).toBe(500);
  });

  it("lets go when the reader scrolls up and offers a jump button", async () => {
    const { rerender } = render(
      <ChatPanel>
        <p>one</p>
      </ChatPanel>,
    );
    geometry(log(), { scrollHeight: 1000, clientHeight: 200, scrollTop: 100 });
    fireEvent.scroll(log());
    expect(screen.getByRole("button", { name: "Jump to latest" })).toBeInTheDocument();
    rerender(
      <ChatPanel>
        <p>one</p>
        <p>two</p>
      </ChatPanel>,
    );
    await act(async () => {
      await Promise.resolve();
    });
    expect(log().scrollTop).toBe(100);
  });

  it("jumps to the end and hides the button", () => {
    render(
      <ChatPanel>
        <p>one</p>
      </ChatPanel>,
    );
    geometry(log(), { scrollHeight: 1000, clientHeight: 200, scrollTop: 100 });
    fireEvent.scroll(log());
    fireEvent.click(screen.getByRole("button", { name: "Jump to latest" }));
    expect(log().scrollTop).toBe(1000);
    expect(screen.queryByRole("button", { name: "Jump to latest" })).toBeNull();
  });

  it("shows the empty state when the children are conditional falsy values", () => {
    const items: string[] = [];
    render(<ChatPanel empty={<p>How can I help?</p>}>{items.length > 0 && <p>turn</p>}</ChatPanel>);
    expect(screen.getByText("How can I help?")).toBeInTheDocument();
  });

  it("marks the log busy while streaming", () => {
    const { rerender } = render(<ChatPanel streaming>x</ChatPanel>);
    expect(log()).toHaveAttribute("aria-busy", "true");
    rerender(<ChatPanel>x</ChatPanel>);
    expect(log()).toHaveAttribute("aria-busy", "false");
  });

  it("keeps focus in the log after the jump button goes away", () => {
    render(<ChatPanel>x</ChatPanel>);
    geometry(log(), { scrollHeight: 1000, clientHeight: 200, scrollTop: 100 });
    fireEvent.scroll(log());
    screen.getByRole("button", { name: "Jump to latest" }).focus();
    fireEvent.click(screen.getByRole("button", { name: "Jump to latest" }));
    expect(log()).toHaveFocus();
  });

  it("renders the composer under the log", () => {
    render(<ChatPanel composer={<textarea aria-label="Message" />}>x</ChatPanel>);
    expect(screen.getByRole("textbox", { name: "Message" })).toBeInTheDocument();
  });
});

describe("ChatDock", () => {
  it("expanded: header, conversation and composer, and collapses from the header", () => {
    const onExpandedChange = vi.fn();
    render(
      <ChatDock title="Assistant" onExpandedChange={onExpandedChange} composer={<textarea aria-label="Message" />}>
        <p>turn</p>
      </ChatDock>,
    );
    expect(screen.getByRole("heading", { name: "Assistant" })).toBeInTheDocument();
    expect(screen.getByText("turn")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Collapse" }));
    expect(onExpandedChange).toHaveBeenCalledWith(false);
    expect(screen.getByRole("button", { name: "Open assistant" })).toBeInTheDocument();
    expect(screen.queryByText("turn")).toBeNull();
  });

  it("moves focus to the launcher on collapse and to the composer on expand", () => {
    render(
      <ChatDock title="Assistant" composer={<textarea aria-label="Message" />}>
        <p>turn</p>
      </ChatDock>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Collapse" }));
    expect(screen.getByRole("button", { name: "Open assistant" })).toHaveFocus();
    fireEvent.click(screen.getByRole("button", { name: "Open assistant" }));
    expect(screen.getByRole("textbox", { name: "Message" })).toHaveFocus();
  });

  it("collapsed: a launcher bar that expands it", () => {
    const onExpandedChange = vi.fn();
    render(
      <ChatDock title="Assistant" defaultExpanded={false} onExpandedChange={onExpandedChange}>
        <p>turn</p>
      </ChatDock>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Open assistant" }));
    expect(onExpandedChange).toHaveBeenCalledWith(true);
    expect(screen.getByText("turn")).toBeInTheDocument();
  });

  it("obeys a controlled expanded value", () => {
    render(
      <ChatDock title="Assistant" expanded={false}>
        <p>turn</p>
      </ChatDock>,
    );
    fireEvent.click(screen.getByRole("button", { name: "Open assistant" }));
    expect(screen.queryByText("turn")).toBeNull();
  });
});
