use crate::memory::Memory;
use crate::cpu::CPU;

pub struct Chip8 {
    // The memory. Notable addresses:
    // 0x000 to 0x1FF - Reserved for the interpreter originally. In this case, only 0x00 to 0x80 are used to store default font sprites
    // 0x200 - Start of most Chip-8 programs
    // 0xFFF - End of Chip-8 RAM
    ram: Memory,
    cpu: CPU,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 { ram: Memory::new(), cpu: CPU::new() }
    }
}
