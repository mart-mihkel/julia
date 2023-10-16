mod util;
mod state;
mod vertex;
mod palette;

use clap::Parser;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::palette::Palette;
use crate::state::State;

type ComplexNumber = [f64; 2];
type Rgb = [f32; 3];

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Julia parameter
    #[arg(long, value_parser = Self::parse_complex_number, default_value = "0.4+0.1i")]
    constant: ComplexNumber,

    /// Maximum number of iterations per vertex when not using the shader
    #[arg(long, default_value_t = 250)]
    maximum_iterations: u32,

    /// Color palette
    #[arg(long, value_parser = Self::parse_palette, default_value = "ultra-fractal")]
    palette: Palette,

    /// Window size
    #[arg(long, value_parser = Self::parse_resolution, default_value = "800:800")]
    resolution: PhysicalSize<f32>,

    /// Perform Julia iteration in the shader, a tradeoff between speed and precision
    #[arg(long, value_parser = Self::parse_bool, default_value = "no")]
    use_gpu: bool,
}

impl Args {
    fn parse_complex_number(s: &str) -> Result<ComplexNumber, &'static str> {
        const MESSAGE: &str = "constant must be a complex number in cartesian notation";
        let loc = s.rfind("+").or_else(|| s.rfind("-")).ok_or(MESSAGE)?;
        let err = |_| MESSAGE;

        Ok([
            s[..loc].parse::<f64>().map_err(err)?,
            s[loc..s.len() - 1].parse::<f64>().map_err(err)?
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

    fn parse_palette(s: &str) -> Result<Palette, &'static str> {
        const MESSAGE: &str = "must be one of: ultra-fractal, green.";
        match s {
            "ultra-fractal" => Ok(Palette::UltraFractal),
            "green" => Ok(Palette::Green),
            _ => Err(MESSAGE),
        }
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
