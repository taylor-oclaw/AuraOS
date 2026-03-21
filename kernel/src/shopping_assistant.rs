extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ShoppingAssistant {
    items: Vec<String>,
}

impl ShoppingAssistant {
    pub fn new() -> Self {
        ShoppingAssistant { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        if !self.items.contains(&item) {
            self.items.push(item);
        }
    }

    pub fn remove_item(&mut self, item: &str) {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.items.remove(index);
        }
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn has_item(&self, item: &str) -> bool {
        self.items.contains(&String::from(item))
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}
