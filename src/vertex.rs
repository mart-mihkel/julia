use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat};
use crate::Rgb;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: Rgb, // todo remove
}

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0], color: [0f32; 3] },
    Vertex { position: [1.0, 1.0], color: [0f32; 3] },
    Vertex { position: [-1.0, 1.0], color: [0f32; 3] },
    Vertex { position: [1.0, -1.0], color: [0f32; 3] },
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    0, 3, 1,
];

impl Vertex {
    pub fn new(position: [f32; 2], color: Rgb) -> Self {
        Self { position, color }
    }

    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ],
        }
    }
}
