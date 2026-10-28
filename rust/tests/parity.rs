//! Both ports render the same HTML, and a component exists in both ports or
//! neither (docs/ARCHITECTURE.md, "The `uikit` library"). Every gallery label is
//! rendered through dioxus-ssr and Leptos SSR, and the two must agree once each
//! is reduced to a canonical tree. Each test reports every problem in one run, so
//! a porting agent finds their own labels among everybody else's.
//! `PARITY_RAW=1` adds both sides' raw HTML to each mismatch.
#![cfg(all(feature = "uikit_dioxus", feature = "uikit_leptos"))]

use std::{
	collections::{BTreeMap, HashMap},
	fmt::Write as _,
};

use scraper::{ElementRef, Html, node::Node as Parsed};

mod dioxus_side {
	include!("support/gallery.rs");
	include!("support/gallery_parity_only.rs");

	pub(super) fn render() -> Vec<(&'static str, String)> {
		GALLERY.iter().chain(PARITY_ONLY).map(|&(label, demo)| (label, settled(demo))).collect()
	}

	/// The Leptos port registers parts as they render, Dioxus parts often in
	/// effects. Those run a component at a time and a pass can leave the markup
	/// unchanged mid-chain, so run well past any demo's chain, then check one
	/// more pass changes nothing.
	fn settled(app: fn() -> Element) -> String {
		let mut dom = VirtualDom::new(app);
		dom.rebuild_in_place();
		let pass = |dom: &mut VirtualDom| {
			dom.process_events();
			dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
			dioxus_ssr::render(dom)
		};
		for _ in 0..32 {
			pass(&mut dom);
		}
		let html = pass(&mut dom);
		assert_eq!(pass(&mut dom), html, "never settled");
		html
	}
}

mod leptos_side {
	include!("support/gallery_leptos.rs");

	pub(super) fn render() -> Vec<(&'static str, String)> {
		GALLERY.iter().chain(PARITY_ONLY).map(|&(label, demo)| (label, Owner::new().with(|| demo().to_html()))).collect()
	}
}

/// HTML boolean attributes: present means on, whatever the value says.
const BOOLEAN: &[&str] = &[
	"allowfullscreen", "async", "autofocus", "autoplay", "checked", "controls", "default", "defer", "disabled", "formnovalidate", "hidden", "inert", "ismap", "itemscope", "loop",
	"multiple", "muted", "nomodule", "novalidate", "open", "playsinline", "readonly", "required", "reversed", "selected",
];

/// `aria-*` attributes whose value is a list of element ids.
const IDREFS: &[&str] = &[
	"aria-activedescendant",
	"aria-controls",
	"aria-describedby",
	"aria-details",
	"aria-errormessage",
	"aria-flowto",
	"aria-labelledby",
	"aria-owns",
];

#[test]
fn ports_render_the_same_html() {
	let dioxus = by_label("Dioxus", dioxus_side::render());
	let leptos = by_label("Leptos", leptos_side::render());
	let mut report = String::new();
	let missing = |from: &BTreeMap<&str, String>, has: &BTreeMap<&str, String>| has.keys().filter(|l| !from.contains_key(*l)).copied().collect::<Vec<_>>().join(", ");
	for (side, files, absent) in [
		("Leptos", "gallery_leptos.rs", missing(&leptos, &dioxus)),
		("Dioxus", "gallery.rs / gallery_parity_only.rs", missing(&dioxus, &leptos)),
	] {
		if !absent.is_empty() {
			writeln!(report, "missing in {side} ({files}): {absent}").unwrap();
		}
	}
	let raw = std::env::var_os("PARITY_RAW").is_some();
	let mut mismatched = Vec::new();
	for (label, d_html) in &dioxus {
		let Some(l_html) = leptos.get(label) else { continue };
		let (d, l) = (canonical(d_html), canonical(l_html));
		if d != l {
			mismatched.push(*label);
			writeln!(report, "\n── {label} ── (- dioxus, + leptos)\n{}", diff(&d, &l)).unwrap();
			if raw {
				writeln!(report, "dioxus: {d_html}\nleptos: {l_html}").unwrap();
			}
		}
	}
	if !mismatched.is_empty() {
		writeln!(report, "\nmismatching: {}", mismatched.join(", ")).unwrap();
	}
	assert!(report.is_empty(), "the ports disagree\n{report}");
}

