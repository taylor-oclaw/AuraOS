extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn predict_file_access_init() {
    // Initialization logic for the module
}

#[no_mangle]
pub extern "C" fn predict_file_access_exit() {
    // Cleanup logic for the module
}

pub struct FileAccessPredictor {
    file_access_history: Vec<String>,
}

impl FileAccessPredictor {
    pub fn new() -> Self {
        FileAccessPredictor {
            file_access_history: Vec::new(),
        }
    }

    pub fn record_access(&mut self, file_path: &str) {
        self.file_access_history.push(file_path.to_string());
    }

    pub fn get_recent_files(&self, count: usize) -> Vec<String> {
        let start = if self.file_access_history.len() > count {
            self.file_access_history.len() - count
        } else {
            0
        };
        self.file_access_history[start..].to_vec()
    }

    pub fn predict_next_file(&self) -> Option<&String> {
        if self.file_access_history.is_empty() {
            None
        } else {
            Some(self.file_access_history.last().unwrap())
        }
    }

    pub fn clear_history(&mut self) {
        self.file_access_history.clear();
    }

    pub fn get_history_length(&self) -> usize {
        self.file_access_history.len()
    }
}
