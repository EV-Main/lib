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

/// Every class the kit emits: the inventory (the tables and both ports' literals)
/// and the string literals of the hand-written TS components, as `(where, class)`.
fn classes() -> Vec<(String, String)> {
	let root = Path::new(ROOT);
	let inventory = fs::read_to_string(root.join("rust/classes/uikit-classes.txt")).expect("read the class inventory");
	let mut found: Vec<(String, String)> = inventory
		.lines()
		.skip(1)
		.flat_map(|line| line.split_whitespace().map(|c| ("uikit-classes.txt".to_owned(), c.to_owned())))
		.collect();
	let mut dirs = vec![root.join("ts/uikit/src/components")];
	while let Some(dir) = dirs.pop() {
		for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
			let path = entry.expect("dir entry").path();
			if path.is_dir() {
				dirs.push(path);
				continue;
			}
			let src = fs::read_to_string(&path).expect("read component");
			let at = path.strip_prefix(root).expect("walked from root").display().to_string();
			found.extend(src.split('"').skip(1).step_by(2).flat_map(|piece| piece.split_whitespace().map(|c| (at.clone(), c.to_owned()))));
		}
	}
	found
}

/// `classes()` that match, minus the exempt ones; fails if an exemption matches nothing.
fn literals(is_literal: impl Fn(&str) -> bool, exempt: &[(&str, &str)]) -> Vec<String> {
	let all = classes();
	for (class, _) in exempt {
		assert!(all.iter().any(|(_, c)| c == class), "{class} is exempt but no longer emitted");
	}
	all.into_iter()
		.filter(|(_, c)| is_literal(c) && !exempt.iter().any(|(e, _)| e == c))
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
	let offending = literals(literal, &[("rounded-tl-sm", "the navigation viewport's arrow tip, a shape and no surface")]);
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
			("border-accent-trace/40", OUTLINE_EDGE),
			("border-accent-debug/40", OUTLINE_EDGE),
			("border-accent-info/40", OUTLINE_EDGE),
			("border-accent-warn/40", OUTLINE_EDGE),
			("border-accent-error/40", OUTLINE_EDGE),
			("bg-primary-ink/20", MEASURED),
			("has-data-[state=checked]:bg-primary-ink/5", MEASURED),
			("aria-selected:bg-accent-info/20", FILLED_MARK),
			("data-[reset]:not-aria-selected:bg-accent-trace/20", FILLED_MARK),
			("bg-primary-ink/15", FILLED_MARK),
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
			("size-7", MEDIA),
			("size-8", MEDIA),
			("size-10", MEDIA),
			("[&_svg:not([class*='size-'])]:size-6", MEDIA),
			("size-9", "a calendar day cell and a chat attachment tile: a grid's cell and media"),
			("h-10", "the smart header bar and the status screen's mark"),
			("h-11", BAR),
			("h-6", "the typing indicator, a line of text"),
			("min-h-6", "the composer's field, a block's minimum line"),
		],
	);
	assert!(offending.is_empty(), "a control box at a rung's size, where its --control-h-* belongs: {offending:#?}");
}
