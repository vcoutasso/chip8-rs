use crate::chip8::{Address, Register};

use Instructions::*;

/// All the Chip-8 instructions, with the exception of 0nnn, which is ignored by modern interpreters including this one
#[derive(Debug)]
pub enum Instructions {
    /// Clear display
    ClearDisplay,
    /// Return from subroutine
    Return,
    /// Jump to Address
    Jump(Address),
    /// Call subroutine at Address
    Call(Address),
    /// Skip next instruction if value of vx equals nn
    SkipIfEqualsByte(Register, u8),
    /// Skip next instruction if value of vx is not equal to nn
    SkipIfNotEqualsByte(Register, u8),
    /// Skip next instruction if value of vx equals value of vy
    SkipIfEquals(Register, Register),
    /// Sets vx to nn
    SetRegisterByte(Register, u8),
    /// Sets vx to (vx + nn)
    AddByte(Register, u8),
    /// Sets vx to the value of vy
    SetRegister(Register, Register),
    /// Stores the result of vx | vy in vx
    Or(Register, Register),
    /// Stores the result of vx & vy in vx
    And(Register, Register),
    /// Stores the result of vx ^ vy in vx
    Xor(Register, Register),
    /// Sets vx value to (vx + vy)
    Add(Register, Register),
    /// Sets vx value to (vx - vy)
    Sub(Register, Register),
    /// Shifts vx one bit to the right
    ShiftRight(Register),
    /// Sets vx to (vy - vx)
    ReverseSub(Register, Register),
    /// Shifts vx one bit to the left
    ShiftLeft(Register),
    /// Skip next instruction if value of vx is not equal to the value of vy
    SkipIfNotEquals(Register, Register),
    /// Sets the register I to Address
    SetI(Address),
    /// Jumps to (Address + v0)
    JumpPlusV0(Address),
    /// Sets the result of (random_byte & nn) to vx
    SetRandAnd(Register, u8),
    /// Draws a sprite of width 8 and height n at coordinate (x, y)
    Draw(Register, Register, u8),
    /// Skip next instruction if key pressed equals the value of vx
    SkipIfKeyPressed(Register),
    /// Skip next instruction if key pressed is not equal to the value of vx
    SkipIfKeyNotPressed(Register),
    /// Sets vx to the value of the delay timer (dt)
    SetToDelayTimer(Register),
    /// Waits for a key press. (Blocking instruction)
    WaitKeyPress(Register),
    /// Sets delay timer (dt) to the value of vx
    SetDelayTimer(Register),
    /// Sets sound timer (st) to the value of vx
    SetSoundTimer(Register),
    /// Adds vx to i register
    AddRegisterI(Register),
    /// Sets i register to the memory address of the sprite for the character in vx
    SetSpriteI(u8),
    /// Binary-coded decimal representation. Stores the most significant digit of vx to i, the middle digit to i+1 and the last digit to i+2
    BCDRepresentation(Register),
    /// Copies the values of the registers v0 to vx(inclusive) in memory starting at the address of i
    CopyRegistersMemory(Register),
    /// Sets the values of the register v0 to vx(inclusive) with the values from memory starting at the addres of i
    SetRegistersMemory(Register),
}

impl Instructions {

    /// Transforms the raw data from the ROM to an Instruction (one instruction at a time)
    pub fn new(raw: u16) -> Option<Instructions> {
        let first_digit: u8 = ((raw >> 12) & 0xF) as u8;
        let last_digit: u8 = (raw & 0xF) as u8;
        // Least significant byte (last two digits)
        let ls_byte: u8 = (raw & 0xFF) as u8;
        // I am borrowing the terminology from Cowgod's Technical Reference for the variables below
        //
        // Last three digits. Always refer to a memory address
        let nnn: u16 = raw & 0xFFF;
        // Second most significant digit
        let x: u8 = ((raw >> 8) & 0xF) as u8;
        // Second least significant digit
        let y: u8 = ((raw >> 4) & 0xF) as u8;

        match first_digit {
            0x0 if x == 0x0 => match ls_byte {
                0xE0 => Some(ClearDisplay),
                0xEE => Some(Return),
                _ => None,
            },
            0x1 => Some(Jump(nnn)),
            0x2 => Some(Call(nnn)),
            0x3 => Some(SkipIfEqualsByte(x, ls_byte)),
            0x4 => Some(SkipIfNotEqualsByte(x, ls_byte)),
            0x5 => match last_digit {
                0x0 => Some(SkipIfEquals(x, y)),
                _ => None,
            },
            0x6 => Some(SetRegisterByte(x, ls_byte)),
            0x7 => Some(AddByte(x, ls_byte)),
            0x8 => match last_digit {
                0x0 => Some(SetRegister(x, y)),
                0x1 => Some(Or(x, y)),
                0x2 => Some(And(x, y)),
                0x3 => Some(Xor(x, y)),
                0x4 => Some(Add(x, y)),
                0x5 => Some(Sub(x, y)),
                0x6 => Some(ShiftRight(x)),
                0x7 => Some(ReverseSub(x, y)),
                0xE => Some(ShiftLeft(x)),
                _ => None,
            },
            0x9 => match last_digit {
                0x0 => Some(SkipIfNotEquals(x, y)),
                _ => None,
            },
            0xA => Some(SetI(nnn)),
            0xB => Some(JumpPlusV0(nnn)),
            0xC => Some(SetRandAnd(x, ls_byte)),
            0xD => Some(Draw(x, y, last_digit)),
            0xE => match ls_byte {
                0x9E => Some(SkipIfKeyPressed(x)),
                0xA1 => Some(SkipIfKeyNotPressed(x)),
                _ => None,
            },
            0xF => match ls_byte {
                0x07 => Some(SetToDelayTimer(x)),
                0x0A => Some(WaitKeyPress(x)),
                0x15 => Some(SetDelayTimer(x)),
                0x18 => Some(SetSoundTimer(x)),
                0x1E => Some(AddRegisterI(x)),
                0x29 => Some(SetSpriteI(x)),
                0x33 => Some(BCDRepresentation(x)),
                0x55 => Some(CopyRegistersMemory(x)),
                0x65 => Some(SetRegistersMemory(x)),
                _ => None,
            },
            _ => None,
        }
    }
}
