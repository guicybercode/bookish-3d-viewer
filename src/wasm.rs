#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{HtmlCanvasElement, Window, Document};
#[cfg(target_arch = "wasm32")]
use crate::{App, model::Model};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmViewer {
    app: App,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmViewer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmViewer, JsValue> {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
        
        Ok(WasmViewer {
            app: App::new(),
        })
    }

    #[wasm_bindgen]
    pub fn load_model_from_bytes(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.app.load_file_from_bytes(data, "obj")
            .map_err(|e| JsValue::from_str(&format!("Failed to load model: {}", e)))
    }

    #[wasm_bindgen]
    pub fn handle_keyboard(&mut self, key: &str) {
        self.app.handle_keyboard(key);
    }

    #[wasm_bindgen]
    pub fn rotate_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.app.camera.rotate(delta_x, delta_y);
    }

    #[wasm_bindgen]
    pub fn zoom_camera(&mut self, delta: f32) {
        self.app.camera.zoom(delta);
    }

    #[wasm_bindgen]
    pub fn pan_camera(&mut self, delta_x: f32, delta_y: f32) {
        self.app.camera.pan(delta_x, delta_y);
    }

    #[wasm_bindgen]
    pub fn reset_camera(&mut self) {
        self.app.camera.reset();
    }

    #[wasm_bindgen]
    pub fn toggle_menu(&mut self) {
        self.app.menu.toggle();
    }

    #[wasm_bindgen]
    pub fn get_model_info(&self) -> Option<String> {
        self.app.model_info.as_ref().map(|info| info.format_info())
    }

    #[wasm_bindgen]
    pub fn has_model(&self) -> bool {
        self.app.model.is_some()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

