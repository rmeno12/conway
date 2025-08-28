use std::sync::Arc;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct Conway {
    cells: Vec<bool>,
    scratch_cells: Vec<bool>,
}

impl Conway {
    fn new() -> Self {
        let mut empty_cells = Vec::with_capacity((WIDTH * HEIGHT).try_into().unwrap());
        empty_cells.resize(empty_cells.capacity(), false);
        Self {
            cells: empty_cells.clone(),
            scratch_cells: empty_cells,
        }
    }

    fn new_random() -> Self {
        let mut rng = rand::rng();
        let mut con = Self::new();
        for c in con.cells.iter_mut() {
            *c = rng.random_bool(0.3);
        }
        con
    }

    fn update(&mut self) {
        debug_assert_eq!(self.cells.len(), self.scratch_cells.len());
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = self.neighbors(x.try_into().unwrap(), y.try_into().unwrap());
                let idx: usize = (x + WIDTH * y).try_into().unwrap();
                self.scratch_cells[idx] = Self::next_cell_state(self.cells[idx], n);
            }
        }
        std::mem::swap(&mut self.cells, &mut self.scratch_cells);
    }

    fn draw(&self, frame: &mut [u8]) {
        for (c, pix) in self.cells.iter().zip(frame.chunks_exact_mut(4)) {
            let color = if *c {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0, 0, 0, 0xff]
            };
            pix.copy_from_slice(&color);
        }
    }

    fn neighbors(&self, x: i32, y: i32) -> u32 {
        let mut n = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let (nx, ny) = (x + dx, y + dy);
                if 0 <= nx
                    && nx < WIDTH.try_into().unwrap()
                    && 0 <= ny
                    && ny < HEIGHT.try_into().unwrap()
                    && self.cells[TryInto::<usize>::try_into(
                        nx + ny * TryInto::<i32>::try_into(WIDTH).unwrap(),
                    )
                    .unwrap()]
                {
                    n += 1;
                }
            }
        }
        n
    }

    fn next_cell_state(alive: bool, neighbors: u32) -> bool {
        if alive {
            2 <= neighbors && neighbors <= 3
        } else {
            neighbors == 3
        }
    }
}

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    conway: Conway,
}

impl App {
    fn new() -> Self {
        App {
            window: None,
            pixels: None,
            conway: Conway::new_random(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(2.0 * WIDTH as f64, 2.0 * HEIGHT as f64);
            Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("Hello Pixels")
                            .with_inner_size(size)
                            .with_min_inner_size(size),
                    )
                    .unwrap(),
            )
        };

        self.window = Some(window.clone());
        self.pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            match Pixels::new(WIDTH, HEIGHT, surface_texture) {
                Ok(pixels) => {
                    window.request_redraw();
                    Some(pixels)
                }
                Err(err) => {
                    eprintln!("pixels::new {err}");
                    event_loop.exit();
                    None
                }
            }
        };
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("close button");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.conway.update();
                self.conway.draw(self.pixels.as_mut().unwrap().frame_mut());
                if let Err(err) = self.pixels.as_ref().unwrap().render() {
                    eprintln!("pixels.render {err}");
                    event_loop.exit();
                }
                std::thread::sleep(std::time::Duration::from_millis(33));
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new().unwrap();

    let mut conway = App::new();
    event_loop.run_app(&mut conway)?;

    Ok(())
}
