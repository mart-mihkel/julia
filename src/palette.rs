use crate::Rgb;

const ULTRA_FRACTAL_PALETTE: [Rgb; 5] = [
    [0.0, 0.027, 0.392],
    [0.124, 0.42, 0.796],
    [0.929, 1.0, 1.0],
    [1.0, 0.666, 0.0],
    [0.0, 0.008, 0.0],
];

pub fn pick(i: usize) -> Rgb {
    ULTRA_FRACTAL_PALETTE[i % ULTRA_FRACTAL_PALETTE.len()]
}

pub fn linear_interpolate(start: Rgb, end: Rgb, weight: f32) -> Rgb {
    [
        start[0] * (1.0 - weight) + end[0] * weight,
        start[1] * (1.0 - weight) + end[1] * weight,
        start[2] * (1.0 - weight) + end[2] * weight,
    ]
}
