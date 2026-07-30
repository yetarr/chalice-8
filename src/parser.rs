#[derive(Debug)]
pub enum Operation {
    Clear,
    SetPc(u16),
    SetI(u16),
    AddI(usize),
    SetRg(usize, u8),
    AddReg(usize, u8),
    GetFont(usize),
    SplitReg(usize),
    Arithmetic(u8, usize, usize),
    Random(usize, u8),
    SkipK(bool, usize, u8),
    SkipR(bool, usize, usize),
    SkipI(bool, usize),
    Call(u16),
    Return,
    Display(usize, usize, usize),
    CopyDelay(usize),
    WaitKey(usize),
    SetDelay(usize),
    SetSound(usize),
    Read(usize),
    Write(usize),
    None,
    Invalid(u16),
}

pub fn decode(opcode: u16) -> Operation {
    let n1 = (opcode & 0xF000) >> 12;
    let n2 = (opcode & 0x0F00) >> 8;
    let n3 = (opcode & 0x00F0) >> 4;
    let n4 = opcode & 0x000F;
    let nnn = opcode & 0x0FFF;
    let nn = (opcode & 0x00FF) as u8;

    match n1 {
        0x0 => match n4 {
            0x0 if opcode == 0x00E0 => Operation::Clear,
            0xE if opcode == 0x00EE => Operation::Return,
            _ => {
                return 
                    if opcode == 0 { Operation::None }
                    else { Operation::Invalid(opcode) }
            }
        },
        0x1 => Operation::SetPc(nnn),
        0x2 => Operation::Call(nnn),
        0x3 => Operation::SkipK(true, n2 as usize, nn),
        0x4 => Operation::SkipK(false, n2 as usize, nn),
        0x5 if n4 == 0 => Operation::SkipR(true, n2 as usize, n3 as usize),
        0x6 => Operation::SetRg(n2 as usize, nn),
        0x7 => Operation::AddReg(n2 as usize, nn),
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
                _ => Operation::Invalid(opcode),
            }
        }
        0x9 if n4 == 0 => Operation::SkipR(false, n2 as usize, n3 as usize),
        0xA => Operation::SetI(nnn),
        0xC => Operation::Random(n2 as usize, nn),
        0xD => Operation::Display(n2 as usize, n3 as usize, n4 as usize),
        0xE => match nn {
            0x9E => Operation::SkipI(true, n2 as usize),
            0xA1 => Operation::SkipI(false, n2 as usize),
            _ => Operation::Invalid(opcode),
        },
        0xF => match nn {
            0x07 => Operation::CopyDelay(n2 as usize),
            0x0A => Operation::WaitKey(n2 as usize),
            0x15 => Operation::SetDelay(n2 as usize),
            0x18 => Operation::SetSound(n2 as usize),
            0x1E => Operation::AddI(n2 as usize),
            0x29 => Operation::GetFont(n2 as usize),
            0x33 => Operation::SplitReg(n2 as usize),
            0x55 => Operation::Write(n2 as usize),
            0x65 => Operation::Read(n2 as usize),
            _ => Operation::Invalid(opcode),
        },
        _ => Operation::Invalid(opcode),
    }
}
