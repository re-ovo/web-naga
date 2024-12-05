use web_naga::common::ShaderStage;
use web_naga::glsl::{GlslFrontend, GlslVersion, GlslWriterFlags};
use web_naga::wgsl::{GlslBackendOptions, WgslFrontend};

fn main() {
    test_glsl();
    test_wgsl();
}

fn test_glsl() {
    let glsl = r#"
#version 450

layout(location = 0) in vec4 position;

void main() {
    gl_Position = position;
}
"#;

    let mut front = GlslFrontend::new();
    let shader = front.parse(glsl, ShaderStage::Vertex).unwrap();

    let wgsl = shader.to_wgsl();
    println!("{}", wgsl);
}

fn test_wgsl() {
    let wgsl = r#"
    struct VertexInput {
        @location(0) position: vec4<f32>,
    };

    struct VertexOutput {
        @builtin(position) position: vec4<f32>,
    };

    @vertex
    fn vertex_main(input: VertexInput) -> VertexOutput {
        var output: VertexOutput;
        output.position = input.position;
        return output;
    }
    "#;
    
    let mut front = WgslFrontend::new();
    
    let shader = front.parse(wgsl).unwrap();
    let glsl = shader.to_glsl("vertex_main".into(),GlslBackendOptions {
        version: GlslVersion::desktop(450),
        stage: ShaderStage::Vertex,
        flags: GlslWriterFlags::NONE,
    });
    println!("{}", glsl);
}
