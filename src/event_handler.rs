use glium::glutin::event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glium::glutin::event_loop::ControlFlow;

pub fn handle_event(event: Event<()>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(event, control_flow),
        _ => ()
    }
}

fn handle_window_event(event: WindowEvent, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::CloseRequested => control_flow.set_exit(),
        WindowEvent::KeyboardInput { input, .. } => handle_keyboard_input(input, control_flow),
        _ => ()
    }
}

fn handle_keyboard_input(input: KeyboardInput, control_flow: &mut ControlFlow) {
    if let Some(key) = input.virtual_keycode {
        match key {
            // exit
            VirtualKeyCode::Escape => control_flow.set_exit(),
            _ => ()
        }
    }
}