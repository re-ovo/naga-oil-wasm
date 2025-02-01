# naga-oil-wasm

WASM bindings for naga-oil.

## Installation

```bash
npm install naga-oil-wasm
```

## Documentation
You can view the naga_oil official documentation [here](https://github.com/bevyengine/naga_oil).

## Basic Usage
```ts
const shaderLib = new ShaderLibrary();

// Register composable shader modules
// You must define the module name so that the shader can import it
shaderLib.register_module(`
#define_import_path module_name;

// ... module code ...
`);

// Shader definitions
const shaderDefs = new ShaderDefs();
shaderDefs.add("MAX_LIGHTS", ShaderDefValue.new_int(10));

// Process the shader
const processedShader = shaderLib.process(`
#define_import_path module_name;

// ... shader code ...

sturct PointLight {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
}

@group(0) @binding(0)
var<uniform> pointLights: array<PointLight, #MAX_LIGHTS>;

@vertex
fn main() -> @builtin(position) vec4<f32> {
  // ... vertex code ...
}
`, shaderDefs);

console.log(processedShader); // Output the processed shader
```
