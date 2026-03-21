extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Clipboard {
    content: Vec<u8>,
}

impl Clipboard {
    pub fn new() -> Self {
        Clipboard {
            content: Vec::new(),
        }
    }

    pub fn set_content(&mut self, data: &[u8]) {
        self.content.clear();
        self.content.extend_from_slice(data);
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    pub fn append_content(&mut self, data: &[u8]) {
        self.content.extend_from_slice(data);
    }

    pub fn content_size(&self) -> usize {
        self.content.len()
    }
}
