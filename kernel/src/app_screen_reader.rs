extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AppScreenReader {
    text_buffer: Vec<String>,
    current_index: usize,
}

impl AppScreenReader {
    pub fn new() -> Self {
        AppScreenReader {
            text_buffer: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_text(&mut self, text: String) {
        self.text_buffer.push(text);
    }

    pub fn get_current_text(&self) -> Option<&String> {
        if self.current_index < self.text_buffer.len() {
            Some(&self.text_buffer[self.current_index])
        } else {
            None
        }
    }

    pub fn next_text(&mut self) -> Option<&String> {
        if self.current_index + 1 < self.text_buffer.len() {
            self.current_index += 1;
            Some(&self.text_buffer[self.current_index])
        } else {
            None
        }
    }

    pub fn previous_text(&mut self) -> Option<&String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.text_buffer[self.current_index])
        } else {
            None
        }
    }

    pub fn clear_texts(&mut self) {
        self.text_buffer.clear();
        self.current_index = 0;
    }
}
