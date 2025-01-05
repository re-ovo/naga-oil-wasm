mod shader_def;
mod utils;

use std::{collections::HashMap, iter::FromIterator};

use naga::{
    back::wgsl,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
};
use naga_oil::compose::{
    ComposableModuleDescriptor, Composer, NagaModuleDescriptor,
    ShaderDefValue as InternalShaderDefValue,
};
use shader_def::ShaderDefValue;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
pub struct ShaderLibrary {
    composer: Composer,
}

#[wasm_bindgen]
impl ShaderLibrary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            composer: Composer::default(),
        }
    }

    pub fn register_module(&mut self, source: &str) -> Result<(), JsValue> {
        self.composer
            .add_composable_module(ComposableModuleDescriptor {
                source,
                ..Default::default()
            })
            .map(|_| ())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn has_module(&self, name: &str) -> bool {
        self.composer.contains_module(name)
    }

    pub fn unregister_module(&mut self, name: &str) {
        self.composer.remove_composable_module(name)
    }

    pub fn process(&mut self, source: &str, shader_defs: shader_def::ShaderDefs) -> Result<String, JsValue> {
        
        // Create naga module
        let module = self
            .composer
            .make_naga_module(NagaModuleDescriptor {
                source: source,
                shader_defs: shader_defs.into_hash_map(),
                ..Default::default()
            })
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Validate module
        let module_info = Validator::new(ValidationFlags::empty(), Capabilities::empty())
            .validate(&module)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Generate WGSL output
        let mut out = String::new();
        let mut writer = wgsl::Writer::new(&mut out, wgsl::WriterFlags::all());

        writer
            .write(&module, &module_info)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(out)
    }
}
