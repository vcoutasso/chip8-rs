extern crate rand;

use crate::memory::Memory;
use crate::cpu::CPU;
use crate::display::Display;
use crate::instructions::Instructions;

use rand::random;

/// Type aliases
///
/// Address is used when the value refers to a position in memory
pub type Address = u16;
/// Register is used when the value refers to the name of a vx register (x ranges from 0x0 to 0xF)
pub type Register = u8;

/// Constants
///
/// Program start address
pub const PROGRAM_START: u16 = 0x200;
/// Window width. Original value is 64, but since it is too tiny I chose 1024 (pixels are 16 times bigger)
pub const WINDOW_WIDTH: usize = 1024;
/// Window height. Original value is 32, scaled to match with WINDOW_WIDTH
pub const WINDOW_HEIGHT: usize = 512;

pub struct Chip8 {
    // The memory. Notable addresses:
    // 0x000 to 0x1FF - Reserved for the interpreter originally. In this case, only 0x00 to 0x80 are used to store default font sprites
    // 0x200 - Start of most Chip-8 programs
    // 0xFFF - End of Chip-8 RAM
    ram: Memory,
    cpu: CPU,
    display: Display,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Memory::new(), cpu: CPU::new() , display: Display::new(WINDOW_WIDTH, WINDOW_HEIGHT) }
    }

    pub fn run(&mut self) {
        while self.display.is_window_open() {
            self.run_next_instruction();
            self.display.draw();
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let curr_pc = self.cpu.get_pc();

        for (i, byte) in rom.iter().enumerate() {
            self.ram.write_byte(curr_pc + i as Address, *byte);
        }
    }

    fn get_next_instruction(&mut self) -> Option<Instructions> {
        let curr_pc = self.cpu.get_pc();
        let ms_byte = self.ram.read_byte(curr_pc);
        let ls_byte = self.ram.read_byte(curr_pc + 1);
        let opcode: u16 = ((ms_byte as u16) << 8) + ls_byte as u16;

        Instructions::new(opcode)
    }

    pub fn run_next_instruction(&mut self) {
        let next_inst = self.get_next_instruction();

        if let Some(inst) = next_inst {
            self.run_instruction(inst);
        }
    }

    fn run_instruction(&mut self, inst: Instructions) {
        match inst {
            Instructions::ClearDisplay => self.display.clear(),
            Instructions::Return => self.cpu.subroutine_return(),
            Instructions::Jump(addr) => self.cpu.jump(addr),
            Instructions::Call(addr) => self.cpu.call(addr),
            Instructions::SkipIfEqualsByte(reg, byte) => {
                if self.cpu.get_vx(reg) == byte {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SkipIfNotEqualsByte(reg, byte) => {
                if self.cpu.get_vx(reg) != byte {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SkipIfEquals(reg1, reg2) => {
                if self.cpu.get_vx(reg1) == self.cpu.get_vx(reg2) {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SetRegisterByte(reg, byte) => self.cpu.set_vx(reg, byte),
            Instructions::AddByte(reg, byte) => self.cpu.add_vx(reg, byte),
            Instructions::SetRegister(reg1, reg2) => self.cpu.set_vx(reg1, self.cpu.get_vx(reg2)),
            Instructions::And(reg1, reg2) => self.cpu.set_vx(reg1, self.cpu.get_vx(reg1) | self.cpu.get_vx(reg2)),
            Instructions::Or(reg1, reg2) => self.cpu.set_vx(reg1, self.cpu.get_vx(reg1) & self.cpu.get_vx(reg2)),
            Instructions::Xor(reg1, reg2) => self.cpu.set_vx(reg1, self.cpu.get_vx(reg1) ^ self.cpu.get_vx(reg2)),
            Instructions::Add(reg1, reg2) => self.cpu.add(reg1, reg2),
            Instructions::Sub(reg1, reg2) => self.cpu.sub(reg1, reg2),
            Instructions::ShiftRight(reg) => self.cpu.shift_right(reg),
            Instructions::ReverseSub(reg1, reg2) => self.cpu.sub(reg2, reg1),
            Instructions::ShiftLeft(reg) => self.cpu.shift_left(reg),
            Instructions::SkipIfNotEquals(reg1, reg2) => {
                if self.cpu.get_vx(reg1) != self.cpu.get_vx(reg2) {
                    self.cpu.skip_instruction();
                }
            },
            Instructions::SetI(addr) => self.cpu.set_i(addr),
            Instructions::JumpPlusV0(addr) => self.cpu.jump(addr + self.cpu.get_vx(0x0) as u16),
            Instructions::SetRandAnd(reg, byte) => self.cpu.set_vx(reg, byte & random::<u8>()),
            Instructions::Draw(reg1, reg2, n) => {},
            Instructions::SkipIfKeyPressed(reg) => {},
            Instructions::SkipIfKeyNotPressed(reg) => {},
            Instructions::SetToDelayTimer(reg) => self.cpu.set_vx(reg, self.cpu.get_dt()),
            Instructions::GetKeyPress(reg) => {},
            Instructions::SetDelayTimer(reg) => self.cpu.set_dt(reg),
            Instructions::SetSoundTimer(reg) => self.cpu.set_st(reg),
            Instructions::AddRegisterI(reg) => self.cpu.set_i(self.cpu.get_i() + self.cpu.get_vx(reg) as u16),
            Instructions::SetSpriteI(byte) => self.cpu.set_sprite_i(byte),
            Instructions::BCDRepresentation(reg) => {
                let curr_i = self.cpu.get_i();
                let value = self.cpu.get_vx(reg);
                // Representation of value digit by digit
                let first_digit = (value / 100) % 10;
                let second_digit = (value / 10) % 10;
                let third_digit = value % 10;

                self.ram.write_byte(curr_i, first_digit);
                self.ram.write_byte(curr_i + 1, second_digit);
                self.ram.write_byte(curr_i + 2, third_digit);
            },
            Instructions::CopyRegistersMemory(reg) => {
                let curr_i = self.cpu.get_i();
                for j in 0..=reg {
                    self.ram.write_byte(curr_i + j as u16, self.cpu.get_vx(j));
                }
            },
            Instructions::SetRegistersMemory(reg) => {
                let curr_i = self.cpu.get_i();
                for j in 0..=reg {
                    self.cpu.set_vx(j, self.ram.read_byte(curr_i + j as u16));
                }
            },
        }
        // Next instruction
        self.cpu.skip_instruction();
    }
}
