mod chip8;
mod cpu;
mod display;
mod instructions;
mod memory;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use chip8::Chip8;

fn main() {
    // Info about the program and arguments
    let args = App::new("chip8")
        .version("0.1.0")
        .author("Vinícius Couto <vinicouto12@gmail.com>")
        .about("A CHIP8 emulator written in Rust")
        .arg(
            Arg::with_name("rom")
                .value_name("PATH_TO_ROM")
                .help("Path to ROM file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Get path to ROM from args
    let rom_path = Path::new(args.value_of("rom").unwrap());

    // Reference to the ROM file
    let mut rom = File::open(rom_path).expect("Could not open file");

    // Buffer to store file contents
    let mut data: Vec<u8> = vec![];

    // Reads ROM file and stores contents to data
    rom.read_to_end(&mut data).expect("Error reading ROM");

    // Creates an instance of the emulator
    let mut chip8 = Chip8::new();

    // Loads ROM to RAM
    chip8.load_rom(&data);

    // Runs ROM
    chip8.run();
}
