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
		let utility = class.rsplit(':').next().expect("rsplit yields at least one piece");
		["h-", "min-h-", "size-", "min-w-"]
			.iter()
			.any(|axis| utility.strip_prefix(axis).is_some_and(|n| n.starts_with(|c: char| c.is_ascii_digit())))
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
