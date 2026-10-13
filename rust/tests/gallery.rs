//! Visual regression of the uikit: every primitive (all variants), the
//! combined board and the colorscheme page, rendered via dioxus-ssr, captured by
//! Playwright and compared byte for byte against `tests/visual/__screenshots__/`.
//! `UPDATE_EXPECT=1` accepts the current render instead. See tests/visual/README.md.
#![cfg(feature = "uikit")]

include!("support/gallery.rs");

use std::{collections::BTreeMap, path::PathBuf, process::Command};

const VISUAL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/visual");

#[test]
fn matches_baselines() {
	write_dist();
	let shots = PathBuf::from(format!("{VISUAL}/__screenshots__"));
	let captured = pngs(&capture());
	assert!(!captured.is_empty(), "playwright captured nothing");

	if std::env::var_os("UPDATE_EXPECT").is_some() {
		// cleared, not merged into: a page dropped from the manifest must not leave its baseline behind
		for path in pngs(&shots).into_values() {
			std::fs::remove_file(path).expect("remove stale baseline");
		}
		for (name, path) in &captured {
			std::fs::copy(path, shots.join(name)).expect("write baseline");
		}
		return;
	}

	let baselines = pngs(&shots);
	let read = |p: &PathBuf| std::fs::read(p).expect("listed by read_dir");
	let mismatched: Vec<&String> = captured
		.iter()
		.filter(|(name, path)| baselines.get(*name).is_none_or(|b| read(b) != read(path)))
		.map(|(name, _)| name)
		.collect();
	let orphaned: Vec<&String> = baselines.keys().filter(|name| !captured.contains_key(*name)).collect();
	assert!(
		mismatched.is_empty() && orphaned.is_empty(),
		"render differs from baseline: {mismatched:?}; baseline without a page: {orphaned:?}. Look at {DIST} (`nix run .#visual`), then accept with UPDATE_EXPECT=1"
	);
}

fn pngs(dir: &Path) -> BTreeMap<String, PathBuf> {
	std::fs::read_dir(dir)
		.unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
		.map(|entry| entry.expect("read_dir entry").path())
		.filter(|p| p.extension().is_some_and(|e| e == "png"))
		.map(|p| (p.file_name().expect("read_dir yields named entries").to_string_lossy().into_owned(), p))
		.collect()
}

/// The render is OS-sensitive but arch-insensitive, and the baselines are linux's.
#[cfg(target_os = "linux")]
fn capture() -> PathBuf {
	let out = PathBuf::from(format!("{DIST}/captured"));
	std::fs::create_dir_all(&out).expect("create capture dir");
	let status = Command::new("playwright")
		.arg("test")
		.current_dir(VISUAL)
		.env("EV_SNAPSHOT_OUT", &out)
		.status()
		.expect("playwright is on PATH in the devShell; enter it with `nix develop`");
	assert!(status.success(), "playwright exited with {status}");
	out
}

/// Off linux, the capture is the `*-linux` derivation — a linux builder runs it natively.
#[cfg(not(target_os = "linux"))]
fn capture() -> PathBuf {
	let nix = |args: &[&str], env: &[(&str, &str)]| {
		let out = Command::new("nix").args(args).envs(env.iter().copied()).output().expect("nix is on PATH");
		assert!(out.status.success(), "nix {args:?}: {}", String::from_utf8_lossy(&out.stderr));
		String::from_utf8(out.stdout).expect("nix prints store paths").trim().to_owned()
	};
	let dist = nix(&["store", "add-path", DIST, "--name", "ev-visual-dist"], &[]);
	let flake = format!("{}/..#packages.{}-linux.visual-snapshots", env!("CARGO_MANIFEST_DIR"), std::env::consts::ARCH);
	PathBuf::from(nix(&["build", "--impure", "--no-link", "--print-out-paths", &flake], &[("EV_VISUAL_DIST", &dist)]))
}
