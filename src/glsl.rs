use naga::back::glsl::Version;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::common::{ShaderModule, ShaderStage};

#[wasm_bindgen]
pub struct GlslFrontend(naga::front::glsl::Frontend);

#[wasm_bindgen]
impl GlslFrontend {
    pub fn new() -> Self {
        GlslFrontend(naga::front::glsl::Frontend::default())
    }

    pub fn parse(&mut self, source: &str, stage: ShaderStage) -> Result<ShaderModule, JsValue> {
        let options = naga::front::glsl::Options::from(stage.to_naga_shader_stage());
        self.0
            .parse(&options, source)
            .map(|module| ShaderModule::wrap(module))
            .map_err(|err| JsValue::from_str(&format!("{}", err)))
    }
}

#[wasm_bindgen]
pub struct GlslVersion {
    version: Version
}

#[wasm_bindgen]
impl GlslVersion {
    pub fn desktop(version: u16) -> Self {
        GlslVersion {
            version: Version::Desktop(version)
        }
    }

    pub fn embedded(version: u16) -> Self {
        GlslVersion {
            version: Version::Embedded {
                version,
                is_webgl: true
            }
        }
    }
}