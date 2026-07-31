use std::time::Instant;
use std::{env, error::Error};

use winit::event_loop::{ControlFlow, EventLoop};

mod chip8;
mod display;

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

    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());

    let mut app = display::App {
        window: None,
        pixels: None,
        chip8,
        cycle: Instant::now(),
        player,
    };

    event_loop.run_app(&mut app)?;

    handle.log_on_drop(false);

    Ok(())
}
