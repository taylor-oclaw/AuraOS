extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn rust_start() -> i32 {
    0
}

pub struct ActionItemVerify {
    items: Vec<String>,
}

impl ActionItemVerify {
    pub fn new() -> Self {
        ActionItemVerify {
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.items.iter().any(|i| i == item)
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }
}
