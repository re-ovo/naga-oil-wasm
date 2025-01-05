#define_import_path test::constants

const PI = 3.14;

@group(0) @binding(auto)
var<uniform> bbb: vec3<f32>;

@group(0) @binding(auto)
var<uniform> ccc: vec3<f32>;

fn gen_vertex_pos() -> vec4<f32> {
    #if FUCK == true
    return vec4<f32>(PI, 0.0, 0.0, 1.0);
    #else
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    #endif
}
