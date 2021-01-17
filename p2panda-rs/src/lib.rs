//! # p2panda-rs
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// Basic structs and methods to interact with p2panda data structures
pub mod atomic;
/// Special error types from this crate
pub mod error;
/// Author identities to sign data with
pub mod keypair;

#[cfg(target_arch = "wasm32")]
mod wasm_utils {
    use console_error_panic_hook::hook as panic_hook;
    use std::panic;
    use wasm_bindgen::prelude::wasm_bindgen;

    /// Sets a panic hook for better error messages in NodeJS or web browser. See:
    /// https://crates.io/crates/console_error_panic_hook
    #[wasm_bindgen(js_name = setWasmPanicHook)]
    pub fn set_wasm_panic_hook() {
        panic::set_hook(Box::new(panic_hook));
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_utils::set_wasm_panic_hook;
