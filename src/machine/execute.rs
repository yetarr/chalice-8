use crate::machine::{INSTRUCTION_SIZE, Machine, REG_VF};
use crate::parser::Operation;

impl Machine {
    pub fn execute(&mut self, op: Operation) {
        match op {
            Operation::Clear => self.display_buf = [false; 64 * 32],
            Operation::SetPc(x) => self.pc = x,
            Operation::SetI(x) => self.i = x,
            Operation::SetRg(i, x) => self.registers[i] = x,
            Operation::AddReg(x, kk) => {
                let vx = self.registers[x];
                let res = vx.wrapping_add(kk);
                self.registers[x] = res;
            },
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
                    }
                    5 => {
                        let (res, _) = vx.overflowing_sub(vy);
                        self.registers[REG_VF] = if vx >= vy { 1 } else { 0 };
                        self.registers[x] = res;
                    }
                    6 => {
                        self.registers[REG_VF] = if vx & 1 == 1 { 1 } else { 0 };
                        self.registers[x] = vx >> 1;
                    }
                    7 => {
                        let (res, _) = vy.overflowing_sub(vx);
                        self.registers[REG_VF] = if vy >= vx { 1 } else { 0 };
                        self.registers[x] = res;
                    }
                    8 => {
                        let msb = (vx >> 7) & 1;
                        self.registers[REG_VF] = if msb == 1 { 1 } else { 0 };
                        self.registers[x] = vx.wrapping_shl(1);
                    }
                    _ => {}
                }
            },
            Operation::Random(x, kk) => {
                let rnd = self.next_random();
                self.registers[x] = rnd & kk;
            },
            Operation::SkipK(use_eq, x, kk) => {
                let vx = self.registers[x];
                let skip = if use_eq { vx == kk } else { vx != kk };
                if skip {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            Operation::SkipR(use_eq, x, y) => {
                let (vx, vy) = (self.registers[x], self.registers[y]);
                let skip = if use_eq { vx == vy } else { vx != vy };
                if skip {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            Operation::Call(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Operation::Display(x, y, n) => {
                self.registers[REG_VF] = 0;
                let bytes = &self.memory[self.i as usize..self.i as usize + n];
                for (row, byte) in bytes.iter().enumerate() {
                    for bit_pos in 0..8 {
                        let px = (x as usize + bit_pos) % 64;
                        let py = (y as usize + row) % 32;
                        let pos = py * 64 + px;
                        let sprite_pixel = ((byte >> (7 - bit_pos)) & 1) == 1;
                        let prev = self.display_buf[pos];
                        self.display_buf[pos] ^= sprite_pixel;
                        if prev && !self.display_buf[pos] {
                            self.registers[REG_VF] = 1;
                        }
                    }
                }
            }
            Operation::Return => self.pc = self.stack.pop().expect("return with empty stack"),
            Operation::Unsupported | Operation::None => {}
        };
    }
}
