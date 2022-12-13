use wasm_bindgen::JsValue;

pub struct Logger;

impl Logger {
    // pub fn log(&self, msg: &str) {
    //     web_sys::console::log_1(&JsValue::from_str(msg))
    // }
}

// TODO remove this
pub fn console_log(msg: &str) {
    web_sys::console::log_1(&JsValue::from_str(msg))
}
