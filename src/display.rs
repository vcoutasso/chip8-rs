extern crate minifb;

use crate::chip8::{ORIGINAL_HEIGHT, ORIGINAL_WIDTH, PIXEL_COLOR, WINDOW_SCALE};
use minifb::{Window, WindowOptions};

/// Interface between user and the CHIP8
/// It handles graphics and keyboard input
pub struct Display {
    /// Buffer with pixel values of displayed window. The coordinates of coord are maped here according to WINDOW_SCALE
    buffer: Vec<u32>,
    /// Original pixel coordinates
    coord: Vec<u8>,
    /// Stack to keep track of changes made do the screen. This allows for much faster scaling of the original window size
    changes_stack: Vec<usize>,
    /// Window that displays the graphics and handles input (keyboard)
    window: Window,
    /// Window width. Product of ORIGINAL_WIDTH (64) and WINDOW_SCALE
    window_width: usize,
    /// Window height. Product of ORIGINAL_HEIGHT (32) and WINDOW_SCALE
    window_height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        let buffer = vec![0; width * height];
        let coord = vec![0; 64 * 32];
        let changes_stack = vec![];

        let window = Window::new("CHIP-8 Emulator", width, height, WindowOptions::default())
            .expect("Error creating window");

        Display {
            buffer,
            coord,
            changes_stack,
            window,
            window_width: width,
            window_height: height,
        }
    }

    pub fn coord_at(&mut self, idx: usize) -> u8 {
        self.coord[idx]
    }

    pub fn set_coord(&mut self, idx: usize, bit: u8) {
        self.coord[idx] = bit;
        self.changes_stack.push(idx);
    }

    pub fn draw(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.window_width, self.window_height)
            .expect("Error drawing to window");
    }

    pub fn update(&mut self) {
        self.window.update();
    }

    pub fn map_pixels(&mut self) {
        while let Some(i) = self.changes_stack.pop() {
            let x = (i % ORIGINAL_WIDTH) * WINDOW_SCALE;
            let y = (i / ORIGINAL_WIDTH) * WINDOW_SCALE;

            // Update buffer to reflect on the changes made to the original virtual window that self.coord represents
            for j in 0..WINDOW_SCALE {
                for k in 0..WINDOW_SCALE {
                    let idx = (ORIGINAL_WIDTH * WINDOW_SCALE * (y + j as usize) + x + k as usize)
                        % (ORIGINAL_WIDTH * WINDOW_SCALE * ORIGINAL_HEIGHT * WINDOW_SCALE);
                    self.buffer[idx] = if self.coord[i] == 1 { PIXEL_COLOR } else { 0x0 };
                }
            }
        }
    }

    pub fn is_window_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape)
    }

    pub fn clear(&mut self) {
        self.coord = vec![0; self.coord.len()];
        self.buffer = vec![0; self.buffer.len()];
        self.draw()
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        let keys = self.window.get_keys_pressed(minifb::KeyRepeat::Yes);
        match keys {
            Some(vec) => match vec.iter().next() {
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
            },
            None => None,
        }
    }
}
