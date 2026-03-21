extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

pub struct MarketplacePricingEngine {
    items: Vec<Item>,
}

impl MarketplacePricingEngine {
    pub fn new() -> Self {
        MarketplacePricingEngine { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String, price: u32) {
        let item = Item { name, price };
        self.items.push(item);
    }

    pub fn get_item_price(&self, name: &str) -> Option<u32> {
        for item in &self.items {
            if item.name == name {
                return Some(item.price);
            }
        }
        None
    }

    pub fn remove_item(&mut self, name: &str) {
        self.items.retain(|item| item.name != name);
    }

    pub fn update_price(&mut self, name: &str, new_price: u32) -> bool {
        for item in &mut self.items {
            if item.name == name {
                item.price = new_price;
                return true;
            }
        }
        false
    }

    pub fn list_items(&self) -> Vec<String> {
        self.items.iter().map(|item| item.name.clone()).collect()
    }
}

struct Item {
    name: String,
    price: u32,
}
