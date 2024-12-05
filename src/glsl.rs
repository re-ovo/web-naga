use naga::back::glsl::{Version, WriterFlags as NagaGlslWriterFlags};
use naga::back::wgsl::WriterFlags as WgslWriterFlags;
use naga::front::glsl::Frontend;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::common::{ShaderModule, ShaderStage};

#[wasm_bindgen]
pub struct GlslFrontend(Frontend);

#[wasm_bindgen]
impl GlslFrontend {
    pub fn new() -> Self {
        GlslFrontend(Frontend::default())
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
impl ShaderModule {
    pub fn to_wgsl(&self) -> String {
        let mut buffer = String::new();
        let module_info = self.validate();

        let mut writer = naga::back::wgsl::Writer::new(&mut buffer, WgslWriterFlags::all());
        writer.write(&self.0, &module_info).unwrap();

        buffer
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct GlslVersion {
    version: Version,
}

#[wasm_bindgen]
impl GlslVersion {
    pub fn desktop(version: u16) -> Self {
        GlslVersion {
            version: Version::Desktop(version),
        }
    }

    pub fn embedded(version: u16) -> Self {
        GlslVersion {
            version: Version::Embedded {
                version,
                is_webgl: true,
            },
        }
    }
}

impl GlslVersion {
    pub fn get_naga_version(&self) -> Version {
        self.version
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum  GlslWriterFlags {
    None = 0x0,
    AdjustCoordinateSpace = 0x1,
    TextureShadowLod = 0x2,
    DrawParameters = 0x4,
    IncludeUnusedItems = 0x10,
    ForcePointSize = 0x20,
}