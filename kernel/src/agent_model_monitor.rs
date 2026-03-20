extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AgentModelMonitor {
    models: Vec<String>,
    active_model: Option<usize>,
}

impl AgentModelMonitor {
    pub fn new() -> Self {
        AgentModelMonitor {
            models: Vec::new(),
            active_model: None,
        }
    }

    pub fn add_model(&mut self, model_name: &str) {
        let name = String::from(model_name);
        self.models.push(name);
    }

    pub fn remove_model(&mut self, model_index: usize) -> Option<String> {
        if model_index < self.models.len() {
            Some(self.models.remove(model_index))
        } else {
            None
        }
    }

    pub fn set_active_model(&mut self, model_index: usize) -> bool {
        if model_index < self.models.len() {
            self.active_model = Some(model_index);
            true
        } else {
            false
        }
    }

    pub fn get_active_model(&self) -> Option<&String> {
        self.active_model.map(|index| &self.models[index])
    }

    pub fn list_models(&self) -> &[String] {
        &self.models
    }
}
