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

pub fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
}
