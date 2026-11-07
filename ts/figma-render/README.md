# `@evinvest/figma-render`

A site's pages, as its own Playwright suite renders them, serialized into Figma frames through
`generate_figma_design`'s capture script. Figma is a debug artifact here: nothing is read back. No Rust
twin.

```ts
// tests/e2e/figma.spec.ts — the config's projects are the breakpoints
import { test } from "@playwright/test";
import { defineFigmaSuite } from "@evinvest/figma-render";

defineFigmaSuite({ test }, ["/", "/usage/"], { hide: ["nextjs-portal"] });
```

```sh
evinvest-figma-render frames -c tests/e2e --file <fileKey>   # {"file", "frames": ["<project> <page>", …]}
FIGMA_CAPTURES='{"desktop /": {"captureId": …, "endpoint": …}}' evinvest-figma-render -c tests/e2e
```

A capture id is single-use and only the keys `FIGMA_CAPTURES` names are captured; the ids come from the
Figma MCP, which the `figma-render` skill drives. `playwright` is taken from `PATH`, so the caller's pinned
runner and browsers render the frames. `settle(page, selector)` is exported for screenshot suites that
want the same notion of a finished page.
