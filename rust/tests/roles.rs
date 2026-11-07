//! A class table names a role, never a magnitude (docs/spec/roles.md): a literal
//! where the theme has a role token is a missing token. One test per kind of
//! literal, over what the kit emits; an exemption says why its literal is no role.

use std::{collections::BTreeSet, fs, path::Path};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SIZES: [&str; 6] = ["xs", "sm", "md", "lg", "xl", "touch"];

/// Size tables whose steps are not a control's box on its own rung.
const IRREGULAR_SIZE_TABLES: [(&str, &str); 3] = [
	("textareaSizeClasses", "a minimum block height, not a control height"),
	("itemSizes", "a list row's density, padded all round, with no control box"),
	("sizeClasses", "the ramp itself"),
];

/// Every class the kit emits, as `(repo-relative path, class)`: the string
/// literals of the class tables, both ports and the hand-written TS, short of
/// whole-line comments and a Rust file's test module. `ts/uikit/src/generated`
/// mirrors `rust/classes` and would count each site twice.
fn classes() -> Vec<(String, String)> {
	let root = Path::new(ROOT);
	let mut dirs: Vec<_> = [
		"rust/classes/src",
		"rust/src/ui/uikit/core",
		"rust/src/ui/uikit/dioxus",
		"rust/src/ui/uikit/leptos",
		"ts/uikit/src",
	]
	.iter()
	.map(|dir| root.join(dir))
	.collect();
	let generated = root.join("ts/uikit/src/generated");
	let mut found = Vec::new();
	while let Some(dir) = dirs.pop() {
		for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
			let path = entry.expect("dir entry").path();
			if path.is_dir() {
				if path != generated {
					dirs.push(path);
				}
				continue;
			}
			if !path.extension().is_some_and(|e| e == "rs" || e == "ts" || e == "tsx") {
				continue;
			}
			let src = fs::read_to_string(&path).expect("read source");
			let lines: Vec<&str> = src.lines().collect();
			// `#[cfg(test)]` also gates items outside a test module (core/order.rs)
			let emitted = lines
				.windows(2)
				.position(|w| w[0].trim() == "#[cfg(test)]" && w[1].trim_start().starts_with("mod "))
				.map_or(&lines[..], |at| &lines[..at]);
			let code = emitted.iter().filter(|line| !line.trim_start().starts_with("//")).copied().collect::<Vec<_>>().join("\n");
			let at = path.strip_prefix(root).expect("walked from root").display().to_string();
			found.extend(code.split('"').skip(1).step_by(2).flat_map(|piece| piece.split_whitespace().map(|c| (at.clone(), c.to_owned()))));
		}
	}
	found
}

/// `classes()` that match, minus the ones exempt at their site; fails on an
/// exemption that matches nothing there.
fn literals(is_literal: impl Fn(&str) -> bool, exempt: &[(&str, &str, &str)]) -> Vec<String> {
	let all = classes();
	for (site, class, _) in exempt {
		assert!(all.iter().any(|(at, c)| at == site && c == class), "{class} is exempt at {site} but no longer emitted there");
	}
	all.into_iter()
		.filter(|(at, c)| is_literal(c) && !exempt.iter().any(|(site, e, _)| site == at && e == c))
		.map(|(at, c)| format!("{at}: {c}"))
		.collect::<BTreeSet<_>>()
		.into_iter()
		.collect()
}

/// The utility a class applies, without its variants.
fn utility(class: &str) -> &str {
	let mut depth = 0;
	let mut start = 0;
	for (i, c) in class.char_indices() {
		match c {
			'[' | '(' => depth += 1,
			']' | ')' => depth -= 1,
			':' if depth == 0 => start = i + 1,
			_ => {}
		}
	}
	class[start..].trim_start_matches('!')
}

/// Every generated TS table keyed by `Size` steps, as `(name, [(key, classes)])`.
fn size_tables() -> Vec<(String, Vec<(String, String)>)> {
	let dir = Path::new(ROOT).join("ts/uikit/src/generated");
	let mut tables = Vec::new();
	for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
		let src = fs::read_to_string(entry.expect("dir entry").path()).expect("read generated module");
		for block in src.split("export const ").skip(1) {
			let Some((name, body)) = block.split_once(" = {\n") else { continue };
			let body = body.split("\n} as const;").next().expect("split yields at least one piece");
			let entries: Vec<(String, String)> = body
				.lines()
				.map(|line| {
					let (key, value) = line.trim().split_once("\": \"").unwrap_or_else(|| panic!("{name}: not a table row: {line}"));
					(key.trim_start_matches('"').to_owned(), value.trim_end_matches("\",").to_owned())
				})
				.collect();
			if entries.iter().all(|(key, _)| SIZES.contains(&key.as_str())) {
				tables.push((name.to_owned(), entries));
			}
		}
	}
	assert!(tables.iter().any(|(name, _)| name == "sizeClasses"), "size tables not found in {}", dir.display());
	tables
}

