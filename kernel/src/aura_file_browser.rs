extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_ffi_init() {
    // Initialize the module if needed
}

#[no_mangle]
pub extern "C" fn rust_ffi_exit() {
    // Clean up the module if needed
}

pub struct AuraFileBrowser {
    files: Vec<String>,
    current_directory: String,
}

impl AuraFileBrowser {
    pub fn new(initial_directory: &str) -> Self {
        AuraFileBrowser {
            files: Vec::new(),
            current_directory: initial_directory.to_string(),
        }
    }

    pub fn change_directory(&mut self, directory: &str) {
        self.current_directory = directory.to_string();
    }

    pub fn list_files(&self) -> &[String] {
        &self.files
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(file_name.to_string());
    }

    pub fn remove_file(&mut self, file_name: &str) {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
        }
    }

    pub fn get_current_directory(&self) -> &String {
        &self.current_directory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aura_file_browser() {
        let mut browser = AuraFileBrowser::new("/home/user");
        assert_eq!(browser.get_current_directory(), "/home/user");

        browser.add_file("file1.txt");
        browser.add_file("file2.txt");
        assert_eq!(browser.list_files().len(), 2);

        browser.remove_file("file1.txt");
        assert_eq!(browser.list_files().len(), 1);
        assert_eq!(browser.list_files()[0], "file2.txt");

        browser.change_directory("/home/user/documents");
        assert_eq!(browser.get_current_directory(), "/home/user/documents");
    }
}
