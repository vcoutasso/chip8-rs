extern crate minifb;

use minifb::{Window, WindowOptions};
use crate::chip8::{WINDOW_SCALE, ORIGINAL_WIDTH};

pub struct Display {
    pub buffer: Vec<u32>,
    /// Original pixel coordinates
    pub coord: Vec<u8>,
    window: Window,
    window_width: usize,
    window_height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        let buffer = vec![0; width * height];
        let coord = vec![0; 64 * 32];

        let mut window = Window::new("CHIP-8 Emulator", width, height, WindowOptions::default()).expect("Error creating window");

        // Limit refresh rate to 60fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));

        Display { buffer, coord, window, window_width: width, window_height: height }
    }

    pub fn draw(&mut self) {
        for (i, pixel) in self.coord.iter().enumerate() {
            let x = (i % ORIGINAL_WIDTH) * WINDOW_SCALE;
            let y = (i / ORIGINAL_WIDTH) * WINDOW_SCALE;

            // Update buffer to reflect on the changes made to the original virtual window that self.coord represents
            for j in 0..WINDOW_SCALE {
                for k in 0..WINDOW_SCALE {
                    if *pixel == 1 {
                        self.buffer[ORIGINAL_WIDTH * WINDOW_SCALE * (y + j as usize) + x + k as usize] = 0x00FF_FFFF;
                    }
                    else {
                        self.buffer[ORIGINAL_WIDTH * WINDOW_SCALE * (y + j as usize) + x + k as usize] = 0x00;
                    }
                }
            }
        }

        self.window.update_with_buffer(&self.buffer, self.window_width, self.window_height).expect("Error drawing to window");
        //self.window.update();
    }

    pub fn is_window_open(&self) -> bool{
        self.window.is_open()
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0; self.window_width * self.window_height];
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        let keys = self.window.get_keys_pressed(minifb::KeyRepeat::No);
        match keys {
            Some(vec) => {
                match vec.iter().next() {
                    Some(minifb::Key::Q) => Some(0x1),
                    Some(minifb::Key::W) => Some(0x2),
                    Some(minifb::Key::E) => Some(0x3),
                    Some(minifb::Key::R) => Some(0xC),
                    Some(minifb::Key::A) => Some(0x4),
                    Some(minifb::Key::S) => Some(0x5),
                    Some(minifb::Key::D) => Some(0x6),
                    Some(minifb::Key::F) => Some(0xD),
                    Some(minifb::Key::U) => Some(0x7),
                    Some(minifb::Key::I) => Some(0x8),
                    Some(minifb::Key::O) => Some(0x9),
                    Some(minifb::Key::P) => Some(0xE),
                    Some(minifb::Key::J) => Some(0x7),
                    Some(minifb::Key::K) => Some(0x8),
                    Some(minifb::Key::L) => Some(0x9),
                    Some(minifb::Key::Semicolon) => Some(0xF),
                    _ => None,
                }
            },
            None => None,
        }
    }
}