/// A control's height is its `Size` step's `--control-box` and its inset the
/// step's `--control-pad`, so a step table carries neither of its own.
#[test]
fn size_tables_read_the_ramp() {
	let magnitude = |class: &str| {
		["h-", "min-h-", "size-", "min-w-", "px-", "gap-"]
			.iter()
			.any(|axis| utility(class).strip_prefix(axis).is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit())))
	};
	let tables = size_tables();
	for (exempt, _) in IRREGULAR_SIZE_TABLES {
		assert!(tables.iter().any(|(name, _)| name == exempt), "{exempt} is exempt but is no longer a size table");
	}
	let mut offending = Vec::new();
	for (name, entries) in tables.iter().filter(|(name, _)| !IRREGULAR_SIZE_TABLES.iter().any(|(exempt, _)| exempt == name)) {
		for (key, classes) in entries {
			offending.extend(classes.split_whitespace().filter(|c| magnitude(c)).map(|c| format!("{name}[{key}]: {c}")));
		}
	}
	assert!(
		offending.is_empty(),
		"a literal height or inset in a size table, where the step's --control-box or --control-pad belongs: {offending:#?}"
	);
}

/// A corner is `rounded-{control,panel,chip,row}` or the `full` shape.
#[test]
fn radius_is_a_role() {
	let literal = |class: &str| {
		let Some(rest) = utility(class).strip_prefix("rounded") else { return false };
		let value = ["-t", "-r", "-b", "-l", "-tl", "-tr", "-bl", "-br", "-s", "-e", "-ss", "-se", "-es", "-ee"]
			.iter()
			.find_map(|side| rest.strip_prefix(side).filter(|v| v.starts_with('-')))
			.unwrap_or(rest);
		["-xs", "-sm", "-md", "-lg", "-xl", "-2xl", "-3xl", "-4xl"].contains(&value) || (value.starts_with("-[") && value != "-[inherit]") || value.starts_with("-(")
	};
	let offending = literals(
		literal,
		&[(
			"rust/classes/src/navigation_menu.rs",
			"rounded-tl-sm",
			"the navigation viewport's arrow tip, a shape and no surface",
		)],
	);
	assert!(offending.is_empty(), "a radius literal, where rounded-{{control,panel,chip,row}} belongs: {offending:#?}");
}

/// A state on a surface is the ladder: `bg-hover` pointed at or open,
/// `bg-active` pressed, `bg-selected` chosen. A surface at an alpha, or `muted`
/// as a hover, is a state spelled by hand.
#[test]
fn state_is_the_ladder() {
	let literal = |class: &str| {
		let fill = utility(class);
		let surface = ["hover", "muted", "secondary", "card", "popover", "background", "active", "selected"]
			.iter()
			.any(|s| fill.strip_prefix("bg-").and_then(|f| f.strip_prefix(s)).is_some_and(|alpha| alpha.starts_with('/')));
		surface || (fill == "bg-muted" && class.contains("hover:"))
	};
	let offending = literals(
		literal,
		&[(
			"rust/classes/src/kbd.rs",
			"[[data-slot=tooltip-content]_&]:bg-background/20",
			"a key cap on the inverted tooltip face: a tint of it, no state",
		)],
	);
	assert!(offending.is_empty(), "a hand-spelled state fill, where bg-{{hover,active,selected}} belongs: {offending:#?}");
}

