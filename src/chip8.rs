use std::{error::Error, fs::File, io::Read};

use rand::random_range;
use winit::keyboard::KeyCode;

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

pub const KEYMAP: [KeyCode; 16] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::KeyQ,
    KeyCode::KeyW,
    KeyCode::KeyE,
    KeyCode::KeyR,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::KeyF,
    KeyCode::KeyZ,
    KeyCode::KeyX,
    KeyCode::KeyC,
    KeyCode::KeyV,
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

    pub display_buffer: [bool; 64 * 32],

    pub draw_flag: bool,

    pub keys: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0u8; RAM_SIZE];

        memory[0x50..0x50 + FONTSET.len()].copy_from_slice(&FONTSET);

        Chip8 {
            memory: memory,
            registers: [0; 16],
            address_register: 0,
            stack: [0; 12],
            pc: 0x200,
            sp: 0,
            d_timer: 0,
            s_timer: 0,
            display_buffer: [false; 64 * 32],
            draw_flag: false,
            keys: [false; 16],
        }
    }

    pub fn load_rom(&mut self, file_path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(file_path)?;

        if file.metadata()?.len() > RAM_SIZE as u64 {
            return Err(("ROM to large to fit inside the Chip8 memory").into());
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
        let n: u8 = (opcode & 0xF) as u8;

        let vx = self.registers[x];
        let vy = self.registers[y];
        let i = self.address_register as usize;

        match (prefix, x, y, end) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.display_buffer = [false; 64 * 32];
                self.draw_flag = true;
            }

            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            (0x1, _, _, _) => self.pc = nnn,

            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            (0x3, _, _, _) => {
                if vx == nn {
                    self.pc += 2
                }
            }

            (0x4, _, _, _) => {
                if vx != nn {
                    self.pc += 2
                }
            }

            (0x5, _, _, _) => {
                if vx == vy {
                    self.pc += 2
                }
            }

            (0x6, _, _, _) => self.registers[x] = nn,

            (0x7, _, _, _) => self.registers[x] = self.registers[x].wrapping_add(nn),

            (0x8, _, _, 0x0) => self.registers[x] = vy,

            (0x8, _, _, 0x1) => self.registers[x] = vx | vy,

            (0x8, _, _, 0x2) => self.registers[x] = vx & vy,

            (0x8, _, _, 0x3) => self.registers[x] = vx ^ vy,

            (0x8, _, _, 0x4) => match vx.checked_add(vy) {
                Some(sum) => {
                    self.registers[x] = sum;
                    self.registers[0xF] = 0
                }
                None => {
                    self.registers[x] = vx.wrapping_add(vy);
                    self.registers[0xF] = 1
                }
            },

            (0x8, _, _, 0x5) => {
                if vx >= vy {
                    self.registers[0xF] = 1
                } else {
                    self.registers[0xF] = 0
                }

                self.registers[x] = self.registers[x].wrapping_sub(vy);
            }

            (0x8, _, _, 0x6) => {
                self.registers[0xF] = vx & 0x1;
                self.registers[x] >>= 1
            }

            (0x8, _, _, 0x7) => {
                if vy >= vx {
                    self.registers[0xF] = 1
                } else {
                    self.registers[0xF] = 0
                }

                self.registers[x] = vy - vx
            }

            (0x8, _, _, 0xE) => {
                self.registers[0xF] = vx & 0x80;
                self.registers[x] <<= 1
            }

            (0x9, _, _, 0x0) => {
                if vx != vy {
                    self.pc += 2
                }
            }

            (0xA, _, _, _) => self.address_register = nnn,

            (0xB, _, _, _) => self.pc = (self.registers[0x0] as u16).wrapping_add(nnn),

            (0xC, _, _, _) => self.registers[x] = random_range(0..=255) & nn,

            (0xD, _, _, _) => {
                let mut is_flipped = false;

                for row in 0..n {
                    for pixel in 0..8 {
                        let x = (vx as usize + pixel as usize) % 64;
                        let y = (vy as usize + row as usize) % 32;
                        let index = (y * 64) + x;

                        if self.memory[i + row as usize] & (0x80 >> pixel) != 0 {
                            if self.display_buffer[index as usize] == true {
                                is_flipped = true;
                            }

                            self.display_buffer[index as usize] ^= true
                        }
                    }
                }
                if is_flipped {
                    self.registers[0xF] = 1
                } else {
                    self.registers[0xF] = 0
                }
                self.draw_flag = true;
            }

            (0xE, _, 0x9, 0xE) => {
                if self.keys[(self.registers[x] & 0x0F) as usize] {
                    self.pc += 2
                }
            }

            (0xE, _, 0xA, 0x1) => {
                if !self.keys[(self.registers[x] & 0x0F) as usize] {
                    self.pc += 2
                }
            }

            (0xF, _, 0x0, 0x7) => self.registers[x] = self.d_timer,

            (0xF, _, 0x0, 0xA) => {
                let mut key_pressed = false;

                for (i, &value) in self.keys.iter().enumerate() {
                    if value {
                        self.registers[x] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }

                if !key_pressed {
                    self.pc -= 2;
                }
            }

            (0xF, _, 0x1, 0x5) => self.d_timer = vx,

            (0xF, _, 0x1, 0x8) => self.s_timer = vx,

            (0xF, _, 0x1, 0xE) => self.address_register += vx as u16,

            (0xF, _, 0x2, 0x9) => self.address_register = 0x50 + (5 * vx) as u16,

            (0xF, _, 0x3, 0x3) => {
                self.memory[i + 2] = vx % 10;
                self.memory[i + 1] = (vx / 10) % 10;
                self.memory[i] = vx / 100;
            }

            (0xF, _, 0x5, 0x5) => {
                for register in 0..=x {
                    self.memory[i + register] = self.registers[register]
                }
            }

            (0xF, _, 0x6, 0x5) => {
                for register in 0..=x {
                    self.registers[register] = self.memory[i + register]
                }
            }

            (0x0, 0x0, 0x0, 0x0) => return,
            _ => eprintln!("Unknown opcode: {:x}", opcode),
        }
    }
}
