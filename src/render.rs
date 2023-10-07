use std::time::Instant;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::index::PrimitiveType;

#[derive(Copy, Clone)]
struct Vertex {
    position: (f32, f32),
}

implement_vertex!(Vertex, position);

const VERTICES: [Vertex; 4] = [
    Vertex { position: (-1.0, -1.0) },
    Vertex { position: (1.0, 1.0) },
    Vertex { position: (-1.0, 1.0) },
    Vertex { position: (1.0, -1.0) },
];

const INDICES: [u8; 6] = [
    0, 1, 2,
    0, 3, 1,
];

const VERTEX_SHADER: &str = "
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
";

const FRAGMENT_SHADER: &str = "
    #version 140
    #define MAXIMUM_ITERATIONS 250
    #define MAXIMUM_DISTANCE_SQUARED 4.0

    in vec4 gl_FragCoord;

    uniform vec2 julia_param;
    uniform vec2 offset;
    uniform float zoom;

    out vec4 color;

    int iter() {
        // [0.0, 800.0] -> [0.0, 4.0] -> [-2.0, 2.0] -> scale and offset
        float re = ((gl_FragCoord.x / 200.0) - 2.0) * zoom + offset.x;
        float im = ((gl_FragCoord.y / 200.0) - 2.0) * zoom + offset.y;

        float re_const = julia_param.x;
        float im_const = julia_param.y;

        float dist2 = re * re + im * im;
        int it = 0;
        while (it < MAXIMUM_ITERATIONS && dist2 < MAXIMUM_DISTANCE_SQUARED) {
            float temp_re = re;

            re = re * re - im * im + re_const;
            im = 2.0 * im * temp_re + im_const;

            dist2 = re * re + im * im;
            it++;
        }

        return it;
    }

    vec4 make_color() {
        int it = iter();

        // in the set -> black
        if (it == MAXIMUM_ITERATIONS) {
            return vec4(0.0, 0.0, 0.0, 1.0);
        }

        float ratio = float(it) / MAXIMUM_ITERATIONS;
        return vec4(0.0, 0.0, ratio, 1.0);
    }

    void main() {
        color = make_color();
    }
";

pub struct RenderState {
    last_render: Instant,
    vertices: VertexBuffer<Vertex>,
    indices: IndexBuffer<u8>,
    program: Program,
    facade: Display,
    fps: f32,
}

impl RenderState {
    pub fn new(facade: Display, fps: f32) -> RenderState {
        let vertices = VertexBuffer::new(&facade, &VERTICES).unwrap();
        let indices = IndexBuffer::new(&facade, PrimitiveType::TrianglesList, &INDICES).unwrap();
        let program = Program::from_source(&facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
        let last_render = Instant::now();

        RenderState { facade, vertices, indices, program, fps, last_render }
    }

    pub fn render(&mut self, julia_param: (f32, f32)) {
        // count fps
        if !self.should_render() { return; }

        // fragment shader parameters
        let uniforms = uniform! {
            julia_param: julia_param,
            offset: (0f32, 0f32),
            zoom: 0.6f32,
        };

        // render
        let mut target = self.facade.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&self.vertices, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        self.last_render = Instant::now();
    }

    fn should_render(&self) -> bool {
        self.last_render.elapsed().as_secs_f32() > self.fps.powi(-1)
    }
}

