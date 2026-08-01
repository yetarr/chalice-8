use std::fs::File;
use std::io::{self, Read};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::parser;

pub mod execute;

pub const INSTRUCTION_SIZE: u16 = 2;
pub const REG_VF: usize = 15;
pub const FONT_ADDR_START: u16 = 0x50;
pub const USABLE_ADDR_START: u16 = 0x200;
pub const CHAR_SIZE: u16 = 5;
pub const DISPLAY_PIXELS: usize = 64 * 32;
pub const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

const LCG_A: u64 = 6364136223846793005;
const LCG_C: u64 = 1442695040888963407;

const SYNC_FREQ: f64 = 60.0;

#[derive(Debug)]
pub struct Machine {
    pub memory: [u8; 4096],
    registers: [u8; 16],
    stack: Vec<u16>,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
    last_sync: Instant,
    display_buf: [bool; DISPLAY_PIXELS],
    program_size: u16,
    rng_state: u64,
    keys: [bool; 16],
    waiting_for_key: Option<usize>,
}

impl Machine {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let mut machine = Machine {
            memory: [0; 4096],
            registers: [0; 16],
            stack: Vec::new(),
            i: 0,
            pc: 0x200,
            dt: 0,
            st: 0,
            last_sync: Instant::now(),
            display_buf: [false; DISPLAY_PIXELS],
            program_size: 0,
            rng_state: seed,
            keys: [false; 16],
            waiting_for_key: None,
        };

        machine.memory[0x50..0x50 + 80].copy_from_slice(&FONT);
        machine
    }

    pub fn load(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        self.memory[0x200..0x200 + contents.len()].copy_from_slice(&contents);
        self.program_size = contents.len() as u16;
        Ok(())
    }

    fn read(&mut self) -> Option<u16> {
        if self.pc + 1 >= USABLE_ADDR_START + self.program_size {
            return None;
        }
        let (high, low) = (
            self.memory[self.pc as usize] as u16,
            self.memory[self.pc as usize + 1] as u16,
        );
        self.pc += INSTRUCTION_SIZE;
        Some((high << 8) | low)
    }

    pub fn set_key(&mut self, key: &str, pressed: bool) {
        if let Some(key) = parser::key_to_chip8(key) {
            self.keys[key] = pressed;
        }
    }

    pub fn key_pressed(&mut self, key: &str) {
        if let Some(chip8_key) = parser::key_to_chip8(key) {
            if let Some(reg) = self.waiting_for_key {
                self.registers[reg] = chip8_key as u8;
                self.waiting_for_key = None;
            } else {
                self.set_key(key, true);
            }
        }
    }

    pub fn get_active_key(&self) -> Option<u8> {
        for (i, pressed) in self.keys.iter().enumerate() {
            if *pressed {
                return Some(i as u8);
            }
        }

        None
    }

    pub fn sync_timers(&mut self) {
        let elapsed = self.last_sync.elapsed();
        if elapsed.as_secs_f64() >= 1.0 / SYNC_FREQ {
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                self.st -= 1;
            }
            self.last_sync = Instant::now();
        }
    }

    pub fn cycle(&mut self) {
        if self.waiting_for_key.is_some() {
            return;
        }

        if let Some(op) = self.read() {
            let op = parser::decode(op);
            self.execute(op);
        }
    }

    pub fn dump(&self) {
        println!("------------------ DUMP ------------------");
        println!("PC: {}", self.pc);
        println!("I: {}", self.i);
        println!("DT: {}", self.dt);
        println!("ST: {}", self.st);
        println!("Registers:\n\t{:?}", self.registers);
        println!("Stack:\n\t{:?}", self.stack);
        println!("Keys:\n\t{:?}", self.keys);
    }

    fn next_random(&mut self) -> u8 {
        self.rng_state = self.rng_state.wrapping_mul(LCG_A).wrapping_add(LCG_C);
        (self.rng_state >> 24) as u8
    }

    pub fn display_buf(&self) -> &[bool; 64 * 32] {
        &self.display_buf
    }
}
