use crate::chip8::{Address, Register};

use Instructions::*;

/// All the Chip-8 instructions, with the exception of 0nnn, which is ignored by modern interpreters
#[derive(Debug)]
pub enum Instructions {
    ClearDisplay,
    Return,
    Jump(Address),
    Call(Address),
    SkipIfEqualsByte(Register, u8),
    SkipIfNotEqualsByte(Register, u8),
    SkipIfEquals(Register, Register),
    SetRegisterByte(Register, u8),
    AddByte(Register, u8),
    SetRegister(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    DivTwo(Register),
    ReverseSub(Register, Register),
    MultTwo(Register),
    SkipIfNotEquals(Register, Register),
    SetI(Address),
    JumpPlusV0(Address),
    SetRandAnd(Register, u8),
    Draw(Register, Register, u8),
    SkipIfKeyPressed(Register),
    SkipIfKeyNotPressed(Register),
    SetToDelayTimer(Register),
    GetKeyPress(Register),
    SetDelayTimer(Register),
    SetSoundTimer(Register),
    AddRegisterI(Register),
    SetSpriteI(u8),
    BCDRepresentation(Register),
    CopyRegistersMemory(Register),
    SetRegistersMemory(Register),
}

impl Instructions {
    pub fn new(raw: u16) -> Option<Instructions> {
        let first_digit: u8 = ((raw >> 12) & 0xF) as u8;
        let last_digit: u8 = (raw & 0xF) as u8;
        // Least significant byte (last two digits)
        let ls_byte: u8 = (raw & 0xFF) as u8;
        // I am borrowing the terminology for variable names from Cowgod's Technical Reference
        let nnn: u16 = raw & 0xFFF;
        // Second most significant digit
        let x: u8 = ((raw >> 8) & 0xF) as u8;
        // Second least significant digit
        let y: u8 = ((raw >> 4) & 0xF) as u8;

        match first_digit {
            0x0  => {
                match ls_byte {
                    0xE0 => Some(ClearDisplay),
                    0xEE => Some(Return),
                    _ => None,
                }
            }
            0x1 => Some(Jump(nnn)),
            0x2 => Some(Call(nnn)),
            0x3 => Some(SkipIfEqualsByte(x, ls_byte)),
            0x4 => Some(SkipIfNotEqualsByte(x, ls_byte)),
            0x5 => {
                match last_digit {
                    0x0 => Some(SkipIfEquals(x, y)),
                    _ => None,
                }
            }
            0x6 => Some(SetRegisterByte(x, ls_byte)),
            0x7 => Some(AddByte(x, ls_byte)),
            0x8 => {
                match last_digit {
                    0x0 => Some(SetRegister(x, y)),
                    0x1 => Some(Or(x, y)),
                    0x2 => Some(And(x, y)),
                    0x3 => Some(Xor(x, y)),
                    0x4 => Some(Add(x, y)),
                    0x5 => Some(Sub(x, y)),
                    0x6 => Some(DivTwo(x)),
                    0x7 => Some(ReverseSub(x, y)),
                    0xE => Some(MultTwo(x)),
                    _ => None,
                }
            }
            0x9 => {
                match last_digit {
                    0x0 => Some(SkipIfNotEquals(x, y)),
                    _ => None,
                }
            }
            0xA => Some(SetI(nnn)),
            0xB => Some(JumpPlusV0(nnn)),
            0xC => Some(SetRandAnd(x, ls_byte)),
            0xD => Some(Draw(x, y, last_digit)),
            0xE => match ls_byte {
                0x9E => Some(SkipIfKeyPressed(x)),
                0xA1 => Some(SkipIfKeyNotPressed(x)),
                _ => None,
            }
            0xF => match ls_byte {
                0x07 => Some(SetToDelayTimer(x)),
                0x0A => Some(GetKeyPress(x)),
                0x15 => Some(SetDelayTimer(x)),
                0x18 => Some(SetSoundTimer(x)),
                0x1E => Some(AddRegisterI(x)),
                0x29 => Some(SetSpriteI(x)),
                0x33 => Some(BCDRepresentation(x)),
                0x55 => Some(CopyRegistersMemory(x)),
                0x65 => Some(SetRegistersMemory(x)),
                _ => None,
            }
            _ => None,
        }
    }
}
