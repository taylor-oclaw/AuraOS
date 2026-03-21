extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn init_module() -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() -> i32 {
    0
}

pub struct ToneOverExplainAvoid {
    data: Vec<String>,
}

impl ToneOverExplainAvoid {
    pub fn new() -> Self {
        ToneOverExplainAvoid {
            data: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: String) {
        self.data.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<String> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    pub fn get_entry(&self, index: usize) -> Option<&String> {
        self.data.get(index)
    }

    pub fn clear_entries(&mut self) {
        self.data.clear();
    }

    pub fn count_entries(&self) -> usize {
        self.data.len()
    }
}
