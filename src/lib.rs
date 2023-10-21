mod util;
mod state;

use clap::Parser;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::state::State;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Julia parameter
    #[arg(long, value_parser = Self::parse_complex_number, default_value = "0.4+0.1i")]
    constant: [f32; 2],

    /// Maximum number of iterations per vertex when not using the shader
    #[arg(long, default_value_t = 250)]
    maximum_iterations: u32,

    /// Window size
    #[arg(long, value_parser = Self::parse_resolution, default_value = "800:800")]
    resolution: PhysicalSize<f32>,
}

impl Args {
    fn parse_complex_number(s: &str) -> Result<[f32; 2], &'static str> {
        const MESSAGE: &str = "constant must be a complex number in cartesian notation";
        let loc = s.rfind("+").or_else(|| s.rfind("-")).ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok([
            s[..loc].parse::<f32>().map_err(err)?,
            s[loc..s.len() - 1].parse::<f32>().map_err(err)?
        ])
    }

    fn parse_resolution(s: &str) -> Result<PhysicalSize<f32>, &'static str> {
        const MESSAGE: &str = "width and height";
        let loc = s.find(":").ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok(PhysicalSize::new(
            s[..loc].parse::<f32>().map_err(err)?,
            s[loc + 1..].parse::<f32>().map_err(err)?,
        ))
    }
}

pub async fn run() {
    let args = Args::parse();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Julia")
        .with_decorations(false)
        .with_inner_size(args.resolution)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(args, window).await;

    event_loop.run(move |event, _, control_flow| util::handle_event(&mut state, event, control_flow));
}
