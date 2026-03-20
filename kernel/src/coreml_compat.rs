extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct CoreMLCompat {
    models: Vec<String>,
    model_data: Vec<u8>,
    current_model_index: usize,
}

impl CoreMLCompat {
    pub fn new() -> Self {
        CoreMLCompat {
            models: Vec::new(),
            model_data: Vec::new(),
            current_model_index: 0,
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(String::from(model_name));
    }

    pub fn load_model(&mut self, model_data: &[u8]) {
        self.model_data.clear();
        self.model_data.extend_from_slice(model_data);
    }

    pub fn get_current_model(&self) -> Option<&String> {
        if self.current_model_index < self.models.len() {
            Some(&self.models[self.current_model_index])
        } else {
            None
        }
    }

    pub fn switch_to_next_model(&mut self) {
        if self.current_model_index + 1 < self.models.len() {
            self.current_model_index += 1;
        }
    }

    pub fn get_model_data(&self) -> &[u8] {
        &self.model_data
    }
}
