extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AccessCaptionStyle {
    styles: Vec<String>,
}

impl AccessCaptionStyle {
    pub fn new() -> Self {
        AccessCaptionStyle {
            styles: Vec::new(),
        }
    }

    pub fn add_style(&mut self, style: String) {
        self.styles.push(style);
    }

    pub fn remove_style(&mut self, index: usize) -> Option<String> {
        if index < self.styles.len() {
            Some(self.styles.remove(index))
        } else {
            None
        }
    }

    pub fn get_style(&self, index: usize) -> Option<&String> {
        self.styles.get(index)
    }

    pub fn list_styles(&self) -> &[String] {
        &self.styles
    }

    pub fn clear_styles(&mut self) {
        self.styles.clear();
    }
}
