import { useEffect, useState } from 'react';

import { containRemoteStyles, loadElement, REMOTE_LAYER } from '../index';

/** `loading` until the bundle answers; `failed` is final for this mount. */
export type BundleState = 'loading' | 'ready' | 'failed';

export interface ElementBundleOptions {
  minApiVersion?: number;
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
  // Keyed by tag: an instance switching remotes is un-ready for the new one at once.
  const [state, setState] = useState<{ tag: string; value: BundleState } | null>(null);

  useEffect(() => {
    let cancelled = false;
    // Set up before the bundle runs, and on the already-loaded path too: the
    // remote's sheet may be injected late or persist from an earlier mount.
    const stopContainment = layer === false ? () => {} : containRemoteStyles(scriptUrl, layer);
    const options = timeoutMs === undefined ? { minApiVersion } : { minApiVersion, timeoutMs };
    void loadElement(tag, scriptUrl, options).then(ok => {
      if (!cancelled) setState({ tag, value: ok ? 'ready' : 'failed' });
    });
    return () => {
      cancelled = true;
      stopContainment();
    };
  }, [tag, scriptUrl, minApiVersion, timeoutMs, layer]);

  return state?.tag === tag ? state.value : 'loading';
}
