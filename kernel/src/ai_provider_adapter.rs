extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AIProviderAdapter {
    models: Vec<String>,
    current_model: usize,
}

impl AIProviderAdapter {
    pub fn new() -> Self {
        AIProviderAdapter {
            models: Vec::new(),
            current_model: 0,
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

    pub fn list_models(&self) -> Vec<String> {
        self.models.clone()
    }

    pub fn set_current_model(&mut self, model_name: &str) -> bool {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.current_model = index;
            true
        } else {
            false
        }
    }

    pub fn get_current_model(&self) -> Option<&String> {
        self.models.get(self.current_model)
    }
}
