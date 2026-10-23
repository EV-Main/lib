import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { ConfirmDialog } from "../src/components/confirm-dialog";

function deferred() {
  let resolve!: () => void;
  let reject!: (e: Error) => void;
  const promise = new Promise<void>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}

describe("ConfirmDialog", () => {
  it("is an alert dialog named by its title and described by the text", () => {
    render(
      <ConfirmDialog open title="Delete workspace?" description="Cannot be undone." confirmLabel="Delete" onConfirm={() => {}} />,
    );
    expect(screen.getByRole("alertdialog")).toHaveTextContent("Delete workspace?");
    expect(screen.getByText("Cannot be undone.")).toBeInTheDocument();
  });

  it("closes after a synchronous confirm", async () => {
    const onOpenChange = vi.fn();
    const onConfirm = vi.fn();
    render(<ConfirmDialog defaultOpen onOpenChange={onOpenChange} title="Sure?" confirmLabel="Yes" onConfirm={onConfirm} />);
    fireEvent.click(screen.getByRole("button", { name: "Yes" }));
    await waitFor(() => expect(onOpenChange).toHaveBeenCalledWith(false));
    expect(onConfirm).toHaveBeenCalledTimes(1);
  });

  it("stays open and disabled while the promise is pending, then closes", async () => {
    const d = deferred();
    const onOpenChange = vi.fn();
    render(<ConfirmDialog defaultOpen onOpenChange={onOpenChange} title="Sure?" confirmLabel="Yes" onConfirm={() => d.promise} />);
    fireEvent.click(screen.getByRole("button", { name: "Yes" }));
    await waitFor(() => expect(screen.getByRole("button", { name: /Yes/ })).toHaveAttribute("aria-disabled", "true"));
    expect(screen.getByRole("button", { name: "Cancel" })).toHaveAttribute("aria-disabled", "true");
    expect(screen.getByRole("status")).toBeInTheDocument();
    fireEvent.keyDown(document, { key: "Escape" });
    expect(onOpenChange).not.toHaveBeenCalled();
    d.resolve();
    await waitFor(() => expect(onOpenChange).toHaveBeenCalledWith(false));
  });

  it("keeps the dialog open and shows the error when the promise rejects", async () => {
    const onOpenChange = vi.fn();
    render(
      <ConfirmDialog
        defaultOpen
        onOpenChange={onOpenChange}
        title="Sure?"
        confirmLabel="Yes"
        onConfirm={() => Promise.reject(new Error("Server said no"))}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Yes" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Server said no");
    expect(onOpenChange).not.toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Yes" })).toHaveAttribute("aria-disabled", "false");
  });

  it("keeps focus on the confirm button while pending and ignores a second activation", async () => {
    const d = deferred();
    const onConfirm = vi.fn(() => d.promise);
    render(<ConfirmDialog defaultOpen title="Sure?" confirmLabel="Yes" onConfirm={onConfirm} />);
    const yes = screen.getByRole("button", { name: "Yes" });
    yes.focus();
    fireEvent.click(yes);
    await waitFor(() => expect(yes).toHaveAttribute("aria-disabled", "true"));
    expect(yes).not.toBeDisabled();
    expect(yes).toHaveFocus();
    fireEvent.click(yes);
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onConfirm).toHaveBeenCalledTimes(1);
    expect(screen.getByRole("alertdialog")).toBeInTheDocument();
    d.resolve();
  });

  it("returns focus to the trigger after a successful confirm", async () => {
    render(
      <ConfirmDialog
        trigger={<button type="button">Delete</button>}
        title="Sure?"
        confirmLabel="Yes"
        onConfirm={() => Promise.resolve()}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Delete" }));
    fireEvent.click(await screen.findByRole("button", { name: "Yes" }));
    await waitFor(() => expect(screen.queryByRole("alertdialog")).toBeNull());
    expect(screen.getByRole("button", { name: "Delete" })).toHaveFocus();
  });

  it("keeps focus inside the dialog after a rejection", async () => {
    render(<ConfirmDialog defaultOpen title="Sure?" confirmLabel="Yes" onConfirm={() => Promise.reject(new Error("No"))} />);
    const yes = screen.getByRole("button", { name: "Yes" });
    yes.focus();
    fireEvent.click(yes);
    await screen.findByRole("alert");
    expect(screen.getByRole("alertdialog")).toContainElement(document.activeElement as HTMLElement);
  });

  it("drops a stale error when a controlled parent reopens the dialog", async () => {
    const props = { title: "Sure?", confirmLabel: "Yes", onConfirm: () => Promise.reject(new Error("Nope")) };
    const { rerender } = render(<ConfirmDialog open {...props} />);
    fireEvent.click(screen.getByRole("button", { name: "Yes" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Nope");
    rerender(<ConfirmDialog open={false} {...props} />);
    rerender(<ConfirmDialog open {...props} />);
    expect(screen.queryByRole("alert")).toBeNull();
  });

  it("uses the danger face for the confirm button", () => {
    render(<ConfirmDialog open tone="danger" title="Delete?" confirmLabel="Delete" onConfirm={() => {}} />);
    expect(screen.getByRole("button", { name: "Delete" }).className).toContain("danger");
  });

  it("cancels without confirming", async () => {
    const onConfirm = vi.fn();
    const onOpenChange = vi.fn();
    render(<ConfirmDialog defaultOpen onOpenChange={onOpenChange} title="Sure?" confirmLabel="Yes" onConfirm={onConfirm} />);
    fireEvent.click(screen.getByRole("button", { name: "Cancel" }));
    expect(onOpenChange).toHaveBeenCalledWith(false);
    expect(onConfirm).not.toHaveBeenCalled();
  });
});
