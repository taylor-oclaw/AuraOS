extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AgentModelRouter {
    models: Vec<String>,
}

impl AgentModelRouter {
    pub fn new() -> Self {
        AgentModelRouter {
            models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        self.models.push(String::from(model_name));
    }

    pub fn remove_model(&mut self, model_name: &str) {
        if let Some(index) = self.models.iter().position(|m| m == model_name) {
            self.models.remove(index);
        }
    }

    pub fn list_models(&self) -> Vec<String> {
        self.models.clone()
    }

    pub fn has_model(&self, model_name: &str) -> bool {
        self.models.contains(&String::from(model_name))
    }

    pub fn get_model_count(&self) -> usize {
        self.models.len()
    }
}
