extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut package = CruciblePackage::new();
    package.add_file("file1.txt", b"Hello, World!");
    package.add_file("file2.txt", b"This is a test file.");

    if let Some(content) = package.get_file_content("file1.txt") {
        // Process the content of file1.txt
    }

    package.remove_file("file2.txt");

    loop {}
}

pub struct CruciblePackage {
    files: Vec<(String, Vec<u8>)>,
}

impl CruciblePackage {
    pub fn new() -> Self {
        CruciblePackage { files: Vec::new() }
    }

    pub fn add_file(&mut self, filename: &str, content: &[u8]) {
        let file = (String::from(filename), content.to_vec());
        self.files.push(file);
    }

    pub fn get_file_content(&self, filename: &str) -> Option<&Vec<u8>> {
        for (name, content) in &self.files {
            if name == filename {
                return Some(content);
            }
        }
        None
    }

    pub fn remove_file(&mut self, filename: &str) {
        self.files.retain(|(name, _)| name != filename);
    }

    pub fn list_files(&self) -> Vec<&String> {
        self.files.iter().map(|(name, _)| name).collect()
    }

    pub fn file_exists(&self, filename: &str) -> bool {
        self.get_file_content(filename).is_some()
    }
}
