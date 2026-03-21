extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OfflineAILocal {
    model_name: String,
    data_store: Vec<String>,
}

impl OfflineAILocal {
    pub fn new(model_name: &str) -> Self {
        OfflineAILocal {
            model_name: String::from(model_name),
            data_store: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: &str) {
        self.data_store.push(String::from(data));
    }

    pub fn get_data(&self, index: usize) -> Option<&String> {
        self.data_store.get(index)
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data_store.len() {
            Some(self.data_store.remove(index))
        } else {
            None
        }
    }

    pub fn list_data(&self) -> &Vec<String> {
        &self.data_store
    }

    pub fn model_info(&self) -> String {
        format!("Model Name: {}", self.model_name)
    }
}
