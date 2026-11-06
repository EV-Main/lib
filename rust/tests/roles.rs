//! A class table names a role, never a magnitude (docs/spec/roles.md): a literal
//! where the theme has a role token is a missing token. One test per kind of
//! literal, over what the kit emits; an exemption says why its literal is no role.

use std::{fs, path::Path};

const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
const SIZES: [&str; 6] = ["xs", "sm", "md", "lg", "xl", "touch"];

/// Field heights map onto the ramp irregularly (a field's `lg` is the CTA's 48 px).
const IRREGULAR_SIZE_TABLES: [(&str, &str); 6] = [
	("inputSizeClasses", "field heights"),
	("selectTriggerSizeClasses", "field heights"),
	("nativeSelectSizeClasses", "field heights, nested inside a field"),
	("textareaSizeClasses", "a minimum block height, not a control height"),
	("sidebarMenuButtonSizeClasses", "a nav row, sized with its rail"),
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
		.collect::<std::collections::BTreeSet<_>>()
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
	assert!(tables.iter().any(|(name, _)| name == "buttonSizeClasses"), "size tables not found in {}", dir.display());
	tables
}

/// A control's height is its `Size` step's `--control-box`, so a step table
/// carries no height of its own.
#[test]
fn size_tables_read_the_ramp() {
	let magnitude = |class: &str| {
		["h-", "min-h-", "size-", "min-w-"]
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
	assert!(offending.is_empty(), "a literal height in a size table, where the step's --control-box belongs: {offending:#?}");
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
