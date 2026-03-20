extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

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
        let name = String::from(archive_name);
        if !self.archives.contains(&name) {
            self.archives.push(name);
        }
    }

    pub fn remove_archive(&mut self, archive_name: &str) {
        self.archives.retain(|a| a != archive_name);
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
