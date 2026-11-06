//! The scope DAG of docs/ARCHITECTURE.md ("Scopes"): every `crate::<scope>` a
//! scope names, against the edges it may have.

use std::{
	collections::BTreeSet,
	path::{Path, PathBuf},
};

const SCOPES: [&str; 5] = ["codegen", "kernel", "ops", "product", "ui"];

fn allowed(from: &str, to: &str) -> bool {
	from == to || matches!((from, to), ("product" | "ops" | "ui", "kernel"))
}

#[test]
fn scopes_only_reach_down_the_dag() {
	let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
	let mut violations = Vec::new();
	for scope in SCOPES {
		for file in rust_files(&src.join(scope)) {
			let code: String = std::fs::read_to_string(&file)
				.unwrap_or_else(|e| panic!("read {}: {e}", file.display()))
				.lines()
				.filter(|l| !l.trim_start().starts_with("//"))
				.collect::<Vec<_>>()
				.join("\n");
			for target in named_scopes(&code) {
				if !allowed(scope, &target) {
					violations.push(format!("{} ({scope}) -> {target}", file.strip_prefix(&src).expect("under src").display()));
				}
			}
		}
	}
	assert!(violations.is_empty(), "edges against the scope DAG:\n{}", violations.join("\n"));
}

/// The scopes `code` reaches through `crate::`, including at the top of a `crate::{…}` group.
fn named_scopes(code: &str) -> BTreeSet<String> {
	let mut found = BTreeSet::new();
	for (at, _) in code.match_indices("crate::") {
		if code[..at].ends_with('$') {
			continue;
		}
		let rest = &code[at + "crate::".len()..];
		if let Some(group) = rest.strip_prefix('{') {
			let mut depth = 1;
			let mut item_start = true;
			let mut chars = group.char_indices();
			while let Some((i, c)) = chars.next() {
				match c {
					'{' => depth += 1,
					'}' => depth -= 1,
					',' if depth == 1 => item_start = true,
					c if depth == 1 && item_start && (c.is_alphabetic() || c == '_') => {
						found.insert(ident(&group[i..]).to_owned());
						item_start = false;
					}
					c if !c.is_whitespace() => item_start = false,
					_ => {}
				}
				if depth == 0 {
					break;
				}
			}
		} else {
			found.insert(ident(rest).to_owned());
		}
	}
	found.retain(|s| SCOPES.contains(&s.as_str()));
	found
}

fn ident(s: &str) -> &str {
	let end = s.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(s.len());
	&s[..end]
}

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
