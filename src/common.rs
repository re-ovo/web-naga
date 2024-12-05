use naga::valid::{Capabilities, ModuleInfo, ValidationFlags, Validator};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ShaderModule(pub(crate) naga::Module);

impl ShaderModule {
    pub fn wrap(module: naga::Module) -> Self {
        ShaderModule(module)
    }

    pub fn validate(&self) -> ModuleInfo {
        let mut validator = Validator::new(ValidationFlags::default(), Capabilities::default());
        validator.validate(&self.0).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

impl ShaderStage {
    pub fn to_naga_shader_stage(&self) -> naga::ShaderStage {
        match self {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Fragment => naga::ShaderStage::Fragment,
            ShaderStage::Compute => naga::ShaderStage::Compute,
        }
    }
}
