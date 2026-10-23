"use client";

import * as React from "react";

type LayerRef = React.RefObject<HTMLDivElement | null>;

interface LayerInfo {
  /** The layer this one was rendered inside, from context — not the one below it on the stack. */
  parent: LayerRef | null;
  modal: boolean;
}

interface LayerStore {
  // The open layers, bottom first. An overlay opened inside another (a Select
  // in a Dialog, a Popover in a Drawer) is portaled out of its parent's
  // subtree, so "outside the parent" would include it: the stack is what tells
  // a nested layer from the rest of the page.
  stack: LayerRef[];
  info: WeakMap<LayerRef, LayerInfo>;
  // The stack as it stood when an event started its trip. By the time the
  // document hears an Escape, a React `onKeyDown` on the way (a Select's, a
  // consumer's) may already have closed the top layer and — a discrete update
  // commits synchronously — popped it, so the layer below would find itself
  // on top and close on the same key; a pointer-down can close an upper modal
  // the same way and lift its barrier mid-flight. Taken in the window's
  // capture phase, which runs before any DOM listener, React's root included.
  seen: WeakMap<Event, LayerRef[]>;
  snapshot: (event: Event) => void;
}

// One stack per page, not per module copy: a micro-frontend that bundles its
// own uikit must see the host's layers, and the host its. Versioned, so a copy
// with a different store shape keeps to itself instead of misreading this one.
const STORE_KEY = Symbol.for("@evinvest/uikit/dismissable-layers@1");

function isStore(value: unknown): value is LayerStore {
  return typeof value === "object" && value !== null && "stack" in value && Array.isArray(value.stack);
}

function sharedStore(): LayerStore {
  const existing: unknown = Reflect.get(globalThis, STORE_KEY);
  if (isStore(existing)) return existing;
  const store: LayerStore = {
    stack: [],
    info: new WeakMap(),
    seen: new WeakMap(),
    snapshot: (event) => {
      if (!store.seen.has(event)) store.seen.set(event, store.stack.slice());
    },
  };
  Reflect.set(globalThis, STORE_KEY, store);
  return store;
}

const store = sharedStore();

/**
 * Elements carrying this attribute are outside every layer without being a
 * click away from any: a press there dismisses nothing (the toaster).
 */
export const LAYER_IGNORE_ATTR = "data-dismissable-layer-ignore";

function layersAt(event: Event): LayerRef[] {
  // An event dispatched where the window never sees it falls back to the live stack.
  return store.seen.get(event) ?? store.stack;
}

function isPointerOutside(own: LayerRef, event: PointerEvent): boolean {
  const target = event.target;
  if (!(target instanceof Node)) return false;
  const el = target instanceof Element ? target : target.parentElement;
  if (el?.closest(`[${LAYER_IGNORE_ATTR}]`)) return false;
  const layers = layersAt(event);
  const index = layers.indexOf(own);
  // Not on the stack (a listener outliving its cleanup): nothing is above it.
  if (index < 0) return true;
  const above = layers.slice(index + 1);
  // A modal above is a barrier: whatever the press landed on — its scrim, its
  // panel, a toast over it — the modal answers for it, not the layers below.
  if (above.some((layer) => store.info.get(layer)?.modal === true)) return false;
  return !above.some((layer) => layer.current?.contains(target) ?? false);
}

function ownsEscape(own: LayerRef, event: KeyboardEvent): boolean {
  const layers = layersAt(event);
  return layers[layers.length - 1] === own;
}

function isDescendant(layer: LayerRef, ancestor: LayerRef): boolean {
  for (let p = store.info.get(layer)?.parent ?? null; p; p = store.info.get(p)?.parent ?? null) {
    if (p === ancestor) return true;
  }
  return false;
}

function push(layer: LayerRef) {
  const { stack } = store;
  if (stack.length === 0) {
    window.addEventListener("keydown", store.snapshot, true);
    window.addEventListener("pointerdown", store.snapshot, true);
  }
  // Children's effects run before their parent's, so a layer opened in the
  // same commit as one nested in it (`defaultOpen` on both, a shared `open`)
  // arrives second: it goes below its first descendant, not on top.
  const firstChild = stack.findIndex((other) => isDescendant(other, layer));
  if (firstChild < 0) stack.push(layer);
  else stack.splice(firstChild, 0, layer);
}

function pop(layer: LayerRef) {
  const { stack } = store;
  const index = stack.indexOf(layer);
  if (index >= 0) stack.splice(index, 1);
  if (stack.length === 0) {
    window.removeEventListener("keydown", store.snapshot, true);
    window.removeEventListener("pointerdown", store.snapshot, true);
  }
}

interface ParentLayer {
  layer: LayerRef;
}

const ParentLayerContext = React.createContext<ParentLayer | null>(null);

// Whether each layer asked for `modal`, known from its first render. The stack
// learns it only in an effect, too late for the scope below a layer opened in
// the same commit to tell its children that a modal is under them.
const renderedModal = new WeakMap<LayerRef, boolean>();

// Layers waiting for their parent's node: a child's layout effect runs before
// the parent's ref is attached when both open in one commit.
const waitingForParent = new WeakMap<LayerRef, Set<LayerRef>>();

/**
 * Children's portals mount before their parent's when both open in one
 * commit, so with equal `z-index` the parent paints over the child and its
 * scrim. Lifts `child` (and the scrim rendered just before it) over `parent`
 * only in that case, so overlays that do not share a tree keep the stacking
 * their classes give them. Returns false while a node is still missing.
 */
