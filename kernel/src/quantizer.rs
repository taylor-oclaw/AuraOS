extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Quantizer {
    scale: u32,
    offset: i32,
    data: Vec<i32>,
}

impl Quantizer {
    pub fn new(scale: u32, offset: i32) -> Self {
        Quantizer {
            scale,
            offset,
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn quantize(&self) -> Vec<i32> {
        self.data.iter().map(|&x| (x + self.offset) / self.scale as i32).collect()
    }

    pub fn dequantize(&self, quantized: &[i32]) -> Vec<i32> {
        quantized.iter().map(|&x| x * self.scale as i32 - self.offset).collect()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn get_scale(&self) -> u32 {
        self.scale
    }
}
