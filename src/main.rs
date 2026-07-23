use std::fs::File;
use std::io::{self, Read};

const INSTRUCTION_SIZE: u16 = 2;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];

struct Machine {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: Vec<u16>,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
    display_buf: [bool; 64 * 32],
    program_size: u16,
}

impl Machine {
    fn new() -> Self {
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
        };

        machine.memory[0x50..0x50 + 80].copy_from_slice(&FONT);
        machine
    }

    fn load(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        self.memory[0x200..0x200 + contents.len()].copy_from_slice(&contents);
        self.program_size = contents.len() as u16;
        Ok(())
    }

    fn read(&mut self) -> u16 {
        if self.pc + 1 >= self.memory.len() as u16 {
            panic!("invalid memory access")
        }

        let (high, low) = (self.memory[self.pc as usize] as u16, self.memory[self.pc as usize + 1] as u16);
        self.pc += INSTRUCTION_SIZE;
        (high << 8) | low
    }
}

fn main() {
    let mut machine = Machine::new();
    machine.load("test.ch8").unwrap();
    while machine.pc < 0x200 + machine.program_size {
        let ins = machine.read();
        println!("0x{:04x}", ins);
    }
}
