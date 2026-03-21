extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshGpuShare {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl MeshGpuShare {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height * 3; // Assuming RGB format
        MeshGpuShare {
            data: vec![0; size],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * 3;
            self.data[index] = r;
            self.data[index + 1] = g;
            self.data[index + 2] = b;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * 3;
            Some((self.data[index], self.data[index + 1], self.data[index + 2]))
        } else {
            None
        }
    }

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for chunk in self.data.chunks_mut(3) {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some((r, g, b)) = self.get_pixel(x, y) {
                    result.push_str(&format!("({}, {}, {}), ", r, g, b));
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
