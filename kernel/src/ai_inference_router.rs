extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AIInferenceRouter {
    models: Vec<String>,
    current_model_index: usize,
}

impl AIInferenceRouter {
    pub fn new() -> Self {
        AIInferenceRouter {
            models: Vec::new(),
            current_model_index: 0,
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(String::from(model_name));
    }

    pub fn remove_model(&mut self, model_name: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.models.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_model(&self) -> Option<&String> {
        self.models.get(self.current_model_index)
    }

    pub fn switch_to_next_model(&mut self) {
        if !self.models.is_empty() {
            self.current_model_index = (self.current_model_index + 1) % self.models.len();
        }
    }

    pub fn list_models(&self) -> Vec<&String> {
        self.models.iter().collect()
    }
}
