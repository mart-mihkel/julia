use wgpu::SurfaceError;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, MouseScrollDelta, TouchPhase, VirtualKeyCode, WindowEvent};
use winit::event_loop::ControlFlow;
use crate::state::State;

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
        Event::MainEventsCleared => state.window().request_redraw(),
        _ => (),
    }
}

fn handle_window_event(state: &mut State, control_flow: &mut ControlFlow, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(state, control_flow, input),
        WindowEvent::MouseWheel { delta, phase: TouchPhase::Moved, .. } => handle_mouse_scroll(state, delta),
        WindowEvent::MouseInput { button, state: ElementState::Pressed, .. } => handle_mouse_pressed(state, button),
        WindowEvent::CursorMoved { position, .. } => state.set_mouse_position(*position),
        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
        _ => ()
    }
}

fn handle_keyboard_input(state: &mut State, control_flow: &mut ControlFlow, input: &KeyboardInput) {
    if let Some(key) = input.virtual_keycode {
        match key {
            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
            VirtualKeyCode::Up => state.scaled_add_im_constant(0.01),
            VirtualKeyCode::Down => state.scaled_add_im_constant(-0.01),
            VirtualKeyCode::Left => state.scaled_add_re_constant(-0.01),
            VirtualKeyCode::Right => state.scaled_add_re_constant(0.01),
            _ => ()
        }
    }
}

fn handle_mouse_scroll(state: &mut State, delta: &MouseScrollDelta) {
    match delta {
        MouseScrollDelta::LineDelta(_, lines_vertical) => state.zoom(*lines_vertical),
        MouseScrollDelta::PixelDelta(_) => (), // todo touchpad support
    }
}

fn handle_mouse_pressed(state: &mut State, button: &MouseButton) {
    match button {
        MouseButton::Left => state.offset_to_mouse(),
        _ => (),
    }
}
