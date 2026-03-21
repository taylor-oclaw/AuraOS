extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut braille_input = AccessBrailleInput::new();
    braille_input.add_character('a');
    braille_input.add_character('b');
    braille_input.add_character('c');
}

pub struct AccessBrailleInput {
    input_buffer: Vec<char>,
    max_length: usize,
}

impl AccessBrailleInput {
    pub fn new() -> Self {
        AccessBrailleInput {
            input_buffer: Vec::new(),
            max_length: 100, // Example max length
        }
    }

    pub fn add_character(&mut self, character: char) {
        if self.input_buffer.len() < self.max_length {
            self.input_buffer.push(character);
        }
    }

    pub fn remove_last_character(&mut self) {
        if !self.input_buffer.is_empty() {
            self.input_buffer.pop();
        }
    }

    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
    }

    pub fn get_current_input(&self) -> String {
        let mut input_string = String::new();
        for &c in &self.input_buffer {
            input_string.push(c);
        }
        input_string
    }

    pub fn set_max_length(&mut self, length: usize) {
        if length > 0 {
            self.max_length = length;
        }
    }
}
