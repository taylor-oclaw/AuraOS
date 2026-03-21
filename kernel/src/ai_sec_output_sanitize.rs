extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AISecOutputSanitizer {
    sanitized_data: Vec<u8>,
    max_length: usize,
}

impl AISecOutputSanitizer {
    pub fn new(max_length: usize) -> Self {
        AISecOutputSanitizer {
            sanitized_data: Vec::new(),
            max_length,
        }
    }

    pub fn add_data(&mut self, data: &[u8]) {
        if self.sanitized_data.len() + data.len() <= self.max_length {
            self.sanitized_data.extend_from_slice(data);
        }
    }

    pub fn clear_data(&mut self) {
        self.sanitized_data.clear();
    }

    pub fn get_data(&self) -> &[u8] {
        &self.sanitized_data
    }

    pub fn is_full(&self) -> bool {
        self.sanitized_data.len() >= self.max_length
    }
}
