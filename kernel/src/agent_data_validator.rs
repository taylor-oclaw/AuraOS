extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentDataValidator {
    data: Vec<u8>,
}

impl AgentDataValidator {
    pub fn new(data: Vec<u8>) -> Self {
        AgentDataValidator { data }
    }

    pub fn validate_length(&self, min_len: usize, max_len: usize) -> bool {
        self.data.len() >= min_len && self.data.len() <= max_len
    }

    pub fn contains_valid_characters(&self) -> bool {
        for &byte in &self.data {
            if !byte.is_ascii_alphanumeric() && byte != b' ' {
                return false;
            }
        }
        true
    }

    pub fn count_whitespace(&self) -> usize {
        self.data.iter().filter(|&&b| b == b' ').count()
    }

    pub fn extract_substring(&self, start: usize, end: usize) -> Option<String> {
        if start <= end && end <= self.data.len() {
            let sub_data = &self.data[start..end];
            Some(String::from_utf8_lossy(sub_data).into())
        } else {
            None
        }
    }

    pub fn replace_character(&mut self, old_char: u8, new_char: u8) {
        for byte in &mut self.data {
            if *byte == old_char {
                *byte = new_char;
            }
        }
    }
}
