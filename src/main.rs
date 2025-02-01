use naga::{
    back::wgsl,
    valid::{Capabilities, ValidationFlags, Validator},
};
use naga_oil::compose::{
    ComposableModuleDescriptor, Composer, NagaModuleDescriptor, ShaderDefValue, ShaderLanguage,
};

fn main() {
    let mut composer = Composer::default();
    let mut add_module = |source: &str, path: &str| {
        composer
            .add_composable_module(ComposableModuleDescriptor {
                source,
                ..Default::default()
            })
            .unwrap();
    };

    add_module(include_str!("../shaders/c.wgsl"), "c.wgsl");

    println!("{}", composer.contains_module("test::constants2"));

    process(&mut composer);
}

fn process(composer: &mut Composer) {
    let module = composer
        .make_naga_module(NagaModuleDescriptor {
            source: include_str!("../shaders/pbr.wgsl"),
            file_path: "pbr.wgsl",
            shader_defs: [("MAX_LIGHTS".to_string(), ShaderDefValue::Int(222))].into(),
            ..Default::default()
        })
        .unwrap();

    println!("{:?}", module.global_variables);

    let module_info = Validator::new(ValidationFlags::all(), Capabilities::default())
        .validate(&module)
        .unwrap();

    let mut out = String::new();
    let mut writer = wgsl::Writer::new(&mut out, wgsl::WriterFlags::all());

    writer.write(&module, &module_info).unwrap();

    println!("Result:");
    println!("> {}", out);
}
