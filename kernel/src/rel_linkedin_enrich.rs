extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct LinkedInEnricher {
    data: Vec<String>,
}

impl LinkedInEnricher {
    pub fn new() -> Self {
        LinkedInEnricher { data: Vec::new() }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn find_data(&self, query: &str) -> Vec<&String> {
        self.data.iter().filter(|&&item| item.contains(query)).collect()
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}
