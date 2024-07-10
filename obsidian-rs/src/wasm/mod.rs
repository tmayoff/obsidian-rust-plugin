use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Command {
    id: String,
    name: String,
    callback: JsValue,
}

impl Command {
    pub fn new<F: 'static>(id: &str, name: &str, callback: F) -> Self
    where
        F: FnMut(),
    {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            callback: Closure::<dyn FnMut()>::new(callback).into_js_value(),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub type Plugin;

    #[wasm_bindgen(structural, method)]
    pub fn addCommand(this: &Plugin, command: Command);

    #[wasm_bindgen(structural, method)]
    pub fn addRibbonIcon(
        this: &Plugin,
        id: &str,
        name: &str,
        callback: JsValue,
    ) -> web_sys::Element;

    #[wasm_bindgen(structural, method)]
    pub fn addStatusBarItem(this: &Plugin) -> web_sys::Element;

    pub type Notice;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> Notice;
}
