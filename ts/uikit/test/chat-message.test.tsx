import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ChatAttachment, ChatBubble, ChatMarker, ChatMessage } from "../src/components/chat-message";

describe("ChatBubble", () => {
  it("boxes user and muted text and leaves the assistant's open", () => {
    render(
      <>
        <ChatBubble variant="user">u</ChatBubble>
        <ChatBubble variant="muted">m</ChatBubble>
        <ChatBubble>a</ChatBubble>
      </>,
    );
    expect(screen.getByText("u")).toHaveClass("border", "bg-secondary");
    expect(screen.getByText("m")).toHaveClass("bg-muted");
    expect(screen.getByText("a")).not.toHaveClass("border");
  });

  it("marks a bubble that is still receiving text, for the caret in motion.css", () => {
    render(
      <>
        <ChatBubble streaming>s</ChatBubble>
        <ChatBubble>d</ChatBubble>
      </>,
    );
    expect(screen.getByText("s")).toHaveAttribute("data-streaming");
    expect(screen.getByText("d")).not.toHaveAttribute("data-streaming");
  });
});

describe("ChatMessage", () => {
  it("enters with the rise motion and shows who and when", () => {
    const { container } = render(
      <ChatMessage role="assistant" name="Assistant" time="13:04">
        <ChatBubble>Hi</ChatBubble>
      </ChatMessage>,
    );
    expect(container.firstChild).toHaveAttribute("data-enter", "rise");
    expect(screen.getByText("Assistant")).toBeInTheDocument();
    expect(screen.getByText("13:04")).toBeInTheDocument();
  });

  it("aligns the user's turn to the end and has no avatar tile", () => {
    const { container } = render(
      <ChatMessage role="user" name="You">
        <ChatBubble variant="user">Hello</ChatBubble>
      </ChatMessage>,
    );
    expect(container.firstChild).toHaveClass("justify-end");
    expect(container.querySelector("svg")).toBeNull();
  });

  it("compact drops the header and the avatar", () => {
    const { container } = render(
      <ChatMessage role="assistant" name="Assistant" compact>
        <ChatBubble>Hi</ChatBubble>
      </ChatMessage>,
    );
    expect(screen.queryByText("Assistant")).toBeNull();
    expect(container.querySelector("svg")).toBeNull();
  });
});

describe("ChatMarker", () => {
  it("announces a running status and stays silent once done", () => {
    const { rerender } = render(<ChatMarker type="status" state="running">Generating response…</ChatMarker>);
    expect(screen.getByRole("status")).toHaveTextContent("Generating response…");
    rerender(<ChatMarker type="status">Thought for 6s</ChatMarker>);
    expect(screen.queryByRole("status")).toBeNull();
  });

  it("shows a tool call with its detail and meta, and flags a failure", () => {
    const { container, rerender } = render(
      <ChatMarker type="tool" title="get_positions" detail="account=main" meta="0.4s" />,
    );
    expect(screen.getByText("get_positions")).toBeInTheDocument();
    expect(screen.getByText("0.4s")).toHaveClass("text-ink-soft");
    rerender(<ChatMarker type="tool" state="error" title="get_positions" meta="Failed · 1.2s" />);
    expect(container.firstChild).toHaveAttribute("data-state", "error");
    expect(screen.getByText("Failed · 1.2s")).toHaveClass("text-accent-error-ink");
  });

  it("draws a labelled separator", () => {
    render(<ChatMarker type="separator">Today</ChatMarker>);
    // No separator role: it would hide the label from assistive tech.
    expect(screen.getByText("Today")).toBeVisible();
    expect(screen.queryByRole("separator")).toBeNull();
  });
});

describe("ChatAttachment", () => {
  it("shows name and meta and removes through a named button", () => {
    const onRemove = vi.fn();
    render(<ChatAttachment name="q3.pdf" meta="PDF · 1.2 MB" onRemove={onRemove} />);
    expect(screen.getByText("PDF · 1.2 MB")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Remove q3.pdf" }));
    expect(onRemove).toHaveBeenCalledTimes(1);
  });

  it("reports upload progress as a progressbar and in the meta", () => {
    render(<ChatAttachment type="image" state="uploading" name="chart.png" progress={62} />);
    expect(screen.getByRole("progressbar", { name: "chart.png" })).toHaveAttribute("aria-valuenow", "62");
    expect(screen.getByText("Uploading… 62%")).toBeInTheDocument();
  });

  it("clamps progress to the range", () => {
    render(<ChatAttachment state="uploading" name="a" progress={140} />);
    expect(screen.getByRole("progressbar")).toHaveAttribute("aria-valuenow", "100");
  });
});
