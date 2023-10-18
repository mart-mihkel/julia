// Vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shader
struct JuliaUniforms {
    constant: vec2<f32>,
};

@group(0)
@binding(0)
var<uniform> julia_uniforms: JuliaUniforms;

fn iter(position: vec4<f32>, max_it: i32) -> f32 {
    var re: f32 = (position.x / 200.0) - 2.0;
    var im: f32 = (position.y / 200.0) - 2.0;

    let re_c = julia_uniforms.constant.x;
    let im_c = julia_uniforms.constant.y;

    var dist2: f32 = re * re + im * im;
    var it: i32 = 0;

    loop {
        if (it == max_it || dist2 >= 4.0) {
            break;
        }

        var temp = re;
        re = temp * temp - im * im + re_c;
        im = 2.0 * im * temp + im_c;

        continuing {
            dist2 = re * re + im * im;
            it++;
        }
    }

    return f32(it) / f32(max_it);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let it = iter(in.clip_position, 250);
    return vec4<f32>(0.0, 0.0, it, 1.0);
}
