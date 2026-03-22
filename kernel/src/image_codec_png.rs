extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Png {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Png {
    pub fn new(width: u32, height: u32) -> Self {
        Png { width, height, data: Vec::new() }
    }

    pub fn add_pixel(&mut self, r: u8, g: u8, b: u8) {
        let pixel = [r, g, b];
        self.data.extend_from_slice(&pixel);
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn save_to_file(&self, filename: String) {
        let mut file = std::fs::File::create(filename).unwrap();
        for byte in self.data.iter() {
            file.write(byte).unwrap();
        }
    }
}