extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ZipHandler {
    files: Vec<String>,
}

impl ZipHandler {
    pub fn new() -> Self {
        ZipHandler { files: Vec::new() }
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(String::from(file_name));
    }

    pub fn remove_file(&mut self, file_name: &str) {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn contains_file(&self, file_name: &str) -> bool {
        self.files.contains(&String::from(file_name))
    }

    pub fn clear_files(&mut self) {
        self.files.clear();
    }
}
