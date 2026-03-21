extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppUIElementFinder {
    elements: Vec<String>,
}

impl AppUIElementFinder {
    pub fn new() -> Self {
        AppUIElementFinder {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element_name: &str) {
        self.elements.push(String::from(element_name));
    }

    pub fn remove_element(&mut self, element_name: &str) {
        if let Some(index) = self.elements.iter().position(|x| x == element_name) {
            self.elements.remove(index);
        }
    }

    pub fn find_element(&self, element_name: &str) -> bool {
        self.elements.contains(&String::from(element_name))
    }

    pub fn list_elements(&self) -> Vec<String> {
        self.elements.clone()
    }

    pub fn count_elements(&self) -> usize {
        self.elements.len()
    }
}
