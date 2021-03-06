extern crate rand;

use crate::cpu::CPU;
use crate::display::Display;
use crate::instructions::Instructions;
use crate::memory::Memory;

use rand::random;
use std::thread::sleep;
use std::time::{Duration, Instant};

use rodio::Sink;

/// Type aliases
///
/// Address is used when the value refers to a position in memory
pub type Address = u16;
/// Register is used when the value refers to the name of a vx register (x ranges from 0x0 to 0xF)
pub type Register = u8;

// Constants
//
/// Program start address
pub const PROGRAM_START: u16 = 0x200;
/// Window width of the original CHIP-8.
pub const ORIGINAL_WIDTH: usize = 64;
/// Window height of the original CHIP-8.
pub const ORIGINAL_HEIGHT: usize = 32;
/// Length of the coord buffer. This value represents the amount of pixels the original CHIP-8 had
pub const COORD_LENGTH: usize = ORIGINAL_WIDTH * ORIGINAL_HEIGHT;

// The following constants cam be altered to change the emulator behavior
//
/// Scale for the window size of the emulator. Since 64x32 is too tiny of a window for today's screens, a scale is necessary
/// Enlarges the window by a factor of WINDOW_SCALE
pub const WINDOW_SCALE: usize = 8;
/// Color of the pixel. White by default
pub const PIXEL_COLOR: u32 = 0x00FF_FFFF;
/// Clock speed (instructions per second).
pub const CLOCK: u32 = 60 * 20;

/// This struct ties together all components of the emulator.
pub struct Chip8 {
    /// The memory. Notable addresses:
    /// 0x000 to 0x4F - Used to store default font sprites (0-F sequentially)
    /// 0x200 - Start of most Chip-8 programs
    /// 0xFFF - End of Chip-8 RAM
    ram: Memory,
    /// CPU. Handles registers and instructions
    cpu: CPU,
    /// User interface. Handles keyboard input and graphics output
    display: Display,
    /// Audio interface
    audio: Option<Sink>,
}

impl Chip8 {
    /// Creates and returns a new instance of the emulator.
    /// For the audio interface, the default sound device is used.
    /// If there is none, no sound will play.
    pub fn new() -> Chip8 {
        let device = rodio::default_output_device();

        Chip8 {
            ram: Memory::new(),
            cpu: CPU::new(),
            display: Display::new(
                ORIGINAL_WIDTH * WINDOW_SCALE,
                ORIGINAL_HEIGHT * WINDOW_SCALE,
            ),
            audio: match device {
                Some(device) => {
                    // If there is a sound device, create a source and add it to the sink (handle to the device)
                    let source = rodio::source::SineWave::new(440);
                    let sink = Sink::new(&device);
                    // The beep is always the same, so we create it here add it to the sink
                    sink.append(source);
                    // We must pause it to prevent it from playing right now
                    sink.pause();
                    Some(sink)
                }
                None => {
                    println!("No sound device available!");
                    None
                }
            },
        }
    }

    /// Starts the emulator and executes instructions from the provided ROM
    pub fn run(&mut self) {
        let mut timer = Instant::now();
        let period = Duration::from_secs_f32(1.0 / CLOCK as f32);

        // This is the emulator's main loop
        while self.display.is_window_open() {
            // Every cycle we run a new instruction
            self.run_next_instruction();

            if timer.elapsed().as_micros() > 16667 {
                timer = Instant::now();

                // If st value is greater than 1, play beep sound
                if self.cpu.get_st() > 1 {
                    match &self.audio {
                        Some(sink) => sink.play(),
                        None => (),
                    }
                // Else, pause beep sound. sink.pause() has no effect if it is already paused
                } else {
                    match &self.audio {
                        Some(sink) => sink.pause(),
                        None => (),
                    }
                }

                // Ticks the timers and updates display
                self.cpu.tick_timers();
                self.display.draw();
            }
            // Waits for the clock to catch up
            sleep(period);
        }
    }

