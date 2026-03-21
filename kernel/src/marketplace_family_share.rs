extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplaceFamilyShare {
    items: Vec<String>,
}

impl MarketplaceFamilyShare {
    pub fn new() -> Self {
        MarketplaceFamilyShare { items: Vec::new() }
    }

    pub fn add_item(&mut self, item_name: &str) {
        self.items.push(item_name.to_string());
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

    pub fn contains_item(&self, item_name: &str) -> bool {
        self.items.contains(&item_name.to_string())
    }

    pub fn count_items(&self) -> usize {
        self.items.len()
    }
}
