extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_image_codec_avif_init() {
    // Initialization logic for the module
}

pub extern "C" fn rust_image_codec_avif_exit() {
    // Cleanup logic for the module
}

pub struct ImageCodecAvif {
    data: Vec<u8>,
    width: u32,
    height: u32,
    format: String,
}

impl ImageCodecAvif {
    pub fn new(data: Vec<u8>, width: u32, height: u32, format: &str) -> Self {
        ImageCodecAvif {
            data,
            width,
            height,
            format: String::from(format),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_format(&self) -> &str {
        &self.format
    }

    pub fn set_format(&mut self, format: &str) {
        self.format = String::from(format);
    }
}
