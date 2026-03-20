extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct PngCodec {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl PngCodec {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize; // RGBA
        PngCodec {
            width,
            height,
            data: vec![0; size],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            self.data[index] = r;
            self.data[index + 1] = g;
            self.data[index + 2] = b;
            self.data[index + 3] = a;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            Some((self.data[index], self.data[index + 1], self.data[index + 2], self.data[index + 3]))
        } else {
            None
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
