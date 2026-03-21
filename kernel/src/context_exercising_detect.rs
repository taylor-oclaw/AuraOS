extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    let context = ContextExercisingDetect::new();
    context.initialize();
    context.log_status();
    context.update_context("New Data");
    context.log_status();
    context.reset_context();
    context.log_status();
}

pub struct ContextExercisingDetect {
    data: Vec<String>,
    status: String,
}

impl ContextExercisingDetect {
    pub fn new() -> Self {
        ContextExercisingDetect {
            data: Vec::new(),
            status: String::from("Initialized"),
        }
    }

    pub fn initialize(&mut self) {
        self.data.push(String::from("Initial Data"));
        self.status = String::from("Ready");
    }

    pub fn update_context(&mut self, new_data: &str) {
        self.data.push(String::from(new_data));
        self.status = String::from("Updated");
    }

    pub fn reset_context(&mut self) {
        self.data.clear();
        self.status = String::from("Reset");
    }

    pub fn log_status(&self) {
        for item in &self.data {
            // Simulate logging
        }
        // Simulate logging status
    }

    pub fn get_data_count(&self) -> usize {
        self.data.len()
    }
}
