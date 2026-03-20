extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceSearch {
    items: Vec<String>,
}

impl MarketplaceSearch {
    pub fn new() -> Self {
        MarketplaceSearch { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: &str) -> bool {
        if let Some(index) = self.items.iter().position(|i| i == item) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }

    pub fn search_items(&self, query: &str) -> Vec<String> {
        self.items.iter()
            .filter(|item| item.contains(query))
            .cloned()
            .collect()
    }

    pub fn list_all_items(&self) -> Vec<String> {
        self.items.clone()
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}
