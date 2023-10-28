use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[cfg(feature = "console_log")]
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(text: &str);

    #[cfg(feature = "console_log")]
    #[wasm_bindgen(js_namespace=console)]
    pub fn warn(text: &str);

    #[cfg(feature = "console_log")]
    #[wasm_bindgen(js_namespace=console)]
    pub fn error(text: &str);

    pub fn enqueue_rust_events(world_id: usize, events: JsValue);
}
