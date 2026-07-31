use pixels::{Pixels, SurfaceTexture};
use rodio::Player;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{
        KeyCode::{Enter, Escape},
        PhysicalKey::{self},
    },
    window::{Window, WindowId},
};

use crate::chip8::{self, KEYMAP};

pub struct App {
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub chip8: chip8::Chip8,
    pub cycle: Instant,
    pub player: Player,
}
impl ApplicationHandler for App {
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let now = Instant::now();

        let source = rodio::source::SineWave::new(440.0);
        self.player.append(source);

        if now.duration_since(self.cycle) >= Duration::from_micros(1_000_000 / 60) {
            for _ in 0..12 {
                self.chip8.cycle();
            }

            if self.chip8.d_timer > 0 {
                self.chip8.d_timer -= 1
            }
            if self.chip8.s_timer > 0 {
                self.player.play();
                self.chip8.s_timer -= 1
            } else {
                self.player.pause();
            }
            self.cycle = now;
        }

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
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("Failed to resize surface: {err}");
                        event_loop.exit();
                    }
                }
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
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    if key_code == Escape {
                        event_loop.exit();
                    } else if key_code == Enter {
                        self.chip8.pc = 0x200;
                        self.chip8.address_register = 0;
                        self.chip8.d_timer = 0;
                        self.chip8.s_timer = 0;
                        self.chip8.address_register = 0;
                        self.chip8.registers = [0; 16];
                        self.chip8.sp = 0;
                        self.chip8.stack = [0; 12];
                        self.chip8.display_buffer = [false; 64 * 32];
                    }
                    if let Some(key_index) = KEYMAP.iter().position(|k| *k == key_code) {
                        self.chip8.keys[key_index] = event.state.is_pressed();
                    }
                }
            }

            _ => (),
        }
    }
}
