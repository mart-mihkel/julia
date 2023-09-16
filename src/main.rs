#[macro_use]
extern crate glium;

mod utils;

use std::fs;
use std::path::Path;
use glium::{Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::index::PrimitiveType;

fn main() {
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 800.0))
        .with_title("Julia");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    // load screen
    let positions = VertexBuffer::new(&display, &utils::VERTICES).unwrap();
    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &utils::INDICES).unwrap();

    // load shaders
    let vertex_shader = fs::read_to_string(Path::new("src/shader/shader.vert")).unwrap();
    let fragment_shader = fs::read_to_string(Path::new("src/shader/shader.frag")).unwrap();
    let program = Program::from_source(&display, &vertex_shader, &fragment_shader, None).unwrap();

    let mut julia_c = [0.2, -0.6, 0.0f32];
    let zoom = 2.5f32;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            match key {
                                // todo zoom
                                VirtualKeyCode::Left => julia_c[0] += 0.01,
                                VirtualKeyCode::Right => julia_c[0] -= 0.01,
                                VirtualKeyCode::Up => julia_c[1] += 0.01,
                                VirtualKeyCode::Down => julia_c[1] -= 0.01,
                                VirtualKeyCode::Escape => control_flow.set_exit(),
                                _ => ()
                            }
                            display.gl_window().window().request_redraw();
                        }
                    }
                    _ => ()
                }
            }
            Event::RedrawRequested(_) => {

                // rendering
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                target.draw(
                    &positions,
                    &indices,
                    &program,
                    &uniform! { zoom: zoom, julia_c: julia_c },
                    &Default::default(),
                ).unwrap();
                target.finish().unwrap();
            }
            _ => ()
        }
    });
}