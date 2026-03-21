extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct Intent {
    action: String,
    data: String,
    category: Vec<String>,
    extras: Vec<(String, String)>,
}

impl Intent {
    pub fn new(action: &str, data: &str) -> Self {
        Intent {
            action: String::from(action),
            data: String::from(data),
            category: Vec::new(),
            extras: Vec::new(),
        }
    }

    pub fn add_category(&mut self, category: &str) {
        self.category.push(String::from(category));
    }

    pub fn put_extra(&mut self, key: &str, value: &str) {
        self.extras.push((String::from(key), String::from(value)));
    }

    pub fn get_action(&self) -> &str {
        &self.action
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }

    pub fn get_categories(&self) -> &[String] {
        &self.category
    }

    pub fn get_extras(&self) -> &[(String, String)] {
        &self.extras
    }
}
