extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct AutoTaskFormFill {
    form_data: Vec<(String, String)>,
}

impl AutoTaskFormFill {
    pub fn new() -> Self {
        AutoTaskFormFill {
            form_data: Vec::new(),
        }
    }

    pub fn add_field(&mut self, key: &str, value: &str) {
        let key = String::from(key);
        let value = String::from(value);
        self.form_data.push((key, value));
    }

    pub fn get_field(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.form_data {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_field(&mut self, key: &str) {
        self.form_data.retain(|(k, _)| k != key);
    }

    pub fn clear_fields(&mut self) {
        self.form_data.clear();
    }

    pub fn get_all_fields(&self) -> Vec<(&String, &String)> {
        self.form_data.iter().map(|(k, v)| (k, v)).collect()
    }
}
