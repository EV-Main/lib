import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ChatStreamText } from "../src/components/chat-stream-text";
import { ChatTyping } from "../src/components/chat-typing";
import { ChatBubble } from "../src/components/chat-message";
import { ChatPanel } from "../src/components/chat-panel";

const words = (c: HTMLElement) => [...c.querySelectorAll<HTMLElement>("[data-slot=chat-word]")];

describe("ChatStreamText", () => {
  it("animates every word of a stream that starts empty-handed, and keeps the text exact", () => {
    const { container } = render(<ChatStreamText text="Net exposure is 38%" streaming />);
    expect(words(container).map((w) => w.textContent)).toEqual(["Net", "exposure", "is", "38%"]);
    expect(container.textContent).toBe("Net exposure is 38%");
  });

  it("shows a finished string at once, with no animated words", () => {
    const { container } = render(<ChatStreamText text="Already complete" />);
    expect(words(container)).toHaveLength(0);
    expect(container.textContent).toBe("Already complete");
  });

  it("animates only the appended tail and leaves earlier word nodes alone", () => {
    const { container, rerender } = render(<ChatStreamText text="one two" streaming />);
    const [one, two] = words(container);
    rerender(<ChatStreamText text="one two three four" streaming />);
    const after = words(container);
    expect(after.map((w) => w.textContent)).toEqual(["one", "two", "three", "four"]);
    expect(after[0]).toBe(one);
    expect(after[1]).toBe(two);
    expect(container.textContent).toBe("one two three four");
  });

  it("folds landed words into plain text, so only the tail keeps wrappers", () => {
    const { container, rerender } = render(<ChatStreamText text="one two" streaming />);
    rerender(<ChatStreamText text="one two three" streaming />);
    fireEvent.animationEnd(words(container)[1]!);
    expect(words(container).map((w) => w.textContent)).toEqual(["three"]);
    expect(container.textContent).toBe("one two three");
  });

  it("does not restart a word that is still landing when more text arrives", () => {
    const { container, rerender } = render(<ChatStreamText text="one" streaming />);
    const one = words(container)[0]!;
    rerender(<ChatStreamText text="one two" streaming />);
    rerender(<ChatStreamText text="one two three" streaming />);
    expect(words(container)[0]).toBe(one);
  });

  it("starts over when the text is replaced rather than extended", () => {
    const { container, rerender } = render(<ChatStreamText text="old answer" streaming />);
    fireEvent.animationEnd(words(container)[1]!);
    rerender(<ChatStreamText text="new reply" streaming />);
    expect(words(container).map((w) => w.textContent)).toEqual(["new", "reply"]);
  });

  it("staggers a burst of words by growing delays", () => {
    const { container } = render(<ChatStreamText text="a b c" streaming />);
    expect(words(container).map((w) => w.style.animationDelay)).toEqual(["0ms", "28ms", "56ms"]);
  });
});

describe("ChatBubble streaming", () => {
  it("wraps string children for animation only once it has streamed", () => {
    const { container, rerender } = render(<ChatBubble>static text</ChatBubble>);
    expect(words(container)).toHaveLength(0);
    expect(container.firstChild).not.toHaveAttribute("data-caret");
    rerender(<ChatBubble streaming>static text more</ChatBubble>);
    expect(container.firstChild).toHaveAttribute("data-caret");
    expect(words(container).length).toBeGreaterThan(0);
  });

  it("keeps its caret slot after streaming ends so the caret can fade out", () => {
    const { container, rerender } = render(<ChatBubble streaming>hello</ChatBubble>);
    rerender(<ChatBubble>hello</ChatBubble>);
    expect(container.firstChild).toHaveAttribute("data-caret");
    expect(container.firstChild).not.toHaveAttribute("data-streaming");
    expect(container.textContent).toBe("hello");
  });
});

describe("ChatTyping", () => {
  it("is a named status with decorative dots and a still ellipsis for reduced motion", () => {
    const { container } = render(<ChatTyping />);
    expect(screen.getByRole("status", { name: "Assistant is thinking" })).toBeInTheDocument();
    expect(container.querySelectorAll("[data-dot][aria-hidden]")).toHaveLength(3);
    expect(container.querySelector("[data-still]")).toHaveTextContent("…");
  });

  it("takes a custom label", () => {
    render(<ChatTyping label="Searching" />);
    expect(screen.getByRole("status", { name: "Searching" })).toBeInTheDocument();
  });
});

describe("ChatPanel fades", () => {
  it("keeps the jump button mounted but hidden and disabled while at the end", () => {
    render(
      <ChatPanel>
        <p>hi</p>
      </ChatPanel>,
    );
    expect(screen.queryByRole("button", { name: "Jump to latest" })).toBeNull();
    const hidden = document.querySelector("[data-chat-jump]");
    expect(hidden).toHaveAttribute("data-chat-jump", "hidden");
    expect(hidden).toBeDisabled();
  });

  it("marks the log scrolled once it leaves the top", () => {
    render(
      <ChatPanel>
        <p>hi</p>
      </ChatPanel>,
    );
    const log = screen.getByRole("log");
    expect(log).not.toHaveAttribute("data-scrolled");
    log.scrollTop = 50;
    fireEvent.scroll(log);
    expect(log).toHaveAttribute("data-scrolled");
  });
});

describe("chat motion in motion.css", () => {
  const css = readFileSync(resolve(process.cwd(), "../../motion.css"), "utf8");
  const chat = css.slice(css.indexOf("/* ── Chat:"));
  const reduce = chat.slice(chat.indexOf("@media (prefers-reduced-motion: reduce)"));
  const keyframes = [...chat.matchAll(/@keyframes (ev-(?:word-in|caret-pulse|typing-wave|chat-in|chat-in-user|chat-part-in))/g)].map((m) => m[1]!);

  it("defines every chat keyframe", () => {
    expect(keyframes).toHaveLength(6);
  });

  it("never plays a chat keyframe under reduced motion", () => {
    for (const name of keyframes) expect(reduce, name).not.toContain(`${name} `);
  });

  it("covers every animated chat part in the reduced-motion block", () => {
    for (const slot of ["chat-word", "chat-typing", "chat-message", "chat-marker", "chat-attachment", "chat-bubble"]) {
      expect(reduce, slot).toContain(`[data-slot="${slot}"]`);
    }
  });
});
