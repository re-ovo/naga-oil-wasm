#define_import_path test::constants

const PI = 3.14;

@group(0) @binding(auto)
var<uniform> bbb: vec3<f32>;

@group(1) @binding(auto)
var<uniform> ccc: vec3<f32>;

fn test() -> i32 {
    return 1;
}
