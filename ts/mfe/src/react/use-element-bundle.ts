import { useEffect, useState } from 'react';

import { containRemoteStyles, loadElement, REMOTE_LAYER } from '../index';

/** `loading` until the bundle answers; `failed` is final for this mount. */
export type BundleState = 'loading' | 'ready' | 'failed';

export interface ElementBundleOptions {
  /** As in `LoadOptions`; default 0 accepts any bundle. */
  minApiVersion?: number;
  /** As in `LoadOptions`; default 8 s. */
  timeoutMs?: number;
  /** Cascade layer for the remote's stylesheets; `false` leaves them alone. */
  layer?: string | false;
}

/**
 * Load a remote's bundle once per tag and say whether its element is usable.
 * A 404, an older bundle and a timeout all read as `failed`, never an endless
 * `loading`, so the caller's fallback is final rather than a spinner.
 */
export function useElementBundle(
  tag: string,
  scriptUrl: string,
  { minApiVersion = 0, timeoutMs, layer = REMOTE_LAYER }: ElementBundleOptions = {},
): BundleState {
  // Keyed by everything that changes the answer: switching the remote, its URL
  // or the version floor reads as `loading` at once, not as the old verdict.
  const key = JSON.stringify([tag, scriptUrl, minApiVersion]);
  const [state, setState] = useState<{ key: string; value: BundleState } | null>(null);

  useEffect(() => {
    let cancelled = false;
    // Set up before the bundle runs, so the sheet it injects is caught as it lands.
    const stopContainment = layer === false ? () => {} : containRemoteStyles(scriptUrl, layer);
    const options = timeoutMs === undefined ? { minApiVersion } : { minApiVersion, timeoutMs };
    const settledKey = JSON.stringify([tag, scriptUrl, minApiVersion]);
    void loadElement(tag, scriptUrl, options).then(ok => {
      if (!cancelled) setState({ key: settledKey, value: ok ? 'ready' : 'failed' });
    });
    return () => {
      cancelled = true;
      stopContainment();
    };
  }, [tag, scriptUrl, minApiVersion, timeoutMs, layer]);

  return state?.key === key ? state.value : 'loading';
}
