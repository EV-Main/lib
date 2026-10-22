import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { AccountList, AccountListItem } from "../src/components/account-list";

function setup(onValueChange = vi.fn(), defaultValue = "personal") {
  render(
    <AccountList aria-label="Accounts" defaultValue={defaultValue} onValueChange={onValueChange}>
      <AccountListItem value="personal" name="JD" handle="jane.doe" badges={<span>Owner</span>} />
      <AccountListItem value="team" name="Acme" actions={<button type="button">More</button>} />
      <AccountListItem value="old" name="Legacy" disabled />
    </AccountList>,
  );
  return onValueChange;
}

describe("AccountList", () => {
  it("is a named radio group with the current account checked and tabbable", () => {
    setup();
    expect(screen.getByRole("radiogroup", { name: "Accounts" })).toBeInTheDocument();
    const radios = screen.getAllByRole("radio");
    expect(radios).toHaveLength(3);
    expect(screen.getByRole("radio", { name: /JD/ })).toHaveAttribute("aria-checked", "true");
    expect(screen.getByRole("radio", { name: /JD/ })).toHaveAttribute("tabindex", "0");
    expect(screen.getByRole("radio", { name: /Acme/ })).toHaveAttribute("tabindex", "-1");
  });

  it("selects on click", () => {
    const onValueChange = setup();
    fireEvent.click(screen.getByRole("radio", { name: /Acme/ }));
    expect(onValueChange).toHaveBeenCalledWith("team");
    expect(screen.getByRole("radio", { name: /Acme/ })).toHaveAttribute("aria-checked", "true");
  });

  it("moves and selects with the arrow keys, skipping disabled items and wrapping", () => {
    const onValueChange = setup();
    const first = screen.getByRole("radio", { name: /JD/ });
    first.focus();
    fireEvent.keyDown(first, { key: "ArrowDown" });
    expect(screen.getByRole("radio", { name: /Acme/ })).toHaveFocus();
    expect(onValueChange).toHaveBeenLastCalledWith("team");
    fireEvent.keyDown(screen.getByRole("radio", { name: /Acme/ }), { key: "ArrowDown" });
    expect(screen.getByRole("radio", { name: /JD/ })).toHaveFocus();
  });

  it("keeps row actions outside the radio", () => {
    setup();
    expect(screen.getByRole("button", { name: "More" }).closest('[role="radio"]')).toBeNull();
  });

  it("makes every radio tabbable while nothing is chosen", () => {
    setup(vi.fn(), "");
    expect(screen.getByRole("radio", { name: /JD/ })).toHaveAttribute("tabindex", "0");
    expect(screen.getByRole("radio", { name: /Acme/ })).toHaveAttribute("tabindex", "0");
  });
});
