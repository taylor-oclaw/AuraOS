extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct SystemTray {
    items: Vec<String>,
}

impl SystemTray {
    pub fn new() -> Self {
        SystemTray { items: Vec::new() }
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

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn clear_tray(&mut self) {
        self.items.clear();
    }
}