/// A tone worn as a tint is its `--tint-{tone}-{fill,edge}`, and a tone's
/// pointed-at row or face is it at `--tint-hover`; a tone at a numeric alpha
/// is a tint spelled by hand.
#[test]
fn a_tone_is_a_tint() {
	let literal = |class: &str| {
		let Some(color) = utility(class).strip_prefix("bg-").or_else(|| utility(class).strip_prefix("border-")) else {
			return false;
		};
		let Some((tone, alpha)) = color.split_once('/') else { return false };
		["accent-", "positive", "primary"].iter().any(|t| tone.starts_with(t)) && alpha.starts_with(|c: char| c.is_ascii_digit())
	};
	const OUTLINE_EDGE: &str = "an outlined control's edge, which a tint's (none in Oldschool) cannot be";
	const MEASURED: &str = "a shape whose ratio docs/spec/accents.md measures";
	const FILLED_MARK: &str = "a fill at rest in its tone, which u2's tint (an outline) cannot give";
	let offending = literals(
		literal,
		&[
			("rust/classes/src/accent.rs", "border-accent-trace/40", OUTLINE_EDGE),
			("rust/classes/src/accent.rs", "border-accent-debug/40", OUTLINE_EDGE),
			("rust/classes/src/accent.rs", "border-accent-info/40", OUTLINE_EDGE),
			("rust/classes/src/accent.rs", "border-accent-warn/40", OUTLINE_EDGE),
			("rust/classes/src/accent.rs", "border-accent-error/40", OUTLINE_EDGE),
			("rust/classes/src/progress.rs", "bg-primary-ink/20", MEASURED),
			("rust/classes/src/field.rs", "has-data-[state=checked]:bg-primary-ink/5", MEASURED),
			("rust/classes/src/select.rs", "aria-selected:bg-accent-info/20", FILLED_MARK),
			("rust/classes/src/select.rs", "data-[reset]:not-aria-selected:bg-accent-trace/20", FILLED_MARK),
			("ts/uikit/src/components/shell/nav-badge.tsx", "bg-primary-ink/15", FILLED_MARK),
		],
	);
	assert!(offending.is_empty(), "a tone at a numeric alpha, where its --tint-* belongs: {offending:#?}");
}

/// A disabled control fades to `--disabled-opacity`, one value for the kit.
#[test]
fn disabled_is_one_token() {
	let literal = |class: &str| {
		let fade = utility(class);
		class[..class.len() - fade.len()].contains("disabled") && fade.strip_prefix("opacity-").is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit()))
	};
	let offending = literals(literal, &[]);
	assert!(
		offending.is_empty(),
		"a disabled fade spelled by hand, where opacity-(--disabled-opacity) belongs: {offending:#?}"
	);
}

/// A control's box is a rung of the ramp, so a theme that moves `--control-h`
/// reaches it; a literal at a rung's size is a box the theme cannot reach.
#[test]
fn control_boxes_read_the_ramp() {
	let literal = |class: &str| {
		let Some(n) = ["h-", "min-h-", "size-"].iter().find_map(|axis| utility(class).strip_prefix(axis)) else {
			return false;
		};
		["6", "7", "8", "9", "10", "11", "12"].contains(&n)
	};
	const MEDIA: &str = "media (an avatar, an item's or an empty state's tile), sized to what it frames";
	const BAR: &str = "a bar (smart header, aside header, chat dock), sized with the frame";
	let offending = literals(
		literal,
		&[
			("rust/classes/src/chat_message.rs", "size-7", MEDIA),
			("rust/classes/src/avatar.rs", "size-8", MEDIA),
			("rust/classes/src/item.rs", "size-8", MEDIA),
			("rust/classes/src/item.rs", "size-10", MEDIA),
			("rust/classes/src/empty.rs", "size-10", MEDIA),
			("rust/classes/src/empty.rs", "[&_svg:not([class*='size-'])]:size-6", MEDIA),
			("rust/classes/src/chat_message.rs", "size-9", "a chat attachment tile, media"),
			("rust/classes/src/smart_header.rs", "h-10", "the smart header bar"),
			("rust/classes/src/status_screen.rs", "h-10", "the status screen's mark"),
			("rust/classes/src/smart_header.rs", "h-11", BAR),
			("rust/classes/src/aside.rs", "h-11", BAR),
			("ts/uikit/src/components/shell/chat-panel.tsx", "h-11", BAR),
			("rust/classes/src/chat_typing.rs", "h-6", "the typing indicator, a line of text"),
			("rust/classes/src/chat_composer.rs", "min-h-6", "the composer's field, a block's minimum line"),
		],
	);
	assert!(offending.is_empty(), "a control box at a rung's size, where its --control-h-* belongs: {offending:#?}");
}

/// A type step is a rung of the text or the display ramp, so a theme that moves
/// `--body-size` or `--display-base` reaches it.
#[test]
fn type_steps_read_the_ramps() {
	let sheet = fs::read_to_string(Path::new(ROOT).join("theme.css")).expect("read theme.css");
	let steps: Vec<&str> = sheet.lines().map(str::trim).filter(|line| line.starts_with("--text-")).collect();
	assert!(!steps.is_empty(), "no --text-* step in theme.css");
	let offending: Vec<&&str> = steps
		.iter()
		.filter(|line| {
			let value = line.split_once(':').expect("a declaration").1;
			!value.contains("var(--body-") && !value.contains("var(--display-")
		})
		.collect();
	assert!(offending.is_empty(), "a type step spelled by hand, where the text or display ramp belongs: {offending:#?}");
}
