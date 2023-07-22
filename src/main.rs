mod utils;

use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::{Event, Key, Style, VideoMode};
use crate::utils::Renderer;

fn main() {
    let max_it = 100;

    let mut c = Vector2f::new(0.2, -0.6);
    let c_increment_r = Vector2f::new(0.05, 0.0);
    let c_increment_i = Vector2f::new(0.0, 0.05);

    let window_size = Vector2u::new(1920, 1080);
    let mut window = RenderWindow::new(
        VideoMode::new(window_size.x, window_size.y, 24),
        "Julia",
        Style::FULLSCREEN,
        &Default::default(),
    );

    window.set_framerate_limit(60);
    window.set_mouse_cursor_visible(false);

    let renderer = Renderer::new(window_size, 16);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::Right, .. } => c += c_increment_r,
                Event::KeyPressed { code: Key::Left, .. } => c -= c_increment_r,
                Event::KeyPressed { code: Key::Up, .. } => c += c_increment_i,
                Event::KeyPressed { code: Key::Down, .. } => c -= c_increment_i,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        renderer.render(&mut window, c, max_it);
        window.display();
    }
}
