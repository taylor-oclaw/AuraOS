extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct MCPErrorHandler {
    errors: Vec<String>,
}

impl MCPErrorHandler {
    pub fn new() -> Self {
        MCPErrorHandler {
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
