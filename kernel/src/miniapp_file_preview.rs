extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> i32 {
    0
}

struct MiniAppFilePreview {
    file_name: String,
    content: Vec<u8>,
    preview_lines: usize,
}

impl MiniAppFilePreview {
    pub fn new(file_name: &str, content: &[u8], preview_lines: usize) -> Self {
        MiniAppFilePreview {
            file_name: String::from(file_name),
            content: content.to_vec(),
            preview_lines,
        }
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }

    pub fn set_preview_lines(&mut self, lines: usize) {
        self.preview_lines = lines;
    }

    pub fn get_content_length(&self) -> usize {
        self.content.len()
    }

    pub fn preview_content(&self) -> Vec<String> {
        let mut lines = Vec::new();
        let content_str = String::from_utf8_lossy(&self.content);
        for line in content_str.lines().take(self.preview_lines) {
            lines.push(line.to_string());
        }
        lines
    }

    pub fn search_content(&self, query: &str) -> Vec<String> {
        let mut results = Vec::new();
        let content_str = String::from_utf8_lossy(&self.content);
        for line in content_str.lines() {
            if line.contains(query) {
                results.push(line.to_string());
            }
        }
        results
    }
}
