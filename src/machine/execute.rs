use crate::machine::{self, DISPLAY_PIXELS, INSTRUCTION_SIZE, Machine, REG_VF};
use crate::parser::Operation;

impl Machine {
    pub fn execute(&mut self, op: Operation) {
        match op {
            Operation::Clear => self.display_buf = [false; DISPLAY_PIXELS],
            Operation::SetPc(x) => self.pc = x,
            Operation::SetI(x) => self.i = x,
            Operation::AddI(x) => {
                let vx = self.registers[x];
                let res = self.i.wrapping_add(vx as u16);
                self.i = res;
            }
            Operation::SetRg(i, x) => self.registers[i] = x,
            Operation::AddReg(x, kk) => {
                let vx = self.registers[x];
                let res = vx.wrapping_add(kk);
                self.registers[x] = res;
            }
            Operation::SplitReg(x) => {
                let vx = self.registers[x];
                self.memory[self.i as usize] = vx / 100;
                self.memory[self.i as usize + 1] = (vx / 10) % 10;
                self.memory[self.i as usize + 2] = vx % 10;
            }
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
            }
            Operation::Random(x, kk) => {
                let rnd = self.next_random();
                self.registers[x] = rnd & kk;
            }
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
            Operation::SkipI(con_key_down, x) => {
                let vx = self.registers[x] as usize;
                let key_pressed = self.keys[vx];
                let skip = if con_key_down {
                    key_pressed
                } else {
                    !key_pressed
                };
                if skip {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            Operation::Call(nnn) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            Operation::GetFont(x) => {
                let vx = self.registers[x];
                let addr = machine::FONT_ADDR_START + (machine::CHAR_SIZE * vx as u16);
                self.i = addr;
            }
            Operation::Display(x, y, n) => {
                let (vx, vy) = (self.registers[x], self.registers[y]);
                self.registers[REG_VF] = 0;
                let bytes = &self.memory[self.i as usize..self.i as usize + n];
                for (row, byte) in bytes.iter().enumerate() {
                    for bit_pos in 0..8 {
                        let px = (vx as usize + bit_pos) % 64;
                        let py = (vy as usize + row) % 32;
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
            Operation::Write(x) => {
                for reg in 0..=x {
                    self.memory[self.i as usize + reg] = self.registers[reg];
                }
            }
            Operation::Read(x) => {
                for reg in 0..=x {
                    self.registers[reg] = self.memory[self.i as usize + reg];
                }
            }
            Operation::Return => self.pc = self.stack.pop().expect("return with empty stack"),
            Operation::CopyDelay(x) => self.registers[x] = self.dt,
            Operation::WaitKey(x) => {
                if !self.keys.iter().any(|k| *k) {
                    self.halt = true
                } else {
                    self.halt = false;
                    self.registers[x] = self.get_active_key().unwrap();
                }
            }
            Operation::SetDelay(x) => self.dt = self.registers[x],
            Operation::SetSound(x) => self.st = self.registers[x],
            Operation::Invalid(ins) => panic!("invalid instruction: {ins}"),
            Operation::None => {}
        };
    }
}
