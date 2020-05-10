use crate::chip8::PROGRAM_START;
use crate::chip8::{Register, Address};

/// Decrement rate [Hz] of special purpose registers
pub const DEC_RATE: u8 = 60;

/// This struct holds all CPU registers
struct Registers {
    // General purpose register V0 to VF
    // VF is normally used as a flag
    vx: [Register; 0x10],
    // Stores memory addresses
    i: Address,
    // Special purpose registers. Both DT and ST decrement by one at a rate of DEC_RATE [Hz]
    //
    // Delay timer. Delay timer activates whenever this register is non-zero.
    dt: Register,
    // Sound timer. Sound timer activates whenever this register is non-zero.
    // As long as ST has a value greater than zero, the Chip-8 buzzer will beep
    st: Register,
    // pseudo-registers below are not accessable from programs
    //
    // Program counter. Stores currently executing address
    pc: Address,
    // Stack pointer. Points to topmost level of the stack
    sp: Register,
}

/// Chip-8's CPU
pub struct CPU {
    // CPU's registers
    reg: Registers,
    // The stack is used to store the address that the interpreter should return when done with a subroutine
    stack: Vec<Address>,
}

impl CPU {
    pub fn new() -> CPU {
        // Initializes all registers with 0
        let mut reg = Registers { vx: [0; 0x10], i: 0, dt: 0, st: 0, pc: PROGRAM_START, sp:0 };
        // 16 bytes means up to 16 levels of nested routines
        let mut stack: Vec<Address> = vec![0; 0x10];

        CPU { reg , stack }
    }

    pub fn program_counter(&self) -> u16 {
        self.reg.pc
    }

}
