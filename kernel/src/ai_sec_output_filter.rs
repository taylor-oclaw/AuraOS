extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
    let mut filter = AIFilter::new();
    filter.add_allowed_output("Hello, World!");
    filter.add_allowed_output("AI is awesome!");

    if filter.is_output_allowed("Hello, World!") {
    }

    if !filter.is_output_allowed("This should be blocked") {
    }

    let filtered_outputs = filter.get_allowed_outputs();
    for output in filtered_outputs.iter() {
    }
}

pub struct AIFilter {
    allowed_outputs: Vec<String>,
}

impl AIFilter {
    pub fn new() -> Self {
        AIFilter {
            allowed_outputs: Vec::new(),
        }
    }

    pub fn add_allowed_output(&mut self, output: &str) {
        self.allowed_outputs.push(String::from(output));
    }

    pub fn remove_allowed_output(&mut self, output: &str) {
        if let Some(index) = self.allowed_outputs.iter().position(|o| o == output) {
            self.allowed_outputs.remove(index);
        }
    }

    pub fn is_output_allowed(&self, output: &str) -> bool {
        self.allowed_outputs.contains(&String::from(output))
    }

    pub fn get_allowed_outputs(&self) -> Vec<String> {
        self.allowed_outputs.clone()
    }

    pub fn clear_allowed_outputs(&mut self) {
        self.allowed_outputs.clear();
    }
}
