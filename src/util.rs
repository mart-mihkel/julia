use std::ops::Neg;
use wgpu::SurfaceError;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use crate::ComplexNumber;
use crate::state::State;

pub fn julia_iter(z: ComplexNumber, c: ComplexNumber, max_it: u32) -> (u32, f32) {
    // todo use bigdecimal
    // todo reduce number of multiplications via algebraic simplifications
    let mut re = z[0];
    let mut im = z[1];

    let mut modulus = (re * re + im * im).sqrt();
    let mut exp_smoothing = modulus.neg().exp();
    let mut it = 0;
    while it < max_it && modulus < 2.0 {
        let temp = re;
        re = temp * temp - im * im + c[0];
        im = 2.0 * im * temp + c[1];

        modulus = (re * re + im * im).sqrt();
        exp_smoothing += modulus.neg().exp();
        it += 1;
    }

    (it, exp_smoothing)
}

pub fn handle_event(mut state: &mut State, event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { ref event, window_id }  if window_id == state.window().id() => {
            handle_window_event(&mut state, control_flow, event);
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(SurfaceError::Lost) => state.resize(state.size()),
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared if state.use_gpu() => state.window().request_redraw(),
        _ => (),
    }
}

fn handle_window_event(state: &mut State, control_flow: &mut ControlFlow, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(control_flow, input),
        WindowEvent::MouseInput { button, state: ElementState::Pressed, .. } => handle_mouse_pressed(state, button),
        WindowEvent::CursorMoved { position, .. } => state.set_mouse_position(*position),
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

fn handle_mouse_pressed(state: &mut State, button: &MouseButton) {
    match button {
        MouseButton::Left => state.zoom_in(),
        MouseButton::Right => state.zoom_out(),
        MouseButton::Middle => state.offset_to_mouse(),
        _ => (),
    }
    state.window().request_redraw();
}
