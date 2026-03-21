extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct SharedFileHistory {
    history: Vec<String>,
}

impl SharedFileHistory {
    pub fn new() -> Self {
        SharedFileHistory {
            history: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        let file_name = String::from(file_name);
        if !self.history.contains(&file_name) {
            self.history.push(file_name);
        }
    }

    pub fn remove_file(&mut self, file_name: &str) {
        if let Some(index) = self.history.iter().position(|x| x == file_name) {
            self.history.remove(index);
        }
    }

    pub fn get_history(&self) -> &[String] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn contains_file(&self, file_name: &str) -> bool {
        self.history.contains(&String::from(file_name))
    }
}
