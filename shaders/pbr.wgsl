#import test::constants as C
#import test::constants::test

@group(0) @binding(auto)
var<uniform> aaa: vec3<f32>;

struct PointLight {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(0) @binding(auto)
var<uniform> lights: array<PointLight, #MAX_LIGHTS>;

@vertex
fn main() -> @builtin(position) vec4<f32> {
    let a = C::bbb;
    let w = C::ccc;
    test();
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
