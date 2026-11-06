//! Every `pub const` of `ev_lib_classes` is a TS export of the same name, in the
//! module named after its file — so this lists them instead of a hand-kept manifest.
//! Values come from the compiled crate: the scan only supplies names.

use std::{fmt::Write as _, fs, path::Path};

fn main() {
	let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../classes/src");
	println!("cargo::rerun-if-changed={}", src.display());
	let lib = fs::read_to_string(src.join("lib.rs")).expect("read classes lib.rs");

	let mut modules: Vec<String> = fs::read_dir(&src)
		.expect("read classes src")
		.map(|e| e.expect("dir entry").path())
		.filter(|p| p.extension().is_some_and(|e| e == "rs") && p.file_stem().is_some_and(|s| s != "lib"))
		.map(|p| p.file_stem().expect("has stem").to_string_lossy().into_owned())
		.collect();
	modules.sort();

	let mut out = String::from("pub fn consts() -> Vec<(&'static str, Vec<Ts>)> {\n\tvec![\n");
	for module in &modules {
		let path = src.join(format!("{module}.rs"));
		println!("cargo::rerun-if-changed={}", path.display());
		// A `pub mod` is reached through its own path; every private one is glob-re-exported at the root.
		let prefix = if lib.lines().any(|l| l.trim() == format!("pub mod {module};")) {
			format!("ev_lib_classes::{module}::")
		} else {
			"ev_lib_classes::".to_owned()
		};
		let mut exports = String::new();
		for (i, line) in fs::read_to_string(&path).expect("read classes module").lines().enumerate() {
			let at = || format!("{}:{}", path.display(), i + 1);
			assert!(!line.starts_with("pub static"), "{}: a `pub static` is not exported to TS; make it a `pub const`", at());
			let Some(decl) = line.strip_prefix("pub const ") else { continue };
			let (name, rest) = decl.split_once(':').unwrap_or_else(|| panic!("{}: expected `pub const NAME: TYPE = …`", at()));
			let ty = rest.split_once(" =").unwrap_or_else(|| panic!("{}: expected `pub const NAME: TYPE = …` on one line", at())).0.trim();
			assert!(name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'), "{}: `{name}` is not a SCREAMING_SNAKE const name", at());
			match ty {
				"&str" => writeln!(exports, "\t\t\tTs::Const {{ name: {name:?}, value: {prefix}{name} }},").unwrap(),
				"&'static str" => panic!("{}: write `&str`", at()),
				ty if ty.starts_with('&') => panic!("{}: `{ty}` is a reference this scan does not classify", at()),
				_ => writeln!(exports, "\t\t\tTs::Value {{ name: {name:?}, value: serde_json::to_value({prefix}{name}).expect(\"plain data\") }},").unwrap(),
			}
		}
		writeln!(out, "\t\t({:?}, vec![\n{exports}\t\t]),", module.replace('_', "-")).unwrap();
	}
	out.push_str("\t]\n}\n");
	fs::write(Path::new(&std::env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join("consts.rs"), out).expect("write consts.rs");
}