    /// Loads ROM to RAM memory
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.ram.write_byte(PROGRAM_START + i as Address, *byte);
        }
    }

    /// Returns the next instruction to be executed
    fn get_next_instruction(&mut self) -> Option<Instructions> {
        let curr_pc = self.cpu.get_pc();
        // Get MSB
        let ms_byte = self.ram.read_byte(curr_pc);
        // Get LSB
        let ls_byte = self.ram.read_byte(curr_pc + 1);
        let opcode: u16 = ((ms_byte as u16) << 8) + ls_byte as u16;

        Instructions::new(opcode)
    }

    /// Runs the next instruction
    pub fn run_next_instruction(&mut self) {
        let next_inst = self.get_next_instruction();

        if let Some(inst) = next_inst {
            self.run_instruction(inst);
        }
    }

    /// This function receives a single instruction and properly executes it (according to CHIP8 techinical reference). Used by `run_next_instruction()`
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
            Instructions::And(reg1, reg2) => self
                .cpu
                .set_vx(reg1, self.cpu.get_vx(reg1) & self.cpu.get_vx(reg2)),
            Instructions::Or(reg1, reg2) => self
                .cpu
                .set_vx(reg1, self.cpu.get_vx(reg1) | self.cpu.get_vx(reg2)),
            Instructions::Xor(reg1, reg2) => self
                .cpu
                .set_vx(reg1, self.cpu.get_vx(reg1) ^ self.cpu.get_vx(reg2)),
            Instructions::Add(reg1, reg2) => self.cpu.add(reg1, reg2),
            Instructions::Sub(reg1, reg2) => self.cpu.sub(reg1, reg2),
            Instructions::ShiftRight(reg) => self.cpu.shift_right(reg),
            Instructions::ReverseSub(reg1, reg2) => self.cpu.sub(reg2, reg1),
            Instructions::ShiftLeft(reg) => self.cpu.shift_left(reg),
            Instructions::SkipIfNotEquals(reg1, reg2) => {
                if self.cpu.get_vx(reg1) != self.cpu.get_vx(reg2) {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SetI(addr) => self.cpu.set_i(addr),
            Instructions::JumpPlusV0(addr) => self.cpu.jump(addr + self.cpu.get_vx(0x0) as u16),
            Instructions::SetRandAnd(reg, byte) => self.cpu.set_vx(reg, byte & random::<u8>()),
            Instructions::Draw(reg1, reg2, n) => {
                let curr_i = self.cpu.get_i();
                let x = self.cpu.get_vx(reg1) as usize;
                let y = self.cpu.get_vx(reg2) as usize;

                self.cpu.set_vx(0xF, 0);
                for j in 0..n {
                    let byte: u8 = self.ram.read_byte(curr_i + j as u16);
                    for k in 0..8 {
                        let idx =
                            (ORIGINAL_WIDTH * (y + j as usize) + x + k as usize) % COORD_LENGTH;
                        let bit_before = self.display.coord_at(idx);
                        let bit_after = bit_before ^ (byte >> (7 - k) & 0x01);
                        self.display.set_coord(idx, bit_after);

                        if (bit_after != bit_before) && bit_before == 1 {
                            self.cpu.set_vx(0xF, 1);
                        }
                    }
                }
                self.display.map_pixels();
            }
            Instructions::SkipIfKeyPressed(reg) => {
                if self.display.get_key_pressed() == Some(self.cpu.get_vx(reg)) {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SkipIfKeyNotPressed(reg) => {
                if self.display.get_key_pressed() != Some(self.cpu.get_vx(reg)) {
                    self.cpu.skip_instruction();
                }
            }
            Instructions::SetToDelayTimer(reg) => self.cpu.set_vx(reg, self.cpu.get_dt()),
            Instructions::WaitKeyPress(reg) => {
                while {
                    self.display.update();
                    let key = self.display.get_key_pressed();
                    if let Some(pressed) = key {
                        self.cpu.set_vx(reg, pressed);
                    }

                    // Loop exit condition
                    key == None
                } {
                    sleep(Duration::from_millis(1));
                }
            }
            Instructions::SetDelayTimer(reg) => self.cpu.set_dt(reg),
            Instructions::SetSoundTimer(reg) => self.cpu.set_st(reg),
            Instructions::AddRegisterI(reg) => self
                .cpu
                .set_i((self.cpu.get_i() + self.cpu.get_vx(reg) as u16) % 0x1000),
            Instructions::SetSpriteI(reg) => self.cpu.set_sprite_i(self.cpu.get_vx(reg)),
            Instructions::BCDRepresentation(reg) => {
                let curr_i = self.cpu.get_i();
                let value = self.cpu.get_vx(reg);
                // Representation of value digit by digit
                let first_digit = value / 100;
                let second_digit = (value % 100) / 10;
                let third_digit = value % 10;

                self.ram.write_byte(curr_i, first_digit);
                self.ram.write_byte(curr_i + 1, second_digit);
                self.ram.write_byte(curr_i + 2, third_digit);
            }
            Instructions::CopyRegistersMemory(reg) => {
                let curr_i = self.cpu.get_i();
                for j in 0..=reg {
                    self.ram.write_byte(curr_i + j as u16, self.cpu.get_vx(j));
                }
            }
            Instructions::SetRegistersMemory(reg) => {
                let curr_i = self.cpu.get_i();
                for j in 0..=reg {
                    self.cpu.set_vx(j, self.ram.read_byte(curr_i + j as u16));
                }
            }
        }
        // Next instruction
        self.cpu.skip_instruction();
    }
}