function liftAbove(child: LayerRef, parent: LayerRef): boolean {
  const node = child.current;
  const parentNode = parent.current;
  if (!node || !parentNode) return false;
  if (!(parentNode.compareDocumentPosition(node) & Node.DOCUMENT_POSITION_PRECEDING)) return true;
  const view = node.ownerDocument.defaultView;
  if (!view) return true;
  const parentZ = Number.parseInt(view.getComputedStyle(parentNode).zIndex, 10);
  const ownZ = Number.parseInt(view.getComputedStyle(node).zIndex, 10);
  if (Number.isNaN(parentZ) || Number.isNaN(ownZ) || ownZ > parentZ) return true;
  node.style.zIndex = String(parentZ + 2);
  const scrim = node.previousElementSibling;
  // Only a modal renders its own scrim just before its panel; for anything
  // else that sibling belongs to someone else.
  if (renderedModal.get(child) && scrim instanceof HTMLElement && scrim.dataset["slot"]?.endsWith("overlay")) {
    scrim.style.zIndex = String(parentZ + 1);
  }
  return true;
}

/**
 * Marks `children` as rendered inside `layer` (the ref `useDismissableLayer`
 * returned), so a layer among them always stacks above it — even when both
 * open in the same commit. Wrap the overlay's content in it.
 */
export function DismissableLayerScope({ layer, children }: { layer: LayerRef; children?: React.ReactNode }) {
  const value = React.useMemo<ParentLayer>(() => ({ layer }), [layer]);
  return <ParentLayerContext.Provider value={value}>{children}</ParentLayerContext.Provider>;
}

/** Why a layer is being dismissed: the event itself, for a caller that treats keys and pointers apart. */
export type DismissEvent = PointerEvent | KeyboardEvent;

/**
 * Calls `onDismiss` on Escape or on a pointer-down outside the layer — the
 * dep-light core of every overlay's "click away to close". Pass refs to any
 * nodes that should NOT count as outside (e.g. the trigger), via `exclude`.
 *
 * Layers stack: a layer rendered inside another's `DismissableLayerScope`
 * sits above it, the rest in the order they opened. Escape dismisses only the
 * topmost layer, and a pointer-down inside a layer above this one is not
 * outside it — so a Select's list inside a Dialog closes the list, not the
 * Dialog. The Escape goes to the layer that was on top when the key was
 * pressed, even if a handler on its way closed that layer first.
 *
 * `modal: true` makes the layer a barrier: while it is open, the layers below
 * ignore every pointer-down — a press on its scrim closes it alone. Elements
 * marked with `LAYER_IGNORE_ATTR` (the toaster) are outside no layer.
 *
 * `pointerOutside: false` keeps the layer on the stack (it takes Escape, and
 * the layers below see a pointer-down inside it as inside) but leaves outside
 * clicks to the caller — for an overlay whose own scrim is what "outside"
 * means (Drawer, CommandDialog).
 *
 * The stack is shared through `globalThis`, so every copy of this module on
 * the page sees one stack; parent links need one React context, so they hold
 * within a copy only.
 *
 * Mirrors the dismiss behaviour Rust expresses with a full-screen backdrop.
 */
export function useDismissableLayer(opts: {
  enabled: boolean;
  onDismiss: (event: DismissEvent) => void;
  exclude?: Array<React.RefObject<Element | null>>;
  /** Dismiss on a pointer-down outside the layer. Default `true`. */
  pointerOutside?: boolean;
  /** Bar the layers below from pointer-downs while open. Default `false`. */
  modal?: boolean;
}): React.RefObject<HTMLDivElement | null> {
  const { enabled, onDismiss, exclude, pointerOutside = true, modal = false } = opts;
  const parent = React.useContext(ParentLayerContext)?.layer ?? null;
  const ref = React.useRef<HTMLDivElement | null>(null);
  // Idempotent per ref, so safe in render; see `renderedModal`.
  renderedModal.set(ref, modal);
  const onDismissRef = React.useRef(onDismiss);
  // Callers pass `exclude` inline, a new array each render: read through a ref,
  // or the effect would re-run and move this layer to the top of the stack.
  const excludeRef = React.useRef(exclude);
  React.useEffect(() => {
    onDismissRef.current = onDismiss;
    excludeRef.current = exclude;
  });

  React.useLayoutEffect(() => {
    if (!enabled) return;
    // Late children first (they ran before this layer's ref existed), then this
    // layer against its own parent.
    const waiting = waitingForParent.get(ref);
    if (waiting) {
      waitingForParent.delete(ref);
      for (const child of waiting) liftAbove(child, ref);
    }
    if (parent && !liftAbove(ref, parent)) {
      const siblings = waitingForParent.get(parent) ?? new Set<LayerRef>();
      siblings.add(ref);
      waitingForParent.set(parent, siblings);
    }
    return () => {
      if (parent) waitingForParent.get(parent)?.delete(ref);
    };
  }, [enabled, parent]);

  React.useEffect(() => {
    if (!enabled) return;
    store.info.set(ref, { parent, modal });
    push(ref);

    function onPointerDown(event: PointerEvent) {
      const target = event.target;
      if (!(target instanceof Node)) return;
      if (ref.current && ref.current.contains(target)) return;
      for (const r of excludeRef.current ?? []) {
        if (r.current && r.current.contains(target)) return;
      }
      if (isPointerOutside(ref, event)) onDismissRef.current(event);
    }

    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape" && ownsEscape(ref, event)) onDismissRef.current(event);
    }

    if (pointerOutside) document.addEventListener("pointerdown", onPointerDown, true);
    document.addEventListener("keydown", onKeyDown);
    return () => {
      pop(ref);
      document.removeEventListener("pointerdown", onPointerDown, true);
      document.removeEventListener("keydown", onKeyDown);
    };
  }, [enabled, pointerOutside, modal, parent]);

  return ref;
}
