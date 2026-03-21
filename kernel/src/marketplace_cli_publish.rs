extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceCLI {
    items: Vec<String>,
}

impl MarketplaceCLI {
    pub fn new() -> Self {
        MarketplaceCLI { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_name: &str) {
        let item = String::from(item_name);
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|x| x == item_name) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn find_item(&self, item_name: &str) -> Option<&String> {
        self.items.iter().find(|&&x| x == item_name)
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}
