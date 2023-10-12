use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 2], color: [f32; 3]) -> Self {
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

    pub fn init_indices() -> Vec<u16> {
        // todo slice/array
        vec![0, 1, 2, 0, 3, 1]
    }

    pub fn init_vertices(use_gpu: bool) -> Vec<Vertex> {
        // todo slice/array
        match use_gpu {
            true => Self::init_triangle_list_vertices(),
            false => Self::init_point_list_vertices(),
        }
    }

    fn init_triangle_list_vertices() -> Vec<Vertex> {
        vec![
            Vertex::new([-1.0, -1.0], [0.0, 0.0, 0.0]),
            Vertex::new([1.0, 1.0], [0.0, 0.0, 0.0]),
            Vertex::new([-1.0, 1.0], [0.0, 0.0, 0.0]),
            Vertex::new([1.0, -1.0], [0.0, 0.0, 0.0]),
        ]
    }

    fn init_point_list_vertices() -> Vec<Vertex> {
        let mut vertices = vec![Vertex::default(); 800 * 800];
        for x in 0..800 {
            for y in 0..800 {
                // [0, 800] -> [0, 2] -> [-1, 1]
                let pos_x = x as f32 / 400.0 - 1.0;
                let pos_y = y as f32 / 400.0 - 1.0;
                vertices[x + y * 800].position = [pos_x, pos_y];
            }
        }

        vertices
    }

    pub fn translate_position(&self, offset_x: f32, offset_y: f32, zoom: f32) -> [f32; 2] {
        [
            self.position[0] * zoom + offset_x,
            self.position[1] * zoom + offset_y,
        ]
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: [0f32; 2], color: [0f32; 3] }
    }
}