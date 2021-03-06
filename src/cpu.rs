use crate::chip8::PROGRAM_START;
use crate::chip8::{Address, Register};

/// This struct holds all CPU registers
struct Registers {
    /// General purpose register V0 to VF
    /// VF is normally used as a flag
    vx: [Register; 0x10],
    /// Stores memory addresses
    i: Address,
    /// Special purpose registers. Both DT and ST decrement by one at a rate of DEC_RATE [Hz]
    ///
    /// Delay timer. Delay timer activates whenever this register is non-zero.
    dt: Register,
    /// Sound timer. Sound timer activates whenever this register is non-zero.
    /// As long as ST has a value greater than zero, the Chip-8 buzzer will beep
    st: Register,
    /// pseudo-registers below are not accessable from programs
    ///
    /// Program counter. Stores currently executing address
    pc: Address,
    /// Stack pointer. Points to topmost level of the stack (Actually, here it is an index to the topmost level of the stack)
    sp: usize,
}

/// Chip-8's CPU. Handles instructions and registers
pub struct CPU {
    /// CPU's registers
    reg: Registers,
    /// The stack is used to store the address that the interpreter should return when done with a subroutine
    stack: Vec<Address>,
}

impl CPU {
    /// Creates and returns a new instance of the CPU
    pub fn new() -> CPU {
        // Initializes all registers with 0
        let reg = Registers {
            vx: [0; 0x10],
            i: 0,
            dt: 0,
            st: 0,
            pc: PROGRAM_START,
            sp: 0,
        };
        // 16 bytes means up to 16 levels of nested routines. This is the original value so I will be using it
        let stack: Vec<Address> = vec![0; 0x10];

        CPU { reg, stack }
    }

    /// Returns the value of the Vx register
    pub fn get_vx(&self, x: Register) -> u8 {
        self.reg.vx[x as usize]
    }

    /// Sets the value of the Vx register
    pub fn set_vx(&mut self, x: Register, byte: u8) {
        self.reg.vx[x as usize] = byte;
    }

    /// Returns the value of the Vx register
    pub fn add_vx(&mut self, x: Register, byte: u8) {
        self.reg.vx[x as usize] = self.reg.vx[x as usize].wrapping_add(byte);
    }

    /// Returns the value of Program Counter (PC)
    pub fn get_pc(&self) -> Address {
        self.reg.pc
    }

    /// Returns the value of the Delay Timer (DT)
    pub fn get_dt(&self) -> u8 {
        self.reg.dt
    }

    /// Returns the value of the Sound Timer (ST)
    pub fn get_st(&self) -> u8 {
        self.reg.st
    }

    /// Returns the value of the I register
    pub fn get_i(&self) -> u16 {
        self.reg.i
    }

    /// Sets the value of DT
    pub fn set_dt(&mut self, reg: Register) {
        self.reg.dt = self.get_vx(reg);
    }

    /// Sets the value of ST
    pub fn set_st(&mut self, reg: Register) {
        self.reg.st = self.get_vx(reg);
    }

    /// Sets the value of I
    pub fn set_i(&mut self, addr: Address) {
        self.reg.i = addr;
    }

    /// Returns from a subroutine, updating the value of PC and SP
    pub fn subroutine_return(&mut self) {
        self.reg.pc = self.stack[self.reg.sp];
        self.reg.sp -= 1;
    }

    /// Jumps to a memory address, updating PC value
    pub fn jump(&mut self, addr: Address) {
        // After the instruction is dealed with, pc will be incremented by 2
        // Therefore, it will land right at addr with the current value that pc is attributed
        self.reg.pc = addr - 2;
    }

    /// Calls a subroutine at a given memory address
    pub fn call(&mut self, addr: Address) {
        self.reg.sp += 1;
        self.stack[self.reg.sp] = self.reg.pc;
        self.jump(addr);
    }

    /// Updates the value of PC to skip current instruction
    pub fn skip_instruction(&mut self) {
        self.reg.pc += 2;
    }

    /// Adds the value of two registers. If the result is greater than 255 VF is set to 1, otherwise 0
    pub fn add(&mut self, reg1: Register, reg2: Register) {
        let ans = self.get_vx(reg1) as u16 + self.get_vx(reg2) as u16;
        // Check if overflow occurred
        self.set_vx(0xF, (ans > 255) as u8);
        self.set_vx(reg1, ans as u8);
    }

    /// If reg1 > reg2, then VF is set to 1, otherwise 0
    pub fn sub(&mut self, reg1: Register, reg2: Register) {
        let ans = self.get_vx(reg1).wrapping_sub(self.get_vx(reg2));
        // Check if overflow occurred
        self.set_vx(0xF, (self.get_vx(reg1) > self.get_vx(reg2)) as u8);
        self.set_vx(reg1, ans);
    }

    /// Shifts the value of reg to right
    pub fn shift_right(&mut self, reg: Register) {
        // Store lsb prior to shift
        self.set_vx(0xF, self.get_vx(reg) & 0x01);
        self.set_vx(reg, self.get_vx(reg) >> 1);
    }

    /// Shifts the value of reg to left
    pub fn shift_left(&mut self, reg: Register) {
        // Store msb prior to shift
        self.set_vx(0xF, self.get_vx(reg) >> 7);
        self.set_vx(reg, self.get_vx(reg) << 1);
    }

    /// Sets the value of I to the address of the byte sprite.
    pub fn set_sprite_i(&mut self, byte: u8) {
        // Each sprite occupies 5 bytes of memory. They are placed in order, so to get the address of a digit's first byte in memory, we can multiply its value by 5
        self.set_i((byte * 5) as u16);
    }

    /// Updates the value of DT and ST
    pub fn tick_timers(&mut self) {
        if self.reg.dt != 0 {
            self.reg.dt -= 1;
        }
        if self.reg.st != 0 {
            self.reg.st -= 1;
        }
    }
}
