extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MeshKeyRotation {
    keys: Vec<String>,
    current_key_index: usize,
}

impl MeshKeyRotation {
    pub fn new(initial_keys: Vec<String>) -> Self {
        MeshKeyRotation {
            keys: initial_keys,
            current_key_index: 0,
        }
    }

    pub fn add_key(&mut self, key: String) {
        self.keys.push(key);
    }

    pub fn remove_key(&mut self, index: usize) -> Option<String> {
        if index < self.keys.len() {
            Some(self.keys.remove(index))
        } else {
            None
        }
    }

    pub fn get_current_key(&self) -> &String {
        &self.keys[self.current_key_index]
    }

    pub fn rotate_keys(&mut self) {
        self.current_key_index = (self.current_key_index + 1) % self.keys.len();
    }

    pub fn list_keys(&self) -> &[String] {
        &self.keys
    }
}
