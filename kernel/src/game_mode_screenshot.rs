extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct GameModeScreenshot {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl GameModeScreenshot {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_count = width * height;
        let pixels = vec![0u32; pixel_count];
        GameModeScreenshot { width, height, pixels }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.pixels[y * self.width + x])
        } else {
            None
        }
    }

    pub fn clear_screen(&mut self, color: u32) {
        for pixel in &mut self.pixels {
            *pixel = color;
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}
