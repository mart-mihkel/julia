use crate::Vertex;

pub const MAXIMUM_ITERATIONS: u32 = 250;
const MAXIMUM_DISTANCE_SQUARE: f32 = 4.0;

pub fn julia_iter(z: [f32; 2], c: [f32; 2]) -> u32 {
    let mut re = z[0];
    let mut im = z[1];

    let mut dist_square = re.powi(2) + im.powi(2);
    let mut it = 0;
    while it < MAXIMUM_ITERATIONS && dist_square < MAXIMUM_DISTANCE_SQUARE {
        let temp = re;
        re = temp.powi(2) - im.powi(2) + c[0];
        im = 2.0 * im * temp + c[1];

        dist_square = re * re + im * im;
        it += 1;
    }

    it
}

pub fn init_vertices() -> Vec<Vertex> {
    // todo cpu/gpu vertices
    let mut vertices = vec![Vertex { position: [0.0; 3], color: [0.0; 3] }; 800 * 800];
    for x in 0..800 {
        for y in 0..800 {
            vertices[x + y * 800].position = [x as f32 / 400.0 - 1.0, y as f32 / 400.0 - 1.0, 0.0];
        }
    }

    vertices
}