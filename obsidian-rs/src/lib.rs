use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Plugin;

    #[wasm_bindgen(structural, method)]
    pub fn addCommand(this: &Plugin, command: JsValue);

    #[wasm_bindgen(structural, method)]
    pub fn addRibbonIcon(this: &Plugin, id: &str, name: &str, callback: &Closure<dyn FnMut()>);

    pub type Notice;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Notice;
}
