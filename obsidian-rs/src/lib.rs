use wasm::Command;
use wasm_bindgen::closure::Closure;
use web_sys::Element;

pub use wasm::Notice;

pub mod wasm;

pub struct Plugin {
    plugin: wasm::Plugin,
}

impl From<wasm::Plugin> for Plugin {
    fn from(plugin: wasm::Plugin) -> Self {
        Plugin { plugin }
    }
}

impl Plugin {
    pub fn add_command<F: 'static>(&self, id: &str, name: &str, callback: F)
    where
        F: FnMut(),
    {
        self.plugin.addCommand(Command::new(id, name, callback))
    }

    pub fn add_ribbon_icon<F: 'static>(&self, id: &str, name: &str, callback: F) -> Element
    where
        F: FnMut(),
    {
        self.plugin
            .addRibbonIcon(id, name, Closure::new(callback).into_js_value())
    }

    pub fn add_status_bar_item(&self) -> Element {
        self.plugin.addStatusBarItem()
    }
}
