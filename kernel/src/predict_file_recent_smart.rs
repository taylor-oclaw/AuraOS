extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::cmp::Ordering;

pub struct PredictFileRecentSmart {
    file_history: Vec<String>,
    max_capacity: usize,
}

impl PredictFileRecentSmart {
    pub fn new(max_capacity: usize) -> Self {
        PredictFileRecentSmart {
            file_history: Vec::new(),
            max_capacity,
        }
    }

    pub fn add_file(&mut self, file_path: &str) {
        if let Some(index) = self.file_history.iter().position(|f| f == file_path) {
            self.file_history.remove(index);
        }
        self.file_history.insert(0, String::from(file_path));
        if self.file_history.len() > self.max_capacity {
            self.file_history.pop();
        }
    }

    pub fn get_recent_files(&self) -> Vec<String> {
        self.file_history.clone()
    }

    pub fn clear_history(&mut self) {
        self.file_history.clear();
    }

    pub fn predict_next_file(&self, prefix: &str) -> Option<&String> {
        self.file_history.iter().find(|f| f.starts_with(prefix))
    }

    pub fn remove_file(&mut self, file_path: &str) {
        if let Some(index) = self.file_history.iter().position(|f| f == file_path) {
            self.file_history.remove(index);
        }
    }
}
