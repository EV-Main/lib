import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ChatComposer, ModelChip } from "../src/components/chat-composer";

const field = () => screen.getByRole("textbox") as HTMLTextAreaElement;

describe("ChatComposer", () => {
  it("sends the trimmed text on Enter and clears the field", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} />);
    fireEvent.change(field(), { target: { value: "  hello  " } });
    fireEvent.keyDown(field(), { key: "Enter" });
    expect(onSend).toHaveBeenCalledWith("hello");
    expect(field().value).toBe("");
  });

  it("keeps Shift+Enter for a new line", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} />);
    fireEvent.change(field(), { target: { value: "a" } });
    const notPrevented = fireEvent.keyDown(field(), { key: "Enter", shiftKey: true });
    expect(onSend).not.toHaveBeenCalled();
    expect(notPrevented).toBe(true);
  });

  it("does not send while an IME candidate is being confirmed", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} />);
    fireEvent.change(field(), { target: { value: "かな" } });
    fireEvent.keyDown(field(), { key: "Enter", isComposing: true });
    expect(onSend).not.toHaveBeenCalled();
  });

  it("does not send on the Enter that confirms an IME candidate in Safari", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} />);
    fireEvent.change(field(), { target: { value: "привет" } });
    fireEvent.keyDown(field(), { key: "Enter", keyCode: 229 });
    expect(onSend).not.toHaveBeenCalled();
  });

  it("names the field", () => {
    render(<ChatComposer onSend={() => {}} labels={{ field: "Сообщение" }} />);
    expect(screen.getByRole("textbox", { name: "Сообщение" })).toBeInTheDocument();
  });

  it("ignores an empty message and disables the send button", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} />);
    fireEvent.keyDown(field(), { key: "Enter" });
    expect(onSend).not.toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Send" })).toBeDisabled();
  });

  it("sends from the button once there is text", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} defaultValue="hi" />);
    fireEvent.click(screen.getByRole("button", { name: "Send" }));
    expect(onSend).toHaveBeenCalledWith("hi");
  });

  it("streaming swaps send for stop, and Enter still queues a follow-up", () => {
    const onSend = vi.fn();
    const onStop = vi.fn();
    const { container } = render(<ChatComposer onSend={onSend} onStop={onStop} streaming defaultValue="next" />);
    expect(screen.queryByRole("button", { name: "Send" })).toBeNull();
    expect(container.firstChild).toHaveAttribute("data-state", "streaming");
    fireEvent.click(screen.getByRole("button", { name: "Stop" }));
    expect(onStop).toHaveBeenCalledTimes(1);
    fireEvent.keyDown(field(), { key: "Enter" });
    expect(onSend).toHaveBeenCalledWith("next");
  });

  it("moves focus to the field after Stop", () => {
    render(<ChatComposer onSend={() => {}} onStop={() => {}} streaming />);
    const stop = screen.getByRole("button", { name: "Stop" });
    stop.focus();
    fireEvent.click(stop);
    expect(field()).toHaveFocus();
  });

  it("shows the attach button, the attachments and the tools it was given", () => {
    const onAttach = vi.fn();
    render(<ChatComposer onSend={() => {}} onAttach={onAttach} attachments={<span>file.pdf</span>} tools={<span>Fast</span>} />);
    fireEvent.click(screen.getByRole("button", { name: "Attach file" }));
    expect(onAttach).toHaveBeenCalledTimes(1);
    expect(screen.getByText("file.pdf")).toBeInTheDocument();
    expect(screen.getByText("Fast")).toBeInTheDocument();
  });

  it("does nothing when disabled", () => {
    const onSend = vi.fn();
    render(<ChatComposer onSend={onSend} disabled defaultValue="x" />);
    fireEvent.keyDown(field(), { key: "Enter" });
    expect(onSend).not.toHaveBeenCalled();
    expect(field()).toBeDisabled();
  });
});

describe("ModelChip", () => {
  const models = [
    { id: "fast", label: "Fast" },
    { id: "deep", label: "Reasoning" },
  ];

  it("names the current model and picks another from the menu", () => {
    const onValueChange = vi.fn();
    render(<ModelChip models={models} value="fast" onValueChange={onValueChange} />);
    fireEvent.click(screen.getByRole("button", { name: "Model: Fast" }));
    expect(screen.getByRole("menuitemradio", { name: "Fast" })).toHaveAttribute("aria-checked", "true");
    fireEvent.click(screen.getByRole("menuitemradio", { name: "Reasoning" }));
    expect(onValueChange).toHaveBeenCalledWith("deep");
  });
});
