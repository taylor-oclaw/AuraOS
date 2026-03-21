extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct PhotoPersonTag {
    name: String,
    age: u8,
    tags: Vec<String>,
}

impl PhotoPersonTag {
    pub fn new(name: &str, age: u8) -> Self {
        PhotoPersonTag {
            name: String::from(name),
            age,
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(String::from(tag));
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(index) = self.tags.iter().position(|t| t == tag) {
            self.tags.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn update_age(&mut self, new_age: u8) {
        self.age = new_age;
    }
}
