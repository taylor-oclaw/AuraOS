extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rel_archive_manage_init() {
    // Initialization logic for the module
}

pub extern "C" fn rel_archive_manage_exit() {
    // Cleanup logic for the module
}

pub struct ArchiveManager {
    archives: Vec<String>,
}

impl ArchiveManager {
    pub fn new() -> Self {
        ArchiveManager {
            archives: Vec::new(),
        }
    }

    pub fn add_archive(&mut self, archive_name: &str) {
        self.archives.push(String::from(archive_name));
    }

    pub fn remove_archive(&mut self, archive_name: &str) {
        if let Some(index) = self.archives.iter().position(|x| x == archive_name) {
            self.archives.remove(index);
        }
    }

    pub fn list_archives(&self) -> Vec<String> {
        self.archives.clone()
    }

    pub fn contains_archive(&self, archive_name: &str) -> bool {
        self.archives.contains(&String::from(archive_name))
    }

    pub fn count_archives(&self) -> usize {
        self.archives.len()
    }
}

pub extern "C" fn rel_archive_manage_create() -> *mut ArchiveManager {
    Box::into_raw(Box::new(ArchiveManager::new()))
}

pub extern "C" fn rel_archive_manage_destroy(manager: *mut ArchiveManager) {
    unsafe { drop(Box::from_raw(manager)) }
}
