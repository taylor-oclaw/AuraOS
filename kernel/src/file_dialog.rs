extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct FileDialog {
    files: Vec<String>,
    selected_index: usize,
}

impl FileDialog {
    pub fn new(files: Vec<String>) -> Self {
        FileDialog {
            files,
            selected_index: 0,
        }
    }

    pub fn add_file(&mut self, file_name: String) {
        self.files.push(file_name);
    }

    pub fn remove_file(&mut self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
            if self.selected_index >= self.files.len() && !self.files.is_empty() {
                self.selected_index = self.files.len() - 1;
            }
        }
    }

    pub fn select_next(&mut self) {
        if !self.files.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.files.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.files.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.files.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    pub fn get_selected_file(&self) -> Option<&String> {
        self.files.get(self.selected_index)
    }
}
