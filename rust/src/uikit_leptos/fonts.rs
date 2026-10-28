use leptos::prelude::*;

/// Self-hosted brand webfonts (Inter + Playfair Display), the families named by
/// the `--font-sans` / `--font-serif` token chains in `tokens.css`. Render once
/// near the app root: it puts the `@font-face` rules over the variable TTFs into
/// the document head through `leptos_meta` (so the host provides its
/// `MetaContext`, as for any head tag).
///
/// Leptos has no asset pipeline to collect the files the way Dioxus's `asset!`
/// does, so the consumer serves [`FONT_FILES`] under `base` (e.g. `"/fonts"`),
/// and the rules point there.
#[component]
pub fn Fonts(#[prop(into)] base: String) -> impl IntoView {
	let css = format!(
		"@font-face{{font-family:'Inter';font-style:normal;font-weight:100 900;font-display:swap;src:url('{base}/Inter.ttf') format('truetype')}}\
		 @font-face{{font-family:'Inter';font-style:italic;font-weight:100 900;font-display:swap;src:url('{base}/Inter-Italic.ttf') format('truetype')}}\
		 @font-face{{font-family:'Playfair Display';font-style:normal;font-weight:300 900;font-display:swap;src:url('{base}/PlayfairDisplay.ttf') format('truetype')}}\
		 @font-face{{font-family:'Playfair Display';font-style:italic;font-weight:300 900;font-display:swap;src:url('{base}/PlayfairDisplay-Italic.ttf') format('truetype')}}"
	);
	view! { <leptos_meta::Style>{css}</leptos_meta::Style> }
}

/// The files [`Fonts`] points at, by the name it expects under its `base`.
pub static FONT_FILES: [(&str, &[u8]); 4] = [
	("Inter.ttf", include_bytes!("../uikit/fonts/Inter.ttf")),
	("Inter-Italic.ttf", include_bytes!("../uikit/fonts/Inter-Italic.ttf")),
	("PlayfairDisplay.ttf", include_bytes!("../uikit/fonts/PlayfairDisplay.ttf")),
	("PlayfairDisplay-Italic.ttf", include_bytes!("../uikit/fonts/PlayfairDisplay-Italic.ttf")),
];
