// The board pages around gallery.rs's demos, `include!`d after it.
//
// Rendered to static HTML via dioxus-ssr, wrapped in a page that pulls the
// Tailwind v4 browser engine (compiles utilities in-browser) plus the crate's
// own tokens.css, so the emitted classes actually paint. Playwright screenshots
// the result for visual regression; a human opens dist/index.html as the board.

use std::path::Path;

/// The theme contract plus EV's palette; inlined into the Tailwind-processed
/// `<style>` so its `@theme`/`:root` tokens drive the utilities. The flattened
/// sheet a Rust consumer ships, not the repo-root source: `@tailwindcss/browser`
/// rejects relative `@import` outright (not a resolver gap — copying the file
/// next to the page does not help), and one rejected rule aborts the whole
/// compile, leaving every page unstyled.
const TOKENS: &str = ev_lib_classes::TOKENS_CSS;
const DIST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/visual/dist");
fn render_fragment(app: fn() -> Element) -> String {
	let mut dom = VirtualDom::new(app);
	dom.rebuild_in_place();
	dioxus_ssr::render(&dom)
}

fn head(title: &str) -> String {
	assert!(!TOKENS.contains("@import"), "the shipped tokens.css must stay flat — @tailwindcss/browser rejects @import");
	format!(
		r#"<!doctype html>
<html class="dark">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<script src="./tailwind.js"></script>
<style type="text/tailwindcss">
@import "tailwindcss";
{TOKENS}
</style>
<style>
  html, body {{ background: var(--background); color: var(--ink); }}
  /* A `transform` makes any `position: fixed` overlay descendant resolve
     against its cell, not the viewport — so open dropdowns/dialogs/tooltips
     stay boxed inside the component they belong to. */
  .stage {{ transform: translateZ(0); position: relative; }}
</style>
</head>"#
	)
}

/// One primitive, alone on a dark page — the unit Playwright screenshots.
fn standalone(title: &str, fragment: &str) -> String {
	// No `.stage` transform here: alone on the page a `position: fixed` overlay
	// should size against the viewport (full width, centred), not a collapsed
	// containing block. The transform trick is only for the board's cells.
	format!(
		"{}\n<body class=\"min-h-screen flex items-center justify-center p-10\">\n\
		 <div id=\"stage\">{fragment}</div>\n</body></html>",
		head(title)
	)
}

/// One labelled cell in the combined board.
fn cell(name: &str, fragment: &str) -> String {
	format!(
		"<section class=\"stage flex flex-col gap-3 rounded-lg border border-border bg-card/30 p-5\">\
		 <h3 class=\"text-xs font-medium uppercase tracking-wide text-ink-soft\">{name}</h3>\
		 <div class=\"flex flex-1 flex-wrap items-center gap-3\">{fragment}</div></section>"
	)
}

/// The Figma-like board: every primitive in one scrollable grid.
fn board(cells: &str) -> String {
	format!(
		"{}\n<body class=\"min-h-screen p-8\">\n\
		 <h1 class=\"mb-6 text-2xl font-semibold\">EV UIKit — Component Library</h1>\n\
		 <div class=\"grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3\">{cells}</div>\n\
		 </body></html>",
		head("EV UIKit — Components")
	)
}

fn slug(name: &str) -> String {
	name.to_lowercase().replace(' ', "-")
}

/// Render every primitive to `tests/visual/dist/`: one standalone page each (for
/// screenshots), a `manifest.json` the spec iterates, and the combined board.
fn write_dist() {
	// Cleared, not merged into: a primitive dropped from GALLERY otherwise keeps
	// its page here and its baseline forever. That is how `aspectratio` outlived
	// the component it screenshotted.
	if Path::new(DIST).exists() {
		std::fs::remove_dir_all(DIST).expect("clear dist dir");
	}
	std::fs::create_dir_all(DIST).expect("create dist dir");
	// The pages load Tailwind's browser engine from `./tailwind.js`, not a CDN:
	// the snapshot derivation builds with no network, and a CDN is one more input
	// that can move under a baseline. Pinned in flake.nix, handed over by path.
	let tailwind = std::env::var("TAILWIND_BROWSER_JS").expect("set by the devShell; enter it with `nix develop`");
	std::fs::copy(&tailwind, format!("{DIST}/tailwind.js")).expect("copy tailwind browser engine");
	let mut cells = String::new();
	let mut names = Vec::new();
	for (name, f) in GALLERY {
		let fragment = render_fragment(*f);
		assert!(!fragment.trim().is_empty(), "{name} rendered empty");
		std::fs::write(format!("{DIST}/{}.html", slug(name)), standalone(name, &fragment)).expect("write primitive page");
		cells.push_str(&cell(name, &fragment));
		names.push(format!("{:?}", slug(name)));
	}
	std::fs::write(format!("{DIST}/colorscheme.html"), colorscheme()).expect("write colorscheme page");
	std::fs::write(format!("{DIST}/index.html"), board(&cells)).expect("write board");
	names.extend(["\"colorscheme\"".into(), "\"index\"".into()]);
	std::fs::write(format!("{DIST}/manifest.json"), format!("[{}]", names.join(","))).expect("write manifest");
}

/// Typst's `lorem(n)`: the first `n` words of the passage, wrapping around.
fn lorem(n: usize) -> String {
	const PASSAGE: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et \
		dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo \
		consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
		Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
	PASSAGE.split_whitespace().cycle().take(n).collect::<Vec<_>>().join(" ")
}

