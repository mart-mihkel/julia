const ULTRA_FRACTAL_PALETTE: [[f32; 3]; 5] = [
    [0.0, 0.027, 0.392],
    [0.124, 0.42, 0.796],
    [0.929, 1.0, 1.0],
    [1.0, 0.666, 0.0],
    [0.0, 0.008, 0.0],
];

pub fn pick(i: usize) -> [f32; 3] {
    ULTRA_FRACTAL_PALETTE[i % ULTRA_FRACTAL_PALETTE.len()]
}

pub fn linear_interpolate(start: [f32; 3], end: [f32; 3], weight: f32) -> [f32; 3] {
    [
        start[0] * (1.0 - weight) + end[0] * weight,
        start[1] * (1.0 - weight) + end[1] * weight,
        start[2] * (1.0 - weight) + end[2] * weight,
    ]
}
