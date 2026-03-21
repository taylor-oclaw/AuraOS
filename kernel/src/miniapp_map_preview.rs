extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

pub struct MiniAppMapPreview {
    map_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl MiniAppMapPreview {
    pub fn new(width: usize, height: usize) -> Self {
        let map_data = vec![0; width * height];
        MiniAppMapPreview { map_data, width, height }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        if x < self.width && y < self.height {
            self.map_data[y * self.width + x] = value;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<u8> {
        if x < self.width && y < self.height {
            Some(self.map_data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn fill(&mut self, value: u8) {
        for pixel in &mut self.map_data {
            *pixel = value;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(char::from_digit(self.map_data[y * self.width + x] as u32 % 10, 10).unwrap_or(' '));
            }
            if y < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        let mut new_data = vec![0; new_width * new_height];
        for y in 0..min(self.height, new_height) {
            for x in 0..min(self.width, new_width) {
                new_data[y * new_width + x] = self.map_data[y * self.width + x];
            }
        }
        self.map_data = new_data;
        self.width = new_width;
        self.height = new_height;
    }
}
