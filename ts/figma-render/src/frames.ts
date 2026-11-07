/** The describe every frame test sits in; the bin greps and lists by it. */
export const FIGMA_SUITE = "@figma";

interface ListSuite {
  title: string;
  specs: { title: string; tests: { projectName: string }[] }[];
  suites?: ListSuite[];
}

export interface Frame {
  project: string;
  page: string;
}

/** A frame's key in `FIGMA_CAPTURES`, `frames`' output and its Figma name. */
export const frameKey = (f: Frame) => `${f.project} ${f.page}`;

/** The `@figma` frames of `playwright test --list --reporter=json`, in its order. */
export function frames(list: { suites: ListSuite[]; errors: unknown[] }): Frame[] {
  if (list.errors.length > 0) throw new Error(`playwright --list: ${JSON.stringify(list.errors)}`);
  const walk = (suites: ListSuite[]): Frame[] =>
    suites.flatMap(s => [
      ...(s.title === FIGMA_SUITE ? s.specs.flatMap(spec => spec.tests.map(t => ({ project: t.projectName, page: spec.title }))) : []),
      ...walk(s.suites ?? []),
    ]);
  return walk(list.suites);
}
