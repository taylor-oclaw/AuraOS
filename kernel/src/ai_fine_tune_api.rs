extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AIFineTuneAPI {
    model_name: String,
    parameters: Vec<String>,
    learning_rate: f32,
    epochs: u32,
    batch_size: u32,
}

impl AIFineTuneAPI {
    pub fn new(model_name: &str, parameters: &[&str], learning_rate: f32, epochs: u32, batch_size: u32) -> Self {
        AIFineTuneAPI {
            model_name: String::from(model_name),
            parameters: parameters.iter().map(|&s| String::from(s)).collect(),
            learning_rate,
            epochs,
            batch_size,
        }
    }

    pub fn get_model_name(&self) -> &str {
        &self.model_name
    }

    pub fn set_model_name(&mut self, model_name: &str) {
        self.model_name = String::from(model_name);
    }

    pub fn add_parameter(&mut self, parameter: &str) {
        self.parameters.push(String::from(parameter));
    }

    pub fn remove_parameter(&mut self, parameter: &str) -> bool {
        if let Some(index) = self.parameters.iter().position(|p| p == parameter) {
            self.parameters.remove(index);
            true
        } else {
            false
        }
    }

    pub fn get_parameters(&self) -> &[String] {
        &self.parameters
    }

    pub fn set_learning_rate(&mut self, learning_rate: f32) {
        self.learning_rate = learning_rate;
    }

    pub fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }

    pub fn set_epochs(&mut self, epochs: u32) {
        self.epochs = epochs;
    }

    pub fn get_epochs(&self) -> u32 {
        self.epochs
    }

    pub fn set_batch_size(&mut self, batch_size: u32) {
        self.batch_size = batch_size;
    }

    pub fn get_batch_size(&self) -> u32 {
        self.batch_size
    }
}