// ── Colorscheme: a mini page in the palette, then every contract colour ─────

/// Every `--color-X: var(--X)` the contract maps, with the one value the
/// shipped sheet declares for `--X`.
fn palette() -> Vec<(&'static str, &'static str)> {
	let names: Vec<&str> = ev_lib_classes::THEME_CSS
		.lines()
		.filter_map(|l| l.trim().strip_prefix("--color-"))
		.filter(|l| l.contains(": var(--"))
		.map(|l| l.split_once(':').expect("filtered on ': var(--'").0)
		.collect();
	assert!(names.len() > 30, "contract colour list not found in THEME_CSS: {names:?}");
	names
		.into_iter()
		.map(|name| {
			let decl = format!("--{name}:");
			let values: Vec<&str> = TOKENS.lines().filter_map(|l| l.trim().strip_prefix(decl.as_str())).collect();
			// a second declaration would be a polarity or brand override this one swatch cannot show
			assert_eq!(values.len(), 1, "--{name} declared {} times in TOKENS", values.len());
			(name, values[0].trim().trim_end_matches(';'))
		})
		.collect()
}

fn d_hero() -> Element {
	let nav = lorem(4);
	let stats = [("text-primary-ink", "18.4%"), ("text-positive", "+9.1%"), ("text-accent-error-ink", "-2.7%")];
	let chart = [
		("bg-chart-1", "h-10"),
		("bg-chart-2", "h-6"),
		("bg-chart-3", "h-12"),
		("bg-chart-4", "h-4"),
		("bg-chart-5", "h-8"),
	];
	rsx! {
		div { class: "overflow-hidden rounded-xl border border-border bg-background",
			header { class: "flex items-center justify-between border-b border-border bg-secondary px-6 py-3 text-on-secondary",
				span { class: "flex items-center gap-2 font-semibold",
					span { class: "flex size-7 items-center justify-center rounded-md bg-brand text-xs font-bold", "EV" }
					{lorem(2)}
				}
				nav { class: "flex gap-5 text-sm text-ink-soft",
					for word in nav.split(' ') {
						a { {word} }
					}
				}
				Button { size: Size::Sm, {lorem(1)} }
			}
			div { class: "grid grid-cols-[3fr_2fr] gap-6 border-t-4 border-brand bg-card px-8 py-6",
				div { class: "flex flex-col gap-3",
					span { class: "text-xs font-medium uppercase tracking-wide text-primary-ink", {lorem(3)} }
					h1 { class: "font-display text-3xl text-ink", {lorem(6)} }
					p { class: "text-ink-mid", {lorem(24)} }
					div { class: "flex gap-3",
						Button { {lorem(2)} }
						Button { variant: ButtonVariant::Secondary, {lorem(2)} }
						Button { variant: ButtonVariant::Outline, {lorem(1)} }
					}
				}
				div { class: "flex flex-col gap-2 rounded-lg bg-popover p-2 shadow-(--shadow-overlay)",
					div { class: "rounded-sm px-2 py-1.5 text-sm text-ink", {lorem(2)} }
					div { class: "rounded-sm bg-hover px-2 py-1.5 text-sm text-ink", {lorem(3)} }
					div { class: "rounded-sm bg-accent-error/10 px-2 py-1.5 text-sm text-accent-error-ink", {lorem(1)} }
					div { class: "flex items-end gap-1.5 rounded-sm bg-muted px-2 pt-2",
						for (bg, h) in chart {
							div { class: "w-6 rounded-t-sm {bg} {h}" }
						}
					}
				}
			}
			div { class: "flex flex-col gap-4 px-8 py-5",
				div { class: "grid grid-cols-3 gap-4",
					for (ink, value) in stats {
						Card { class: "gap-1 py-4",
							CardContent {
								p { class: "text-sm text-ink-soft", {lorem(2)} }
								p { class: "text-2xl font-semibold {ink}", {value} }
							}
						}
					}
				}
				div { class: "flex flex-wrap items-center gap-2",
					for accent in [Accent::Trace, Accent::Debug, Accent::Info, Accent::Warn, Accent::Error] {
						Button { size: Size::Sm, accent, "{accent.as_ref()}" }
					}
					Badge { variant: BadgeVariant::Success, {lorem(1)} }
					Input { class: "ml-auto w-56", placeholder: lorem(3) }
					Input { class: "w-40 ring-3 ring-ring/50 border-ring", value: lorem(1) }
				}
			}
			footer { class: "flex justify-between border-t border-border bg-secondary px-6 py-3 text-xs text-ink-soft",
				span { {lorem(3)} }
				span { {lorem(5)} }
			}
		}
	}
}

/// Hero first, so the palette is judged in use before it is read as values.
fn colorscheme() -> String {
	let swatches: String = palette()
		.into_iter()
		.map(|(name, value)| {
			format!(
				"<div class=\"flex items-center gap-2\"><div class=\"h-5 w-12 shrink-0 rounded-sm border border-border\" style=\"background: var(--{name})\"></div>\
				 <span class=\"font-mono text-[11px] leading-tight\"><span class=\"text-ink\">{name}</span> <span class=\"text-ink-soft\">{value}</span></span></div>"
			)
		})
		.collect();
	format!(
		"{}\n<body class=\"flex flex-col gap-4 p-5\">\n{}\n<div class=\"grid grid-cols-4 gap-x-4 gap-y-1.5\">{swatches}</div>\n</body></html>",
		head("EV UIKit — Colorscheme"),
		render_fragment(d_hero),
	)
}
