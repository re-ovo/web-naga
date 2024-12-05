use naga::back::glsl::Version;
use naga::back::wgsl::WriterFlags;
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

        let mut writer = naga::back::wgsl::Writer::new(&mut buffer, WriterFlags::all());
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

bitflags::bitflags! {
    #[wasm_bindgen]
    #[derive(Copy, Clone)]
    pub struct GlslWriterFlags: u32 {
        const NONE = 0x0;
        /// Flip output Y and extend Z from (0, 1) to (-1, 1).
        const ADJUST_COORDINATE_SPACE = 0x1;
        /// Supports GL_EXT_texture_shadow_lod on the host, which provides
        /// additional functions on shadows and arrays of shadows.
        const TEXTURE_SHADOW_LOD = 0x2;
        /// Supports ARB_shader_draw_parameters on the host, which provides
        /// support for `gl_BaseInstanceARB`, `gl_BaseVertexARB`, `gl_DrawIDARB`, and `gl_DrawID`.
        const DRAW_PARAMETERS = 0x4;
        /// Include unused global variables, constants and functions. By default the output will exclude
        /// global variables that are not used in the specified entrypoint (including indirect use),
        /// all constant declarations, and functions that use excluded global variables.
        const INCLUDE_UNUSED_ITEMS = 0x10;
        /// Emit `PointSize` output builtin to vertex shaders, which is
        /// required for drawing with `PointList` topology.
        ///
        /// https://registry.khronos.org/OpenGL/specs/es/3.2/GLSL_ES_Specification_3.20.html#built-in-language-variables
        /// The variable gl_PointSize is intended for a shader to write the size of the point to be rasterized. It is measured in pixels.
        /// If gl_PointSize is not written to, its value is undefined in subsequent pipe stages.
        const FORCE_POINT_SIZE = 0x20;
    }
}
