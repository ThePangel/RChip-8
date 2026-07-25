use std::{error::Error, fs::File, io::Read};

const RAM_SIZE: usize = 4096;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
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
    pub fn new() -> Self {
        let mut memory = [0u8; RAM_SIZE];

        memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        Chip8 {
            memory: (memory),
            registers: ([0; 16]),
            address_register: (0),
            stack: ([0; 12]),
            pc: (0x200),
            sp: (0),
            d_timer: (0),
            s_timer: (0),
        }
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

    pub fn cycle(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);
        self.pc += 2;

        self.exec_opcode(opcode);
    }

    fn exec_opcode(&mut self, opcode: u16) {
        let prefix = opcode >> 12;
        let x: usize = ((opcode >> 8) & 0xF).into();
        let y: usize = ((opcode >> 4) & 0xF).into();
        let end = opcode & 0xF;

        let nnn = opcode & 0xFFF;
        let nn = (opcode & 0xFF) as u8;
        let n = (opcode & 0xF) as u8;

        match (prefix, x, y, end) {
            (0x0, 0x0, 0xE, 0x0) => todo!("Screen handling"),

            (0x0, 0x0, 0xE, 0xE) => todo!("Handle return subroutine"),

            (0x1, _, _, _) => self.pc = nnn,

            (0x2, _, _, _) => todo!("handle call subroutine"),

            (0x3, _, _, _) => {
                if self.registers[x] == self.memory[nn as usize] {
                    self.pc += 1
                }
            }

            (0x4, _, _, _) => {
                if self.registers[x] != self.memory[nn as usize] {
                    self.pc += 1
                }
            }

            (0x5, _, _, _) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 1
                }
            }

            (0x6, _, _, _) => self.registers[x] = nn,

            (0x7, _, _, _) => self.registers[x] += nn,

            (0x8, _, _, 0x0) => self.registers[x] = self.registers[y],

            (0x8, _, _, 0x1) => self.registers[x] = self.registers[x] | self.registers[y],

            (0x8, _, _, 0x2) => self.registers[x] = self.registers[x] & self.registers[y],

            (0x8, _, _, 0x3) => self.registers[x] = self.registers[x] ^ self.registers[y],

            (0x9, _, _, 0x4) => match self.registers[x].checked_add(self.registers[y]) {
                Some(sum) => {
                    self.registers[x] = sum;
                    self.registers[0xF] = 0
                }
                None => {
                    self.registers[x] = self.registers[x] + self.registers[y];
                    self.registers[0xF] = 1
                }
            },

            _ => eprintln!("Unknown opcode: {:x}", opcode),
        }
    }
}
