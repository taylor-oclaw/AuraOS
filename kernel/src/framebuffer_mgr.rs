extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FramebufferManager {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl FramebufferManager {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        FramebufferManager {
            width,
            height,
            buffer: vec![0; size],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.buffer[y * self.width + x])
        } else {
            None
        }
    }

    pub fn clear_screen(&mut self, color: u32) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color;
        }
    }
}
