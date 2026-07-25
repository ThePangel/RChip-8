use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
mod chip8;



fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Too few arguments!!".into());
    }

    let file_path = &args[1];

    let mut chip8 = chip8::Chip8::new();

    chip8.load_rom(file_path)?;

    Ok(())
}
