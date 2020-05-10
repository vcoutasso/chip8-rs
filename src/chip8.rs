use crate::memory::Memory;
use crate::cpu::CPU;

pub type Address = u16;
pub type Register = u8;

/// Program start address
pub const PROGRAM_START: u16 = 0x200;

pub struct Chip8 {
    // The memory. Notable addresses:
    // 0x000 to 0x1FF - Reserved for the interpreter originally. In this case, only 0x00 to 0x80 are used to store default font sprites
    // 0x200 - Start of most Chip-8 programs
    // 0xFFF - End of Chip-8 RAM
    ram: Memory,
    cpu: CPU,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Memory::new(), cpu: CPU::new() }
    }

    pub fn next_instruction(&mut self) {
        let opcode = self.ram.read_byte(self.cpu.program_counter() as usize);
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let curr_pc = self.cpu.program_counter();

        for (i, byte) in rom.iter().enumerate() {
            self.ram.write_byte(curr_pc as usize + i, *byte);
        }
    }
}
