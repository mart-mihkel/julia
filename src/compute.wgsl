struct JuliaUniform {
    constant: vec2<f32>,
    offset: vec2<f32>,
    width: f32,
    height: f32,
    zoom: f32,
}

const max_it: i32 = 512;

// palette
const palette_size: i32 = 5;
const color0: vec4<f32> = vec4<f32>(0.0, 0.03, 0.39, 1.0);
const color1: vec4<f32> = vec4<f32>(0.12, 0.42, 0.8, 1.0);
const color2: vec4<f32> = vec4<f32>(0.93, 1.0, 1.0, 1.0);
const color3: vec4<f32> = vec4<f32>(1.0, 0.67, 0.0, 1.0);
const color4: vec4<f32> = vec4<f32>(0.0, 0.01, 0.0, 1.0);

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
    let color: vec4<f32> = pick_color(smoothing);

    textureStore(output_texture, vec2<i32>(global_index.xy), color);
}

fn julia_iter(fragment_coordinate: vec2<f32>) -> f32 {
    var z: vec2<f32> = vec2<f32>(
        fragment_coordinate.x * parameters.zoom + parameters.offset.x,
        fragment_coordinate.y * parameters.zoom - parameters.offset.y,
    );

    var modulus: f32 = sqrt(modulus_square(z));
    var smoothing: f32 = exp(-modulus); // exponential distance estimation for a smoother transition between colors
    var it: i32 = 0;

    loop {
        if (it == max_it) { return -1.0; } // color the fragment black if it's in the set
        if (modulus >= 2.0) { break; }

        z = powi_complex(z, 4u) + parameters.constant;

        continuing {
            modulus = sqrt(modulus_square(z));
            smoothing += exp(-modulus);
            it++;
        }
    }

    return smoothing;
}

fn pick_color(smoothing: f32) -> vec4<f32> {
    let interpolation_fraction: f32 = smoothing % 1.0;
    let i: i32 = i32(smoothing) % palette_size;
    switch (i) {
        case 0: { return mix(color0, color1, interpolation_fraction); }
        case 1: { return mix(color1, color2, interpolation_fraction); }
        case 2: { return mix(color2, color3, interpolation_fraction); }
        case 3: { return mix(color3, color4, interpolation_fraction); }
        case 4: { return mix(color4, color0, interpolation_fraction); }
        default: { return vec4<f32>(0.0, 0.0, 0.0, 1.0); }
    }
}

fn modulus_square(z: vec2<f32>) -> f32 {
    return z.x * z.x + z.y * z.y;
}

fn mul_complex(lhs: vec2<f32>, rhs: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(lhs.x * rhs.x - lhs.y * rhs.y, lhs.x * rhs.y + lhs.y * rhs.x);
}

/// Raise a complex number to the power of a positive integer
fn powi_complex(z: vec2<f32>, n: u32) -> vec2<f32> {
    var product: vec2<f32> = z;
    var i: u32 = 1u;

    loop {
        if (i == n) { break; }
        continuing {
            product = mul_complex(product, z);
            i++;
        }
    }

    return product;
}