#[test]
fn every_component_is_in_both_ports_and_the_gallery() {
	let dioxus = components("src/uikit_dioxus");
	let leptos = components("src/uikit_leptos");
	let dioxus_gallery = source("tests/support/gallery.rs") + &source("tests/support/gallery_parity_only.rs");
	let leptos_gallery = source("tests/support/gallery_leptos.rs");
	let list = |names: Vec<String>| names.join(", ");
	let mut report = String::new();
	for (problem, names) in [
		(
			"Dioxus only",
			list(dioxus.iter().filter(|(c, _)| !leptos.contains_key(*c)).map(|(c, m)| format!("{c} ({m})")).collect()),
		),
		(
			"Leptos only",
			list(leptos.iter().filter(|(c, _)| !dioxus.contains_key(*c)).map(|(c, m)| format!("{c} ({m})")).collect()),
		),
		(
			"in no Dioxus gallery demo (gallery.rs / gallery_parity_only.rs)",
			list(dioxus.keys().filter(|c| !used(&dioxus_gallery, c, |rest| rest.trim_start().starts_with('{'))).cloned().collect()),
		),
		(
			"in no Leptos gallery demo (gallery_leptos.rs)",
			list(
				leptos
					.keys()
					.filter(|c| !used(&leptos_gallery, &format!("<{c}"), |rest| rest.starts_with([' ', '\t', '\n', '/', '>'])))
					.cloned()
					.collect(),
			),
		),
	] {
		if !names.is_empty() {
			writeln!(report, "{problem}: {names}").unwrap();
		}
	}
	assert!(report.is_empty(), "component coverage\n{report}");
}

