import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@fontsource-variable/inter";
import "@fontsource-variable/source-serif-4";
import { Gallery } from "./gallery/Gallery";
import "./app.css";
// the second brand's palette, generated from its brand.toml (see vite.config.ts)
import "virtual:brand-palettes.css";

const root = document.getElementById("root");
if (root === null) throw new Error("index.html has no #root");

createRoot(root).render(
  <StrictMode>
    <Gallery />
  </StrictMode>,
);
