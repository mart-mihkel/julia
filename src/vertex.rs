#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
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
            Vertex::new([-1.0, -1.0, 0.0], [0.0, 0.0, 0.0]),
            Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0, 0.0]),
            Vertex::new([-1.0, 1.0, 0.0], [0.0, 0.0, 0.0]),
            Vertex::new([1.0, -1.0, 0.0], [0.0, 0.0, 0.0]),
        ]
    }

    fn init_point_list_vertices() -> Vec<Vertex> {
        let mut vertices = vec![Vertex::default(); 800 * 800];
        for x in 0..800 {
            for y in 0..800 {
                vertices[x + y * 800].position = [x as f32 / 400.0 - 1.0, y as f32 / 400.0 - 1.0, 0.0]; // todo ilusamalt
            }
        }

        vertices
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    pub fn position(&self) -> [f32; 3] {
        self.position
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: [0f32; 3], color: [0f32; 3] }
    }
}