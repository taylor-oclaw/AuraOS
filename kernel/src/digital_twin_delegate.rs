extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct DigitalTwinDelegate {
    name: String,
    attributes: Vec<(String, String)>,
}

impl DigitalTwinDelegate {
    pub fn new(name: &str) -> Self {
        DigitalTwinDelegate {
            name: String::from(name),
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.push((String::from(key), String::from(value)));
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        for (k, v) in &self.attributes {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    pub fn remove_attribute(&mut self, key: &str) {
        self.attributes.retain(|(k, _)| k != key);
    }

    pub fn list_attributes(&self) -> Vec<&String> {
        self.attributes.iter().map(|(_, v)| v).collect()
    }

    pub fn update_attribute(&mut self, key: &str, new_value: &str) {
        if let Some((_, value)) = self.attributes.iter_mut().find(|(k, _)| k == key) {
            *value = String::from(new_value);
        }
    }
}
