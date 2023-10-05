#[macro_use]
extern crate glium;

mod utils;

use std::fs;
use std::path::Path;
use std::time::Instant;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::index::PrimitiveType;
use crate::utils::Vertex;

enum Set {
    JULIA,
    MANDLEBROT,
}

struct JuliaArgs {
    parameter: [f32; 2],
    offset: [f32; 2],
    zoom: f32,
    fps: f32,
    set: Set,
}

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 800.0))
        .with_title("Julia");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    // load screen
    let vertex_buffer = VertexBuffer::new(&display, &utils::VERTICES).unwrap();
    let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &utils::INDICES).unwrap();

    // load shaders
    let vertex_shader = fs::read_to_string(Path::new("src/shader/shader.vert")).unwrap();
    let fragment_shader = fs::read_to_string(Path::new("src/shader/shader.frag")).unwrap();
    let program = Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let mut args = JuliaArgs {
        parameter: [0.0, 0.0],
        offset: [0.0, 0.0],
        zoom: 1.0,
        fps: 60.0,
        set: Set::MANDLEBROT,
    };

    let mut render_end = render(&args, &display, &vertex_buffer, &index_buffer, &program);
    event_loop.run(move |event, _, control_flow| {
        handle_event(&mut args, event, control_flow);

        let secs_since = Instant::now().duration_since(render_end).as_secs_f32();
        if secs_since > 1.0 / args.fps {
            render_end = render(&args, &display, &vertex_buffer, &index_buffer, &program);
        }
    });
}

fn handle_event(args: &mut JuliaArgs, event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(args, event, control_flow),
        _ => ()
    }
}

fn handle_window_event(args: &mut JuliaArgs, event: WindowEvent, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::CloseRequested => control_flow.set_exit(),
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(args, input, control_flow),
        _ => ()
    }
}

fn handle_keyboard_input(JuliaArgs { parameter, offset, zoom, set, .. }: &mut JuliaArgs, input: KeyboardInput, control_flow: &mut ControlFlow) {
    if let Some(key) = input.virtual_keycode {
        let coef = *zoom * 0.01;
        match key {
            // julia constant
            VirtualKeyCode::Left => parameter[0] += coef,
            VirtualKeyCode::Right => parameter[0] -= coef,
            VirtualKeyCode::Up => parameter[1] += coef,
            VirtualKeyCode::Down => parameter[1] -= coef,
            // movement
            VirtualKeyCode::D => offset[0] += coef,
            VirtualKeyCode::A => offset[0] -= coef,
            VirtualKeyCode::W => offset[1] += coef,
            VirtualKeyCode::S => offset[1] -= coef,
            // zoom
            VirtualKeyCode::Period => *zoom *= 1.02,
            VirtualKeyCode::Comma => *zoom *= 0.98,
            // set
            VirtualKeyCode::M => *set = Set::MANDLEBROT,
            VirtualKeyCode::J => *set = Set::JULIA,
            // exit
            VirtualKeyCode::Escape => control_flow.set_exit(),
            _ => ()
        }
    }
}

fn render(args: &JuliaArgs, display: &Display, vertex_buffer: &VertexBuffer<Vertex>, index_buffer: &IndexBuffer<u16>, program: &Program) -> Instant {
    let uniforms = uniform! {
        parameter: args.parameter,
        offset: args.offset,
        zoom: args.zoom,
        mandelbrot: match args.set {
            Set::MANDLEBROT => true,
            Set::JULIA => false,
        },
    };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.draw(vertex_buffer, index_buffer, program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    Instant::now()
}

fn iter(JuliaArgs { parameter, offset, zoom, set, .. }: &JuliaArgs) {
    todo!("bigdecimal iter")
}
