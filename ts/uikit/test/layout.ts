// jsdom lays nothing out, so every offsetWidth is 0 and a visibility check
// (focus-scope's Tab stops) would see no control at all.
for (const prop of ["offsetWidth", "offsetHeight"] as const) {
  Object.defineProperty(HTMLElement.prototype, prop, {
    configurable: true,
    get(this: HTMLElement) {
      if (!this.isConnected) return 0;
      for (let el: Element | null = this; el; el = el.parentElement) {
        if (getComputedStyle(el).display === "none") return 0;
      }
      return 1;
    },
  });
}
