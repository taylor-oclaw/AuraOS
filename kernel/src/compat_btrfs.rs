extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

struct BtrfsFileSystem {
    name: String,
    mount_point: String,
    files: Vec<String>,
}

impl BtrfsFileSystem {
    pub fn new(name: &str, mount_point: &str) -> Self {
        BtrfsFileSystem {
            name: String::from(name),
            mount_point: String::from(mount_point),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, file_name: &str) {
        self.files.push(String::from(file_name));
    }

    pub fn remove_file(&mut self, file_name: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_name) {
            self.files.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_files(&self) -> Vec<String> {
        self.files.clone()
    }

    pub fn is_file_present(&self, file_name: &str) -> bool {
        self.files.contains(&String::from(file_name))
    }

    pub fn get_file_count(&self) -> usize {
        self.files.len()
    }
}

#[no_mangle]
pub extern "C" fn rust_stop() {
    // Cleanup code for the kernel module
}
