// FIXME: These should be removed as soon as possible
// They are here only to allow for cleaner build logs. As of right now, there are many things yet to be implemented and used, and lots of warnings because of that
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

mod memory;
mod cpu;
mod chip8;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

use chip8::Chip8;

fn main() {
    let mut args = env::args();

    args.next();

    let rom_path = args.next().expect("Did not get a rom path");
    let rom_path = Path::new(&rom_path);

    let mut rom = File::open(rom_path).expect("Could not open file");
    let mut data: Vec<u8> = vec![];

    rom.read_to_end(&mut data).expect("Error reading rom");

    let mut chip8 = Chip8::new();

    chip8.load_rom(&data);
}
