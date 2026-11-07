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
evinvest-figma-render -c tests/e2e --file <fileKey>          # stdout: the new page's link
```

Without `frames` the bin is a client of Figma's MCP server and runs the whole render: a capture id per
frame (`generate_figma_design`), the suite with them, a poll per id for its frame, then one `use_figma` that
puts the frames on a page named `<short sha> <YYYY-MM-DD>[ dirty]`, names each `<project> <page>` and lays
them out a row per page, a column per project. That `use_figma` is the run's only call Figma counts toward
its rate limit. A frame still pending after the polls is a failed test: it is named, and the exit is 1.

The first run logs in to Figma in the browser (OAuth with PKCE, a callback on `127.0.0.1:47613`); the
client and its tokens are kept in `$XDG_STATE_HOME/evinvest-figma-render/oauth.json`. `playwright` is taken
from `PATH`, so the caller's pinned runner and browsers render the frames. `settle(page, selector)` is
exported for screenshot suites that want the same notion of a finished page.
