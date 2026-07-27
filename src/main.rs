use std::sync::Arc;
use std::{env, error::Error};

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

mod chip8;
mod display;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    chip8: chip8::Chip8,
}
impl ApplicationHandler for App {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.chip8.cycle();

        if let Some(window) = &self.window {
            if self.chip8.draw_flag {
                window.request_redraw();
                self.chip8.draw_flag = false
            }
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("RChip-8")
                .with_inner_size(LogicalSize::new(640.0, 320.0))
                .with_min_inner_size(LogicalSize::new(128.0, 64.0));

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            let pixels = Pixels::new(64, 32, surface_texture).unwrap();

            self.window = Some(window);
            self.pixels = Some(pixels);
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing...");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let frame = self.pixels.as_mut().unwrap().frame_mut();

                for y in 0..32 {
                    for x in 0..64 {
                        let index = ((y * 64) + x) * 4;

                        let color = if self.chip8.display_buffer[index / 4] {
                            [255, 255, 255, 255]
                        } else {
                            [0, 0, 0, 255]
                        };
                        frame[index..index + 4].copy_from_slice(&color);
                    }
                }

                if let Err(err) = self.pixels.as_mut().unwrap().render() {
                    eprintln!("render failed: {err}");
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err(("Too few arguments!!").into());
    }

    let file_path = &args[1];

    let mut chip8 = chip8::Chip8::new();

    chip8.load_rom(file_path)?;

    let mut app = App {
        window: None,
        pixels: None,
        chip8,
    };

    event_loop.run_app(&mut app)?;

    Ok(())
}
