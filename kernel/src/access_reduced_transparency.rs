extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    // Entry point for the kernel module
}

pub struct AccessReducedTransparency {
    data: Vec<String>,
    access_log: Vec<String>,
}

impl AccessReducedTransparency {
    pub fn new() -> Self {
        AccessReducedTransparency {
            data: Vec::new(),
            access_log: Vec::new(),
        }
    }

    pub fn add_data(&mut self, item: String) {
        self.data.push(item);
    }

    pub fn get_data(&self, index: usize) -> Option<&String> {
        if let Some(data) = self.data.get(index) {
            self.access_log.push(format!("Accessed data at index {}", index));
            Some(data)
        } else {
            None
        }
    }

    pub fn remove_data(&mut self, index: usize) -> Option<String> {
        if let Some(removed_item) = self.data.remove(index) {
            self.access_log.push(format!("Removed data at index {}", index));
            Some(removed_item)
        } else {
            None
        }
    }

    pub fn log_size(&self) -> usize {
        self.access_log.len()
    }

    pub fn get_access_log(&self) -> &Vec<String> {
        &self.access_log
    }
}
