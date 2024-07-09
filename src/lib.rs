use gloo::console::log;
use wasm_bindgen::prelude::*;

struct Plugin {}

#[wasm_bindgen]
pub fn onload() {
    log!("Loaded");
}
