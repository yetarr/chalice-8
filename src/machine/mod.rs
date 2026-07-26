use std::fs::File;
use std::io::{self, Read};
use std::time::{UNIX_EPOCH, SystemTime};

use crate::parser;

pub mod execute;

pub const INSTRUCTION_SIZE: u16 = 2;
pub const REG_VF: usize = 15;
pub const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

const LCG_A: u64 = 6364136223846793005;
const LCG_C: u64 = 1442695040888963407;

#[derive(Debug)]
pub struct Machine {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: Vec<u16>,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
    display_buf: [bool; 64 * 32],
    program_size: u16,
    rng_state: u64,
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
            display_buf: [false; 64 * 32],
            program_size: 0,
            rng_state: seed,
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

    pub fn read(&mut self) -> u16 {
        if self.pc + 1 >= self.memory.len() as u16 {
            panic!("invalid memory access")
        }
        let (high, low) = (
            self.memory[self.pc as usize] as u16,
            self.memory[self.pc as usize + 1] as u16,
        );
        self.pc += INSTRUCTION_SIZE;
        (high << 8) | low
    }

    pub fn cycle(&mut self) {
        let opcode = self.read();
        let op = parser::decode(opcode);
        self.execute(op);
    }

    pub fn run(&mut self) {
        let mut prev_pc = self.pc;
        while self.pc < 0x200 + self.program_size {
            self.cycle();
            if prev_pc == self.pc {
                break;
            }
            prev_pc = self.pc;
        }
    }

    pub fn run_with_dump(&mut self) {
        let mut prev_pc = self.pc;
        while self.pc < 0x200 + self.program_size {
            self.cycle();
            self.dump();
            if prev_pc == self.pc {
                break;
            }
            prev_pc = self.pc;
        }
    }

    pub fn dump(&self) {
        println!("PC: {}", self.pc);
        println!("Registers:\n\t{:?}", self.registers);
        println!("Stack:\n\t{:?}", self.stack);
    }

    pub fn next_random(&mut self) -> u8 {
        self.rng_state = self.rng_state.wrapping_mul(LCG_A).wrapping_add(LCG_C);
        (self.rng_state >> 24) as u8
    }
    
    pub fn print_display(&self) {
        for y in 0..32 {
            for x in 0..64 {
                let pixel = self.display_buf[y * 64 + x];
                print!("{}", if pixel { "#" } else { " " });
            }
            println!();
        }
    }
}
