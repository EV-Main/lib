//! Page behaviour beyond the kit: microfrontend remotes.

// wasm-only: the module links wasm-bindgen/web-sys, which aren't deps on native.
// The `mfe!` macro is `#[macro_export]`ed from inside, so it too is wasm-only —
// the sole consumer (a producer cdylib) is always a wasm build.
#[cfg(all(feature = "mfe", target_arch = "wasm32"))]
pub mod mfe;
