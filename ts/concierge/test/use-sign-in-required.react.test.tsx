// useSignInRequired opens the sign-in dialog on demand or on a 401, and on
// nothing else: a 403 means "signed in, not allowed" and must not prompt again.
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import { useSignInRequired, type SignInRequired, type SignInRequiredOptions } from '../src/react/index';

let container: HTMLDivElement;
let root: Root;
let latest: SignInRequired | null = null;

function Probe({ options }: { options?: SignInRequiredOptions }) {
  latest = useSignInRequired(options);
  return null;
}

beforeEach(async () => {
  latest = null;
  container = document.createElement('div');
  document.body.appendChild(container);
  root = createRoot(container);
});

afterEach(() => {
  act(() => root.unmount());
  container.remove();
});

async function mount(options?: SignInRequiredOptions): Promise<void> {
  await act(async () => {
    root.render(<Probe {...(options === undefined ? {} : { options })} />);
  });
}

function current(): SignInRequired {
  if (latest === null) throw new Error('Probe has not rendered');
  return latest;
}

describe('useSignInRequired', () => {
  it('should_start_closed', async () => {
    await mount();

    expect(current().open).toBe(false);
    expect(current().dialogProps.open).toBe(false);
  });

  it('should_open_on_requireSignIn', async () => {
    await mount();

    await act(async () => current().requireSignIn());

    expect(current().open).toBe(true);
  });

  it('should_open_and_report_true_for_a_401', async () => {
    await mount();
    let handled: boolean | undefined;

    await act(async () => {
      handled = current().guardResponse({ status: 401 });
    });

    expect(handled).toBe(true);
    expect(current().open).toBe(true);
  });

  it.each([200, 400, 403, 404, 419, 500])('should_stay_closed_and_report_false_for_%i', async status => {
    await mount();
    let handled: boolean | undefined;

    await act(async () => {
      handled = current().guardResponse({ status });
    });

    expect(handled).toBe(false);
    expect(current().open).toBe(false);
  });

  it('should_close_through_dialogProps_onOpenChange', async () => {
    await mount();
    await act(async () => current().requireSignIn());

    await act(async () => current().dialogProps.onOpenChange(false));

    expect(current().open).toBe(false);
  });

  it('should_pass_the_given_onSignedIn_to_the_dialog', async () => {
    const onSignedIn = vi.fn<() => void>();
    await mount({ onSignedIn });

    current().dialogProps.onSignedIn();

    expect(onSignedIn).toHaveBeenCalledTimes(1);
  });
});
