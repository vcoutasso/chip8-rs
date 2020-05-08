mod memory;

use memory::Memory;

#[allow(non_snake_case)]
struct Register {
    // Register V0
    V0: u8,
    // Register V1
    V1: u8,
    // Register V2
    V2: u8,
    // Register V3
    V3: u8,
    // Register V4
    V4: u8,
    // Register V5
    V5: u8,
    // Register V6
    V6: u8,
    // Register V7
    V7: u8,
    // Register V8
    V8: u8,
    // Register V9
    V9: u8,
    // Register VA
    VA: u8,
    // Register VB
    VB: u8,
    // Register VC
    VC: u8,
    // Register VD
    VD: u8,
    // Register VE
    VE: u8,
    // Should not be used by programs. Mostly used as a flag
    VF: u8,
    // Stores memory addresses
    I: u16,
    // Special purpose registers. Both DT and ST decrement by one at a rate of *dec_rate*
    //
    // Delay timer. Delay timer activates whenever this register is non-zero.
    DT: u8,
    // Sound timer. Sound timer activates whenever this register is non-zero.
    // As long as ST has a value greater than zero, the Chip-8 buzzer will beep
    ST: u8,
    // Pseudo-registers below are not accessable from programs
    //
    // Program counter. Stores currently executing address
    PC: u16,
    // Stack pointer. Points to topmost level of the stack
    SP: u8,
}

fn main() {
    // Decrement rate of special purpose registers
    const dec_rate: u8 = 60;

    // The stack is used to store the address that the interpreter should return when done with a subroutine
    // 16 bytes means up to 16 levels of nested routines
    let mut stack: Vec<u16> = vec![0; 16];

    // The memory. Notable addresses:
    // 0x000 to 0x1FF - Reserved for the interpreter
    // 0x200 - Start of most Chip-8 programs
    // 0x600 - Programs intended for the ETI 660 can start at this address
    // 0xFFF - End of Chip-8 RAM
    let mut memory: Vec<u8> = vec![0; 4096];

    let ram = Memory::new();

}
