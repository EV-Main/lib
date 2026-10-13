//! Serves the source-of-truth component board as a *live*, interactive page —
//! every primitive, every variant, wired through dioxus-liveview so the
//! components actually respond to clicks/typing (no wasm build: the VDOM runs
//! native and patches the browser over a websocket) — and, under `/dist/`, the
//! static pages the visual-regression test screenshots.
//!
//!   nix run .#visual
//!   → http://127.0.0.1:52414            live board
//!   → http://127.0.0.1:52414/dist/      static pages (index, colorscheme, one per primitive)

include!("../tests/support/gallery.rs");

use axum::{
	Router,
	extract::{Path as UrlPath, ws::WebSocketUpgrade},
	http::{StatusCode, header},
	response::Html,
	routing::get,
};
use dioxus_liveview::{LiveViewPool, axum_socket, interpreter_glue};

const PORT: u16 = 52414;

#[tokio::main]
async fn main() {
	write_dist();

	let pool = LiveViewPool::new();
	let index = Html(format!(
		"{}\n<body class=\"min-h-screen p-8\">\n<div id=\"main\"></div>\n{}\n</body></html>",
		head("EV UIKit — Components"),
		interpreter_glue("/ws"),
	));

	let tailwind = std::fs::read_to_string(format!("{DIST}/tailwind.js")).expect("written by write_dist");
	let router = Router::new()
		.route("/", get(move || async move { index.clone() }))
		.route("/tailwind.js", get(move || async move { ([(header::CONTENT_TYPE, "text/javascript")], tailwind) }))
		.route(
			"/dist/",
			get(|| async { Html(std::fs::read_to_string(format!("{DIST}/index.html")).expect("written by write_dist")) }),
		)
		.route("/dist/{file}", get(dist_file))
		.route(
			"/ws",
			get(move |ws: WebSocketUpgrade| async move {
				ws.on_upgrade(move |socket| async move {
					_ = pool.launch(axum_socket(socket), app).await;
				})
			}),
		);

	let addr = std::net::SocketAddr::from(([127, 0, 0, 1], PORT));
	println!("EV UIKit live on http://{addr}");
	let listener = tokio::net::TcpListener::bind(addr).await.expect("bind port");
	axum::serve(listener, router.into_make_service()).await.expect("serve");
}
async fn dist_file(UrlPath(file): UrlPath<String>) -> Result<([(header::HeaderName, &'static str); 1], Vec<u8>), StatusCode> {
	let mime = match file.rsplit_once('.').map(|(_, ext)| ext) {
		Some("html") => "text/html; charset=utf-8",
		Some("js") => "text/javascript",
		Some("json") => "application/json",
		_ => return Err(StatusCode::NOT_FOUND),
	};
	// `{file}` matches one segment, so no `..` can climb out of DIST
	let body = std::fs::read(format!("{DIST}/{file}")).map_err(|_| StatusCode::NOT_FOUND)?;
	Ok(([(header::CONTENT_TYPE, mime)], body))
}

/// The whole board as one live component: each cell hosts a real, interactive
/// instance of the primitive (mirrors `cell()`/`board()` markup from gallery.rs).
fn app() -> Element {
	rsx! {
		h1 { class: "mb-6 text-2xl font-semibold", "EV UIKit — Component Library" }
		div { class: "grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3",
			for (name, render) in GALLERY.iter().copied() {
				section { class: "stage flex flex-col gap-3 rounded-lg border border-border bg-card/30 p-5",
					h3 { class: "text-xs font-medium uppercase tracking-wide text-ink-soft", {name} }
					div { class: "flex flex-1 flex-wrap items-center gap-3", {render()} }
				}
			}
		}
	}
}
