struct JuliaUniform {
    constant: vec2<f32>,
    offset: vec2<f32>,
    width: f32,
    height: f32,
    zoom: f32,
    palette: array<vec4<f32>, 5>,
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
    let fragment_coordinate: vec2<f32> = vec2<f32>(global_index.xy) / vec2<f32>(parameters.width / 2.0, parameters.height / 2.0) - vec2<f32>(1.0, 1.0);

    let smoothing: f32 = julia_iter(fragment_coordinate);
    let i1: i32 = i32(smoothing) % 5;
    let i2: i32 = i32(smoothing + 1.0) % 5;
    let color = mix(parameters.palette[i1], parameters.palette[i2], smoothing % 1.0);

    textureStore(output_texture, vec2<i32>(global_index.xy), color);
}

fn julia_iter(fragment_coordinate: vec2<f32>) -> f32 {
    var re: f32 = fragment_coordinate.x * parameters.zoom + parameters.offset.x;
    var im: f32 = fragment_coordinate.y * parameters.zoom - parameters.offset.y;

    let re_c = parameters.constant.x;
    let im_c = parameters.constant.y;

    var dist: f32 = sqrt(re * re + im * im);
    var smoothing: f32 = exp(-dist);
    var it: i32 = 0;

    loop {
        if (it == 1024 || dist >= 2.0) {
            break;
        }

        let temp = re;
        re = temp * temp - im * im + re_c;
        im = 2.0 * im * temp + im_c;

        continuing {
            dist = sqrt(re * re + im * im);
            smoothing += exp(-dist);
            it++;
        }
    }

    return smoothing;
}
