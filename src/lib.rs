mod util;
mod state;
mod vertex;
mod palette;

use clap::Parser;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::state::State;

type ComplexNumber = [f32; 2];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Frames per second limit
    #[arg(long, default_value_t = 60f32)]
    fps: f32,

    /// Julia parameter
    #[arg(long, value_parser = Self::parse_complex_number, default_value = "-0.162+1.04i")]
    constant: ComplexNumber,

    /// Perform Julia iteration in the shader, a tradeoff between speed and precision
    #[arg(long, value_parser = Self::parse_bool, default_value = "no")]
    use_gpu: bool,

    /// Maximum number of iterations per vertex when not using the shader
    #[arg(long, default_value_t = 250)]
    maximum_iterations: u32,

    // todo resolution as command line argument
}

impl Args {
    fn parse_complex_number(s: &str) -> Result<ComplexNumber, &'static str> {
        const MESSAGE: &str = "constant must be a complex number in cartesian notation";
        let loc = s.rfind("+").or_else(|| s.rfind("-")).ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok([
            s[..loc].parse::<f32>().map_err(err)?,
            s[loc..s.len() - 1].parse::<f32>().map_err(err)?
        ])
    }

    fn parse_bool(s: &str) -> Result<bool, &'static str> {
        const MESSAGE: &str = "yes/no";
        match s {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err(MESSAGE),
        }
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Julia")
        .with_inner_size(PhysicalSize::new(800, 800)) // todo resolution as command line argument
        .build(&event_loop)
        .unwrap();

    let args = Args::parse();
    let mut state = State::new(args, window).await;

    event_loop.run(move |event, _, control_flow| util::handle_event(&mut state, event, control_flow));
}
