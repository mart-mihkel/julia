#[macro_use]
extern crate glium;

mod screen;
mod render;
mod event_handler;

use std::fmt::Pointer;
use std::time::Instant;
use glium::{Display, Surface};
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Frames per second limit
    #[arg(long, default_value_t = 60)]
    fps: u8,

    /// Julia parameter
    #[arg(long, value_parser = Self::parse_complex_number, default_value = "0.355-0.355i")]
    julia_param: (f32, f32),
}

impl Args {
    fn parse_complex_number(s: &str) -> Result<(f32, f32), &'static str> {
        let loc = s.rfind("+").or_else(|| s.rfind("-")).unwrap();
        let err = |_| "julia parameter must be a complex number";
        Ok((
            s[..loc].parse::<f32>().map_err(err)?,
            s[loc..s.len() - 1].parse::<f32>().map_err(err)?
        ))
    }
}

fn main() {
    let args = Args::parse();

    // create window
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800.0, 800.0))
        .with_title("Julia");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    // load screen
    let vertices = screen::load_vertices(&display);
    let indices = screen::load_indices(&display);

    // load shaders
    let program = render::load_shaders(&display);

    let mut render_end = render::render(args.julia_param, &display, &vertices, &indices, &program);
    event_loop.run(move |event, _, control_flow| {
        event_handler::handle_event(event, control_flow);

        let secs_since = Instant::now().duration_since(render_end).as_secs_f32();
        if secs_since > 1.0 / args.fps as f32 {
            render_end = render::render(args.julia_param, &display, &vertices, &indices, &program);
        }
    });
}
