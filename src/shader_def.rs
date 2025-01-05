use std::collections::HashMap;

use naga_oil::compose::ShaderDefValue as InternalShaderDefValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ShaderDefValue {
    internal: InternalShaderDefValue,
}

#[wasm_bindgen]
impl ShaderDefValue {
    pub fn new_bool(value: bool) -> Self {
        Self {
            internal: InternalShaderDefValue::Bool(value),
        }
    }

    pub fn new_int(value: i32) -> Self {
        Self {
            internal: InternalShaderDefValue::Int(value),
        }
    }

    pub fn new_uint(value: u32) -> Self {
        Self {
            internal: InternalShaderDefValue::UInt(value),
        }
    }
}

#[wasm_bindgen]
pub struct ShaderDefs {
    shader_defs: HashMap<String, ShaderDefValue>,
}

#[wasm_bindgen]
impl ShaderDefs {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            shader_defs: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: ShaderDefValue) {
        self.shader_defs.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<ShaderDefValue> {
        self.shader_defs.get(name).cloned()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.shader_defs.contains_key(name)
    }

    pub fn remove(&mut self, name: &str) {
        self.shader_defs.remove(name);
    }
}

impl ShaderDefs {
    pub fn into_hash_map(self) -> HashMap<String, InternalShaderDefValue> {
        self.shader_defs
            .into_iter()
            .map(|(k, v)| (k, v.internal))
            .collect()
    }
}
