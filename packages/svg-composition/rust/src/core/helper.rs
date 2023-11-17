use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub fn convert_optional_jsvalue<T>(value: JsValue) -> Option<T>
where
    T: DeserializeOwned,
{
    if value.is_undefined() || value.is_null() {
        None
    } else {
        serde_wasm_bindgen::from_value(value).ok()
    }
}
