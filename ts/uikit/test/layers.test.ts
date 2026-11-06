import { readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { describe, expect, it } from "vitest";

// A component renders only what its layer may (docs/ARCHITECTURE.md, "The
// `uikit` library"): atomic renders no component, derived only atomic ones,
// shell anything. Only rendering counts; reusing another module's classes
// (`buttonVariants`) is the shared atomic language.
const COMPONENTS = join(import.meta.dirname, "../src/components");
const LAYERS = ["atomic", "derived", "shell"] as const;

const IMPORT = /import\s*\{([^}]*)\}\s*from\s*"(\.[^"]+)"/g;

const modules = LAYERS.flatMap(layer =>
  readdirSync(join(COMPONENTS, layer)).map(file => {
    const path = join(COMPONENTS, layer, file);
    const code = readFileSync(path, "utf8");
    // local name → the module it is imported from
    const imported = [...code.matchAll(IMPORT)].flatMap(([, names = "", spec = ""]) =>
      names.split(",").map(n => [n.trim().replace(/^type\s+/, "").split(/\s+as\s+/).at(-1) ?? "", resolve(join(COMPONENTS, layer), spec)] as const),
    );
    return { name: `${layer}/${file.replace(/\.tsx?$/, "")}`, layer, path: path.replace(/\.tsx?$/, ""), code, imported };
  }),
);

describe("component layers", () => {
  it("renders only lower layers", () => {
    const violations = modules.flatMap(own =>
      own.imported.flatMap(([local, from]) => {
        const other = modules.find(m => m.path === from);
        if (local === "") return [];
        const allowed = own.layer === "shell" || (own.layer === "derived" && other?.layer === "atomic");
        if (other === undefined || allowed) return [];
        return new RegExp(`<${local}[\\s/>]`).test(own.code) ? [`${own.name} renders ${local} (${other.name})`] : [];
      }),
    );
    expect(violations).toEqual([]);
  });
});
