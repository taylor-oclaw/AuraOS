extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct TarHandler {
    files: Vec<(String, Vec<u8>)>,
}

impl TarHandler {
    pub fn new() -> Self {
        TarHandler { files: Vec::new() }
    }

    pub fn add_file(&mut self, name: String, content: Vec<u8>) {
        self.files.push((name, content));
    }

    pub fn get_file(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.iter().find_map(|(n, c)| if n == name { Some(c) } else { None })
    }

    pub fn list_files(&self) -> Vec<&String> {
        self.files.iter().map(|(n, _)| n).collect()
    }

    pub fn remove_file(&mut self, name: &str) {
        self.files.retain(|(n, _)| n != name);
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}
