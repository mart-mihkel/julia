use sfml::graphics::{Color, Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex};
use sfml::system::{Vector2f, Vector2u};

pub struct Grid {
    size: Vector2u,
    pub vertices: Vec<Vertex>,
}

impl Grid {
    pub fn new(size: Vector2u) -> Grid {
        let mut vertices = Vec::with_capacity((size.x * size.y) as usize);
        for x in 0..size.x {
            for y in 0..size.y {
                let position = Vector2f::new(x as f32, y as f32);
                vertices.push(Vertex::with_pos_color(position, Color::BLACK));
            }
        }

        Grid { size, vertices }
    }

    pub fn set_vertex_color(&mut self, x: u32, y: u32, color: Color) {
        let i = x * self.size.y + y;
        self.vertices[i as usize].color = color;
    }
}

impl Drawable for Grid {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_primitives(&self.vertices, PrimitiveType::POINTS, states);
    }
}
