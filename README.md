# WebNaga

Provides mutual conversion between different Shader language syntaxes, implemented using naga and wasm

## Frontend and Backend
### Frontend
- GLSL
- WGSL

## Backend
- WGSL
- GLSL

## Usage
GLSL -> WGSL
```javascript
const front = GlslFrontend.new();
const module = front.parse(code, ShaderStage.Vertex);
const result = module.to_wgsl()
```

WGSL -> GLSL
```javascript
const front = WgslFrontend.new();
const module = front.parse(code);
const options = GlslBackendOptions.new();
// options.stage = ShaderStage.Vertex;
// options.version = GlslVersion.embedded(330)
// options.flags = GlslWriterFlags.None
const result = module.to_glsl("main", options)
```

