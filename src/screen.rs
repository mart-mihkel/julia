use glium::{Display, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32),
}

implement_vertex!(Vertex, position);

const VERTICES: [Vertex; 4] = [
    Vertex { position: (-1.0, -1.0) },
    Vertex { position: (1.0, 1.0) },
    Vertex { position: (-1.0, 1.0) },
    Vertex { position: (1.0, -1.0) },
];

const INDICES: [u16; 6] = [
    0, 1, 2,
    0, 3, 1,
];

pub fn load_vertices(facade: &Display) -> VertexBuffer<Vertex> {
    VertexBuffer::new(facade, &VERTICES).unwrap()
}

pub fn load_indices(facade: &Display) -> IndexBuffer<u16> {
    IndexBuffer::new(facade, PrimitiveType::TrianglesList, &INDICES).unwrap()
}