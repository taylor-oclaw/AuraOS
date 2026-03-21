extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn lang_accessibility_screen_reader_lang_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn lang_accessibility_screen_reader_lang_exit() {
    // Cleanup logic for the module
}

pub struct ScreenReader {
    text_buffer: Vec<String>,
    current_index: usize,
}

impl ScreenReader {
    pub fn new() -> Self {
        ScreenReader {
            text_buffer: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_text(&mut self, text: String) {
        self.text_buffer.push(text);
    }

    pub fn read_current(&self) -> Option<&String> {
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
