use std::sync::{Arc, Mutex};
use sfml::graphics::{Color, Drawable, RenderStates, RenderTarget};
use sfml::system::{Vector2f, Vector2u};
use crate::utils::{Grid, ThreadPool};

pub struct Renderer {
    target_size: Vector2u,
    thread_pool: ThreadPool,
    grid: Arc<Mutex<Grid>>,
}

impl Renderer {
    pub fn new(target_size: Vector2u, n_workers: usize) -> Renderer {
        let thread_pool = ThreadPool::new(n_workers);
        let grid = Arc::new(Mutex::new(Grid::new(target_size)));

        Renderer { target_size, thread_pool, grid }
    }

    pub fn render(&self, target: &mut dyn RenderTarget, c: Vector2f, max_it: u8) {
        if !self.thread_pool.has_work() {
            self.generate(c, max_it);
        }

        let grid_guard = self.grid
            .lock()
            .expect("Renderer::render other holder panicked with locked Grid mutex");

        grid_guard.draw(target, &RenderStates::DEFAULT);
    }

    fn generate(&self, c: Vector2f, max_it: u8) {
        let target_size = self.target_size;
        let target_size_f: Vector2f = target_size.as_other();

        let slice_height = self.target_size.y / self.thread_pool.n_workers as u32;

        for i in 0..self.thread_pool.n_workers as u32 {
            let grid = Arc::clone(&self.grid);

            let y_start = i * slice_height;
            let y_end = y_start + slice_height;

            let scale = 1.0 / (target_size_f.y * 0.5);

            self.thread_pool.add_job(move || {
                for x in 0..target_size.x {
                    let zr = (x as f32 - target_size_f.x * 0.5) * scale;

                    for y in y_start..y_end {
                        let zi = (y as f32 - target_size_f.y * 0.5) * scale;

                        let it = iter(Vector2f::new(zr, zi), c, max_it);
                        let color = gradient(it, max_it);

                        let mut grid_guard = grid
                            .lock()
                            .expect("Renderer::generate other holder panicked with locked Grid mutex");

                        grid_guard.set_vertex_color(x, y, color);
                    }
                }
            });
        }
    }
}

fn iter(z: Vector2f, c: Vector2f, max_it: u8) -> u8 {
    let mut it = 0;
    let mut zr = z.x;
    let mut zi = z.y;
    let mut mod_sq = zr * zr + zi * zi;

    while mod_sq < 4.0 && it < max_it {
        let temp_zr = zr;

        zr = zr * zr - zi * zi + c.x;
        zi = 2.0 * zi * temp_zr + c.y;
        mod_sq = zr * zr + zi * zi;

        it += 1;
    }

    it
}

fn gradient(it: u8, max_it: u8) -> Color {
    let val = 255.0 * it as f32 / max_it as f32;
    let val = val as u8;

    Color::rgb(0, val, val / 2)
}
