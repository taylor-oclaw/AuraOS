extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct AppDomParser {
    dom: Vec<String>,
}

impl AppDomParser {
    pub fn new() -> Self {
        AppDomParser { dom: Vec::new() }
    }

    pub fn add_element(&mut self, element: &str) {
        self.dom.push(String::from(element));
    }

    pub fn remove_element(&mut self, index: usize) -> Option<String> {
        if index < self.dom.len() {
            Some(self.dom.remove(index))
        } else {
            None
        }
    }

    pub fn get_element(&self, index: usize) -> Option<&str> {
        self.dom.get(index).map(|s| s.as_str())
    }

    pub fn find_elements_by_name(&self, name: &str) -> Vec<usize> {
        let mut indices = Vec::new();
        for (i, element) in self.dom.iter().enumerate() {
            if element == name {
                indices.push(i);
            }
        }
        indices
    }

    pub fn count_elements(&self) -> usize {
        self.dom.len()
    }
}
