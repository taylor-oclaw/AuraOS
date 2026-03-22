extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AiFineTuneApi {
    model_name: String,
    parameters: Vec<(String, String)>,
}

impl AiFineTuneApi {
    pub fn new(model_name: &str) -> Self {
        AiFineTuneApi {
            model_name: String::from(model_name),
            parameters: Vec::new(),
        }
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let param = (String::from(key), String::from(value));
        self.parameters.push(param);
    }

    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.parameters {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn list_parameters(&self) -> Vec<&(String, String)> {
        self.parameters.iter().collect()
    }

    pub fn remove_parameter(&mut self, key: &str) {
        self.parameters.retain(|k, _| k != key);
    }

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }
}