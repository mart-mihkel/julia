#[macro_use]
extern crate glium;

mod render;
mod event_util;

use glium::Display;
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use clap::Parser;
use crate::event_util::EventUtil;
use crate::render::RenderState;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Frames per second limit
    #[arg(long, default_value_t = 60f32)]
    fps: f32,

    /// Julia parameter
    #[arg(long, value_parser = Self::parse_complex_number, default_value = "0.355-0.355i")]
    julia_param: (f32, f32),

    /// Window size
    #[arg(long, value_parser = Self::parse_window_size, default_value = "800x800")]
    window_size: LogicalSize<u16>,
}

impl Args {
    fn parse_complex_number(s: &str) -> Result<(f32, f32), &'static str> {
        const MESSAGE: &str = "uh oh!";
        let loc = s.rfind("+").or_else(|| s.rfind("-")).ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok((
            s[..loc].parse::<f32>().map_err(err)?,
            s[loc..s.len() - 1].parse::<f32>().map_err(err)?
        ))
    }

    fn parse_window_size(s: &str) -> Result<LogicalSize<u16>, &'static str> {
        const MESSAGE: &str = "uh oh!";
        let loc = s.find("x").ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok(LogicalSize::new(
            s[..loc].parse::<u16>().map_err(err)?,
            s[loc + 1..].parse::<u16>().map_err(err)?,
        ))
    }
}

fn main() {
    let args = Args::parse();

    let event_loop = EventLoop::new();
    let display = Display::new(
        WindowBuilder::new()
            .with_inner_size(args.window_size)
            .with_title("Julia")
            .with_decorations(false),
        ContextBuilder::new(),
        &event_loop
    ).unwrap();

    let mut render_state = RenderState::new(display, args.fps);

    event_loop.run(move |event, _, control_flow| {
        EventUtil::handle_event(event, control_flow);
        render_state.render(args.julia_param);
    });
}
