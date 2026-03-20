extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AsfOutputValidator {
    valid_outputs: Vec<String>,
}

impl AsfOutputValidator {
    pub fn new() -> Self {
        AsfOutputValidator {
            valid_outputs: Vec::new(),
        }
    }

    pub fn add_valid_output(&mut self, output: String) {
        self.valid_outputs.push(output);
    }

    pub fn is_output_valid(&self, output: &str) -> bool {
        self.valid_outputs.contains(&String::from(output))
    }

    pub fn remove_valid_output(&mut self, output: &str) {
        if let Some(index) = self.valid_outputs.iter().position(|o| o == output) {
            self.valid_outputs.remove(index);
        }
    }

    pub fn get_all_valid_outputs(&self) -> Vec<String> {
        self.valid_outputs.clone()
    }

    pub fn clear_valid_outputs(&mut self) {
        self.valid_outputs.clear();
    }
}
