struct JuliaUniform {
    constant: vec2<f32>,
    offset: vec2<f32>,
    width: f32,
    height: f32,
    zoom: f32,
}

@group(0)
@binding(0)
var<uniform> parameters: JuliaUniform;

@group(0)
@binding(1)
var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16, 16)
fn cs_main(@builtin(global_invocation_id) global_index: vec3<u32>) {
    let fragment_coordinate: vec2<f32> = vec2<f32>(global_index.xy) / vec2<f32>(parameters.width, parameters.height) - vec2<f32>(0.5, 0.5);

    // todo
    let color = vec4<f32>(0.0, 0.5, 0.0, 1.0);

    textureStore(output_texture, vec2<i32>(global_index.xy), color);
}
