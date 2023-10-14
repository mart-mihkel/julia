use crate::Rgb;

const GREEN: [Rgb; 8] = [
    [0.0, 0.025, 0.0],
    [0.0, 0.1, 0.0],
    [0.0, 0.125, 0.0],
    [0.0, 0.325, 0.0],
    [0.0, 0.5, 0.0],
    [0.0, 0.725, 0.0],
    [0.0, 0.925, 0.0],
    [0.0, 1.0, 0.0],
];

const ULTRA_FRACTAL: [Rgb; 5] = [
    [0.0, 0.027, 0.392],
    [0.124, 0.42, 0.796],
    [0.929, 1.0, 1.0],
    [1.0, 0.666, 0.0],
    [0.0, 0.008, 0.0],
];

#[derive(Copy, Clone)]
pub enum Palette {
    Green,
    UltraFractal,
}

pub fn pick(palette: Palette, i: usize) -> Rgb {
    match palette {
        Palette::Green => GREEN[i % GREEN.len()],
        Palette::UltraFractal => ULTRA_FRACTAL[i % ULTRA_FRACTAL.len()],
    }
}

pub fn linear_interpolate(start: Rgb, end: Rgb, weight: f32) -> Rgb {
    [
        start[0] * (1.0 - weight) + end[0] * weight,
        start[1] * (1.0 - weight) + end[1] * weight,
        start[2] * (1.0 - weight) + end[2] * weight,
    ]
}
