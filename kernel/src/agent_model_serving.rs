extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct AgentModelServing {
    models: Vec<String>,
    current_model: Option<usize>,
}

impl AgentModelServing {
    pub fn new() -> Self {
        AgentModelServing {
            models: Vec::new(),
            current_model: None,
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
            self.current_model = Some(index);
            true
        } else {
            false
        }
    }

    pub fn get_current_model(&self) -> Option<&String> {
        self.current_model.map(|index| &self.models[index])
    }
}