fn by_label(side: &str, rendered: Vec<(&'static str, String)>) -> BTreeMap<&'static str, String> {
	let mut map = BTreeMap::new();
	for (label, html) in rendered {
		assert!(map.insert(label, html).is_none(), "{side} gallery lists {label} twice");
	}
	map
}

// ── Coverage scan ──────────────────────────────────────────────────────────

fn source(rel: &str) -> String {
	let path = format!("{}/{rel}", env!("CARGO_MANIFEST_DIR"));
	std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// Every `#[component] pub fn` of a port, name → module. Private components and
/// test modules (everything after `#[cfg(test)]`) are not exported.
fn components(dir: &str) -> BTreeMap<String, String> {
	let dir = format!("{}/{dir}", env!("CARGO_MANIFEST_DIR"));
	let mut found = BTreeMap::new();
	for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {dir}: {e}")) {
		let path = entry.expect("read_dir entry").path();
		let module = path.file_stem().expect("read_dir yields named entries").to_string_lossy().into_owned();
		let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
		let mut pending = false;
		for line in text.lines().map(str::trim).take_while(|l| !l.starts_with("#[cfg(test)]")) {
			if line.starts_with("#[component") {
				pending = true;
			} else if pending && let Some(rest) = line.strip_prefix("pub fn ") {
				let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
				if let Some(other) = found.insert(name.clone(), module.clone()) {
					panic!("{name} is a component in both {other}.rs and {module}.rs");
				}
				pending = false;
			} else if pending && line.starts_with("fn ") {
				pending = false;
			}
		}
	}
	found
}

/// Whether `needle` occurs in `text` at a word start with `then` holding for what follows.
fn used(text: &str, needle: &str, then: impl Fn(&str) -> bool) -> bool {
	text.match_indices(needle).any(|(at, _)| {
		let word_start = text[..at].chars().next_back().is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
		word_start && then(&text[at + needle.len()..])
	})
}

// ── Canonical form ─────────────────────────────────────────────────────────

/// The parsed tree, one node or attribute (`@name`) per line, indented by depth: attributes sorted,
/// comments (Leptos's hydration markers among them) and whitespace-only text
/// dropped, adjacent text merged, spellings unified, ids renamed.
fn canonical(html: &str) -> Vec<String> {
	let mut lines = Vec::new();
	children(Html::parse_fragment(html).root_element(), 0, &mut HashMap::new(), &mut lines);
	lines
}

fn children(parent: ElementRef, depth: usize, ids: &mut HashMap<String, String>, lines: &mut Vec<String>) {
	let mut text = String::new();
	for child in parent.children() {
		match child.value() {
			Parsed::Text(t) => text.push_str(t),
			Parsed::Comment(_) => {}
			Parsed::Element(_) => {
				flush(&mut text, depth, lines);
				element(ElementRef::wrap(child).expect("matched an element"), depth, ids, lines);
			}
			other => panic!("unexpected node in a fragment: {other:?}"),
		}
	}
	flush(&mut text, depth, lines);
}

fn element(el: ElementRef, depth: usize, ids: &mut HashMap<String, String>, lines: &mut Vec<String>) {
	let indent = "  ".repeat(depth);
	lines.push(format!("{indent}<{}>", el.value().name()));
	let mut attrs: Vec<(&str, String)> = el.value().attrs().map(|(name, value)| (name, value.to_owned())).collect();
	attrs.sort();
	for (name, mut value) in attrs {
		if BOOLEAN.contains(&name) && (value.is_empty() || value == "true" || value == name) {
			value.clear();
		}
		if name == "style" {
			value = value.split(';').map(str::trim).filter(|decl| !decl.is_empty()).collect::<Vec<_>>().join("; "); // Leptos appends `;` to every style value
		}
		if name == "id" || name == "for" || IDREFS.contains(&name) {
			value = value.split_whitespace().map(|id| rename(ids, id)).collect::<Vec<_>>().join(" ");
		}
		lines.push(format!("{indent}  @{name}={value:?}"));
	}
	children(el, depth + 1, ids, lines);
}

fn flush(text: &mut String, depth: usize, lines: &mut Vec<String>) {
	if !text.trim().is_empty() {
		lines.push(format!("{}{text:?}", "  ".repeat(depth)));
	}
	text.clear();
}

fn rename(ids: &mut HashMap<String, String>, id: &str) -> String {
	let next = ids.len();
	ids.entry(id.to_owned()).or_insert_with(|| format!("id{next}")).clone()
}

/// Changed lines with two lines of context, each hunk headed by its enclosing element, from a
/// longest-common-subsequence alignment.
fn diff(a: &[String], b: &[String]) -> String {
	let mut lcs = vec![vec![0usize; b.len() + 1]; a.len() + 1];
	for i in (0..a.len()).rev() {
		for j in (0..b.len()).rev() {
			lcs[i][j] = if a[i] == b[j] { lcs[i + 1][j + 1] + 1 } else { lcs[i + 1][j].max(lcs[i][j + 1]) };
		}
	}
	let (mut i, mut j, mut ops) = (0, 0, Vec::new());
	while i < a.len() || j < b.len() {
		if i < a.len() && j < b.len() && a[i] == b[j] {
			ops.push((' ', &a[i]));
			(i, j) = (i + 1, j + 1);
		} else if i < a.len() && (j == b.len() || lcs[i + 1][j] >= lcs[i][j + 1]) {
			ops.push(('-', &a[i]));
			i += 1;
		} else {
			ops.push(('+', &b[j]));
			j += 1;
		}
	}
	let indent = |l: &str| l.len() - l.trim_start().len();
	let near_change = |k: usize| ops[k.saturating_sub(2)..(k + 3).min(ops.len())].iter().any(|(op, _)| *op != ' ');
	let mut out = String::new();
	let mut skipped = false;
	for (k, (op, line)) in ops.iter().enumerate() {
		if near_change(k) {
			if skipped {
				let enclosing = ops[..k]
					.iter()
					.rev()
					.map(|(_, l)| l.as_str())
					.find(|l| l.trim_start().starts_with('<') && indent(l) < indent(line));
				writeln!(out, "  … {}", enclosing.map_or("", str::trim_start)).unwrap(); // a top-level hunk has no enclosing element
			}
			skipped = false;
			writeln!(out, "{op} {line}").unwrap();
		} else {
			skipped = true;
		}
	}
	out
}
