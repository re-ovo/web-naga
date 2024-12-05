use wasm_bindgen::prelude::wasm_bindgen;


#[wasm_bindgen]
pub struct ShaderModule(naga::Module);

impl ShaderModule {
    pub fn wrap(module: naga::Module) -> Self {
        ShaderModule(module)
    }
}

#[wasm_bindgen]
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