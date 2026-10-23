import { describe, it, expect, vi, afterEach } from "vitest";
import { render, screen, fireEvent, waitFor, act } from "@testing-library/react";
import { KeyValue } from "../src/components/key-value";

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe("KeyValue", () => {
  it("shows the label above the value and no button without copyText", () => {
    render(<KeyValue label="Account ID" value="acc_42" />);
    expect(screen.getByText("Account ID")).toBeInTheDocument();
    expect(screen.getByText("acc_42")).toBeInTheDocument();
    expect(screen.queryByRole("button")).toBeNull();
  });

  it("copies through the clipboard API and announces it politely", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal("navigator", { clipboard: { writeText } });
    render(<KeyValue label="ID" value="acc_42" copyText="acc_42" />);
    expect(screen.getByRole("status")).toHaveAttribute("aria-live", "polite");
    expect(screen.getByRole("status")).toBeEmptyDOMElement();
    fireEvent.click(screen.getByRole("button", { name: "Copy ID" }));
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("Copied"));
    expect(writeText).toHaveBeenCalledWith("acc_42");
  });

  it("clears the message after a moment", async () => {
    vi.stubGlobal("navigator", { clipboard: { writeText: vi.fn().mockResolvedValue(undefined) } });
    render(<KeyValue label="ID" value="x" copyText="x" />);
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    fireEvent.click(screen.getByRole("button", { name: "Copy ID" }));
    await vi.waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("Copied"));
    await act(async () => {
      await vi.advanceTimersByTimeAsync(2100);
    });
    expect(screen.getByRole("status")).toBeEmptyDOMElement();
  });

  it("falls back to execCommand when the clipboard API is missing", async () => {
    vi.stubGlobal("navigator", {});
    const exec = vi.fn().mockReturnValue(true);
    Object.defineProperty(document, "execCommand", { value: exec, configurable: true });
    render(<KeyValue label="ID" value="x" copyText="x" labels={{ copied: "Done" }} />);
    fireEvent.click(screen.getByRole("button", { name: "Copy ID" }));
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent("Done"));
    expect(exec).toHaveBeenCalledWith("copy");
  });

  it("keeps the fallback's helper inside the dialog that holds focus", async () => {
    vi.stubGlobal("navigator", {});
    let host: Element | null = null;
    const exec = vi.fn(() => {
      host = document.querySelector("textarea")?.parentElement ?? null;
      return true;
    });
    Object.defineProperty(document, "execCommand", { value: exec, configurable: true });
    render(
      <div role="dialog" aria-label="Account">
        <KeyValue label="ID" value="x" copyText="x" />
      </div>,
    );
    screen.getByRole("button", { name: "Copy ID" }).focus();
    fireEvent.click(screen.getByRole("button", { name: "Copy ID" }));
    await waitFor(() => expect(exec).toHaveBeenCalled());
    expect(host).toBe(screen.getByRole("dialog"));
  });

  it("names the copy button after a text label, and falls back to Copy", () => {
    render(
      <>
        <KeyValue label="Email" value="a" copyText="a" />
        <KeyValue label={<b>Node</b>} value="b" copyText="b" />
      </>,
    );
    expect(screen.getByRole("button", { name: "Copy Email" })).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Copy" })).toBeInTheDocument();
  });
});
