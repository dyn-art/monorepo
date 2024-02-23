mod js_bindings;
mod logger;
pub mod utils;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Initializes the Rust-JavaScript connection
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_log")]
    setup_console_logger();

    #[cfg(feature = "console_log")]
    setup_console_error_panic_hook();
}

/// Provides a handle to access the raw WASM memory
#[wasm_bindgen(js_name = wasmMemory)]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[cfg(feature = "console_log")]
fn setup_console_logger() {
    if let Err(e) = log::set_logger(logger::console::default_logger()) {
        eprintln!("Failed to setup Javascript logger: {}", e);
        return;
    }
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Console log wasm bindgen bridge set up");
}

// The `console_error_panic_hook` crate provides better debugging of panics by
// logging them with `console.error`. This is great for development, but requires
// all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
// code size when deploying.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_log")]
fn setup_console_error_panic_hook() {
    console_error_panic_hook::set_once();
    log::info!("Console error panic hook setup");
}
