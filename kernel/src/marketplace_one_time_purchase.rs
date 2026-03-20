extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn marketplace_one_time_purchase_init() {
    // Initialization logic for the module
}

pub extern "C" fn marketplace_one_time_purchase_exit() {
    // Cleanup logic for the module
}

pub struct MarketplaceOneTimePurchase {
    items: Vec<Item>,
}

impl MarketplaceOneTimePurchase {
    pub fn new() -> Self {
        MarketplaceOneTimePurchase { items: Vec::new() }
    }

    pub fn add_item(&mut self, name: String, price: u32) {
        let item = Item { name, price };
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn get_item(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    pub fn list_items(&self) -> &[Item] {
        &self.items
    }

    pub fn total_price(&self) -> u32 {
        self.items.iter().map(|item| item.price).sum()
    }
}

struct Item {
    name: String,
    price: u32,
}
