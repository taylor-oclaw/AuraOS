extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ColorSpaceManager {
    color_spaces: Vec<String>,
}

impl ColorSpaceManager {
    pub fn new() -> Self {
        ColorSpaceManager {
            color_spaces: Vec::new(),
        }
    }

    pub fn add_color_space(&mut self, name: &str) {
        if !self.color_spaces.contains(&String::from(name)) {
            self.color_spaces.push(String::from(name));
        }
    }

    pub fn remove_color_space(&mut self, name: &str) {
        self.color_spaces.retain(|space| space != name);
    }

    pub fn list_color_spaces(&self) -> Vec<String> {
        self.color_spaces.clone()
    }

    pub fn has_color_space(&self, name: &str) -> bool {
        self.color_spaces.contains(&String::from(name))
    }

    pub fn count_color_spaces(&self) -> usize {
        self.color_spaces.len()
    }
}
