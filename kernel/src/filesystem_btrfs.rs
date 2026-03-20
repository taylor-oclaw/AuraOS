extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn filesystem_btrfs_init() -> i32 {
    // Initialization logic for the Btrfs filesystem module
    0
}

pub extern "C" fn filesystem_btrfs_exit() {
    // Cleanup logic for the Btrfs filesystem module
}

pub struct FilesystemBtrfs {
    name: String,
    mount_point: String,
    files: Vec<String>,
}

impl FilesystemBtrfs {
    pub fn new(name: &str, mount_point: &str) -> Self {
        FilesystemBtrfs {
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

    pub fn get_mount_point(&self) -> String {
        self.mount_point.clone()
    }
}
