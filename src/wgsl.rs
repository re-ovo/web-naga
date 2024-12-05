use naga::front::wgsl::Frontend;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::common::ShaderModule;

#[wasm_bindgen]
pub struct WgslFrontend(naga::front::wgsl::Frontend);

#[wasm_bindgen]
impl WgslFrontend {
    pub fn new() -> Self {
        WgslFrontend(Frontend::new())
    }

    pub fn parse(&mut self, source: &str) -> Result<ShaderModule, JsValue> {
        self.0
            .parse(source)
            .map(|module| ShaderModule::wrap(module))
            .map_err(|err| JsValue::from_str(&format!("{:?}", err)))
    }
}
