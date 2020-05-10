extern crate minifb;

use minifb::{Window, WindowOptions};

pub struct Display {
    buffer: Vec<u32>,
    window: Window,
    window_width: usize,
    window_height: usize,
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        let buffer = vec![0; width * height];

        let mut window = Window::new("CHIP-8 Emulator", width, height, WindowOptions::default()).expect("Error creating window");

        // Limit refresh rate to 60fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(16667)));

        Display { buffer, window, window_width: width, window_height: height }
    }

    pub fn draw(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.window_width, self.window_height).expect("Error drawing to window");
        //self.window.update();
    }

    pub fn is_window_open(&self) -> bool{
        self.window.is_open()
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0; self.window_width * self.window_height];
    }
}
