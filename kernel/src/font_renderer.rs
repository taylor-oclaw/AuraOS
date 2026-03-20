extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FontRenderer {
    font_data: Vec<u8>,
    width: usize,
    height: usize,
}

impl FontRenderer {
    pub fn new(font_data: Vec<u8>, width: usize, height: usize) -> Self {
        FontRenderer { font_data, width, height }
    }

    pub fn get_char_width(&self) -> usize {
        self.width
    }

    pub fn get_char_height(&self) -> usize {
        self.height
    }

    pub fn render_char(&self, char_index: usize, buffer: &mut [u8]) -> bool {
        if char_index >= self.font_data.len() / (self.width * self.height) {
            return false;
        }
        let start = char_index * self.width * self.height;
        let end = start + self.width * self.height;
        buffer.copy_from_slice(&self.font_data[start..end]);
        true
    }

    pub fn render_string(&self, text: &str, buffer: &mut [u8], x_offset: usize) -> bool {
        for (i, c) in text.chars().enumerate() {
            if let Some(char_index) = c as u32 as usize - 0x20 { // Assuming ASCII starting from space
                let char_buffer_start = i * self.width * self.height;
                let char_buffer_end = char_buffer_start + self.width * self.height;
                if !self.render_char(char_index, &mut buffer[char_buffer_start..char_buffer_end]) {
                    return false;
                }
            } else {
                return false; // Character not found in font
            }
        }
        true
    }

    pub fn get_font_data(&self) -> &[u8] {
        &self.font_data
    }
}
