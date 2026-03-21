extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut scanner = CodeScanner::new();
    scanner.add_pattern("malicious_code".to_string());
    scanner.scan_file("example_file.txt".to_string());

    loop {}
}

pub struct CodeScanner {
    patterns: Vec<String>,
    results: Vec<String>,
}

impl CodeScanner {
    pub fn new() -> Self {
        CodeScanner {
            patterns: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: String) {
        self.patterns.push(pattern);
    }

    pub fn scan_file(&mut self, file_content: String) {
        for pattern in &self.patterns {
            if file_content.contains(pattern) {
                self.results.push(pattern.clone());
            }
        }
    }

    pub fn get_results(&self) -> Vec<String> {
        self.results.clone()
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    pub fn has_patterns(&self) -> bool {
        !self.patterns.is_empty()
    }
}
