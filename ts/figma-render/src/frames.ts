/** The describe every frame test sits in; the bin greps and lists by it. */
export const FIGMA_SUITE = "@figma";

interface ListSuite {
  title: string;
  specs: { title: string; tests: { projectName: string }[] }[];
  suites?: ListSuite[];
}

/** The `<project> <page>` keys of `playwright test --list --reporter=json`, in its order. */
export function frames(list: { suites: ListSuite[]; errors: unknown[] }): string[] {
  if (list.errors.length > 0) throw new Error(`playwright --list: ${JSON.stringify(list.errors)}`);
  const walk = (suites: ListSuite[]): string[] =>
    suites.flatMap(s => [
      ...(s.title === FIGMA_SUITE ? s.specs.flatMap(spec => spec.tests.map(t => `${t.projectName} ${spec.title}`)) : []),
      ...walk(s.suites ?? []),
    ]);
  return walk(list.suites);
}
