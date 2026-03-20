extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfErrorHandler {
    errors: Vec<String>,
}

impl AsfErrorHandler {
    pub fn new() -> Self {
        AsfErrorHandler {
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error_message: &str) {
        self.errors.push(String::from(error_message));
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    pub fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn count_errors(&self) -> usize {
        self.errors.len()
    }
}
