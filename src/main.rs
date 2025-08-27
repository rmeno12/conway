use std::sync::Arc;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::EventLoop,
    window::Window,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    fn new() -> Self {
        App {
            window: None,
            pixels: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
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
                    // Kick off the redraw loop
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
                if let Err(err) = self.pixels.as_ref().unwrap().render() {
                    eprintln!("pixels.render {err}");
                    event_loop.exit();
                }
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
