import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { act, render, renderHook } from "@testing-library/react";
import { renderToString } from "react-dom/server";
import { AnimatedNumber } from "../src/components/animated-number";
import { useCountTween } from "../src/components/use-count-tween";
import { digitGlyphs, rollStep, splitNumber } from "../src/lib/number-parts";

function stubMotionPreference(reduce: boolean) {
  vi.stubGlobal("matchMedia", (query: string) => ({
    matches: reduce && query.includes("prefers-reduced-motion"),
    addEventListener: () => {},
    removeEventListener: () => {},
  }));
}

const parts = (root: HTMLElement) => [...root.querySelectorAll<HTMLElement>('[data-slot="animated-number-part"]')];
const digits = (root: HTMLElement) => [...root.querySelectorAll<HTMLElement>('[data-slot="animated-number-digit"]')];
const spokenText = (root: HTMLElement) => root.querySelector('[data-slot="animated-number-text"]')?.textContent;

beforeEach(() => stubMotionPreference(false));
afterEach(() => {
  vi.unstubAllGlobals();
  vi.useRealTimers();
});

describe("number parts", () => {
  const en = new Intl.NumberFormat("en", { style: "currency", currency: "USD" });
  const keys = (n: number) => splitNumber(en, n, digitGlyphs(en)).map((p) => p.key);

  it("keys integer digits by place value from the right", () => {
    expect(keys(1250)).toEqual(expect.arrayContaining(["int:3", "int:2", "int:1", "int:0", "group:2"]));
  });

  it("shares the keys of the places two values have in common", () => {
    const shared = keys(9).filter((k) => keys(1250).includes(k));
    expect(shared).toEqual(expect.arrayContaining(["int:0", "decimal", "frac:0", "frac:1", "currency:pre:0"]));
  });

  it("reads the digits of a non-Latin numbering system", () => {
    const ar = new Intl.NumberFormat("ar-EG");
    const found = splitNumber(ar, 305, digitGlyphs(ar)).flatMap((p) => (p.kind === "digit" ? [p.digit] : []));
    expect(found).toEqual([3, 0, 5]);
  });

  it("steps up, down, or the short way", () => {
    expect(rollStep(9, 0, 1)).toBe(1);
    expect(rollStep(0, 9, 1)).toBe(9);
    expect(rollStep(1, 2, -1)).toBe(-9);
    expect(rollStep(0, 9, 0)).toBe(-1);
    expect(rollStep(0, 5, 0)).toBe(5);
  });
});

describe("AnimatedNumber roll", () => {
  it("keeps the existing digit node when a leading digit is added", () => {
    const { container, rerender } = render(<AnimatedNumber value={9} />);
    const [ones] = parts(container);
    rerender(<AnimatedNumber value={10} />);
    expect(parts(container)).toHaveLength(2);
    expect(parts(container)[1]).toBe(ones);
  });

  it("opens an entering column after it mounts collapsed", () => {
    const { container, rerender } = render(<AnimatedNumber value={9} />);
    rerender(<AnimatedNumber value={10} />);
    expect(parts(container).map((p) => p.dataset["state"])).toEqual(["open", "open"]);
  });

  it("collapses a leaving column, then removes it", () => {
    vi.useFakeTimers();
    const { container, rerender } = render(<AnimatedNumber value={10} />);
    rerender(<AnimatedNumber value={9} />);
    expect(parts(container).map((p) => p.dataset["state"])).toEqual(["collapsed", "open"]);
    act(() => void vi.advanceTimersByTime(500));
    expect(parts(container)).toHaveLength(1);
  });

  it("spins up when the value rose, wrapping 9 to 0", () => {
    const { container, rerender } = render(<AnimatedNumber value={19} />);
    rerender(<AnimatedNumber value={20} />);
    expect(digits(container).map((d) => d.dataset["roll"])).toEqual(["up", "up"]);
  });

  it("spins down when the value fell", () => {
    const { container, rerender } = render(<AnimatedNumber value={20} />);
    rerender(<AnimatedNumber value={19} />);
    expect(digits(container).map((d) => d.dataset["roll"])).toEqual(["down", "down"]);
  });

  it("obeys a forced trend against the change", () => {
    const { container, rerender } = render(<AnimatedNumber value={1} trend={-1} />);
    rerender(<AnimatedNumber value={2} trend={-1} />);
    expect(digits(container)[0]?.dataset["roll"]).toBe("down");
  });

  it("lets each column take its short way with trend 0", () => {
    const { container, rerender } = render(<AnimatedNumber value={38} trend={0} />);
    rerender(<AnimatedNumber value={91} trend={0} />);
    expect(digits(container).map((d) => d.dataset["roll"])).toEqual(["down", "up"]);
  });

  it("goes back to rest when the transition ends", () => {
    const { container, rerender } = render(<AnimatedNumber value={1} />);
    rerender(<AnimatedNumber value={2} />);
    const strip = digits(container)[0]!.children[1]!;
    const end = Object.assign(new Event("transitionend", { bubbles: true }), { propertyName: "transform" });
    act(() => void strip.dispatchEvent(end));
    expect(digits(container)[0]?.dataset["roll"]).toBe("rest");
  });

  it("speaks the formatted final value once", () => {
    const format = { style: "currency", currency: "USD" } as const;
    const { container, rerender } = render(<AnimatedNumber value={1234.5} locales="en" format={format} />);
    expect(spokenText(container)).toBe("$1,234.50");
    rerender(<AnimatedNumber value={99} locales="en" format={format} />);
    expect(spokenText(container)).toBe("$99.00");
  });

  it("hides the moving glyphs from assistive tech", () => {
    const { container } = render(<AnimatedNumber value={7} />);
    expect(container.querySelector('[aria-hidden="true"] [data-slot="animated-number-digit"]')).not.toBeNull();
  });

  it("announces politely only when asked", () => {
    const { container, rerender } = render(<AnimatedNumber value={1} />);
    expect(container.querySelector("[aria-live]")).toBeNull();
    rerender(<AnimatedNumber value={1} live />);
    expect(container.querySelector("[aria-live]")).toHaveAttribute("aria-live", "polite");
  });

  it("swaps at once under reduced motion", () => {
    stubMotionPreference(true);
    const { container, rerender } = render(<AnimatedNumber value={19} />);
    rerender(<AnimatedNumber value={20} />);
    expect(spokenText(container)).toBe("20");
    expect(digits(container).map((d) => d.dataset["roll"])).toEqual(["rest", "rest"]);
    expect(container.innerHTML).not.toMatch(/transition: (?!none)/);
  });

  it("sets no spin transition on the first render", () => {
    const { container } = render(<AnimatedNumber value={42} />);
    expect(container.innerHTML).not.toContain("transform 750ms");
  });
});

