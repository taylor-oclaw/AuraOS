extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn miniapp_file_quick_share_init() {
    // Initialization logic for the module
}

pub extern "C" fn miniapp_file_quick_share_exit() {
    // Cleanup logic for the module
}

pub struct FileShareApp {
    files: Vec<String>,
    shared_files: Vec<String>,
}

impl FileShareApp {
    pub fn new() -> Self {
        FileShareApp {
            files: Vec::new(),
            shared_files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(file_name.to_string());
    }

    pub fn remove_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
            true
        } else {
            false
        }
    }

    pub fn share_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            let file = self.files.remove(index);
            self.shared_files.push(file);
            true
        } else {
            false
        }
    }

    pub fn unshare_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.shared_files.iter().position(|f| f == file_name) {
            let file = self.shared_files.remove(index);
            self.files.push(file);
            true
        } else {
            false
        }
    }

    pub fn list_shared_files(&self) -> Vec<String> {
        self.shared_files.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_operations() {
        let mut app = FileShareApp::new();
        app.add_file("file1.txt");
        assert_eq!(app.files.len(), 1);

        app.share_file("file1.txt");
        assert_eq!(app.files.len(), 0);
        assert_eq!(app.shared_files.len(), 1);

        app.unshare_file("file1.txt");
        assert_eq!(app.files.len(), 1);
        assert_eq!(app.shared_files.len(), 0);

        let removed = app.remove_file("file1.txt");
        assert!(removed);
        assert_eq!(app.files.len(), 0);
    }
}
