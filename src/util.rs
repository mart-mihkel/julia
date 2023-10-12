use wgpu::SurfaceError;
use winit::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event::Event::RedrawRequested;
use winit::event_loop::ControlFlow;
use crate::state::State;

pub const MAXIMUM_ITERATIONS: u32 = 250;
const MAXIMUM_DISTANCE_SQUARE: f32 = 4.0;

pub fn julia_iter(z: [f32; 3], c: [f32; 2]) -> u32 {
    // todo bigdecimal
    let mut re = z[0];
    let mut im = z[1];

    let mut dist_square = re.powi(2) + im.powi(2);
    let mut it = 0;
    while it < MAXIMUM_ITERATIONS && dist_square < MAXIMUM_DISTANCE_SQUARE {
        let temp = re;
        re = temp.powi(2) - im.powi(2) + c[0];
        im = 2.0 * im * temp + c[1];

        dist_square = re * re + im * im;
        it += 1;
    }

    it
}

pub fn handle_event(mut state: &mut State, event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent { ref event, window_id }  if window_id == state.window().id() => if !state.input(event) {
            handle_window_event(&mut state, control_flow, event);
        }
        RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(SurfaceError::Lost) => state.resize(state.size()),
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        MainEventsCleared => state.window().request_redraw(),
        _ => ()
    }
}

fn handle_window_event(state: &mut State, control_flow: &mut ControlFlow, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(control_flow, input),
        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
        _ => ()
    }
}

fn handle_keyboard_input(control_flow: &mut ControlFlow, input: &KeyboardInput) {
    if let Some(key) = input.virtual_keycode {
        match key {
            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
            _ => ()
        }
    }
}
