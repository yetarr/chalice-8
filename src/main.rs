use std::fs::File;
use std::io::{self, Read};

const INSTRUCTION_SIZE: u16 = 2;
const REG_VF: usize = 15;
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

#[derive(Debug)]
enum Operation {
    Clear,
    SetPc(u16),
    SetRg(usize, u8),
    Arithmetic(u8, usize, usize),
    SkipK(bool, usize, u8),
    SkipR(bool, usize, usize),
    Unsupported,
    None,
}

#[derive(Debug)]
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

    fn decode(&self, opcode: u16) -> Operation {
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0x0F00) >> 8;
        let n3 = (opcode & 0x00F0) >> 4;
        let n4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        match n1 {
            0x0 if opcode == 0x00E0 => Operation::Clear,
            0x1 => Operation::SetPc(nnn),
            0x3 => Operation::SkipK(true, n2 as usize, nn),
            0x4 => Operation::SkipK(false, n2 as usize, nn),
            0x5 if n4 == 0 => Operation::SkipR(true, n2 as usize, n3 as usize),
            0x6 => Operation::SetRg(n2 as usize, nn),
            0x8 => {
                let n2 = n2 as usize;
                let n3 = n3 as usize;
                match n4 {
                    0x0 => Operation::Arithmetic(0, n2, n3),
                    0x1 => Operation::Arithmetic(1, n2, n3),
                    0x2 => Operation::Arithmetic(2, n2, n3),
                    0x3 => Operation::Arithmetic(3, n2, n3),
                    0x4 => Operation::Arithmetic(4, n2, n3),
                    0x5 => Operation::Arithmetic(5, n2, n3),
                    0x6 => Operation::Arithmetic(6, n2, n3),
                    0x7 => Operation::Arithmetic(7, n2, n3),
                    0xE => Operation::Arithmetic(8, n2, n3),
                    _   => Operation::None,
                }
            },
            0x9 if n4 == 0 => Operation::SkipR(false, n2 as usize, n3 as usize),
            _   => Operation::Unsupported,
        }
    }

    fn execute(&mut self, op: Operation) {
        match op {
            Operation::Clear => self.display_buf = [false; 64 * 32],
            Operation::SetPc(x) => self.pc = x,
            Operation::SetRg(i, x) => self.registers[i] = x,
            Operation::Arithmetic(code, x, y) => {
                let (vx, vy) = (self.registers[x], self.registers[y]);
                match code {
                    0 => self.registers[x] = vy,
                    1 => self.registers[x] = vx | vy,
                    2 => self.registers[x] = vx & vy,
                    3 => self.registers[x] = vx ^ vy,
                    4 => {
                        let (res, overflow) = vx.overflowing_add(vy);
                        self.registers[REG_VF] = if overflow { 1 } else { 0 };
                        self.registers[x] = res;
                    },
                    5 => {
                        let (res, _) = vx.overflowing_sub(vy);
                        self.registers[REG_VF] = if vx >= vy { 1 } else { 0 };
                        self.registers[x] = res;
                    },
                    6 => {
                        self.registers[REG_VF] = if vx & 1 == 1 { 1 } else { 0 };
                        self.registers[x] = vx >> 1;
                    },
                    7 => {
                        let (res, _) = vy.overflowing_sub(vx);
                        self.registers[REG_VF] = if vy >= vx { 1 } else { 0 };
                        self.registers[x] = res;
                    },
                    8 => {
                        let msb = (vx >> 7) & 1; 
                        self.registers[REG_VF] = if msb == 1 { 1 } else { 0 };
                        self.registers[x] = vx.wrapping_shl(1);
                    },
                    _ => {},
                }
            },
            Operation::SkipK(use_eq, x, kk) => {
                let vx = self.registers[x];
                let skip = if use_eq { vx == kk } else { vx != kk };
                if skip {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            Operation::SkipR(use_eq, x, y) => {
                let (vx, vy) = (self.registers[x], self.registers[y]);
                let skip = if use_eq { vx == vy } else { vx != vy };
                if skip {
                    self.pc += INSTRUCTION_SIZE;
                }
            },
            Operation::Unsupported | Operation::None => {},
        };
    }
}

fn main() {
    let mut machine = Machine::new();
    machine.load("test.ch8").unwrap();
    while machine.pc < 0x200 + machine.program_size {
        let ins = machine.read();
        println!("0x{:04x}", ins);
        let op = machine.decode(ins);
        println!("{:?}", op);
        machine.execute(op);
        println!("{:?}", machine.registers);
    }

    //println!("{:?}", machine);
}
