use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat};
use winit::dpi::PhysicalSize;
use crate::{ComplexNumber, Rgb};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: ComplexNumber,
    color: Rgb,
}

impl Vertex {
    pub fn new(position: ComplexNumber, color: Rgb) -> Self {
        Self { position, color }
    }

    pub fn with_position(position: ComplexNumber) -> Self {
        Self { position, color: [0f32; 3] }
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
                    offset: std::mem::size_of::<ComplexNumber>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                }
            ],
        }
    }

    pub fn init_indices() -> Vec<u16> {
        vec![0, 1, 2, 0, 3, 1]
    }

    pub fn init_vertices(use_gpu: bool, inner_size: PhysicalSize<u32>) -> Vec<Vertex> {
        match use_gpu {
            true => Self::init_triangle_list_vertices(),
            false => Self::init_point_list_vertices(inner_size),
        }
    }

    fn init_triangle_list_vertices() -> Vec<Vertex> {
        vec![
            Vertex::new([-1.0, -1.0], [0f32; 3]),
            Vertex::new([1.0, 1.0], [0f32; 3]),
            Vertex::new([-1.0, 1.0], [0f32; 3]),
            Vertex::new([1.0, -1.0], [0f32; 3]),
        ]
    }

    fn init_point_list_vertices(inner_size: PhysicalSize<u32>) -> Vec<Vertex> {
        let half_width = inner_size.width as f32 / 2.0;
        let half_height = inner_size.height as f32 / 2.0;

        let capacity = inner_size.height * inner_size.width;
        let mut vertices = Vec::with_capacity(capacity as usize);

        for x in 0..inner_size.width {
            for y in 0..inner_size.height {
                let pos_re = x as f32 / half_width - 1.0;
                let pos_im = y as f32 / -half_height + 1.0;
                vertices.push(Vertex::with_position([pos_re, pos_im]))
            }
        }

        vertices
    }

    pub fn translate_position(&self, offset: ComplexNumber, zoom: f32) -> ComplexNumber {
        // todo bigdecimal
        [
            self.position[0] * zoom + offset[0],
            self.position[1] * zoom + offset[1],
        ]
    }

    pub fn set_color(&mut self, color: Rgb) {
        self.color = color;
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: [0f32; 2], color: [0f32; 3] }
    }
}