describe("AnimatedNumber on the server", () => {
  it.each(["roll", "count"] as const)("prints the final value in %s mode, even with animateOnMount", (mode) => {
    const html = renderToString(
      <AnimatedNumber
        value={1234.5}
        mode={mode}
        animateOnMount
        from={0}
        locales="en"
        format={{ style: "currency", currency: "USD" }}
      />,
    );
    expect(html).toContain(">$1,234.50<");
    if (mode === "roll") expect(html.match(/data-roll="rest"/g)).toHaveLength(6);
    if (mode === "count") expect(html).toContain("<span>$1,234.50</span>");
  });
});

describe("useCountTween", () => {
  beforeEach(() => vi.useFakeTimers({ toFake: ["requestAnimationFrame", "cancelAnimationFrame", "performance"] }));

  it("returns the value itself before anything changes", () => {
    const { result } = renderHook(() => useCountTween(50));
    expect(result.current).toBe(50);
  });

  it("reaches the exact final value", () => {
    const { result, rerender } = renderHook(({ v }) => useCountTween(v, { duration: 1000 }), {
      initialProps: { v: 0 },
    });
    rerender({ v: 100.1 });
    act(() => void vi.advanceTimersByTime(500));
    expect(result.current).toBeGreaterThan(0);
    expect(result.current).toBeLessThan(100.1);
    act(() => void vi.advanceTimersByTime(600));
    expect(result.current).toBe(100.1);
  });

  it("restarts from what is on screen when the value changes mid-flight", () => {
    const { result, rerender } = renderHook(({ v }) => useCountTween(v, { duration: 1000 }), {
      initialProps: { v: 0 },
    });
    rerender({ v: 100 });
    act(() => void vi.advanceTimersByTime(400));
    const midway = result.current;
    rerender({ v: 0 });
    expect(result.current).toBe(midway);
    act(() => void vi.advanceTimersByTime(1100));
    expect(result.current).toBe(0);
  });

  it("cancels its frame on unmount", () => {
    const { rerender, unmount } = renderHook(({ v }) => useCountTween(v, { duration: 1000 }), {
      initialProps: { v: 0 },
    });
    rerender({ v: 100 });
    expect(vi.getTimerCount()).toBeGreaterThan(0);
    unmount();
    expect(vi.getTimerCount()).toBe(0);
  });

  it("counts up from `from` on mount when asked", () => {
    const { result } = renderHook(() => useCountTween(80, { duration: 1000, animateOnMount: true, from: 10 }));
    expect(result.current).toBe(10);
    act(() => void vi.advanceTimersByTime(1100));
    expect(result.current).toBe(80);
  });

  it("jumps under reduced motion", () => {
    stubMotionPreference(true);
    const { result, rerender } = renderHook(({ v }) => useCountTween(v), { initialProps: { v: 0 } });
    rerender({ v: 100 });
    expect(result.current).toBe(100);
    expect(vi.getTimerCount()).toBe(0);
  });
});
