use naga::back::glsl::{Options, PipelineOptions, WriterFlags};
use naga::front::wgsl::Frontend;
use naga::proc::BoundsCheckPolicies;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::common::{ShaderModule, ShaderStage};
use crate::glsl::{GlslVersion, GlslWriterFlags};

#[wasm_bindgen]
pub struct WgslFrontend(Frontend);

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

#[wasm_bindgen]
pub struct GlslBackendOptions {
    pub version: GlslVersion,
    pub stage: ShaderStage,
    pub flags: GlslWriterFlags,
}

#[wasm_bindgen]
impl ShaderModule {
    pub fn to_glsl(
        &self,
        entry_point: String,
        opts: GlslBackendOptions,
    ) -> String {
        let module_info = self.validate();
        let mut buffer = String::new();

        let mut options = Options::default();
        options.version = opts.version.get_naga_version();
        options.writer_flags = WriterFlags::from_bits(opts.flags.bits()).unwrap();

        let pipeline_options = PipelineOptions {
            shader_stage: opts.stage.to_naga_shader_stage(),
            entry_point: entry_point.to_string(),
            multiview: None,
        };

        let mut writer = naga::back::glsl::Writer::new(
            &mut buffer,
            &self.0,
            &module_info,
            &options,
            &pipeline_options,
            BoundsCheckPolicies::default()
        ).unwrap();

        writer.write().unwrap();

        buffer
    }
}