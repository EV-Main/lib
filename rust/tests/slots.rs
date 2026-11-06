//! `data-slot` names come from `ev_lib_classes::Slot` (docs/ARCHITECTURE.md,
//! "The `uikit` library"): no port writes one as a literal, and no class string
//! selects one that `Slot` lacks.
#![cfg(feature = "uikit_dioxus")]

use std::{
	collections::BTreeSet,
	path::{Path, PathBuf},
};

use ev_lib_classes::Slot;
use strum::IntoEnumIterator;

fn rust_files(dir: &Path) -> Vec<PathBuf> {
	let mut found = Vec::new();
	for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
		let path = entry.expect("read_dir entry").path();
		if path.is_dir() {
			found.extend(rust_files(&path));
		} else if path.extension().is_some_and(|e| e == "rs") {
			found.push(path);
		}
	}
	found
}

fn read(file: &Path) -> String {
	std::fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()))
}

#[test]
fn ports_name_slots_through_the_enum() {
	let ports = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/ui/uikit");
	let mut violations = Vec::new();
	for port in ["dioxus", "leptos"] {
		for file in rust_files(&ports.join(port)) {
			for (n, line) in read(&file).lines().enumerate() {
				if !line.trim_start().starts_with("//") && (line.contains("\"data-slot\": \"") || line.contains("data-slot=\"")) {
					violations.push(format!("{}:{}", file.strip_prefix(&ports).expect("under uikit").display(), n + 1));
				}
			}
		}
	}
	assert!(violations.is_empty(), "literal data-slot, use `Slot`:\n{}", violations.join("\n"));
}

#[test]
fn class_strings_select_only_known_slots() {
	let known: BTreeSet<&str> = Slot::iter().map(Slot::as_str).collect();
	let mut unknown = Vec::new();
	for file in rust_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("classes/src")) {
		for (n, line) in read(&file).lines().enumerate() {
			if line.trim_start().starts_with("//") {
				continue;
			}
			for (at, _) in line.match_indices("[data-slot=") {
				let rest = &line[at + "[data-slot=".len()..];
				let name = &rest[..rest.find(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')).unwrap_or(rest.len())];
				if !known.contains(name) {
					unknown.push(format!("{}:{}: {name}", file.file_name().expect("file").to_string_lossy(), n + 1));
				}
			}
		}
	}
	assert!(unknown.is_empty(), "selector names no `Slot`:\n{}", unknown.join("\n"));
}
