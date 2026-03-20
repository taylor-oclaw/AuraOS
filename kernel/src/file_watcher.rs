extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FileWatcher {
    watched_files: Vec<String>,
}

impl FileWatcher {
    pub fn new() -> Self {
        FileWatcher {
            watched_files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_path: &str) {
        if !self.watched_files.contains(&file_path.to_string()) {
            self.watched_files.push(file_path.to_string());
        }
    }

    pub fn remove_file(&mut self, file_path: &str) {
        self.watched_files.retain(|f| f != file_path);
    }

    pub fn is_watching(&self, file_path: &str) -> bool {
        self.watched_files.contains(&file_path.to_string())
    }

    pub fn list_watched_files(&self) -> Vec<String> {
        self.watched_files.clone()
    }

    pub fn clear_all(&mut self) {
        self.watched_files.clear();
    }
}
