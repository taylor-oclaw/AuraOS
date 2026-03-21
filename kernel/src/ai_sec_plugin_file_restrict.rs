extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct FileRestrict {
    allowed_files: Vec<String>,
    blocked_files: Vec<String>,
}

impl FileRestrict {
    pub fn new() -> Self {
        FileRestrict {
            allowed_files: Vec::new(),
            blocked_files: Vec::new(),
        }
    }

    pub fn allow_file(&mut self, file_path: &str) {
        if !self.allowed_files.contains(&file_path.to_string()) {
            self.allowed_files.push(file_path.to_string());
        }
    }

    pub fn block_file(&mut self, file_path: &str) {
        if !self.blocked_files.contains(&file_path.to_string()) {
            self.blocked_files.push(file_path.to_string());
        }
    }

    pub fn is_allowed(&self, file_path: &str) -> bool {
        self.allowed_files.contains(&file_path.to_string())
    }

    pub fn is_blocked(&self, file_path: &str) -> bool {
        self.blocked_files.contains(&file_path.to_string())
    }

    pub fn list_allowed_files(&self) -> Vec<String> {
        self.allowed_files.clone()
    }

    pub fn list_blocked_files(&self) -> Vec<String> {
        self.blocked_files.clone()
    }
}
