mod console_logger;
pub mod event_queue;
pub mod js_bindings;

pub fn setup_bindgen() {
    #[cfg(feature = "console_error_panic_hook")]
    setup_panic_hook();

    #[cfg(feature = "console_log")]
    setup_console_logger();
}

#[cfg(feature = "console_log")]
fn setup_console_logger() {
    log::set_logger(console_logger::default_logger()).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function at least once during initialization, and then
// we will get better error messages if our code ever panics.
//
// For more details see
// https://github.com/rustwasm/console_error_panic_hook#readme
#[cfg(feature = "console_error_panic_hook")]
fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}
