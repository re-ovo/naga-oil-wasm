# naga-oil-wasm

WASM bindings for naga-oil.

## Installation

```bash
npm install naga-oil-wasm
```

## Usage
You can view the naga_oil official documentation [here](https://github.com/bevyengine/naga_oil).

## Basic Usage
```ts
const shaderLib = new ShaderLibrary();

// Register composable shader modules
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
`, shaderDefs);

console.log(processedShader); // Output the processed shader
```
