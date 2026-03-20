extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FilesystemFat {
    // Simulated FAT filesystem data structures
    root_directory: Vec<String>,
    files: Vec<(String, String)>, // (filename, content)
}

impl FilesystemFat {
    pub fn new() -> Self {
        FilesystemFat {
            root_directory: Vec::new(),
            files: Vec::new(),
        }
    }

    pub fn create_file(&mut self, filename: &str, content: &str) {
        if !self.file_exists(filename) {
            self.files.push((String::from(filename), String::from(content)));
            self.root_directory.push(String::from(filename));
        }
    }

    pub fn read_file(&self, filename: &str) -> Option<&str> {
        for (file_name, content) in &self.files {
            if file_name == filename {
                return Some(content);
            }
        }
        None
    }

    pub fn delete_file(&mut self, filename: &str) {
        self.files.retain(|(name, _)| name != filename);
        self.root_directory.retain(|name| name != filename);
    }

    pub fn list_files(&self) -> Vec<&str> {
        self.root_directory.iter().map(|s| s.as_str()).collect()
    }

    pub fn file_exists(&self, filename: &str) -> bool {
        self.files.iter().any(|(name, _)| name == filename)
    }
}
