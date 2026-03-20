extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_ffi_init() {
    // Initialization code for the module
}

pub extern "C" fn rust_ffi_exit() {
    // Cleanup code for the module
}

pub struct AuraColorMgr {
    colors: Vec<String>,
}

impl AuraColorMgr {
    pub fn new() -> Self {
        AuraColorMgr {
            colors: Vec::new(),
        }
    }

    pub fn add_color(&mut self, color: String) {
        self.colors.push(color);
    }

    pub fn remove_color(&mut self, index: usize) -> Option<String> {
        if index < self.colors.len() {
            Some(self.colors.remove(index))
        } else {
            None
        }
    }

    pub fn get_color(&self, index: usize) -> Option<&String> {
        self.colors.get(index)
    }

    pub fn list_colors(&self) -> &Vec<String> {
        &self.colors
    }

    pub fn clear_colors(&mut self) {
        self.colors.clear();
    }
}
