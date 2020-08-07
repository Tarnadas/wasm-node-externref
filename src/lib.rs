use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"It works".into());

    Ok(())
}
