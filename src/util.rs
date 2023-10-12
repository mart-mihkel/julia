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

pub fn match_event(mut state: &mut State, event: winit::event::Event<()>, control_flow: &mut winit::event_loop::ControlFlow) {
    match event {
        winit::event::Event::WindowEvent { ref event, window_id }  if window_id == state.window().id() => if !state.input(event) {
            match_window_event(&mut state, control_flow, event);
        }
        winit::event::Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = winit::event_loop::ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        winit::event::Event::MainEventsCleared => state.window().request_redraw(),
        _ => ()
    }
}

fn match_window_event(state: &mut State, control_flow: &mut winit::event_loop::ControlFlow, event: &winit::event::WindowEvent) {
    match event {
        winit::event::WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
        winit::event::WindowEvent::KeyboardInput { input, .. } => match_keyboard_input(control_flow, input),
        winit::event::WindowEvent::Resized(physical_size) => state.resize(*physical_size),
        winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
        _ => ()
    }
}

fn match_keyboard_input(control_flow: &mut winit::event_loop::ControlFlow, input: &winit::event::KeyboardInput) {
    if let Some(key) = input.virtual_keycode {
        match key {
            winit::event::VirtualKeyCode::Escape => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => ()
        }
    }
}
