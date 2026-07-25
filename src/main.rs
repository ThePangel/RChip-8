use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const RAM_SIZE: usize = 4096;

struct Chip8 {
    pub memory: [u8; RAM_SIZE],

    pub registers: [u8; 16],

    pub address_register: u16,

    pub stack: [u16; 12],

    pub pc: u16,

    pub sp: u8,

    pub d_timer: u8,

    pub s_timer: u8,
}

impl Chip8 {
    pub fn initialize() -> Chip8 {
        return Chip8 {
            memory: ([0; RAM_SIZE]),
            registers: ([0; 16]),
            address_register: (0),
            stack: ([0; 12]),
            pc: (0x200),
            sp: (0),
            d_timer: (0),
            s_timer: (0),
        };
    }

    pub fn load_rom(&mut self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(file_path)?;

        if file.metadata()?.len() > RAM_SIZE as u64 {

            return Err("ROM to large to fit inside the Chip8 memory".into());
        } else {
            let start_memory = &mut self.memory[0x200..];

            file.read(start_memory)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        return Err("Too few arguments!!".into());
    } 

    let file_path = &args[1];

    let mut chip8 = Chip8::initialize();

    chip8.load_rom(file_path)?;

    Ok(())
}
